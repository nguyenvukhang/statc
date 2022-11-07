use crate::types::Analysis;
use crate::types::{PEval, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

pub struct Geometric {}

impl Geometric {
    pub fn new(p: f64) -> Result<SR::Geometric> {
        SR::Geometric::new(p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Geometric {
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
        let p = self.p();
        match x {
            Some(x) => format!("X ~ G({p}), x = {x}"),
            None => format!("X ~ G({p})"),
        }
    }
}
