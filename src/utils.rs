pub type Result<T> = std::result::Result<T, String>;

pub fn err<T>(msg: &str) -> Result<T> {
    Err(msg.to_string())
}

pub trait Clear<T, E> {
    fn clear(self) -> std::result::Result<(), E>;
}

impl<T, E> Clear<T, E> for std::result::Result<T, E> {
    fn clear(self) -> std::result::Result<(), E> {
        self.map(|_| ())
    }
}
