// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataPartition {
    Vendor,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MountPolicy {
    MountOnly,
    FormatIfEmpty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidInput,
    NoSuchEntry,
    AlreadyExists,
    NoSpace,
    Corrupt,
    Io,
    Unsupported,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Volume {
    fn read_file(&mut self, path: &str, out: &mut Vec<u8>) -> Result<()>;
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()>;
    fn remove_file(&mut self, path: &str) -> Result<()>;
    fn rename(&mut self, from: &str, to: &str) -> Result<()>;
    fn exists(&mut self, path: &str) -> Result<bool>;
}

pub trait Filesystem {
    fn with_volume<R, F>(
        &mut self,
        partition: DataPartition,
        policy: MountPolicy,
        f: F,
    ) -> Result<R>
    where
        F: FnOnce(&mut dyn Volume) -> Result<R>;

    fn format_volume(&mut self, partition: DataPartition) -> Result<()>;
}
