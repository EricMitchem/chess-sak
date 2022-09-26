use std::error;
use std::fmt;
use std::io;
use std::result;
use std::str;
use pest::error::Error as PestError;
use crate::pgn::ParseRule;

#[derive(Debug)]
pub enum SakError {
    Io(io::Error),
    Pest(PestError<ParseRule>),
    Untyped(String),
    Utf8(str::Utf8Error),
}

pub type SakResult<T> = result::Result<T, SakError>;

impl error::Error for SakError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            SakError::Io(err) => Some(err),
            SakError::Pest(err) => Some(err),
            SakError::Untyped(_) => None,
            SakError::Utf8(err) => Some(err),
        }
    }
}

impl fmt::Display for SakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SakError::Io(err) => err.fmt(f),
            SakError::Pest(err) => err.fmt(f),
            SakError::Untyped(err) => err.fmt(f),
            SakError::Utf8(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for SakError {
    fn from(err: io::Error) -> Self {
        SakError::Io(err)
    }
}

impl From<PestError<ParseRule>> for SakError {
    fn from(err: PestError<ParseRule>) -> Self {
        SakError::Pest(err)
    }
}

impl From<str::Utf8Error> for SakError {
    fn from(err: str::Utf8Error) -> Self {
        SakError::Utf8(err)
    }
}