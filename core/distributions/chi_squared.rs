use crate::distributions::{build, ChiSquared, Continuous, Distribution};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous as Y, ContinuousCDF};
use statrs::statistics::Distribution as X;

impl ChiSquared {
    pub fn new(freedom: u64) -> Result<ChiSquared> {
        let core = build(SR::ChiSquared::new(freedom as f64))?;
        Ok(ChiSquared { freedom, core })
    }
}

impl Distribution for ChiSquared {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ χ²({n})", n = self.freedom)
    }
}

impl Continuous for ChiSquared {
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
    // if X ~ χ²(n), then
    // * E(X) = n
    // * var(X) = 2n

    let chisq = |v| ChiSquared::new(v).unwrap();
    float_eq!(chisq(8).mean().unwrap(), 8);
    float_eq!(chisq(8).variance().unwrap(), 16);
    float_eq!(chisq(9).cdf(1.2), 0.0011788966);

    Ok(())
}
