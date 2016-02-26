//Autogenerated
use hyper::Client;
pub fn put_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 9 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 9 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn put_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn put_index_type_name(base: String, index: String, type: String,
                       name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_index_type_name(base: String, index: String, type: String,
                        name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn put_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn put_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn put_index_type_name(base: String, index: String, type: String,
                       name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn post_index_type_name(base: String, index: String, type: String,
                        name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    url_fmtd
}
