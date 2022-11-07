use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::{Distribution, Max, Min};

/// continuous distribution
pub struct Uniform {}

impl Uniform {
    pub fn new(a: f64, b: f64) -> Result<SR::Uniform> {
        SR::Uniform::new(a, b).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Uniform {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: pdf_points(values, |v| self.pdf(v), false),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn display(&self) -> String {
        let (a, b) = (self.min(), self.max());
        format!("X ~ U({a}, {b})")
    }
}
