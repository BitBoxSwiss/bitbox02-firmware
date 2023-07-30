//! Attribute handling.

pub mod typeparam;
pub mod codec;
pub mod encoding;
pub mod idx;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::iter;
use syn::spanned::Spanned;

pub use typeparam::TypeParams;
pub use codec::CustomCodec;
pub use encoding::Encoding;
pub use idx::Idx;

/// Recognised attributes.
#[derive(Debug, Clone)]
pub struct Attributes(Level, HashMap<Kind, Value>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Kind {
    Codec,
    Encoding,
    Index,
    IndexOnly,
    Transparent,
    TypeParam,
    Nil,
    IsNil,
    HasNil,
    ContextBound,
    CborLen
}

#[derive(Debug, Clone)]
enum Value {
    Codec(CustomCodec, proc_macro2::Span),
    Encoding(Encoding, proc_macro2::Span),
    Index(Idx, proc_macro2::Span),
    Span(proc_macro2::Span),
    TypeParam(TypeParams, proc_macro2::Span),
    Nil(syn::ExprPath, proc_macro2::Span),
    IsNil(syn::ExprPath, proc_macro2::Span),
    HasNil(proc_macro2::Span),
    ContextBound(HashSet<syn::TraitBound>, proc_macro2::Span),
    CborLen(syn::ExprPath, proc_macro2::Span)
}

#[derive(Debug, Copy, Clone)]
pub enum Level {
    Enum,
    Struct,
    Variant,
    Field
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Enum    => f.write_str("enum"),
            Level::Struct  => f.write_str("struct"),
            Level::Variant => f.write_str("variant"),
            Level::Field   => f.write_str("field")
        }
    }
}

impl Attributes {
    pub fn new(l: Level) -> Self {
        Attributes(l, HashMap::new())
    }

    pub fn try_from_iter<'a, I>(l: Level, attrs: I) -> syn::Result<Self>
    where
        I: IntoIterator<Item = &'a syn::Attribute>
    {
        let mut this = Attributes::new(l);
        for m in attrs.into_iter().map(|a| Attributes::try_from(l, a)) {
            let m = m?;
            for (k, v) in m.1.into_iter() {
                this.try_insert(k, v)?;
            }
        }
        if let Some(Value::IsNil(_, s)) = this.get(Kind::IsNil) {
            return Err(syn::Error::new(*s, "`is_nil` requires `encode_with`"))
        }
        if let Some(Value::Nil(_, s)) = this.get(Kind::Nil) {
            return Err(syn::Error::new(*s, "`nil` requires `decode_with`"))
        }
        if let Some(Value::HasNil(s)) = this.get(Kind::HasNil) {
            return Err(syn::Error::new(*s, "`has_nil` requires `with`"))
        }
        Ok(this)
    }

    fn try_from(l: Level, a: &syn::Attribute) -> syn::Result<Self> {
        let mut attrs = Attributes::new(l);

        // #[n(...)]
        if a.path.is_ident("n") {
            let idx = parse_u32_arg(a).map(Idx::N)?;
            attrs.try_insert(Kind::Index, Value::Index(idx, a.tokens.span()))?;
            return Ok(attrs)
        }

        // #[b(...)]
        if a.path.is_ident("b") {
            let idx = parse_u32_arg(a).map(Idx::B)?;
            attrs.try_insert(Kind::Index, Value::Index(idx, a.tokens.span()))?;
            return Ok(attrs)
        }

        // #[cbor(...)]
        let cbor =
            if let syn::Meta::List(ml) = a.parse_meta()? {
                if !ml.path.is_ident("cbor") {
                    return Ok(Attributes::new(l))
                }
                ml
            } else {
                return Ok(Attributes::new(l))
            };

        for nested in &cbor.nested {
            match nested {
                syn::NestedMeta::Meta(syn::Meta::Path(arg)) =>
                    if arg.is_ident("index_only") {
                        attrs.try_insert(Kind::IndexOnly, Value::Span(nested.span()))?
                    } else if arg.is_ident("transparent") {
                        attrs.try_insert(Kind::Transparent, Value::Span(nested.span()))?
                    } else if arg.is_ident("map") {
                        attrs.try_insert(Kind::Encoding, Value::Encoding(Encoding::Map, nested.span()))?
                    } else if arg.is_ident("array") {
                        attrs.try_insert(Kind::Encoding, Value::Encoding(Encoding::Array, nested.span()))?
                    } else if arg.is_ident("has_nil") {
                        attrs.try_insert(Kind::HasNil, Value::HasNil(nested.span()))?
                    } else {
                        return Err(syn::Error::new(nested.span(), "unknown attribute"))
                    }
                syn::NestedMeta::Meta(syn::Meta::NameValue(arg)) =>
                    if arg.path.is_ident("encode_with") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let cc = CustomCodec::Encode(codec::Encode {
                                encode: syn::parse_str(&path.value())?,
                                is_nil: None
                            });
                            attrs.try_insert(Kind::Codec, Value::Codec(cc, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("is_nil") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            attrs.try_insert(Kind::IsNil, Value::IsNil(syn::parse_str(&path.value())?, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("decode_with") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let cc = CustomCodec::Decode(codec::Decode {
                                decode: syn::parse_str(&path.value())?,
                                nil: None
                            });
                            attrs.try_insert(Kind::Codec, Value::Codec(cc, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("nil") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            attrs.try_insert(Kind::Nil, Value::Nil(syn::parse_str(&path.value())?, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("with") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let cc = CustomCodec::Module(syn::parse_str(&path.value())?, false);
                            attrs.try_insert(Kind::Codec, Value::Codec(cc, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("encode_bound") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let t: syn::TypeParam = syn::parse_str(&path.value())?;
                            let b = TypeParams::Encode(iter::once((t.ident.clone(), t)).collect());
                            attrs.try_insert(Kind::TypeParam, Value::TypeParam(b, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("decode_bound") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let t: syn::TypeParam = syn::parse_str(&path.value())?;
                            let b = TypeParams::Decode(iter::once((t.ident.clone(), t)).collect());
                            attrs.try_insert(Kind::TypeParam, Value::TypeParam(b, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("bound") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let t: syn::TypeParam = syn::parse_str(&path.value())?;
                            let m = iter::once((t.ident.clone(), t)).collect::<HashMap<_, _>>();
                            let b = TypeParams::Both { encode: m.clone(), decode: m };
                            attrs.try_insert(Kind::TypeParam, Value::TypeParam(b, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("context_bound") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let mut s = HashSet::new();
                            for b in path.value().split('+') {
                                if b.is_empty() {
                                    continue
                                }
                                s.insert(syn::parse_str::<syn::TraitBound>(b.trim())?);
                            }
                            attrs.try_insert(Kind::ContextBound, Value::ContextBound(s, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else if arg.path.is_ident("cbor_len") {
                        if let syn::Lit::Str(path) = &arg.lit {
                            let cl = syn::parse_str(&path.value())?;
                            attrs.try_insert(Kind::CborLen, Value::CborLen(cl, nested.span()))?
                        } else {
                            return Err(syn::Error::new(arg.span(), "string required"))
                        }
                    } else {
                        return Err(syn::Error::new(nested.span(), "unknown attribute"))
                    }
                syn::NestedMeta::Meta(syn::Meta::List(arg)) =>
                    if arg.path.is_ident("n") {
                        if let Some(syn::NestedMeta::Lit(syn::Lit::Int(n))) = arg.nested.first() {
                            let idx = parse_int(n).map(Idx::N)?;
                            attrs.try_insert(Kind::Index, Value::Index(idx, a.tokens.span()))?;
                        } else {
                            return Err(syn::Error::new(arg.span(), "`n` expects a u32 argument"))
                        }
                    } else if arg.path.is_ident("b") {
                        if let Some(syn::NestedMeta::Lit(syn::Lit::Int(n))) = arg.nested.first() {
                            let idx = parse_int(n).map(Idx::B)?;
                            attrs.try_insert(Kind::Index, Value::Index(idx, a.tokens.span()))?;
                        } else {
                            return Err(syn::Error::new(arg.span(), "`b` expects a u32 argument"))
                        }
                    } else {
                        return Err(syn::Error::new(nested.span(), "unknown attribute"))
                    }
                syn::NestedMeta::Lit(_) => {
                    return Err(syn::Error::new(nested.span(), "unknown attribute"))
                }
            }
        }

        Ok(attrs)
    }

    pub fn encoding(&self) -> Option<Encoding> {
        self.get(Kind::Encoding).and_then(|v| v.encoding())
    }

    pub fn index(&self) -> Option<Idx> {
        self.get(Kind::Index).and_then(|v| v.index())
    }

    pub fn codec(&self) -> Option<&CustomCodec> {
        self.get(Kind::Codec).and_then(|v| v.codec())
    }

    pub fn type_params(&self) -> Option<&TypeParams> {
        self.get(Kind::TypeParam).and_then(|v| v.type_params())
    }

    pub fn context_bound(&self) -> Option<impl Iterator<Item = &syn::TraitBound>> {
        self.get(Kind::ContextBound).and_then(|v| v.context_bound())
    }

    pub fn transparent(&self) -> bool {
        self.contains_key(Kind::Transparent)
    }

    pub fn index_only(&self) -> bool {
        self.contains_key(Kind::IndexOnly)
    }

    pub fn cbor_len(&self) -> Option<&syn::ExprPath> {
        self.get(Kind::CborLen).and_then(|v| v.cbor_len())
    }

    fn contains_key(&self, k: Kind) -> bool {
        self.1.contains_key(&k)
    }

    fn get(&self, k: Kind) -> Option<&Value> {
        self.1.get(&k)
    }

    fn get_mut(&mut self, k: Kind) -> Option<&mut Value> {
        self.1.get_mut(&k)
    }

    fn remove(&mut self, k: Kind) -> Option<Value> {
        self.1.remove(&k)
    }

    fn try_insert(&mut self, key: Kind, mut val: Value) -> syn::Result<()> {
        match self.0 {
            Level::Struct => match key {
                | Kind::Encoding
                | Kind::Transparent
                | Kind::ContextBound
                => {}
                | Kind::TypeParam
                | Kind::Codec
                | Kind::Index
                | Kind::IndexOnly
                | Kind::Nil
                | Kind::IsNil
                | Kind::HasNil
                | Kind::CborLen
                => {
                    let msg = format!("attribute is not supported on {}-level", self.0);
                    return Err(syn::Error::new(val.span(), msg))
                }
            }
            Level::Field => match key {
                | Kind::TypeParam
                | Kind::Codec
                | Kind::Index
                | Kind::Nil
                | Kind::IsNil
                | Kind::HasNil
                | Kind::CborLen
                => {}
                | Kind::Encoding
                | Kind::IndexOnly
                | Kind::Transparent
                | Kind::ContextBound
                => {
                    let msg = format!("attribute is not supported on {}-level", self.0);
                    return Err(syn::Error::new(val.span(), msg))
                }
            }
            Level::Enum => match key {
                | Kind::Encoding
                | Kind::IndexOnly
                | Kind::ContextBound
                => {}
                | Kind::TypeParam
                | Kind::Codec
                | Kind::Index
                | Kind::Transparent
                | Kind::Nil
                | Kind::IsNil
                | Kind::HasNil
                | Kind::CborLen
                => {
                    let msg = format!("attribute is not supported on {}-level", self.0);
                    return Err(syn::Error::new(val.span(), msg))
                }
            }
            Level::Variant => match key {
                | Kind::Encoding
                | Kind::Index
                => {}
                | Kind::TypeParam
                | Kind::Codec
                | Kind::IndexOnly
                | Kind::Transparent
                | Kind::Nil
                | Kind::IsNil
                | Kind::HasNil
                | Kind::ContextBound
                | Kind::CborLen
                => {
                    let msg = format!("attribute is not supported on {}-level", self.0);
                    return Err(syn::Error::new(val.span(), msg))
                }
            }
        }
        if self.contains_key(key) {
            if let Some(Value::Codec(cc, _)) = self.get_mut(key) {
                let s = val.span();
                match (val, &cc) {
                    (Value::Codec(CustomCodec::Encode(e), _), CustomCodec::Decode(d)) => {
                        let d = codec::Decode { decode: d.decode.clone(), nil: d.nil.clone() };
                        *cc = CustomCodec::Both(Box::new(e), Box::new(d));
                        return Ok(())
                    }
                    (Value::Codec(CustomCodec::Decode(d), _), CustomCodec::Encode(e)) => {
                        let e = codec::Encode { encode: e.encode.clone(), is_nil: e.is_nil.clone() };
                        *cc = CustomCodec::Both(Box::new(e), Box::new(d));
                        return Ok(())
                    }
                    _ => return Err(syn::Error::new(s, "duplicate attribute"))
                }
            } else if let Some(Value::TypeParam(cb, _)) = self.get_mut(key) {
                let s = val.span();
                if let Value::TypeParam(p, _) = val {
                    cb.try_merge(s, p)?;
                    return Ok(())
                }
                return Err(syn::Error::new(s, "duplicate attribute"))
            } else if let Some(Value::ContextBound(cb, _)) = self.get_mut(key) {
                let s = val.span();
                if let Value::ContextBound(x, _) = val {
                    cb.extend(x);
                    return Ok(())
                }
                return Err(syn::Error::new(s, "duplicate attribute"))
            } else {
                return Err(syn::Error::new(val.span(), "duplicate attribute"))
            }
        }
        match &mut val {
            Value::IsNil(is_nil, s) => {
                match self.get_mut(Kind::Codec) {
                    Some(Value::Codec(CustomCodec::Encode(e), _)) => {
                        if e.is_nil.is_some() {
                            return Err(syn::Error::new(*s, "duplicate attribute"))
                        }
                        e.is_nil = Some(is_nil.clone());
                        return Ok(())
                    }
                    Some(Value::Codec(CustomCodec::Both(e, _), _)) => {
                        if e.is_nil.is_some() {
                            return Err(syn::Error::new(*s, "duplicate attribute"))
                        }
                        e.is_nil = Some(is_nil.clone());
                        return Ok(())
                    }
                    _ => {}
                }
            }
            Value::Nil(nil, s) => {
                match self.get_mut(Kind::Codec) {
                    Some(Value::Codec(CustomCodec::Decode(d), _)) => {
                        if d.nil.is_some() {
                            return Err(syn::Error::new(*s, "duplicate attribute"))
                        }
                        d.nil = Some(nil.clone());
                        return Ok(())
                    }
                    Some(Value::Codec(CustomCodec::Both(_, d), _)) => {
                        if d.nil.is_some() {
                            return Err(syn::Error::new(*s, "duplicate attribute"))
                        }
                        d.nil = Some(nil.clone());
                        return Ok(())
                    }
                    _ => {}
                }
            }
            Value::HasNil(s) => {
                if let Some(Value::Codec(CustomCodec::Module(_, b), _)) = self.get_mut(Kind::Codec) {
                    if *b {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    *b = true;
                    return Ok(())
                }
            }
            Value::Codec(CustomCodec::Encode(e), s) => {
                if let Some(Value::IsNil(is_nil, _)) = self.remove(Kind::IsNil) {
                    if e.is_nil.is_some() {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    e.is_nil = Some(is_nil)
                }
            }
            Value::Codec(CustomCodec::Decode(d), s) => {
                if let Some(Value::Nil(nil, _)) = self.remove(Kind::Nil) {
                    if d.nil.is_some() {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    d.nil = Some(nil)
                }
            }
            Value::Codec(CustomCodec::Both(e, d), s) => {
                if let Some(Value::IsNil(is_nil, _)) = self.remove(Kind::IsNil) {
                    if e.is_nil.is_some() {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    e.is_nil = Some(is_nil)
                }
                if let Some(Value::Nil(nil, _)) = self.remove(Kind::Nil) {
                    if d.nil.is_some() {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    d.nil = Some(nil)
                }
            }
            Value::Codec(CustomCodec::Module(_, b), s) => {
                if let Some(Value::HasNil(_)) = self.remove(Kind::HasNil) {
                    if *b {
                        return Err(syn::Error::new(*s, "duplicate attribute"))
                    }
                    *b = true
                }
                if self.contains_key(Kind::CborLen) {
                    return Err(syn::Error::new(*s, "`with` and `cbor_len` are mutually exclusive"))
                }
            }
            Value::CborLen(_, s) => {
                if let Some(Value::Codec(c, _)) = self.get(Kind::Codec) {
                    if c.is_module() {
                        return Err(syn::Error::new(*s, "`cbor_len` and `with` are mutually exclusive"))
                    }
                }
            }
            _ => {}
        }
        self.1.insert(key, val);
        Ok(())
    }
}

impl Value {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Value::TypeParam(_, s)    => *s,
            Value::Codec(_, s)        => *s,
            Value::Encoding(_, s)     => *s,
            Value::Index(_, s)        => *s,
            Value::Span(s)            => *s,
            Value::Nil(_, s)          => *s,
            Value::IsNil(_, s)        => *s,
            Value::HasNil(s)          => *s,
            Value::ContextBound(_, s) => *s,
            Value::CborLen(_, s)      => *s
        }
    }

    fn index(&self) -> Option<Idx> {
        if let Value::Index(i, _) = self {
            Some(*i)
        } else {
            None
        }
    }

    fn codec(&self) -> Option<&CustomCodec> {
        if let Value::Codec(c, _) = self {
            Some(c)
        } else {
            None
        }
    }

    fn encoding(&self) -> Option<Encoding> {
        if let Value::Encoding(e, _) = self {
            Some(*e)
        } else {
            None
        }
    }

    fn type_params(&self) -> Option<&TypeParams> {
        if let Value::TypeParam(t, _) = self {
            Some(t)
        } else {
            None
        }
    }

    fn context_bound(&self) -> Option<impl Iterator<Item = &syn::TraitBound>> {
        if let Value::ContextBound(x, _) = self {
            Some(x.iter())
        } else {
            None
        }
    }

    fn cbor_len(&self) -> Option<&syn::ExprPath> {
        if let Value::CborLen(x, _) = self {
            Some(x)
        } else {
            None
        }
    }

}

fn parse_u32_arg(a: &syn::Attribute) -> syn::Result<u32> {
    parse_int(&a.parse_args()?)
}

fn parse_int(n: &syn::LitInt) -> syn::Result<u32> {
    n.base10_digits()
     .parse()
     .map_err(|_| syn::Error::new(n.span(), "expected `u32` value"))
}

