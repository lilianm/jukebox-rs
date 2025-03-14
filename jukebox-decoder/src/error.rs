use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    InvalidData,
}

impl StdError for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidData => write!(f, "invalid data"),
        }
    }
}
