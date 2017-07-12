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
extern crate tokio_core;
extern crate url;

use futures::Future;
use hyper::client::{FutureResponse, HttpConnector};
use hyper::header;
use hyper::{Client, Method, Request, Uri};
use tokio_core::reactor::Core;
use url::Url;

pub mod error;

pub use error::Error;

use error::Result;

pub struct Endpoint {
    base: Url,
    core: Core,
    client: Client<HttpConnector>,
}

impl Endpoint {
    pub fn new(base: &str) -> Result<Self> {
        let base = Url::parse(base)?;
        let core = Core::new()?;
        let client = Client::new(&core.handle());
        Ok(Self { base, core, client })
    }

    pub fn get(&self, path: &str) -> Result<FutureResponse> {
        let uri = join_to_uri(&self.base, path)?;
        let work = self.client.get(uri);
        Ok(work)
    }

    // TODO: What should be used in place of `body: &'static str`?
    pub fn post(&self, path: &str, body: &'static str) -> Result<FutureResponse> {
        let uri = join_to_uri(&self.base, path)?;
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(
            header::ContentLength(body.len() as u64),
        );
        req.set_body(body);
        let work = self.client.request(req);
        Ok(work)
    }

    // TODO: Is this constrained to work only with Hyper structs?
    //
    // NB: It may be necessary to leave it somewhat unconstrained, in
    // order to allow composition of requests from API
    // consumers. Maybe a newtype would help to keep layers separate.
    pub fn run<T>(&mut self, work: T) -> Result<T::Item>
    where
        T: Future,
        Error: From<T::Error>,
    {
        let resp = self.core.run(work)?;
        Ok(resp)
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
