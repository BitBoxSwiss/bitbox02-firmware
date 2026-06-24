//! Paths

use core::{
    cmp::Ordering,
    convert::TryFrom,
    ffi::{c_char, CStr},
    fmt,
    iter::FusedIterator,
    ops, ptr, slice, str,
};

use crate::path;

/// A path
///
/// Paths must be null terminated ASCII strings with at most [`PathBuf::MAX_SIZE`][] bytes (not
/// including the trailing null).
// Invariants:
// 1. inner.to_bytes().is_ascii()
// 2. inner.to_bytes().len() <= PathBuf::MAX_SIZE
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Path {
    inner: CStr,
}

impl Path {
    /// Checks two paths for equality.
    ///
    /// This provides an easy way to check paths in a const context.
    ///
    /// # Example
    ///
    /// ```
    /// # use littlefs2_core::{Path, path};
    /// const fn check(path: &Path) -> bool {
    ///     !path.const_eq(path!("forbidden-path"))
    /// }
    ///
    /// assert!(check(path!("allowed-path")));
    /// assert!(!check(path!("forbidden-path")));
    /// ```
    pub const fn const_eq(&self, path: &Path) -> bool {
        let a = self.inner.to_bytes();
        let b = path.inner.to_bytes();

        if a.len() != b.len() {
            return false;
        }

        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Compare the path using their string representation
    /// This comarison function as would be expected for a `String` type.
    ///
    /// <div class="warning">
    ///   This ordering does not match the ordering obsvered when iterating over a directory.
    ///
    ///   See <a href="#method.cmp_lfs">cmp_lfs</a> and <a href = "https://github.com/littlefs-project/littlefs/issues/923">littlefs#923</a>.
    /// </div>
    ///
    /// ```
    ///# use std::cmp::Ordering;
    ///# use littlefs2_core::path;
    /// assert_eq!(path!("some_path_a").cmp_str(path!("some_path_b")), Ordering::Less);
    /// assert_eq!(path!("some_path_b").cmp_str(path!("some_path_a")), Ordering::Greater);
    /// assert_eq!(path!("some_path").cmp_str(path!("some_path_a")), Ordering::Less);
    /// assert_eq!(path!("some_path").cmp_str(path!("some_path_b")), Ordering::Less);
    /// assert_eq!(path!("some_path").cmp_str(path!("some_path")), Ordering::Equal);
    ///```
    pub fn cmp_str(&self, other: &Path) -> Ordering {
        self.inner.cmp(&other.inner)
    }

    /// Compare the path using their string representation
    ///
    /// This comparison function matches the iteration order of `littlefs` when iterating over directory.
    /// For more information, see [littlefs#923](https://github.com/littlefs-project/littlefs/issues/923)
    ///
    /// ```
    ///# use std::cmp::Ordering;
    ///# use littlefs2_core::path;
    /// assert_eq!(path!("some_path_a").cmp_lfs(path!("some_path_b")), Ordering::Less);
    /// assert_eq!(path!("some_path_b").cmp_lfs(path!("some_path_a")), Ordering::Greater);
    /// assert_eq!(path!("some_path").cmp_lfs(path!("some_path_a")), Ordering::Greater);
    /// assert_eq!(path!("some_path").cmp_lfs(path!("some_path_b")), Ordering::Greater);
    /// assert_eq!(path!("some_path_a").cmp_lfs(path!("some_path")), Ordering::Less);
    /// assert_eq!(path!("some_path_b").cmp_lfs(path!("some_path")), Ordering::Less);
    /// assert_eq!(path!("some_path").cmp_lfs(path!("some_path")), Ordering::Equal);
    ///```
    pub fn cmp_lfs(&self, other: &Path) -> Ordering {
        let this = self.inner.to_bytes();
        let other = other.inner.to_bytes();

        let min_len = this.len().min(other.len());

        match this[0..min_len].cmp(&other[0..min_len]) {
            // if they have a clear ordering, return this ordering
            Ordering::Less => Ordering::Less,
            // if they have a clear ordering, return this ordering
            Ordering::Greater => Ordering::Greater,
            // If one is a prefix of the other, the longest on is the first
            Ordering::Equal => other.len().cmp(&this.len()),
        }
    }
}

/// Iterator over the ancestors of a Path
///
/// See documentation for [`Path::ancestors`][]
pub struct Ancestors<'a> {
    path: &'a str,
}

impl Iterator for Ancestors<'_> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        if self.path.is_empty() {
            return None;
        } else if self.path == "/" {
            self.path = "";
            return Some(path!("/").into());
        }

        let item = self.path;

        let Some((rem, item_name)) = self.path.rsplit_once('/') else {
            self.path = "";
            return item.try_into().ok();
        };

        if self.path.starts_with('/') && rem.is_empty() {
            self.path = "/";
        } else {
            self.path = rem;
        }

        // Case of a path ending with a trailing `/`
        if item_name.is_empty() {
            self.next();
        }
        item.try_into().ok()
    }
}

impl FusedIterator for Ancestors<'_> {}

/// Iterator over the components of a Path
///
/// See documentation for [`Path::iter`][]
pub struct Iter<'a> {
    path: &'a str,
}

impl Iterator for Iter<'_> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        if self.path.is_empty() {
            return None;
        }
        if self.path.starts_with('/') {
            self.path = &self.path[1..];
            return Some(path!("/").into());
        }

        let Some((path, rem)) = self.path.split_once('/') else {
            let ret_val = self.path.try_into().ok();
            self.path = "";
            return ret_val;
        };

        self.path = rem;
        path.try_into().ok()
    }
}

impl Path {
    /// Return true if the path is empty
    ///
    /// ```rust
    ///# use littlefs2_core::path;
    ///
    /// assert!(path!("").is_empty());
    /// assert!(!path!("something").is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.inner.to_bytes().is_empty()
    }

    /// Get the name of the file this path points to if it points to one
    ///
    /// ```
    ///# use littlefs2_core::path;
    /// let path = path!("/some/path/file.extension");
    /// assert_eq!(path.file_name(), Some(path!("file.extension")));
    ///
    /// let path = path!("/");
    /// assert_eq!(path.file_name(), None);
    ///
    /// let path = path!("");
    /// assert_eq!(path.file_name(), None);
    ///
    /// let path = path!("/some/path/file.extension/");
    /// assert_eq!(path.file_name(), None);
    /// ```
    pub fn file_name(&self) -> Option<&Path> {
        if self.is_empty() {
            return None;
        }

        let this = self.as_str_ref_with_trailing_nul();
        match this.rsplit_once('/') {
            None | Some((_, "\x00")) => None,
            Some((_, path)) => {
                debug_assert!(path.ends_with('\x00'));
                unsafe {
                    let cstr = CStr::from_bytes_with_nul_unchecked(path.as_bytes());
                    Some(Path::from_cstr_unchecked(cstr))
                }
            }
        }
    }

    /// Iterate over the ancestors of the path
    ///
    /// ```
    ///# use littlefs2_core::path;
    /// let path = path!("/some/path/file.extension");
    /// let mut ancestors = path.ancestors();
    /// assert_eq!(&*ancestors.next().unwrap(), path!("/some/path/file.extension"));
    /// assert_eq!(&*ancestors.next().unwrap(), path!("/some/path"));
    /// assert_eq!(&*ancestors.next().unwrap(), path!("/some"));
    /// assert_eq!(&*ancestors.next().unwrap(), path!("/"));
    /// assert!(ancestors.next().is_none());
    /// ```
    pub fn ancestors(&self) -> Ancestors<'_> {
        Ancestors {
            path: self.as_str(),
        }
    }

    /// Iterate over the components of the path
    ///
    /// ```
    ///# use littlefs2_core::path;
    /// let path = path!("/some/path/file.extension");
    /// let mut iter = path.iter();
    /// assert_eq!(&*iter.next().unwrap(), path!("/"));
    /// assert_eq!(&*iter.next().unwrap(), path!("some"));
    /// assert_eq!(&*iter.next().unwrap(), path!("path"));
    /// assert_eq!(&*iter.next().unwrap(), path!("file.extension"));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            path: self.as_str(),
        }
    }

    /// Creates a path from a string.
    ///
    /// The string must only consist of ASCII characters.  The last character must be null.  It
    /// must contain at most [`PathBuf::MAX_SIZE`][] bytes, not including the trailing null.  If
    /// these conditions are not met, this function returns an error.
    pub const fn from_str_with_nul(s: &str) -> Result<&Self> {
        Self::from_bytes_with_nul(s.as_bytes())
    }

    /// Creates a path from a byte buffer.
    ///
    /// The byte buffer must only consist of ASCII characters.  The last character must be null.
    /// It must contain at most [`PathBuf::MAX_SIZE`][] bytes, not including the trailing null.  If
    /// these conditions are not met, this function returns an error.
    pub const fn from_bytes_with_nul(bytes: &[u8]) -> Result<&Self> {
        match CStr::from_bytes_with_nul(bytes) {
            Ok(cstr) => Self::from_cstr(cstr),
            Err(_) => Err(PathError::NotCStr),
        }
    }

    /// Creates a path from a C string.
    ///
    /// The string must only consist of ASCII characters.  It must contain at most
    /// [`PathBuf::MAX_SIZE`][] bytes, not including the trailing null.  If these conditions are
    /// not met, this function returns an error.
    // XXX should we reject empty paths (`""`) here?
    pub const fn from_cstr(cstr: &CStr) -> Result<&Self> {
        let bytes = cstr.to_bytes();
        let n = cstr.to_bytes().len();
        if n > PathBuf::MAX_SIZE {
            Err(PathError::TooLarge)
        } else if bytes.is_ascii() {
            Ok(unsafe { Self::from_cstr_unchecked(cstr) })
        } else {
            Err(PathError::NotAscii)
        }
    }

    /// Creates a path from a C string without checking the invariants.
    ///
    /// # Safety
    /// The string must only consist of ASCII characters.  It must contain at most
    /// [`PathBuf::MAX_SIZE`][] bytes, not including the trailing null.
    pub const unsafe fn from_cstr_unchecked(cstr: &CStr) -> &Self {
        &*(cstr as *const CStr as *const Path)
    }

    /// Returns the inner pointer to this C string.
    pub const fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr()
    }

    /// Creates an owned `PathBuf` with `path` adjoined to `self`.
    pub fn join(&self, path: &Path) -> PathBuf {
        let mut p = PathBuf::from(self);
        p.push(path);
        p
    }

    // helpful for debugging wither the trailing nul is indeed a trailing nul.
    pub const fn as_str_ref_with_trailing_nul(&self) -> &str {
        // SAFETY: ASCII is valid UTF-8
        unsafe { str::from_utf8_unchecked(self.inner.to_bytes_with_nul()) }
    }

    pub const fn as_str(&self) -> &str {
        // SAFETY: ASCII is valid UTF-8
        unsafe { str::from_utf8_unchecked(self.inner.to_bytes()) }
    }

    pub fn parent(&self) -> Option<PathBuf> {
        let rk_path_bytes = self.as_ref().as_bytes();
        match rk_path_bytes.iter().rposition(|x| *x == b'/') {
            Some(0) if rk_path_bytes.len() != 1 => Some(path!("/").into()),
            Some(slash_index) => {
                // if we have a directory that ends with `/`,
                // still need to "go up" one parent
                if slash_index + 1 == rk_path_bytes.len() {
                    PathBuf::try_from(&rk_path_bytes[..slash_index])
                        .ok()?
                        .parent()
                } else {
                    PathBuf::try_from(&rk_path_bytes[..slash_index]).ok()
                }
            }
            None => None,
        }
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // helpful for debugging wither the trailing nul is indeed a trailing nul.
        write!(f, "p{:?}", self.as_str_ref_with_trailing_nul())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'b> TryFrom<&'b [u8]> for &'b Path {
    type Error = PathError;

    fn try_from(bytes: &[u8]) -> Result<&Path> {
        Path::from_bytes_with_nul(bytes)
    }
}

impl PartialEq<str> for Path {
    fn eq(&self, rhs: &str) -> bool {
        self.as_ref() == rhs
    }
}

// without this you need to slice byte string literals (`b"foo\0"[..].try_into()`)
macro_rules! array_impls {
    ($($N:expr),+) => {
        $(
            impl<'b> TryFrom<&'b [u8; $N]> for &'b Path {
                type Error = PathError;

                fn try_from(bytes: &[u8; $N]) -> Result<&Path> {
                    Path::from_bytes_with_nul(&bytes[..])
                }
            }

            impl TryFrom<&[u8; $N]> for PathBuf {
                type Error = PathError;

                fn try_from(bytes: &[u8; $N]) -> Result<Self> {
                    Self::try_from(&bytes[..])
                }
            }

            impl PartialEq<[u8; $N]> for Path {
                fn eq(&self, rhs: &[u8; $N]) -> bool {
                    self.as_ref().as_bytes() == &rhs[..]
                }
            }

        )+
    }
}

array_impls!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

/// An owned, mutable path
///
/// Paths must be null terminated ASCII strings with at most [`PathBuf::MAX_SIZE`][] bytes (not
/// including the trailing null).
// Invariants:
// 1. 0 < len <= PathBuf::MAX_SIZE_PLUS_ONE
// 2. buf[len - 1] == 0
// 3. buf[i].is_ascii() for 0 <= i < len - 1
#[derive(Clone)]
pub struct PathBuf {
    buf: [c_char; PathBuf::MAX_SIZE_PLUS_ONE],
    // NOTE `len` DOES include the final null byte
    len: usize,
}

/// # Safety
/// `s` must point to valid memory; `s` will be treated as a null terminated string
const unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut n = 0;
    while *s != 0 {
        s = s.add(1);
        n += 1;
    }
    n
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBuf {
    pub const MAX_SIZE: usize = 255;
    pub const MAX_SIZE_PLUS_ONE: usize = Self::MAX_SIZE + 1;

    pub const fn new() -> Self {
        Self {
            buf: [0; Self::MAX_SIZE_PLUS_ONE],
            len: 1,
        }
    }

    /// Creates a `PathBuf` from a `Path`.
    ///
    /// This method is a const-friendly version of the `From<&Path>` implementation.  If you don’t
    /// need a const method, prefer `From<&Path>` as it is more idiomatic and more efficient.
    ///
    /// The [`path_buf`][`crate::path_buf`] macro can be used instead to construct a `PathBuf` from
    /// a string literal.
    ///
    /// # Example
    ///
    /// ```        
    /// # use littlefs2_core::{path, path_buf, PathBuf};
    /// const PATH: PathBuf = PathBuf::from_path(path!("test"));
    /// assert_eq!(PATH, path_buf!("test"));
    /// ```
    pub const fn from_path(path: &Path) -> Self {
        let bytes = path.inner.to_bytes();

        let mut buf = [0; Self::MAX_SIZE_PLUS_ONE];
        let len = bytes.len();
        assert!(len < Self::MAX_SIZE_PLUS_ONE);

        let mut i = 0;
        while i < len {
            buf[i] = bytes[i] as _;
            i += 1;
        }

        Self { buf, len: len + 1 }
    }

    pub const fn as_path(&self) -> &Path {
        unsafe {
            let bytes = slice::from_raw_parts(self.buf.as_ptr().cast(), self.len);
            let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
            Path::from_cstr_unchecked(cstr)
        }
    }

    pub const fn as_str(&self) -> &str {
        self.as_path().as_str()
    }

    pub fn clear(&mut self) {
        self.buf = [0; Self::MAX_SIZE_PLUS_ONE];
        self.len = 1;
    }

    /// Creates a from a raw buffer containing a null-terminated ASCII string.
    ///
    /// # Safety
    ///
    /// The buffer must contain only ASCII characters and at least one null byte.
    pub const unsafe fn from_buffer_unchecked(buf: [c_char; Self::MAX_SIZE_PLUS_ONE]) -> Self {
        let len = strlen(buf.as_ptr()) + 1 /* null byte */;
        PathBuf { buf, len }
    }

    /// Extends `self` with `path`
    pub fn push(&mut self, path: &Path) {
        match path.as_ref() {
            // no-operation
            "" => return,

            // `self` becomes `/` (root), to match `std::Path` implementation
            // NOTE(allow) cast is necessary on some architectures (e.g. x86)
            #[allow(clippy::unnecessary_cast)]
            "/" => {
                self.buf[0] = b'/' as c_char;
                self.buf[1] = 0;
                self.len = 2;
                return;
            }
            _ => {}
        }

        let src = path.as_ref().as_bytes();
        let needs_separator = self
            .as_ref()
            .as_bytes()
            .last()
            .map(|byte| *byte != b'/')
            .unwrap_or(false);
        let slen = src.len();
        assert!(
            self.len
                + slen
                + if needs_separator {
                    // b'/'
                    1
                } else {
                    0
                }
                <= Self::MAX_SIZE_PLUS_ONE
        );

        let len = self.len;
        unsafe {
            let mut p = self.buf.as_mut_ptr().cast::<u8>().add(len - 1);
            if needs_separator {
                p.write(b'/');
                p = p.add(1);
                self.len += 1;
            }
            ptr::copy_nonoverlapping(src.as_ptr(), p, slen);
            p.add(slen).write(0); // null byte
            self.len += slen;
        }
    }
}

impl From<&Path> for PathBuf {
    #[inline(never)]
    fn from(path: &Path) -> Self {
        let bytes = path.as_ref().as_bytes();

        let mut buf = [0; Self::MAX_SIZE_PLUS_ONE];
        let len = bytes.len();
        assert!(len <= Self::MAX_SIZE_PLUS_ONE);
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr().cast(), len + 1) }
        Self { buf, len: len + 1 }
    }
}

/// Accepts byte strings, with or without trailing nul.
impl TryFrom<&[u8]> for PathBuf {
    type Error = PathError;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        // NB: This needs to set the final NUL byte, unless it already has one
        // It also checks that there are no inner NUL bytes
        let bytes = if !bytes.is_empty() && bytes[bytes.len() - 1] == b'\0' {
            &bytes[..bytes.len() - 1]
        } else {
            bytes
        };
        if bytes.len() > Self::MAX_SIZE {
            return Err(PathError::TooLarge);
        }
        for byte in bytes {
            if *byte == 0 {
                return Err(PathError::NotCStr);
            }
            if !byte.is_ascii() {
                return Err(PathError::NotAscii);
            }
        }

        let mut buf = [0; Self::MAX_SIZE_PLUS_ONE];
        let len = bytes.len();
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr().cast(), len) }
        Ok(Self { buf, len: len + 1 })
    }
}

/// Accepts strings, with or without trailing nul.
impl TryFrom<&str> for PathBuf {
    type Error = PathError;

    fn try_from(s: &str) -> Result<Self> {
        PathBuf::try_from(s.as_bytes())
    }
}

impl ops::Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.as_path()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for PathBuf {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref().as_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PathBuf {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::marker::PhantomData;

        struct ValueVisitor<'de>(PhantomData<&'de ()>);

        impl<'de> serde::de::Visitor<'de> for ValueVisitor<'de> {
            type Value = PathBuf;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a path buffer")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() > PathBuf::MAX_SIZE {
                    return Err(E::invalid_length(v.len(), &self));
                }
                PathBuf::try_from(v).map_err(|_| E::custom("invalid path buffer"))
            }
        }

        deserializer.deserialize_bytes(ValueVisitor(PhantomData))
    }
}

impl fmt::Debug for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Path as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Path as fmt::Display>::fmt(self, f)
    }
}

impl core::cmp::PartialEq for PathBuf {
    fn eq(&self, other: &Self) -> bool {
        // from cstr_core
        self.as_ref() == other.as_ref()

        // // use cortex_m_semihosting::hprintln;
        // // hprintln!("inside PathBuf PartialEq");
        // // hprintln!("self.len {}, other.len {}", self.len, other.len).ok();
        // // hprintln!("self..len {:?}, other..len {:?}", &self.buf[..self.len], &other.buf[..other.len]).ok();
        // self.len == other.len && self.buf[..self.len - 1] == other.buf[..other.len - 1]
    }
}

impl core::cmp::Eq for PathBuf {}

// use core::cmp::Ordering;

// impl Ord for PathBuf {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.len.cmp(&other.len)
//     }
// }

// impl PartialOrd for PathBuf {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

/// Errors that arise from converting byte buffers into paths
#[derive(Clone, Copy, Debug)]
pub enum PathError {
    /// Byte buffer contains non-ASCII characters
    NotAscii,
    /// Byte buffer is not a C string
    NotCStr,
    /// Byte buffer is too long (longer than [`PathBuf::MAX_SIZE_PLUS_ONE`][])
    TooLarge,
}

type Result<T> = core::result::Result<T, PathError>;

#[cfg(test)]
mod tests {
    use super::{Path, PathBuf};
    use crate::path;

    const EMPTY: &Path = path!("");
    const SLASH: &Path = path!("/");

    #[test]
    fn path_macro() {
        assert_eq!(EMPTY, &*PathBuf::try_from("").unwrap());
        assert_eq!(SLASH, &*PathBuf::try_from("/").unwrap());
    }

    // does not compile:
    // const NON_ASCII: &Path = path!("über");
    // const NULL: &Path = path!("ub\0er");

    #[test]
    fn nul_in_from_str_with_nul() {
        assert!(Path::from_str_with_nul("ub\0er").is_err());
    }

    #[test]
    fn non_ascii_in_from_str_with_nul() {
        assert!(Path::from_str_with_nul("über").is_err());
    }

    #[test]
    fn join() {
        let empty = Path::from_bytes_with_nul(b"\0").unwrap();
        let slash = Path::from_bytes_with_nul(b"/\0").unwrap();
        let a = Path::from_bytes_with_nul(b"a\0").unwrap();
        let b = Path::from_bytes_with_nul(b"b\0").unwrap();

        assert_eq!(empty.join(empty).as_ref(), "");
        assert_eq!(empty.join(slash).as_ref(), "/");
        assert_eq!(empty.join(a).as_ref(), "a");
        assert_eq!(empty.join(b).as_ref(), "b");

        assert_eq!(slash.join(empty).as_ref(), "/");
        assert_eq!(slash.join(slash).as_ref(), "/");
        assert_eq!(slash.join(a).as_ref(), "/a");
        assert_eq!(slash.join(b).as_ref(), "/b");

        assert_eq!(a.join(empty).as_ref(), "a");
        assert_eq!(a.join(slash).as_ref(), "/");
        assert_eq!(a.join(a).as_ref(), "a/a");
        assert_eq!(a.join(b).as_ref(), "a/b");

        assert_eq!(b.join(empty).as_ref(), "b");
        assert_eq!(b.join(slash).as_ref(), "/");
        assert_eq!(b.join(a).as_ref(), "b/a");
        assert_eq!(b.join(b).as_ref(), "b/b");
    }

    #[test]
    fn nulls() {
        assert!(Path::from_bytes_with_nul(b"abc\0def").is_err());
    }

    #[test]
    fn trailing_nuls() {
        assert_eq!(
            PathBuf::try_from("abc").unwrap(),
            PathBuf::try_from("abc\0").unwrap()
        );
    }

    #[test]
    fn ancestors() {
        fn assert_ancestor_parent(path: &Path) {
            let mut ancestors = path.ancestors();
            if !path.as_str().is_empty() {
                assert_eq!(&*ancestors.next().unwrap(), path);
            }
            let mut buf = PathBuf::from(path);
            loop {
                let parent = buf.parent();
                assert_eq!(parent, ancestors.next());
                match parent {
                    Some(p) => buf = p,
                    None => return,
                }
            }
        }

        let path = path!("/some/path/.././file.extension");
        assert_ancestor_parent(path);
        let mut ancestors = path.ancestors();
        assert_eq!(
            &*ancestors.next().unwrap(),
            path!("/some/path/.././file.extension")
        );
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path/../."));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path/.."));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path"));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some"));
        assert_eq!(&*ancestors.next().unwrap(), path!("/"));
        assert!(ancestors.next().is_none());

        let path = path!("/some/path/.././file.extension/");
        assert_ancestor_parent(path);
        let mut ancestors = path.ancestors();
        assert_eq!(
            &*ancestors.next().unwrap(),
            path!("/some/path/.././file.extension/")
        );
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path/../."));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path/.."));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some/path"));
        assert_eq!(&*ancestors.next().unwrap(), path!("/some"));
        assert_eq!(&*ancestors.next().unwrap(), path!("/"));
        assert!(ancestors.next().is_none());

        let path = path!("some/path/.././file.extension");
        assert_ancestor_parent(path);
        let mut ancestors = path.ancestors();
        assert_eq!(
            &*ancestors.next().unwrap(),
            path!("some/path/.././file.extension")
        );
        assert_eq!(&*ancestors.next().unwrap(), path!("some/path/../."));
        assert_eq!(&*ancestors.next().unwrap(), path!("some/path/.."));
        assert_eq!(&*ancestors.next().unwrap(), path!("some/path"));
        assert_eq!(&*ancestors.next().unwrap(), path!("some"));
        assert!(ancestors.next().is_none());
    }

    #[test]
    fn iter() {
        let path = path!("/some/path/.././file.extension");
        let mut ancestors = path.iter();
        assert_eq!(&*ancestors.next().unwrap(), path!("/"));
        assert_eq!(&*ancestors.next().unwrap(), path!("some"));
        assert_eq!(&*ancestors.next().unwrap(), path!("path"));
        assert_eq!(&*ancestors.next().unwrap(), path!(".."));
        assert_eq!(&*ancestors.next().unwrap(), path!("."));
        assert_eq!(&*ancestors.next().unwrap(), path!("file.extension"));
        assert!(ancestors.next().is_none());
        let path = path!("some/path/.././file.extension/");
        let mut ancestors = path.iter();
        assert_eq!(&*ancestors.next().unwrap(), path!("some"));
        assert_eq!(&*ancestors.next().unwrap(), path!("path"));
        assert_eq!(&*ancestors.next().unwrap(), path!(".."));
        assert_eq!(&*ancestors.next().unwrap(), path!("."));
        assert_eq!(&*ancestors.next().unwrap(), path!("file.extension"));
        assert!(ancestors.next().is_none());
    }

    #[test]
    fn file_name() {
        let path = path!("/some/path/.././file.extension");
        assert_eq!(path.file_name(), Some(path!("file.extension")));

        let path = path!("/");
        assert_eq!(path.file_name(), None);

        let path = path!("");
        assert_eq!(path.file_name(), None);

        let path = path!("/some/path/.././file.extension/");
        assert_eq!(path.file_name(), None);
    }
}
