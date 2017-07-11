#[macro_use]
extern crate serde_json;

extern crate crest_next;
extern crate futures;
extern crate hyper;

use crest_next::Endpoint;
use crest_next::error::Error;
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
        Endpoint::new("http://httpbin.org/").unwrap()
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
    let work = endpoint.get(path).unwrap().and_then(|res| {
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
    let work = endpoint.post(path, body).unwrap().and_then(|res| {
        Helper::status_ok(&res);
        Helper::get_concat_body(res)
    });
    let res = Helper::run_and_get_json_value(&mut endpoint, work);
    let data = res.get("data").unwrap();
    assert_eq!(*data, json!("crest-next"));
}
