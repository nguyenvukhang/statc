use crate::distributions::INFO;
use crate::types::{Analysis, Summary};
use crate::utils::{cdf_intervals, pdf_points, Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

/// discrete distribution
pub struct Binomial {}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<SR::Binomial> {
        SR::Binomial::new(p, n).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Binomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            header: self.header(),
            pdf_eval: pdf_points(
                values,
                |v| self.pmf(v),
                INFO.binomial.discrete,
            ),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
        }
    }

    fn header(&self) -> String {
        let (n, p) = (self.n(), self.p());
        format!("X ~ B({n}, {p})")
    }
}

#[test]
fn binomial_test() -> Result<()> {
    use crate::math::Round;
    let dist = Binomial::new(5, 0.2)?;
    assert_eq!(dist.pmf(1).roundn(10), 0.4096);
    let dist = Binomial::new(6, 2.0 / 3.0)?;
    assert_eq!(dist.pmf(0).roundn(10), 0.0013717421);
    Ok(())
}
