use crate::math::Round;

pub struct PEval {
    pub val: f64,
    pub desc: String,
}

impl PEval {
    fn round(&self) -> Self {
        Self { val: self.val.roundn(10), desc: self.desc.to_string() }
    }
    pub fn new(desc: &str, val: Option<f64>) -> Option<Self> {
        Some(Self { desc: desc.to_string(), val: val? })
    }
}

pub struct Analysis {
    pub display: String,
    pub expected: Option<f64>,
    pub variance: Option<f64>,
    pub pdf_eval: Vec<PEval>,
    pub cdf_eval: Vec<PEval>,
}

impl Default for Analysis {
    fn default() -> Analysis {
        Analysis {
            display: "null analysis".to_string(),
            expected: None,
            variance: None,
            pdf_eval: Vec::new(),
            cdf_eval: Vec::new(),
        }
    }
}

impl Analysis {
    pub fn round(&self) -> Self {
        Self {
            display: self.display.to_string(),
            expected: self.expected.map(|x| x.roundn(10)),
            variance: self.variance.map(|x| x.roundn(10)),
            pdf_eval: self.pdf_eval.iter().map(|x| x.round()).collect(),
            cdf_eval: self.cdf_eval.iter().map(|x| x.round()).collect(),
        }
    }
}

pub trait Summary<T> {
    fn analyze(&self, values: &Vec<T>) -> Analysis;
    fn display(&self) -> String;
}
