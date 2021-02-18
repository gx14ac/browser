#[derive(Debug, Clone, Copy)]
pub enum Error {
    ReadError,
}

impl Error {
    pub fn msg(&self) -> String {
        match *self {
            Error::ReadError => "can't read char".to_string(),
        }
    }
}
