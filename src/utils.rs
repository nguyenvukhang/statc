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
