//! Requirements for mapping `user-defined` types.
//!
//! # Examples
//!
//! Define your Elasticsearch types using _Plain Old Rust Structures_.
//! Your types should at least derive `Default`, `Clone` and `serde::Serialize`.
//!
//! ## Derive Mapping
//!
//! Mapping can be generated by deriving `ElasticType` on a struct:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::prelude::*;
//!
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! It's also possible to adjust the mapping using the `elastic` attribute.
//!
//! ### Use a Custom Mapping
//!
//! Use `elastic(mapping="{TypeName}")` to associate a given mapping with your type:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! #[elastic(mapping="MyTypeMapping")]
//! pub struct MyType {
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
//! #[derive(Default, Clone)]
//! struct MyTypeMapping;
//! impl ElasticObjectMapping for MyTypeMapping {
//! 	fn data_type() -> &'static str {
//! 		OBJECT_DATATYPE
//! 	}
//!
//! 	fn dynamic() -> Option<Dynamic> {
//! 		Some(Dynamic::True)
//! 	}
//!
//! 	fn enabled() -> Option<bool> {
//! 		Some(false)
//! 	}
//!
//! 	fn include_in_all() -> Option<bool> {
//! 		Some(true)
//! 	}
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! ### Use a Custom Type Name
//!
//! By default, the Elasticsearch type name (the value of `_type`) is the lowercase variant of the Rust struct.
//! So `MyType` would become `mytype`.
//! You can change this by providing `elastic(ty="{TypeName}")`:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! #[elastic(ty="my_type")]
//! pub struct MyType {
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! ### Ignore or Rename Fields
//!
//! If you add the `serde_attrs` feature to `elastic_types_macros`,
//! the mapping will respect standard `serde` attributes for renaming or ignoring fields.
//!
//! Use `elsatic_types_macros` in your `Cargo.toml` as follows:
//!
//! ```ignore
//! elastic_types_macros = { version = "*", features = [ "serde-attrs" ] }
//! ```
//!
//! You can then serialise type mappings with `serde` attributes:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	#[serde(rename="my_renamed_date")]
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	#[serde(skip_serializing)]
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! ## Limitations
//!
//! Automatically deriving mapping has the following limitations:
//!
//! - Generics aren't supported by auto deriving.
//! So you can't `#[derive(ElasticType)]` on `MyType<T>`.
//! - Mapping types can't be shared. This is because they need to map the type fields, so are specific to that type.
//! So you can't share `MyTypeMapping` between `MyType` and `MyOtherType`.
//!
//! All of the above limitations can be worked around by implementing the mapping manually.
//!
//! Remember that Elasticsearch will automatically update mappings based on the objects it sees though,
//! so if your 'un-mapped' field is serialised on `index`, then some mapping will be added for it.
//!
//! ## Derive with Macros
//!
//! You can also build object mappings on `stable` using the `type_mapping!` macro:
//!
//! ```
//! # #![feature(custom_derive, custom_attribute, plugin)]
//! # #![plugin(serde_macros, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::prelude::*;
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
//! //Implement ElasticType for your type. This binds it to the mapping
//! impl ElasticType<MyTypeMapping, ()> for MyType { }
//! 
//! //Define the type mapping for our type
//! #[derive(Default, Clone)]
//! pub struct MyTypeMapping;
//! type_mapping!(my_type MyTypeMapping {
//! 	fn props_len() -> usize { 3 }
//! 		
//! 	fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
//! 	where S: serde::Serializer {
//! 		try!(serializer.serialize_struct_elt(state, "my_date", ElasticDate::<DefaultDateFormat>::mapping()));
//! 		try!(serializer.serialize_struct_elt(state, "my_string", String::mapping()));
//! 		try!(serializer.serialize_struct_elt(state, "my_num", i32::mapping()));
//! 
//! 		Ok(())
//! 	}
//! });
//! # fn main() {
//! # }
//! ```
//! 
//! The first ident passsed to the `type_mapping!` macro is the name of the type to use in Elasticsearch.
//! Property types can be mapped by calling their static `mapping()` method.
//! Any type that implements `ElasticFieldMapping` can be mapped this way.
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/_basic_concepts.html#_type)

/// Elasticsearch datatype name.
pub const OBJECT_DATATYPE: &'static str = "object";
/// Elasticsearch datatype name.
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";
/// Elasticsearch datatype name.
pub const NESTED_DATATYPE: &'static str = "nested";

use serde::{ self, Serialize, Serializer };
use ::mapping::ElasticFieldMapping;

/// The base requirements for mapping an `object` type.
pub trait ElasticObjectMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// Get the type name for this mapping, like `object` or `nested`.
	fn data_type() -> &'static str { NESTED_DATATYPE }

	/// Whether or not new properties should be added dynamically to an existing object.
	/// Accepts `true` (default), `false` and `strict`.
	fn dynamic() -> Option<Dynamic> { None }

	/// Whether the JSON value given for the object field should be parsed and indexed
	/// (`true`, default) or completely ignored (`false`).
	fn enabled() -> Option<bool> { None }

	/// Sets the default `include_in_all` value for all the properties within the object.
	/// The object itself is not added to the `_all` field.
	fn include_in_all() -> Option<bool> { None }

	/// The number of property fields on this mapping.
	fn props_len() -> usize;

	/// Serialisation for the property fields on this mapping.
	fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
	where S: serde::Serializer;
}

/// The base requirements for mapping a user-defined, indexable type.
///
/// User-defined type mappings are tied to `object` mappings.
pub trait ElasticUserTypeMapping where
Self: ElasticObjectMapping {
	/// Serialise this mapping as an indexable type instead of as a field
	/// on another type.
	fn serialize_type<S>(serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer;
}

#[macro_export]
macro_rules! props_ser {
    ($t:ident $p:ident) => (
    	struct $p;
		impl ::serde::Serialize for $p {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("properties", $t::props_len()));
				try!($t::serialize_props(serializer, &mut state));
				serializer.serialize_struct_end(state)
			}
		}
    )
}

#[macro_export]
macro_rules! object_ser {
    ($t:ident) => (
		impl ::serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 5));

				let ty = <$t as ElasticFieldMapping<()>>::data_type();
				try!(serializer.serialize_struct_elt(&mut state, "type", ty));

				ser_field!(serializer, &mut state, $t::dynamic(), "dynamic");
				ser_field!(serializer, &mut state, $t::include_in_all(), "include_in_all");

				if ty == OBJECT_DATATYPE {
					ser_field!(serializer, &mut state, $t::enabled(), "enabled");
				}

				props_ser!($t Properties);
				try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties));

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define an `object` field mapping.
#[macro_export]
macro_rules! object_mapping {
	($t:ident) => (
    	impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { <$t as ElasticObjectMapping>::data_type() }
		}

		object_ser!($t);
    );
    ($n:ident $t:ident) => (
    	impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { <$t as ElasticObjectMapping>::data_type() }

			fn name() -> &'static str { stringify!($n) }
		}

		object_ser!($t);
    );
    ($n:ident $t:ident $b:tt) => (
    	impl $crate::object::ElasticObjectMapping for $t $b

    	object_mapping!($n $t);
    )
}

/// Define an indexable `type` mapping.
#[macro_export]
macro_rules! type_mapping {
    ($n:ident $t:ident $b:tt) => (
		impl $crate::object::ElasticUserTypeMapping for $t {
			fn serialize_type<S>(serializer: &mut S) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 1));

				props_ser!($t Properties);
				try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties));

				serializer.serialize_struct_end(state)
			}
		}

		object_mapping!($n $t $b);
	)
}

/// The dynamic setting may be set at the mapping type level, and on each inner object.
/// Inner objects inherit the setting from their parent object or from the mapping type.
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
	/// Newly detected fields are added to the mapping. (default).
	True,
	/// Newly detected fields are ignored. New fields must be added explicitly.
	False,
	/// If new fields are detected, an exception is thrown and the document is rejected.
	Strict
}

impl serde::Serialize for Dynamic {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		match *self {
			Dynamic::True => serializer.serialize_bool(true),
			Dynamic::False => serializer.serialize_bool(false),
			Dynamic::Strict => serializer.serialize_str("strict")
		}
	}
}