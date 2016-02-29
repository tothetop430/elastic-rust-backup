//! Implementation of the Elasticsearch `date` type.
//! 
//! Dates in Elasticsearch are exposed as a formatted `string` which can contain a `date` and/or a `time` component.
//! 
//! All dates used by `elastic_types` are expected to be given in `UTC`, and if no time is supplied, then 12:00am will be used instead.
//! Where performance is paramount, the `EpochMillis` date format will parse and format dates the fastest.
//! 
//! Because date conversion needs to be done by the `caller`, the `Format` is a first-class citizen in the `DateTime` design.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)

mod format;
mod date;
mod formats;

pub use self::format::*;
pub use self::date::*;
pub use self::formats::*;
