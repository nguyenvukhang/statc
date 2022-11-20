use crate::analyze::{cdf_intervals, pdf_points};
use crate::distributions::{Continuous, Discrete, Distribution};
use crate::display::Analysis;

pub trait Summary<T> {
    fn analyze(&self, values: &Vec<T>) -> Analysis;
}

impl<D: Distribution + Discrete> Summary<u64> for D {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            pdf_eval: pdf_points(values, |v| self.pmf(v), true),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
            title: self.title(),
        }
    }
}

impl<D: Distribution + Continuous> Summary<f64> for D {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            pdf_eval: pdf_points(values, |v| self.pdf(v), false),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
            title: self.title(),
        }
    }
}
