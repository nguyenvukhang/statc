use crate::types::Analysis;
use crate::types::{Summary, PEval};
use crate::utils::{Result, ResultOps};

use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::DiscreteDistribution;

pub struct NegativeBinomial {}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<SR::NegativeBinomial> {
        SR::NegativeBinomial::new(k as f64, p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::NegativeBinomial {
    fn analyze(&self, x: Option<u64>, y: Option<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(x, y),
            pdf_eval: PEval::new("P(X = x)", x.map(|x| self.pmf(x))),
            cdf_eval: PEval::new("P(X <= x)", x.map(|x| self.cdf(x))),
        }
    }

    fn display(&self, x: Option<u64>, _y: Option<u64>) -> String {
        let (k, p) = (self.r(), self.p());
        match x {
            Some(x) => format!("X ~ NB({k}, {p}), x = {x}"),
            None => format!("X ~ NB({k}, {p})"),
        }
    }
}
