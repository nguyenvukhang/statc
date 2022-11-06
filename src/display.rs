use crate::types::Analysis;
use std::fmt::{self, Display, Formatter};

const MARGIN: usize = 16;

fn margin(left: impl Display, right: impl Display) -> String {
    let (m, l) = (MARGIN, format!("{}", left));
    let l = (0..m.checked_sub(l.len()).unwrap_or(1)).fold(l, |a, _| a + " ");
    format!("{}| {}", l, right)
}

fn pushln(base: &mut String, line: &String) {
    base.push_str(&line);
    base.push('\n');
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = &mut String::new();
        pushln(s, &self.display);
        pushln(s, &margin("expected:", self.expected));
        pushln(s, &margin("variance:", self.variance));
        match self.pdf_eval {
            Some(_) => {
                let pdf_eval = self.pdf_eval.ok_or(fmt::Error)?;
                let cdf_eval = self.cdf_eval.ok_or(fmt::Error)?;
                pushln(s, &margin("P(X = x)", pdf_eval));
                pushln(s, &margin("P(X <= x)", cdf_eval));
            }
            None => (),
        };
        write!(f, "{}", s.trim_end())
    }
}
