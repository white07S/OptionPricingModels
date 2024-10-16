use crate::errors::OptionPricingError;
use crate::regression::{Regression, RegressionDataPoint, RegressionInput, RegressionModel};
use smartcore::linalg::naive::dense_matrix::*;
use smartcore::tree::decision_tree_regressor::DecisionTreeRegressorParameters;
use smartcore::ensemble::random_forest_regressor::{RandomForestRegressor, RandomForestRegressorParameters};

pub struct RandomForestRegression {}

impl Regression for RandomForestRegression {
    fn fit(&self, data: &[RegressionDataPoint]) -> Result<Box<dyn RegressionModel>, OptionPricingError> {
        let n = data.len();

        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);

        for point in data {
            x.push(vec![point.asset_price]);
            y.push(point.continuation_value);
        }

        let x_matrix = DenseMatrix::from_2d_vec(&x);

        let params = RandomForestRegressorParameters::default();
        let model = RandomForestRegressor::fit(&x_matrix, &y, params).map_err(|e| {
            OptionPricingError::RegressionError(format!("Random Forest regression failed: {:?}", e))
        })?;

        Ok(Box::new(RandomForestModel { model }))
    }
}

pub struct RandomForestModel {
    pub model: RandomForestRegressor<f64>,
}

impl RegressionModel for RandomForestModel {
    fn predict(&self, input: &RegressionInput) -> f64 {
        let x = DenseMatrix::from_2d_vec(&[vec![input.asset_price]]);
        if let Ok(predictions) = self.model.predict(&x) {
            predictions[0]
        } else {
            0.0
        }
    }
}
