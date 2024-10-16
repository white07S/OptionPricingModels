use crate::data::InterestRateCurve;
use crate::errors::OptionPricingError;

pub fn interpolate_rate(curve: &InterestRateCurve, time: f64) -> Result<f64, OptionPricingError> {
    let times = &curve.times;
    let rates = &curve.rates;

    if times.is_empty() || rates.is_empty() || times.len() != rates.len() {
        return Err(OptionPricingError::InvalidInput(
            "Interest rate curve data is invalid.".to_string(),
        ));
    }

    if time <= times[0] {
        return Ok(rates[0]);
    }

    if time >= times[times.len() - 1] {
        return Ok(rates[rates.len() - 1]);
    }

    for i in 0..times.len() - 1 {
        if time >= times[i] && time <= times[i + 1] {
            let t0 = times[i];
            let t1 = times[i + 1];
            let r0 = rates[i];
            let r1 = rates[i + 1];

            let interpolated_rate = r0 + (r1 - r0) * (time - t0) / (t1 - t0);
            return Ok(interpolated_rate);
        }
    }

    Err(OptionPricingError::InterpolationError(
        "Failed to interpolate rate.".to_string(),
    ))
}
