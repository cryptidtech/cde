extern crate proc_macro;
use cde::Tag;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::str::FromStr;
use syn::Error as SynError;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, NestedMeta, Lit/*, Meta*/};

#[proc_macro_attribute]
pub fn cde(args: TokenStream, item: TokenStream) -> TokenStream {
    let args2 = Ts2::from(args.clone());
    let mut ts = TokenStream::new();
    ts.extend(item.clone());
    let pargs = parse_macro_input!(args as AttributeArgs);
    let pitem = parse_macro_input!(item as ItemStruct);

    // get the cde type tag string
    let a = match pargs.iter().nth(0) {
        Some(a) => a,
        None => {
            return SynError::new_spanned(args2, "no cde type string supplied").into_compile_error().into();
        }
    };

    // grab the literal string argument
    let s = match a {
        NestedMeta::Lit(Lit::Str(s)) => s,
        _ => {
            return syn::Error::new_spanned(a, "invalid cde type tag definition").into_compile_error().into();
        }
    };

    // try to parse the cde type tag string into a Tag
    match Tag::from_str(&s.value()) {
        Ok(_) => {},
        Err(e) => {
            return syn::Error::new_spanned(a, e).into_compile_error().into();
        }
    };

    // generate the impl of TypedObject for the struct
    let ident = pitem.ident;
    let tt = s.value();
    let gen = quote! {

        impl ::cde::CryptoData for #ident {
            fn tag(&self) -> ::std::string::String {
                String::from(#tt)
            }
        }

    };
    let qs: TokenStream = gen.into();
    ts.extend(qs.into_iter());
    ts
}
