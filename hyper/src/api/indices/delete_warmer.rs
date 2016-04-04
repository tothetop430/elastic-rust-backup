//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-warmers.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn delete_index_name<'a>(client: &'a mut Client, req: RequestParams,
                         index: &'a str, name: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(name);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.delete(&url_fmtd).headers(headers);
    res.send()
}
