use crate::types::Analysis;
use crate::types::Summary;
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;
pub struct Poisson {}

impl Poisson {
    pub fn new(l: f64) -> Result<SR::Poisson> {
        SR::Poisson::new(l).serr("Bad parameters.")
    }
}

impl Summary for SR::Poisson {
    fn analyze(&self, x: Option<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            display: self.display(x),
            pdf_eval: x.map(|x| self.pmf(x)),
            cdf_eval: x.map(|x| self.cdf(x)),
        }
    }

    fn display(&self, x: Option<u64>) -> String {
        let l = self.lambda();
        match x {
            Some(x) => format!("X ~ Poisson({l}), x = {x}"),
            None => format!("X ~ Poisson({l})"),
        }
    }
}
