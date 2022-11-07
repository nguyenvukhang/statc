use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::DiscreteDistribution;

/// discrete distribution
pub struct NegativeBinomial {}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<SR::NegativeBinomial> {
        SR::NegativeBinomial::new(k as f64, p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::NegativeBinomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: pdf_points(values, |v| self.pmf(v), true),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn display(&self) -> String {
        let (k, p) = (self.r(), self.p());
        format!("X ~ NB({k}, {p})")
    }
}
