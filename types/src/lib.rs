//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//!
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
//!
//! # Usage
//!
//! To get started, add `elastic_types` and `elastic_macros` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_types = "*"
//! elastic_macros = "*"
//! ```
//!
//! And reference them in your crate root:
//!
//! ```ignore
//! #![feature(plugin)]
//! #![plugin(elastic_macros)]
//!
//! extern crate elastic_types;
//! ```
//!
//! ## Map Your Types
//!
//! Derive `ElasticType` on your Elasticsearch-mappable types:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::ElasticDate;
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
//! Of course, structs that derive `ElasticType` can also be used as fields on other Elasticsearch types:
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
//! # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate,
//! # 	pub my_string: String,
//! # 	pub my_num: i32
//! # }
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
//! #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
//! pub struct MyOtherType {
//! 	pub my_type: MyType
//! }
//! # impl serde::Serialize for MyOtherType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyOtherType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! Note there's a lot of traits that are required for Elasticsearch types.
//! This [may change](https://github.com/KodrAus/elasticsearch-rs/issues/75) in the future,
//! but if the bounds are an issue, you may have to just create an Elasticsearch mappable type as a DTO
//! (this is probably a good idea anyway).
//!
//! # Exclude Type Dependencies
//!
//! Each datatype is actually feature-gated, but included by default.
//! This means you can exclude datatypes and their dependant crates if you need to.
//! In your `Cargo.toml`, include `elastic_types` as follows:
//!
//! ```ignore
//! [dependencies.elastic_types]
//! version = "*"
//! default-features = false
//! features = [ "date_ty", "response_ty" ] }
//! ```
//!
//! The full list of features is as follows:
//!
//!  Elasticsearch Type | Feature
//!  ------------------ | ---------------
//!  `integer`          | `number_ty`
//!  `long`             | `number_ty`
//!  `short`            | `number_ty`
//!  `byte`             | `number_ty`
//!  `float`            | `number_ty`
//!  `double`           | `number_ty`
//!  `string`           | `string_ty`
//!  `boolean`          | `boolean_ty`
//!  `date`             | `date_ty`
//!  responses          | `response_ty`
//!
//! To include all types except for responses, you can use the `no_response_ty` feature.
//!
//! # Types
//!
//! Types in Elasticsearch are a combination of _source_ and _mapping_.
//! The source is the data (like `42` or `"my string"`) and the mapping is metadata about how to
//! interpret and use the data (like the format of a date string).
//!
//! The approach `elastic_types` takes to types is to bundle the mapping up as a _Zero Sized Type_.
//! This mapping type is then bound to a field as a generic parameter. For example:
//!
//! ```text
//! ElasticString<DefaultStringMapping>
//! ```
//!
//! The source is a `string` and the mapping is `DefaultStringMapping`.
//!
//! All Elasticsearch types implement the base `ElasticType<M: ElasticTypeMapping<F>, F>` trait
//! where `M` is the mapping and `F` is a type-specific format.
//!
//! The following table illustrates the types provided by `elastic_types`:
//!
//!  Elasticsearch Type | Rust Type (Default Mapping) | Crate     | Rust Type (Custom Mapping)    | Format Type
//!  ------------------ | --------------------------- | --------- | ----------------------------- | -----------
//!  `integer`          | `i32`                       | `std`     | `ElasticInteger<M>`           | `()`
//!  `long`             | `i64`                       | `std`     | `ElasticLong<M>`              | `()`
//!  `short`            | `i16`                       | `std`     | `ElasticShort<M>`             | `()`
//!  `byte`             | `i8`                        | `std`     | `ElasticByte<M>`              | `()`
//!  `float`            | `f32`                       | `std`     | `ElasticFloat<M>`             | `()`
//!  `double`           | `f64`                       | `std`     | `ElasticDouble<M>`            | `()`
//!  `string`           | `String`                    | `std`     | `ElasticString<M>`            | `()`
//!  `boolean`          | `bool`                      | `std`     | `ElasticBoolean<M>`           | `()`
//!  `date`             | `DateTime<UTC>`             | `chrono`  | `ElasticDate<F, M>`           | `DateFormat`
//!  `object`           | -                           | -         | user-defined `struct`         | `()`
//!
//! The following sections explain this table.
//!
//! ## Mapping
//!
//! Having the mapping available at compile-time makes it easy to write efficient generic methods
//! that use type mapping.
//!
//! Where there's a `std` type that's equivalent to an Elasticsearch type (like `i32` for `integer`),
//! a default mapping is implemented for that type.
//! That means you can use primitives in your structs and have them mapped to the correct type in Elasticsearch.
//! If you want to provide your own mapping for a `std` type, there's also a struct provided by `elastic_types`
//! that wraps the `std` type but also takes an explicit mapping (like `ElasticInteger` for `i32`).
//!
//! Where there isn't a `std` type available (like `date`), an external crate is used and an implementation of
//! that type is provided (like `ElasticDate`, which implements `chrono::DateLike + chrono::TimeLike`).
//!
//! ## Formats
//!
//! For some types (like `ElasticDate`), it's helpful to have an extra generic parameter that describes the
//! `format` the data can take. For most types the format is `()`, because there aren't any alternative formats available.
//!
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![feature(custom_derive, custom_attribute, plugin, optin_builtin_traits, associated_type_defaults)]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![plugin(serde_macros, elastic_macros)]

extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
#[doc(hidden)]
pub mod macros;
pub mod mapping;
pub mod mappers;

pub mod object;
#[cfg(feature="date_ty")]
pub mod date;
#[cfg(feature="string_ty")]
pub mod string;
#[cfg(feature="number_ty")]
pub mod number;
#[cfg(feature="boolean_ty")]
pub mod boolean;
#[cfg(feature="response_ty")]
pub mod response;
