use crate::math::{range, MathOps};
use crate::types::Analysis;
use crate::types::Distribution;
use crate::utils::{err, Result};

#[derive(PartialEq, Debug)]
pub struct Poisson {
    l: f64,
    x: Option<u64>,
}

/// X ~ Poisson(n, p) -> P(X = x)
fn pdf(l: f64, mut x: u64) -> f64 {
    let mut res = std::f64::consts::E.powf(-l) * l.pow(x);
    while x > 1 {
        res = res / x as f64;
        x -= 1;
    }
    res
}

/// X ~ Poisson(n, p) -> P(X <= x)
fn cdf(l: f64, x: u64) -> f64 {
    range(0, x + 1, |i| pdf(l, i))
}

impl Poisson {
    pub fn new(l: f64, x: Option<u64>) -> Result<Self> {
        match l.gt(&0.0) {
            true => Ok(Self { l: l, x }),
            false => err("Use l > 0 for Poission."),
        }
    }

    fn display(&self) -> String {
        let (l, x) = (self.l, self.x);
        match x {
            Some(x) => format!("X ~ Poisson({l}), x = {x}"),
            None => format!("X ~ Poisson({l})"),
        }
    }
}

impl Distribution for Poisson {
    fn expected(&self) -> f64 {
        self.l
    }

    fn variance(&self) -> f64 {
        self.l
    }

    fn analyze(&self) -> Analysis {
        Analysis {
            expected: self.expected(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: self.x.map(|x| pdf(self.l, x)),
            cdf_eval: self.x.map(|x| cdf(self.l, x)),
        }
    }
}

#[test]
fn pdf_test() -> Result<()> {
    assert_eq!(pdf(4.3, 3), 0.1797992368971766);
    assert_eq!(pdf(0.3, 4), 0.00025002614948007966);
    Ok(())
}

#[test]
fn cdf_test() -> Result<()> {
    use crate::r::r_debug;
    let (l, x) = (0.7, 2);
    r_debug(&format!("ppois({x}, {l})"));
    assert_eq!(cdf(0.3, 4), 0.9999842149594583);
    assert_eq!(cdf(0.7, 2), 0.9658584158742917);
    Ok(())
}

#[test]
fn exp_test() -> Result<()> {
    assert_eq!(Poisson::new(0.2, Some(3))?.expected(), 0.2);
    assert_eq!(Poisson::new(0.4, Some(7))?.expected(), 0.4);
    Ok(())
}

#[test]
fn var_test() -> Result<()> {
    assert_eq!(Poisson::new(0.2, Some(3))?.variance(), 0.2);
    assert_eq!(Poisson::new(0.4, Some(7))?.variance(), 0.4);
    Ok(())
}
