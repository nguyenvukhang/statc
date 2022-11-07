use crate::types::Analysis;
use crate::types::{PEval, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::*;

pub struct Uniform {}

impl Uniform {
    pub fn new(a: f64, b: f64) -> Result<SR::Uniform> {
        SR::Uniform::new(a, b).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Uniform {
    fn analyze(&self, x: Option<f64>, y: Option<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(x, y),
            pdf_eval: match y {
                Some(_) => None,
                None => PEval::new("p.d.f.", x.map(|x| self.pdf(x))),
            },
            cdf_eval: match (x, y) {
                (Some(x), Some(y)) => PEval::new(
                    &format!("P(X in [{x}, {y}])"),
                    Some(self.cdf(y) - self.cdf(x)),
                ),
                (Some(x), None) => {
                    PEval::new(&format!("P(X <= {x})"), Some(self.cdf(x)))
                }
                _ => None,
            },
        }
    }

    fn display(&self, x: Option<f64>, y: Option<f64>) -> String {
        let (a, b) = (self.min(), self.max());
        match (x, y) {
            (Some(x), Some(y)) => format!("X ~ U({a}, {b}), x in [{x}, {y}]"),
            (Some(x), None) => format!("X ~ U({a}, {b}), x = {x}"),
            _ => format!("X ~ U({a}, {b})"),
        }
    }
}
