use std::fmt;
use std::num::ParseFloatError;

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
    fn diff(&self) -> Result<DataPoint, ParseFloatError>;
    fn point(&self) -> Result<DataPoint, ParseFloatError>;
    fn val_prob(&self) -> Result<DataPoint, ParseFloatError>;
}

impl ParseData for String {
    fn diff(&self) -> Result<DataPoint, ParseFloatError> {
        let p = self.split_once(' ').unwrap_or_default();
        let parse = |v: &str| v.parse::<f64>();
        Ok(DataPoint { val: parse(p.0)? - parse(p.1)?, prob: 0.0 })
    }

    fn val_prob(&self) -> Result<DataPoint, ParseFloatError> {
        let p = self.split_once(' ').unwrap_or_default();
        Ok(DataPoint { val: p.0.parse()?, prob: p.1.parse()? })
    }

    fn point(&self) -> Result<DataPoint, ParseFloatError> {
        Ok(DataPoint { prob: 0.0, val: self.parse()? })
    }
}
