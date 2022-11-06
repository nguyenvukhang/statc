use crate::math::Round;

pub struct Analysis {
    pub display: String,
    pub expected: f64,
    pub variance: f64,
    pub pdf_eval: Option<f64>,
    pub cdf_eval: Option<f64>,
}

impl Analysis {
    pub fn round(&self) -> Self {
        Self {
            cdf_eval: self.cdf_eval.map(|x| x.roundn(10)),
            pdf_eval: self.pdf_eval.map(|x| x.roundn(10)),
            expected: self.expected.roundn(10),
            variance: self.variance.roundn(10),
            display: self.display.to_string(),
        }
    }
}

pub trait Distribution {
    fn expected(&self) -> f64;
    fn variance(&self) -> f64;
    fn analyze(&self) -> Analysis;
}
