use crate::distributions::{build, Discrete, Distribution, Poisson};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete as Y, DiscreteCDF};
use statrs::statistics::Distribution as X;

impl Poisson {
    pub fn new(l: f64) -> Result<Poisson> {
        Ok(Poisson { l, core: build(SR::Poisson::new(l))? })
    }
}

impl Distribution for Poisson {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ Poisson({l})", l = self.l)
    }
}

impl Discrete for Poisson {
    fn pmf(&self, x: u64) -> f64 {
        self.core.pmf(x)
    }
    fn cdf(&self, x: u64) -> f64 {
        self.core.cdf(x)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ Poisson(l), then
    // * E(X) = l
    // * var(X) = l
    let dist = Poisson::new(3.0)?;
    float_eq!(dist.mean().unwrap(), 3);
    float_eq!(dist.variance().unwrap(), 3);
    float_eq!(dist.pmf(0), 0.04978706836);
    float_eq!(dist.cdf(3), 0.64723188878);
    float_eq!(Poisson::new(8.0)?.pmf(6), 0.12213821545);
    Ok(())
}
