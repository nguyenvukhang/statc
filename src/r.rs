use crate::utils::{err, Result};
use std::process::Command;

fn num_from_stdout(out: std::process::Output) -> Result<f64> {
    // don't take value if return code is a failure
    if !out.status.success() {
        return err("Bad exit code from R.");
    }
    // only take non-empty outputs
    let res: f64 = match String::from_utf8_lossy(&out.stdout).trim() {
        v if v.is_empty() => return err("Empty stdout from R."),
        v => v.parse().map_err(|_| "Unable to parse R output.")?,
    };
    match res.is_nan() {
        true => err("Parsed a NaN value"),
        false => Ok(res),
    }
}

fn rcmd() -> Command {
    let mut r = Command::new("R");
    r.args(["--vanilla", "--no-echo", "-e"]);
    r
}

/// for testing against R
#[allow(dead_code)]
pub fn r(r_code: &str) -> Result<f64> {
    let mut rcmd = rcmd();
    rcmd.arg(format!("cat(sprintf('%.12f',{}))", r_code));
    num_from_stdout(rcmd.output().map_err(|_| "Unable to spawn R.")?)
}

/// for testing against R
#[allow(dead_code)]
pub fn r_debug(r_code: &str) {
    println!("{:?}", r(r_code));
}
