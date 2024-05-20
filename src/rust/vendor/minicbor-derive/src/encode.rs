use crate::Mode;
use crate::{add_bound_to_type_params, collect_type_params, is_option};
use crate::{add_typeparam, gen_ctx_param};
use crate::attrs::{Attributes, CustomCodec, Encoding, Level};
use crate::fields::{Field, Fields};
use crate::variants::Variants;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::spanned::Spanned;

/// Entry point to derive `minicbor::Encode` on structs and enums.
pub fn derive_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = match &input.data {
        syn::Data::Struct(_) => on_struct(&mut input),
        syn::Data::Enum(_)   => on_enum(&mut input),
        syn::Data::Union(u)  => {
            let msg = "deriving `minicbor::Encode` for a `union` is not supported";
            Err(syn::Error::new(u.union_token.span(), msg))
        }
    };
    proc_macro::TokenStream::from(result.unwrap_or_else(|e| e.to_compile_error()))
}

/// Create an `Encode` impl for (tuple) structs.
fn on_struct(inp: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let data =
        if let syn::Data::Struct(data) = &inp.data {
            data
        } else {
            unreachable!("`derive_from` matched against `syn::Data::Struct`")
        };

    let name     = &inp.ident;
    let attrs    = Attributes::try_from_iter(Level::Struct, inp.attrs.iter())?;
    let encoding = attrs.encoding().unwrap_or_default();
    let fields   = Fields::try_from(name.span(), data.fields.iter())?;

    // Collect type parameters which should not have an `Encode` bound added,
    // i.e. from fields which have a custom encode function defined.
    let blacklist = collect_type_params(&inp.generics, fields.fields().filter(|f| {
        f.attrs.codec().map(|c| c.is_encode()).unwrap_or(false)
    }));

    {
        let bound  = gen_encode_bound()?;
        let params = inp.generics.type_params_mut();
        add_bound_to_type_params(bound, params, &blacklist, fields.fields().attributes(), Mode::Encode);
    }

    let gen = add_typeparam(&inp.generics, gen_ctx_param()?, attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;

    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    // If transparent, just forward the encode call to the inner type.
    if attrs.transparent() {
        if fields.fields().len() != 1 {
            let msg = "#[cbor(transparent)] requires a struct with one field";
            return Err(syn::Error::new(inp.ident.span(), msg))
        }
        let f = fields.fields().next().expect("struct has 1 field");
        return make_transparent_impl(&inp.ident, f, impl_generics, typ_generics, where_clause)
    }

    let tag = encode_tag(&attrs);
    let statements = encode_fields(&fields, true, encoding)?;

    Ok(quote! {
        impl #impl_generics minicbor::Encode<Ctx> for #name #typ_generics #where_clause {
            fn encode<__W777>(&self, __e777: &mut minicbor::Encoder<__W777>, __ctx777: &mut Ctx) -> core::result::Result<(), minicbor::encode::Error<__W777::Error>>
            where
                __W777: minicbor::encode::Write
            {
                #tag
                #statements
            }
        }
    })
}

/// Create an `Encode` impl for enums.
fn on_enum(inp: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let data =
        if let syn::Data::Enum(data) = &inp.data {
            data
        } else {
            unreachable!("`derive_from` matched against `syn::Data::Enum`")
        };

    let name          = &inp.ident;
    let enum_attrs    = Attributes::try_from_iter(Level::Enum, inp.attrs.iter())?;
    let enum_encoding = enum_attrs.encoding().unwrap_or_default();
    let index_only    = enum_attrs.index_only();
    let variants      = Variants::try_from(name.span(), data.variants.iter())?;

    let mut blacklist = HashSet::new();
    let mut field_attrs = Vec::new();
    let mut rows = Vec::new();
    for ((var, idx), attrs) in data.variants.iter().zip(variants.indices.iter()).zip(&variants.attrs) {
        let fields = Fields::try_from(var.ident.span(), var.fields.iter())?;
        // Collect type parameters which should not have an `Encode` bound added,
        // i.e. from fields which have a custom encode function defined.
        blacklist.extend(collect_type_params(&inp.generics, fields.fields().filter(|f| {
            f.attrs.codec().map(|c| c.is_encode()).unwrap_or(false)
        })));
        let con = &var.ident;
        let encoding = attrs.encoding().unwrap_or(enum_encoding);
        let tag = encode_tag(attrs);
        let row = match &var.fields {
            syn::Fields::Unit => match encoding {
                Encoding::Array | Encoding::Map if index_only => quote! {
                    #name::#con => {
                        __e777.u32(#idx)?;
                        Ok(())
                    }
                },
                Encoding::Array => quote! {
                    #name::#con => {
                        __e777.array(2)?;
                        __e777.u32(#idx)?;
                        #tag
                        __e777.array(0)?;
                        Ok(())
                    }
                },
                Encoding::Map => quote! {
                    #name::#con => {
                        __e777.array(2)?;
                        __e777.u32(#idx)?;
                        #tag
                        __e777.map(0)?;
                        Ok(())
                    }
                }
            }
            syn::Fields::Named(f) if index_only => {
                return Err(syn::Error::new(f.span(), "index_only enums must not have fields"))
            }
            syn::Fields::Named(_) => {
                let statements = encode_fields(&fields, false, encoding)?;
                let idents = fields.fields().idents();
                quote! {
                    #name::#con{#(#idents,)* ..} => {
                        __e777.array(2)?;
                        __e777.u32(#idx)?;
                        #tag
                        #statements
                    }
                }
            }
            syn::Fields::Unnamed(f) if index_only => {
                return Err(syn::Error::new(f.span(), "index_only enums must not have fields"))
            }
            syn::Fields::Unnamed(_) => {
                let statements = encode_fields(&fields, false, encoding)?;
                let idents = fields.match_idents();
                quote! {
                    #name::#con(#(#idents,)*) => {
                        __e777.array(2)?;
                        __e777.u32(#idx)?;
                        #tag
                        #statements
                    }
                }
            }
        };
        field_attrs.extend(fields.fields().attributes().cloned());
        rows.push(row)
    }

    {
        let bound  = gen_encode_bound()?;
        let params = inp.generics.type_params_mut();
        add_bound_to_type_params(bound, params, &blacklist, &field_attrs, Mode::Encode);
    }

    let gen = add_typeparam(&inp.generics, gen_ctx_param()?, enum_attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;

    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    let body = if rows.is_empty() {
        quote! {
            unreachable!("empty type")
        }
    } else {
        quote! {
            match self {
                #(#rows)*
            }
        }
    };

    let tag = encode_tag(&enum_attrs);

    Ok(quote! {
        impl #impl_generics minicbor::Encode<Ctx> for #name #typ_generics #where_clause {
            fn encode<__W777>(&self, __e777: &mut minicbor::Encoder<__W777>, __ctx777: &mut Ctx) -> core::result::Result<(), minicbor::encode::Error<__W777::Error>>
            where
                __W777: minicbor::encode::Write
            {
                #tag
                #body
            }
        }
    })
}

/// The encoding logic of fields.
///
/// We first generate code to determine at runtime the number of fields to
/// encode so that we can use regular map or array containers instead of
/// indefinite ones. Since this value depends on optional values being present
/// we can not calculate this number statically but have to generate code
/// with runtime tests.
///
/// Then the actual field encoding happens which is slightly different
/// depending on the encoding.
///
/// NB: The `fields` parameter is assumed to be sorted by index.
fn encode_fields(fields: &Fields, has_self: bool, encoding: Encoding) -> syn::Result<proc_macro2::TokenStream> {
    let default_encode_fn: syn::ExprPath = syn::parse_str("minicbor::Encode::encode")?;

    let mut tests = Vec::new();

    match encoding {
        // Under array encoding the number of elements is the highest
        // index + 1. Each value is checked if it is not nil and if so,
        // the highest index is incremented.
        Encoding::Array => {
            for field in fields.fields() {
                if field.attrs.skip() {
                    continue
                }
                let is_nil = is_nil(&field.typ, field.attrs.codec());
                let n = field.index.val();
                let ident = &field.ident;
                let expr =
                    if has_self {
                        if field.is_name {
                            quote! {
                                if !#is_nil(&self.#ident) {
                                    __max_index777 = Some(#n)
                                }
                            }
                        } else {
                            let i = syn::Index::from(field.pos);
                            quote! {
                                if !#is_nil(&self.#i) {
                                    __max_index777 = Some(#n)
                                }
                            }
                        }
                    } else {
                        quote! {
                            if !#is_nil(&#ident) {
                                __max_index777 = Some(#n)
                            }
                        }
                    };
                tests.push(expr)
            }
        }
        // Under map encoding the number of map entries is the number
        // of fields minus those which are nil. Further down we define
        // the total number of fields and here for each nil value we
        // substract 1 from the total.
        Encoding::Map => {
            for field in fields.fields() {
                if field.attrs.skip() {
                    continue
                }
                let is_nil = is_nil(&field.typ, field.attrs.codec());
                let ident = &field.ident;
                let expr =
                    if has_self {
                        if field.is_name {
                            quote! {
                                if #is_nil(&self.#ident) {
                                    __max_fields777 -= 1
                                }
                            }
                        } else {
                            let i = syn::Index::from(field.pos);
                            quote! {
                                if #is_nil(&self.#i) {
                                    __max_fields777 -= 1
                                }
                            }
                        }
                    } else {
                        quote! {
                            if #is_nil(&#ident) {
                                __max_fields777 -= 1
                            }
                        }
                    };
                tests.push(expr);
            }
        }
    }

    let mut statements = Vec::new();

    const IS_NAME: bool = true;
    const NO_NAME: bool = false;
    const HAS_SELF: bool = true;
    const NO_SELF: bool = false;
    const HAS_GAPS: bool = true;
    const NO_GAPS: bool = false;

    match encoding {
        // Under map encoding each field is encoded with its index.
        // Only field values which are not nil are encoded.
        Encoding::Map => for field in fields.fields() {
            if field.attrs.skip() {
                continue
            }
            let is_nil = is_nil(&field.typ, field.attrs.codec());
            let encode_fn = field.attrs.codec().as_ref()
                .and_then(|f| f.to_encode_path())
                .unwrap_or_else(|| default_encode_fn.clone());
            let tag   = encode_tag(&field.attrs);
            let ident = &field.ident;
            let idx   = &field.index;
            let statement =
                match (field.is_name, has_self) {
                    // struct
                    (IS_NAME, HAS_SELF) => quote! {
                        if !#is_nil(&self.#ident) {
                            __e777.u32(#idx)?;
                            #tag
                            #encode_fn(&self.#ident, __e777, __ctx777)?
                        }
                    },
                    // tuple struct
                    (IS_NAME, NO_SELF) => quote! {
                        if !#is_nil(&#ident) {
                            __e777.u32(#idx)?;
                            #tag
                            #encode_fn(#ident, __e777, __ctx777)?
                        }
                    },
                    // enum struct
                    (NO_NAME, HAS_SELF) => {
                        let i = syn::Index::from(field.pos);
                        quote! {
                            if !#is_nil(&self.#i) {
                                __e777.u32(#idx)?;
                                #tag
                                #encode_fn(&self.#i, __e777, __ctx777)?
                            }
                        }
                    }
                    // enum tuple
                    (NO_NAME, NO_SELF) => quote! {
                        if !#is_nil(&#ident) {
                            __e777.u32(#idx)?;
                            #tag
                            #encode_fn(#ident, __e777, __ctx777)?
                        }
                    }
                };
            statements.push(statement)
        }
        // Under array encoding only field values are encoded and their
        // index is represented as the array position. Gaps between indexes
        // need to be filled with null.
        Encoding::Array => {
            let mut first = true;
            let mut k = 0;
            for field in fields.fields() {
                if field.attrs.skip() {
                    continue
                }
                let encode_fn = field.attrs.codec().as_ref()
                    .and_then(|f| f.to_encode_path())
                    .unwrap_or_else(|| default_encode_fn.clone());
                let tag = encode_tag(&field.attrs);
                let idx = &field.index;
                let gaps = if first {
                    first = false;
                    idx.val() - k
                } else {
                    idx.val() - k - 1
                };
                let ident = &field.ident;
                let statement =
                    match (field.is_name, has_self, gaps > 0) {
                        // struct
                        (IS_NAME, HAS_SELF, HAS_GAPS) => quote! {
                            if #idx <= __i777 {
                                for _ in 0 .. #gaps {
                                    __e777.null()?;
                                }
                                #tag
                                #encode_fn(&self.#ident, __e777, __ctx777)?
                            }
                        },
                        (IS_NAME, HAS_SELF, NO_GAPS) => quote! {
                            if #idx <= __i777 {
                                #tag
                                #encode_fn(&self.#ident, __e777, __ctx777)?
                            }
                        },
                        // enum struct
                        (IS_NAME, NO_SELF, HAS_GAPS) => quote! {
                            if #idx <= __i777 {
                                for _ in 0 .. #gaps {
                                    __e777.null()?;
                                }
                                #tag
                                #encode_fn(#ident, __e777, __ctx777)?
                            }
                        },
                        (IS_NAME, NO_SELF, NO_GAPS) => quote! {
                            if #idx <= __i777 {
                                #tag
                                #encode_fn(#ident, __e777, __ctx777)?
                            }
                        },
                        // tuple struct
                        (NO_NAME, HAS_SELF, HAS_GAPS) => {
                            let i = syn::Index::from(field.pos);
                            quote! {
                                if #idx <= __i777 {
                                    for _ in 0 .. #gaps {
                                        __e777.null()?;
                                    }
                                    #tag
                                    #encode_fn(&self.#i, __e777, __ctx777)?
                                }
                            }
                        }
                        (NO_NAME, HAS_SELF, NO_GAPS) => {
                            let i = syn::Index::from(field.pos);
                            quote! {
                                if #idx <= __i777 {
                                    #tag
                                    #encode_fn(&self.#i, __e777, __ctx777)?
                                }
                            }
                         }
                        // enum tuple
                        (NO_NAME, NO_SELF, HAS_GAPS) => quote! {
                            if #idx <= __i777 {
                                for _ in 0 .. #gaps {
                                    __e777.null()?;
                                }
                                #tag
                                #encode_fn(#ident, __e777, __ctx777)?
                            }
                        },
                        (NO_NAME, NO_SELF, NO_GAPS) => quote! {
                            if #idx <= __i777 {
                                #tag
                                #encode_fn(#ident, __e777, __ctx777)?
                            }
                        }
                    };
                statements.push(statement);
                k = idx.val()
            }
        }
    }

    let max_fields: u32 = fields.fields().len().try_into()
        .map_err(|_| {
            let msg = "more than 2^32 fields are not supported";
            syn::Error::new(proc_macro2::Span::call_site(), msg)
        })?;

    match encoding {
        Encoding::Array => Ok(quote! {
            let mut __max_index777: core::option::Option<u32> = None;

            #(#tests)*

            if let Some(__i777) = __max_index777 {
                __e777.array(u64::from(__i777) + 1)?;
                #(#statements)*
            } else {
                __e777.array(0)?;
            }

            Ok(())
        }),
        Encoding::Map => Ok(quote! {
            let mut __max_fields777 = #max_fields;

            #(#tests)*

            __e777.map(u64::from(__max_fields777))?;

            #(#statements)*

            Ok(())
        })
    }
}

/// Forward the encoding because of a `#[cbor(transparent)]` attribute.
fn make_transparent_impl
    ( name: &syn::Ident
    , field: &Field
    , impl_generics: syn::ImplGenerics
    , typ_generics: syn::TypeGenerics
    , where_clause: Option<&syn::WhereClause>
    ) -> syn::Result<proc_macro2::TokenStream>
{
    let default_encode_fn: syn::ExprPath = syn::parse_str("minicbor::Encode::encode")?;

    let encode_fn = field.attrs.codec()
        .filter(|cc| cc.is_encode())
        .and_then(CustomCodec::to_encode_path)
        .unwrap_or_else(|| default_encode_fn.clone());

    let call =
        if field.is_name {
            let id = &field.ident;
            quote!(#encode_fn(&self.#id, __e777, __ctx777))
        } else {
            quote!(#encode_fn(&self.0, __e777, __ctx777))
        };

    Ok(quote! {
        impl #impl_generics minicbor::Encode<Ctx> for #name #typ_generics #where_clause {
            fn encode<__W777>(&self, __e777: &mut minicbor::Encoder<__W777>, __ctx777: &mut Ctx) -> core::result::Result<(), minicbor::encode::Error<__W777::Error>>
            where
                __W777: minicbor::encode::Write
            {
                #call
            }
        }
    })
}

fn gen_encode_bound() -> syn::Result<syn::TypeParamBound> {
    syn::parse_str("minicbor::Encode<Ctx>")
}

pub(crate) fn is_nil(ty: &syn::Type, codec: Option<&CustomCodec>) -> proc_macro2::TokenStream {
    if let Some(ce) = codec {
        if let Some(p) = ce.to_is_nil_path() {
            p.to_token_stream()
        } else if is_option(ty, |_| true) {
            quote!(core::option::Option::is_none)
        } else {
            quote!((|_| false))
        }
    } else {
        quote!(minicbor::Encode::<Ctx>::is_nil)
    }
}

fn encode_tag(a: &Attributes) -> proc_macro2::TokenStream {
    if let Some(t) = a.tag() {
        quote!(__e777.tag(minicbor::data::Tag::new(#t))?;)
    } else {
        quote!()
    }
}

