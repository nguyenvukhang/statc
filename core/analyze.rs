use crate::display::Analysis;
use crate::display::Line;
use crate::distributions::{Continuous, Discrete, Distribution};
use std::fmt::Display;

pub trait Analyze<T> {
    fn analyze(&self, values: &Vec<T>) -> Analysis;
}

impl<D: Distribution + Discrete> Analyze<u64> for D {
    fn analyze(&self, values: &Vec<u64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            pdf_eval: lines(values, |v| self.pmf(v), Format::pmf),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
            title: self.title(),
        }
    }
}

impl<D: Distribution + Continuous> Analyze<f64> for D {
    fn analyze(&self, values: &Vec<f64>) -> Analysis {
        Analysis {
            expected: self.mean(),
            variance: self.variance(),
            pdf_eval: lines(values, |v| self.pdf(v), Format::pdf),
            cdf_eval: cdf_intervals(values, |v| self.cdf(v)),
            title: self.title(),
        }
    }
}

/// Generally, T is expected to be either u64 or f64
/// math_fn takes T and maps them to floating-point values
/// fmt_fn takes T and maps them to descriptions
pub fn lines<T: Display + Copy, M: Fn(T) -> f64, F: Fn(T) -> String>(
    list: &Vec<T>,
    math_fn: M,
    fmt_fn: F,
) -> Vec<Line> {
    list.iter().map(|v| Line::new(&fmt_fn(*v), Some(math_fn(*v)))).collect()
}

struct Format {}

/// formatters compatible with lines()
impl Format {
    /// Takes an input value x and returns a string
    /// "pdf @ {x}"
    pub fn pdf<T: Display>(x: T) -> String {
        format!("pdf @ {x}")
    }

    /// Takes an input value x and returns a string
    /// "P(X = {x})"
    pub fn pmf<T: Display>(x: T) -> String {
        format!("P(X = {x})")
    }
}

/// Takes a list of n points supplied by the user
/// and maps them to n + 1 intervals calculated by c.d.f.
/// * starts from P(X <= first element)
/// * ends at P(X > last element)
pub fn cdf_intervals<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    cdf: F,
) -> Vec<Line> {
    let mut result = vec![];

    // descriptions for first/mid/last ranges
    // lb: left bound, rb: right bound
    let first = |rb: &T| format!("P(X <= {rb})");
    let mid = |lb: &T, rb: &T| format!("P({lb} < X <= {rb})");
    let last = |lb: &T| format!("P(X > {lb})");

    // return empty list for an empty list
    if list.is_empty() {
        return result;
    };

    // first element: calculate P(X <= x)
    if let Some(x) = list.first() {
        result.push(Line::new(&first(x), Some(cdf(*x))))
    }

    let mut iter = list.iter().peekable();
    while let Some(lb) = iter.next() {
        let (desc, val) = match iter.peek() {
            // middle element: calculate P(left < X <= right)
            Some(rb) => (mid(lb, rb), cdf(**rb) - cdf(*lb)),
            // last element: calculate P(X > x)
            None => (last(lb), 1.0 - cdf(*lb)),
        };
        result.push(Line::new(&desc, Some(val)));
    }
    return result;
}
