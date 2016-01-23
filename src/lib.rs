/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

*/

extern crate hyper;
extern crate url;

use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

use hyper::client::{
    Client,
    Response,
};
use hyper::error::Error as HyperError;
use url::{
    ParseError as UrlParseError,
    Url,
};

#[doc(no_inline)]
pub use hyper::header::Headers;
#[doc(no_inline)]
pub use hyper::method::Method;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
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
            Error::Hyper(ref e) => e.description(),
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

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}

pub struct Request<'a> {
    method: Method,
    path: &'a [&'a str],
    headers: Headers,
    body: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn get(path: &'a [&'a str]) -> Request<'a> {
        Request {
            method: Method::Get,
            path: path,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn post(
        path: &'a [&'a str],
        body: &'a str,
    ) -> Request<'a> {
        Request {
            method: Method::Post,
            path: path,
            headers: Headers::new(),
            body: Some(body),
        }
    }

    pub fn delete(path: &'a [&'a str]) -> Request<'a> {
        Request {
            method: Method::Delete,
            path: path,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn headers(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&mut self, body: &'a str) {
        self.body = Some(body);
    }
}

pub struct Endpoint {
    base: Url,
    client: Client,
}

impl Endpoint {
    pub fn new(base: &str) -> Result<Endpoint> {
        let url = try!(Url::parse(base));
        let client = Client::new();

        Ok(Endpoint {
            base: url,
            client: client,
        })
    }

    pub fn send<'a>(&self, request: Request<'a>) -> Result<Response> {
        let path = request.path.join("/");
        let url = self.base.join(&path).unwrap();

        let mut req = match request.method {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
            Method::Delete => self.client.delete(url),
            _ => unimplemented!(),
        };

        req = req.headers(request.headers);

        if request.body.is_some() {
            let b = request.body.unwrap();
            req = req.body(b);
        }

        let response = req
            .send()
            .map_err(From::from);

        response
    }
}
