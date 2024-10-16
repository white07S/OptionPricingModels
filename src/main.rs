use option_pricing_lib::data::InterestRateCurve;
use option_pricing_lib::models::heston::HestonModel;
use option_pricing_lib::regression::RegressionMethod;
use option_pricing_lib::traits::OptionPricingModel;
use option_pricing_lib::OptionType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interest_rate_curve = InterestRateCurve::new(vec![0.0, 1.0], vec![0.05, 0.05]);

    let heston_model = HestonModel {
        option_type: OptionType::Put,
        spot_price: 100.0,
        strike_price: 100.0,
        time_to_expiry: 1.0,
        initial_variance: 0.04,
        risk_free_rate_curve: interest_rate_curve,
        kappa: 2.0,
        theta: 0.04,
        sigma: 0.1,
        rho: -0.7,
        is_american: true,
        regression_method: RegressionMethod::LeastSquaresMonteCarlo,
        num_paths: 10000,
        num_steps: 50,
    };

    let price = heston_model.price()?;
    println!("Heston Model American Option Price: {:.4}", price);

    Ok(())
}
