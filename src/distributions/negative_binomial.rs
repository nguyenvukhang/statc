use crate::prob::{AsProb};
use crate::r::r;
use crate::utils::Result;
use crate::Mode;

#[derive(PartialEq, Debug)]
pub struct NegativeBinomial {
    k: u64,
    p: f64,
}

fn diff(x: u64, k: u64) -> Result<u64> {
    x.checked_sub(k).ok_or("Invalid parameters".to_string())
}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<Self> {
        Ok(Self { p: f64::as_prob(p)?, k })
    }

    // core R calls
    fn eq(&self, x: u64) -> Result<f64> {
        if self.k > x {
            return Ok(0.0);
        }
        r(&format!("dnbinom({}, {}, {})", diff(x, self.k)?, self.k, self.p))
    }
    fn le(&self, x: u64) -> Result<f64> {
        if self.k > x {
            return Ok(0.0);
        }
        r(&format!("pnbinom({}, {}, {})", diff(x, self.k)?, self.k, self.p))
    }
    fn gt(&self, x: u64) -> Result<f64> {
        if self.k > x {
            return Ok(0.0);
        }
        r(&format!(
            "pnbinom({}, {}, {}, lower.tail=F)",
            diff(x, self.k)?,
            self.k,
            self.p
        ))
    }

    // deriative functions
    fn ge(&self, x: u64) -> Result<f64> {
        Ok(self.gt(x)? + self.eq(x)?)
    }
    fn lt(&self, x: u64) -> Result<f64> {
        Ok(self.le(x)? - self.eq(x)?)
    }
    pub fn run(&self, mode: Mode, x: u64) -> Result<f64> {
        use Mode::*;
        match mode {
            Eq => self.eq(x),
            Le => self.le(x),
            Lt => self.lt(x),
            Ge => self.ge(x),
            Gt => self.gt(x),
        }
    }
}

#[test]
fn negative_binomial_eq_test() -> Result<()> {
    use crate::utils::err;
    assert_eq!(NegativeBinomial::new(10, 0.2)?.eq(4)?, 0.0);
    assert_eq!(NegativeBinomial::new(2, 0.3)?.eq(4)?, 0.1323);
    assert_eq!(NegativeBinomial::new(2, 1.0)?.eq(3)?, 0.0);
    assert_eq!(NegativeBinomial::new(2, 0.0)?.eq(9), err("Parsed a NaN value"));
    Ok(())
}

#[test]
fn negative_binomial_le_test() -> Result<()> {
    assert_eq!(NegativeBinomial::new(1, 0.2)?.le(4)?, 0.5904);
    Ok(())
}

#[test]
fn negative_binomial_gt_test() -> Result<()> {
    assert_eq!(NegativeBinomial::new(1, 0.2)?.gt(4)?, 0.4096);
    Ok(())
}
