use crate::distributions::{build, Geometric, Discrete, Distribution};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete as Y, DiscreteCDF};
use statrs::statistics::Distribution as X;

impl Geometric {
    pub fn new(p: f64) -> Result<Geometric> {
        Ok(Geometric { p, core: build(SR::Geometric::new(p))? })
    }
}

impl Distribution for Geometric {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ G({p})", p = self.p)
    }
}

impl Discrete for Geometric {
    fn pmf(&self, x: u64) -> f64 {
        self.core.pmf(x)
    }
    fn cdf(&self, x: u64) -> f64 {
        self.core.cdf(x)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ G(p), then
    // * E(X) = 1/p
    // * var(X) = (1-p)/p²
    // * PMF finds probability of winning for the first time on the
    //   xth try with win-rate p
    let dist = Geometric::new(0.05)?;
    float_eq!(dist.mean().unwrap(), 20);
    float_eq!(dist.variance().unwrap(), 380);
    float_eq!(dist.pmf(5), 0.0407253125);
    Ok(())
}
