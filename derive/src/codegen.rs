use std::convert::TryFrom;
use proc_macro2::{Span, TokenStream};
use syn::{fold::{self, Fold}, parse_quote, punctuated::Punctuated, Error, Ident, Result, Token, ItemTrait, ItemImpl, ImplItem, Item,
    FnArg, PatType, Type, ImplItemMethod
};

use crate::options::DeriveOptions;

mod c {
    use syn::{Ident, Type, Signature, Error, Result, FnArg, PatType, Receiver, Pat, PatIdent, spanned::Spanned};
    use std::convert::TryFrom;

    pub struct NamedArg {
        pub name: Ident,
        pub ty: Type,
    }

    pub struct Return(Type);

    pub enum ContextReference { None, SelfRef, SelfMutRef }

    pub struct Method {
        pub context_ref: ContextReference,
        pub name: Ident,
        pub args: Vec<NamedArg>,
        pub ret: Option<Return>,
    }

    impl Method {
        pub fn new(name: Ident) -> Self {
            Method {
                context_ref: ContextReference::None,
                name,
                args: vec![],
                ret: None,
            }
        }
    }

    impl TryFrom<&Signature> for Method {
        type Error = Error;

        fn try_from(v: &Signature) -> Result<Self> {
            let mut res = Method::new(v.ident.clone());

            for arg in v.inputs.iter() {
                match arg {
                    FnArg::Receiver(receiver) => {
                        if receiver.reference.is_none() {
                            return Err(Error::new(receiver.span(), "Move self is not yet supported. Coming soon!"));
                        }

                        if receiver.mutability.is_some() {
                            res.context_ref = ContextReference::SelfMutRef;
                        } else {
                            res.context_ref = ContextReference::SelfRef;
                        }
                    },
                    FnArg::Typed(PatType { pat, ty, .. }) => {
                        if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                            if ident == "self" {
                                // not supporting those custom receivers
                                return Err(Error::new(ident.span(), "Custom receivers are not yet supported. Coming soon!"));
                            } else {
                                res.args.push(NamedArg { name: ident.clone(), ty: *ty.clone() });
                                continue;
                            }
                        }

                        return Err(Error::new(pat.span(), "This type of argument is not supported!!"));
                    }
                }
            }

            Ok(res)
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