use crate::distributions::{build, MyContinuous, MyDist, Normal};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

impl Normal {
    pub fn new(m: f64, s: f64) -> Result<Normal> {
        let core = build(SR::Normal::new(m, s))?;
        Ok(Normal { m, s, core })
    }
}

impl MyDist for Normal {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ N({m}, {s}²)", m = self.m, s = self.s)
    }
}

impl MyContinuous for Normal {
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
    // if X ~ N(m, s²), then
    // * E(X) = m
    // * var(X) = s²
    let dist = Normal::new(10.0, 5.0)?;
    float_eq!(dist.mean().unwrap(), 10);
    float_eq!(dist.variance().unwrap(), 25);
    float_eq!(dist.cdf(8.0), 0.344578258389);
    Ok(())
}
