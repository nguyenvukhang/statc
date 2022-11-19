use crate::utils::Result;
use statrs::distribution as SR;

// discrete
mod binomial;
mod geometric;
mod negatve_binomial;
mod poisson;

// continuous
mod chi_squared;
mod exponential;
mod fisher_snedecor;
mod normal;
mod students_t;
mod uniform;

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
pub struct Uniform {
    core: SR::Uniform,
    pub min: f64,
    pub max: f64,
}

pub struct Exponential {
    core: SR::Exp,
    l: f64,
}

pub struct Normal {
    core: SR::Normal,
    m: f64,
    s: f64,
}

pub struct StudentsT {
    core: SR::StudentsT,
    freedom: u64,
}

pub struct ChiSquared {
    core: SR::ChiSquared,
    freedom: u64,
}

pub struct FisherSnedecor {
    core: SR::FisherSnedecor,
    f1: u64,
    f2: u64,
}

pub trait MyDist {
    fn mean(&self) -> Option<f64>;
    fn variance(&self) -> Option<f64>;
    fn title(&self) -> String;
}

pub trait MyDiscrete {
    fn pmf(&self, x: u64) -> f64;
    fn cdf(&self, x: u64) -> f64;
}

pub trait MyContinuous {
    fn pdf(&self, x: f64) -> f64;
    fn cdf(&self, x: f64) -> f64;
    fn inv_cdf(&self, x: f64) -> f64;
}

fn build<T>(r: std::result::Result<T, statrs::StatsError>) -> Result<T> {
    r.map_err(|v| v.to_string())
}
