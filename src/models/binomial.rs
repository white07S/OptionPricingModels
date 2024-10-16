use crate::errors::OptionPricingError;
use crate::traits::OptionPricingModel;
use crate::OptionType;

pub struct BinomialModel {
    pub option_type: OptionType,
    pub spot_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub steps: usize,
    pub is_american: bool,
}

impl OptionPricingModel for BinomialModel {
    fn price(&self) -> Result<f64, OptionPricingError> {
        if self.steps == 0 {
            return Err(OptionPricingError::InvalidInput(
                "Number of steps must be greater than zero.".to_string(),
            ));
        }

        let delta_t = self.time_to_expiry / self.steps as f64;
        let up = (self.volatility * delta_t.sqrt()).exp();
        let down = 1.0 / up;
        let discount = (-self.risk_free_rate * delta_t).exp();
        let p = (discount.recip() - down) / (up - down);

        let mut asset_prices = vec![0.0; self.steps + 1];
        let mut option_values = vec![0.0; self.steps + 1];

        // Compute asset prices at maturity
        for i in 0..=self.steps {
            let j = self.steps - i;
            asset_prices[i] = self.spot_price * up.powi(j as i32) * down.powi(i as i32);
            option_values[i] = match self.option_type {
                OptionType::Call => (asset_prices[i] - self.strike_price).max(0.0),
                OptionType::Put => (self.strike_price - asset_prices[i]).max(0.0),
            };
        }

        // Backward induction
        for step in (0..self.steps).rev() {
            for i in 0..=step {
                option_values[i] = discount * (p * option_values[i] + (1.0 - p) * option_values[i + 1]);

                if self.is_american {
                    let exercise_value = match self.option_type {
                        OptionType::Call => (asset_prices[i] - self.strike_price).max(0.0),
                        OptionType::Put => (self.strike_price - asset_prices[i]).max(0.0),
                    };
                    option_values[i] = option_values[i].max(exercise_value);
                }

                asset_prices[i] = asset_prices[i] * down / up;
            }
        }

        Ok(option_values[0])
    }
}
