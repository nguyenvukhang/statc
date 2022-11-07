use crate::types::PEval;
use std::fmt::Display;
use std::result as core;

pub type Result<T> = core::Result<T, String>;

pub fn err<T>(msg: &str) -> Result<T> {
    Err(msg.to_string())
}

pub trait ResultOps<T, E> {
    fn clear(self) -> core::Result<(), E>;
    fn serr(self, msg: &str) -> core::Result<T, String>;
}

impl<T, E> ResultOps<T, E> for core::Result<T, E> {
    fn clear(self) -> core::Result<(), E> {
        self.map(|_| ())
    }
    fn serr(self, msg: &str) -> core::Result<T, String> {
        self.map_err(|_| msg.to_string())
    }
}

/// Takes a list of points supplied by the user
/// and maps them to a list of p.d.f.-evaluted values
pub fn pdf_points<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    pdf: F,
    discrete: bool,
) -> Vec<PEval> {
    let msg = |v: &T| match discrete {
        true => format!("P(X = {})", v),
        false => format!("pdf @ {}", v),
    };
    list.iter().filter_map(|v| PEval::new(&msg(v), Some(pdf(*v)))).collect()
}

/// Takes a list of n points supplied by the user
/// and maps them to n - 1 intervals calculated by c.d.f.
/// Every interval is left < X <= right
pub fn cdf_intervals<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    cdf: F,
) -> Vec<PEval> {
    if list.len() == 1 {
        let hi = match list.get(0) {
            Some(v) => v,
            None => return vec![],
        };
        let desc = format!("P(X <= {hi})");
        return vec![PEval::new(&desc, Some(cdf(*hi))).unwrap()];
    }
    let mut iter = list.iter().peekable();
    let mut result = Vec::new();
    while let Some(lo) = iter.next() {
        if let Some(hi) = iter.peek() {
            let desc = format!("P({lo} < X <= {hi})");
            let val = cdf(**hi) - cdf(*lo);
            result.push(PEval::new(&desc, Some(val)).unwrap());
        }
    }
    return result;
}
