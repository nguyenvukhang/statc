use crate::analyze::Analyze;
use crate::distributions::{build, Binomial, MyDiscrete, MyDist};
use crate::types::{Analysis, Summary};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Discrete, DiscreteCDF};
use statrs::statistics::Distribution;

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<Binomial> {
        Ok(Binomial { n, p, core: build(SR::Binomial::new(p, n))? })
    }
}

impl Summary<u64> for Binomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(&self.core, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ B({n}, {p})", n = self.n, p = self.p)
    }
}

impl MyDist for Binomial {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
}

impl MyDiscrete for Binomial {
    fn pmf(&self, x: u64) -> f64 {
        self.core.pmf(x)
    }
    fn cdf(&self, x: u64) -> f64 {
        self.core.cdf(x)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ B(n, p), then
    // * E(X) = np
    // * var(X) = np(1−p)
    // * X is the number of successes in n Bernoulli trials of
    //   win-rate p.
    let binom = Binomial::new(10, 0.5)?;
    float_eq!(binom.mean().unwrap(), 5.0);
    float_eq!(binom.variance().unwrap(), 2.5);
    float_eq!(binom.pmf(6), 0.205078125);

    let binom = Binomial::new(10, 0.2)?;
    float_eq!(binom.mean().unwrap(), 2);
    float_eq!(binom.variance().unwrap(), 1.6);
    float_eq!(binom.pmf(2), 0.301989888);
    float_eq!(binom.cdf(4), 0.9672065024);
    Ok(())
}
