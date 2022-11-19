use crate::distributions::{build, MyDiscrete, MyDist, NegativeBinomial};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<NegativeBinomial> {
        let core = build(SR::NegativeBinomial::new(k as f64, p))?;
        Ok(NegativeBinomial { k, p, core })
    }
}

impl MyDist for NegativeBinomial {
    fn mean(&self) -> Option<f64> {
        Some(self.k as f64 / self.p)
    }
    fn variance(&self) -> Option<f64> {
        let (k, p) = (self.k as f64, self.p);
        Some((1.0 - p) * k / p / p)
    }
    fn title(&self) -> String {
        format!("X ~ NB({k}, {p})", k = self.k, p = self.p)
    }
}

impl MyDiscrete for NegativeBinomial {
    fn pmf(&self, x: u64) -> f64 {
        self.core.pmf(x - self.k)
    }
    fn cdf(&self, x: u64) -> f64 {
        self.core.cdf(x - self.k)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ NB(k, p), then
    // * E(X) = k/p
    // * var(X) = (1-p)k/p²
    // * PMF finds probability of winning for the kth time on the
    //   xth trial with win-rate p.
    let dist = NegativeBinomial::new(6, 1.0 / 6.0)?;
    float_eq!(dist.mean().unwrap(), 36);
    float_eq!(dist.variance().unwrap(), 180);
    float_eq!(dist.pmf(10), 0.00130238102);

    let dist = NegativeBinomial::new(4, 0.55)?;
    float_eq!(dist.pmf(6), 0.18530015624);
    Ok(())
}
