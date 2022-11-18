use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};

/// discrete distribution
pub struct NegativeBinomial {}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<SR::NegativeBinomial> {
        SR::NegativeBinomial::new(k as f64, p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::NegativeBinomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        let (k, p) = (self.r(), self.p());
        Analysis {
            expected: Some(k / p),
            variance: Some((1.0 - p) * k / p / p),
            title: self.title(),
            pdf_eval: pdf_points(
                values,
                |v| self.pmf(v - self.r() as u64),
                INFO.negative_binomial.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v - self.r() as u64)),
        }
    }

    fn title(&self) -> String {
        let (k, p) = (self.r(), self.p());
        format!("X ~ NB({k}, {p})")
    }
}
