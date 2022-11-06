use crate::math::range;
use crate::math::{choose, MathOps};
use crate::prob::AsProb;
use crate::types::Analysis;
use crate::types::Distribution;
use crate::utils::Result;

#[derive(PartialEq, Debug)]
pub struct NegativeBinomial {
    k: u64,
    p: f64,
    x: Option<u64>,
}

/// X ~ NB(k, p) -> P(X = x)
fn pdf(k: u64, p: f64, x: u64) -> f64 {
    if x < k {
        return 0.0;
    }
    p.pow(k) * (1.0 - p).pow(x - k) * choose(x - 1, k - 1) as f64
}

/// X ~ NB(k, p) -> P(X <= x)
fn cdf(n: u64, p: f64, x: u64) -> f64 {
    range(0, x + 1, |i| pdf(n, p, i))
}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64, x: Option<u64>) -> Result<Self> {
        Ok(Self { p: f64::as_prob(p)?, k, x })
    }

    fn display(&self) -> String {
        let (k, p, x) = (self.k, self.p, self.x);
        match x {
            Some(x) => format!("X ~ NB({k}, {p}), x = {x}"),
            None => format!("X ~ NB({k}, {p})"),
        }
    }
}

impl Distribution for NegativeBinomial {
    fn expected(&self) -> f64 {
        self.k as f64 / self.p
    }

    fn variance(&self) -> f64 {
        self.k as f64 * (1.0 - self.p) / self.p.pow(2)
    }

    fn analyze(&self) -> Analysis {
        Analysis {
            expected: self.expected(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: self.x.map(|x| pdf(self.k, self.p, x)),
            cdf_eval: self.x.map(|x| cdf(self.k, self.p, x)),
        }
    }
}

#[test]
fn pdf_test() -> Result<()> {
    assert_eq!(pdf(2, 0.2, 5), 0.08192000000000009);
    assert_eq!(pdf(4, 0.7, 6), 0.21609000000000017);
    Ok(())
}

#[test]
fn cdf_test() -> Result<()> {
    // use crate::r::r_debug;
    // let (k, p, x) = (2, 0.3, 9);
    // let x = x - k;
    // r_debug(&format!("pnbinom({x}, {k}, {p})"));
    assert_eq!(cdf(4, 0.7, 6), 0.7443100000000002);
    assert_eq!(cdf(2, 0.3, 9), 0.8039967659999997);
    Ok(())
}

#[test]
fn exp_test() -> Result<()> {
    assert_eq!(NegativeBinomial::new(10, 0.2, Some(3))?.expected(), 50.0);
    assert_eq!(NegativeBinomial::new(10, 0.4, Some(7))?.expected(), 25.0);
    Ok(())
}

#[test]
fn var_test() -> Result<()> {
    assert_eq!(
        NegativeBinomial::new(10, 0.2, Some(3))?.variance(),
        199.99999999999983
    );
    assert_eq!(
        NegativeBinomial::new(10, 0.4, Some(7))?.variance(),
        37.49999999999999
    );
    Ok(())
}
