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
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::ElasticDate;
//!
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
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
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::ElasticDate;
//!
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! #[elastic(mapping="MyTypeMapping")]
//! pub struct MyType {
//! 	pub my_date: ElasticDate,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
//! #[derive(Default, Clone)]
//! struct MyTypeMapping;
//! impl ElasticObjectMapping for MyTypeMapping {
//! 	fn data_type() -> &'static str {
//! 		"object"
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
//!
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
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::ElasticDate;
//!
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! #[elastic(ty="my_type")]
//! pub struct MyType {
//! 	pub my_date: ElasticDate,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
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
//! The mapping will respect standard `serde` attributes for renaming or ignoring fields:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::ElasticDate;
//!
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	#[serde(rename="my_renamed_date")]
//! 	pub my_date: ElasticDate,
//! 	#[serde(skip_serializing)]
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
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
//! ## Mapping with Macros
//!
//! The `impl_object_mapping` macro can be used to hide a lot of the boilerplate in serialising the mapping:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::ElasticDate;
//!
//! //Define a struct for your type
//! #[derive(Default, Clone, Serialize, Deserialize)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
//! //Define the object mapping for the type
//! #[derive(Default, Clone)]
//! struct MyTypeMapping;
//! impl ElasticObjectMapping for MyTypeMapping {
//! 	fn data_type() -> &'static str {
//! 		"object"
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
//!
//! impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [my_date, my_string]);
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
//! It's also possible to use different names for fields:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::ElasticDate;
//!
//! //Define a struct for your type
//! #[derive(Default, Clone, Serialize, Deserialize)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//!
//! //Define the object mapping for the type
//! #[derive(Default, Clone)]
//! struct MyTypeMapping;
//! impl ElasticObjectMapping for MyTypeMapping {
//! 	fn data_type() -> &'static str {
//! 		"object"
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
//!
//! impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [
//! 	{"my_renamed_date", my_date},
//! 	{"my_renamed_string", my_string}
//! ]);
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
//! The fields passed to `impl_type_mapping` must all implement `ElasticType`.
//! Note that not all fields need to be included in the macro call.
//! Also remember that Elasticsearch will automatically update mappings based on the objects it sees though,
//! so if your 'un-mapped' field is serialised on `index`, then some mapping will be added for it.

use serde;

/// Visitor for serialising a user-defined Rust struct as a field of another type.
pub trait ElasticObjectTypeVisitor<'a, T> where
T: 'a + Clone + Default,
Self: serde::ser::MapVisitor {
	/// Create a new visitor from a borrowed user-defined type.
	fn new(data: &'a T) -> Self;
}

mod object;
mod user_type;

pub use self::object::*;
pub use self::user_type::*;
