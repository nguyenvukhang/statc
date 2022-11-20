use std::fmt;

#[derive(Debug)]
pub struct DataPoint {
    pub val: f64,
    pub prob: f64,
}

impl fmt::Display for DataPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P(X = {}) = {}", self.val, self.prob)
    }
}

pub trait ParseData {
    fn diff(&self) -> Option<DataPoint>;
    fn point(&self) -> Option<DataPoint>;
    fn val_prob(&self) -> Option<DataPoint>;
}

fn parse(v: &str) -> Option<f64> {
    meval::eval_str(v).ok().or_else(|| v.parse::<f64>().ok())
}

impl ParseData for String {
    fn diff(&self) -> Option<DataPoint> {
        let p = self.split_once(' ').unwrap_or_default();
        Some(DataPoint { val: parse(p.0)? - parse(p.1)?, prob: 0.0 })
    }

    fn val_prob(&self) -> Option<DataPoint> {
        let p = self.split_once(' ').unwrap_or_default();
        Some(DataPoint { val: parse(p.0)?, prob: parse(p.1)? })
    }

    fn point(&self) -> Option<DataPoint> {
        Some(DataPoint { prob: 0.0, val: parse(self)? })
    }
}
