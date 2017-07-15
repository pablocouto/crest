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
extern crate tokio_core;
extern crate url;

use futures::Future;
use hyper::client::{FutureResponse, HttpConnector};
use hyper::{Client, Headers, Method, Uri};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
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
            // TODO: Default to number of processing units.
            .connector(HttpsConnector::new(4, &core.handle())?)
            .build(&core.handle());
        Ok(Self { base, core, client })
    }

    fn request(&self, method: Method, path: &str) -> Result<Request> {
        let uri = join_to_uri(&self.base, path)?;
        let req = hyper::Request::new(method, uri);
        Ok(Request {
            endpoint: &self,
            request: req,
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
}

impl<'a> Request<'a> {
    pub fn headers_mut(&mut self) -> &mut Headers {
        self.request.headers_mut()
    }

    // TODO: Consider relying on From<T> for Body.
    pub fn set_body(&mut self, body: &'static str) {
        self.request.set_body(body);
    }

    pub fn into_future(self) -> FutureResponse {
        self.endpoint.client.request(self.request)
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
