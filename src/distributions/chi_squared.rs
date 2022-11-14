use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// continuous distribution
pub struct ChiSquared {}

impl ChiSquared {
    pub fn new(n: u64) -> Result<SR::ChiSquared> {
        SR::ChiSquared::new(n as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::ChiSquared {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            header: self.header(),
            pdf_eval: pdf_points(
                values,
                |v| self.pdf(v),
                INFO.chi_squared.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn header(&self) -> String {
        format!("X ~ χ²({n})", n = self.freedom())
    }
}
