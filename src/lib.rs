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

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate native_tls;
extern crate num_cpus;
extern crate tokio_core;
extern crate tokio_timer;
extern crate url;

use futures::{future, stream, Future, Stream};
use hyper::client::HttpConnector;
use hyper::{Client, Method, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use std::time::Duration;
use tokio_core::reactor::Core;
use tokio_timer::{Timeout, TimeoutStream, Timer};
use url::Url;

mod error;
mod impls;

pub use error::*;

pub struct Endpoint {
    base: Url,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Endpoint {
    // TODO: Use builder pattern?
    pub fn new(base: &str, keep_alive: bool) -> Result<Self> {
        let base = Url::parse(base)?;
        let core = Core::new().chain_err(
            || "Failed to create Tokio event loop",
        )?;
        let tls_connector = HttpsConnector::new(num_cpus::get(), &core.handle())
            .chain_err(|| "Failed to create TLS connector")?;
        let client = Client::configure()
            .connector(tls_connector)
            .keep_alive(keep_alive)
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

    pub fn into_future(self) -> Response {
        let timeout = self.timeout;
        let future = self.endpoint.client.request(self.request).from_err();
        if let Some(t) = timeout {
            let future = timeout_future(future, t);
            let future = Box::new(future);
            return Response { future, timeout };
        }
        let future = Box::new(future);
        Response { future, timeout }
    }
}

pub struct Response {
    future: Box<Future<Item = hyper::Response, Error = Error> + 'static>,
    timeout: Option<Duration>,
}

impl Response {
    // NB: May panic.
    pub fn assert_status(self, status: StatusCode) -> Self {
        let timeout = self.timeout;
        let future = self.map(move |elem| {
            assert_eq!(elem.status(), status);
            elem
        });
        let future = Box::new(future);
        Self { future, timeout }
    }

    pub fn body(self) -> ResponseBody {
        let timeout = self.timeout;
        let future = self.future.and_then(move |elem| {
            let body: Box<Stream<Item = hyper::Chunk, Error = Error>>;
            if let Some(t) = timeout {
                body = Box::new(timeout_stream(elem.body(), t));
            } else {
                body = Box::new(elem.body().from_err());
            }
            body.concat2()
        });
        ResponseBody(Box::new(future))
    }
}

pub struct ResponseBody(Box<Future<Item = hyper::Chunk, Error = Error> + 'static>);

fn to_uri(url: &Url) -> Result<Uri> {
    let uri = url.as_str().parse().chain_err(|| "Failed to parse URL")?;
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

fn timeout_stream<T>(stream: T, timeout: Duration) -> TimeoutStream<stream::FromErr<T, Error>>
where
    T: Stream,
    Error: From<T::Error>,
{
    let timer = Timer::default();
    let stream = timer.timeout_stream(stream.from_err(), timeout);
    stream
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
