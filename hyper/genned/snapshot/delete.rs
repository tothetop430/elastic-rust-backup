//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn delete_repository_snapshot(client: &'a mut hyper::Client, base: String,
                              repository: String, snapshot: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + 1 + repository.len() +
                                  snapshot.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(&repository);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&snapshot);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.delete(&url_fmtd).headers(headers);
    res.send()
}
