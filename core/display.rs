use crate::math::Round;
use crate::printer::Printer;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Line {
    pub desc: String,
    pub val: Option<f64>,
}

impl Line {
    fn round(&mut self) {
        self.val = self.val.map(|v| v.roundn(10));
    }
    pub fn new(desc: &str, val: Option<f64>) -> Self {
        Self { desc: desc.to_string(), val: val.map(|v| v.roundn(10)) }
    }
}

#[derive(Debug, Default)]
pub struct Analysis {
    pub title: String,
    pub expected: Option<f64>,
    pub variance: Option<f64>,
    pub pdf_eval: Vec<Line>,
    pub cdf_eval: Vec<Line>,
}

pub struct LineList {
    pub title: String,
    pub list: Vec<Line>,
}

impl LineList {
    pub fn push(&mut self, desc: &str, val: f64) {
        let mut line = Line::new(desc, Some(val));
        line.round();
        self.list.push(line);
    }
    pub fn header(&mut self, header: &str) {
        self.list.push(Line::new(&format!("[{}]", header), None));
    }
    pub fn new() -> Self {
        Self { list: Vec::new(), title: String::new() }
    }
    pub fn set_title(&mut self, t: &str) {
        self.title = t.to_string();
    }
    pub fn append(&mut self, other: &LineList) {
        other
            .list
            .iter()
            .for_each(|v| self.list.push(Line::new(&v.desc, v.val)));
    }
}

impl Analysis {
    pub fn round(&mut self) {
        self.expected = self.expected.map(|v| v.roundn(10));
        self.variance = self.variance.map(|v| v.roundn(10));
        self.pdf_eval.iter_mut().for_each(|v| v.round());
        self.cdf_eval.iter_mut().for_each(|v| v.round());
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printer = Printer::new();
        printer.raw_line("expected", self.expected);
        printer.raw_line("variance", self.variance);
        self.pdf_eval.iter().for_each(|v| printer.push_line(&v));
        self.cdf_eval.iter().for_each(|v| printer.push_line(&v));
        printer.set_title(&self.title);
        printer.flush(f)
    }
}

impl Display for LineList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printer = Printer::new();
        printer.set_title(&self.title);
        self.list.iter().for_each(|v| printer.push_line(v));
        printer.flush(f)
    }
}
