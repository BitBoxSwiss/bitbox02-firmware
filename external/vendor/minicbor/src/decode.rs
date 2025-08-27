//! Traits and types for decoding CBOR.
//!
//! This module defines the trait [`Decode`] and the actual [`Decoder`].

use core::mem::MaybeUninit;

mod decoder;
mod error;
pub mod info;

use crate::data::{Int, Tag, Tagged};

pub use decoder::{Decoder, Probe};
pub use decoder::{ArrayIter, ArrayIterWithCtx, BytesIter, MapIter, MapIterWithCtx, StrIter};
pub use error::Error;

#[cfg(feature = "half")]
mod tokenizer;

#[cfg(feature = "half")]
pub use tokenizer::Tokenizer;

#[cfg(feature = "half")]
#[deprecated(since = "0.23.0", note = "import `Token` from `minicbor::data` instead")]
pub type Token<'b> = crate::data::Token<'b>;

/// A type that can be decoded from CBOR.
pub trait Decode<'b, C>: Sized {
    /// Decode a value using the given `Decoder`.
    ///
    /// In addition to the decoder a user provided decoding context is given
    /// as another parameter. Most implementations of this trait do not need
    /// a decoding context and should be completely generic in the context
    /// type. In cases where a context is needed and the `Decode` impl type is
    /// meant to be combined with other types that require a different context
    /// type, it is preferrable to constrain the context type variable `C` with
    /// a trait bound instead of fixing the type.
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error>;

    /// If possible, return a nil value of `Self`.
    ///
    /// This method is primarily used by `minicbor-derive` and allows
    /// creating a special value denoting the absence of a "real" value if
    /// no CBOR value is present. The canonical example of a type where
    /// this is sensible is the `Option` type, whose `Decode::nil` method
    /// would return `Some(None)`.
    ///
    /// With the exception of `Option<_>` all types `T` are considered
    /// mandatory by default, i.e. `T::nil()` returns `None`. Missing values
    /// of `T` therefore cause decoding errors in derived `Decode`
    /// implementations.
    ///
    /// NB: A type implementing `Decode` with an overriden `Decode::nil`
    /// method should also override `Encode::is_nil` if it implements `Encode`
    /// at all.
    fn nil() -> Option<Self> {
        None
    }
}

#[cfg(feature = "alloc")]
impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for alloc::boxed::Box<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        T::decode(d, ctx).map(alloc::boxed::Box::new)
    }
}

impl<'a, 'b: 'a, C> Decode<'b, C> for &'a str {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.str()
    }
}

#[cfg(feature = "alloc")]
impl<'b, C, T> Decode<'b, C> for alloc::borrow::Cow<'_, T>
where
    T: alloc::borrow::ToOwned + ?Sized,
    T::Owned: Decode<'b, C>
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(alloc::borrow::Cow::Owned)
    }
}

#[cfg(feature = "alloc")]
impl<'b, C> Decode<'b, C> for alloc::string::String {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.str().map(alloc::string::String::from)
    }
}

impl<'a, 'b: 'a, C> Decode<'b, C> for &'a core::ffi::CStr {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        let p = d.position();
        let b = d.bytes()?;
        core::ffi::CStr::from_bytes_with_nul(b).map_err(|_| Error::message("invalid c-string").at(p))
    }
}

#[cfg(feature = "alloc")]
impl<'b, C> Decode<'b, C> for alloc::ffi::CString {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        let c: &core::ffi::CStr = d.decode()?;
        Ok(Self::from(c))
    }
}

#[cfg(feature = "alloc")]
impl<'b, C> Decode<'b, C> for alloc::boxed::Box<str> {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.str().map(Into::into)
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for Option<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        if crate::data::Type::Null == d.datatype()? {
            d.skip()?;
            return Ok(None)
        }
        T::decode(d, ctx).map(Some)
    }

    fn nil() -> Option<Self> {
        Some(None)
    }
}

impl<'b, C, T, E> Decode<'b, C> for Result<T, E>
where
    T: Decode<'b, C>,
    E: Decode<'b, C>
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(Error::message("expected enum (2-element array)").at(p))
        }
        let p = d.position();
        match d.u32()? {
            0 => T::decode(d, ctx).map(Ok),
            1 => E::decode(d, ctx).map(Err),
            n => Err(Error::unknown_variant(n).at(p))
        }
    }
}

#[cfg(feature = "alloc")]
impl<'b, C, T> Decode<'b, C> for alloc::collections::BinaryHeap<T>
where
    T: Decode<'b, C> + Ord
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let iter: ArrayIterWithCtx<C, T> = d.array_iter_with(ctx)?;
        let mut v = alloc::collections::BinaryHeap::new();
        for x in iter {
            v.push(x?)
        }
        Ok(v)
    }
}

#[cfg(feature = "std")]
impl<'b, C, T, S> Decode<'b, C> for std::collections::HashSet<T, S>
where
    T: Decode<'b, C> + Eq + std::hash::Hash,
    S: std::hash::BuildHasher + std::default::Default
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let iter: ArrayIterWithCtx<C, T> = d.array_iter_with(ctx)?;
        let mut v = std::collections::HashSet::default();
        for x in iter {
            v.insert(x?);
        }
        Ok(v)
    }
}

#[cfg(feature = "alloc")]
impl<'b, C, T> Decode<'b, C> for alloc::collections::BTreeSet<T>
where
    T: Decode<'b, C> + Ord
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let iter: ArrayIterWithCtx<C, T> = d.array_iter_with(ctx)?;
        let mut v = alloc::collections::BTreeSet::new();
        for x in iter {
            v.insert(x?);
        }
        Ok(v)
    }
}

#[cfg(feature = "std")]
impl<'b, C, K, V, S> Decode<'b, C> for std::collections::HashMap<K, V, S>
where
    K: Decode<'b, C> + Eq + std::hash::Hash,
    V: Decode<'b, C>,
    S: std::hash::BuildHasher + std::default::Default
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let mut m = std::collections::HashMap::default();
        let iter: MapIterWithCtx<C, K, V> = d.map_iter_with(ctx)?;
        for x in iter {
            let (k, v) = x?;
            m.insert(k, v);
        }
        Ok(m)
    }
}

#[cfg(feature = "alloc")]
impl<'b, C, K, V> Decode<'b, C> for alloc::collections::BTreeMap<K, V>
where
    K: Decode<'b, C> + Eq + Ord,
    V: Decode<'b, C>
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let mut m = alloc::collections::BTreeMap::new();
        let iter: MapIterWithCtx<C, K, V> = d.map_iter_with(ctx)?;
        for x in iter {
            let (k, v) = x?;
            m.insert(k, v);
        }
        Ok(m)
    }
}

impl<'b, C, T> Decode<'b, C> for core::marker::PhantomData<T> {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(0) != d.array()? {
            return Err(Error::message("expected phantom data, i.e. an empty array").at(p))
        }
        Ok(core::marker::PhantomData)
    }
}

impl<'b, C> Decode<'b, C> for () {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(0) != d.array()? {
            return Err(Error::message("expected unit, i.e. an empty array").at(p))
        }
        Ok(())
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::num::Wrapping<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(core::num::Wrapping)
    }
}

#[cfg(target_pointer_width = "32")]
impl<'b, C> Decode<'b, C> for usize {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.u32().map(|n| n as usize)
    }
}

#[cfg(target_pointer_width = "64")]
impl<'b, C> Decode<'b, C> for usize {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.u64().map(|n| n as usize)
    }
}

#[cfg(target_pointer_width = "32")]
impl<'b, C> Decode<'b, C> for isize {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.i32().map(|n| n as isize)
    }
}

#[cfg(target_pointer_width = "64")]
impl<'b, C> Decode<'b, C> for isize {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.i64().map(|n| n as isize)
    }
}

impl<'b, C> Decode<'b, C> for Int {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.int()
    }
}

impl<'b, C> Decode<'b, C> for Tag {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.tag()
    }
}

impl<'b, C, const N: u64, T: Decode<'b, C>> Decode<'b, C> for Tagged<N, T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        let t = d.tag()?;
        if N != t.as_u64() {
            #[cfg(feature = "alloc")]
            return Err(Error::tag_mismatch(t).with_message(alloc::format!("expected tag {N}")).at(p));
            #[cfg(not(feature = "alloc"))]
            return Err(Error::tag_mismatch(t).at(p))
        }
        let v = d.decode_with(ctx)?;
        Ok(Tagged::new(v))
    }
}

macro_rules! decode_basic {
    ($($t:ident)*) => {
        $(
            impl<'b, C> Decode<'b, C> for $t {
                fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
                    d.$t()
                }
            }
        )*
    }
}

decode_basic!(u8 i8 u16 i16 u32 i32 u64 i64 bool f32 f64 char);

macro_rules! decode_nonzero {
    ($($t:ty, $msg:expr)*) => {
        $(
            impl<'b, C> Decode<'b, C> for $t {
                fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
                    let p = d.position();
                    <$t>::new(Decode::decode(d, ctx)?).ok_or_else(|| Error::message($msg).at(p))
                }
            }
        )*
    }
}

decode_nonzero! {
    core::num::NonZeroU8,  "unexpected 0 when decoding a `NonZeroU8`"
    core::num::NonZeroU16, "unexpected 0 when decoding a `NonZeroU16`"
    core::num::NonZeroU32, "unexpected 0 when decoding a `NonZeroU32`"
    core::num::NonZeroU64, "unexpected 0 when decoding a `NonZeroU64`"
    core::num::NonZeroI8,  "unexpected 0 when decoding a `NonZeroI8`"
    core::num::NonZeroI16, "unexpected 0 when decoding a `NonZeroI16`"
    core::num::NonZeroI32, "unexpected 0 when decoding a `NonZeroI32`"
    core::num::NonZeroI64, "unexpected 0 when decoding a `NonZeroI64`"
}

#[cfg(any(atomic32, atomic64))]
macro_rules! decode_atomic {
    ($($t:ty)*) => {
        $(
            impl<'b, C> Decode<'b, C> for $t {
                fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
                    d.decode_with(ctx).map(<$t>::new)
                }
            }
        )*
    }
}

#[cfg(atomic32)]
decode_atomic! {
    core::sync::atomic::AtomicBool
    core::sync::atomic::AtomicU8
    core::sync::atomic::AtomicU16
    core::sync::atomic::AtomicU32
    core::sync::atomic::AtomicUsize
    core::sync::atomic::AtomicI8
    core::sync::atomic::AtomicI16
    core::sync::atomic::AtomicI32
    core::sync::atomic::AtomicIsize
}

#[cfg(atomic64)]
decode_atomic! {
    core::sync::atomic::AtomicBool
    core::sync::atomic::AtomicU8
    core::sync::atomic::AtomicU16
    core::sync::atomic::AtomicU32
    core::sync::atomic::AtomicU64
    core::sync::atomic::AtomicUsize
    core::sync::atomic::AtomicI8
    core::sync::atomic::AtomicI16
    core::sync::atomic::AtomicI32
    core::sync::atomic::AtomicI64
    core::sync::atomic::AtomicIsize
}

#[cfg(feature = "alloc")]
macro_rules! decode_sequential {
    ($($t:ty, $push:ident)*) => {
        $(
            impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for $t {
                fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
                    let iter: ArrayIterWithCtx<C, T> = d.array_iter_with(ctx)?;
                    let mut v = <$t>::new();
                    for x in iter {
                        v.$push(x?)
                    }
                    Ok(v)
                }
            }
        )*
    }
}

#[cfg(feature = "alloc")]
decode_sequential! {
    alloc::vec::Vec<T>, push
    alloc::collections::VecDeque<T>, push_back
    alloc::collections::LinkedList<T>, push_back
}

struct ArrayVec<T, const N: usize>{
    len: usize,
    buffer: [MaybeUninit<T>; N],
}

impl <T, const N: usize> ArrayVec<T, N> {
    const ELEM: MaybeUninit<T> = MaybeUninit::uninit();

    fn new() -> Self {
        Self {
            len: 0,
            buffer: [Self::ELEM; N]
        }
    }

    fn into_array(self) -> Result<[T; N], Self> {
        if self.len == N {
            let array = unsafe {
                (&self.buffer as *const [MaybeUninit<T>; N] as *const [T; N]).read()
            };

            // We don't want `self`'s destructor to be called because that would drop all the
            // items in the array
            core::mem::forget(self);

            Ok(array)
        } else {
            Err(self)
        }
    }

    fn push(&mut self, item: T) -> Result<(), T> {
        if let Some(slot) = self.buffer.get_mut(self.len) {
            slot.write(item);
            self.len += 1;
            Ok(())
        } else {
            Err(item)
        }
    }
}

impl <T, const N: usize> core::ops::Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        unsafe {
            let s = core::slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut T, self.len);
            core::ptr::drop_in_place(s)
        }
    }
}

impl<'b, C, T: Decode<'b, C>, const N: usize> Decode<'b, C> for [T; N] {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        let iter: ArrayIterWithCtx<C, T> = d.array_iter_with(ctx)?;
        let mut a = ArrayVec::<T, N>::new();
        for x in iter {
            a.push(x?).map_err(|_| {
                #[cfg(feature = "alloc")]
                let msg = &alloc::format!("array has more than {N} elements");
                #[cfg(not(feature = "alloc"))]
                let msg = "array has too many elements";
                Error::message(msg).at(p)
            })?;
        }
        a.into_array().map_err(|_| {
            #[cfg(feature = "alloc")]
            let msg = &alloc::format!("array has less than {N} elements");
            #[cfg(not(feature = "alloc"))]
            let msg = "array has too few elements";
            Error::message(msg).at(p)
        })
    }
}

macro_rules! decode_tuples {
    ($( $len:expr => { $($T:ident)+ } )+) => {
        $(
            impl<'b, Ctx, $($T: Decode<'b, Ctx>),+> Decode<'b, Ctx> for ($($T,)+) {
                fn decode(d: &mut Decoder<'b>, ctx: &mut Ctx) -> Result<Self, Error> {
                    let p = d.position();
                    let n = d.array()?;
                    if n != Some($len) {
                        return Err(Error::message(concat!("invalid ", $len, "-tuple length")).at(p))
                    }
                    Ok(($($T::decode(d, ctx)?,)+))
                }
            }
        )+
    }
}

decode_tuples! {
    1  => { A }
    2  => { A B }
    3  => { A B C }
    4  => { A B C D }
    5  => { A B C D E }
    6  => { A B C D E F }
    7  => { A B C D E F G }
    8  => { A B C D E F G H }
    9  => { A B C D E F G H I }
    10 => { A B C D E F G H I J }
    11 => { A B C D E F G H I J K }
    12 => { A B C D E F G H I J K L }
    13 => { A B C D E F G H I J K L M }
    14 => { A B C D E F G H I J K L M N }
    15 => { A B C D E F G H I J K L M N O }
    16 => { A B C D E F G H I J K L M N O P }
}

macro_rules! decode_fields {
    ($d:ident $c:ident | $($n:literal $x:ident => $t:ty ; $msg:literal)*) => {
        $(let mut $x : core::option::Option<$t> = None;)*

        let p = $d.position();

        match $d.array()? {
            Some(n) => for i in 0 .. n {
                match i {
                    $($n => $x = Some(Decode::decode($d, $c)?),)*
                    _    => $d.skip()?
                }
            }
            None => {
                let mut i = 0;
                while $d.datatype()? != crate::data::Type::Break {
                    match i {
                        $($n => $x = Some(Decode::decode($d, $c)?),)*
                        _    => $d.skip()?
                    }
                    i += 1
                }
                $d.skip()?
            }
        }

        $(let $x = if let Some(x) = $x {
            x
        } else {
            return Err(Error::missing_value($n).at(p).with_message($msg))
        };)*
    }
}

impl<'b, C> Decode<'b, C> for core::time::Duration {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 secs  => u64 ; "Duration::secs"
            1 nanos => u32 ; "Duration::nanos"
        }
        Ok(core::time::Duration::new(secs, nanos))
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::time::SystemTime {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        std::time::UNIX_EPOCH
            .checked_add(d.decode_with(ctx)?)
            .ok_or_else(|| Error::message("duration value can not represent system time").at(p))
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::cell::Cell<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(core::cell::Cell::new)
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::cell::RefCell<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(core::cell::RefCell::new)
    }
}

#[cfg(feature = "std")]
impl<'a, 'b: 'a, C> Decode<'b, C> for &'a std::path::Path {
    fn decode(d: &mut Decoder<'b>, _: &mut C) -> Result<Self, Error> {
        d.str().map(std::path::Path::new)
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for Box<std::path::Path> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(std::path::PathBuf::into_boxed_path)
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::path::PathBuf {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        d.decode_with(ctx).map(std::path::Path::to_path_buf)
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::IpAddr {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(Error::message("expected enum (2-element array)").at(p))
        }
        let p = d.position();
        match d.u32()? {
            0 => Ok(std::net::Ipv4Addr::decode(d, ctx)?.into()),
            1 => Ok(std::net::Ipv6Addr::decode(d, ctx)?.into()),
            n => Err(Error::unknown_variant(n).at(p))
        }
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::Ipv4Addr {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let octets: crate::bytes::ByteArray<4> = Decode::decode(d, ctx)?;
        Ok(<[u8; 4]>::from(octets).into())
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::Ipv6Addr {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let octets: crate::bytes::ByteArray<16> = Decode::decode(d, ctx)?;
        Ok(<[u8; 16]>::from(octets).into())
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::SocketAddr {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(Error::message("expected enum (2-element array)").at(p))
        }
        let p = d.position();
        match d.u32()? {
            0 => Ok(std::net::SocketAddrV4::decode(d, ctx)?.into()),
            1 => Ok(std::net::SocketAddrV6::decode(d, ctx)?.into()),
            n => Err(Error::unknown_variant(n).at(p))
        }
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::SocketAddrV4 {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 ip   => std::net::Ipv4Addr ; "SocketAddrV4::ip"
            1 port => u16                ; "SocketAddrV4::port"
        }
        Ok(std::net::SocketAddrV4::new(ip, port))
    }
}

#[cfg(feature = "std")]
impl<'b, C> Decode<'b, C> for std::net::SocketAddrV6 {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 ip   => std::net::Ipv6Addr ; "SocketAddrV6::ip"
            1 port => u16                ; "SocketAddrV6::port"
        }
        Ok(std::net::SocketAddrV6::new(ip, port, 0, 0))
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b,C > for core::ops::Range<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 start => T ; "Range::start"
            1 end   => T ; "Range::end"
        }
        Ok(core::ops::Range { start, end })
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::ops::RangeFrom<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 start => T ; "RangeFrom::start"
        }
        Ok(core::ops::RangeFrom { start })
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::ops::RangeTo<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 end => T ; "RangeTo::end"
        }
        Ok(core::ops::RangeTo { end })
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::ops::RangeToInclusive<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 end => T ; "RangeToInclusive::end"
        }
        Ok(core::ops::RangeToInclusive { end })
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::ops::RangeInclusive<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        decode_fields! { d ctx |
            0 start => T ; "RangeInclusive::start"
            1 end   => T ; "RangeInclusive::end"
        }
        Ok(core::ops::RangeInclusive::new(start, end))
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for core::ops::Bound<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(Error::message("expected enum (2-element array)").at(p))
        }
        let p = d.position();
        match d.u32()? {
            0 => d.decode_with(ctx).map(core::ops::Bound::Included),
            1 => d.decode_with(ctx).map(core::ops::Bound::Excluded),
            2 => d.skip().map(|_| core::ops::Bound::Unbounded),
            n => Err(Error::unknown_variant(n).at(p))
        }
    }
}

