use crate::prob::AsProb;
use crate::r::r;
use crate::utils::Result;
use crate::Mode;

#[derive(PartialEq, Debug)]
pub struct Binomial {
    n: u64,
    p: f64,
}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<Self> {
        Ok(Self { p: f64::as_prob(p)?, n })
    }

    // core R calls
    fn eq(&self, x: u64) -> Result<f64> {
        r(&format!("dbinom({x}, {}, {})", self.n, self.p))
    }
    fn le(&self, x: u64) -> Result<f64> {
        r(&format!("pbinom({x}, {}, {})", self.n, self.p))
    }
    fn gt(&self, x: u64) -> Result<f64> {
        r(&format!("pbinom({x}, {}, {}, lower.tail=F)", self.n, self.p))
    }

    // derivative functions
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
fn binomial_eq_test() -> Result<()> {
    assert_eq!(Binomial::new(10, 0.2)?.eq(4)?, 0.088080384);
    assert_eq!(Binomial::new(7, 0.3)?.eq(4)?, 0.0972405);
    assert_eq!(Binomial::new(7, 1.0)?.eq(3)?, 0.0);
    assert_eq!(Binomial::new(7, 0.0)?.eq(9)?, 0.0);
    Ok(())
}

#[test]
fn binomial_le_test() -> Result<()> {
    assert_eq!(Binomial::new(10, 0.2)?.le(4)?, 0.9672065024);
    Ok(())
}

#[test]
fn binomial_gt_test() -> Result<()> {
    assert_eq!(Binomial::new(10, 0.2)?.gt(4)?, 0.0327934976);
    Ok(())
}
