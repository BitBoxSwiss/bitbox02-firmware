use crate::attrs::{Attributes, Idx, Level};
use crate::attrs::idx;
use proc_macro2::Span;

#[derive(Debug, Clone)]
pub struct Variants {
    /// CBOR indices of variants
    pub indices: Vec<Idx>,
    /// variant attributes
    pub attrs: Vec<Attributes>
}

impl Variants {
    pub fn try_from<'a, I>(span: Span, iter: I) -> syn::Result<Self>
    where
        I: IntoIterator<Item = &'a syn::Variant>
    {
        let mut indices = Vec::new();
        let mut attrs   = Vec::new();

        for v in iter.into_iter() {
            let attr = Attributes::try_from_iter(Level::Variant, &v.attrs)?;
            let idex = attr.index().ok_or_else(|| {
                syn::Error::new(v.ident.span(), "missing `#[n(...)]` or `#[b(...)]` attribute")
            })?;
            indices.push(idex);
            attrs.push(attr);
        }

        idx::check_uniq(span, indices.iter().copied())?;

        Ok(Variants { indices, attrs })
    }
}
