use proc_macro2::{Span, TokenStream};
use syn::{fold::{self, Fold}, parse_quote, punctuated::Punctuated, Error, Ident, Result, Token, ItemTrait, ItemImpl, Item};

use crate::options::DeriveOptions;

fn generate_trait(input: ItemTrait, options: DeriveOptions) -> Result<TokenStream>
{
    Ok(TokenStream::new())
}

fn generate_impl(input: ItemImpl, options: DeriveOptions) -> Result<TokenStream> {
    Ok(TokenStream::new())
}

pub fn generate(input: syn::Item, options: DeriveOptions) -> Result<TokenStream> {
    let token_stream = match input {
		syn::Item::Trait(item_trait) => generate_trait(item_trait, options),
        syn::Item::Impl(item_impl) => generate_impl(item_impl, options),
		item => {
			return Err(syn::Error::new_spanned(
				item,
				"The #[rpc] custom attribute only works with trait declarations",
			));
		}
	}?;


    Ok(token_stream)
}