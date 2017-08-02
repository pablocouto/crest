// Copyright 2016–2017 Pablo Couto

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// version 3 as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License version 3 for more details.

// You should have received a copy of the GNU Lesser General Public
// License version 3 along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use hyper;
use native_tls;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;
use tokio_timer;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Io(io::Error),
    NativeTls(native_tls::Error),
    TokioTimer(tokio_timer::TimerError),
    UriError(hyper::error::UriError),
    Url(url::ParseError),

    Timeout,
    Unknown,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::NativeTls(ref error) => error.description(),
            Error::TokioTimer(ref error) => error.description(),
            Error::UriError(ref error) => error.description(),
            Error::Url(ref error) => error.description(),

            Error::Timeout => "Operation timed out",
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

impl<T> From<tokio_timer::TimeoutError<T>> for Error {
    fn from(error: tokio_timer::TimeoutError<T>) -> Self {
        match error {
            tokio_timer::TimeoutError::Timer(_, error) => Error::TokioTimer(error),
            tokio_timer::TimeoutError::TimedOut(_) => Error::Timeout,
        }
    }
}

// TODO: Refactor into `Error::Hyper`?
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

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self {
        Error::NativeTls(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Url(error)
    }
}
