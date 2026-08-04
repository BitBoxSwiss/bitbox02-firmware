//! Object-safe traits for [`File`][], [`Filesystem`][] and [`Storage`][].

use generic_array::typenum::Unsigned as _;

use crate::{
    driver::Storage,
    fs::{Attribute, File, FileOpenFlags, Filesystem, Metadata},
    io::{Error, OpenSeekFrom, Result},
    path::Path,
};

pub use littlefs2_core::{DirEntriesCallback, DynFile, DynFilesystem, FileCallback, Predicate};

// Make sure that the traits actually are object safe.
const _: Option<&dyn DynStorage> = None;

pub type FilesystemCallback<'a, R = ()> = &'a mut dyn FnMut(&dyn DynFilesystem) -> Result<R>;
#[cfg(feature = "alloc")]
pub type FilesystemCallbackOnce<'a, R = ()> =
    alloc::boxed::Box<dyn FnOnce(&dyn DynFilesystem) -> Result<R> + 'a>;

impl<S: Storage> DynFile for File<'_, '_, S> {
    fn sync(&self) -> Result<()> {
        File::sync(self)
    }

    fn len(&self) -> Result<usize> {
        File::len(self)
    }

    fn is_empty(&self) -> Result<bool> {
        File::is_empty(self)
    }

    fn set_len(&self, size: usize) -> Result<()> {
        File::set_len(self, size)
    }
}

impl<S: Storage> DynFilesystem for Filesystem<'_, S> {
    fn total_blocks(&self) -> usize {
        Filesystem::total_blocks(self)
    }

    fn total_space(&self) -> usize {
        Filesystem::total_space(self)
    }

    fn available_blocks(&self) -> Result<usize> {
        Filesystem::available_blocks(self)
    }

    fn available_space(&self) -> Result<usize> {
        Filesystem::available_space(self)
    }

    fn remove(&self, path: &Path) -> Result<()> {
        Filesystem::remove(self, path)
    }

    fn remove_dir(&self, path: &Path) -> Result<()> {
        Filesystem::remove_dir(self, path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        Filesystem::remove_dir_all(self, path)
    }

    fn remove_dir_all_where(&self, path: &Path, predicate: Predicate<'_>) -> Result<usize> {
        Filesystem::remove_dir_all_where(self, path, &|entry| predicate(entry))
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<()> {
        Filesystem::rename(self, from, to)
    }

    fn exists(&self, path: &Path) -> bool {
        Filesystem::exists(self, path)
    }

    fn metadata(&self, path: &Path) -> Result<Metadata> {
        Filesystem::metadata(self, path)
    }

    fn create_file_and_then_unit(&self, path: &Path, f: FileCallback<'_>) -> Result<()> {
        Filesystem::create_file_and_then(self, path, |file| f(file))
    }

    fn open_file_and_then_unit(&self, path: &Path, f: FileCallback<'_>) -> Result<()> {
        Filesystem::open_file_and_then(self, path, |file| f(file))
    }

    fn open_file_with_flags_and_then_unit(
        &self,
        flags: FileOpenFlags,
        path: &Path,
        f: FileCallback<'_>,
    ) -> Result<()> {
        Filesystem::open_file_with_options_and_then(
            self,
            |o| {
                *o = flags.into();
                o
            },
            path,
            |file| f(file),
        )
    }

    fn attribute<'a>(
        &self,
        path: &Path,
        id: u8,
        buffer: &'a mut [u8],
    ) -> Result<Option<Attribute<'a>>> {
        Filesystem::attribute(self, path, id, buffer)
    }

    fn remove_attribute(&self, path: &Path, id: u8) -> Result<()> {
        Filesystem::remove_attribute(self, path, id)
    }

    fn set_attribute(&self, path: &Path, id: u8, data: &[u8]) -> Result<()> {
        Filesystem::set_attribute(self, path, id, data)
    }

    fn read_dir_and_then_unit(&self, path: &Path, f: DirEntriesCallback<'_>) -> Result<()> {
        Filesystem::read_dir_and_then(self, path, |entries| f(entries))
    }

    fn create_dir(&self, path: &Path) -> Result<()> {
        Filesystem::create_dir(self, path)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        Filesystem::create_dir_all(self, path)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        Filesystem::write(self, path, contents)
    }

    fn write_chunk(&self, path: &Path, contents: &[u8], pos: OpenSeekFrom) -> Result<()> {
        Filesystem::write_chunk(self, path, contents, pos)
    }
}

/// Object-safe trait for [`Storage`][].
///
/// It contains these additional methods from [`Filesystem`][]:
/// - [`DynStorage::format`][]
/// - [`DynStorage::is_mountable`][]
/// - [`DynStorage::mount_and_then`](#method.mount_and_then)
///
/// The following methods cannot support generic return types in the callbacks:
/// - [`DynStorage::mount_and_then_unit`][]
///
/// Use these helper functions instead:
/// - [`DynStorage::mount_and_then`](#method.mount_and_then)
///
/// The `read`, `write` and `erase` methods are mirrored directly.  The associated constants and
/// types are transformed into methods.  See the documentation for [`Storage`][] for more
/// information.
pub trait DynStorage {
    fn read_size(&self) -> usize;
    fn write_size(&self) -> usize;
    fn block_size(&self) -> usize;
    fn block_count(&self) -> usize;
    fn block_cycles(&self) -> isize;
    fn cache_size(&self) -> usize;
    fn lookahead_size(&self) -> usize;
    fn read(&mut self, off: usize, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, off: usize, data: &[u8]) -> Result<usize>;
    fn erase(&mut self, off: usize, len: usize) -> Result<usize>;
    fn format(&mut self) -> Result<()>;
    fn is_mountable(&mut self) -> bool;
    fn mount_and_then_unit(&mut self, f: FilesystemCallback<'_>) -> Result<()>;
}

impl<S: Storage> DynStorage for S {
    fn read_size(&self) -> usize {
        Self::READ_SIZE
    }

    fn write_size(&self) -> usize {
        Self::WRITE_SIZE
    }

    fn block_size(&self) -> usize {
        Self::BLOCK_SIZE
    }

    fn block_count(&self) -> usize {
        Self::BLOCK_COUNT
    }

    fn block_cycles(&self) -> isize {
        Self::BLOCK_CYCLES
    }

    fn cache_size(&self) -> usize {
        S::CACHE_SIZE::to_usize()
    }

    fn lookahead_size(&self) -> usize {
        S::LOOKAHEAD_SIZE::to_usize()
    }

    fn read(&mut self, off: usize, buf: &mut [u8]) -> Result<usize> {
        Storage::read(self, off, buf)
    }

    fn write(&mut self, off: usize, data: &[u8]) -> Result<usize> {
        Storage::write(self, off, data)
    }

    fn erase(&mut self, off: usize, len: usize) -> Result<usize> {
        Storage::erase(self, off, len)
    }

    fn format(&mut self) -> Result<()> {
        Filesystem::format(self)
    }

    fn is_mountable(&mut self) -> bool {
        Filesystem::is_mountable(self)
    }

    fn mount_and_then_unit(&mut self, f: FilesystemCallback<'_>) -> Result<()> {
        Filesystem::mount_and_then(self, |fs| f(fs))
    }
}

impl dyn DynStorage + '_ {
    pub fn mount_and_then<R>(&mut self, f: FilesystemCallback<'_, R>) -> Result<R> {
        let mut result = Err(Error::IO);
        self.mount_and_then_unit(&mut |fs| {
            result = Ok(f(fs)?);
            Ok(())
        })?;
        result
    }
}

#[cfg(feature = "alloc")]
/// Extension trait for [`DynStorage`][] that requires `alloc`.
///
/// This extension trait uses owned `FnOnce` callbacks instead of references to `FnMut`.
pub trait DynStorageAlloc: DynStorage {
    fn mount_and_then_unit_once(&mut self, f: FilesystemCallbackOnce<'_>) -> Result<()>;
}

#[cfg(feature = "alloc")]
impl<T: Storage> DynStorageAlloc for T {
    fn mount_and_then_unit_once(&mut self, f: FilesystemCallbackOnce<'_>) -> Result<()> {
        Filesystem::mount_and_then(self, |fs| f(fs))
    }
}

#[cfg(feature = "alloc")]
impl dyn DynStorageAlloc + '_ {
    pub fn mount_and_then_once<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn DynFilesystem) -> Result<R>,
    {
        let mut result = Err(Error::IO);
        self.mount_and_then_unit_once(alloc::boxed::Box::new(|fs| {
            result = Ok(f(fs)?);
            Ok(())
        }))?;
        result
    }
}
