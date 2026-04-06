// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            proc_macro2::TokenStream::from(attr).span(),
            "async_test::test does not accept arguments",
        )
        .to_compile_error()
        .into();
    }

    let input = parse_macro_input!(item as ItemFn);
    match expand(input) {
        Ok(output) => output,
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(input: ItemFn) -> Result<TokenStream, syn::Error> {
    let attrs = input.attrs;
    let vis = input.vis;
    let block = input.block;
    let sig = input.sig;

    if sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            sig.constness,
            "async_test::test only supports non-const async fn",
        ));
    }
    if sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            sig.unsafety,
            "async_test::test only supports safe async fn",
        ));
    }
    if sig.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &sig.abi,
            "async_test::test does not support extern functions",
        ));
    }
    if sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            sig.fn_token,
            "async_test::test requires async fn",
        ));
    }
    if !sig.generics.params.is_empty() || sig.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &sig.generics,
            "async_test::test does not support generics",
        ));
    }
    if !sig.inputs.is_empty() {
        return Err(syn::Error::new_spanned(
            &sig.inputs,
            "async_test::test does not support function arguments",
        ));
    }
    if !matches!(sig.output, ReturnType::Default) {
        return Err(syn::Error::new_spanned(
            &sig.output,
            "async_test::test does not support explicit return types",
        ));
    }

    let ident = sig.ident;

    Ok(quote! {
        #(#attrs)*
        #[test]
        #vis fn #ident() {
            ::util::bb02_async::block_on(async #block)
        }
    }
    .into())
}
