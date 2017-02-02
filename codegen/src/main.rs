#![feature(proc_macro)]

#[cfg(test)]
#[macro_use]
extern crate json_str;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate syn;
#[macro_use]
extern crate quote;

extern crate inflector;

pub mod parse;
pub mod gen;

use std::collections::BTreeMap;
use std::io::{stdout, Read, Write};
use std::fs::{File, read_dir};

use quote::Tokens;
use parse::*;

fn start_comment_block_for_logging() {
    stdout().write(b"/*").unwrap();
}

fn end_comment_block_for_logging() {
    stdout().write(b"*/").unwrap();
}

fn from_dir(path: &str) -> Result<Vec<(String, Endpoint)>, String> {
    let mut all_parsed: Vec<(String, Endpoint)> = Vec::new();

    let paths = read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let display = path.to_string_lossy().into_owned();

        let mut f = File::open(path).unwrap();
        let parsed = try!(from_reader(display, &mut f));

        all_parsed.push(parsed);
    }

    Ok(all_parsed)
}

fn from_reader<R>(name: String, rdr: &mut R) -> Result<(String, Endpoint), String>
    where R: Read
{
    let endpoint: BTreeMap<String, Endpoint> = try!(serde_json::from_reader(rdr)
        .map_err(|e| format!("Failed to parse {} because: {}", name, e)));

    Ok(endpoint.endpoint())
}

fn strip_verbs(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    // Choose a single HTTP verb per endpoint: either POST or 1st entry
    let mut iter = endpoint.methods.into_iter();
    let verb = match iter.len() {
        0 => unreachable!(),
        1 => iter.next().unwrap(),
        _ => {
            if iter.any(|m| m == HttpMethod::Post) {
                HttpMethod::Post
            } else {
                iter.next().unwrap()
            }
        }
    };

    endpoint.methods = vec![verb];

    (name, endpoint)
}

fn dedup_urls(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    let mut deduped_paths = BTreeMap::new();

    for path in endpoint.url.paths {
        let key = path.params().join("");

        deduped_paths.insert(key, path);
    }

    endpoint.url.paths = deduped_paths.into_iter()
        .map(|(_, p)| p)
        .collect();

    (name, endpoint)
}

fn add_simple_search(endpoints: &mut Vec<(String, Endpoint)>) {
    let mut endpoint = {
        let &(_, ref endpoint) = endpoints
            .iter()
            .find(|ref endpoint| endpoint.0 == "search")
            .unwrap();

        endpoint.clone()
    };

    let name = String::from("simple_search");

    endpoint.methods = vec![HttpMethod::Get];
    endpoint.body = None;

    endpoints.push((name, endpoint));
}

fn main() {
    start_comment_block_for_logging();

    println!("This code is automatically generated");

    let dir = "./spec";

    // BTreeMap<String, bool> : <type name, is emitted>
    let mut params_to_emit = BTreeMap::new();
    params_to_emit.insert(String::from("vertices"), false);

    let mut derives = Tokens::new();
    derives.append("#[derive(Debug, PartialEq, Clone)]");

    let mut tokens = quote::Tokens::new();

    let mut endpoints: Vec<(String, Endpoint)> = from_dir(dir)
        .expect("Couldn't parse the REST API spec");

    add_simple_search(&mut endpoints);

    endpoints = endpoints    
        .into_iter()
        .map(|e| strip_verbs(e))
        .map(|e| dedup_urls(e))
        .collect();

    tokens.append("pub mod requests {");

        let uses = quote!(
            use super::params::*;
            use std::borrow::Cow;
        );

        tokens.append(uses.to_string().as_ref());
        tokens.append("\n\n");

        for e in endpoints {
            for (ty, _) in &e.1.url.parts {
                params_to_emit.insert(ty.to_owned(), true);
            }

            let url_params = gen::url_params::UrlParamBuilder::from(&e).build();
            let (ref url_params_item, _) = url_params;

            let (req_params_item, req_params_ty) =
                gen::request_params::RequestParamBuilder::from(&e).build();

            let req_ctors_item =
                gen::request_ctors::RequestParamsCtorBuilder::from((&e, &req_params_ty, &url_params))
                    .build();

            let url_method_item =
                gen::url_builder::UrlMethodBuilder::from((&e, &url_params)).build();

            let req_into_http_item =
                gen::request_into_http::RequestIntoHttpRequestBuilder::from((&e, &req_params_ty))
                    .build();

            tokens.append_all(vec![
                derives.clone(),
                quote!(#url_params_item),
                quote!(#url_method_item),
                derives.clone(),
                quote!(#req_params_item),
                quote!(#req_ctors_item),
                quote!(#req_into_http_item)
            ]);
        }

    tokens.append("}");

    tokens.append("\n\n");

    tokens.append("pub mod params {");

        let uses = quote!(
            use std::ops::Deref;
            use std::borrow::Cow;
        );

        tokens.append(uses.to_string().as_ref());
        tokens.append("\n\n");

        let url_tokens = gen::types::url::tokens();
        let body_tokens = gen::types::body::tokens();
        let http_method_item = gen::types::request::method_item();
        let http_req_item = gen::types::request::req_tokens();

        tokens.append_all(vec![
            derives.clone(),
            url_tokens, 
            derives.clone(),
            body_tokens, 
            derives.clone(),
            http_req_item, 
            derives.clone(),
            quote!(#http_method_item)
        ]);

        let params_to_emit = params_to_emit.iter()
            .filter(|&(_, is_emitted)| *is_emitted);

        for (ty, _) in params_to_emit {
            let ty_item = gen::types::wrapped_ty::item(ty);

            tokens.append_all(vec![
                derives.clone(),
                quote!(#ty_item)
            ]);

            tokens.append("\n\n");
        }

    tokens.append("}");

    end_comment_block_for_logging();

    stdout().write(tokens.to_string().as_bytes()).unwrap();
}
