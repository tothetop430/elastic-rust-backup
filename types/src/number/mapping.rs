//! Mapping for the Elasticsearch `number` types.
//!
//! Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
//! The implementation is the same for all number types, the only difference is the return type of `null_value`.
//!
//! # Examples
//!
//! Define a custom `ElasticIntegerMapping`:
//!
//! ## Derive Mapping
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! integer_mapping!(MyIntegerMapping {
//! 	//Overload the mapping functions here
//! 	fn null_value() -> Option<i32> {
//! 		Some(42)
//! 	}
//! });
//! # fn main() {}
//! ```
//!
//! This will produce the following mapping:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_types_macros)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # integer_mapping!(MyIntegerMapping {
//! # 	//Overload the mapping functions here
//! # 	fn null_value() -> Option<i32> {
//! # 		Some(42)
//! # 	}
//! # });
//! # fn main() {
//! # let mapping = serde_json::to_string(&MyIntegerMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "type": "integer",
//! 	"null_value": 42
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```

use serde::Serialize;
use ::mapping::{ ElasticType, ElasticFieldMapping };

/// Elasticsearch datatype name.
pub const INTEGER_DATATYPE: &'static str = "integer";
/// Elasticsearch datatype name.
pub const LONG_DATATYPE: &'static str = "long";
/// Elasticsearch datatype name.
pub const SHORT_DATATYPE: &'static str = "short";
/// Elasticsearch datatype name.
pub const BYTE_DATATYPE: &'static str = "byte";
/// Elasticsearch datatype name.
pub const DOUBLE_DATATYPE: &'static str = "double";
/// Elasticsearch datatype name.
pub const FLOAT_DATATYPE: &'static str = "float";

macro_rules! number_mapping {
	($m:ident, $v:ident, $n:ty) => (
		/// Base `number` mapping.
		pub trait $m where
		Self: ElasticFieldMapping<()> + Sized + Serialize {
			/// Try to convert strings to numbers and truncate fractions for integers. Accepts `true` (default) and `false`.
			fn coerce() -> Option<bool> { None }

			/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
			fn boost() -> Option<f32> { None }

			/// Should the field be stored on disk in a column-stride fashion,
			/// so that it can later be used for sorting, aggregations, or scripting?
			/// Accepts `true` (default) or `false`.
			fn doc_values() -> Option<bool> { None }

			/// If `true`, malformed numbers are ignored. If `false` (default),
			/// malformed numbers throw an exception and reject the whole document.
			fn ignore_malformed() -> Option<bool> { None }

			/// Whether or not the field value should be included in the `_all` field?
			/// Accepts `true` or `false`. Defaults to false if index is set to no,
			/// or if a parent object field sets `include_in_all` to false.
			/// Otherwise defaults to `true`.
			fn include_in_all() -> Option<bool> { None }

			/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
			fn index() -> Option<bool> { None }

			/// Accepts a numeric value of the same type as the field which is substituted for any explicit null values.
			/// Defaults to `null`, which means the field is treated as missing.
			fn null_value() -> Option<$n> { None }

			/// Whether the field value should be stored and retrievable separately from the `_source` field.
			/// Accepts true or false (default).
			fn store() -> Option<bool> { None }
		}
	)
}

/// Implement `serde` serialisation for a `geo_shape` mapping type.
#[macro_export]
macro_rules! number_ser {
	($t:ident) => (
		impl ::serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 8));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::data_type()));

				ser_field!(serializer, &mut state, $t::coerce(), "coerce");
				ser_field!(serializer, &mut state, $t::boost(), "boost");
				ser_field!(serializer, &mut state, $t::doc_values(), "doc_values");
				ser_field!(serializer, &mut state, $t::ignore_malformed(), "ignore_malformed");
				ser_field!(serializer, &mut state, $t::include_in_all(), "include_in_all");
				ser_field!(serializer, &mut state, $t::null_value(), "null_value");
				ser_field!(serializer, &mut state, $t::store(), "store");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Base mapping requirements for an `integer`.
number_mapping!(ElasticIntegerMapping, ElasticIntegerMappingVisitor, i32);

/// Base mapping requirements for an `long`.
number_mapping!(ElasticLongMapping, ElasticLongMappingVisitor, i64);

/// Base mapping requirements for an `short`.
number_mapping!(ElasticShortMapping, ElasticShortMappingVisitor, i16);

/// Base mapping requirements for an `byte`.
number_mapping!(ElasticByteMapping, ElasticByteMappingVisitor, i8);

/// Base mapping requirements for an `float`.
number_mapping!(ElasticFloatMapping, ElasticFloatMappingVisitor, f32);

/// Base mapping requirements for an `double`.
number_mapping!(ElasticDoubleMapping, ElasticDoubleMappingVisitor, f64);

/// Default mapping for an `integer` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl ElasticIntegerMapping for DefaultIntegerMapping { }
integer_mapping!(DefaultIntegerMapping);
impl ElasticType<DefaultIntegerMapping, ()> for i32 { }

/// Default mapping for a `long` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl ElasticLongMapping for DefaultLongMapping { }
long_mapping!(DefaultLongMapping);
impl ElasticType<DefaultLongMapping, ()> for i64 { }
impl ElasticType<DefaultLongMapping, ()> for isize { }

/// Default mapping for a `short` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl ElasticShortMapping for DefaultShortMapping { }
short_mapping!(DefaultShortMapping);
impl ElasticType<DefaultShortMapping, ()> for i16 { }

/// Default mapping for a `byte` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl ElasticByteMapping for DefaultByteMapping { }
byte_mapping!(DefaultByteMapping);
impl ElasticType<DefaultByteMapping, ()> for i8 { }

/// Default mapping for a `float` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl ElasticFloatMapping for DefaultFloatMapping { }
float_mapping!(DefaultFloatMapping);
impl ElasticType<DefaultFloatMapping, ()> for f32 { }

/// Default mapping for a `double` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl ElasticDoubleMapping for DefaultDoubleMapping { }
double_mapping!(DefaultDoubleMapping);
impl ElasticType<DefaultDoubleMapping, ()> for f64 { }
