use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// continuous distribution
pub struct FisherSnedecor {}

impl FisherSnedecor {
    pub fn new(m: u64, n: u64) -> Result<SR::FisherSnedecor> {
        SR::FisherSnedecor::new(m as f64, n as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::FisherSnedecor {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            header: self.header(),
            pdf_eval: pdf_points(
                values,
                |v| self.pdf(v),
                INFO.fisher_snedecor.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn header(&self) -> String {
        format!("X ~ F({m},{n})", m = self.freedom_1(), n = self.freedom_2())
    }
}
