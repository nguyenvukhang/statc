use crate::analyze::Analyze;
use crate::distributions::{build, Geometric, MyDiscrete, MyDist};
use crate::types::{Analysis, Summary};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

impl Geometric {
    pub fn new(p: f64) -> Result<Geometric> {
        Ok(Geometric { p, core: build(SR::Geometric::new(p))? })
    }
}

impl Summary<u64> for Geometric {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(&self.core, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ G({p})", p = self.p)
    }
}

impl MyDist for Geometric {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
}

impl MyDiscrete for Geometric {
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
    let binom = Geometric::new(0.05)?;
    float_eq!(binom.mean().unwrap(), 20);
    float_eq!(binom.variance().unwrap(), 380);
    float_eq!(binom.pmf(5), 0.0407253125);
    Ok(())
}
