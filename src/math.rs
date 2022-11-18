use crate::types::LineList;

pub trait MathOps {
    fn pow(self, exponent: u64) -> Self;
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

pub fn pooled_variance(n1: f64, v1: f64, n2: f64, v2: f64) -> LineList {
    let mut plist = LineList::new();
    let p = ((n1 - 1.0) * v1 + (n2 - 1.0) * v2) / (n1 + n2 - 2.0);
    plist.push("[1] sample size", n1);
    plist.push("[1] sample variance", v1);
    plist.push("[2] sample size", n2);
    plist.push("[2] sample variance", v2);
    plist.push("pooled sample variance", p);
    plist.push("pooled sample std.dev", p.sqrt());
    plist
}
