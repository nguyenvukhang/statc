use crate::math::{choose, MathOps};
use crate::prob::AsProb;
use crate::types::Analysis;
use crate::types::Distribution;
use crate::utils::Result;

#[derive(PartialEq, Debug)]
pub struct Binomial {
    n: u64,
    p: f64,
    x: Option<u64>,
}

/// X ~ B(n, p)
///
/// returns P(X = x)
fn binomial_pdf(n: u64, p: f64, x: u64) -> f64 {
    if x > n {
        return 0.0;
    }
    p.pow(x) * (1.0 - p).pow(n - x) * choose(n, x) as f64
}

#[test]
fn binomial_pdf_test() -> Result<()> {
    assert_eq!(binomial_pdf(10, 0.2, 4), 0.08808038400000258);
    assert_eq!(binomial_pdf(7, 0.3, 4), 0.09724049999999994);
    assert_eq!(binomial_pdf(7, 1.0, 4), 0.0);
    assert_eq!(binomial_pdf(7, 0.0, 4), 0.0);
    Ok(())
}

/// X ~ B(n, p)
///
/// returns P(X <= x)
fn binomial_cdf(n: u64, p: f64, x: u64) -> f64 {
    (0..x + 1).fold(0.0, |acc, i| acc + binomial_pdf(n, p, i))
}

#[test]
fn binomial_cdf_test() -> Result<()> {
    assert_eq!(binomial_cdf(10, 0.2, 4), 0.9672065024000038);
    assert_eq!(binomial_cdf(7, 0.7, 2), 0.02879549999999984);
    Ok(())
}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<Self> {
        Ok(Self { p: f64::as_prob(p)?, n, x: None })
    }
    pub fn load(&self, x: Option<u64>) -> Self {
        Self { p: self.p, n: self.n, x }
    }
    pub fn pdf(&self, x: u64) -> f64 {
        binomial_pdf(self.n, self.p, x)
    }
    pub fn cdf(&self, x: u64) -> f64 {
        binomial_cdf(self.n, self.p, x)
    }
    pub fn display(&self) -> String {
        let (n, p, x) = (self.n, self.p, self.x);
        match x {
            Some(x) => format!("X ~ B({n}, {p}), x = {x}"),
            None => format!("X ~ B({n}, {p})"),
        }
    }
}

impl Distribution for Binomial {
    fn expected(&self) -> f64 {
        self.n as f64 * self.p
    }

    fn variance(&self) -> f64 {
        self.expected() * (1.0 - self.p)
    }

    fn analyze(&self) -> Analysis {
        Analysis {
            expected: self.expected(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: self.x.map(|x| self.pdf(x)),
            cdf_eval: self.x.map(|x| self.cdf(x)),
        }
    }
}
