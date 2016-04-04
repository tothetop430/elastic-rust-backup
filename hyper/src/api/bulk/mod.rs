//! http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html

//Autogenerated

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn post_index_type<'a>(client: &'a mut Client, req: RequestParams,
                       index: &'a str, _type: &'a str, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 6 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.post(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
pub fn put<'a>(client: &'a mut Client, req: RequestParams, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 6 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.put(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
pub fn put_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str,
                 body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 6 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.put(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
pub fn put_index_type<'a>(client: &'a mut Client, req: RequestParams,
                      index: &'a str, _type: &'a str, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 6 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.put(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, req: RequestParams, body: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 6 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.post(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
pub fn post_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str,
                  body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 6 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_bulk");
    url_fmtd.push_str(url_qry);
    let res = client.post(&url_fmtd).headers(req.headers).body(body);
    res.send()
}
