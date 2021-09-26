//! Traits and types for encoding CBOR.
//!
//! This module defines the trait [`Encode`] and the actual [`Encoder`].
//! It also defines a [`Write`] trait to store the encoded bytes.

mod encoder;
mod error;
pub mod write;

pub use encoder::Encoder;
pub use error::Error;
pub use write::Write;

/// A type that can be encoded to CBOR.
///
/// If this type's CBOR encoding is meant to be decoded by `Decode` impls
/// derived with [`minicbor_derive`] *it is advisable to only produce a
/// single CBOR data item*. Tagging, maps or arrays can and should be used
/// for multiple values.
pub trait Encode {
    /// Encode a value of this type using the given `Encoder`.
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>>;
}

impl<T: Encode + ?Sized> Encode for &T {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        (**self).encode(e)
    }
}

impl<T: Encode + ?Sized> Encode for &mut T {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        (**self).encode(e)
    }
}

#[cfg(feature = "alloc")]
impl<T: Encode + ?Sized> Encode for alloc::boxed::Box<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        (**self).encode(e)
    }
}

impl Encode for str {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.str(self)?.ok()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        if let Some(x) = self {
            x.encode(e)?;
        } else {
            e.null()?;
        }
        Ok(())
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?;
        match self {
            Ok(v)  => e.u32(0)?.encode(v)?.ok(),
            Err(v) => e.u32(1)?.encode(v)?.ok()
        }
    }
}

#[cfg(feature = "alloc")]
impl Encode for alloc::string::String {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.str(self)?.ok()
    }
}

#[cfg(feature = "alloc")]
impl<T> Encode for alloc::borrow::Cow<'_, T>
where
    T: Encode + alloc::borrow::ToOwned + ?Sized
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.as_ref().encode(e)
    }
}

#[cfg(feature = "std")]
impl<T, S> Encode for std::collections::HashSet<T, S>
where
    T: Encode,
    S: std::hash::BuildHasher
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(as_u64(self.len()))?;
        for x in self {
            x.encode(e)?
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<K, V, S> Encode for std::collections::HashMap<K, V, S>
where
    K: Encode + Eq + std::hash::Hash,
    V: Encode,
    S: std::hash::BuildHasher
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.map(as_u64(self.len()))?;
        for (k, v) in self {
            k.encode(e)?;
            v.encode(e)?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<K, V> Encode for alloc::collections::BTreeMap<K, V>
where
    K: Encode + Eq + Ord,
    V: Encode
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.map(as_u64(self.len()))?;
        for (k, v) in self {
            k.encode(e)?;
            v.encode(e)?;
        }
        Ok(())
    }
}

impl<T> Encode for core::marker::PhantomData<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(0)?.ok()
    }
}

impl Encode for () {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(0)?.ok()
    }
}

impl<T: Encode> Encode for core::num::Wrapping<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.0.encode(e)
    }
}

#[cfg(target_pointer_width = "32")]
impl Encode for usize {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.u32(*self as u32)?.ok()
    }
}

#[cfg(target_pointer_width = "64")]
impl Encode for usize {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.u64(*self as u64)?.ok()
    }
}

#[cfg(target_pointer_width = "32")]
impl Encode for isize {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.i32(*self as i32)?.ok()
    }
}

#[cfg(target_pointer_width = "64")]
impl Encode for isize {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.i64(*self as i64)?.ok()
    }
}

macro_rules! encode_basic {
    ($($t:ident)*) => {
        $(
            impl Encode for $t {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    e.$t(*self)?;
                    Ok(())
                }
            }
        )*
    }
}

encode_basic!(u8 i8 u16 i16 u32 i32 u64 i64 bool f32 f64 char);

macro_rules! encode_nonzero {
    ($($t:ty)*) => {
        $(
            impl Encode for $t {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    self.get().encode(e)
                }
            }
        )*
    }
}

encode_nonzero! {
    core::num::NonZeroU8
    core::num::NonZeroU16
    core::num::NonZeroU32
    core::num::NonZeroU64
    core::num::NonZeroI8
    core::num::NonZeroI16
    core::num::NonZeroI32
    core::num::NonZeroI64
}

#[cfg(any(atomic32, atomic64))]
macro_rules! encode_atomic {
    ($($t:ty)*) => {
        $(
            impl Encode for $t {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    self.load(core::sync::atomic::Ordering::SeqCst).encode(e)?;
                    Ok(())
                }
            }
        )*
    }
}

#[cfg(atomic32)]
encode_atomic! {
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
encode_atomic! {
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

macro_rules! encode_sequential {
    ($($t:ty)*) => {
        $(
            impl<T: Encode> Encode for $t {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    e.array(as_u64(self.len()))?;
                    for x in self {
                        x.encode(e)?
                    }
                    Ok(())
                }
            }
        )*
    }
}

encode_sequential!([T]);

#[cfg(feature = "alloc")]
encode_sequential! {
    alloc::vec::Vec<T>
    alloc::collections::VecDeque<T>
    alloc::collections::LinkedList<T>
    alloc::collections::BinaryHeap<T>
    alloc::collections::BTreeSet<T>
}

macro_rules! encode_arrays {
    ($($n:expr)*) => {
        $(
            impl<T: Encode> Encode for [T; $n] {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    e.array($n)?;
                    for x in self {
                        x.encode(e)?
                    }
                    Ok(())
                }
            }
        )*
    }
}

encode_arrays!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16);

macro_rules! encode_tuples {
    ($( $len:expr => { $($T:ident ($idx:tt))+ } )+) => {
        $(
            impl<$($T: Encode),+> Encode for ($($T,)+) {
                fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
                    e.array($len)?
                        $(.encode(&self.$idx)?)+
                        .ok()
                }
            }
        )+
    }
}

encode_tuples! {
    1  => { A(0) }
    2  => { A(0) B(1) }
    3  => { A(0) B(1) C(2) }
    4  => { A(0) B(1) C(2) D(3) }
    5  => { A(0) B(1) C(2) D(3) E(4) }
    6  => { A(0) B(1) C(2) D(3) E(4) F(5) }
    7  => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) }
    8  => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) }
    9  => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) }
    10 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) }
    11 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) }
    12 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) L(11) }
    13 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) L(11) M(12) }
    14 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) L(11) M(12) N(13) }
    15 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) L(11) M(12) N(13) O(14) }
    16 => { A(0) B(1) C(2) D(3) E(4) F(5) G(6) H(7) I(8) J(9) K(10) L(11) M(12) N(13) O(14) P(15) }
}

impl Encode for core::time::Duration {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?
            .encode(self.as_secs())?
            .encode(self.subsec_nanos())?
            .ok()
    }
}

#[cfg(feature = "std")]
impl Encode for std::time::SystemTime {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        if let Ok(d) = self.duration_since(std::time::UNIX_EPOCH) {
            d.encode(e)
        } else {
            Err(Error::Message("could not determine system time duration"))
        }
    }
}

impl<T: Encode + Copy> Encode for core::cell::Cell<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.get().encode(e)
    }
}

impl<T: Encode> Encode for core::cell::RefCell<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        if let Ok(v) = self.try_borrow() {
            v.encode(e)
        } else {
            Err(Error::Message("could not borrow ref cell value"))
        }
    }
}

#[cfg(feature = "std")]
impl Encode for std::path::Path {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        if let Some(s) = self.to_str() {
            e.str(s)?.ok()
        } else {
            Err(Error::Message("non-utf-8 path values are not supported"))
        }
    }
}

#[cfg(feature = "std")]
impl Encode for std::path::PathBuf {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.as_path().encode(e)
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::IpAddr {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?;
        match self {
            std::net::IpAddr::V4(a) => e.u32(0)?.encode(a)?.ok(),
            std::net::IpAddr::V6(a) => e.u32(1)?.encode(a)?.ok()
        }
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::Ipv4Addr {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.octets().encode(e)
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::Ipv6Addr {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.octets().encode(e)
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::SocketAddr {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?;
        match self {
            std::net::SocketAddr::V4(a) => e.u32(0)?.encode(a)?.ok(),
            std::net::SocketAddr::V6(a) => e.u32(1)?.encode(a)?.ok()
        }
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::SocketAddrV4 {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?
            .encode(self.ip())?
            .encode(self.port())?
            .ok()
    }
}

#[cfg(feature = "std")]
impl Encode for std::net::SocketAddrV6 {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?
            .encode(self.ip())?
            .encode(self.port())?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::Range<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?
            .encode(&self.start)?
            .encode(&self.end)?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::RangeFrom<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(1)?
            .encode(&self.start)?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::RangeTo<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(1)?
            .encode(&self.end)?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::RangeToInclusive<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(1)?
            .encode(&self.end)?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::RangeInclusive<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?
            .encode(self.start())?
            .encode(self.end())?
            .ok()
    }
}

impl<T: Encode> Encode for core::ops::Bound<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        e.array(2)?;
        match self {
            core::ops::Bound::Included(v) => e.u32(0)?.encode(v)?.ok(),
            core::ops::Bound::Excluded(v) => e.u32(1)?.encode(v)?.ok(),
            core::ops::Bound::Unbounded   => e.u32(2)?.array(0)?.ok()
        }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn as_u64(n: usize) -> u64 {
    n as u64
}

