use crate::Mode;
use crate::{add_bound_to_type_params, collect_type_params, is_cow, is_option, is_str, is_byte_slice};
use crate::{add_typeparam, gen_ctx_param};
use crate::attrs::{Attributes, CustomCodec, Encoding, Level};
use crate::fields::{Field, Fields};
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

    let mut lifetime = gen_lifetime()?;
    for l in lifetimes_to_constrain(fields.fields().map(|f| (&f.index, &f.typ))) {
        if !lifetime.bounds.iter().any(|b| *b == l) {
            lifetime.bounds.push(l.clone())
        }
    }

    // Collect type parameters which should not have a `Decode` bound added,
    // i.e. from fields which have a custom decode function defined.
    let blacklist = collect_type_params(&inp.generics, fields.fields().filter(|f| {
        f.attrs.codec().map(|c| c.is_decode()).unwrap_or(false)
    }));

    {
        let bound  = gen_decode_bound()?;
        let params = inp.generics.type_params_mut();
        add_bound_to_type_params(bound, params, &blacklist, fields.fields().attributes(), Mode::Decode);
    }

    let gen = add_lifetime(&inp.generics, lifetime);
    let gen = add_typeparam(&gen, gen_ctx_param()?, attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;

    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    // If transparent, just forward the decode call to the inner type.
    if attrs.transparent() {
        if fields.fields().len() != 1 {
            let msg = "#[cbor(transparent)] requires a struct with one field";
            return Err(syn::Error::new(inp.ident.span(), msg))
        }
        let f = fields.fields().next().expect("struct has 1 field");
        return make_transparent_impl(&inp.ident, f, impl_generics, typ_generics, where_clause)
    }

    let statements = gen_statements(&fields, attrs.encoding().unwrap_or_default())?;

    let result = if let syn::Fields::Named(_) = data.fields {
        let nils      = nils(fields.fields());
        let indices   = fields.fields().indices();
        let idents    = fields.fields().idents();
        let field_str = fields.fields().idents().map(|n| format!("{}::{}", name, n));
        let skipped   = fields.skipped().idents();
        quote! {
            Ok(#name {
                #(#idents : if let Some(x) = #idents {
                    x
                } else if let Some(z) = #nils {
                    z
                } else {
                    return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
                },)*
                #(#skipped : Default::default(),)*
            })
        }
    } else if let syn::Fields::Unit = data.fields {
        quote!(Ok(#name))
    } else {
        let expr = field_inits(&name.to_string(), &fields);
        quote! {
            Ok(#name(#expr))
        }
    };

    let tag = decode_tag(&attrs);

    Ok(quote! {
        impl #impl_generics minicbor::Decode<'bytes, Ctx> for #name #typ_generics #where_clause {
            fn decode(__d777: &mut minicbor::Decoder<'bytes>, __ctx777: &mut Ctx) -> core::result::Result<#name #typ_generics, minicbor::decode::Error> {
                #tag
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
        let tag = decode_tag(attrs);
        let row = if let syn::Fields::Unit = var.fields {
            if index_only {
                quote!(#idx => Ok(#name::#con),)
            } else {
                quote!(#idx => {
                    #tag
                    __d777.skip()?;
                    Ok(#name::#con)
                })
            }
        } else {
            for l in lifetimes_to_constrain(fields.fields().map(|f| (&f.index, &f.typ))) {
                if !lifetime.bounds.iter().any(|b| *b == l) {
                    lifetime.bounds.push(l.clone())
                }
            }
            // Collect type parameters which should not have an `Decode` bound added,
            // i.e. from fields which have a custom decode function defined.
            blacklist.extend(collect_type_params(&inp.generics, fields.fields().filter(|f| {
                f.attrs.codec().map(|c| c.is_decode()).unwrap_or(false)
            })));
            let statements = gen_statements(&fields, encoding)?;
            if let syn::Fields::Named(_) = var.fields {
                let nils      = nils(fields.fields());
                let indices   = fields.fields().indices();
                let idents    = fields.fields().idents();
                let field_str = fields.fields().idents().map(|n| format!("{}::{}::{}", name, con, n));
                let skipped   = fields.skipped().idents();
                quote! {
                    #idx => {
                        #tag
                        #statements
                        Ok(#name::#con {
                            #(#idents : if let Some(x) = #idents {
                                x
                            } else if let Some(z) = #nils {
                                z
                            } else {
                                return Err(minicbor::decode::Error::missing_value(#indices).with_message(#field_str).at(__p777))
                            },)*
                            #(#skipped : Default::default(),)*
                        })
                    }
                }
            } else {
                let pref = format!("{name}::{con}");
                let expr = field_inits(&pref, &fields);
                quote! {
                    #idx => {
                        #tag
                        #statements
                        Ok(#name::#con(#expr))
                    }
                }
            }
        };
        field_attrs.extend(fields.fields().attributes().cloned());
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

    let tag = decode_tag(&enum_attrs);

    Ok(quote! {
        impl #impl_generics minicbor::Decode<'bytes, Ctx> for #name #typ_generics #where_clause {
            fn decode(__d777: &mut minicbor::Decoder<'bytes>, __ctx777: &mut Ctx) -> core::result::Result<#name #typ_generics, minicbor::decode::Error> {
                #tag
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
fn gen_statements(fields: &Fields, encoding: Encoding) -> syn::Result<proc_macro2::TokenStream> {
    let default_decode_fn: syn::ExprPath = syn::parse_str("minicbor::Decode::decode")?;

    let actions = fields.fields().map(|field| {
        let decode_fn = field.attrs.codec()
            .and_then(CustomCodec::to_decode_path)
            .unwrap_or_else(|| default_decode_fn.clone());

        let unknown_var_err =
            if let Some(cd) = field.attrs.codec() {
                if let Some(p) = cd.to_nil_path() {
                    quote! {
                        Err(e) if e.is_unknown_variant() && #p().is_some() => {
                            __d777.skip()?
                        }
                    }
                } else if is_option(&field.typ, |_| true) {
                    quote! {
                        Err(e) if e.is_unknown_variant() => __d777.skip()?,
                    }
                } else {
                    quote!()
                }
            } else if is_option(&field.typ, |_| true) {
                quote! {
                    Err(e) if e.is_unknown_variant() => __d777.skip()?,
                }
            } else {
                let ty = &field.typ;
                quote! {
                    Err(e) if e.is_unknown_variant() && <#ty as minicbor::Decode::<Ctx>>::nil().is_some() => {
                        __d777.skip()?
                    }
                }
            };

            let value =
                if cfg!(any(feature = "alloc", feature = "std"))
                    && field.index.is_b()
                    && is_cow(&field.typ, |t| is_str(t) || is_byte_slice(t))
                {
                    if cfg!(feature = "std") {
                        quote!(Some(std::borrow::Cow::Borrowed(__v777)))
                    } else {
                        quote!(Some(alloc::borrow::Cow::Borrowed(__v777)))
                    }
                } else {
                    quote!(Some(__v777))
                };

            let tag  = decode_tag(&field.attrs);
            let name = &field.ident;

            quote! {{
                #tag
                match #decode_fn(__d777, __ctx777) {
                    Ok(__v777) => #name = #value,
                    #unknown_var_err
                    Err(e) => return Err(e)
                }
            }}
    })
    .collect::<Vec<_>>();

    let inits = fields.fields().types().map(|ty| {
        if is_option(ty, |_| true) {
            quote!(Some(None))
        } else {
            quote!(None)
        }
    });

    let idents  = fields.fields().idents();
    let types   = fields.fields().types();
    let indices = fields.fields().indices().collect::<Vec<_>>();

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
fn make_transparent_impl
    ( name: &syn::Ident
    , field: &Field
    , impl_generics: syn::ImplGenerics
    , typ_generics: syn::TypeGenerics
    , where_clause: Option<&syn::WhereClause>
    ) -> syn::Result<proc_macro2::TokenStream>
{
    let default_decode_fn: syn::ExprPath = syn::parse_str("minicbor::Decode::decode")?;

    let decode_fn = field.attrs.codec()
        .filter(|cc| cc.is_decode())
        .and_then(CustomCodec::to_decode_path)
        .unwrap_or_else(|| default_decode_fn.clone());

    let call =
        if cfg!(any(feature = "alloc", feature = "std"))
            && field.index.is_b()
            && is_cow(&field.typ, |t| is_str(t) || is_byte_slice(t))
        {
            let cow =
                if cfg!(feature = "std") {
                    quote!(std::borrow::Cow::Borrowed(v))
                } else {
                    quote!(alloc::borrow::Cow::Borrowed(v))
                };
            if field.is_name {
                let id = &field.ident;
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
        } else if field.is_name {
            let id = &field.ident;
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

fn nils<'a, T>(fields: T) -> Vec<proc_macro2::TokenStream>
where
    T: IntoIterator<Item = &'a Field>
{
    fields.into_iter().map(nil).collect()
}

fn nil(f: &Field) -> proc_macro2::TokenStream {
    if let Some(d) = f.attrs.codec() {
        if let Some(p) = d.to_nil_path() {
            quote!(#p())
        } else if is_option(&f.typ, |_| true) {
            quote!(Some(None))
        } else {
            quote!(None)
        }
    } else {
        let ty = &f.typ;
        quote!(<#ty as minicbor::Decode::<Ctx>>::nil())
    }
}


fn decode_tag(a: &Attributes) -> proc_macro2::TokenStream {
    if let Some(t) = a.tag() {
        let err =
            if cfg!(feature = "std") {
                quote! {
                    minicbor::decode::Error::tag_mismatch(__t777)
                        .with_message(format!("expected tag {}", #t))
                        .at(__p777)
                }
            } else if cfg!(feature = "alloc") {
                quote! {
                    minicbor::decode::Error::tag_mismatch(__t777)
                        .with_message(alloc::format!("expected tag {}", #t))
                        .at(__p777)
                }
            } else {
                quote!(minicbor::decode::Error::tag_mismatch(__t777).at(__p777))
            };
        quote! {
            let __p777 = __d777.position();
            let __t777 = __d777.tag()?;
            if #t != __t777.as_u64() {
                return Err(#err)
            }
        }
    } else {
        quote!()
    }
}

fn field_inits(name: &str, fields: &Fields) -> proc_macro2::TokenStream {
    let mut fragments = Vec::new();
    for field in fields.fields() {
        let nil = nil(field);
        let idt = &field.ident;
        let idx = field.index;
        let str = format!("{name}::{idt}");
        fragments.push((field.pos, quote! {
            if let Some(x) = #idt {
                x
            } else if let Some(z) = #nil {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(#idx).with_message(#str).at(__p777))
            },
        }))
    }
    for skipped in fields.skipped() {
        fragments.push((skipped.pos, quote!(Default::default(),)))
    }
    fragments.sort_unstable_by_key(|(k, _)| *k);
    let mut expr = quote!();
    expr.extend(fragments.into_iter().map(|(_, f)| f));
    expr
}
