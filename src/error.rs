use std::result::Result as StdResult;

pub struct Error {
    pub kind: ErrorKind,
}

pub enum ErrorKind {}

pub type Result<T> = StdResult<T, Error>;
