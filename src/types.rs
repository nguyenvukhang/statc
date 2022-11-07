use crate::math::Round;

pub struct Analysis {
    pub display: String,
    pub expected: Option<f64>,
    pub variance: Option<f64>,
    pub pdf_eval: Option<f64>,
    pub cdf_eval: Option<f64>,
}

impl Analysis {
    pub fn round(&self) -> Self {
        Self {
            display: self.display.to_string(),
            expected: self.expected.map(|x| x.roundn(10)),
            variance: self.variance.map(|x| x.roundn(10)),
            pdf_eval: self.pdf_eval.map(|x| x.roundn(10)),
            cdf_eval: self.cdf_eval.map(|x| x.roundn(10)),
        }
    }
}

pub trait Summary {
    fn analyze(&self, x: Option<u64>) -> Analysis;
    fn display(&self, x: Option<u64>) -> String;
}
