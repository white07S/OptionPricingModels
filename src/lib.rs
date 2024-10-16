pub mod errors;
pub mod traits;
pub mod utils;
pub mod data;
pub mod models;
pub mod regression;

pub use errors::OptionPricingError;
pub use traits::OptionPricingModel;
pub use models::intrinsic_value::IntrinsicValue;
pub use models::binomial::BinomialModel;
pub use models::black_scholes::BlackScholesModel;
pub use models::heston::HestonModel;
pub use data::interest_rates::InterestRateCurve;
pub use regression::RegressionMethod;

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}
