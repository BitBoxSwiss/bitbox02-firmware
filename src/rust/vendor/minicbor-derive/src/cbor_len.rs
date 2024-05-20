use crate::{attrs::{Attributes, Level, Encoding, CustomCodec}, fields::Fields, add_typeparam, gen_ctx_param, variants::Variants, encode::is_nil};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

/// Entry point to derive `minicbor::CborLen` on structs and enums.
pub fn derive_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = match &input.data {
        syn::Data::Struct(_) => on_struct(&mut input),
        syn::Data::Enum(_)   => on_enum(&mut input),
        syn::Data::Union(u)  => {
            let msg = "deriving `minicbor::CborLen` for a `union` is not supported";
            Err(syn::Error::new(u.union_token.span(), msg))
        }
    };
    proc_macro::TokenStream::from(result.unwrap_or_else(|e| e.to_compile_error()))
}

/// Create a `CborLen` impl for (tuple) structs.
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

    let cbor_len_bound = gen_cbor_len_bound()?;
    let encode_bound   = gen_encode_bound()?;
    for p in inp.generics.type_params_mut() {
        p.bounds.push(cbor_len_bound.clone());
        p.bounds.push(encode_bound.clone())
    }

    let gen = add_typeparam(&inp.generics, gen_ctx_param()?, attrs.context_bound());
    let impl_generics = gen.split_for_impl().0;
    let (_, typ_generics, where_clause) = inp.generics.split_for_impl();

    let tag = on_tag(&attrs);
    let steps = on_fields(&fields, true, attrs.encoding().unwrap_or_default())?;

    Ok(quote! {
        impl #impl_generics minicbor::CborLen<Ctx> for #name #typ_generics #where_clause {
            fn cbor_len(&self, __ctx777: &mut Ctx) -> usize {
                #tag +
                {
                    #(#steps)*
                }
            }
        }
    })
}

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

    let mut rows = Vec::new();
    for ((var, idx), attrs) in data.variants.iter().zip(variants.indices.iter()).zip(&variants.attrs) {
        let fields   = Fields::try_from(var.ident.span(), var.fields.iter())?;
        let con      = &var.ident;
        let encoding = attrs.encoding().unwrap_or(enum_encoding);
        let tag      = on_tag(attrs);
        let row = match &var.fields {
            syn::Fields::Unit => if index_only {
                quote! {
                    #name::#con => { #idx.cbor_len(__ctx777) }
                }
            } else {
                quote! {
                    #name::#con => { 1 + #idx.cbor_len(__ctx777) + #tag + 1 }
                }
            }
            syn::Fields::Named(f) if index_only => {
                return Err(syn::Error::new(f.span(), "index_only enums must not have fields"))
            }
            syn::Fields::Named(_) => {
                let steps = on_fields(&fields, false, encoding)?;
                let idents = fields.fields().idents();
                match encoding {
                    Encoding::Map => quote! {
                        #name::#con{#(#idents,)* ..} => { 1 + #idx.cbor_len(__ctx777) + #tag + #(#steps)* }
                    },
                    Encoding::Array => quote! {
                        #name::#con{#(#idents,)* ..} => { #(#steps)* + #tag + 1 + #idx.cbor_len(__ctx777) }
                    }
                }
            }
            syn::Fields::Unnamed(f) if index_only => {
                return Err(syn::Error::new(f.span(), "index_only enums must not have fields"))
            }
            syn::Fields::Unnamed(_) => {
                let steps  = on_fields(&fields, false, encoding)?;
                let idents = fields.match_idents();
                match encoding {
                    Encoding::Map => quote! {
                        #name::#con(#(#idents,)*) => { 1 + #idx.cbor_len(__ctx777) + #tag + #(#steps)* }
                    },
                    Encoding::Array => quote! {
                        #name::#con(#(#idents,)*) => { #(#steps)* + #tag + 1 + #idx.cbor_len(__ctx777) }
                    }
                }
            }
        };
        rows.push(row)
    }

    let cbor_len_bound = gen_cbor_len_bound()?;
    let encode_bound   = gen_encode_bound()?;
    for p in inp.generics.type_params_mut() {
        p.bounds.push(cbor_len_bound.clone());
        p.bounds.push(encode_bound.clone())
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

    let tag = on_tag(&enum_attrs);

    Ok(quote! {
        impl #impl_generics minicbor::CborLen<Ctx> for #name #typ_generics #where_clause {
            fn cbor_len(&self, __ctx777: &mut Ctx) -> usize {
                #tag +
                {
                    #body
                }
            }
        }
    })
}

fn on_fields(fields: &Fields, has_self: bool, encoding: Encoding) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let steps = match encoding {
        Encoding::Map => {
            let mut steps = Vec::new();
            let len = fields.fields().len();
            steps.push(quote!(#len.cbor_len(__ctx777)));
            for field in fields.fields() {
                if field.attrs.skip() {
                    continue
                }
                let cbor_len = cbor_len(field.attrs.cbor_len(), field.attrs.codec());
                let is_nil   = is_nil(&field.typ, field.attrs.codec());
                let ident    = &field.ident;
                let idx      = field.index;
                let tag      = on_tag(&field.attrs);
                if has_self {
                    if field.is_name {
                        steps.push(quote! {
                            + if #is_nil(&self.#ident) {
                                0
                            } else {
                                #idx.cbor_len(__ctx777) + #tag + #cbor_len(&self.#ident, __ctx777)
                            }
                        })
                    } else {
                        let i = syn::Index::from(field.pos);
                        steps.push(quote! {
                            + if #is_nil(&self.#i) {
                                0
                            } else {
                                #idx.cbor_len(__ctx777) + #tag + #cbor_len(&self.#i, __ctx777)
                            }
                        })
                    }
                } else {
                    steps.push(quote! {
                        + if #is_nil(&#ident) {
                            0
                        } else {
                            #idx.cbor_len(__ctx777) + #tag + #cbor_len(&#ident, __ctx777)
                        }
                    })
                }
            }
            steps
        }
        Encoding::Array => {
            let mut steps = Vec::new();
            steps.push(quote! {
                let mut __num777 = 0;
                let mut __len777 = 0;
            });
            for field in fields.fields() {
                if field.attrs.skip() {
                    continue
                }
                let n: usize = field.index.val()
                    .try_into()
                    .map_err(|_| syn::Error::new(field.index.span(), "index does not fit into usize"))?;
                let cbor_len = cbor_len(field.attrs.cbor_len(), field.attrs.codec());
                let is_nil   = is_nil(&field.typ, field.attrs.codec());
                let ident    = &field.ident;
                let tag      = on_tag(&field.attrs);
                if has_self {
                    if field.is_name {
                        steps.push(quote! {
                            if !#is_nil(&self.#ident) {
                                __len777 += (#n - __num777) + #tag + #cbor_len(&self.#ident, __ctx777);
                                __num777 = #n + 1
                            }
                        })
                    } else {
                        let i = syn::Index::from(field.pos);
                        steps.push(quote! {
                            if !#is_nil(&self.#i) {
                                __len777 += (#n - __num777) + #tag + #cbor_len(&self.#i, __ctx777);
                                __num777 = #n + 1
                            }
                        })
                    }
                } else {
                    steps.push(quote! {
                        if !#is_nil(&#ident) {
                            __len777 += (#n - __num777) + #tag + #cbor_len(&#ident, __ctx777);
                            __num777 = #n + 1
                        }
                    })
                }
            }
            steps.push(quote! { __num777.cbor_len(__ctx777) + __len777 });
            steps
        }
    };

    Ok(steps)

}

fn cbor_len(custom: Option<&syn::ExprPath>, codec: Option<&CustomCodec>) -> proc_macro2::TokenStream {
    if let Some(cu) = custom {
        return cu.to_token_stream()
    }
    if let Some(ce) = codec {
        if let Some(p) = ce.to_cbor_len_path() {
            return p.to_token_stream()
        }
    }
    quote!(minicbor::CborLen::<Ctx>::cbor_len)
}

fn gen_cbor_len_bound() -> syn::Result<syn::TypeParamBound> {
    syn::parse_str("minicbor::CborLen<Ctx>")
}

fn gen_encode_bound() -> syn::Result<syn::TypeParamBound> {
    syn::parse_str("minicbor::Encode<Ctx>")
}

fn on_tag(a: &Attributes) -> proc_macro2::TokenStream {
    if let Some(t) = a.tag() {
        quote!(minicbor::data::Tag::new(#t).cbor_len(__ctx777))
    } else {
        quote!(0)
    }
}

