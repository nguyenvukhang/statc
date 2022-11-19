use crate::analyze::Analyze;
use crate::distributions::{build, MyDiscrete, MyDist, Poisson};
use crate::types::{Analysis, Summary};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

impl Poisson {
    pub fn new(l: f64) -> Result<Poisson> {
        Ok(Poisson { l, core: build(SR::Poisson::new(l))? })
    }
}

impl Summary<u64> for Poisson {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(&self.core, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ Poisson({l})", l = self.l)
    }
}

impl MyDist for Poisson {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
}

impl MyDiscrete for Poisson {
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
    let binom = Poisson::new(3.0)?;
    float_eq!(binom.mean().unwrap(), 3);
    float_eq!(binom.variance().unwrap(), 3);
    float_eq!(binom.pmf(0), 0.04978706836);
    float_eq!(binom.cdf(3), 0.64723188878);
    Ok(())
}
