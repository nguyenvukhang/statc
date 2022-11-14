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
    list.iter().map(|v| PEval::new(&msg(v), pdf(*v))).collect()
}

/// Takes a list of n points supplied by the user
/// and maps them to n + 1 intervals calculated by c.d.f.
/// * starts from P(X <= first element)
/// * ends at P(X > last element)
pub fn cdf_intervals<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    cdf: F,
) -> Vec<PEval> {
    // first element: calculate P(X <= x)
    let mut result = match list.first() {
        None => return vec![],
        Some(hi) => vec![PEval::new(&format!("P(X <= {hi})"), cdf(*hi))],
    };
    let mut iter = list.iter().peekable();
    let mut send = |d: &str, v: f64| result.push(PEval::new(&d, v));
    while let Some(lo) = iter.next() {
        if let Some(hi) = iter.peek() {
            send(&format!("P({lo} < X <= {hi})"), cdf(**hi) - cdf(*lo));
        } else {
            // last element: calculate P(X > x)
            send(&format!("P(X > {lo})"), 1.0 - cdf(*lo));
        }
    }
    return result;
}

pub fn eval(s: &str) -> Result<f64> {
    return meval::eval_str(s).serr("Bad input.");
}

pub fn eval_prob(s: &str) -> Result<f64> {
    let p = meval::eval_str(s).serr("Bad input.")?;
    if p < 0.0 || p > 1.0 {
        return err("Probability values must be between 0 and 1.");
    }
    Ok(p)
}

pub fn eval_u64(s: &str) -> Result<u64> {
    let v = meval::eval_str(s).serr("Bad input.")?;
    match v.fract() > 1e-10 {
        true => err("Not an integer."),
        false => Ok(v as u64),
    }
}
