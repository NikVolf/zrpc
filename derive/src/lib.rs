#![recursion_limit = "256"]
extern crate proc_macro;

mod options;
mod codegen;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use codegen::rpc_impl;

#[proc_macro_attribute]
pub fn rpc(args: TokenStream, input: TokenStream) -> TokenStream {
	let input_toks = parse_macro_input!(input as syn::Item);

	let options = match options::DeriveOptions::try_from(args) {
		Ok(options) => options,
		Err(error) => return error.to_compile_error().into(),
	};

	match rpc_impl(input_toks, options) {
		Ok(output) => output.into(),
		Err(err) => err.to_compile_error().into(),
	}
}

