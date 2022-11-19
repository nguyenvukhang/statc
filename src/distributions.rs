use crate::analyze::{cdf_intervals, pdf_points, Analyze};
use crate::types::{Analysis, Summary};
use crate::utils::{Result, ResultOps};
use statrs::distribution::{self as SR, Discrete, DiscreteCDF};
use statrs::statistics::{Distribution, Max, Min};

// discrete distributions
pub struct Binomial {}
pub struct NegativeBinomial {}
pub struct Geometric {}
pub struct Poisson {}
// continuous distributions
pub struct Uniform {}
pub struct Exponential {}
pub struct Normal {}
pub struct StudentsT {}
pub struct ChiSquared {}
pub struct FisherSnedecor {}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<SR::Binomial> {
        SR::Binomial::new(p, n).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Binomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ B({n}, {p})", n = self.n(), p = self.p())
    }
}

impl NegativeBinomial {
    pub fn new(k: u64, p: f64) -> Result<SR::NegativeBinomial> {
        SR::NegativeBinomial::new(k as f64, p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::NegativeBinomial {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        let (k, p) = (self.r(), self.p());
        Analysis {
            expected: Some(k / p),
            variance: Some((1.0 - p) * k / p / p),
            title: self.title(),
            pdf_eval: pdf_points(values, |v| self.pmf(v - k as u64), true),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v - k as u64)),
        }
    }

    fn title(&self) -> String {
        format!("X ~ NB({k}, {p})", k = self.r(), p = self.p())
    }
}

impl Geometric {
    pub fn new(p: f64) -> Result<SR::Geometric> {
        SR::Geometric::new(p).serr("Bad parameters.")
    }
}

impl Summary<u64> for SR::Geometric {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analyze::discrete(self, values, self.title())
    }

    fn title(&self) -> String {
        format!("X ~ G({p})", p = self.p())
    }
}

impl Poisson {
    pub fn new(l: f64) -> Result<SR::Poisson> {
        SR::Poisson::new(l).serr("Bad parameters.")
    }
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
