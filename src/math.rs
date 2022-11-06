/// Calculates n choose k
pub fn choose(n: u64, k: u64) -> u64 {
    if n < k {
        return 0;
    }
    let (mut n, mut i, mut r) = (n, 1, 1);
    while i <= k {
        r = r * n / i;
        (n, i) = (n - 1, i + 1);
    }
    r
}

pub trait MathOps {
    fn pow(self, exponent: u64) -> Self;
}

impl MathOps for f64 {
    fn pow(self, mut e: u64) -> Self {
        let (mut b, mut r) = (self, 1.0);
        while e > 0 {
            (r, b, e) = (r + r * (e & 1) as f64 * (b - 1.0), b * b, e >> 1);
        }
        r
    }
}

impl MathOps for u64 {
    fn pow(self, mut e: u64) -> Self {
        let (mut b, mut r) = (self, 1);
        while e > 0 {
            (r, b, e) = (r + r * (e & 1) * (b - 1), b * b, e >> 1);
        }
        r
    }
}

pub trait Round {
    fn roundn(&self, decimals: u64) -> Self;
}

impl Round for f64 {
    fn roundn(&self, decimals: u64) -> f64 {
        let shift = 10.pow(decimals) as f64;
        let res = self * shift;
        let res = res.round();
        let res = res / shift;
        res
    }
}

/// useful for calculating cumulative probability once
/// Probability Density Function is known.
pub fn range<F>(start: u64, end: u64, mut f: F) -> f64
where
    F: FnMut(u64) -> f64,
{
    (start..end).fold(0.0, |a, i| a + f(i))
}

#[test]
fn choose_test() {
    assert_eq!(choose(10, 3), 120);
    assert_eq!(choose(2, 1), 2);
    assert_eq!(choose(2, 3), 0);
    assert_eq!(choose(20, 10), 184756);
    assert_eq!(choose(50, 25), 126410606437752);
}

#[test]
fn pow_test_u64() {
    assert_eq!(10.pow(10), 10000000000);
    assert_eq!(2.pow(0), 1);
    assert_eq!(4.pow(1), 4);
    assert_eq!(7.pow(2), 49);
}

#[test]
fn pow_test_f64() {
    assert_eq!(10.0.pow(10), 10000000000.0);
    assert_eq!(2.0.pow(0), 1.0);
    assert_eq!(4.0.pow(1), 4.0);
    assert_eq!(7.0.pow(2), 49.0);
}
