use core::{cmp, ffi::c_int};

use bitflags::bitflags;

use crate::path::{Path, PathBuf};

bitflags! {
    /// Definition of file open flags which can be mixed and matched as appropriate. These definitions
    /// are reminiscent of the ones defined by POSIX.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FileOpenFlags: c_int {
        /// Open file in read only mode.
        const READ = 0x1;
        /// Open file in write only mode.
        const WRITE = 0x2;
        /// Open file for reading and writing.
        const READWRITE = Self::READ.bits() | Self::WRITE.bits();
        /// Create the file if it does not exist.
        const CREATE = 0x0100;
        /// Fail if creating a file that already exists.
        /// TODO: Good name for this
        const EXCL = 0x0200;
        /// Truncate the file if it already exists.
        const TRUNCATE = 0x0400;
        /// Open the file in append only mode.
        const APPEND = 0x0800;
    }
}

/// Regular file vs directory
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileType {
    File,
    Dir,
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        *self == FileType::Dir
    }

    pub fn is_file(&self) -> bool {
        *self == FileType::File
    }
}

/// File type (regular vs directory) and size of a file.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata {
    file_type: FileType,
    size: usize,
}

impl Metadata {
    pub fn new(file_type: FileType, size: usize) -> Self {
        Self { file_type, size }
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Custom user attribute that can be set on files and directories.
///
/// This struct stores the data that has been read from the filesystem and
/// the total size of the attribute on the filesystem.  The maximum size of an
/// attribute is [`Attribute::MAX_SIZE`][].
///
/// See [`DynFilesystem::attribute`](`crate::object_safe::DynFilesystem::attribute`).
pub struct Attribute<'a> {
    data: &'a [u8],
    total_size: usize,
}

impl<'a> Attribute<'a> {
    pub const MAX_SIZE: u32 = 1_022;

    pub fn new(data: &'a [u8], total_size: usize) -> Self {
        let n = cmp::min(data.len(), total_size);
        let data = &data[..n];
        Attribute { data, total_size }
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn total_size(&self) -> usize {
        self.total_size
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirEntry {
    file_name: PathBuf,
    metadata: Metadata,
    path: PathBuf,
}

impl DirEntry {
    pub fn new(file_name: PathBuf, metadata: Metadata, path: PathBuf) -> Self {
        Self {
            file_name,
            metadata,
            path,
        }
    }

    // Returns the metadata for the file that this entry points at.
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    // Returns the file type for the file that this entry points at.
    pub fn file_type(&self) -> FileType {
        self.metadata.file_type
    }

    // Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name(&self) -> &Path {
        &self.file_name
    }

    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to read_dir with the filename of this entry.
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[doc(hidden)]
    // This is used in `crypto-service` to "namespace" paths
    // by mutating a DirEntry in-place.
    pub unsafe fn path_buf_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}
