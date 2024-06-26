#![deny(clippy::dbg_macro)]
#![deny(unused_crate_dependencies)]

use proc_macro2::{Span, TokenStream};
use syn::spanned::Spanned;

mod types;

pub(crate) use self::types::*;

fn fatality2_inner(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let bail_if_has_generics = |generics: &syn::Generics, span: Span| -> syn::Result<()> {
        if !generics.params.is_empty() {
            return Err(syn::Error::new(
                span,
                "Generics  `enum`-types are currently supported",
            ));
        }
        Ok(())
    };
    let attr = syn::parse2::<Attr>(attr)?;
    match syn::parse2::<syn::Item>(input.clone())? {
        syn::Item::Enum(item) => {
            bail_if_has_generics(&item.generics, item.span())?;
            fatality_enum_gen(attr, item)
        }
        syn::Item::Struct(item) => {
            bail_if_has_generics(&item.generics, item.span())?;
            fatality_struct_gen(attr, item)
        }
        other => Err(syn::Error::new(
            other.span(),
            "Only `enum` and `struct` types are supported",
        )),
    }
}

fn fatality2(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let orig = input.clone();
    let ts = fatality2_inner(attr, input).unwrap_or_else(|e| -> proc_macro2::TokenStream {
        let mut ts = proc_macro2::TokenStream::new();
        ts.extend(orig);
        ts.extend(e.to_compile_error());
        ts
    });

    expander::Expander::new("fatality")
        .add_comment("Generated by `#[fatality::fatality]`".to_owned())
        // .fmt(expander::Edition::_2021)
        .dry(!cfg!(feature = "expand"))
        .write_to_out_dir(ts)
        .unwrap()
}

#[proc_macro_attribute]
pub fn fatality(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let input = TokenStream::from(input);

    let output: TokenStream = fatality2(attr, input);

    proc_macro::TokenStream::from(output)
}

#[cfg(test)]
mod tests;
