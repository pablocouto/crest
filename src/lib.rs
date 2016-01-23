/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

*/

/*!
_Crest_ is a REST client library built upon [Hyper](http://hyper.rs/).

# Usage

## Making a `GET` request

```
extern crate crest;
extern crate hyper;

use crest::*;

fn main() {
    // 1. Construct the endpoint with a base URL
    let endpoint = Endpoint::new("https://httpbin.org/").unwrap();

    // 2. Declare the request
    let path = ["status", "418"];
    let request = Request::get(&path);

    // 3. Make the request
    let response = endpoint.send(request).unwrap();

    assert_eq!(response.status, ::hyper::status::StatusCode::ImATeapot);
}
```
!*/

extern crate hyper;
extern crate url;

mod error;

use hyper::client::{
    Client,
    Response,
};
use hyper::method::Method;
use url::Url;

#[doc(no_inline)]
pub use hyper::header::Headers;

use error::Result;

/**
Used to declare a REST request.

The `path` argument is common to many of the functions below. This refers to the
path of the REST resource; for example, a resource `/status/418` can be
represented in this way:

```
let resource = &["status", "418"];
```
*/
pub struct Request<'a> {
    method: Method,
    path: &'a [&'a str],
    headers: Headers,
    body: Option<&'a str>,
}

impl<'a> Request<'a> {
    /**
    Declares a `GET` request.
     */
    pub fn get(path: &'a [&'a str]) -> Request<'a> {
        Request {
            method: Method::Get,
            path: path,
            headers: Headers::new(),
            body: None,
        }
    }

    /**
    Declares a `POST` request.
     */
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

    /**
    Declares a `DELETE` request.
     */
    pub fn delete(path: &'a [&'a str]) -> Request<'a> {
        Request {
            method: Method::Delete,
            path: path,
            headers: Headers::new(),
            body: None,
        }
    }

    /**
    Gives a mutable reference to the `Headers` inside a `Request`.

    For example, to declare a header with `Connection: close`:

    ```
    # extern crate crest;
    # extern crate hyper;
    # use crest::*;
    use hyper::header;

    # fn main() {
    let resource = ["ip"];
    let mut request = Request::get(&resource);
    request.headers().set(header::Connection::close());
    # }
    ```
     */
    pub fn headers(&mut self) -> &mut Headers {
        &mut self.headers
    }

    /**
    Sets the body of a `Request`.
     */
    pub fn body(&mut self, body: &'a str) {
        self.body = Some(body);
    }
}

/**
Handle for working with `Request`s.
*/
pub struct Endpoint {
    base: Url,
    client: Client,
}

impl Endpoint {
    /**
    Creates a new `Endpoint` off a `base` URL.

    Requests will be made relative to the given URL.
     */
    pub fn new(base: &str) -> Result<Endpoint> {
        let url = try!(Url::parse(base));
        let client = Client::new();

        Ok(Endpoint {
            base: url,
            client: client,
        })
    }

    /**
    Sends the given `Request`.
     */
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
