#[derive(Debug, Clone)]
pub struct InterestRateCurve {
    pub times: Vec<f64>,
    pub rates: Vec<f64>,
}

impl InterestRateCurve {
    pub fn new(times: Vec<f64>, rates: Vec<f64>) -> Self {
        InterestRateCurve { times, rates }
    }
}
