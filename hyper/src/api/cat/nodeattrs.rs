//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html

// Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 15);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/nodeattrs");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
