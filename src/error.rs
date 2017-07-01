/*

Copyright 2016–2017 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

*/

/*!
Error and Result types.
 */

use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

use hyper::error::Error as HyperError;
use serde_json::error::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum Error {
    NoJson,
    Hyper(HyperError),
    SerdeJson(SerdeJsonError),
    Url(UrlParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoJson => "The response is not in JSON format.",
            Error::Hyper(ref e) => e.description(),
            Error::SerdeJson(ref e) => e.description(),
            Error::Url(ref e) => e.description(),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<UrlParseError> for Error {
    fn from(e: UrlParseError) -> Error {
        Error::Url(e)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(e: SerdeJsonError) -> Error {
        Error::SerdeJson(e)
    }
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}
