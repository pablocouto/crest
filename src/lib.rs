/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

 */

/*!
_Crest_ is a REST client library built upon [Hyper](http://hyper.rs/).

Its repository can be found [here](https://github.com/pablocouto/crest/).

# Usage

## Making a `GET` request and deserializing the response

The following code first constructs a `GET` request for a resource at
`https://httpbin.org/ip`, and then deserializes the response – in JSON format –
into a custom type.

Note that deserialization is performed by
[serde](https://crates.io/crates/serde/); for more information on how to derive
`Deserialize` for custom types, refer to serde
[documentation](https://github.com/serde-rs/serde).

```
# #![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
# #![cfg_attr(feature = "nightly", plugin(serde_macros))]
#
extern crate crest;
extern crate serde;

use crest::error::Result;
use crest::prelude::*;

# #[cfg(not(feature = "nightly"))]
# include!(concat!(env!("OUT_DIR"), "/type.rs.out"));
#
# #[cfg(feature = "nightly")]
#[derive(Debug, Deserialize)]
struct HttpbinIP {
    origin: String,
}

fn example() -> Result<HttpbinIP> {
    // 1. Construct the endpoint off a base URL
    let endpoint = try!(Endpoint::new("https://httpbin.org/"));

    // 2. Construct the request
    let request = try!(endpoint.get(vec!["ip"]));

    // 3. Perform the request
    let response = try!(request.send());

    // 4. Deserialize the response
    let ip = try!(response.into::<HttpbinIP>());
    # let ip_ = ip.origin.parse::<::std::net::Ipv4Addr>();
    # assert!(ip_.is_ok());

    Ok(ip)
}

# fn main() { example().unwrap(); }
```
!*/

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod prelude {
    /*!
    Crest’s prelude.

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

use std::fmt;
use std::sync::Arc;

use hyper::client::Client;
use url::Url;

use error::Result;
use request::*;

/**
Handle for working with `Request`s. This is the main entry point to the library.
*/
pub struct Endpoint {
    base: Url,
    client: Arc<Client>,
}

impl fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let client_ptr = &self.client as *const Arc<Client>;
        f.debug_struct("Endpoint")
            .field("base", &self.base)
            .field("client as *const", &client_ptr)
            .finish()
    }
}

impl Endpoint {
    /**
    Creates a new `Endpoint` off a `base` URL.

    Requests will be made relative to the given URL.
     */
    pub fn new(base: &str) -> Result<Endpoint> {
        let url = try!(Url::parse(base));
        let client = Arc::new(Client::new());

        Ok(Endpoint {
            base: url,
            client: client,
        })
    }

    /**
    Convenience function to create a `GET` request.
     */
    pub fn get<'a, P>(&'a self, path: P) -> Result<Get<'a>> where
        P: IntoIterator<Item = &'a str>
    {
        Get::new(self, path)
    }

    /**
    Convenience function to create a `POST` request.
     */
    pub fn post<'a, P>(&'a self, path: P) -> Result<Post<'a>> where
        P: IntoIterator<Item = &'a str>
    {
        Post::new(self, path)
    }

    /**
    Convenience function to create a `DELETE` request.
     */
    pub fn delete<'a, P>(&'a self, path: P) -> Result<Delete<'a>>  where
        P: IntoIterator<Item = &'a str>
    {
        Delete::new(self, path)
    }
}
