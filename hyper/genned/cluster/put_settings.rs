//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn put(client: &'a mut hyper::Client, base: String, body: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 18);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cluster/settings");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}