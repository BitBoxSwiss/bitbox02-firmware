/// Custom encode/decode functions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomCodec {
    /// Custom encode function.
    ///
    /// Assumed to be of a type equivalent to:
    ///
    ///   `fn<T, W: Write>(&T, &mut Encoder<W>) -> Result<(), Error<W::Error>>`
    ///
    /// Declared with `#[cbor(encode_with = "...")]`.
    ///
    /// In addition, an optional custom `is_nil` function can be declared which
    /// is assumed to be of a type equivalent to:
    ///
    ///   `fn<T>(&T) -> bool`
    ///
    /// Declared with `#[cbor(is_nil = "...")]`
    Encode(Encode),
    /// Custom decode function.
    ///
    /// Assumed to be of a type equivalent to:
    ///
    ///   `fn<T>(&mut Decoder<'_>) -> Result<T, Error>`
    ///
    /// Declared with `#[cbor(decode_with = "...")]`.
    ///
    /// In addition, an optional custom `nil` function can be declared which
    /// is assumed to be of a type equivalent to:
    ///
    ///   `fn<T>() -> Option<T>`
    ///
    /// Declared with `#[cbor(nil = "...")]`
    Decode(Decode),
    /// The combination of `encode_with` + `is_nil` and `decode_with` + `nil`.
    Both(Box<Encode>, Box<Decode>),
    /// A module which contains custom encode/decode functions.
    ///
    /// The module is assumed to contain two functions named `encode` and
    /// `decode` whose types match those declared with
    /// `#[cbor(encode_with = "...")]` or `#[cbor(decode_with = "...")]`
    /// respectively. Declared with `#[cbor(with = "...")]`.
    ///
    /// Optionally, the attribute `has_nil` can be added which means that
    /// the module contains functions `is_nil` and `nil` matching those
    /// declared with `is_nil` and `nil` when using `encode_with` and
    /// `decode_with`.
    Module(syn::ExprPath, bool)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Encode {
    pub encode: syn::ExprPath,
    pub is_nil: Option<syn::ExprPath>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decode {
    pub decode: syn::ExprPath,
    pub nil: Option<syn::ExprPath>
}

impl CustomCodec {
    /// Is this a custom codec from `encode_with` or `with`?
    pub fn is_encode(&self) -> bool {
        !matches!(self, CustomCodec::Decode(_))
    }

    /// Is this a custom codec from `decode_with` or `with`?
    pub fn is_decode(&self) -> bool {
        !matches!(self, CustomCodec::Encode(_))
    }

    /// Is this a custom codec from `with`?
    pub fn is_module(&self) -> bool {
        matches!(self, CustomCodec::Module(..))
    }

    /// Extract the encode function unless this `CustomCodec` does not declare one.
    pub fn to_encode_path(&self) -> Option<syn::ExprPath> {
        match self {
            CustomCodec::Encode(e)    => Some(e.encode.clone()),
            CustomCodec::Both(e, _)   => Some(e.encode.clone()),
            CustomCodec::Decode(_)    => None,
            CustomCodec::Module(p, _) => {
                let mut p = p.clone();
                let ident = syn::Ident::new("encode", proc_macro2::Span::call_site());
                p.path.segments.push(ident.into());
                Some(p)
            }
        }
    }

    /// Extract the decode function unless this `CustomCodec` does not declare one.
    pub fn to_decode_path(&self) -> Option<syn::ExprPath> {
        match self {
            CustomCodec::Decode(d)    => Some(d.decode.clone()),
            CustomCodec::Both(_, d)   => Some(d.decode.clone()),
            CustomCodec::Encode(_)    => None,
            CustomCodec::Module(p, _) => {
                let mut p = p.clone();
                let ident = syn::Ident::new("decode", proc_macro2::Span::call_site());
                p.path.segments.push(ident.into());
                Some(p)
            }
        }
    }

    /// Extract the `is_nil` function if possible.
    pub fn to_is_nil_path(&self) -> Option<syn::ExprPath> {
        match self {
            CustomCodec::Encode(e)       => e.is_nil.clone(),
            CustomCodec::Both(e, _)      => e.is_nil.clone(),
            CustomCodec::Module(p, true) => {
                let mut p = p.clone();
                let ident = syn::Ident::new("is_nil", proc_macro2::Span::call_site());
                p.path.segments.push(ident.into());
                Some(p)
            }
            CustomCodec::Module(_, false) => None,
            CustomCodec::Decode(_)        => None
        }
    }

    /// Extract the `nil` function if possible.
    pub fn to_nil_path(&self) -> Option<syn::ExprPath> {
        match self {
            CustomCodec::Decode(d)       => d.nil.clone(),
            CustomCodec::Both(_, d)      => d.nil.clone(),
            CustomCodec::Module(p, true) => {
                let mut p = p.clone();
                let ident = syn::Ident::new("nil", proc_macro2::Span::call_site());
                p.path.segments.push(ident.into());
                Some(p)
            }
            CustomCodec::Module(_, false) => None,
            CustomCodec::Encode(_)        => None
        }
    }

    /// Extract the `cbor_len` function if possible.
    pub fn to_cbor_len_path(&self) -> Option<syn::ExprPath> {
        if let CustomCodec::Module(p, _) = self {
            let mut p = p.clone();
            let ident = syn::Ident::new("cbor_len", proc_macro2::Span::call_site());
            p.path.segments.push(ident.into());
            Some(p)
        } else {
            None
        }
    }
}

