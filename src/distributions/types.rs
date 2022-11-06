use crate::prob::{Meta, P};

pub trait DiscretePdf {
    fn pdf(&self, x: u64) -> P;
}

pub trait DiscreteCdf {
    fn cdf(&self, x: u64) -> P;
}

pub trait ContinuousCdf {
    fn cdf(&self, x: u64) -> P;
}

pub trait ContinuousRange {
    fn cdf(&self, lo: f64, hi: f64) -> P;
}

pub trait DistributionMeta {
    fn meta(&self) -> Meta;
}
