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

/// X ~ NB(k, p)
///
/// returns P(X = x)
fn negative_binomial_pdf(k: u64, p: f64, x: u64) -> f64 {
    if x < k {
        return 0.0;
    }
    p.pow(k) * (1.0 - p).pow(x - k) * choose(x - 1, k - 1) as f64
}

#[test]
fn negative_binomial_pdf_test() -> Result<()> {
    assert_eq!(negative_binomial_pdf(2, 0.2, 5), 0.08192000000000009);
    assert_eq!(negative_binomial_pdf(4, 0.7, 6), 0.21609000000000017);
    Ok(())
}

/// X ~ NB(k, p)
///
/// returns P(X <= x)
fn negative_binomial_cdf(n: u64, p: f64, x: u64) -> f64 {
    range(0, x + 1, |i| negative_binomial_pdf(n, p, i))
}

#[test]
fn negative_binomial_cdf_test() -> Result<()> {
    // use crate::r::r_debug;
    // let (k, p, x) = (2, 0.3, 9);
    // let x = x - k;
    // r_debug(&format!("pnbinom({x}, {k}, {p})"));
    assert_eq!(negative_binomial_cdf(4, 0.7, 6), 0.7443100000000002);
    assert_eq!(negative_binomial_cdf(2, 0.3, 9), 0.8039967659999997);
    Ok(())
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
            pdf_eval: self.x.map(|x| negative_binomial_pdf(self.k, self.p, x)),
            cdf_eval: self.x.map(|x| negative_binomial_cdf(self.k, self.p, x)),
        }
    }
}
