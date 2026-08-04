#![allow(non_camel_case_types)]

/// Re-export of `typenum::consts`.
pub use generic_array::typenum::consts::*;

pub const PATH_MAX: usize = littlefs2_core::PathBuf::MAX_SIZE;
pub const PATH_MAX_PLUS_ONE: usize = littlefs2_core::PathBuf::MAX_SIZE_PLUS_ONE;
pub const FILENAME_MAX_PLUS_ONE: u32 = 255 + 1;
pub const FILEBYTES_MAX: u32 = crate::ll::LFS_FILE_MAX as _;
pub const ATTRBYTES_MAX: u32 = littlefs2_core::Attribute::MAX_SIZE;
pub const LOOKAHEADWORDS_SIZE: u32 = 16;
