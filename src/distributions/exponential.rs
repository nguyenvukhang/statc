use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// continuous distribution
pub struct Exponential {}

impl Exponential {
    pub fn new(l: f64) -> Result<SR::Exp> {
        SR::Exp::new(l).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Exp {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            title: self.title(),
            pdf_eval: pdf_points(
                values,
                |v| self.pdf(v),
                INFO.exponential.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn title(&self) -> String {
        let l = self.rate();
        format!("X ~ Exp({l})")
    }
}
