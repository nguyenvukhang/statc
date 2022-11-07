use crate::types::Analysis;
use std::fmt::{self, Display, Formatter};

const MARGIN: usize = 16;

fn margin(left: impl Display, right: impl Display) -> String {
    let (m, l) = (MARGIN, format!("{}", left));
    let l = (0..m.checked_sub(l.len()).unwrap_or(1)).fold(l, |a, _| a + " ");
    format!("{} | {}", l, right)
}

fn line(s: &mut String, left: &str, data: Option<impl Display>) {
    if let Some(data) = data {
        s.push_str(&margin(left.to_owned() + ":", data));
        s.push('\n');
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = &mut (self.display.to_owned() + "\n");
        line(s, "expected", self.expected);
        line(s, "variance", self.variance);
        line(s, "P(X = x)", self.pdf_eval);
        line(s, "P(X <= x)", self.cdf_eval);
        write!(f, "{}", s.trim_end())
    }
}
