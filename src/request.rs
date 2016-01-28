/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

 */

/*!
REST requests.
*/

use hyper::header::Headers;
use hyper::client::Response;

use error::Result;
use Endpoint;

macro_rules! impl_Request {
    ($ty: ident, $method: ident) => (
        impl<'a> Request<'a> for $ty<'a> {
            fn new<P>(
                endpoint: &'a Endpoint,
                path: P,
            ) -> Self where
                P: IntoIterator<Item = &'a str>
            {
                let path = path.into_iter().collect();
                let data = Data {
                    path: path,
                    headers: Headers::new(),
                    body: None,
                };

                $ty {
                    endpoint: endpoint,
                    data: data,
                }
            }

            fn headers(&mut self) -> &mut Headers {
                &mut self.data.headers
            }

            fn send(self) -> Result<Response> {
                let path = self.data.path.join("/");
                let url = self.endpoint.base.join(&path).unwrap();

                let mut request = self.endpoint.client
                    .$method(url)
                    .headers(self.data.headers);

                if self.data.body.is_some() {
                    let body = self.data.body.unwrap();
                    request = request.body(body);
                }

                let response = request
                    .send()
                    .map_err(From::from);

                response
            }
        }
    )
}

pub trait Request<'a> {
    /**
    Constructs a REST request from a given `Endpoint`.

    The `path` argument locates a REST resource; for example, a resource at
    `/status/418` can be referenced like this:

    ```
    let resource = &["status", "418"];
    ```
     */
    fn new<P>(
        endpoint: &'a Endpoint,
        path: P,
    ) -> Self where
        P: IntoIterator<Item = &'a str>;

    /**
    Gives a mutable reference to the `Headers` inside a `Request`.

    For example, to declare a header with `Connection: close`:

    ```
    # extern crate hyper;
    # extern crate crest;
    use hyper::header;
    use crest::prelude::*;

    # fn main() {
    let endpoint = Endpoint::new("https://httpbin.org/").unwrap();
    let resource = ["ip"];
    let mut request = endpoint.get(&resource);
    request.headers().set(header::Connection::close());
    # }
    ```
     */
    fn headers(&mut self) -> &mut Headers;

    /**
    Performs the request.
     */
    fn send(self) -> Result<Response>;
}

pub trait Body<'a> where
    Self: Request<'a>
{
    /**
    Sets the body of a `Request`.
     */
    fn body(&mut self, body: &'a str);
}

struct Data<'a> {
    path: Vec<&'a str>,
    headers: Headers,
    body: Option<&'a str>,
}

/**
A `GET` request.
 */
pub struct Get<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Get, get);

/**
A `POST` request.
 */
pub struct Post<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Post, post);

impl<'a> Body<'a> for Post<'a> {
    fn body(&mut self, body: &'a str) {
        self.data.body = Some(body);
    }
}

/**
A `DELETE` request.
 */
pub struct Delete<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Delete, delete);
