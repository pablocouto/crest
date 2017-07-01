# Crest

[![Build Status](https://travis-ci.org/pablocouto/crest.svg?branch=master)](https://travis-ci.org/pablocouto/crest)

_Crest_ is a REST client library, written in Rust.

# Status

It is currently experimental, and incomplete. Pull requests are welcome.

# Installation

_Crest_ is available from Cargo. To use it, add this to
`[dependencies]` in `Cargo.toml`:

```toml
crest = "0.3"
```

# Usage

## Example: Making a `GET` request and deserializing the response

The following code first constructs a `GET` request for a resource at
`https://httpbin.org/ip`, and then deserializes the response – in JSON format –
into a custom type.

Note that deserialization is performed by
[serde](https://crates.io/crates/serde/); for more information on how to derive
`Deserialize` for custom types, refer to serde
[documentation](https://github.com/serde-rs/serde).

```rust
extern crate crest;
extern crate serde;

use crest::error::Result;
use crest::prelude::*;

#[derive(Debug, Deserialize)]
struct HttpbinIP {
    origin: String,
}

fn example() -> Result<HttpbinIP> {
    // 1. Construct the endpoint off a base URL
    let endpoint = try!(Endpoint::new("https://httpbin.org/"));

    // 2. Construct the request
    let request = try!(endpoint.get(&["ip"]));

    // 3. Perform the request
    let response = try!(request.send());

    // 4. Deserialize the response
    let ip = try!(response.into::<HttpbinIP>());

    Ok(ip)
}
```

More documentation is available
[here](https://pablocouto.github.io/crest/crest/index.html).

# License

_Crest_ is licensed under the Apache License, Version 2.0 (see `LICENSE-APACHE`)
or the MIT license (see `LICENSE-MIT`), at your option.
