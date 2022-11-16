use crate::math::Round;
use crate::utils::{err, Result, ResultOps};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

struct DataPoint {
    val: f64,
    prob: f64,
}

impl fmt::Display for DataPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P(X = {}) = {}", self.val, self.prob)
    }
}

fn pretty_display(data: &Vec<DataPoint>) -> String {
    let mut res =
        data.iter().fold(String::new(), |s, v| s + &format!("{}\n", v));
    res.pop();
    res
}

fn valid_total_prob(data: &Vec<DataPoint>) -> bool {
    (data.iter().map(|v| v.prob).fold(0.0, |a, b| a + b) - 1.0).abs() < 1e-10
}

fn check(data: &Vec<DataPoint>) -> Result<()> {
    if data.is_empty() {
        return err("empty dataset");
    }
    if !valid_total_prob(&data) {
        return err("total prob is not 1.");
    }
    Ok(())
}

fn mean(data: &Vec<DataPoint>) -> Result<f64> {
    check(&data)?;
    Ok(data.iter().fold(0.0, |a, v| a + v.prob * v.val))
}

fn variance(data: &Vec<DataPoint>) -> Result<f64> {
    let mean = mean(&data)?;
    let ex2 = data.iter().fold(0.0, |a, v| a + v.prob * v.val * v.val);
    let res = ex2 - mean * mean;
    Ok(res.roundn(10))
}

fn parse(s: &str) -> Result<f64> {
    s.parse().serr("Unable to parse f64 from string.")
}

fn pairs(lines: &Vec<String>) -> Result<Vec<DataPoint>> {
    let mut data: Vec<DataPoint> = Vec::new();
    lines.iter().for_each(|line| {
        let s = match line.split_once(' ') {
            Some(v) => (parse(v.0), parse(v.1)),
            None => return,
        };
        match s {
            (Ok(a), Ok(b)) => data.push(DataPoint { val: a, prob: b }),
            _ => return,
        }
    });
    Ok(data)
}

fn points(lines: &Vec<String>) -> Result<Vec<DataPoint>> {
    let mut data: Vec<DataPoint> = Vec::new();
    lines.iter().for_each(|line| match parse(&line) {
        Ok(v) => data.push(DataPoint { val: v, prob: 0.0 }),
        _ => return,
    });
    let prob = 1.0 / data.len() as f64;
    Ok(data.iter().map(|p| DataPoint { val: p.val, prob }).collect())
}

fn print(data: &Vec<DataPoint>) {
    println!("{}", pretty_display(&data));
    match mean(&data) {
        Ok(v) => println!("mean:     {}", v),
        Err(e) => println!("Error: {}", e),
    }
    match variance(&data) {
        Ok(v) => println!("variance: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn open_file(file: &str) -> Result<File> {
    let cwd = env::current_dir().serr("Unable to get current dir.")?;
    File::open(cwd.join(file)).serr("Unable to get file")
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

pub fn analyze(file: &str) {
    let v: Vec<String> = match open_file(file) {
        Err(_) => return,
        Ok(v) => lines(v).collect(),
    };
    let mut data = pairs(&v).unwrap_or_default();
    if data.is_empty() {
        data = points(&v).unwrap_or_default();
    }
    print(&data);
}
