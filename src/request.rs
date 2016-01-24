/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

 */

#[doc(no_inline)]
pub use hyper::header::Headers;

use hyper::method::Method;

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
    pub method: Method,
    pub path: &'a [&'a str],
    pub headers: Headers,
    pub body: Option<&'a str>,
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
