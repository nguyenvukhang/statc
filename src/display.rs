use crate::types::{Analysis, PEval, PEvalList};
use std::fmt::{self, Display, Formatter};

fn margin(left: impl Display, right: impl Display, margin: usize) -> String {
    let (m, l) = (margin, format!("{}", left));
    let l = (0..m.checked_sub(l.len()).unwrap_or(1)).fold(l, |a, _| a + " ");
    let r = format!("{}", right);
    match r.is_empty() {
        true => format!("{}", l),
        false => format!("{} | {}", l, r),
    }
}

struct Printer {
    margin: usize,
    lines: Vec<[String; 2]>,
    build: String,
}

impl Printer {
    fn new() -> Self {
        Self { margin: 0, lines: Vec::new(), build: String::new() }
    }

    fn word(&mut self, w: &str) {
        self.margin = self.margin.max(w.len());
    }

    fn line(&mut self, left: &str, data: Option<impl Display>) {
        if let Some(data) = data {
            self.word(left);
            self.lines.push([left.to_string(), data.to_string()]);
        }
    }

    fn line_eval(&mut self, p_eval: &PEval) {
        let left = p_eval.desc.to_string();
        self.word(&left);
        let right = p_eval.val.map(|v| v.to_string()).unwrap_or_default();
        self.lines.push([left, right]);
    }

    fn build(&self) -> String {
        let mut build = self.lines.iter().fold(String::new(), |s, l| {
            format!("{}{}\n", s, margin(&l[0], &l[1], self.margin))
        });
        build.pop(); // remove last newline
        build
    }

    fn set_header(&mut self, title: &str) {
        if self.build.is_empty() && !title.is_empty() {
            self.build.push_str(title);
        }
    }

    fn flush(&mut self, f: &mut Formatter) -> fmt::Result {
        if !self.build.is_empty() {
            self.build.push('\n');
        }
        self.build.push_str(&self.build());
        write!(f, "{}", self.build)
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printer = Printer::new();
        printer.line("expected", self.expected);
        printer.line("variance", self.variance);
        self.pdf_eval.iter().for_each(|v| printer.line_eval(v));
        self.cdf_eval.iter().for_each(|v| printer.line_eval(v));
        printer.set_header(&self.header);
        printer.flush(f)
    }
}

impl Display for PEvalList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printer = Printer::new();
        self.list.iter().for_each(|v| printer.line_eval(&v));
        printer.flush(f)
    }
}
