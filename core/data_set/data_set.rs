use crate::data::Data;
use crate::utils::{Result, ResultOps};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn open_file(file: &str) -> Result<File> {
    let cwd = env::current_dir().serr("Unable to get current dir.")?;
    File::open(cwd.join(file)).serr("Unable to get file")
}

pub fn analyze(file: &str) {
    let lines: Vec<String> = match open_file(file) {
        Err(_) => return,
        Ok(v) => BufReader::new(v).lines().filter_map(|v| v.ok()).collect(),
    };
    let data = Data::new(&lines);
    match data.is_valid() {
        Ok(_) => println!("{}", data.export()),
        Err(msg) => println!("Error: {}", msg),
    }
}
