use crate::prob::{Meta, P};
use std::fmt::Display;

/// fancy-prints a title in full caps
pub fn title(s: &str) {
    println!("--- {} ---", s.to_uppercase());
}

const MARGIN: usize = 16;

fn margin(left: impl Display, right: impl Display) {
    let (m, l) = (MARGIN, format!("{}", left));
    let l = (0..m.checked_sub(l.len()).unwrap_or(1)).fold(l, |a, _| a + " ");
    println!("{}| {}", l, right)
}

pub const BINOMIAL: &str = "
Probability of winning x times in n trials given a win-rate of p.";

pub fn meta(meta: Meta) {
    margin("expected", meta.expected);
    margin("variance", meta.variance);
}

pub fn binomial(trials: u64, win_rate: P, wins: u64, pdf: P, cdf: P) {
    margin("n: trials", trials);
    margin("p: win-rate", win_rate);
    margin("x: wins", wins);
    margin("P(X = x)", pdf);
    margin("P(X <= x)", cdf);
    margin("P(X > x)", 1.0 - cdf);
}

pub const NEGATIVE_BINOMIAL: &str = "
Probability of winning for the kth time on the nth trial given a
win-rate of p.
";

pub fn negative_binomial(wins: u64, win_rate: P, trials: u64, p: P) {
    margin("k: wins", wins);
    margin("p: win-rate", win_rate);
    margin("x: trials", trials);
    margin("P(X = x)", p);
}
