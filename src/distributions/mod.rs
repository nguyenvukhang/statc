use crate::analyze::Analyze;
use crate::types::{Analysis, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution::{self as SR};
use statrs::statistics::{Distribution, Max, Min};

mod binomial;
mod geometric;
mod negatve_binomial;
mod poisson;

// discrete distributions
pub struct Binomial {
    core: SR::Binomial,
    /// number of trials
    pub n: u64,
    /// win-rate
    pub p: f64,
}
pub struct NegativeBinomial {
    core: SR::NegativeBinomial,
    /// required number of wins
    pub k: u64,
    /// win-rate
    pub p: f64,
}
pub struct Geometric {
    core: SR::Geometric,
    /// win-rate
    pub p: f64,
}
pub struct Poisson {
    core: SR::Poisson,
    /// expected
    pub l: f64,
}
// continuous distributions
pub struct Uniform {}
pub struct Exponential {}
pub struct Normal {}
pub struct StudentsT {}
pub struct ChiSquared {}
pub struct FisherSnedecor {}

trait MyDist {
    fn mean(&self) -> Option<f64>;
    fn variance(&self) -> Option<f64>;
}

trait MyDiscrete {
    fn pmf(&self, x: u64) -> f64;
    fn cdf(&self, x: u64) -> f64;
}

trait MyContinuous {
    fn pdf(&self, x: u64) -> f64;
    fn cdf(&self, x: u64) -> f64;
}

fn build<T>(r: std::result::Result<T, statrs::StatsError>) -> Result<T> {
    r.map_err(|v| v.to_string())
}

impl Summary<u64> for SR::Poisson {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ Poisson({l})", l = self.lambda())
    }
}

impl Uniform {
    pub fn new(a: f64, b: f64) -> Result<SR::Uniform> {
        SR::Uniform::new(a, b).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Uniform {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ U({a}, {b})", a = self.min(), b = self.max())
    }
}

impl Exponential {
    pub fn new(l: f64) -> Result<SR::Exp> {
        SR::Exp::new(l).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Exp {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ Exp({l})", l = self.rate())
    }
}

impl Normal {
    pub fn new(m: f64, s: f64) -> Result<SR::Normal> {
        SR::Normal::new(m, s).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::Normal {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        let u = |v: Option<f64>| v.map(|x| x.to_string()).unwrap_or("_".into());
        format!("X ~ N({m}, {s}²)", m = u(self.mean()), s = u(self.std_dev()))
    }
}

impl StudentsT {
    pub fn new(f: u64) -> Result<SR::StudentsT> {
        SR::StudentsT::new(0.0, 1.0, f as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::StudentsT {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ t({f})", f = self.freedom())
    }
}

impl ChiSquared {
    pub fn new(n: u64) -> Result<SR::ChiSquared> {
        SR::ChiSquared::new(n as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::ChiSquared {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ χ²({n})", n = self.freedom())
    }
}

impl FisherSnedecor {
    pub fn new(m: u64, n: u64) -> Result<SR::FisherSnedecor> {
        SR::FisherSnedecor::new(m as f64, n as f64).serr("Bad parameters.")
    }
}

impl Summary<f64> for SR::FisherSnedecor {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analyze::continuous(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ F({m},{n})", m = self.freedom_1(), n = self.freedom_2())
    }
}
