use crate::distributions::{build, Distribution, Continuous, Uniform};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous as Y, ContinuousCDF};
use statrs::statistics::Distribution as X;

impl Uniform {
    pub fn new(min: f64, max: f64) -> Result<Uniform> {
        Ok(Uniform { min, max, core: build(SR::Uniform::new(min, max))? })
    }
}

impl Distribution for Uniform {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ U({a}, {b})", a = self.min, b = self.max)
    }
}

impl Continuous for Uniform {
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
    // if X ~ U(a, b), then
    // * E(X) = (a+b)/2
    // * var(X) = (b-a)²/12
    let dist = Uniform::new(2.0, 4.0)?;
    float_eq!(dist.mean().unwrap(), 3);
    float_eq!(dist.variance().unwrap(), 0.3333333333);
    float_eq!(dist.pdf(2.0), 0.5);
    float_eq!(dist.pdf(3.0), 0.5);
    float_eq!(dist.pdf(1.99), 0);
    float_eq!(dist.pdf(4.01), 0);
    Ok(())
}
