use std::convert::TryFrom;
use proc_macro2::{Span, TokenStream};
use syn::{fold::{self, Fold}, parse_quote, punctuated::Punctuated, Error, Ident, Result, Token, ItemTrait, ItemImpl, ImplItem, Item,
    FnArg, PatType, Type, ImplItemMethod
};

use crate::options::DeriveOptions;

mod c {
    use syn::{Ident, Type, Signature, Error, Result};
    use std::convert::TryFrom;

    pub struct NamedArg {
        pub name: Ident,
        pub ty: Type,
    }

    pub struct Return(Type);

    pub struct Method {
        pub self_mut: bool,
        pub name: Ident,
        pub args: Vec<NamedArg>,
        pub ret: Option<Return>,
    }

    impl TryFrom<&Signature> for Method {
        type Error = Error;

        fn try_from(v: &Signature) -> Result<Self> {
            Ok(
                Method {
                    self_mut: false,
                    name: v.ident.clone(),
                    args: vec![],
                    ret: None,
                }
            )
        }
    }
}

fn generate_trait(input: ItemTrait, options: DeriveOptions) -> Result<TokenStream>
{
    Ok(TokenStream::new())
}

fn generate_impl(input: ItemImpl, options: DeriveOptions) -> Result<TokenStream> {
    let methods = input
		.items
		.iter()
		.filter_map(|impl_item| {
			if let ImplItem::Method(m) = impl_item {
                Some(c::Method::try_from(&m.sig))
			} else {
				None
			}
		})
		.collect::<Result<Vec<c::Method>>>()?;

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