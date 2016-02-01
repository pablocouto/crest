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

use std::ops::{Deref, DerefMut};

use hyper::client::Client;
use hyper::header::Headers;
use hyper::method::Method;
use serde::de::Deserialize;
use serde_json;
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
            P: IntoIterator<Item = &'a str>,
        Self: Sized
        {
            let path = path.into_iter().collect::<Vec<_>>().join("/");
            let url = try!(endpoint.base.join(&path));
            let method = Method::$ty;
            let data = Data {
                url: url,
                headers: None,
                body: None,
            };

            Ok($ty {
                endpoint: endpoint,
                method: method,
                data: data,
            })
        }
    )
}

macro_rules! impl_Request {
    ($ty: ident) => (
        impl<'a> Request<'a> for $ty<'a> {
            #[doc(hidden)]
            fn get_client(&self) -> &Client {
                &self.endpoint.client
            }

            #[doc(hidden)]
            fn get_method(&self) -> &Method {
                &self.method
            }

            #[doc(hidden)]
            fn get_url(&self) -> Url {
                self.data.url.clone()
            }

            #[doc(hidden)]
            fn get_mut_data(&mut self) -> &mut Data {
                &mut self.data
            }

            #[doc(hidden)]
            fn get_owned_data(self) -> Data {
                self.data
            }
        }
    )
}

macro_rules! impl_Body {
    ($ty: ident) => (
        impl<'a> Body<'a> for $ty<'a> {}
    )
}

/**
Affords default request functionality.
 */
pub trait Request<'a> {
    #[doc(hidden)] fn get_client(&self) -> &Client;
    #[doc(hidden)] fn get_method(&self) -> &Method;
    #[doc(hidden)] fn get_url(&self) -> Url;
    #[doc(hidden)] fn get_mut_data(&mut self) -> &mut Data;
    #[doc(hidden)] fn get_owned_data(self) -> Data;

    /**
    Appends the passed parameters to the HTTP query.

    Parameters may be stored like this:

    ```
    let params = vec![
        ("param1", "value1"),
        ("param2", "value2"),
    ];
    ```
     */
    fn parameters<P>(&mut self, params: P) where
        P: IntoIterator<Item = (&'a str, &'a str)>
    {
        let data = self.get_mut_data();

        let mut params = params.into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();

        let new_params;
        if let Some(mut found_params) = data.url.query_pairs() {
            found_params.append(&mut params);
            new_params = found_params
        } else {
            new_params = params
        };
        data.url.set_query_from_pairs(new_params);
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

        let client: &Client = unsafe { &*(self.get_client() as *const _) };
        let method: Box<Fn(_) -> _> = match *self.get_method() {
            Method::Get => Box::new(|x| client.get(x)),
            Method::Post => Box::new(|x| client.post(x)),
            Method::Delete => Box::new(|x| client.delete(x)),
            _ => unimplemented!(),
        };

        let url = self.get_url();
        let mut request = method(url);

        let data = self.get_owned_data();

        if let Some(h) = data.headers {
            request = request.headers(h);
        }

        if let Some(b) = data.body {
            body = b;
            request = request.body(&body);
        }

        let response = try!(request
                            .send()
                            .map_err::<Error, _>(From::from));

        Ok(Response { inner: response })
    }

    /**
    Convenience function to perform a request, deserializing its response.
     */
    fn send_and_into<T>(self) -> Result<T> where
        T: Deserialize,
        Self: Sized
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
    fn body(&mut self, body: &'a str) {
        self.get_mut_data().body = Some(body.into());
    }
}

/**
Stores data internal to a request.
 */
#[doc(hidden)]
#[derive(Debug)]
pub struct Data {
    url: Url,
    headers: Option<Headers>,
    body: Option<String>,
}

/**
Contains the response to a REST request.
 */
#[derive(Debug)]
pub struct Response {
    inner: ::hyper::client::Response,
}

impl Deref for Response {
    type Target = ::hyper::client::Response;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
        serde_json::from_reader(self.inner)
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
    data: Data,
}

impl_Request!(Get);

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
    data: Data,
}

impl_Request!(Post);
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
    data: Data,
}

impl_Request!(Delete);

impl<'a> Delete<'a> {
    fn_new!(Delete);
}
