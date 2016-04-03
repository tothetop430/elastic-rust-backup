//! http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn post_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str,
                  body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 1 + 14 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 1 + 14 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_index_type<'a>(client: &'a mut Client, req: RequestParams,
                       index: &'a str, _type: &'a str, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 1 + 1 + 14 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get_index_type<'a>(client: &'a mut Client, req: RequestParams,
                      index: &'a str, _type: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 1 + 1 + 14 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, req: RequestParams, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 14 + url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, req: RequestParams) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 14 + url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/_mtermvectors");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
