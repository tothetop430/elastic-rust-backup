//! Implementation of the Elasticsearch `geo_shape` type.
//!
//! Geo shape is a wrapper for storing [GeoJSON](http://geojson.org/) structures in Elasticsearch.
//!
//! # Examples
//!
//! For defining your own geo shape mapping, see [mapping details](mapping/trait.ElasticGeoShapeMapping.html#derive-mapping).
//!
//! Map with a default `geo_shape`:
//!
//! ```
//! # use elastic_types::geo::shape::prelude::*;
//! struct MyType {
//! 	pub field: ElasticGeoShape
//! }
//! ```
//!
//! Map with a custom `geo_shape`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # use std::marker::PhantomData;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::geo::shape::prelude::*;
//! # #[derive(Default, Clone, ElasticGeoShapeMapping)]
//! # pub struct MyGeoShapeMapping;
//! # impl ElasticGeoShapeMapping for MyGeoShapeMapping { }
//! struct MyType {
//! 	pub field: ElasticGeoShape<MyGeoShapeMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)

mod shape;

pub use self::shape::*;
pub mod mapping;

pub mod prelude {
	//! Includes non-mapping types for the `geo_shape` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::shape::*;
}
