use crate::types::Line;
use std::fmt::{self, Display, Formatter};

fn margin(left: impl Display, right: impl Display, margin: usize) -> String {
    let (l, r) = (left.to_string(), right.to_string());
    let spaces = margin.checked_sub(l.len()).unwrap_or(0);
    let spaces = (0..spaces).map(|_| " ").collect::<String>();
    match r.is_empty() {
        true => format!("{}", l),
        false => format!("{}{} | {}", l, spaces, r),
    }
}

struct PrintLine {
    desc: String,
    val: String,
}

pub struct Printer {
    margin: usize,
    lines: Vec<PrintLine>,
    title: String,
}

impl Printer {
    /// create a new printer
    pub fn new() -> Self {
        Self { margin: 0, lines: Vec::new(), title: String::new() }
    }

    /// updates the margin to accomodate this left text
    fn update_margin(&mut self, w: &str) {
        self.margin = self.margin.max(w.len());
    }

    /// creates a line with a description and a value
    /// and pushes it to the printer
    pub fn raw_line(&mut self, desc: &str, val: Option<impl Display>) {
        if let Some(val) = val {
            self.update_margin(desc);
            let [desc, val] = [desc.to_string(), val.to_string()];
            self.lines.push(PrintLine { desc, val });
        }
    }

    /// add a line to the printer
    pub fn push_line(&mut self, line: &Line) {
        self.update_margin(&line.desc);
        let desc = line.desc.to_string();
        let right = line.val.map(|v| v.to_string()).unwrap_or_default();
        self.lines.push(PrintLine { desc, val: right });
    }

    fn build(&self) -> String {
        self.lines
            .iter()
            .map(|v| margin(&v.desc, &v.val, self.margin))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// set the title of the print buffer
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// write contents of print buffer to the formatter for printing
    pub fn flush(&mut self, f: &mut Formatter) -> fmt::Result {
        if !self.title.is_empty() {
            writeln!(f, "{}", self.title).ok();
        }
        write!(f, "{}", self.build())
    }
}
