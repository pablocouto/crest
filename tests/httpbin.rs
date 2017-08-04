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

use crest::{Endpoint, Error};
use futures::Future;
use futures::stream::Stream;
use hyper::{header, Response, StatusCode};
use serde_json::Value;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use std::ops::Deref;
use std::str;

struct Helper {}

impl Helper {
    fn new_endpoint() -> Endpoint {
        let keep_alive = true;
        Endpoint::new("https://httpbin.org/", keep_alive).unwrap()
    }

    fn status_ok(res: &Response) {
        assert_eq!(res.status(), StatusCode::Ok);
    }

    fn get_concat_body(res: Response) -> Box<Future<Item = hyper::Chunk, Error = Error>> {
        let body = res.body().concat2().from_err();
        Box::new(body)
    }

    fn to_json_value(data: &[u8]) -> Value {
        serde_json::from_slice(data).unwrap()
    }

    fn run_and_get_json_value<T>(endpoint: &mut Endpoint, work: T) -> Value
    where
        T: Future,
        T::Item: Deref<Target = [u8]>,
        T::Error: Debug,
    {
        let res = endpoint.run(work).unwrap();
        Helper::to_json_value(&*res)
    }
}

#[test]
fn get_ip() {
    let mut endpoint = Helper::new_endpoint();
    let path = "ip";
    let work = endpoint.get(path).unwrap().into_future().and_then(|res| {
        Helper::status_ok(&res);
        Helper::get_concat_body(res)
    });
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
    let work = endpoint
        .post(path)
        .unwrap()
        .header(header::ContentLength(body.len() as u64))
        .body(body)
        .into_future()
        .and_then(|res| {
            Helper::status_ok(&res);
            Helper::get_concat_body(res)
        });
    let res = Helper::run_and_get_json_value(&mut endpoint, work);
    let data = res.get("data").unwrap();
    assert_eq!(*data, json!("crest-next"));
}
