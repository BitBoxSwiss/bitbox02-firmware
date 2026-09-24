//! Compile-time configuration.
//!
//! The sizes of the packet pool and of the buffers in it are set at compile
//! time. They can be set in two ways:
//!
//! - With a cargo feature named `<name>-<value>`, lowercase and with dashes
//!   instead of underscores. For example `packet-buf-count-32`. Only the values
//!   listed in `Cargo.toml` can be set this way.
//! - With an environment variable named `XARXA_<NAME>` at build time. For
//!   example `XARXA_PACKET_BUF_COUNT=32 cargo build`. They can also be set in
//!   the `[env]` section of `.cargo/config.toml`. Any value can be set this way.
//!
//! Environment variables win over cargo features. Enabling two cargo features
//! for the same setting with different values fails the build.
//!
//! The `xarxa` crate forwards the features of the same name to this crate, and
//! has its own knobs in `xarxa::config`.

mod raw {
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

/// Number of buffers in the packet pool.
///
/// Every packet in flight takes one buffer: in a driver's receive ring, in a
/// socket's queue, being reassembled, or parked waiting for a neighbor. When
/// they are all in use, [`PacketBuf::try_new`](crate::PacketBuf::try_new) fails
/// and packets are dropped.
///
/// Default: 16.
pub const PACKET_BUF_COUNT: usize = raw::PACKET_BUF_COUNT;

/// Alignment of the buffer in a [`PacketBuf`](crate::PacketBuf), in bytes.
///
/// DMA engines often require the buffers they write to be aligned. Raising this
/// also rounds [`PACKET_BUF_SIZE`] up to a multiple of it, since such engines
/// write whole bus words past the end of the frame.
///
/// Can only be set with cargo features, not with an environment variable. If
/// several are enabled, the highest wins.
///
/// Supported values: 1, 2, 4, 8, 16, 32.
///
/// Default: 1.
pub const PACKET_BUF_ALIGN: usize = cfg_select! {
    feature = "packet-buf-align-32" => 32,
    feature = "packet-buf-align-16" => 16,
    feature = "packet-buf-align-8" => 8,
    feature = "packet-buf-align-4" => 4,
    feature = "packet-buf-align-2" => 2,
    _ => 1,
};

/// Size of the buffer in a [`PacketBuf`](crate::PacketBuf), in bytes.
///
/// This is the largest frame that can be sent or received, headers included.
///
/// Not configurable yet: it is 1514 (the largest Ethernet frame without the FCS)
/// rounded up to a multiple of [`PACKET_BUF_ALIGN`].
// TODO: make configurable
pub const PACKET_BUF_SIZE: usize = 1514usize.next_multiple_of(PACKET_BUF_ALIGN);
