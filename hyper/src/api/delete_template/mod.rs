//! http://www.elastic.co/guide/en/elasticsearch/reference/master/search-template.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn delete_id<'a>(client: &'a mut Client, req: RequestParams, id: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 18 + id.len() +
                                  url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/_search/template/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.delete(&url_fmtd).headers(headers);
    res.send()
}
