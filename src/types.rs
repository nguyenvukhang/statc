use crate::math::Round;

pub struct PEval {
    pub val: Option<f64>,
    pub desc: String,
}

impl PEval {
    fn round(&self) -> Self {
        Self {
            val: self.val.map(|v| v.roundn(10)),
            desc: self.desc.to_string(),
        }
    }
    pub fn new(desc: &str, val: Option<f64>) -> Self {
        Self { desc: desc.to_string(), val: val.map(|v| v.roundn(10)) }
    }
}

pub struct Analysis {
    pub header: String,
    pub expected: Option<f64>,
    pub variance: Option<f64>,
    pub pdf_eval: Vec<PEval>,
    pub cdf_eval: Vec<PEval>,
}

pub struct PEvalList {
    pub list: Vec<PEval>,
}

impl PEvalList {
    pub fn push(&mut self, desc: &str, val: f64) {
        self.list.push(PEval::new(desc, Some(val)));
    }
    pub fn header(&mut self, header: &str) {
        self.list.push(PEval::new(&format!("[{}]", header), None));
    }
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn append(&mut self, other: &PEvalList) {
        other
            .list
            .iter()
            .for_each(|v| self.list.push(PEval::new(&v.desc, v.val)));
    }
}

impl Default for Analysis {
    fn default() -> Analysis {
        Analysis {
            header: "null analysis".to_string(),
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
            header: self.header.to_string(),
            expected: self.expected.map(|x| x.roundn(10)),
            variance: self.variance.map(|x| x.roundn(10)),
            pdf_eval: self.pdf_eval.iter().map(|x| x.round()).collect(),
            cdf_eval: self.cdf_eval.iter().map(|x| x.round()).collect(),
        }
    }
}

pub trait Summary<T> {
    fn analyze(&self, values: &Vec<T>) -> Analysis;
    fn header(&self) -> String;
}
