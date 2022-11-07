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
