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

#[macro_use]
extern crate serde_json;

extern crate crest;
extern crate futures;
extern crate hyper;

use crest::{header, Endpoint, Error};
use futures::Future;
use futures::stream::{Concat2, Stream};
use hyper::{Body, Response, StatusCode};
use serde_json::Value;
use std::net::Ipv4Addr;
use std::ops::Deref;
use std::str;

struct Helper {}

impl Helper {
    fn new_endpoint() -> Endpoint {
        Endpoint::new("https://httpbin.org/").unwrap()
    }

    fn status_ok(res: &Response) {
        assert_eq!(res.status(), StatusCode::Ok);
    }

    fn get_concat_body(res: Response) -> Concat2<Body> {
        res.body().concat2()
    }

    fn to_json_value(data: &[u8]) -> Value {
        serde_json::from_slice(data).unwrap()
    }

    fn run_and_get_json_value<T>(endpoint: &mut Endpoint, work: T) -> Value
    where
        T: Future,
        T::Item: Deref<Target = [u8]>,
        Error: From<T::Error>,
    {
        let res = endpoint.run(work).unwrap();
        Helper::to_json_value(&*res)
    }
}

#[test]
fn get_ip() {
    let mut endpoint = Helper::new_endpoint();
    let path = "ip";
    // Given the use of `into_future()`, it isn’t necessary to keep a
    // ref to `endpoint` beyond the following block. Building the
    // request this way avoids ref aliasing.
    let work = {
        let req = endpoint.get(path).unwrap();
        let work = req.into_future().and_then(|res| {
            Helper::status_ok(&res);
            Helper::get_concat_body(res)
        });
        work
    };
    let res = Helper::run_and_get_json_value(&mut endpoint, work);
    let data = res.get("origin").unwrap();
    assert!(data.is_string());
    let ip: Result<Ipv4Addr, _> = data.as_str().unwrap().parse();
    assert!(ip.is_ok());
}

#[test]
fn post_crate_name() {
    let mut endpoint = Helper::new_endpoint();
    let path = "post";
    let body = "crest-next";
    let work = {
        let mut req = endpoint.post(path).unwrap();
        req.headers_mut().set(
            header::ContentLength(body.len() as u64),
        );
        req.set_body(body);
        let work = req.into_future().and_then(|res| {
            Helper::status_ok(&res);
            Helper::get_concat_body(res)
        });
        work
    };
    let res = Helper::run_and_get_json_value(&mut endpoint, work);
    let data = res.get("data").unwrap();
    assert_eq!(*data, json!("crest-next"));
}
