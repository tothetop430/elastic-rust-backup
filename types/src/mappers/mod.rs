//! Helper mappers for `ElasticType`.
//!
//! Mapping for types is inferred from the generic mapping parameters on `ElasticType`.
//! There are two mappers provided:
//!
//! - `TypeMapper` for mapping user-defined types for the [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html).
//! - `RsesMapper` for mapping with [`rs-es`](http://benashford.github.io/rs-es/rs_es/index.html).

use std::error::Error;
use std::marker::PhantomData;
use std::collections::HashMap;
use serde;
use serde::Serializer;
use serde::ser::Error as SerError;
use serde_json;
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };
use ::object::{ ElasticUserTypeMapping, ElasticUserTypeMappingVisitor };

pub mod rs_es_map;

/// Helper for mapping user-defined types.
///
/// This mapper is designed to take a given user-defined type and pass it around to various visitors to map fields.
pub struct TypeMapper<M> where
M: ElasticUserTypeMapping {
	phantom_m: PhantomData<M>
}
impl <M> TypeMapper<M> where
M: ElasticUserTypeMapping {
	/// Map a user-defined type with a given `Serializer`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = Vec::new();
	/// let mut ser = serde_json::Serializer::new(&mut writer);
	/// let _ = TypeMapper::to_writer(MyTypeMapping, &mut ser).unwrap();
	/// # }
	/// ```
	pub fn to_writer<S>(_: M, serializer: &mut S) -> Result<(), S::Error> where
	S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticFieldMapping<()>>::data_type(),
			<M as ElasticUserTypeMapping>::Visitor::new()
		)
	}

	/// Map a user-defined type to a `String`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let _ = TypeMapper::to_string(MyTypeMapping).unwrap();
	/// # }
	/// ```
	pub fn to_string(t: M) -> Result<String, serde_json::Error> {
		let mut writer = Vec::new();
		{
			let mut ser = serde_json::Serializer::new(&mut writer);
			try!(Self::to_writer(t, &mut ser));
		}

		String::from_utf8(writer).map_err(|e| serde_json::Error::custom(e.description()))
	}

	/// Map a user-defined type to a `serde_json::Value`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let val = TypeMapper::to_value(MyTypeMapping).unwrap();
	///
	/// let ty = val.lookup("properties.my_date.type");
	/// # }
	/// ```
	pub fn to_value(t: M) -> Result<serde_json::Value, serde_json::Error> {
		let mut ser = serde_json::value::Serializer::new();
		try!(Self::to_writer(t, &mut ser));

		Ok(ser.unwrap())
	}
}

/// Helper for mapping `elastic_type`s with `rs-es`.
pub struct RsesMapper<M, V> where
M: ElasticUserTypeMapping<Visitor = ElasticUserTypeMappingVisitor<V>>,
V: ElasticTypeVisitor {
    phantom_m: PhantomData<M>,
    phantom_v: PhantomData<V>
}

impl <M, V> RsesMapper<M, V> where
M: ElasticUserTypeMapping<Visitor = ElasticUserTypeMappingVisitor<V>>,
V: ElasticTypeVisitor {
    /// Map a user-defined type to a `HashMap`.
	///
    /// # Examples
	///
	/// ```no_run
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # use serde::{ Serialize, Deserialize };
	/// use elastic_types::mapping::prelude::*;
    ///
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = rs_es_map::Serializer::default();
	/// let mut ser = RsesMapper::to_value(MyTypeMapping, &mut writer);
    ///
    /// //Map `ser` with `rs_es::Client`
	/// # }
	/// ```
    pub fn to_value<'a>(_: M, serializer: &'a mut rs_es_map::Serializer) -> Result<HashMap<&'a str, HashMap<&'a str, &'a str>>, rs_es_map::Error> {
        try!(serializer.serialize_struct("", V::new()));

        Ok(serializer.value.result())
    }
}