#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::string::prelude::*;

#[derive(Default)]
struct MyMapping;
impl ElasticStringMapping for MyMapping { 
	fn boost() -> Option<f32> {
		Some(1.01)
	}

	fn index() -> Option<IndexAnalysis> {
		Some(IndexAnalysis::No)
	}

	fn doc_values() -> Option<bool> {
		Some(true)
	}

	fn include_in_all() -> Option<bool> {
		Some(false)
	}

	fn store() -> Option<bool> {
		Some(true)
	}
}

impl ElasticMapping<DefaultStringFormat> for MyMapping {
	type Visitor = ElasticStringMappingVisitor<MyMapping, DefaultStringFormat>;

	fn data_type() -> &'static str {
		"string"
	}
}

impl serde::Serialize for MyMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", Self::get_visitor())
	}
}

//This is a quick mockup struct that accesses the mapping on a struct
use std::marker::PhantomData;
struct MappingDispatch<T: ElasticDataType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T: ElasticDataType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> MappingDispatch<T, M, F> {
	pub fn map(t: &T) -> &'static str {
		//Check out the Visitor associated type on the mapping
		let _ = M::get_visitor();

		//Return the type of this mapping to prove we're looking at something unique
		M::data_type()
	}
}

#[test]
fn can_access_mapping_fns() {
	let ty = ElasticString::<MyMapping>::new("");

	assert_eq!("string", MappingDispatch::map(&ty));
}

#[test]
fn can_access_mapping_for_auto_impls() {
	let ty: i32 = 16;

	//For auto impls, we need to send along at least the type too as a generic param
	assert_eq!("object", MappingDispatch::<i32>::map(&ty));
}

#[test]
fn can_access_type_ellision_for_mappings() {

}

#[test]
fn serialise_mapping_null() {
	let mapping = NullMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"type":"object"}"#, ser);
}

#[test]
fn serialise_mapping_index() {
	let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}