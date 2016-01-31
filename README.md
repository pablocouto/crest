# Crest

[![Build Status](https://travis-ci.org/pablocouto/crest.svg?branch=master)](https://travis-ci.org/pablocouto/crest)

_Crest_ is a REST client library, written in Rust.

# Status

It is currently experimental, and incomplete. Pull requests are welcome.

# Installation

_Crest_ is available from Cargo. If you are using stable Rust, add this to
`[dependencies]` in `Cargo.toml`:

```
crest = "0.3"
```

If you are using nightly Rust, this is needed instead:

```
[dependencies.crest]
version = "0.3"
default-features = false
features = ["nightly"]
```

# Usage

Documentation is available
[here](https://pablocouto.github.io/crest/crest/index.html).

# License

_Crest_ is licensed under the Apache License, Version 2.0 (see `LICENSE-APACHE`)
or the MIT license (see `LICENSE-MIT`), at your option.
