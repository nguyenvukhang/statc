use crate::utils::{err, Result};

pub trait AsProb {
    fn as_prob(val: f64) -> Result<f64>;
}

impl AsProb for f64 {
    fn as_prob(val: f64) -> Result<f64> {
        match val.is_sign_negative() || val.gt(&1.0) {
            true => err(&format!("Invalid probability: {:?}", val)),
            false => Ok(val),
        }
    }
}

#[test]
fn probability_init_test() -> Result<()> {
    use crate::utils::err;
    assert_eq!(f64::as_prob(0.0), Ok(0.0));
    assert_eq!(f64::as_prob(1.0), Ok(1.0));
    assert_eq!(f64::as_prob(0.3141592653589793), Ok(0.3141592653589793));
    assert_eq!(f64::as_prob(-0.0), err("Invalid probability: -0.0"));
    assert_eq!(f64::as_prob(-0.1), err("Invalid probability: -0.1"));
    Ok(())
}
