use crate::utils::{err, Result};

pub type P = f64; // Probability
pub struct Meta {
    pub expected: f64,
    pub variance: f64,
}

pub trait PWrap {
    fn new(val: P) -> Result<P>;
}

impl PWrap for P {
    fn new(val: P) -> Result<P> {
        match val.is_sign_negative() || val.gt(&1.0) {
            true => err(&format!("Invalid probability: {:?}", val)),
            false => Ok(val),
        }
    }
}

#[test]
fn probability_init_test() -> Result<()> {
    use crate::utils::err;
    assert_eq!(P::new(0.0), Ok(0.0));
    assert_eq!(P::new(1.0), Ok(1.0));
    assert_eq!(P::new(0.3141592653589793), Ok(0.3141592653589793));
    assert_eq!(P::new(-0.0), err("Invalid probability: -0.0"));
    assert_eq!(P::new(-0.1), err("Invalid probability: -0.1"));
    Ok(())
}
