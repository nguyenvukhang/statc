use crate::types::Analysis;
use crate::types::{PEval, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::*;

pub struct Exponential {}

impl Exponential {
    pub fn new(l: f64) -> Result<SR::Exp> {
        SR::Exp::new(l).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Exp {
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
        let l = self.rate();
        match (x, y) {
            (Some(x), Some(y)) => format!("X ~ Exp({l}), x in [{x}, {y}]"),
            (Some(x), None) => format!("X ~ Exp({l}), x = {x}"),
            _ => format!("X ~ Exp({l})"),
        }
    }
}
