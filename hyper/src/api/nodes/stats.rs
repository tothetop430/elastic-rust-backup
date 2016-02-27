//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html

// Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 13);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id<'a>(client: &'a mut Client,
                       base: &'a str,
                       node_id: &'a str)
                       -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 8 + 6 + node_id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id_metric<'a>(client: &'a mut Client,
                              base: &'a str,
                              node_id: &'a str,
                              metric: &'a str)
                              -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 8 + 7 + node_id.len() + metric.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats/");
    url_fmtd.push_str(metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_metric<'a>(client: &'a mut Client, base: &'a str, metric: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 14 + metric.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats/");
    url_fmtd.push_str(metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_metric_index_metric<'a>(client: &'a mut Client,
                                   base: &'a str,
                                   metric: &'a str,
                                   index_metric: &'a str)
                                   -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 14 + 1 + metric.len() +
                                             index_metric.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index_metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id_metric_index_metric<'a>(client: &'a mut Client,
                                           base: &'a str,
                                           node_id: &'a str,
                                           metric: &'a str,
                                           index_metric: &'a str)
                                           -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 8 + 7 + 1 + node_id.len() + metric.len() +
                                             index_metric.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index_metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
