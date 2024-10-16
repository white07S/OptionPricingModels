use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptionPricingError {
    #[error("Invalid input parameter: {0}")]
    InvalidInput(String),

    #[error("Computation error: {0}")]
    ComputationError(String),

    #[error("Regression error: {0}")]
    RegressionError(String),

    #[error("Interpolation error: {0}")]
    InterpolationError(String),
}
