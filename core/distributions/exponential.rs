use crate::distributions::{build, Exponential, MyContinuous, MyDist};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

impl Exponential {
    pub fn new(l: f64) -> Result<Exponential> {
        Ok(Exponential { l, core: build(SR::Exp::new(l))? })
    }
}

impl MyDist for Exponential {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ Exp({l})", l = self.l)
    }
}

impl MyContinuous for Exponential {
    fn pdf(&self, x: f64) -> f64 {
        self.core.pdf(x)
    }
    fn cdf(&self, x: f64) -> f64 {
        self.core.cdf(x)
    }
    fn inv_cdf(&self, x: f64) -> f64 {
        self.core.inverse_cdf(x)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ Exp(l), then
    // * E(X) = 1/l
    // * var(X) = 1/l²
    let dist = Exponential::new(0.2)?;
    float_eq!(dist.mean().unwrap(), 5);
    float_eq!(dist.variance().unwrap(), 25);
    float_eq!(dist.cdf(8.0), 0.798103482005);
    Ok(())
}
