//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn get_node_id<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
                   node_id: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + node_id.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id_metric<'a>(client: &'a mut Client, req: RequestParams,
                          base: &'a str, node_id: &'a str, metric: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 1 + node_id.len() +
                                  metric.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_metric<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
                  metric: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + metric.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, req: RequestParams, base: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd = String::with_capacity(base.len() + 7 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
