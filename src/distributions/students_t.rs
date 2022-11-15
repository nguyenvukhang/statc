use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// continuous distribution
pub struct StudentsT {}

impl StudentsT {
    pub fn new(f: u64) -> Result<SR::StudentsT> {
        SR::StudentsT::new(0.0, 1.0, f as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::StudentsT {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            header: self.header(),
            pdf_eval: pdf_points(
                values,
                |v| self.pdf(v),
                INFO.students_t.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn header(&self) -> String {
        format!("X ~ t({f})", f = self.freedom())
    }
}
