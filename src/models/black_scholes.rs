use crate::errors::OptionPricingError;
use crate::traits::OptionPricingModel;
use crate::utils::cumulative_normal_distribution;
use crate::OptionType;

pub struct BlackScholesModel {
    pub option_type: OptionType,
    pub spot_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
}

impl OptionPricingModel for BlackScholesModel {
    fn price(&self) -> Result<f64, OptionPricingError> {
        if self.time_to_expiry <= 0.0 {
            return Err(OptionPricingError::InvalidInput(
                "Time to expiry must be positive.".to_string(),
            ));
        }

        let d1 = ((self.spot_price / self.strike_price).ln()
            + (self.risk_free_rate + 0.5 * self.volatility.powi(2)) * self.time_to_expiry)
            / (self.volatility * self.time_to_expiry.sqrt());

        let d2 = d1 - self.volatility * self.time_to_expiry.sqrt();

        let nd1 = cumulative_normal_distribution(match self.option_type {
            OptionType::Call => d1,
            OptionType::Put => -d1,
        });

        let nd2 = cumulative_normal_distribution(match self.option_type {
            OptionType::Call => d2,
            OptionType::Put => -d2,
        });

        let present_value_strike = self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp();

        let price = match self.option_type {
            OptionType::Call => self.spot_price * nd1 - present_value_strike * nd2,
            OptionType::Put => present_value_strike * nd2 - self.spot_price * nd1,
        };

        Ok(price)
    }
}
