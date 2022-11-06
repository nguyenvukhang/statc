pub struct Analysis {
    pub display: String,
    pub expected: f64,
    pub variance: f64,
    pub pdf_eval: Option<f64>,
    pub cdf_eval: Option<f64>,
}

pub trait Distribution {
    fn expected(&self) -> f64;
    fn variance(&self) -> f64;
    fn analyze(&self) -> Analysis;
}
