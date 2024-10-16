use statrs::distribution::{ContinuousCDF, Normal};

pub fn cumulative_normal_distribution(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(x)
}
