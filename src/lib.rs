//! Elasticsearch Request Types
//!
//! An implementation of the Elasticsearch REST API using strong types for endpoints.
//!
//! The source is automatically generated from the official spec.
//! A `struct` is provided for each endpoint that works with borrowed or owned data.
//! There's also a more general `HttpRequest` type that all requests can be converted into.
//!
//! # Supported Versions
//!
//!  `elastic_requests` | Elasticsearch
//!  ------------------ | -------------
//!  `0.x`              | `5.x`
//!
//! # Usage
//!
//! All request types provide constructor functions of the form
//! `param_1_param_2_param_n`:
//!
//! ```
//! # use elastic_requests::*;
//! let req = SearchRequest::index_ty(
//!     "test_index",
//!     "test_ty",
//!     "{'query': { 'match_all': {}}}"
//! );
//!
//! assert_eq!("/test_index/test_ty/_search", req.url.as_ref());
//! ```
//!
//! Or `new` if the endpoint takes no parameters:
//!
//! ```
//! # use elastic_requests::*;
//! let req = PingRequest::new();
//!
//! assert_eq!("/", req.url.as_ref());
//! ```
//!
//! Parameters can be borrowed or owned string values:
//!
//! ```
//! # use elastic_requests::*;
//! let req = SearchRequest::index(
//!     "test_index".to_string(),
//!     "{'query': { 'match_all': {}}}"
//! );
//!
//! assert_eq!("/test_index/_search", req.url.as_ref());
//! ```
//!
//! # Why are these docs useless?
//!
//! This library is automatically generated, so there's a lot more work to do
//! to get the documentation up to scratch.

mod genned;

pub use genned::*;

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    fn do_something_with_request<'a, I: Into<HttpRequest<'a>>>(_: I) {}

    fn do_something_with_static_request<I: Into<HttpRequest<'static>>>
        (req: I)
         -> thread::JoinHandle<()> {
        let req = req.into();
        thread::spawn(move || {
            assert_eq!("/test_index/test_ty/_search", **req.url);
        })
    }

    #[test]
    fn it_works() {
        let req = SearchRequest::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        assert_eq!("/test_index/test_ty/_search", *req.url);

        do_something_with_request(&req);
        do_something_with_request(req);
    }

    #[test]
    fn it_works_static() {
        let req = SearchRequest::index_ty(String::from("test_index"), "test_ty", Body::none());

        do_something_with_static_request(req).join().unwrap();
    }
}
