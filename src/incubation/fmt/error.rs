// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::fmt;
use std::error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
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
