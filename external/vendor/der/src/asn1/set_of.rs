//! ASN.1 `SET OF` support.
//!
//! # Ordering Notes
//!
//! Some DER serializer implementations fail to properly sort elements of a `SET OF`. This is
//! technically non-canonical, but occurs frequently enough that most DER decoders tolerate it.
//!
//! When decoding with `EncodingRules::Der`, this implementation sorts the elements of `SET OF` at
//! decode-time to ensure reserializations are canonical.

#![cfg(any(feature = "alloc", feature = "heapless"))]

use crate::{
    Decode, DecodeValue, DerOrd, Encode, EncodeValue, Error, ErrorKind, FixedTag, Header, Length,
    Reader, SliceReader, Tag, ValueOrd, Writer, ord::iter_cmp, ord::iter_cmp_owned,
};
use core::cmp::Ordering;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(any(feature = "alloc", feature = "heapless"))]
use core::slice;

/// ASN.1 `SET OF` backed by an array.
///
/// This type implements an append-only `SET OF` type which is stack-based
/// and does not depend on `alloc` support.
// TODO(tarcieri): use `ArrayVec` when/if it's merged into `core` (rust-lang/rfcs#3316)
#[cfg(feature = "heapless")]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SetOf<T, const N: usize>
where
    T: DerOrd,
{
    inner: heapless::Vec<T, N>,
}

// Inner reference of a SetOfRef
//
// An internal reference can either be bytes when constructed during decoding
// or a slice of items of the generic type T.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum InnerRef<'a, T> {
    BytesRef(&'a [u8], usize),
    ObjectsRef(&'a [T]),
}

/// ASN.1 `SET OF` with a reference to an array.
///
/// This type implements a viewer in a `SET OF` type
/// and does not depend on `alloc` support.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SetOfRef<'a, T>
where
    T: DerOrd,
{
    inner: InnerRef<'a, T>,
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> SetOf<T, N>
where
    T: DerOrd,
{
    /// Create a new [`SetOf`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: heapless::Vec::default(),
        }
    }

    /// Insert an item into this [`SetOf`].
    ///
    /// # Errors
    /// If there's a sorting error.
    pub fn insert(&mut self, item: T) -> Result<(), Error> {
        self.try_push(item)?;
        der_sort(self.inner.as_mut())
    }

    /// Insert an item into this [`SetOf`].
    ///
    /// Items MUST be added in lexicographical order according to the [`DerOrd`] impl on `T`.
    ///
    /// # Errors
    /// If items are added out-of-order or there isn't sufficient space.
    pub fn insert_ordered(&mut self, item: T) -> Result<(), Error> {
        // Ensure set elements are lexicographically ordered
        if let Some(last) = self.inner.last() {
            check_der_ordering(last, &item)?;
        }

        self.try_push(item)
    }

    /// Borrow the elements of this [`SetOf`] as a slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Get the nth element from this [`SetOf`].
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Extract the inner `heapless::Vec`.
    pub fn into_inner(self) -> heapless::Vec<T, N> {
        self.inner
    }

    /// Iterate over the elements of this [`SetOf`].
    pub fn iter(&self) -> SetOfIter<'_, T> {
        SetOfIter {
            inner: self.inner.iter(),
        }
    }

    /// Is this [`SetOf`] empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Number of elements in this [`SetOf`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Attempt to push an element onto the [`SetOf`].
    ///
    /// Does not perform ordering or uniqueness checks.
    fn try_push(&mut self, item: T) -> Result<(), Error> {
        self.inner
            .push(item)
            .map_err(|_| ErrorKind::Overlength.into())
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> AsRef<[T]> for SetOf<T, N>
where
    T: DerOrd,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T> SetOfRef<'a, T>
where
    T: Decode<'a> + 'a,
    T: Clone + DerOrd,
{
    /// Creates a [`SetOfRef`] by parsing the *contents* of a DER-encoded `SET OF` —
    /// that is, the raw bytes after the tag and length bytes have been stripped.
    fn from_bytes(v: &'a [u8]) -> Result<Self, T::Error> {
        // Cache len for .iter() calls
        let mut iter_len = 0;

        // Make sure we can decode valid objects from the bytes
        let mut reader = SliceReader::new(v)?;

        // Remember last item to check ordering
        let mut last_item: Option<T> = None;

        while !reader.is_finished() {
            let item = T::decode(&mut reader)?;

            if let Some(last_item) = last_item {
                check_der_ordering(&last_item, &item)?;
            }
            last_item = Some(item);
            iter_len += 1;
        }

        // Generate the set as a byte reference
        Ok(Self {
            inner: InnerRef::BytesRef(v, iter_len),
        })
    }

    /// Get the nth element from this [`SetOfRef`].
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Decode<'a> + 'a,
        T: Clone,
    {
        self.iter().nth(index)
    }

    /// Iterate over the elements of this [`SetOfRef`].
    ///
    /// # Panics
    ///
    /// Panics if the inner byte slice contains invalid data that cannot be
    /// parsed by [`SliceReader`].
    #[must_use]
    pub fn iter(&self) -> SetOfRefIter<'a, T>
    where
        T: Decode<'a> + 'a,
    {
        match self.inner {
            InnerRef::BytesRef(inner, length) => SetOfRefIter {
                inner: InnerIterRef::<'a, T>::BytesRef(
                    SliceReader::new(inner).expect("Invalid data"),
                ),
                length,
            },
            InnerRef::ObjectsRef(inner) => SetOfRefIter {
                inner: InnerIterRef::<'a, T>::ObjectsRef(inner),
                length: inner.len(),
            },
        }
    }

    /// Is this [`SetOfRef`] empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self.inner {
            InnerRef::BytesRef(inner, _) => inner.is_empty(),
            InnerRef::ObjectsRef(inner) => inner.is_empty(),
        }
    }

    /// Number of elements in this [`SetOfRef`].
    #[must_use]
    pub fn len(&self) -> usize
    where
        T: Decode<'a> + 'a,
        T: Clone,
    {
        self.iter().len()
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> Default for SetOf<T, N>
where
    T: DerOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "heapless")]
impl<'a, T, const N: usize> DecodeValue<'a> for SetOf<T, N>
where
    T: Decode<'a> + DerOrd,
{
    type Error = T::Error;

    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> Result<Self, Self::Error> {
        let mut result = Self::new();

        while !reader.is_finished() {
            result.try_push(T::decode(reader)?)?;
        }

        if reader.encoding_rules().is_der() {
            // Ensure elements of the `SetOf` are sorted and will serialize as valid DER
            der_sort(result.inner.as_mut())?;
        }

        Ok(result)
    }
}

impl<'a, T> DecodeValue<'a> for SetOfRef<'a, T>
where
    T: Clone,
    T: Decode<'a> + DerOrd,
{
    type Error = T::Error;

    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self, Self::Error> {
        let inner_slice: &'a [u8] = reader.read_slice(header.length())?;
        SetOfRef::<'a, T>::from_bytes(inner_slice)
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> EncodeValue for SetOf<T, N>
where
    T: Encode + DerOrd,
{
    fn value_len(&self) -> Result<Length, Error> {
        self.iter()
            .try_fold(Length::ZERO, |len, elem| len + elem.encoded_len()?)
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<(), Error> {
        for elem in self.iter() {
            elem.encode(writer)?;
        }

        Ok(())
    }
}

impl<'a, T> EncodeValue for SetOfRef<'a, T>
where
    T: Decode<'a> + Encode + DerOrd,
    T: Clone,
{
    fn value_len(&self) -> Result<Length, Error> {
        self.iter()
            .try_fold(Length::ZERO, |len, elem| len + elem.encoded_len()?)
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<(), Error> {
        for elem in self.iter() {
            elem.encode(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> FixedTag for SetOf<T, N>
where
    T: DerOrd,
{
    const TAG: Tag = Tag::Set;
}

impl<'a, T> FixedTag for SetOfRef<'a, T>
where
    T: DerOrd,
{
    const TAG: Tag = Tag::Set;
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> TryFrom<[T; N]> for SetOf<T, N>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(mut arr: [T; N]) -> Result<SetOf<T, N>, Error> {
        der_sort(&mut arr)?;

        let mut result = SetOf::new();

        for elem in arr {
            result.insert_ordered(elem)?;
        }

        Ok(result)
    }
}

impl<'a, T> TryFrom<&'a [T]> for SetOfRef<'a, T>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(arr: &'a [T]) -> Result<SetOfRef<'a, T>, Error> {
        arr.iter()
            .is_sorted_by(|&a, &b| check_der_ordering(a, b).is_ok())
            .then_some(SetOfRef {
                inner: InnerRef::ObjectsRef(arr),
            })
            .ok_or_else(|| Error::from_kind(ErrorKind::SetOrdering))
    }
}

impl<'a, T> From<&SetOfRef<'a, T>> for SetOfRef<'a, T>
where
    T: Clone + DerOrd,
{
    fn from(value: &SetOfRef<'a, T>) -> SetOfRef<'a, T> {
        value.clone()
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> ValueOrd for SetOf<T, N>
where
    T: DerOrd,
{
    fn value_cmp(&self, other: &Self) -> Result<Ordering, Error> {
        iter_cmp(self.iter(), other.iter())
    }
}

impl<'a, T> ValueOrd for SetOfRef<'a, T>
where
    T: Decode<'a> + DerOrd + 'a,
    T: Clone,
{
    fn value_cmp(&self, other: &Self) -> Result<Ordering, Error> {
        iter_cmp_owned(self.iter(), other.iter())
    }
}

/// Iterator over the elements of an [`SetOf`].
#[derive(Clone, Debug)]
pub struct SetOfIter<'a, T> {
    /// Inner iterator.
    inner: slice::Iter<'a, T>,
}

impl<'a, T: 'a> Iterator for SetOfIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: 'a> ExactSizeIterator for SetOfIter<'a, T> {}

// Inner reference of a SetOfRefIter
//
// An internal reference can either be a slice reader when constructed during decoding
// or a slice of items of the generic type T.
#[derive(Clone, Debug)]
enum InnerIterRef<'a, T> {
    BytesRef(SliceReader<'a>),
    ObjectsRef(&'a [T]),
}

/// Iterator over the elements of an [`SetOfRef`].
#[derive(Clone, Debug)]
pub struct SetOfRefIter<'a, T>
where
    T: Decode<'a>,
{
    /// Inner iterator.
    inner: InnerIterRef<'a, T>,
    length: usize,
}

impl<'a, T> Iterator for SetOfRefIter<'a, T>
where
    T: Decode<'a> + 'a,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match &mut self.inner {
            InnerIterRef::BytesRef(inner_reader) => {
                if inner_reader.is_finished() {
                    return None;
                }

                let next_val = T::decode(inner_reader).ok()?;
                self.length -= 1;
                Some(next_val)
            }
            InnerIterRef::ObjectsRef(inner_slice) => {
                let next_val = inner_slice.first()?;
                self.inner = InnerIterRef::ObjectsRef(&inner_slice[1..]);
                self.length -= 1;

                Some(next_val.clone())
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<'a, T> ExactSizeIterator for SetOfRefIter<'a, T>
where
    T: Decode<'a> + 'a,
    T: Clone,
{
}

/// ASN.1 `SET OF` backed by a [`Vec`].
///
/// This type implements an append-only `SET OF` type which is heap-backed
/// and depends on `alloc` support.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SetOfVec<T>
where
    T: DerOrd,
{
    inner: Vec<T>,
}

#[cfg(feature = "alloc")]
impl<T: DerOrd> Default for SetOfVec<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> SetOfVec<T>
where
    T: DerOrd,
{
    /// Create a new [`SetOfVec`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Vec::default(),
        }
    }

    /// Create a new [`SetOfVec`] from the given iterator.
    ///
    /// Note: this is an inherent method instead of an impl of the [`FromIterator`] trait in order
    /// to be fallible.
    ///
    /// # Errors
    /// If a sorting error occurred.
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<I>(iter: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
    {
        Vec::from_iter(iter).try_into()
    }

    /// Extend a [`SetOfVec`] using an iterator.
    ///
    /// Note: this is an inherent method instead of an impl of the [`Extend`] trait in order to
    /// be fallible.
    ///
    /// # Errors
    /// If a sorting error occurred.
    pub fn extend<I>(&mut self, iter: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter);
        der_sort(&mut self.inner)
    }

    /// Insert an item into this [`SetOfVec`]. Must be unique.
    ///
    /// # Errors
    /// If a sorting error occurred.
    pub fn insert(&mut self, item: T) -> Result<(), Error> {
        self.inner.push(item);
        der_sort(&mut self.inner)
    }

    /// Insert an item into this [`SetOfVec`]. Must be unique.
    ///
    /// Items MUST be added in lexicographical order according to the [`DerOrd`] impl on `T`.
    ///
    /// # Errors
    /// If a sorting error occurred.
    pub fn insert_ordered(&mut self, item: T) -> Result<(), Error> {
        // Ensure set elements are lexicographically ordered
        if let Some(last) = self.inner.last() {
            check_der_ordering(last, &item)?;
        }

        self.inner.push(item);
        Ok(())
    }

    /// Borrow the elements of this [`SetOfVec`] as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Get the nth element from this [`SetOfVec`].
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Convert this [`SetOfVec`] into the inner [`Vec`].
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    /// Iterate over the elements of this [`SetOfVec`].
    #[must_use]
    pub fn iter(&self) -> SetOfIter<'_, T> {
        SetOfIter {
            inner: self.inner.iter(),
        }
    }

    /// Is this [`SetOfVec`] empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Number of elements in this [`SetOfVec`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(feature = "alloc")]
impl<T> AsRef<[T]> for SetOfVec<T>
where
    T: DerOrd,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> DecodeValue<'a> for SetOfVec<T>
where
    T: Decode<'a> + DerOrd,
{
    type Error = T::Error;

    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> Result<Self, Self::Error> {
        let mut inner = Vec::new();

        while !reader.is_finished() {
            inner.push(T::decode(reader)?);
        }

        if reader.encoding_rules().is_der() {
            der_sort(inner.as_mut())?;
        }

        Ok(Self { inner })
    }
}

#[cfg(feature = "alloc")]
impl<T> EncodeValue for SetOfVec<T>
where
    T: Encode + DerOrd,
{
    fn value_len(&self) -> Result<Length, Error> {
        self.iter()
            .try_fold(Length::ZERO, |len, elem| len + elem.encoded_len()?)
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<(), Error> {
        for elem in self.iter() {
            elem.encode(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<T> FixedTag for SetOfVec<T>
where
    T: DerOrd,
{
    const TAG: Tag = Tag::Set;
}

#[cfg(feature = "alloc")]
impl<T> From<SetOfVec<T>> for Vec<T>
where
    T: DerOrd,
{
    fn from(set: SetOfVec<T>) -> Vec<T> {
        set.into_vec()
    }
}

#[cfg(feature = "alloc")]
impl<T> TryFrom<Vec<T>> for SetOfVec<T>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(mut vec: Vec<T>) -> Result<SetOfVec<T>, Error> {
        // TODO(tarcieri): use `[T]::sort_by` here?
        der_sort(vec.as_mut_slice())?;
        Ok(SetOfVec { inner: vec })
    }
}

#[cfg(feature = "alloc")]
impl<T, const N: usize> TryFrom<[T; N]> for SetOfVec<T>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(arr: [T; N]) -> Result<SetOfVec<T>, Error> {
        Vec::from(arr).try_into()
    }
}

#[cfg(feature = "alloc")]
impl<T> ValueOrd for SetOfVec<T>
where
    T: DerOrd,
{
    fn value_cmp(&self, other: &Self) -> Result<Ordering, Error> {
        iter_cmp(self.iter(), other.iter())
    }
}

// Implement by hand because custom derive would create invalid values.
// Use the conversion from Vec to create a valid value.
#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for SetOfVec<T>
where
    T: DerOrd + arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Self::try_from(u.arbitrary_iter()?.collect::<Result<Vec<_>, _>>()?)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}

/// Ensure set elements are lexicographically ordered using [`DerOrd`].
fn check_der_ordering<T: DerOrd>(a: &T, b: &T) -> Result<(), Error> {
    if a.der_cmp(b)? == Ordering::Greater {
        return Err(ErrorKind::SetOrdering.into());
    }

    Ok(())
}

/// Sort a mut slice according to its [`DerOrd`], returning any errors which
/// might occur during the comparison.
///
/// Uses [`slice::sort_unstable_by`] (an O(n log n) introsort) rather than the
/// stable [`slice::sort_by`] because the latter requires `alloc`, while this
/// function must also work on heapless `no_std` targets where only `core` is
/// available. Sorting unstably is fine here: two elements that compare
/// [`Ordering::Equal`] under [`DerOrd`] have identical DER encodings, so their
/// relative order does not affect the serialized output.
///
/// A previous implementation used a hand-rolled insertion sort. On adversarial
/// reverse-sorted input that degrades to O(n^2), which let a crafted `SET OF`
/// turn a single decode into a quadratic-time denial of service (see
/// <https://github.com/RustCrypto/formats/issues/2319>).
///
/// `DerOrd::der_cmp` is fallible. Since the standard sort comparator must
/// return [`Ordering`] rather than a `Result`, the first comparison error is
/// captured and returned after the sort completes; on error the slice is left
/// in an unspecified (but valid) order.
fn der_sort<T: DerOrd>(slice: &mut [T]) -> Result<(), Error> {
    let mut first_err: Option<Error> = None;

    slice.sort_unstable_by(|a, b| {
        a.der_cmp(b).unwrap_or_else(|err| {
            first_err.get_or_insert(err);
            Ordering::Equal
        })
    });

    first_err.map_or(Ok(()), Err)
}

#[cfg(feature = "alloc")]
mod allocating {
    use super::*;
    use crate::referenced::*;

    impl<'a, T> RefToOwned<'a> for SetOfRef<'a, T>
    where
        T: Decode<'a> + EncodeValue + 'a,
        T: DerOrd + FixedTag,
        T: Clone,
    {
        type Owned = SetOfVec<T>;
        fn ref_to_owned(&self) -> Self::Owned {
            SetOfVec::from_iter(self.iter()).expect("SetOfVec: Could not sort inner slice")
        }
    }

    impl<T> OwnedToRef for SetOfVec<T>
    where
        T: Encode,
        T: DerOrd,
    {
        type Borrowed<'a>
            = SetOfRef<'a, T>
        where
            T: 'a;

        fn owned_to_ref(&self) -> Self::Borrowed<'_> {
            SetOfRef::<T>::try_from(self.inner.as_slice()).expect("Unsorted slice")
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {

    use crate::{ErrorKind, Tag, asn1::OctetStringRef};
    #[cfg(feature = "alloc")]
    use {
        super::SetOfVec,
        crate::{Decode, Encode, EncodeValue, SliceWriter},
        alloc::vec,
    };

    #[cfg(feature = "heapless")]
    use super::SetOf;
    #[cfg(any(feature = "alloc", feature = "heapless"))]
    use {super::SetOfRef, crate::DerOrd};

    #[cfg(feature = "heapless")]
    #[test]
    fn setof_insert() {
        let mut setof = SetOf::<u8, 10>::new();
        setof.insert(42).unwrap();
        assert_eq!(setof.len(), 1);
        assert_eq!(*setof.iter().next().unwrap(), 42);
    }

    #[cfg(feature = "heapless")]
    #[test]
    fn setof_insert_duplicate() {
        let mut setof = SetOf::<u8, 10>::new();
        setof.insert(42).unwrap();
        assert_eq!(setof.len(), 1);

        setof.insert(42).unwrap();

        let mut iter = setof.iter();

        assert_eq!(setof.len(), 2);
        assert_eq!(*iter.next().unwrap(), 42);
        assert_eq!(*iter.next().unwrap(), 42);
    }

    #[cfg(feature = "heapless")]
    #[test]
    fn setof_tryfrom_array() {
        let arr = [3u16, 2, 1, 65535, 0];
        let set = SetOf::try_from(arr).unwrap();
        assert!(set.iter().copied().eq([0, 1, 2, 3, 65535]));
    }

    #[cfg(feature = "heapless")]
    #[test]
    fn setof_valueord_value_cmp() {
        use core::cmp::Ordering;

        let arr1 = [3u16, 2, 1, 5, 0];
        let arr2 = [3u16, 2, 1, 4, 0];
        let set1 = SetOf::try_from(arr1).unwrap();
        let set2 = SetOf::try_from(arr2).unwrap();
        assert_eq!(set1.der_cmp(&set2), Ok(Ordering::Greater));
    }

    #[test]
    fn setofref_tryfrom_array() {
        let arr = [0u16, 1, 2, 3, 65535];
        let set = SetOfRef::try_from(arr.as_ref()).unwrap();
        assert!(set.iter().eq([0, 1, 2, 3, 65535]));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofref_tryfrom_der() {
        let arr = SetOfVec::try_from([0u16, 1, 2, 3, 65535])
            .unwrap()
            .to_der()
            .unwrap();
        let set = SetOfRef::<u16>::from_der(arr.as_ref()).unwrap();
        assert!(set.iter().eq([0, 1, 2, 3, 65535]));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofref_tryfrom_bytes() {
        let arr = SetOfVec::try_from([0u16, 1, 2, 3, 65535]).unwrap();

        let mut encoded = vec![0u8; arr.value_len().unwrap().try_into().unwrap()];
        let mut writer = SliceWriter::new(&mut encoded);
        arr.encode_value(&mut writer).unwrap();

        let decoded = SetOfRef::<u16>::from_bytes(writer.finish().unwrap()).unwrap();

        assert!(decoded.iter().eq([0, 1, 2, 3, 65535]));
    }

    #[test]
    fn setofref_tryfrom_array_reject_unsorted() {
        let arr = [3u16, 2, 1, 65535, 0];
        let err = SetOfRef::try_from(arr.as_ref()).err().unwrap();
        assert_eq!(err.kind(), ErrorKind::SetOrdering);
    }

    #[test]
    fn setofref_tryfrom_array_allow_duplicates() {
        let arr = [1u16, 1];
        let set = SetOfRef::try_from(arr.as_ref()).unwrap();
        assert!(set.iter().eq([1, 1]));
    }

    #[test]
    fn setofref_valueord_value_cmp() {
        use core::cmp::Ordering;

        let arr1 = [0u16, 1, 2, 3, 5];
        let arr2 = [0u16, 1, 2, 3, 4];
        let set1 = SetOfRef::try_from(arr1.as_ref()).unwrap();
        let set2 = SetOfRef::try_from(arr2.as_ref()).unwrap();
        assert_eq!(set1.der_cmp(&set2), Ok(Ordering::Greater));
    }

    #[test]
    fn setofref_from_bytes_should_return_decode_error() {
        let inner_slice = [0x04, 0x00, 0x05, 0x00];
        let result = SetOfRef::<'_, &OctetStringRef>::from_bytes(&inner_slice);

        assert_eq!(
            result,
            Err(ErrorKind::TagUnexpected {
                expected: Some(Tag::OctetString),
                actual: Tag::Null
            }
            .into())
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_insert() {
        let mut setof = SetOfVec::new();
        setof.insert(42).unwrap();
        assert_eq!(setof.len(), 1);

        setof.insert(46).unwrap();

        let mut iter = setof.iter();

        assert_eq!(setof.len(), 2);
        assert_eq!(*iter.next().unwrap(), 42);
        assert_eq!(*iter.next().unwrap(), 46);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_tryfrom_array() {
        let arr = [3u16, 2, 1, 65535, 0];
        let set = SetOfVec::try_from(arr).unwrap();
        assert_eq!(set.as_ref(), &[0, 1, 2, 3, 65535]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_tryfrom_vec() {
        let vec = vec![3u16, 2, 1, 65535, 0];
        let set = SetOfVec::try_from(vec).unwrap();
        assert_eq!(set.as_ref(), &[0, 1, 2, 3, 65535]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_tryfrom_vec_allow_duplicates() {
        let vec = vec![1u16, 1];
        let set = SetOfVec::try_from(vec).unwrap();
        assert_eq!(set.as_ref(), &[1, 1]);
    }

    // Regression tests for #2319: `der_sort` was a hand-rolled O(n^2) insertion
    // sort, which a crafted reverse-sorted `SET OF` could exploit for a
    // quadratic-time denial of service. The fix swaps in `sort_unstable_by`
    // (O(n log n)). These tests pin the externally observable contract so the
    // algorithm swap cannot silently change behavior: the resulting order, the
    // DER `SET OF` ordering invariant, and duplicate preservation.

    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    /// Reference implementation: the original insertion sort that `der_sort`
    /// replaced. Used to confirm the new sort yields a byte-identical ordering.
    #[cfg(feature = "alloc")]
    fn insertion_sort_reference<T: DerOrd>(slice: &mut [T]) {
        for i in 0..slice.len() {
            let mut j = i;
            while j > 0 {
                if slice[j - 1].der_cmp(&slice[j]).unwrap() == core::cmp::Ordering::Greater {
                    slice.swap(j - 1, j);
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }

    /// Assert the slice obeys the DER `SET OF` ordering invariant: each element
    /// is less-than-or-equal to its successor under `DerOrd`. Equal neighbors
    /// (duplicates / encoding ties) are allowed, matching the decoder.
    #[cfg(feature = "alloc")]
    fn assert_der_sorted<T: DerOrd>(slice: &[T]) {
        for pair in slice.windows(2) {
            assert_ne!(
                pair[0].der_cmp(&pair[1]).unwrap(),
                core::cmp::Ordering::Greater,
                "der_sort left an out-of-order pair"
            );
        }
    }

    /// A small deterministic LCG so the test is reproducible without a `rand`
    /// dependency (the crate has none in dev-deps for this path).
    #[cfg(feature = "alloc")]
    fn lcg_next(state: &mut u64) -> u16 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        // Intentionally take the low 16 bits of the high word as the output;
        // the mask makes the truncation explicit rather than a lossy `as` cast.
        ((*state >> 33) & 0xFFFF) as u16
    }

    /// The fix must not change which ordering `der_sort` produces. Cross-check
    /// it against the original insertion sort on many inputs: fully randomized
    /// (with intentional duplicates from a narrow value range), the adversarial
    /// reverse-sorted case from the issue, already-sorted, and constant.
    #[cfg(feature = "alloc")]
    #[test]
    fn der_sort_matches_reference_ordering() {
        let mut rng_state = 0x2319_u64;

        // Randomized inputs of varied sizes. The narrow value range forces
        // frequent duplicates so duplicate handling is exercised.
        for &len in &[0usize, 1, 2, 5, 17, 64, 257, 1000] {
            let original: Vec<u16> = (0..len).map(|_| lcg_next(&mut rng_state) % 50).collect();

            let mut via_new = original.clone();
            super::der_sort(&mut via_new).unwrap();

            let mut via_reference = original.clone();
            insertion_sort_reference(&mut via_reference);

            assert_eq!(via_new, via_reference, "ordering diverged at len={len}");
            assert_der_sorted(&via_new);
            // Multiset is preserved (nothing dropped or duplicated by the sort).
            let mut a = original.clone();
            let mut b = via_new.clone();
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b, "der_sort changed the multiset at len={len}");
        }

        // Adversarial worst case for insertion sort: strictly reverse-sorted.
        let reversed: Vec<u16> = (0..2000u16).rev().collect();
        let mut via_new = reversed.clone();
        super::der_sort(&mut via_new).unwrap();
        let mut via_reference = reversed;
        insertion_sort_reference(&mut via_reference);
        assert_eq!(via_new, via_reference);
        assert_der_sorted(&via_new);
        assert!(via_new.iter().copied().eq(0..2000u16));
    }

    /// Decoding a large reverse-sorted DER `SET OF` must succeed, sort
    /// correctly, and re-encode to canonical (sorted) DER. This is the decode
    /// path that the `DoS` in #2319 targeted. With the O(n^2) sort this input
    /// took seconds; with O(n log n) it is effectively instant, but the
    /// assertion here is on correctness, not timing.
    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_decodes_large_reverse_sorted_der() {
        // Build non-canonical (reverse-sorted) DER for a SET OF INTEGER.
        let n = 4000u16;
        let elements: Vec<u16> = (0..n).rev().collect();

        let mut encoded = elements.to_der().unwrap();
        // Trick: change SEQUENCE tag 0x30 to SET 0x31
        encoded[0] = 0x31;

        // Decoder accepts the non-canonical input and sorts it at decode time.
        let set = SetOfVec::<u16>::from_der(&encoded).unwrap();
        assert_eq!(set.len(), n as usize);
        assert!(set.iter().copied().eq(0..n));
        assert_der_sorted(set.as_ref());

        // Re-encoding yields canonical (sorted) DER, distinct from the input.
        let reencoded = set.to_der().unwrap();
        assert_ne!(reencoded, encoded, "expected canonicalization on re-encode");
        // And the canonical form round-trips unchanged.
        let reparsed = SetOfVec::<u16>::from_der(&reencoded).unwrap();
        assert!(reparsed.iter().copied().eq(0..n));
    }

    /// Duplicate elements (allowed since #2272) must survive the sort: equal
    /// encodings are kept, not deduplicated, and end up adjacent.
    #[cfg(feature = "alloc")]
    #[test]
    fn der_sort_preserves_duplicates() {
        let vec = vec![5u16, 1, 5, 1, 3, 1, 5];
        let set = SetOfVec::try_from(vec).unwrap();
        assert_eq!(set.as_ref(), &[1, 1, 1, 3, 5, 5, 5]);
        assert_eq!(set.len(), 7);
    }
}
