//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn delete_repository<'a>(client: &'a mut Client, req: RequestParams,
                         repository: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + repository.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(repository);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.delete(&url_fmtd).headers(headers);
    res.send()
}
