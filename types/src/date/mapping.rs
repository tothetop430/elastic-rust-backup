//! Mapping for the Elasticsearch `date` type.

use std::marker::PhantomData;
use serde::{ Serialize, Serializer };
use super::{ DateFormat, Date };
use ::mapping::{ ElasticFieldMapping, ElasticFieldMappingWrapper };

/// Elasticsearch datatype name.
pub const DATE_DATATYPE: &'static str = "date";

#[doc(hidden)]
#[derive(Default)]
pub struct DateFormatWrapper<F> where
F: DateFormat {
	_f: PhantomData<F>
}

/// The base requirements for mapping a `date` type.
///
/// # Examples
///
/// Define a custom `DateMapping`:
///
/// ## Derive Mapping
///
/// Currently, deriving mapping only works for structs that take a generic `DateFormat` parameter.
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// date_mapping!(MyDateMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// });
/// # fn main() {}
/// ```
///
/// This will produce the following mapping when mapped with the `EpochMillis` format:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// # date_mapping!(MyDateMapping {
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # });
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyDateMapping::<EpochMillis>::default()).unwrap();
/// # let json = json_str!(
/// {
///     "type": "date",
/// 	"format": "epoch_millis",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Limitations
///
/// Automatically deriving mapping has the following limitations:
///
/// - Non-generic mappings aren't supported by auto deriving.
pub trait DateMapping where
Self: Default {
	/// The date format bound to this mapping.
	/// 
	/// The value of `Format::name()` is what's sent to Elasticsearch as the format to use.
	/// This is also used to serialise and deserialise formatted `Date`s.
	type Format: DateFormat;

	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> { None }

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> { None }

	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> { None }

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<bool> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }

	/// If `true`, malformed numbers are ignored.
	/// If `false` (default), malformed numbers throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> { None }

	/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<Date<Self::Format, Self>> { None }
}

impl <T, F> ElasticFieldMapping<DateFormatWrapper<F>> for T where
T: DateMapping<Format = F>,
F: DateFormat {
	type SerType = ElasticFieldMappingWrapper<T, DateFormatWrapper<F>>;

	fn data_type() -> &'static str { DATE_DATATYPE }
}

impl <T, F> Serialize for ElasticFieldMappingWrapper<T, DateFormatWrapper<F>> where
T: ElasticFieldMapping<DateFormatWrapper<F>> + DateMapping<Format = F>,
F: DateFormat {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 9));

		try!(serializer.serialize_struct_elt(&mut state, "type", T::data_type()));
		try!(serializer.serialize_struct_elt(&mut state, "format", T::Format::name()));

		ser_field!(serializer, &mut state, T::boost(), "boost");
		ser_field!(serializer, &mut state, T::doc_values(), "doc_values");
		ser_field!(serializer, &mut state, T::include_in_all(), "include_in_all");
		ser_field!(serializer, &mut state, T::index(), "index");
		ser_field!(serializer, &mut state, T::store(), "store");
		ser_field!(serializer, &mut state, T::ignore_malformed(), "ignore_malformed");
		ser_field!(serializer, &mut state, T::null_value(), "null_value");

		serializer.serialize_struct_end(state)
	}
}

/// Implement `DateFormat` for the given type.
/// 
/// # Examples
///
/// The macro takes 2 string literals; the format to parse and the name to use in Elasticsearch:
/// 
/// ```
/// # #![feature(plugin)]
/// # #![plugin(elastic_date_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate chrono;
/// # fn main() {}
/// use std::marker::PhantomData;
/// 
/// #[derive(Default, Clone)]
/// struct MyFormat;
/// impl_date_fmt!(MyFormat, "yyyy-MM-ddTHH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ssZ");
/// ```
/// 
/// You can then use `MyFormat` as the generic parameter in `Date`.
#[macro_export]
macro_rules! date_fmt {
	($t:ty, $f:tt, $n:expr) => (
		impl $crate::date::DateFormat for $t {
			fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
				date_fmt_to_tokens!($f)
			}

			fn name() -> &'static str { $n }
		}
	)
}

/// Default mapping for `date`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDateMapping<F> where
F: DateFormat {
	_f: PhantomData<F>
}

impl <F> DateMapping for DefaultDateMapping<F> where
F: DateFormat { 
	type Format = F;
}