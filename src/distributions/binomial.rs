use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

/// discrete distribution
pub struct Binomial {}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<SR::Binomial> {
        SR::Binomial::new(p, n).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Binomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            header: self.header(),
            pdf_eval: pdf_points(values, |v| self.pmf(v), true),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn header(&self) -> String {
        let (n, p) = (self.n(), self.p());
        format!("X ~ B({n}, {p})")
    }
}
