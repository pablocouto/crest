#!/bin/bash

set -ev

if [ "${TRAVIS_RUST_VERSION}" = "nightly" ]; then
    travis-cargo build -- --no-default-features &&
    travis-cargo test -- --no-default-features &&
    travis-cargo bench -- --no-default-features
else
    travis-cargo build &&
    travis-cargo test &&
    travis-cargo bench &&
    travis-cargo --only stable doc
fi
