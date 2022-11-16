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

fn parse_pair(s: &str) -> Result<DataPoint, ParseFloatError> {
    let p = s.split_once(' ').unwrap_or_default();
    Ok(DataPoint { val: p.0.parse()?, prob: p.1.parse()? })
}

fn parse_point(s: &str) -> Result<DataPoint, ParseFloatError> {
    Ok(DataPoint { prob: 0.0, val: s.parse()? })
}

impl std::str::FromStr for DataPoint {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<DataPoint, ParseFloatError> {
        match s.contains(' ') {
            true => parse_pair(s),
            false => parse_point(s),
        }
    }
}
