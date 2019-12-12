use std::convert::TryFrom;
use proc_macro2::TokenStream;
use syn::{Result, ItemTrait, ItemImpl, ImplItem};
use quote::quote;

use crate::options::DeriveOptions;

mod c {
    use syn::{
        Ident, Type, Signature, Error, Result, FnArg, PatType,
        Pat, PatIdent, spanned::Spanned, ReturnType
    };
    use std::convert::TryFrom;
    use proc_macro2::{TokenStream, Literal};
    use quote::quote;

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

        pub fn to_arm(&self, index: u16) -> TokenStream {
            let literal = Literal::u16_suffixed(index);
            let name = &self.name;
            let args = ::std::iter::repeat(quote! { arguments.next().unwrap() }).take(self.args.len());

            return match &self.ret {
                None => quote! {
                    #literal => {
                        self.#name(#(#args),*);
                    }
                },
                Some(Return(_)) => quote! {
                    #literal => {
                        result.push(self.#name(#(#args),*));
                    }
                }
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

            if let ReturnType::Type(_, ty) = &v.output {
                res.ret = Some(Return(*ty.clone()));
            }

            Ok(res)
        }
    }
}

fn generate_trait(_input: ItemTrait, _options: DeriveOptions) -> Result<TokenStream>
{
    Ok(TokenStream::new())
}

fn generate_impl(input: ItemImpl, _options: DeriveOptions) -> Result<TokenStream> {
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

    let arms = methods.iter().enumerate().map(|(index, method)| method.to_arm(index as u16));
    let target = &input.self_ty;

    let r = quote! {
        #input

        impl zrpc::ReqRepService for #target {
            type MethodId = u16;
            type Future = futures::future::Ready<std::io::Result<zrpc::ResultBlob>>;

            fn handle(&mut self, method: Self::MethodId, mut arguments: zrpc::DrainBlob) -> Self::Future {
                let mut result = zrpc::ResultBlob::new();
                match method {
                    #(#arms),*
                    _ => unreachable!(),
                }
                futures::future::ready(Ok(result))
            }
        }
    };

    Ok(r)
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