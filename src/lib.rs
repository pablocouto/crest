// Copyright 2017 Pablo Couto

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

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate native_tls;
extern crate num_cpus;
extern crate tokio_core;
extern crate tokio_timer;
extern crate url;

use futures::{future, Future};
use hyper::client::HttpConnector;
use hyper::{Client, Method, Uri};
use hyper_tls::HttpsConnector;
use std::time::Duration;
use tokio_core::reactor::Core;
use tokio_timer::{Timeout, Timer};
use url::Url;

pub mod error;

pub use error::Error;

use error::Result;

pub struct Endpoint {
    base: Url,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Endpoint {
    pub fn new(base: &str) -> Result<Self> {
        let base = Url::parse(base)?;
        let core = Core::new()?;
        let client = Client::configure()
            .connector(HttpsConnector::new(num_cpus::get(), &core.handle())?)
            .build(&core.handle());
        Ok(Self { base, core, client })
    }

    fn request(&self, method: Method, path: &str) -> Result<Request> {
        let uri = join_to_uri(&self.base, path)?;
        let req = hyper::Request::new(method, uri);
        Ok(Request {
            endpoint: &self,
            request: req,
            timeout: None,
        })
    }

    pub fn get(&self, path: &str) -> Result<Request> {
        self.request(Method::Get, path)
    }

    pub fn post(&self, path: &str) -> Result<Request> {
        self.request(Method::Post, path)
    }

    // TODO: Improve API separation?
    pub fn run<T>(&mut self, work: T) -> std::result::Result<T::Item, T::Error>
    where
        T: Future,
    {
        self.core.run(work)
    }
}

// TODO: Implement Deref to avoid wrapping certain Hyper methods?
pub struct Request<'a> {
    endpoint: &'a Endpoint,
    request: hyper::Request,
    timeout: Option<Duration>,
}

impl<'a> Request<'a> {
    pub fn header<T>(mut self, header: T) -> Self
    where
        T: hyper::header::Header,
    {
        self.request.headers_mut().set(header);
        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<hyper::Body>,
    {
        self.request.set_body(body);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn into_future(self) -> Box<Future<Item = hyper::Response, Error = Error>> {
        let future = self.endpoint.client.request(self.request).from_err();
        if let Some(timeout) = self.timeout {
            let future = timeout_future(future, timeout);
            return Box::new(future);
        }
        Box::new(future)
    }
}

fn to_uri(url: &Url) -> Result<Uri> {
    let uri = url.as_str().parse()?;
    Ok(uri)
}

fn join_to_uri(base: &Url, path: &str) -> Result<Uri> {
    let url = base.join(path)?;
    let uri = to_uri(&url)?;
    Ok(uri)
}

fn timeout_future<T>(future: T, timeout: Duration) -> Timeout<future::FromErr<T, Error>>
where
    T: Future,
    Error: From<T::Error>,
{
    let timer = Timer::default();
    let future = timer.timeout(future.from_err(), timeout);
    future
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_uri() {
        let url: Url = "https://httpbin.org/ip".parse().unwrap();
        let uri: Uri = "https://httpbin.org/ip".parse().unwrap();
        let uri_from_fn = super::to_uri(&url).unwrap();
        assert_eq!(uri_from_fn, uri);
    }

    #[test]
    fn join_to_uri() {
        let url: Url = "https://httpbin.org/".parse().unwrap();
        let uri: Uri = "https://httpbin.org/post".parse().unwrap();
        let uri_from_fn = super::join_to_uri(&url, "post").unwrap();
        assert_eq!(uri_from_fn, uri);
    }
}
