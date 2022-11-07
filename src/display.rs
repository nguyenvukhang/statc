use crate::types::{Analysis, PEval};
use std::fmt::{self, Display, Formatter};

fn margin(left: impl Display, right: impl Display, margin: usize) -> String {
    let (m, l) = (margin, format!("{}", left));
    let l = (0..m.checked_sub(l.len()).unwrap_or(1)).fold(l, |a, _| a + " ");
    format!("{} | {}", l, right)
}

struct Printer {
    margin: usize,
    lines: Vec<[String; 2]>,
}

impl Printer {
    fn new() -> Self {
        Self { margin: 0, lines: Vec::new() }
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

    fn line_eval(&mut self, p_eval: Option<&PEval>) {
        if let Some(p_eval) = p_eval {
            let left = p_eval.desc.to_string();
            self.word(&left);
            self.lines.push([left, p_eval.val.to_string()]);
        }
    }

    fn build(&self) -> String {
        self.lines.iter().fold(String::new(), |s, l| {
            format!("{}{}\n", s, margin(&l[0], &l[1], self.margin))
        })
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut printer = Printer::new();
        printer.line("expected", self.expected);
        printer.line("variance", self.variance);
        printer.line_eval(self.pdf_eval.as_ref());
        printer.line_eval(self.cdf_eval.as_ref());
        write!(f, "{}\n{}", self.display, printer.build().trim_end())
    }
}
