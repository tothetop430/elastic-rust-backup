#![feature(test)]
extern crate test;
extern crate chrono;
extern crate elastic_types;

use elastic_types::date::DateTime;
use elastic_types::date::format::{ BasicDateTime };
use elastic_types::date::format::parse;

use test::Bencher;

#[bench]
fn parse_date_format(b: &mut Bencher) {
	b.iter(|| {
		let _ = parse::to_chrono_tokens(parse::to_tokens("yyyyMMddTHHmmss.SSSZ"));
	});
}

#[bench]
fn parse_date_from_format(b: &mut Bencher) {
	b.iter(|| {
		let _ = DateTime::<BasicDateTime>::parse("20150620T134501.034Z").unwrap();
	});
}

#[bench]
fn parse_date_from_tokens(b: &mut Bencher) {
	let f = parse::to_chrono_tokens(parse::to_tokens("yyyyMMddTHHmmss.SSSZ"));

	b.iter(|| {
		let mut parsed = chrono::format::Parsed::new();
		let _ = chrono::format::parse(&mut parsed, "20150620T134501.034Z", f.clone().into_iter()).unwrap();

		if parsed.hour_mod_12.is_none() {
			let _ = parsed.set_hour(0);
			let _ = parsed.set_minute(0);
		}

		let _ = DateTime::<BasicDateTime>::new(
			chrono::DateTime::from_utc(
				parsed.to_naive_datetime_with_offset(0).unwrap(), 
				chrono::UTC
			)	
		);
	});
}