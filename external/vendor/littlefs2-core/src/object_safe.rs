use crate::{
    fs::{Attribute, DirEntry, FileOpenFlags, Metadata},
    io::{Error, OpenSeekFrom, Read, Result, Seek, Write},
    path::Path,
};

// Make sure that the traits actually are object safe.
const _: Option<&dyn DynFile> = None;
const _: Option<&dyn DynFilesystem> = None;

pub type DirEntriesCallback<'a, R = ()> =
    &'a mut dyn FnMut(&mut dyn Iterator<Item = Result<DirEntry>>) -> Result<R>;
pub type FileCallback<'a, R = ()> = &'a mut dyn FnMut(&dyn DynFile) -> Result<R>;
pub type Predicate<'a> = &'a dyn Fn(&DirEntry) -> bool;

pub trait Vec: Default + AsRef<[u8]> + AsMut<[u8]> {
    fn resize_to_capacity(&mut self);
    fn truncate(&mut self, n: usize);
}

#[cfg(feature = "heapless-bytes03")]
impl<const N: usize> Vec for heapless_bytes03::Bytes<N> {
    fn resize_to_capacity(&mut self) {
        heapless_bytes03::Bytes::resize_to_capacity(self)
    }

    fn truncate(&mut self, n: usize) {
        use core::ops::DerefMut as _;

        self.deref_mut().truncate(n)
    }
}

#[cfg(feature = "heapless-bytes04")]
impl<const N: usize> Vec for heapless_bytes04::Bytes<N> {
    fn resize_to_capacity(&mut self) {
        heapless_bytes04::Bytes::resize_to_capacity(self)
    }

    fn truncate(&mut self, n: usize) {
        heapless_bytes04::Bytes::truncate(self, n)
    }
}

#[cfg(feature = "heapless-bytes05")]
impl<const N: usize> Vec for heapless_bytes05::Bytes<N> {
    fn resize_to_capacity(&mut self) {
        heapless_bytes05::Bytes::resize_to_capacity(self)
    }

    fn truncate(&mut self, n: usize) {
        heapless_bytes05::Bytes::truncate(self, n)
    }
}

#[cfg(feature = "heapless07")]
impl<const N: usize> Vec for heapless07::Vec<u8, N> {
    fn resize_to_capacity(&mut self) {
        self.resize_default(self.capacity()).unwrap();
    }

    fn truncate(&mut self, n: usize) {
        heapless07::Vec::truncate(self, n)
    }
}

#[cfg(feature = "heapless08")]
impl<const N: usize> Vec for heapless08::Vec<u8, N> {
    fn resize_to_capacity(&mut self) {
        self.resize_default(self.capacity()).unwrap();
    }

    fn truncate(&mut self, n: usize) {
        heapless08::Vec::truncate(self, n)
    }
}

#[cfg(feature = "heapless09")]
impl<LenT: heapless09::LenType, const N: usize> Vec for heapless09::Vec<u8, N, LenT> {
    fn resize_to_capacity(&mut self) {
        self.resize_default(self.capacity()).unwrap();
    }

    fn truncate(&mut self, n: usize) {
        heapless09::Vec::truncate(self, n)
    }
}

/// Object-safe trait for files.
///
/// The methods for opening files cannot be implemented in this trait.  Use these methods instead:
/// - [`DynFilesystem::create_file_and_then`](trait.DynFilesystem.html#method.create_file_and_then)
/// - [`DynFilesystem::open_file_and_then`](trait.DynFilesystem.html#method.open_file_and_then)
/// - [`DynFilesystem::open_file_with_options_and_then`](trait.DynFilesystem.html#method.open_file_with_options_and_then)
pub trait DynFile: Read + Seek + Write {
    fn sync(&self) -> Result<()>;
    fn len(&self) -> Result<usize>;
    fn is_empty(&self) -> Result<bool>;
    fn set_len(&self, size: usize) -> Result<()>;
}

impl dyn DynFile + '_ {
    pub fn read_to_end<V: Vec>(&self, buf: &mut V) -> Result<usize> {
        let had = buf.as_ref().len();
        buf.resize_to_capacity();
        let read = self.read(&mut buf.as_mut()[had..])?;
        buf.truncate(had + read);
        Ok(read)
    }
}

/// Object-safe trait for filesystems.
///
/// The following methods cannot support generic return types in the callbacks:
/// - [`DynFilesystem::create_file_and_then_unit`][]
/// - [`DynFilesystem::open_file_and_then_unit`][]
/// - [`DynFilesystem::open_file_with_flags_and_then_unit`][]
/// - [`DynFilesystem::read_dir_and_then_unit`][]
///
/// Use these helper functions instead:
/// - [`DynFilesystem::create_file_and_then`](#method.create_file_and_then)
/// - [`DynFilesystem::open_file_and_then`](#method.open_file_and_then)
/// - [`DynFilesystem::open_file_with_flags_and_then`](#method.open_file_with_flags_and_then)
/// - [`DynFilesystem::read_dir_and_then`](#method.read_dir_and_then)
pub trait DynFilesystem {
    fn total_blocks(&self) -> usize;
    fn total_space(&self) -> usize;
    fn available_blocks(&self) -> Result<usize>;
    fn available_space(&self) -> Result<usize>;
    fn remove(&self, path: &Path) -> Result<()>;
    fn remove_dir(&self, path: &Path) -> Result<()>;
    fn remove_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_dir_all_where(&self, path: &Path, predicate: Predicate<'_>) -> Result<usize>;
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn metadata(&self, path: &Path) -> Result<Metadata>;
    fn create_file_and_then_unit(&self, path: &Path, f: FileCallback<'_>) -> Result<()>;
    fn open_file_and_then_unit(&self, path: &Path, f: FileCallback<'_>) -> Result<()>;
    fn open_file_with_flags_and_then_unit(
        &self,
        flags: FileOpenFlags,
        path: &Path,
        f: FileCallback<'_>,
    ) -> Result<()>;
    fn attribute<'a>(
        &self,
        path: &Path,
        id: u8,
        buffer: &'a mut [u8],
    ) -> Result<Option<Attribute<'a>>>;
    fn remove_attribute(&self, path: &Path, id: u8) -> Result<()>;
    fn set_attribute(&self, path: &Path, id: u8, data: &[u8]) -> Result<()>;
    fn read_dir_and_then_unit(&self, path: &Path, f: DirEntriesCallback<'_>) -> Result<()>;
    fn create_dir(&self, path: &Path) -> Result<()>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;
    fn write_chunk(&self, path: &Path, contents: &[u8], pos: OpenSeekFrom) -> Result<()>;
}

impl dyn DynFilesystem + '_ {
    pub fn read<V: Vec>(&self, path: &Path) -> Result<V> {
        let mut contents = V::default();
        self.open_file_and_then(path, &mut |file| {
            file.read_to_end(&mut contents)?;
            Ok(())
        })?;
        Ok(contents)
    }

    pub fn read_chunk<V: Vec>(&self, path: &Path, pos: OpenSeekFrom) -> Result<(V, usize)> {
        let mut contents = V::default();
        contents.resize_to_capacity();
        let file_len = self.open_file_and_then(path, &mut |file| {
            file.seek(pos.into())?;
            let read_n = file.read(contents.as_mut())?;
            contents.truncate(read_n);
            file.len()
        })?;
        Ok((contents, file_len))
    }

    pub fn create_file_and_then<R>(&self, path: &Path, f: FileCallback<'_, R>) -> Result<R> {
        let mut result = Err(Error::IO);
        self.create_file_and_then_unit(path, &mut |file| {
            result = Ok(f(file)?);
            Ok(())
        })?;
        result
    }

    pub fn open_file_and_then<R>(&self, path: &Path, f: FileCallback<'_, R>) -> Result<R> {
        let mut result = Err(Error::IO);
        self.open_file_and_then_unit(path, &mut |file| {
            result = Ok(f(file)?);
            Ok(())
        })?;
        result
    }

    pub fn open_file_with_flags_and_then<R>(
        &self,
        flags: FileOpenFlags,
        path: &Path,
        f: FileCallback<'_, R>,
    ) -> Result<R> {
        let mut result = Err(Error::IO);
        self.open_file_with_flags_and_then_unit(flags, path, &mut |file| {
            result = Ok(f(file)?);
            Ok(())
        })?;
        result
    }

    pub fn read_dir_and_then<R>(&self, path: &Path, f: DirEntriesCallback<'_, R>) -> Result<R> {
        let mut result = Err(Error::IO);
        self.read_dir_and_then_unit(path, &mut |entries| {
            result = Ok(f(entries)?);
            Ok(())
        })?;
        result
    }
}
