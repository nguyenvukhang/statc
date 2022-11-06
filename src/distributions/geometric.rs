use crate::math::range;
use crate::math::MathOps;
use crate::prob::AsProb;
use crate::types::Analysis;
use crate::types::Distribution;
use crate::utils::Result;

#[derive(PartialEq, Debug)]
pub struct Geometric {
    p: f64,
    x: Option<u64>,
}

/// X ~ G(p) -> P(X = x)
///
/// Note that this variant of the Geometric Distribution finds the
/// probability of winning for the first time on the (x + 1)th try.
///
/// This is in alignment with R's dgeom(x, p) function.
fn pdf(p: f64, x: u64) -> f64 {
    p * (1.0 - p).pow(x)
}

/// X ~ G(p) -> P(X <= x)
fn cdf(p: f64, x: u64) -> f64 {
    range(0, x + 1, |i| pdf(p, i))
}

impl Geometric {
    pub fn new(p: f64, x: Option<u64>) -> Result<Self> {
        Ok(Self { p: f64::as_prob(p)?, x })
    }

    fn display(&self) -> String {
        let (p, x) = (self.p, self.x);
        match x {
            Some(x) => format!("X ~ G({p}), x = {x}"),
            None => format!("X ~ G({p})"),
        }
    }
}

impl Distribution for Geometric {
    fn expected(&self) -> f64 {
        1.0 / self.p
    }

    fn variance(&self) -> f64 {
        (1.0 - self.p) / self.p.pow(2)
    }

    fn analyze(&self) -> Analysis {
        Analysis {
            expected: self.expected(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: self.x.map(|x| pdf(self.p, x)),
            cdf_eval: self.x.map(|x| cdf(self.p, x)),
        }
    }
}

#[test]
fn pdf_test() -> Result<()> {
    assert_eq!(pdf(0.3, 0), 0.3);
    assert_eq!(pdf(0.3, 2), 0.147);
    assert_eq!(pdf(0.7, 6), 0.0005102999999999954);
    Ok(())
}

#[test]
fn cdf_test() -> Result<()> {
    assert_eq!(cdf(0.7, 6), 0.9997812999999999);
    assert_eq!(cdf(0.3, 9), 0.9717524751);
    Ok(())
}

#[test]
fn exp_test() -> Result<()> {
    assert_eq!(Geometric::new(0.2, Some(3))?.expected(), 5.0);
    assert_eq!(Geometric::new(0.4, Some(7))?.expected(), 2.5);
    Ok(())
}

#[test]
fn var_test() -> Result<()> {
    assert_eq!(Geometric::new(0.2, Some(3))?.variance(), 19.999999999999982);
    assert_eq!(Geometric::new(0.4, Some(7))?.variance(), 3.749999999999999);
    Ok(())
}
