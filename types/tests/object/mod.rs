#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::object_fixtures::*;

#[test]
fn serialise_mapping_type() {
	//Define an instance of our mapping type
	let ser = TypeMapper::to_string(MyTypeMapping).unwrap();

	let expected = json_str!({
		"properties":{
			"my_date1":{
				"type":"date",
				"format":"yyyy-MM-dd'T'HH:mm:ssZ"
			},
			"my_date2":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_date3":{
				"type":"date",
				"boost":1.01,
				"doc_values":true,
				"include_in_all":false,
				"index":"no",
				"store":true,
				"format":"epoch_millis",
				"ignore_malformed":true,
				"null_value":"1426351513778",
				"precision_step":6
			},
			"my_string1":{
				"type":"text",
				"fields":{
					"keyword":{
						"type":"keyword",
						"ignore_above":256
					}
				}
			},
			"my_string2":{
				"type":"text"
			},
			"my_string3":{
				"type":"keyword"
			},
			"my_num1":{
				"type":"integer"
			},
			"my_num2":{
				"type":"integer",
				"coerce":true,
				"boost":1.1,
				"doc_values":false,
				"ignore_malformed":true,
				"include_in_all":true,
				"null_value":42,
				"precision_step":2147483647,
				"store":true
			},
			"my_bool1":{
				"type":"boolean"
			},
			"my_bool2":{
				"type":"boolean",
				"boost":1.01,
				"doc_values":true,
				"index":"no",
				"store":true,
				"null_value":false
			},
			"my_geo":{
				"type":"geo_shape",
				"tree":"geohash",
				"precision":"50m",
				"tree_levels":8,
				"strategy":"recursive",
				"distance_error_pct":0.5,
				"orientation":"cw",
				"points_only":false
			},
			"my_ips":{
				"type":"ip"
			},
			"my_map1":{
				"type":"object"
			},
			"my_map2":{
				"type":"object"
			},
			"my_val":{
				"type":"object"
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_as_value() {
	//Define an instance of our mapping type
	let value = TypeMapper::to_value(MyTypeMapping).unwrap();

	//Serialise to a json value, and perform a lookup. Make sure the value is serialised properly
	//There does seem to be some weird thing with f32 serialisation; 1.01 results in ~1.0999
	let ser = serde_json::ser::to_string(&value.lookup("properties.my_date2").unwrap()).unwrap();

	let expected = json_str!({
		"format":"basic_date_time",
		"type":"date"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_type_as_nested() {
	//Define an instance of our mapping type
	let ser = TypeMapper::to_string(MyOtherTypeMapping).unwrap();

	let expected = json_str!({
		"properties":{
			"my_date":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_renamed_type":{
				"type":"nested",
				"dynamic":true,
				"enabled":false,
				"include_in_all":true,
				"properties":{
					"my_date1":{
						"type":"date",
						"format":"yyyy-MM-dd'T'HH:mm:ssZ"
					},
					"my_date2":{
						"type":"date",
						"format":"basic_date_time"
					},
					"my_date3":{
						"type":"date",
						"boost":1.01,
						"doc_values":true,
						"include_in_all":false,
						"index":"no",
						"store":true,
						"format":"epoch_millis",
						"ignore_malformed":true,
						"null_value":"1426351513778",
						"precision_step":6
					},
					"my_string1":{
						"type":"text",
						"fields":{
							"keyword":{
								"type":"keyword",
								"ignore_above":256
							}
						}
					},
					"my_string2":{
						"type":"text"
					},
					"my_string3":{
						"type":"keyword"
					},
					"my_num1":{
						"type":"integer"
					},
					"my_num2":{
						"type":"integer",
						"coerce":true,
						"boost":1.1,
						"doc_values":false,
						"ignore_malformed":true,
						"include_in_all":true,
						"null_value":42,
						"precision_step":2147483647,
						"store":true
					},
					"my_bool1":{
						"type":"boolean"
					},
					"my_bool2":{
						"type":"boolean",
						"boost":1.01,
						"doc_values":true,
						"index":"no",
						"store":true,
						"null_value":false
					},
					"my_geo":{
						"type":"geo_shape",
						"tree":"geohash",
						"precision":"50m",
						"tree_levels":8,
						"strategy":"recursive",
						"distance_error_pct":0.5,
						"orientation":"cw",
						"points_only":false
					},
					"my_ips":{
						"type":"ip"
					},
					"my_map1":{
						"type":"object"
					},
					"my_map2":{
						"type":"object"
					},
					"my_val":{
						"type":"object"
					}
				}
			},
			"my_num":{
				"type":"integer"
			},
			"my_point":{
				"type":"geo_point"
			},
			"my_shape":{
				"type":"geo_shape"
			},
			"my_ip":{
				"type":"ip",
				"boost":1.01,
				"doc_values":true,
				"index":"no",
				"store":true,
				"null_value":"127.0.0.1"
			},
			"my_strings":{
				"type":"text",
				"fields":{
					"keyword":{
						"type":"keyword",
						"ignore_above":256
					}
				}
			},
			"my_dates":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_brw_ip":{
				"type":"ip"
			},
			"my_brw_point":{
				"type":"geo_point"
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_dynamic() {
	let d_opts: Vec<String> = vec![
		Dynamic::True,
		Dynamic::False,
		Dynamic::Strict
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#"true"#,
		r#"false"#,
		r#""strict""#
	];

	let mut success = true;
	for i in 0..d_opts.len() {
		if expected_opts[i] != d_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialize_mapping_rses() {
	let mut serialiser = rs_es_map::Serializer::default();
	let ser = RsesMapper::to_value(MyTypeMapping, &mut serialiser).unwrap();

	let expected = hashmap! {
		"my_string3" => hashmap! {
			"type" => "keyword"
		}, 
		"my_date3" => hashmap! {
			"precision_step" => "6", 
			"doc_values" => "true", 
			"ignore_malformed" => "true", 
			"store" => "true", 
			"null_value" => "1426351513778", 
			"format" => "epoch_millis", 
			"type" => "date", 
			"index" => "no", 
			"boost" => "1.01", 
			"include_in_all" => "false"
		}, 
		"my_num1" => hashmap! {
			"type" => "integer"
		}, 
		"my_bool2" => hashmap! {
			"doc_values" => "true", 
			"store" => "true", 
			"null_value" => "false", 
			"type" => "boolean", 
			"index" => "no", 
			"boost" => "1.01"
		}, 
		"my_map2" => hashmap! {
			"type" => "object"
		}, 
		"my_map1" => hashmap! {
			"type" => "object"
		}, 
		"my_num2" => hashmap! {
			"coerce" => "true", 
			"precision_step" => "2147483647", 
			"doc_values" => "false", 
			"ignore_malformed" => "true", 
			"store" => "true", 
			"null_value" => "42", 
			"type" => "integer", 
			"boost" => "1.1", 
			"include_in_all" => "true"
		}, 
		"my_geo" => hashmap! {
			"distance_error_pct" => "0.5", 
			"tree_levels" => "8", 
			"strategy" => "recursive", 
			"type" => "geo_shape", 
			"points_only" => "false", 
			"tree" => "geohash", 
			"precision" => "50m", 
			"orientation" => "cw"
		}, 
		"my_date1" => hashmap! {
			"format" => "yyyy-MM-dd\'T\'HH:mm:ssZ", 
			"type" => "date"
		}, 
		"my_string1" => hashmap! {
			"type" => "text"
		}, 
		"my_date2" => hashmap! {
			"format" => "basic_date_time", 
			"type" => "date"
		}, 
		"my_bool1" => hashmap! {
			"type" => "boolean"
		}, 
		"my_string2" => hashmap! {
			"type" => "text"
		}, 
		"my_ips" => hashmap! {
			"type" => "ip"
		}, 
		"my_val" => hashmap! {
			"type" => "object"
		}
	};

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_rses_nested() {
	let mut serialiser = rs_es_map::Serializer::default();
	let ser = RsesMapper::to_value(MyStructMapping, &mut serialiser).unwrap();

	let expected = hashmap! {
		"title" => hashmap! {
			"type" => "text"
		}, 
		"geo" => hashmap! {
			"type" => "nested"
		}, 
		"timestamp" => hashmap! {
			"format" => "yyyy-MM-dd\'T\'HH:mm:ssZ", 
			"type" => "date"
		}, 
		"id" => hashmap! {
			"type" => "integer"
		}
	};

	assert_eq!(expected, ser);
}

#[test]
fn get_type_name_default() {
	assert_eq!("myothertype", MyOtherType::name());
}

#[test]
fn get_type_name_custom() {
	assert_eq!("my_type", MyType::name());
}
