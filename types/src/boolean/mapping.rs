//! Mapping for the Elasticsearch `boolean` type.

use serde;
use serde::Serialize;
use ::mapping::ElasticFieldMapping;

/// Elasticsearch datatype name.
pub const BOOLEAN_DATATYPE: &'static str = "boolean";

/// The base requirements for mapping a `boolean` type.
///
/// Custom mappings can be defined by implementing `ElasticBooleanMapping`.
///
/// # Examples
///
/// Define a custom `ElasticBooleanMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::boolean::prelude::*;
///
/// #[derive(Debug, Clone, Default, ElasticBooleanMapping)]
/// pub struct MyBooleanMapping;
/// impl ElasticBooleanMapping for MyBooleanMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::mapping::prelude::*;
/// # use elastic_types::boolean::prelude::*;
/// # #[derive(Debug, Clone, Default, ElasticBooleanMapping)]
/// # pub struct MyBooleanMapping;
/// # impl ElasticBooleanMapping for MyBooleanMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyBooleanMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "boolean",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Manually
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::boolean::prelude::*;
///
/// #[derive(Debug, Clone, Default)]
/// pub struct MyBooleanMapping;
/// impl ElasticBooleanMapping for MyBooleanMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticFieldMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticFieldMapping<()> for MyBooleanMapping {
/// 	type Visitor = ElasticBooleanMappingVisitor<MyBooleanMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		BOOLEAN_DATATYPE
/// 	}
/// }
///
/// impl serde::Serialize for MyBooleanMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticBooleanMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> { None }

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> { None }

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<bool> { None }

	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<bool> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }
}

/// Implement `serde` serialisation for a `boolean` mapping type.
macro_rules! boolean_ser {
    ($t:ident) => (
		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 6));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::data_type()));

				ser_field!(serializer, &mut state, $t::boost(), "boost");
				ser_field!(serializer, &mut state, $t::doc_values(), "doc_values");
				ser_field!(serializer, &mut state, $t::index(), "index");
				ser_field!(serializer, &mut state, $t::store(), "store");
				ser_field!(serializer, &mut state, $t::null_value(), "null_value");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define a `boolean` mapping.
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// boolean_mapping!(MyMapping {
/// 	fn null_value() -> Option<bool> { Some(true) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping` and `ElasticBooleanMapping`, along with a few default traits:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// ```
/// 
/// If you want to control the default implementations yourself, you can define your
/// mapping type and just pass it the macro to implement `ElasticFieldMapping`:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// impl ElasticBooleanMapping for MyMapping { }
/// 
/// boolean_mapping!(MyMapping);
/// ```
macro_rules! boolean_mapping {
	($t:ident) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { $crate::boolean::mapping::BOOLEAN_DATATYPE }
		}

		boolean_ser!($t);
	);
	($t:ident $b:tt) => (
		#[derive(Debug, Default, Clone, Copy)]
		pub struct $t;

		impl $crate::boolean::mapping::ElasticBooleanMapping for $t $b

		boolean_mapping!($t);
	)
}

/// Default mapping for `bool`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl ElasticBooleanMapping for DefaultBooleanMapping { }

boolean_mapping!(DefaultBooleanMapping);