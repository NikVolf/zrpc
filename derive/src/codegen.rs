use proc_macro2::{Span, TokenStream};
use syn::{
	fold::{self, Fold},
	parse_quote,
	punctuated::Punctuated,
	Error, Ident, Result, Token,
};

use crate::options::DeriveOptions;

pub fn rpc_impl(input: syn::Item, options: DeriveOptions) -> Result<proc_macro2::TokenStream> {
    Ok(proc_macro2::TokenStream::new())
}