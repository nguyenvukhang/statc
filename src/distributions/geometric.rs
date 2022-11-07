use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

/// discrete distribution
pub struct Geometric {}

impl Geometric {
    pub fn new(p: f64) -> Result<SR::Geometric> {
        SR::Geometric::new(p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Geometric {
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
        let p = self.p();
        format!("X ~ G({p})")
    }
}
