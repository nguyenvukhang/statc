use crate::types::Analysis;
use crate::types::{PEval, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::*;

pub struct Normal {}

impl Normal {
    pub fn new(m: f64, s: f64) -> Result<SR::Normal> {
        SR::Normal::new(m, s).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Normal {
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
        let u = |v: Option<f64>| v.map(|x| x.to_string()).unwrap_or("_".into());
        let (m, s) = (u(self.mean()), u(self.std_dev()));
        match (x, y) {
            (Some(x), Some(y)) => format!("X ~ N({m}, {s}), x in [{x}, {y}]"),
            (Some(x), None) => format!("X ~ N({m}, {s}), x = {x}"),
            _ => format!("X ~ N({m}, {s})"),
        }
    }
}
