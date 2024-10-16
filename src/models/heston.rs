use crate::data::InterestRateCurve;
use crate::errors::OptionPricingError;
use crate::regression::{Regression, RegressionDataPoint, RegressionInput, RegressionMethod};
use crate::regression::lsm::LeastSquaresMonteCarlo;
use crate::regression::random_forest::RandomForestRegression;
use crate::traits::OptionPricingModel;
use crate::utils::interpolate_rate;
use crate::OptionType;
use rand::Rng;
use rand_distr::StandardNormal;
use std::rc::Rc;

pub struct HestonModel {
    pub option_type: OptionType,
    pub spot_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub initial_variance: f64,
    pub risk_free_rate_curve: InterestRateCurve,
    pub kappa: f64,
    pub theta: f64,
    pub sigma: f64,
    pub rho: f64,
    pub is_american: bool,
    pub regression_method: RegressionMethod,
    pub num_paths: usize,
    pub num_steps: usize,
}

impl OptionPricingModel for HestonModel {
    fn price(&self) -> Result<f64, OptionPricingError> {
        // Simulate asset paths
        let mut paths = Vec::with_capacity(self.num_paths);
        let dt = self.time_to_expiry / self.num_steps as f64;
        let sqrt_dt = dt.sqrt();

        let mut rng = rand::thread_rng();

        for _ in 0..self.num_paths {
            let mut spot = self.spot_price;
            let mut variance = self.initial_variance;

            let mut path = Vec::with_capacity(self.num_steps + 1);
            path.push((0.0, spot)); // (time, spot)

            for step in 0..self.num_steps {
                let z1: f64 = rng.sample(StandardNormal);
                let z2: f64 = rng.sample(StandardNormal);
                let w1 = z1;
                let w2 = self.rho * z1 + (1.0 - self.rho.powi(2)).sqrt() * z2;

                // Variance process
                variance = (variance
                    + self.kappa * (self.theta - variance) * dt
                    + self.sigma * variance.sqrt() * w2 * sqrt_dt)
                    .max(0.0);

                // Interest rate interpolation
                let time = (step as f64 + 1.0) * dt;
                let rate = interpolate_rate(&self.risk_free_rate_curve, time)?;

                // Asset price process
                spot = spot * ((rate - 0.5 * variance) * dt + variance.sqrt() * w1 * sqrt_dt).exp();

                path.push((time, spot));
            }
            paths.push(path);
        }

        if !self.is_american {
            // Pricing European option
            let mut payoffs = Vec::with_capacity(self.num_paths);

            match self.option_type {
                OptionType::Call => {
                    for path in &paths {
                        let spot = path[self.num_steps].1;
                        payoffs.push((spot - self.strike_price).max(0.0));
                    }
                }
                OptionType::Put => {
                    for path in &paths {
                        let spot = path[self.num_steps].1;
                        payoffs.push((self.strike_price - spot).max(0.0));
                    }
                }
            }

            let average_payoff: f64 = payoffs.iter().sum::<f64>() / self.num_paths as f64;
            let rate = interpolate_rate(&self.risk_free_rate_curve, self.time_to_expiry)?;
            let discounted_payoff = average_payoff * (-rate * self.time_to_expiry).exp();
            return Ok(discounted_payoff);
        }

        // For American options, use regression methods
        let regression: Rc<dyn Regression> = match self.regression_method {
            RegressionMethod::LeastSquaresMonteCarlo => Rc::new(LeastSquaresMonteCarlo {}),
            RegressionMethod::RandomForest => Rc::new(RandomForestRegression {}),
        };

        // Initialize option values at maturity
        let mut option_values = vec![0.0; self.num_paths];
        for (i, path) in paths.iter().enumerate() {
            let spot = path[self.num_steps].1;
            option_values[i] = match self.option_type {
                OptionType::Call => (spot - self.strike_price).max(0.0),
                OptionType::Put => (self.strike_price - spot).max(0.0),
            };
        }

        // Backward induction
        for step in (1..self.num_steps).rev() {
            let time = step as f64 * dt;

            // Collect regression data
            let mut regression_data = Vec::new();

            for i in 0..self.num_paths {
                let spot = paths[i][step].1;

                let immediate_exercise_value = match self.option_type {
                    OptionType::Call => (spot - self.strike_price).max(0.0),
                    OptionType::Put => (self.strike_price - spot).max(0.0),
                };

                if immediate_exercise_value > 0.0 {
                    // Discounted continuation value
                    let rate = interpolate_rate(&self.risk_free_rate_curve, time)?;
                    let continuation_value = option_values[i] * (-rate * dt).exp();

                    regression_data.push(RegressionDataPoint {
                        time,
                        asset_price: spot,
                        continuation_value,
                    });
                }
            }

            // If there are no in-the-money paths, continue
            if regression_data.is_empty() {
                continue;
            }

            // Fit regression model
            let regression_model = regression.fit(&regression_data)?;

            // Update option values
            for i in 0..self.num_paths {
                let spot = paths[i][step].1;

                let immediate_exercise_value = match self.option_type {
                    OptionType::Call => (spot - self.strike_price).max(0.0),
                    OptionType::Put => (self.strike_price - spot).max(0.0),
                };

                let rate = interpolate_rate(&self.risk_free_rate_curve, time)?;
                let discounted_value = option_values[i] * (-rate * dt).exp();

                if immediate_exercise_value > 0.0 {
                    // Predict continuation value
                    let regression_input = RegressionInput {
                        time,
                        asset_price: spot,
                    };

                    let predicted_continuation_value = regression_model.predict(&regression_input);

                    if immediate_exercise_value >= predicted_continuation_value {
                        // Exercise
                        option_values[i] = immediate_exercise_value;
                    } else {
                        // Continue
                        option_values[i] = discounted_value;
                    }
                } else {
                    // Out of the money, continue
                    option_values[i] = discounted_value;
                }
            }
        }

        // Discount option values to present value
        let mut price = 0.0;
        for i in 0..self.num_paths {
            let rate = interpolate_rate(&self.risk_free_rate_curve, 0.0)?;
            price += option_values[i] * (-rate * dt).exp();
        }
        price /= self.num_paths as f64;

        Ok(price)
    }
}
