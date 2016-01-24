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

pub mod error;
pub mod request;

use hyper::client::{
    Client,
    Response,
};
use hyper::method::Method;
use url::Url;

use error::Result;
use request::*;

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
