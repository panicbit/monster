use std::fmt;
use std::error;
use std::result;

pub type Result = result::Result<String, Error>;

#[derive(Debug)]
pub enum Error {
    NotEnoughArgs,
    UnexpectedChar,
    UnexpectedRightBrace,
    Unkown
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "Formatting error: {}", self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotEnoughArgs => "not enough arguments passed",
            Error::UnexpectedChar => "unexpected character",
            Error::UnexpectedRightBrace => "unexpected right brace",
            Error::Unkown => "unknown error"
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error::Unkown
    }
}
