use std::collections::HashMap;
use std::hash::Hash;
use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParams {
    Encode(HashMap<syn::Ident, syn::TypeParam>),
    Decode(HashMap<syn::Ident, syn::TypeParam>),
    Both {
        encode: HashMap<syn::Ident, syn::TypeParam>,
        decode: HashMap<syn::Ident, syn::TypeParam>
    }
}

impl TypeParams {
    pub fn try_merge(&mut self, s: proc_macro2::Span, other: Self) -> syn::Result<()> {
        match (&mut *self, other) {
            (Self::Encode(e1), Self::Encode(e2)) => {
                try_merge(s, e1, e2)?
            }
            (Self::Decode(d1), Self::Decode(d2)) => {
                try_merge(s, d1, d2)?
            }
            (Self::Encode(e), Self::Decode(d)) => {
                *self = Self::Both { encode: mem::take(e), decode: d }
            }
            (Self::Decode(d), Self::Encode(e)) => {
                *self = Self::Both { encode: e, decode: mem::take(d) }
            }
            (Self::Encode(e1), Self::Both { encode: e2, decode }) => {
                try_merge(s, e1, e2)?;
                *self = Self::Both { encode: mem::take(e1), decode }
            }
            (Self::Decode(d1), Self::Both { encode, decode: d2 }) => {
                try_merge(s, d1, d2)?;
                *self = Self::Both { encode, decode: mem::take(d1) }
            }
            (Self::Both { encode: e1, .. }, Self::Encode(e2)) => {
                try_merge(s, e1, e2)?
            }
            (Self::Both { decode: d1, .. }, Self::Decode(d2)) => {
                try_merge(s, d1, d2)?
            }
            (Self::Both { encode: e1, decode: d1 }, Self::Both { encode: e2, decode: d2 }) => {
                try_merge(s, e1, e2)?;
                try_merge(s, d1, d2)?
            }
        }
        Ok(())
    }

    pub fn get_encode(&self, id: &syn::Ident) -> Option<&syn::TypeParam> {
        match self {
            Self::Decode(_) => None,
            Self::Encode(e) => e.get(id),
            Self::Both { encode, .. } => encode.get(id),
        }
    }

    pub fn get_decode(&self, id: &syn::Ident) -> Option<&syn::TypeParam> {
        match self {
            Self::Encode(_) => None,
            Self::Decode(d) => d.get(id),
            Self::Both { decode, .. } => decode.get(id)
        }
    }
}

fn try_merge<K, V>(s: proc_macro2::Span, a: &mut HashMap<K, V>, b: HashMap<K, V>) -> syn::Result<()>
where
    K: Eq + Hash
{
    for (k, v) in b.into_iter() {
        if a.contains_key(&k) {
            return Err(syn::Error::new(s, "duplicate type parameter"))
        }
        a.insert(k, v);
    }
    Ok(())
}
