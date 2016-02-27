//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-warmers.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn put_index_name<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                      name: &'a str, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_name<'a>(client: &'a mut Client, base: &'a str, name: &'a str,
                 body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn put_index_type_name<'a>(client: &'a mut Client, base: &'a str,
                           index: &'a str, _type: &'a str, name: &'a str,
                           body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  _type.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn put_name<'a>(client: &'a mut Client, base: &'a str, name: &'a str,
                body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_index_type_name<'a>(client: &'a mut Client, base: &'a str,
                            index: &'a str, _type: &'a str, name: &'a str,
                            body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  _type.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_index_name<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                       name: &'a str, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}