//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn post_repository_snapshot<'a>(client: &'a mut Client, base: &'a str,
                                repository: &'a str, snapshot: &'a str,
                                body: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + 1 + 9 + repository.len() +
                                  snapshot.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(repository);
    url_fmtd.push_str("/");
    url_fmtd.push_str(snapshot);
    url_fmtd.push_str("/_restore");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
