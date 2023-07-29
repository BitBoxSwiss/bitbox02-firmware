use crate::Mode;
use crate::{add_bound_to_type_params, collect_type_params, is_cow, is_option, is_str, is_byte_slice};
use crate::{add_typeparam, gen_ctx_param};
use crate::attrs::{Attributes, CustomCodec, Encoding, Idx, Level};
use crate::fields::Fields;
use crate::variants::Variants;
use crate::lifetimes::{gen_lifetime, lifetimes_to_constrain, add_lifetime};
use quote::quote;
use std::collections::HashSet;
use syn::spanned::Spanned;

/// Entry point to derive `minicbor::Decode` on structs and enums.
pub fn derive_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = match &input.data {
        syn::Data::Struct(_) => on_struct(&mut input),
        syn::Data::Enum(_)   => on_enum(&mut input),
        syn::Data::Union(u)  => {
            let msg = "deriving `minicbor::Decode` for a `union` is not supported";
            Err(syn::Error::new(u.union_token.span(), msg))
        }
    };
    proc_macro::TokenStream::from(result.unwrap_or_else(|e| e.to_compile_error()))
}

/// Create a `Decode` impl for (tuple) structs.
fn on_struct(inp: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let data =
        if let syn::Data::Struct(data) = &inp.data {
            data
        } else {
            unreachable!("`derive_from` matched against `syn::Data::Struct`")
        };

    let name   = &inp.ident;
    let attrs  = Attributes::try_from_iter(Level::Struct, inp.attrs.iter())?;
    let fields = Fields::try_from(name.span(), data.fields.iter())?;

    let decode_fns: Vec<Option<CustomCodec>> = fields.attrs.iter()
        .map(|a| a.codec().cloned().filter(CustomCodec::is_decode))
        .collect();

    let mut lifetime = gen_lifetime()?;
    for l in lifetimes_to_constrain(fields.indices.iter().zip(fields.types.iter())) {
        if !lifetime.bounds.iter().any(|b| *b == l) {
            lifetime.bounds.push(l.clone())
        }
    }

    // Collect type parameters which should not have a `Decode` bound added,
    // i.e. from fields which have a custom decode function defined.
    let blacklist = {
        let iter = data.fields.iter()
            .zip(&decode_fns)
            .filter_map(|(f, ff)| ff.is_some().then_some(f));
        collect_type_params(&inp.generics, iter)
    };

    {
        let bound  = gen_decode_bound()?;
        let params = inp.generics.type_params_mut();
        add_bound_to_type_params(bound, params, &blacklist, &fields.attrs, Mode::Decode);
    }

    let gen = add_lifetime(&inp.generics, lifetime);
    let gen = add_typeparam(&gen, gen_ctx_param()?, attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;

    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    // If transparent, just forward the decode call to the inner type.
    if attrs.transparent() {
        if fields.len() != 1 {
            let msg = "#[cbor(transparent)] requires a struct with one field";
            return Err(syn::Error::new(inp.ident.span(), msg))
        }
        let i = fields.indices.first().expect("struct has 1 field");
        let t = fields.types.first().expect("struct has 1 field");
        let f = data.fields.iter().next().expect("struct has 1 field");
        let a = fields.attrs.first().expect("struct has 1 field");
        return make_transparent_impl(&inp.ident, f, *i, t, a, impl_generics, typ_generics, where_clause)
    }

    let field_str  = fields.idents.iter().map(|n| format!("{}::{}", name, n)).collect::<Vec<_>>();
    let statements = gen_statements(&fields, &decode_fns, attrs.encoding().unwrap_or_default())?;
    let nils       = nils(&fields.types, &decode_fns);

    let Fields { indices, idents, .. } = fields;

    let result = if let syn::Fields::Named(_) = data.fields {
        quote! {
            Ok(#name {
                #(#idents : if let Some(x) = #idents {
                    x
                } else if let Some(z) = #nils {
                    z
                } else {
                    return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
                }),*
            })
        }
    } else if let syn::Fields::Unit = &data.fields {
        quote!(Ok(#name))
    } else {
        quote! {
            Ok(#name(#(if let Some(x) = #idents {
                x
            } else if let Some(z) = #nils {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
            }),*))
        }
    };

    Ok(quote! {
        impl #impl_generics minicbor::Decode<'bytes, Ctx> for #name #typ_generics #where_clause {
            fn decode(__d777: &mut minicbor::Decoder<'bytes>, __ctx777: &mut Ctx) -> core::result::Result<#name #typ_generics, minicbor::decode::Error> {
                let __p777 = __d777.position();
                #statements
                #result
            }
        }
    })
}

/// Create a `Decode` impl for enums.
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
    let mut lifetime = gen_lifetime()?;
    let mut rows = Vec::new();
    for ((var, idx), attrs) in data.variants.iter().zip(variants.indices.iter()).zip(&variants.attrs) {
        let fields = Fields::try_from(var.ident.span(), var.fields.iter())?;
        let encoding = attrs.encoding().unwrap_or(enum_encoding);
        let con = &var.ident;
        let row = if let syn::Fields::Unit = &var.fields {
            if index_only {
                quote!(#idx => Ok(#name::#con),)
            } else {
                quote!(#idx => {
                    __d777.skip()?;
                    Ok(#name::#con)
                })
            }
        } else {
            for l in lifetimes_to_constrain(fields.indices.iter().zip(fields.types.iter())) {
                if !lifetime.bounds.iter().any(|b| *b == l) {
                    lifetime.bounds.push(l.clone())
                }
            }
            let decode_fns: Vec<Option<CustomCodec>> = fields.attrs.iter()
                .map(|a| a.codec().cloned().filter(CustomCodec::is_decode))
                .collect();
            let field_str = fields.idents.iter()
                .map(|n| format!("{}::{}::{}", name, con, n))
                .collect::<Vec<_>>();
            // Collect type parameters which should not have an `Decode` bound added,
            // i.e. from fields which have a custom decode function defined.
            blacklist.extend({
                let iter = var.fields.iter()
                    .zip(&decode_fns)
                    .filter_map(|(f, ff)| ff.is_some().then_some(f));
                collect_type_params(&inp.generics, iter)
            });
            let statements = gen_statements(&fields, &decode_fns, encoding)?;
            let nils       = nils(&fields.types, &decode_fns);
            let Fields { indices, idents, .. } = fields;
            if let syn::Fields::Named(_) = var.fields {
                quote! {
                    #idx => {
                        #statements
                        Ok(#name::#con {
                            #(#idents : if let Some(x) = #idents {
                                x
                            } else if let Some(z) = #nils {
                                z
                            } else {
                                return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
                            }),*
                        })
                    }
                }
            } else {
                quote! {
                    #idx => {
                        #statements
                        Ok(#name::#con(#(if let Some(x) = #idents {
                            x
                        } else if let Some(z) = #nils {
                            z
                        } else {
                            return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
                        }),*))
                    }
                }
            }
        };
        field_attrs.extend_from_slice(&fields.attrs);
        rows.push(row)
    }

    {
        let bound  = gen_decode_bound()?;
        let params = inp.generics.type_params_mut();
        add_bound_to_type_params(bound, params, &blacklist, &field_attrs, Mode::Decode);
    }

    let gen = add_lifetime(&inp.generics, lifetime);
    let gen = add_typeparam(&gen, gen_ctx_param()?, enum_attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;

    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    let check = if index_only {
        quote! {
            let __p778 = __d777.position();
        }
    } else {
        quote! {
            let __p777 = __d777.position();
            if Some(2) != __d777.array()? {
                return Err(minicbor::decode::Error::message("expected enum (2-element array)").at(__p777))
            }
            let __p778 = __d777.position();
        }
    };

    Ok(quote! {
        impl #impl_generics minicbor::Decode<'bytes, Ctx> for #name #typ_generics #where_clause {
            fn decode(__d777: &mut minicbor::Decoder<'bytes>, __ctx777: &mut Ctx) -> core::result::Result<#name #typ_generics, minicbor::decode::Error> {
                #check
                match __d777.u32()? {
                    #(#rows)*
                    n => Err(minicbor::decode::Error::unknown_variant(n).at(__p778))
                }
            }
        }
    })
}

/// Generate decoding statements for every item.
//
// For every name `n`, type `t` and index `i` we declare a local mutable
// variable `n` with type `Option<t>` and set it to `None` if `t` is not
// an `Option`, otherwise to `Some(None)`. [1]
//
// Then -- depending on the selected encoding -- we iterate over all CBOR
// map or array elements and if an index `j` equal to `i` is found, we
// attempt to decode the next CBOR item as a value `v` of type `t`. If
// successful, we assign the result to `n` as `Some(v)`, otherwise we
// error, or -- if `t` is an option and the decoding failed because an
// unknown enum variant was decoded -- we skip the variant value and
// continue decoding.
//
// --------------------------------------------------------------------
// [1]: These variables will later be deconstructed in `on_enum` and
// `on_struct` and their inner value will be used to initialise a field.
// If not present, an error will be produced.
fn gen_statements(fields: &Fields, decode_fns: &[Option<CustomCodec>], encoding: Encoding) -> syn::Result<proc_macro2::TokenStream> {
    assert_eq!(fields.len(), decode_fns.len());

    let default_decode_fn: syn::ExprPath = syn::parse_str("minicbor::Decode::decode")?;

    let inits = fields.types.iter().map(|ty| {
        if is_option(ty, |_| true) {
            quote!(Some(None))
        } else {
            quote!(None)
        }
    });

    let actions = fields.indices.iter().zip(fields.idents.iter().zip(fields.types.iter().zip(decode_fns)))
        .map(|(ix, (name, (ty, ff)))| {
            let decode_fn = ff.as_ref()
                .and_then(CustomCodec::to_decode_path)
                .unwrap_or_else(|| default_decode_fn.clone());

            let unknown_var_err =
                if let Some(cd) = ff {
                    if let Some(p) = cd.to_nil_path() {
                        quote! {
                            Err(e) if e.is_unknown_variant() && #p().is_some() => {
                                __d777.skip()?
                            }
                        }
                    } else if is_option(ty, |_| true) {
                        quote! {
                            Err(e) if e.is_unknown_variant() => __d777.skip()?,
                        }
                    } else {
                        quote!()
                    }
                } else if is_option(ty, |_| true) {
                    quote! {
                        Err(e) if e.is_unknown_variant() => __d777.skip()?,
                    }
                } else {
                    quote! {
                        Err(e) if e.is_unknown_variant() && <#ty as minicbor::Decode::<Ctx>>::nil().is_some() => {
                            __d777.skip()?
                        }
                    }
                };

            let value =
                if cfg!(any(feature = "alloc", feature = "std"))
                    && ix.is_b()
                    && is_cow(ty, |t| is_str(t) || is_byte_slice(t))
                {
                    if cfg!(feature = "std") {
                        quote!(Some(std::borrow::Cow::Borrowed(__v777)))
                    } else {
                        quote!(Some(alloc::borrow::Cow::Borrowed(__v777)))
                    }
                } else {
                    quote!(Some(__v777))
                };

            quote! {
                match #decode_fn(__d777, __ctx777) {
                    Ok(__v777) => #name = #value,
                    #unknown_var_err
                    Err(e) => return Err(e)
                }
            }
    })
    .collect::<Vec<_>>();

    let Fields { idents, types, indices, .. } = fields;

    Ok(match encoding {
        Encoding::Array => quote! {
            #(let mut #idents : core::option::Option<#types> = #inits;)*

            if let Some(__len777) = __d777.array()? {
                for __i777 in 0 .. __len777 {
                    match __i777 {
                        #(#indices => #actions)*
                        _          => __d777.skip()?
                    }
                }
            } else {
                let mut __i777 = 0;
                while minicbor::data::Type::Break != __d777.datatype()? {
                    match __i777 {
                        #(#indices => #actions)*
                        _          => __d777.skip()?
                    }
                    __i777 += 1
                }
                __d777.skip()?
            }
        },
        Encoding::Map => quote! {
            #(let mut #idents : core::option::Option<#types> = #inits;)*

            if let Some(__len777) = __d777.map()? {
                for _ in 0 .. __len777 {
                    match __d777.u32()? {
                        #(#indices => #actions)*
                        _          => __d777.skip()?
                    }
                }
            } else {
                while minicbor::data::Type::Break != __d777.datatype()? {
                    match __d777.u32()? {
                        #(#indices => #actions)*
                        _          => __d777.skip()?
                    }
                }
                __d777.skip()?
            }
        }
    })
}

/// Forward the decoding because of a `#[cbor(transparent)]` attribute.
#[allow(clippy::too_many_arguments)]
fn make_transparent_impl
    ( name: &syn::Ident
    , field: &syn::Field
    , index: Idx
    , typ: &syn::Type
    , attrs: &Attributes
    , impl_generics: syn::ImplGenerics
    , typ_generics: syn::TypeGenerics
    , where_clause: Option<&syn::WhereClause>
    ) -> syn::Result<proc_macro2::TokenStream>
{
    let default_decode_fn: syn::ExprPath = syn::parse_str("minicbor::Decode::decode")?;

    let decode_fn = attrs.codec()
        .filter(|cc| cc.is_decode())
        .and_then(CustomCodec::to_decode_path)
        .unwrap_or_else(|| default_decode_fn.clone());

    let call =
        if cfg!(any(feature = "alloc", feature = "std"))
            && index.is_b()
            && is_cow(typ, |t| is_str(t) || is_byte_slice(t))
        {
            let cow =
                if cfg!(feature = "std") {
                    quote!(std::borrow::Cow::Borrowed(v))
                } else {
                    quote!(alloc::borrow::Cow::Borrowed(v))
                };
            if let Some(id) = &field.ident {
                quote! {
                    Ok(#name {
                        #id: match #decode_fn(__d777, __ctx777) {
                            Ok(v)  => #cow,
                            Err(e) => return Err(e)
                        }
                    })
                }
            } else {
                quote! {
                    Ok(#name(match #decode_fn(__d777, __ctx777) {
                        Ok(v)  => #cow,
                        Err(e) => return Err(e)
                    }))
                }
            }
        } else if let Some(id) = &field.ident {
            quote! {
                Ok(#name { #id: #decode_fn(__d777, __ctx777)? })
            }
        } else {
            quote! {
                Ok(#name(#decode_fn(__d777, __ctx777)?))
            }
        };

    Ok(quote! {
        impl #impl_generics minicbor::Decode<'bytes, Ctx> for #name #typ_generics #where_clause {
            fn decode(__d777: &mut minicbor::Decoder<'bytes>, __ctx777: &mut Ctx) -> core::result::Result<#name #typ_generics, minicbor::decode::Error> {
                #call
            }
        }
    })
}

fn gen_decode_bound() -> syn::Result<syn::TypeParamBound> {
    syn::parse_str("minicbor::Decode<'bytes, Ctx>")
}

fn nils(types: &[syn::Type], decode_fns: &[Option<CustomCodec>]) -> Vec<proc_macro2::TokenStream> {
    types.iter().zip(decode_fns)
        .map(|(ty, dec)| {
            if let Some(d) = dec {
                if let Some(p) = d.to_nil_path() {
                    quote!(#p())
                } else if is_option(ty, |_| true) {
                    quote!(Some(None))
                } else {
                    quote!(None)
                }
            } else {
                quote!(<#ty as minicbor::Decode::<Ctx>>::nil())
            }
        })
        .collect()
}
