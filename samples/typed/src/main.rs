//! Elasticsearch Hyper Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This sample executes a simple search request against some indexed data.
//! The data is provided by a struct with mapping generated by `elastic_types`.

#![feature(plugin, proc_macro)]
#![plugin(json_str)]

extern crate serde;
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate elastic_types;
#[macro_use]
extern crate elastic_types_derive;
extern crate elastic_reqwest;
extern crate elastic_requests;

use std::net::Ipv4Addr;
use reqwest::Client;
use elastic_reqwest::{ElasticClient, RequestParams};
use elastic_requests::{IndicesCreateRequest, IndexRequest, SearchRequest};
use elastic_types::prelude::*;

mod data;
use data::*;
mod response;
use response::*;

const INDEX: &'static str = "testidx";

fn main() {
    // Create a new client
    let (client, params) = elastic_reqwest::default().unwrap();

    // Wait for refresh when indexing data.
    // Normally this isn't a good idea, but is ok for this example.
    let index_params = RequestParams::default().url_params(vec![("refresh", String::from("true"))]);

    // Create an index and map our type
    create_index(&client, &params);

    // Index some objects.
    // For lots of data, prefer the `BulkRequest`
    for t in get_data() {
        index_datum(&client, &index_params, &t);
    }

    // Perform a search request and deserialise to `SearchResponse`.
    let res = query(&client, &params);

    println!("results: {}", res.hits.total);

    for hit in res.hits.hits {
        if let Some(hit) = hit.source {
            println!("hit: {:?}", hit);
        }
    }
}

fn create_index(client: &Client, params: &RequestParams) {
    let req = IndicesCreateRequest::for_index(INDEX, serde_json::to_string(&Index::default()).unwrap());

    // Create index
    client.elastic_req(&params, req).unwrap();
}

fn get_data() -> Vec<MyStruct> {
    vec![MyStruct {
             id: 1,
             title: String::from("Some Title"),
             timestamp: Date::now(),
             geo: GeoLocation {
                 ip: Ipv4Addr::new(10, 0, 0, 1),
                 loc: GeoPoint::build(-71.34, 41.12),
             },
         },
         MyStruct {
             id: 2,
             title: String::from("Some Other Title"),
             timestamp: Date::now(),
             geo: GeoLocation {
                 ip: Ipv4Addr::new(10, 0, 0, 2),
                 loc: GeoPoint::build(-71.34, 41.12),
             },
         }]
}

fn index_datum(client: &Client, params: &RequestParams, datum: &MyStruct) {
    let req = IndexRequest::for_index_ty_id(INDEX,
                                            MyStruct::name(),
                                            datum.id.to_string(),
                                            serde_json::to_string(&datum).unwrap());

    client.elastic_req(&params, req).unwrap();
}

fn query(client: &Client, params: &RequestParams) -> SearchResponse<MyStruct> {
    let req = SearchRequest::for_index_ty(INDEX,
                                          MyStruct::name(),
                                          json_lit!({
                                                query: {
                                                    query_string: {
                                                        query: "*"
                                                    }
                                                }
                                          }));

    let res = client.elastic_req(&params, req).unwrap();

    serde_json::de::from_reader(res).unwrap()
}
