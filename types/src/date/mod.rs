//! Implementation of the Elasticsearch `date` type.
//!
//! Dates in Elasticsearch are exposed as a formatted `string` which can contain a `date` and/or a `time` component.
//!
//! All dates used by `elastic_types` are expected to be given in `UTC`, and if no time is supplied, then 12:00am will be used instead.
//! Where performance is paramount, the `EpochMillis` date format will parse and format dates the fastest.
//! The difference is especially obvious on the `stable` channel, where date formats can't be parsed at compile time.
//!
//! # Examples
//!
//! For defining your own date mapping, see [mapping details](mapping/trait.DateMapping.html#derive-mapping).
//!
//! Map with a default `date`:
//!
//! ```
//! # use elastic_types::prelude::*;
//! struct MyType {
//!     pub field: Date<DefaultDateFormat>
//! }
//! ```
//!
//! Map with a custom `date`:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # use std::marker::PhantomData;
//! # use elastic_types::prelude::*;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # #[derive(Default)]
//! # struct MyDateMapping;
//! # impl DateMapping for MyDateMapping { type Format = EpochMillis; }
//! struct MyType {
//!     pub field: Date<EpochMillis, MyDateMapping>
//! }
//! # }
//! ```
//!
//! ## Creating Formats
//!
//! To make it easier to build your own date formats, derive `ElasticDateFormat` on a unit struct.
//! This will convert an Elasticsearch format string into a `Vec<chrono::format::Item>` for efficient parsing at runtime:
//!
//! ```
//! # #[macro_use]
//! # extern crate elastic_types;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # extern crate chrono;
//! # use elastic_types::prelude::*;
//! # fn main() {
//! #[derive(Default, ElasticDateFormat)]
//! #[elastic(date_format="yyyy-MM-dd'T'HH:mm:ss")]
//! struct MyFormat;
//! # }
//! ```
//!
//! You can also manually implement `DateFormat` and write your own arbitrary format/parse logic:
//!
//! ```
//! # extern crate elastic_types;
//! # extern crate chrono;
//! # use elastic_types::prelude::*;
//! # fn main() {
//! use chrono::{DateTime, UTC};
//! 
//! #[derive(Default, Clone)]
//! struct MyCustomFormat;
//! impl DateFormat for MyCustomFormat {
//!     fn name() -> &'static str { "yyyy-MM-dd'T'HH:mm:ssZ" }
//!
//!     fn format<'a>(date: &DateTime<UTC>) -> FormattedDate<'a> {
//!         date.to_rfc3339().into()
//!     }
//!
//!     fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
//!         let date = try!(DateTime::parse_from_rfc3339(date).map_err(|e| ParseError::from(e)));
//!
//!             Ok(DateTime::from_utc(date.naive_local(), UTC))
//!         }
//!     }
//! # }
//! ```
//!
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)

pub mod mapping;

#[macro_use]
mod format;
mod formats;
mod date;
pub use self::format::*;
pub use self::date::*;
pub use self::formats::*;

use chrono::{DateTime, UTC};

/// A re-export of the `chrono::DateTime` struct with `UTC` timezone.
pub type ChronoDateTime = DateTime<UTC>;

/// The default `date` format (`BasicDateTime`).
pub type DefaultDateFormat = BasicDateTime;

pub mod prelude {
    //! Includes all types for the `date` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::DefaultDateFormat;
    pub use super::format::*;
    pub use super::date::*;
    pub use super::formats::*;
    pub use super::mapping::*;
}
