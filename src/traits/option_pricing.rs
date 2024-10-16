use crate::errors::OptionPricingError;

pub trait OptionPricingModel {
    fn price(&self) -> Result<f64, OptionPricingError>;
}
