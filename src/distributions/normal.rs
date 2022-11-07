use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// continuous distribution
pub struct Normal {}

impl Normal {
    pub fn new(m: f64, s: f64) -> Result<SR::Normal> {
        SR::Normal::new(m, s).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Normal {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(),
            pdf_eval: pdf_points(values, |v| self.pdf(v), true),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn display(&self) -> String {
        let u = |v: Option<f64>| v.map(|x| x.to_string()).unwrap_or("_".into());
        let (m, s) = (u(self.mean()), u(self.std_dev()));
        format!("X ~ N({m}, {s})")
    }
}
