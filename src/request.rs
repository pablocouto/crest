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

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use hyper::client::Client;
use hyper::header::{self, Headers};
use hyper::method::Method;
use hyper::mime;
use serde::de::Deserialize;
use serde_json;
use url::form_urlencoded;
use url::Url;

use error::{
    Error,
    Result,
};
use Endpoint;

macro_rules! fn_new {
    ($ty: ident) => (
        /**
        Constructs the request from a given `Endpoint`.

        The `path` argument locates a REST resource; for example, a resource at
        `/status/418` can be represented like this:

        ```
        let resource = vec!["status", "418"];
        ```
         */
        pub fn new<P>(
            endpoint: &'a Endpoint,
            path: P,
        ) -> Result<Self> where
            P: IntoIterator<Item = &'a str>
        {
            let path = path.into_iter().collect::<Vec<_>>().join("/");
            let url = try!(endpoint.base.join(&path));
            let method = Method::$ty;
            let data = Data {
                headers: None,
                body: None,
            };

            Ok($ty {
                endpoint: endpoint,
                method: method,
                url: url,
                data: data,
            })
        }
    )
}

macro_rules! impl_Request_accessors {
    ($ty: ident) => (
        #[doc(hidden)]
        fn get_client(&self) -> Arc<Client> {
            self.endpoint.client.clone()
        }

        #[doc(hidden)]
        fn get_method(&self) -> &Method {
            &self.method
        }

        #[doc(hidden)]
        fn get_url(&self) -> &Url {
            &self.url
        }

        #[doc(hidden)]
        fn get_mut_url(&mut self) -> &mut Url {
            &mut self.url
        }

        #[doc(hidden)]
        fn get_mut_data(&mut self) -> &mut Data {
            &mut self.data
        }

        #[doc(hidden)]
        fn explode(self) -> (Url, Data) {
            (self.url, self.data)
        }
    )
}

macro_rules! impl_Body {
    ($ty: ident) => (
        impl<'a> Body<'a> for $ty<'a> {}
    )
}

/**
Generates an updated parameters list for a request.

If the request already has set parameters, a new list of parameters is
constructed from them, with the passed ones appended. No change is done to the
passed request.
 */
fn updated_parameters<'a, R, I, K, V>(
    request: &R,
    params: I,
) -> Vec<(String, String)> where
    R: Request<'a>,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut params = params.into_iter()
        .map(|item| {
            let &(ref x, ref y) = item.borrow();
            (x.as_ref().into(), y.as_ref().into())
        })
        .collect();

    if let Some(mut found_params) = request.get_url().query_pairs() {
        found_params.append(&mut params);
        found_params
    } else {
        params
    }
}

/**
Affords core request functionality.
 */
pub trait Request<'a> {
    #[doc(hidden)] fn get_client(&self) -> Arc<Client>;
    #[doc(hidden)] fn get_method(&self) -> &Method;
    #[doc(hidden)] fn get_url(&self) -> &Url;
    #[doc(hidden)] fn get_mut_url(&mut self) -> &mut Url;
    #[doc(hidden)] fn get_mut_data(&mut self) -> &mut Data;
    #[doc(hidden)] fn explode(self) -> (Url, Data);

    /**
    Sets the parameters of the request.

    If the request is of type `POST`, the parameters will go in its body, and
    one header will be set to `Content-Type: application/x-www-form-urlencoded`.

    In any other case, the parameters will go in the URL query string, and no
    header will be set.

    Example:

    ```
    # use crest::prelude::*;
    # let endpoint = Endpoint::new("https://httpbin.org/").unwrap();
    # let mut request = endpoint.get(vec!["ip"]).unwrap();
    // assuming a declared `request`
    request.parameters(vec![
        ("param1", "value1"),
        ("param2", "value2"),
    ]);
    ```
     */
    fn parameters<I, K, V>(&mut self, params: I) where
        Self: Sized,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let new_params = updated_parameters(self, params);
        self.get_mut_url().set_query_from_pairs(new_params);
    }

    /**
    Gives a mutable reference to the `Headers` inside a `Request`.

    For example, to declare a header with `Connection: close`:

    ```
    # extern crate hyper;
    # extern crate crest;
    use hyper::header;
    # use crest::prelude::*;

    # fn main() {
    # let endpoint = Endpoint::new("https://httpbin.org/").unwrap();
    # let mut request = endpoint.get(vec!["ip"]).unwrap();
    // assuming a declared `request`
    request.headers().set(header::Connection::close());
    # }
    ```
     */
    fn headers(&mut self) -> &mut Headers {
        let data = self.get_mut_data();

        if let Some(ref mut headers) = data.headers {
            headers
        } else {
            data.headers = Some(Headers::new());
            data.headers.as_mut().unwrap()
        }
    }

    /**
    Performs the request.
     */
    fn send(self) -> Result<Response> where
        Self: Sized
    {
        let body;

        let client = self.get_client();
        let method: Box<Fn(_) -> _> = match *self.get_method() {
            Method::Get => Box::new(|x| client.get(x)),
            Method::Post => Box::new(|x| client.post(x)),
            Method::Delete => Box::new(|x| client.delete(x)),
            _ => unimplemented!(),
        };

        let (url, data) = self.explode();

        let mut request = method(url);
        if let Some(h) = data.headers {
            request = request.headers(h);
        }
        if let Some(b) = data.body {
            body = b;
            request = request.body(&body);
        }

        let response = try!(
            request.send()
                .map_err::<Error, _>(From::from)
        );

        Ok(Response(response))
    }

    /**
    Convenience function to perform a request, deserializing its response.
     */
    fn send_and_into<T>(self) -> Result<T> where
        Self: Sized,
        T: Deserialize
    {
        let response = try!(self.send());
        response.into()
    }
}

/**
Affords setting the body for requests with body semantics.
 */
pub trait Body<'a>: Request<'a> {
    /**
    Sets the body of a `Request`.
     */
    fn body(&mut self, body: &str) {
        self.get_mut_data().body = Some(body.into());
    }
}

/**
Stores data internal to a request.
 */
#[doc(hidden)]
#[derive(Debug)]
pub struct Data {
    headers: Option<Headers>,
    body: Option<String>,
}

/**
Stores the response to a REST request.
 */
#[derive(Debug)]
pub struct Response(::hyper::client::Response);

impl Deref for Response {
    type Target = ::hyper::client::Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Response {
    /**
    Deserializes `Response` into `T`.

    This method assumes that the underlying data in `Response` is in JSON
    format.
     */
    pub fn into<T>(self) -> Result<T> where
        T: Deserialize
    {
        match self.headers.get::<header::ContentType>() {
            Some(h) => match (h.0).1 {
                mime::SubLevel::Json => (),
                _ => return Err(Error::NoJson),
            },
            None =>
                // hoping for the best
                (),
        }

        serde_json::from_reader(self.0)
            .map_err(From::from)
    }
}

/**
A `GET` request.
 */
#[derive(Debug)]
pub struct Get<'a> {
    endpoint: &'a Endpoint,
    method: Method,
    url: Url,
    data: Data,
}

impl<'a> Request<'a> for Get<'a> {
    impl_Request_accessors!(Get);
}

impl<'a> Get<'a> {
    fn_new!(Get);
}

/**
A `POST` request.
 */
#[derive(Debug)]
pub struct Post<'a> {
    endpoint: &'a Endpoint,
    method: Method,
    url: Url,
    data: Data,
}

impl<'a> Request<'a> for Post<'a> {
    impl_Request_accessors!(Post);

    fn parameters<I, K, V>(&mut self, params: I) where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let new_params = updated_parameters(self, params);
        self.get_mut_url().query = None;
        self.headers().set(header::ContentType::form_url_encoded());
        let url_encoded = form_urlencoded::serialize(new_params);
        self.body(&url_encoded);
    }
}

impl_Body!(Post);

impl<'a> Post<'a> {
    fn_new!(Post);
}

/**
A `DELETE` request.
 */
#[derive(Debug)]
pub struct Delete<'a> {
    endpoint: &'a Endpoint,
    method: Method,
    url: Url,
    data: Data,
}

impl<'a> Request<'a> for Delete<'a> {
    impl_Request_accessors!(Delete);
}

impl<'a> Delete<'a> {
    fn_new!(Delete);
}
