use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::result;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Url(url::ParseError),
    Unknown,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Url(ref error) => error.description(),
            Error::Unknown => "Unknown error",
        }
    }

    // TODO: implement `fn cause`?
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Url(error)
    }
}
