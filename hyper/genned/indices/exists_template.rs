//Autogenerated
pub fn head_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_template/");
    url_fmtd.push_str(&name);
    url_fmtd
}
