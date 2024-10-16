pub mod lsm;
pub mod random_forest;

pub use lsm::LeastSquaresMonteCarlo;
pub use random_forest::RandomForestRegression;

use crate::errors::OptionPricingError;

pub enum RegressionMethod {
    LeastSquaresMonteCarlo,
    RandomForest,
}

pub trait Regression {
    fn fit(&self, data: &[RegressionDataPoint]) -> Result<Box<dyn RegressionModel>, OptionPricingError>;
}

pub trait RegressionModel {
    fn predict(&self, input: &RegressionInput) -> f64;
}

pub struct RegressionDataPoint {
    pub time: f64,
    pub asset_price: f64,
    pub continuation_value: f64,
}

pub struct RegressionInput {
    pub time: f64,
    pub asset_price: f64,
}
