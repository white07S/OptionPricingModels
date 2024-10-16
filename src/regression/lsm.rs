use crate::errors::OptionPricingError;
use crate::regression::{Regression, RegressionDataPoint, RegressionInput, RegressionModel};
use ndarray::Array2;
use ndarray_linalg::LeastSquaresSvd;
use std::collections::HashMap;

pub struct LeastSquaresMonteCarlo {}

impl Regression for LeastSquaresMonteCarlo {
    fn fit(&self, data: &[RegressionDataPoint]) -> Result<Box<dyn RegressionModel>, OptionPricingError> {
        let n = data.len();
        let degree = 2; // Degree of polynomial basis functions

        let mut x = Array2::<f64>::zeros((n, degree + 1));
        let mut y = Array2::<f64>::zeros((n, 1));

        for (i, point) in data.iter().enumerate() {
            let s = point.asset_price;
            for j in 0..=degree {
                x[[i, j]] = s.powi(j as i32);
            }
            y[[i, 0]] = point.continuation_value;
        }

        // Solve least squares regression
        let result = x.least_squares(&y).map_err(|e| {
            OptionPricingError::RegressionError(format!("LSM regression failed: {:?}", e))
        })?;

        let coeffs = result.solution.column(0).to_owned().to_vec();

        Ok(Box::new(LSMModel { coefficients: coeffs }))
    }
}

pub struct LSMModel {
    pub coefficients: Vec<f64>,
}

impl RegressionModel for LSMModel {
    fn predict(&self, input: &RegressionInput) -> f64 {
        let s = input.asset_price;
        let mut value = 0.0;
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            value += coeff * s.powi(i as i32);
        }
        value
    }
}
