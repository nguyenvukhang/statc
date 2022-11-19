use std::result as core;

pub type Result<T> = core::Result<T, String>;

pub fn err<T>(msg: &str) -> Result<T> {
    Err(msg.to_string())
}

pub trait ResultOps<T, E> {
    fn clear(self) -> core::Result<(), E>;
    fn serr(self, msg: &str) -> core::Result<T, String>;
}

impl<T, E> ResultOps<T, E> for core::Result<T, E> {
    fn clear(self) -> core::Result<(), E> {
        self.map(|_| ())
    }
    fn serr(self, msg: &str) -> core::Result<T, String> {
        self.map_err(|_| msg.to_string())
    }
}

pub fn eval(s: &str) -> Result<f64> {
    return meval::eval_str(s).serr("Bad input.");
}

pub fn eval_prob(s: &str) -> Result<f64> {
    let p = meval::eval_str(s).serr("Bad input.")?;
    if p < 0.0 || p > 1.0 {
        return err("Probability values must be between 0 and 1.");
    }
    Ok(p)
}

pub fn eval_u64(s: &str) -> Result<u64> {
    let v = meval::eval_str(s).serr("Bad input.")?;
    match v.fract() > 1e-10 {
        true => err("Not an integer."),
        false => Ok(v as u64),
    }
}
