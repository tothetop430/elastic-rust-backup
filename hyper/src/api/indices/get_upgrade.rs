//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get<'a>(client: &'a mut Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 9);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_upgrade");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index<'a>(client: &'a mut Client, base: String, index: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_upgrade");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
