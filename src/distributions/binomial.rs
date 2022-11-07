use crate::types::Analysis;
use crate::types::Summary;
use crate::utils::{Result, ResultOps};
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

pub struct Binomial {}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<SR::Binomial> {
        SR::Binomial::new(p, n).serr("Bad parameters.")
    }
}

impl Summary for SR::Binomial {
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
        let (n, p) = (self.n(), self.p());
        match x {
            Some(x) => format!("X ~ B({n}, {p}), x = {x}"),
            None => format!("X ~ B({n}, {p})"),
        }
    }
}
