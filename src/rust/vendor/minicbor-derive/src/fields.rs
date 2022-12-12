use crate::attrs::{Attributes, Idx, Level};
use crate::attrs::idx;
use proc_macro2::Span;
use syn::{Ident, Type};
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct Fields {
    /// field position
    pub pos: Vec<usize>,
    /// field identifiers
    pub idents: Vec<Ident>,
    /// does the field hava a name or is the identifier generated
    pub is_name: Vec<bool>,
    /// CBOR indices of fields
    pub indices: Vec<Idx>,
    /// field types
    pub types: Vec<Type>,
    /// field attributes
    pub attrs: Vec<Attributes>
}

impl Fields {
    pub fn try_from<'a, I>(span: Span, fields: I) -> syn::Result<Self>
    where
        I: IntoIterator<Item = &'a syn::Field>
    {
        let mut pos     = Vec::new();
        let mut indices = Vec::new();
        let mut idents  = Vec::new();
        let mut is_name = Vec::new();
        let mut types   = Vec::new();
        let mut attrs   = Vec::new();

        let sorted = {
            let mut v = Vec::new();
            for (i, f) in fields.into_iter().enumerate() {
                let attr = Attributes::try_from_iter(Level::Field, &f.attrs)?;
                let idex = attr.index().ok_or_else(|| {
                    let s = f.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| f.ty.span());
                    syn::Error::new(s, "missing `#[n(...)]` or `#[b(...)]` attribute")
                })?;
                let (idnt, is_name) = match &f.ident {
                    Some(n) => (n.clone(), true),
                    None    => (quote::format_ident!("_{}", i), false)
                };
                let typ = f.ty.clone();
                v.push((i, idex, idnt, is_name, typ, attr))
            }
            v.sort_unstable_by_key(|(_, n, ..)| n.val());
            v
        };

        for (i, idx, ident, is, typ, attr) in sorted.into_iter() {
            pos.push(i);
            indices.push(idx);
            idents.push(ident);
            is_name.push(is);
            types.push(typ);
            attrs.push(attr);
        }

        idx::check_uniq(span, &indices)?;

        Ok(Fields { pos, idents, is_name, indices, types, attrs })
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }
}

