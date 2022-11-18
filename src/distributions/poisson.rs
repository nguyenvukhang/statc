use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

/// discrete distribution
pub struct Poisson {}

impl Poisson {
    pub fn new(l: f64) -> Result<SR::Poisson> {
        SR::Poisson::new(l).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Poisson {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            title: self.title(),
            pdf_eval: pdf_points(
                values,
                |v| self.pmf(v),
                INFO.poisson.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn title(&self) -> String {
        let l = self.lambda();
        format!("X ~ Poisson({l})")
    }
}
