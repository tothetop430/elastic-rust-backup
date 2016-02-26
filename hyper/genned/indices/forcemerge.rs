//Autogenerated
use hyper::Client;
pub fn post(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 12);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_forcemerge");
    url_fmtd
}
pub fn post_index(base: String, index: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 12 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_forcemerge");
    url_fmtd
}
