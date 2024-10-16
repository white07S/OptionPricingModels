use crate::errors::OptionPricingError;
use crate::traits::OptionPricingModel;
use crate::OptionType;

pub struct IntrinsicValue {
    pub option_type: OptionType,
    pub spot_price: f64,
    pub strike_price: f64,
}

impl OptionPricingModel for IntrinsicValue {
    fn price(&self) -> Result<f64, OptionPricingError> {
        match self.option_type {
            OptionType::Call => Ok((self.spot_price - self.strike_price).max(0.0)),
            OptionType::Put => Ok((self.strike_price - self.spot_price).max(0.0)),
        }
    }
}
