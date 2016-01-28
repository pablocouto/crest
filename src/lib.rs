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
extern crate hyper;
extern crate crest;

use crest::prelude::*;

fn main() {
    // 1. Construct the endpoint off a base URL
    let endpoint = Endpoint::new("https://httpbin.org/").unwrap();

    // 2. Construct the request
    let resource = ["status", "418"];
    let request = endpoint.get(&resource);

    // 3. Perform the request
    let response = request.send().unwrap();

    assert_eq!(response.status, ::hyper::status::StatusCode::ImATeapot);
}
```
!*/

extern crate hyper;
extern crate url;

pub mod prelude {
    /*!
    To ease getting _Crest_’s main entities into scope.

    ```
    use crest::prelude::*;
    ```
     */
    pub use request::{
        Body,
        Request,
    };
    pub use Endpoint;
}

pub mod error;
pub mod request;

use hyper::client::Client;
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
    Convenience function to create a `GET` request.
     */
    pub fn get<'a>(&'a self, path: &'a [&'a str]) -> Get<'a> {
        Get::new(self, path)
    }

    /**
    Convenience function to create a `POST` request.
     */
    pub fn post<'a>(&'a self, path: &'a [&'a str]) -> Post<'a> {
        Post::new(self, path)
    }

    /**
    Convenience function to create a `DELETE` request.
     */
    pub fn delete<'a>(&'a self, path: &'a [&'a str]) -> Delete<'a> {
        Delete::new(self, path)
    }
}
