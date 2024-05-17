//! Procedural macros to derive minicbor's `Encode`, `Decode`, and `CborLen`
//! traits.
//!
//! Deriving is supported for `struct`s and `enum`s. The encoding is optimised
//! for forward and backward compatibility and the overall approach is
//! influenced by [Google's Protocol Buffers][1].
//!
//! The goal is that ideally a change to a type still allows older software,
//! which is unaware of the changes, to decode values of the changed type
//! (forward compatibility) and newer software, to decode values of types
//! encoded by older software, which do not include the changes made to the
//! type (backward compatibility).
//!
//! In order to reach this goal, the encoding has the following characteristics:
//!
//! 1. The encoding does not contain any names, i.e. no field names, type names
//! or variant names. Instead, every field and every constructor needs to be
//! annotated with an (unsigned) index number, e.g. `#[n(1)]`.
//!
//! 2. Unknown fields are ignored during decoding.[^1]
//!
//! 3. Optional types default to `None` if their value is not present during
//! decoding.
//!
//! 4. Optional enums default to `None` if an unknown variant is encountered
//! during decoding.
//!
//! Item **1** ensures that names can be changed freely without compatibility
//! concerns. Item **2** ensures that new fields do not affect older software.
//! Item **3** ensures that newer software can stop producing optional values.
//! Item **4** ensures that enums can get new variants that older software is
//! not aware of. By "fields" we mean the elements of structs and tuple structs
//! as well as enum structs and enum tuples. In addition, it is a compatible
//! change to turn a unit variant into a struct or tuple variant if all fields
//! are optional.
//!
//! From the above it should be obvious that *non-optional fields need to be
//! present forever*, so they should only be part of a type after careful
//! consideration.
//!
//! It should be emphasised that an `enum` itself can not be changed in a
//! compatible way. An unknown variant causes an error. It is only when they
//! are declared as an optional field type that unknown variants of an enum
//! are mapped to `None`. In other words, *only structs can be used as
//! top-level types in a forward and backward compatible way, enums can not.*
//!
//! # Example
//!
//! ```
//! use minicbor::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Point {
//!     #[n(0)] x: f64,
//!     #[n(1)] y: f64
//! }
//!
//! #[derive(Encode, Decode)]
//! struct ConvexHull {
//!     #[n(0)] left: Point,
//!     #[n(1)] right: Point,
//!     #[n(2)] points: Vec<Point>,
//!     #[n(3)] state: Option<State>
//! }
//!
//! #[derive(Encode, Decode)]
//! enum State {
//!     #[n(0)] Start,
//!     #[n(1)] Search { #[n(0)] info: u64 }
//! }
//! ```
//!
//! In this example the following changes would be compatible in both
//! directions:
//!
//! - Renaming every identifier.
//!
//! - Adding optional fields to `Point`, `ConvexHull`, `State::Start` or
//! `State::Search`.
//!
//! - Adding more variants to `State` *iff* `State` is only decoded as part of
//! `ConvexHull`. Direct decoding of `State` would produce an `UnknownVariant`
//! error for those new variants.
//!
//! [1]: https://developers.google.com/protocol-buffers/
//!
//! # Supported attributes
//!
//! - [`#[n(...)]` and `#[cbor(n(...))]`](#n-and-b-or-cborn-and-cborb)
//! - [`#[b(...)]` and `#[cbor(b(...))]`](#n-and-b-or-cborn-and-cborb)
//! - [`#[cbor(array)]`](#cborarray)
//! - [`#[cbor(map)]`](#cbormap)
//! - [`#[cbor(index_only)]`](#cborindex_only)
//! - [`#[cbor(transparent)]`](#cbortransparent)
//! - [`#[cbor(skip)]`](#cborskip)
//! - [`#[cbor(tag(...))]`](#cbortag)
//! - [`#[cbor(decode_with)]`](#cbordecode_with--path)
//! - [`#[cbor(encode_with)]`](#cborencode_with--path)
//! - [`#[cbor(with)]`](#cborwith--path)
//! - [`#[cbor(nil)]`](#cbornil--path)
//! - [`#[cbor(has_nil)]`](#cborhas_nil)
//! - [`#[cbor(is_nil)]`](#cboris_nil--path)
//! - [`#[cbor(decode_bound)]`](#cbordecode_bound--)
//! - [`#[cbor(encode_bound)]`](#cborencode_bound--)
//! - [`#[cbor(bound)]`](#cborbound)
//! - [`#[cbor(context_bound)]`](#cborcontext_bound--)
//! - [`#[cbor(cbor_len)]`](#cborcbor_len--path)
//!
//! ## `#[n(...)]` and `#[b(...)]` (or `#[cbor(n(...))]` and `#[cbor(b(...))]`)
//!
//! Each field and variant needs to be annotated with an index number, which is
//! used instead of the name. For the encoding it makes no difference which one
//! to choose. For decoding, `b` indicates that the value borrows from the
//! decoding input, whereas `n` produces non-borrowed values (but see section
//! [Implicit borrowing](#implicit-borrowing) below). This means that if a type
//! is annotated with `#[b(...)]`, all its lifetimes will be constrained to the
//! input lifetime (`'bytes`). Further, if the type is a `Cow<'_, str>`,
//! `Cow<'_, minicbor::bytes::ByteSlice>` or `Cow<'_, [u8]>` the generated code
//! will decode the `str`, `ByteSlice` or `[u8]` and construct a `Cow::Borrowed`
//! variant, contrary to the regular `Cow` impls of `Decode` and `DecodeBytes`
//! which produce owned values.
//!
//! ## `#[cbor(array)]`
//!
//! Uses a CBOR array to encode the annotated struct, enum or enum variant.
//! When used with an enum it applies to all its variants but can be overriden
//! per variant. See section [CBOR encoding](#cbor-encoding) for details.
//!
//! If neither `#[cbor(array)]` nor `#[cbor(map)]` are specified, `#[cbor(array)]`
//! is used by default.
//!
//! ## `#[cbor(map)]`
//!
//! Use a CBOR map to encode the annotated struct, enum or enum variant.
//! When used with an enum it applies to all its variants but can be overriden
//! per variant. See section [CBOR encoding](#cbor-encoding) for details.
//!
//! If neither `#[cbor(array)]` nor `#[cbor(map)]` are specified, `#[cbor(array)]`
//! is used by default.
//!
//! ## `#[cbor(index_only)]`
//!
//! Enumerations which do not contain fields may have this attribute attached to
//! them. This changes the encoding to encode only the variant index (cf. section
//! [CBOR encoding](#cbor-encoding) for details).
//!
//! ## `#[cbor(transparent)]`
//!
//! This attribute can be attached to structs with exactly one field (aka newtypes).
//! If present, the generated `Encode` and `Decode` impls will just forward the
//! respective `encode` and `decode` calls to the inner type, i.e. the resulting
//! CBOR representation will be identical to the one of the inner type.
//!
//! ## `#[cbor(skip)]`
//!
//! This attribute can be attached to fields in structs and enums and prevents
//! those fields from being encoded. Field types must implements [`Default`] and
//! when decoding the fields are initialised with `Default::default()`.
//!
//! ## `#[cbor(tag(...))]`
//!
//! This attribute can be attached to structs, enums and their fields. Its argument
//! is a base-10 unsigned integer which is encoded as the CBOR tag of the value.
//! Decoding will also attempt to read the tag and fails otherwise.
//!
//! ## `#[cbor(decode_with = "<path>")]`
//!
//! When applied to a field of type `T`, the function denoted by `<path>` will be
//! used to decode `T`. The function needs to be equivalent to the following type:
//!
//! ```no_run
//! use minicbor::decode::{Decoder, Error};
//!
//! fn decode<'b, Ctx, T: 'b>(d: &mut Decoder<'b>, ctx: &mut Ctx) -> Result<T, Error> {
//!     todo!()
//! }
//! ```
//!
//! Please note that if the decode function is generic in its context parameter that the
//! derive macro uses the type variable name `Ctx`.
//!
//! ## `#[cbor(encode_with = "<path>")]`
//!
//! When applied to a field of type `T`, the function denoted by `<path>` will be
//! used to encode `T`. The function needs to be equivalent to the following type:
//!
//! ```no_run
//! use minicbor::encode::{Encoder, Error, Write};
//!
//! fn encode<Ctx, T, W: Write>(v: &T, e: &mut Encoder<W>, ctx: &mut Ctx) -> Result<(), Error<W::Error>> {
//!     todo!()
//! }
//! ```
//!
//! Please note that if the encode function is generic in its context parameter that the
//! derive macro uses the type variable name `Ctx`.
//!
//! ## `#[cbor(with = "<path>")]`
//!
//! Combines [`#[cbor(decode_with = "...")]`](#cbordecode_with--path) and
//! [`#[cbor(encode_with = "...")]`](#cborencode_with--path). Here, `<path>` denotes
//! a module that contains functions named `encode` and `decode` that satisfy the
//! respective type signatures mentioned in `encode_with` and `decode_with`.
//! If `CborLen` is also derived, the module is assumed to contain a function named
//! `cbor_len` with a signature matching the one described in
//! [`#[cbor(cbor_len = "...")]`](#cborcbor_len--path) below.
//!
//! ## `#[cbor(nil = "<path>")]`
//!
//! Only valid in conjuction with [`#[cbor(decode_with = "...")]`](#cbordecode_with--path).
//! If present, `<path>` denotes a function to create a nil-like value of type `T`.
//! See `minicbor::Decode::nil` for details. The function needs to be equivalent to the
//! following type:
//!
//! ```no_run
//! fn nil<T>() -> Option<T> {
//!     todo!()
//! }
//! ```
//!
//! ## `#[cbor(has_nil)]`
//!
//! Only valid in conjuction with [`#[cbor(with = "...")]`](#cborwith--path). If present,
//! the attribute signals that the module denoted by `with` also contains functions `nil`
//! and `is_nil` to create nil values and to check if a value is a nil value.
//!
//! ## `#[cbor(is_nil = "<path>")]`
//!
//! Only valid in conjuction with [`#[cbor(encode_with = "...")]`](#cborencode_with--path).
//! If present, `<path>` denotes a function to check if a value of type `T` is a
//! nil-like value. See `minicbor::Encode::is_nil` for details. The function needs to
//! be equivalent to the following type:
//!
//! ```no_run
//! fn is_nil<T>(v: &T) -> bool {
//!     todo!()
//! }
//! ```
//!
//! ## `#[cbor(cbor_len = "<path>")]`
//!
//! Only applicable when deriving `CborLen`. When applied to a field of type `T`, the
//! function denoted by `<path>` will be used to calculate the CBOR length in bytes.
//! The function needs to be equivalent to the following type:
//!
//! ```no_run
//! fn cbor_len<Ctx, T>(val: &T, ctx: &mut Ctx) -> usize {
//!     todo!()
//! }
//! ```
//!
//! Please note that if the cbor_len function is generic in its context parameter that the
//! derive macro uses the type variable name `Ctx`.
//!
//! ## `#[cbor(decode_bound = "...")]`
//!
//! When applied to a generic field, this attribute overrides any implicit type
//! parameter bounds generated by `minicbor-derive` for the derived `Decode` impl.
//!
//! ## `#[cbor(encode_bound = "...")]`
//!
//! When applied to a generic field, this attribute overrides any implicit type
//! parameter bounds generated by `minicbor-derive` for the derived `Encode` impl.
//!
//! ## `#[cbor(bound)]`
//!
//! Combines [`#[cbor(encode_bound = "...")]`](#cborencode_bound--) and
//! [`#[cbor(decode_bound = "...")]`](#cbordecode_bound--), i.e. the bound applies
//! to the derived `Encode` and `Decode` impl.
//!
//! ## `#[cbor(context_bound = "...")]`
//!
//! When deriving `Encode` or `Decode` for a type which has parts that constrain the
//! generic context type parameter, this attribute can be used to add the required
//! trait bounds to the context type parameter. The attribute can either be repeated
//! or the bounds can be listed as '+'-separated value, e.g. "A + B + C".
//!
//! ### Example
//! <details>
//!     <summary>A combined context.</summary>
//!
//! ```no_run
//! use minicbor::{Encode, Decode};
//! use minicbor::decode::{self, Decoder};
//!
//! // Some decodable type that uses a custom context.
//! struct A(u8);
//!
//! // `A`'s context type.
//! struct AC { a: u8 }
//!
//! impl AsMut<AC> for AC {
//!     fn as_mut(&mut self) -> &mut AC { self }
//! }
//!
//! impl<'b, C: AsMut<AC>> Decode<'b, C> for A {
//!     fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
//!         Ok(A(ctx.as_mut().a))
//!     }
//! }
//!
//! // Another decodable type that uses a different context.
//! struct B(u8);
//!
//! // `B`'s context type.
//! struct BC { b: u8 }
//!
//! impl AsMut<BC> for BC {
//!     fn as_mut(&mut self) -> &mut BC { self }
//! }
//!
//! impl<'b, C: AsMut<BC>> Decode<'b, C> for B {
//!     fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
//!         Ok(B(ctx.as_mut().b))
//!     }
//! }
//!
//! // Finally, a type that combines `A` and `B` and therefore also needs to provide
//! // a context that can be used by both of them.
//! #[derive(Decode)]
//! #[cbor(context_bound = "AsMut<AC> + AsMut<BC>")]
//! struct C {
//!     #[n(0)] a: A,
//!     #[n(1)] b: B
//! }
//!
//! // The combined context type.
//! struct CC(AC, BC);
//!
//! impl AsMut<AC> for CC {
//!     fn as_mut(&mut self) -> &mut AC {
//!         &mut self.0
//!     }
//! }
//!
//! impl AsMut<BC> for CC {
//!     fn as_mut(&mut self) -> &mut BC {
//!         &mut self.1
//!     }
//! }
//!
//! ```
//! </details>
//!
//! # Implicit borrowing
//!
//! Apart from the explicit borrowing with [`#[b(...)]`](#n-and-b-or-cborn-and-cborb),
//! the following types implicitly borrow from the decoding input, which means
//! their lifetimes are constrained by the input lifetime:
//!
//! - `&'_ str`
//! - `&'_ minicbor::bytes::ByteSlice`
//! - `Option<&'_ str>`
//! - `Option<&'_ minicbor::bytes::ByteSlice>`
//!
//! ## What about `&[u8]`?
//!
//! `&[u8]` is a special case of `&[T]`. The lack of trait impl specialisation
//! in Rust makes it difficult to provide optimised support for byte slices.
//! The generic `[T]` impl of `Encode` produces an array of `T`s. To specifically
//! encode to and decode from CBOR bytes, the types `ByteSlice`, `ByteArray` and
//! `ByteVec` are provided by `minicbor`. In addition, the attributes
//! `encode_with`, `decode_with` and `with` can be used with `&[u8]` when deriving,
//! e.g.
//!
//! ```
//! use minicbor::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Foo<'a> {
//!     #[cbor(n(0), with = "minicbor::bytes")]
//!     field0: &'a [u8],
//!
//!     #[n(1)]
//!     #[cbor(encode_with = "minicbor::bytes::encode")]
//!     #[cbor(decode_with = "minicbor::bytes::decode")]
//!     field1: &'a [u8],
//!
//!     #[cbor(n(2), with = "minicbor::bytes")]
//!     field2: Option<&'a [u8]>,
//!
//!     #[cbor(n(3), with = "minicbor::bytes")]
//!     field3: Vec<u8>,
//!
//!     #[cbor(n(4), with = "minicbor::bytes")]
//!     field4: [u8; 16]
//! }
//! ```
//!
//! # CBOR encoding
//!
//! The CBOR values produced by a derived `Encode` implementation are of the
//! following formats.
//!
//! ## Structs
//!
//! ### Array encoding
//!
//! By default or if a struct has the [`#[cbor(array)]`](#cborarray) attribute,
//! it will be represented as a CBOR array. Its index numbers are represened by
//! the position of the field value in this array. Any gaps between index numbers
//! are filled with CBOR NULL values and `Option`s which are `None` likewise
//! end up as NULLs in this array.
//!
//! ```text
//! <<struct-as-array encoding>> =
//!     `array(n)`
//!         item_0
//!         item_1
//!         ...
//!         item_n
//! ```
//!
//! ### Map encoding
//!
//! If a struct has the [`#[cbor(map)]`](#cbormap) attribute attached, then it
//! will be represented as a CBOR map with keys corresponding to the numeric
//! index value:
//!
//! ```text
//! <<struct-as-map encoding>> =
//!     `map(n)`
//!         `0` item_0
//!         `1` item_1
//!         ...
//!          n  item_n
//! ```
//!
//! Optional fields whose value is `None` are not encoded.
//!
//! ## Enums
//!
//! Unless the [`#[cbor(index_only)]`](#cborindex_only) attribute is used for
//! enums without any fields, each enum variant is encoded as a two-element
//! array. The first element is the variant index and the second the actual
//! variant value. Otherwise, if enums do not have fields and the `index_only`
//! attribute is present, only the variant index is encoded:
//!
//! ```text
//! <<enum encoding>> =
//!     | `array(2)` n <<struct-as-array encoding>> ; if #[cbor(array)]
//!     | `array(2)` n <<struct-as-map encoding>>   ; if #[cbor(map)]
//!     | n                                         ; if #[cbor(index_only)]
//! ```
//!
//! ## Which encoding to use?
//!
//! The map encoding needs to represent the indexes explicitly in the encoding
//! which costs at least one extra byte per field value, whereas the array
//! encoding does not need to encode the indexes. On the other hand, absent
//! values, i.e. `None`s and gaps between indexes are not encoded with maps but
//! need to be encoded explicitly with arrays as NULLs which need one byte each.
//! Which encoding to choose depends therefore on the nature of the type that
//! should be encoded:
//!
//! - *Dense types* are types which contain only few `Option`s or their `Option`s
//! are assumed to be `Some`s usually. They are best encoded as arrays.
//!
//! - *Sparse types* are types with many `Option`s and their `Option`s are usually
//! `None`s. They are best encoded as maps.
//!
//! When selecting the encoding, future changes to the type should be considered
//! as they may turn a dense type into a sparse one over time. This also applies
//! to [`#[cbor(index_only)]`](#cborindex_only) which should be used only with
//! enums which are not expected to ever have fields in their variants.
//!
//! [^1]: CBOR items are ignored using `Decoder::skip`. This method requires
//! feature "alloc" to work for all possible CBOR items. Without "alloc",
//! indefinite maps or arrays inside of regular maps or arrays can not be skipped
//! over. If such a combination occurs and `Decoder::skip` was compiled without
//! feature "alloc", a decoding error is returned.

extern crate proc_macro;

mod decode;
mod encode;
mod cbor_len;

pub(crate) mod attrs;
pub(crate) mod fields;
pub(crate) mod lifetimes;
pub(crate) mod variants;

use std::collections::HashSet;

/// Derive the `minicbor::Decode` trait for a struct or enum.
///
/// See the [crate] documentation for details.
#[proc_macro_derive(Decode, attributes(n, b, cbor))]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    decode::derive_from(input)
}

/// Derive the `minicbor::Encode` trait for a struct or enum.
///
/// See the [crate] documentation for details.
#[proc_macro_derive(Encode, attributes(n, b, cbor))]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    encode::derive_from(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    Encode,
    Decode
}

/// Derive the `minicbor::CborLen` trait for a struct or enum.
///
/// See the [crate] documentation for details.
#[proc_macro_derive(CborLen, attributes(n, b, cbor))]
pub fn derive_cbor_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cbor_len::derive_from(input)
}

// Helpers ////////////////////////////////////////////////////////////////////

/// Check if the given type is an `Option` whose inner type matches the predicate.
fn is_option(ty: &syn::Type, pred: impl FnOnce(&syn::Type) -> bool) -> bool {
    if let syn::Type::Path(t) = ty {
        if let Some(s) = t.path.segments.last() {
            if s.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(b) = &s.arguments {
                    if b.args.len() == 1 {
                        if let syn::GenericArgument::Type(ty) = &b.args[0] {
                            return pred(ty)
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check if the given type is a `Cow` whose inner type matches the predicate.
fn is_cow(ty: &syn::Type, pred: impl FnOnce(&syn::Type) -> bool) -> bool {
    if let syn::Type::Path(t) = ty {
        if let Some(s) = t.path.segments.last() {
            if s.ident == "Cow" {
                if let syn::PathArguments::AngleBracketed(b) = &s.arguments {
                    if b.args.len() == 2 {
                        if let syn::GenericArgument::Lifetime(_) = &b.args[0] {
                            if let syn::GenericArgument::Type(ty) = &b.args[1] {
                                return pred(ty)
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check if the given type is a `&str`.
fn is_str(ty: &syn::Type) -> bool {
    if let syn::Type::Path(t) = ty {
        t.qself.is_none() && t.path.segments.len() == 1 && t.path.segments[0].ident == "str"
    } else {
        false
    }
}

/// Check if the given type is a `&[u8]`.
fn is_byte_slice(ty: &syn::Type) -> bool {
    if let syn::Type::Path(t) = ty {
        return t.qself.is_none() &&
            ((t.path.segments.len() == 1 && t.path.segments[0].ident == "ByteSlice")
                || (t.path.segments.len() == 2
                    && t.path.segments[0].ident == "bytes"
                    && t.path.segments[1].ident == "ByteSlice")
                || (t.path.segments.len() == 3
                    && t.path.segments[0].ident == "minicbor"
                    && t.path.segments[1].ident == "bytes"
                    && t.path.segments[2].ident == "ByteSlice"))
    }
    if let syn::Type::Slice(t) = ty {
        if let syn::Type::Path(t) = &*t.elem {
            t.qself.is_none() && t.path.segments.len() == 1 && t.path.segments[0].ident == "u8"
        } else {
            false
        }
    } else {
        false
    }
}

/// Traverse all field types and collect all type parameters along the way.
fn collect_type_params<'a, I>(all: &syn::Generics, fields: I) -> HashSet<syn::TypeParam>
where
    I: Iterator<Item = &'a fields::Field>
{
    use syn::visit::Visit;

    struct Collector {
        all: Vec<syn::Ident>,
        found: HashSet<syn::TypeParam>
    }

    impl<'a> Visit<'a> for Collector {
        fn visit_field(&mut self, f: &'a syn::Field) {
            if let syn::Type::Path(ty) = &f.ty {
                if let Some(t) = ty.path.segments.first() {
                    if self.all.contains(&t.ident) {
                        self.found.insert(syn::TypeParam::from(t.ident.clone()));
                    }
                }
            }
            self.visit_type(&f.ty)
        }

        fn visit_path(&mut self, p: &'a syn::Path) {
            if p.leading_colon.is_none() && p.segments.len() == 1 {
                let id = &p.segments[0].ident;
                if self.all.contains(id) {
                    self.found.insert(syn::TypeParam::from(id.clone()));
                }
            }
            syn::visit::visit_path(self, p)
        }
    }

    let mut c = Collector {
        all: all.type_params().map(|tp| tp.ident.clone()).collect(),
        found: HashSet::new()
    };

    for f in fields {
        c.visit_field(&f.orig)
    }

    c.found
}

fn add_bound_to_type_params<'a, I, A>
    ( bound: syn::TypeParamBound
    , params: I
    , blacklist: &HashSet<syn::TypeParam>
    , attrs: A
    , mode: Mode
    )
where
    I: IntoIterator<Item = &'a mut syn::TypeParam>,
    A: IntoIterator<Item = &'a attrs::Attributes> + Clone
{
    let find_type_param = |t: &syn::TypeParam| attrs.clone().into_iter()
        .find_map(|a| {
            a.type_params().and_then(|p| match mode {
                Mode::Encode => p.get_encode(&t.ident),
                Mode::Decode => p.get_decode(&t.ident)
            })
        });

    for p in params {
        if let Some(t) = find_type_param(p) {
            p.bounds.extend(t.bounds.iter().cloned())
        } else if !blacklist.contains(p) {
            p.bounds.push(bound.clone())
        }
    }
}

fn add_typeparam<'a, I>(g: &syn::Generics, mut t: syn::TypeParam, b: Option<I>) -> syn::Generics
where
    I: Iterator<Item = &'a syn::TraitBound>
{
    let mut g2 = g.clone();
    if let Some(bounds) = b {
        t.bounds.extend(bounds.cloned().map(syn::TypeParamBound::Trait))
    }
    g2.params = Some(t.into()).into_iter().chain(g2.params).collect();
    g2
}

fn gen_ctx_param() -> syn::Result<syn::TypeParam> {
    syn::parse_str("Ctx")
}

