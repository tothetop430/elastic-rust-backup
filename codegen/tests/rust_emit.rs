#![feature(quote, rustc_private)]

/*
	These tests are a scratchpad for testing the codegen API.
	There's still heaps of work to do here to make things friendlier.
*/

//TODO: Tidy up all_params. Create fun with base param and append args

extern crate syntax;
extern crate elastic_codegen;

use std::fs;
use std::fs::File;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::emit::*;
use elastic_codegen::emit::default::*;
use elastic_codegen::gen::rust::*;
use elastic_codegen::api::gen::{ parse_path_parts, parse_path_params, parse_fmt };
use elastic_codegen::api::gen::rust::*;

fn get_file(path: &str) -> File {
	let p = format!("tests/emit_results/{}.rs", path);

	let _ = fs::remove_file(&p[..]);
	File::create(&p[..]).unwrap()
}

macro_rules! get_ctxt {
    ($cx:ident, $ps:ident, $fgc:ident) => {
    	
		$cx = syntax::ext::base::ExtCtxt::new(
			&$ps, vec![],
			syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
			&mut $fgc
		);
		$cx.bt_push(syntax::codemap::ExpnInfo {
			call_site: DUMMY_SP,
			callee: syntax::codemap::NameAndSpan {
				format: syntax::codemap::MacroBang(intern("")),
				allow_internal_unstable: false,
				span: None,
			}
		});
    };
}

#[test]
fn can_include_default_emit_ops_per_type() {
	use elastic_codegen::emit::default::num::i::*;

	let num = 13;

	let mut buf: Vec<u8> = Vec::new();
	let emitter: CtxtFreeEmitter = CtxtFreeEmitter::new();
	let result = emitter.emit(&num, &mut buf);

	assert!(result.is_ok());
}

#[test]
fn can_emit_rs_fn_to_file() {
	use elastic_codegen::emit::rust::*;

	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut cx;
	get_ctxt!(cx, ps, fgc);
	
	//Function lifetime
	let lifetime = lifetime("'a");

	//Function signature
	let mut fun = build_fn("my_fun", vec![
		arg_ptr::<i32>("arg1", Mutability::Mutable, Some(lifetime)),
		build_arg("arg2", build_ty_ptr("str", Mutability::Immutable, Some(lifetime)))
	]);
	fun.set_return::<i32>();

	//Function body
	{
		let cx = &mut cx;
		fun.set_body(quote_block!(cx, {
			let x = 1;
			x
		}));
	}

	//Create an emitter
	let emitter = RustEmitter::new(cx);

	//Get a file ref
	let mut f = get_file("can_emit_rs_fn_to_file");

	let mut result = emitter.emit_str(&"//Autogenerated\n", &mut f);
	result = emitter.emit(&fun, &mut f);

	assert!(result.is_ok());
}

#[test]
fn can_emit_rs_fn_with_fmt_body_to_file() {
	use elastic_codegen::emit::rust::*;

	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut cx;
	get_ctxt!(cx, ps, fgc);
	
	//Get the params of a url as Idents
	let base = token::str_to_ident("base");
	let url = "/{index}/_alias/{name}";

	let params: Vec<Ident> = parse_path_params(url)
		.unwrap().iter()
		.map(|p| token::str_to_ident(&p))
		.collect();

	let fmt = parse_fmt(url).unwrap();

	//Get the function params
	let mut all_params = Vec::new();
	all_params.push(base);
	for param in params.iter() {
		all_params.push(param.clone());
	}

	//Function signature from params
	let mut fun = build_fn("my_fun", all_params
		.iter()
		.map(|p: &Ident| arg_ident::<String>(p.clone()))
		.collect()
	);
	fun.set_return::<String>();

	//Function body
	{
		let cx = &mut cx;

		//Get the url replacement statement and resulting ident
		let (url_ident, url_stmt) = url_fmt_decl(&fmt, base, params);
		fun.add_body_stmt(url_stmt);

		//Add the rest of the function body. This just returns the formatted url
		fun.add_body_block(quote_block!(cx, {
			$url_ident
		}));
	}

	//Create an emitter
	let emitter = RustEmitter::new(cx);

	//Get a file ref
	let mut f = get_file("can_emit_rs_fn_with_fmt_body_to_file");

	let mut result = emitter.emit_str(&"//Autogenerated\n", &mut f);
	result = emitter.emit(&fun, &mut f);

	assert!(result.is_ok());
}

#[test]
fn can_emit_rs_fn_with_push_body_to_file() {
	use elastic_codegen::emit::rust::*;

	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut cx;
	get_ctxt!(cx, ps, fgc);
	
	//Get the params of the url format as Idents
	let base = token::str_to_ident("base");
	let url = "/{index}/_alias/{name}";

	let params: Vec<Ident> = parse_path_params(url)
		.unwrap().iter()
		.map(|p| token::str_to_ident(&p))
		.collect();
		
	let parts = parse_path_parts(url).unwrap();

	//Function signature from params
	let mut all_params = Vec::new();
	all_params.push(base);
	for param in params.iter() {
		all_params.push(param.clone());
	}

	let mut fun = build_fn("my_fun", all_params
		.iter()
		.map(|p: &Ident| arg_ident::<String>(p.clone()))
		.collect()
	);
	fun.set_return::<String>();

	//Function body
	{
		let cx = &mut cx;

		//Get the url replacement statement and resulting ident
		let (url_ident, url_stmts) = url_push_decl(base, parts, params);
		fun.add_body_stmts(url_stmts);

		//Add the rest of the function body. This just returns the formatted url
		fun.add_body_block(quote_block!(cx, {
			$url_ident
		}));
	}

	//Create an emitter
	let emitter = RustEmitter::new(cx);

	//Get a file ref
	let mut f = get_file("can_emit_rs_fn_with_push_body_to_file");

	let mut result = emitter.emit_str(&"//Autogenerated\n", &mut f);
	result = emitter.emit(&fun, &mut f);

	assert!(result.is_ok());
}
