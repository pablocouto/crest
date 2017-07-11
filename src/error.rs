use hyper;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    UriError(hyper::error::UriError),
    Io(io::Error),
    Url(url::ParseError),
    Unknown,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref error) => error.description(),
            Error::UriError(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::Url(ref error) => error.description(),
            Error::Unknown => "Unknown error",
        }
    }

    // TODO: Implement `fn cause`?
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Hyper(error)
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(error: hyper::error::UriError) -> Self {
        Error::UriError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Url(error)
    }
}
