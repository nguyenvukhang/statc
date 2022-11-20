mod data;
mod point;
use crate::utils::{err, Result, ResultOps};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub use data::{Data, Parser};

fn open_file(file: &str) -> Result<File> {
    let cwd = env::current_dir().serr("Unable to get current dir.")?;
    File::open(cwd.join(file)).serr("Unable to get file")
}

pub fn analyze(file: &str, parser: Parser) -> Result<Data> {
    let lines = match open_file(file) {
        Err(_) => return err(&format!("Unable to open file {}", file)),
        Ok(v) => BufReader::new(v).lines().filter_map(|v| v.ok()).collect(),
    };
    Data::new(&lines, parser)
}
