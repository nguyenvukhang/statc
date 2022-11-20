use crate::data_set::point::{DataPoint, ParseData};
use crate::display::LineList;
use crate::utils::{err, Result};

#[derive(Debug)]
pub struct Data {
    data: Vec<DataPoint>,
    mean: Option<f64>,
    var_p: Option<f64>,
    var_s: Option<f64>,
}

pub enum Parser {
    Single,
    PairDiff,
}

impl Data {
    pub fn new(raw: &Vec<String>, parser: Parser) -> Result<Self> {
        let data: Vec<DataPoint> = match parser {
            Parser::Single => {
                let b: Vec<DataPoint> = raw
                    .iter()
                    .map(|v| v.val_prob())
                    .filter_map(|v| v.ok())
                    .collect();
                match b.is_empty() {
                    true => raw
                        .iter()
                        .map(|v| v.point())
                        .filter_map(|v| v.ok())
                        .collect(),
                    false => b,
                }
            }
            Parser::PairDiff => {
                raw.iter().map(|v| v.diff()).filter_map(|v| v.ok()).collect()
            }
        };
        let mut data = Data { mean: None, var_p: None, var_s: None, data };
        data.balance();
        data.mean().ok();
        data.var_p().ok();
        data.var_s().ok();
        data.check()
    }

    /// evenly distributes probability such that sum is 1
    /// only runs when total prob is zero.
    fn balance(&mut self) {
        let total = self.data.iter().map(|v| v.prob).reduce(|a, b| a + b);
        if total.unwrap_or(0.0) > 1e-10 {
            return;
        }
        let prob = 1.0 / self.n();
        self.data =
            self.data.iter().map(|v| DataPoint { val: v.val, prob }).collect();
    }

    pub fn n(&self) -> f64 {
        self.data.len() as f64
    }

    /// Calculate mean and update self.
    pub fn mean(&mut self) -> Result<f64> {
        if let Some(v) = self.mean {
            return Ok(v);
        }
        let mean = self.data.iter().fold(0.0, |a, v| a + v.prob * v.val);
        self.mean = Some(mean);
        Ok(mean)
    }

    /// Assumes self.data contains entire population
    /// and calculates population variance.
    pub fn var_p(&mut self) -> Result<f64> {
        if let Some(v) = self.var_p {
            return Ok(v);
        }
        let mean = self.mean()?;
        let ex2 = self.data.iter().fold(0.0, |a, v| a + v.prob * v.val * v.val);
        let var_p = ex2 - mean * mean;
        self.var_p = Some(var_p);
        Ok(var_p)
    }

    pub fn var_s(&mut self) -> Result<f64> {
        if let Some(v) = self.var_s {
            return Ok(v);
        }
        let n = self.n();
        self.var_s = self.var_p.map(|v| v * n / (n - 1.0));
        self.var_s.ok_or("Unable to get sample variance".to_string())
    }

    /// true if and only if dataset is non-empty and total probability adds up to 1
    pub fn check(self) -> Result<Self> {
        if self.data.is_empty() {
            return err("empty dataset.");
        }
        let total_prob = self.data.iter().map(|v| v.prob).reduce(|a, b| a + b);
        if (total_prob.unwrap_or(0.0) - 1.0).abs() > 1e-10 {
            return err("total probability is not 1.");
        }
        Ok(self)
    }

    pub fn export(&self) -> LineList {
        let mut list = LineList::new();
        let mut push = |d: &str, v: &Option<f64>| match v {
            Some(v) => list.push(d, *v),
            None => (),
        };
        push("mean", &self.mean);
        push("population variance", &self.var_p);
        push("population std.dev", &self.var_p.map(|v| v.sqrt()));
        push("sample variance", &self.var_s);
        push("sample std.err", &self.var_s.map(|v| v.sqrt()));
        list
    }
}
