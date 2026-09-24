//! Owned packet buffers.
//!
//! Every packet in the stack is a [`PacketBuf`]: one fixed-size buffer, owned by
//! whoever holds it (the driver, the stack, a socket, the application).
//!
//! Buffers are allocated from a static pool.

use core::cell::UnsafeCell;
use core::fmt;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use core::sync::atomic::{AtomicU32, Ordering};

use crate::config::PACKET_BUF_SIZE;
use crate::meta::PacketMeta;

#[cfg(not(test))]
const PACKET_BUF_COUNT: usize = crate::config::PACKET_BUF_COUNT;
// The unit tests run in parallel threads of one process, all sharing the one
// pool. The default is too small.
#[cfg(test)]
const PACKET_BUF_COUNT: usize = if crate::config::PACKET_BUF_COUNT > 1024 {
    crate::config::PACKET_BUF_COUNT
} else {
    1024
};

const BITMAP_WORDS: usize = PACKET_BUF_COUNT.div_ceil(32);

cfg_select! {
    feature = "packet-buf-align-32" => { #[repr(C, align(32))] struct Data([u8; PACKET_BUF_SIZE]); }
    feature = "packet-buf-align-16" => { #[repr(C, align(16))] struct Data([u8; PACKET_BUF_SIZE]); }
    feature = "packet-buf-align-8" => { #[repr(C, align(8))] struct Data([u8; PACKET_BUF_SIZE]); }
    feature = "packet-buf-align-4" => { #[repr(C, align(4))] struct Data([u8; PACKET_BUF_SIZE]); }
    feature = "packet-buf-align-2" => { #[repr(C, align(2))] struct Data([u8; PACKET_BUF_SIZE]); }
    _ => { #[repr(C, align(1))] struct Data([u8; PACKET_BUF_SIZE]); }
}

impl Deref for Data {
    type Target = [u8; PACKET_BUF_SIZE];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct PacketBufInner {
    /// Offset of the first valid byte within `data`.
    headroom: u16,
    /// Number of valid bytes.
    len: u16,
    // invariant: headroom + len <= PACKET_BUF_SIZE
    /// Per-packet metadata. Zero-sized unless a `packetmeta-*` feature is enabled.
    meta: PacketMeta,
    data: Data,
}

struct Pool {
    /// Bit `i` is set while slot `i` is owned by a `PacketBuf`.
    used: [AtomicU32; BITMAP_WORDS],
    slots: [UnsafeCell<MaybeUninit<PacketBufInner>>; PACKET_BUF_COUNT],
}

// SAFETY: a slot is handed to at most one `PacketBuf` at a time. Its bit is
// set by the one CAS that wins it in `alloc_slot`, and cleared only by the
// `PacketBuf` that owns it, in `Drop`. So no two threads ever touch the same
// slot, and the bitmap is atomic.
unsafe impl Sync for Pool {}

static POOL: Pool = Pool {
    used: [const { AtomicU32::new(0) }; BITMAP_WORDS],
    slots: [const { UnsafeCell::new(MaybeUninit::zeroed()) }; PACKET_BUF_COUNT],
};

/// Claim a free slot: the first zero bit of the bitmap, set with a CAS.
#[cfg(target_has_atomic = "32")]
#[inline(never)]
fn alloc_slot() -> Option<usize> {
    for (w, word) in POOL.used.iter().enumerate() {
        let mut cur = word.load(Ordering::Relaxed);
        loop {
            let bit = cur.trailing_ones() as usize;
            if bit >= 32 {
                break;
            }
            let index = w * 32 + bit;
            if index >= PACKET_BUF_COUNT {
                // Only the last word can have bits past the end. Everything
                // before it was full, so the pool is.
                return None;
            }
            // Acquire pairs with the Release in `Drop`: the previous owner's
            // writes to the slot are done before ours start.
            match word.compare_exchange_weak(cur, cur | (1 << bit), Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return Some(index),
                Err(actual) => cur = actual,
            }
        }
    }
    None
}

/// Give a slot back: clear its bit.
#[cfg(target_has_atomic = "32")]
#[inline(never)]
fn free_slot(index: usize) {
    POOL.used[index / 32].fetch_and(!(1 << (index % 32)), Ordering::Release);
}

// Fallback for targets with 32-bit atomic load/store but no atomic
// read-modify-write (e.g. thumbv6m): the whole bit update runs inside a
// critical section, so a plain load + store can't race another one. The
// Acquire load here still pairs with the Release store in `free_slot`, same
// as the atomic version.

/// Claim a free slot: the first zero bit of the bitmap.
#[cfg(not(target_has_atomic = "32"))]
fn alloc_slot() -> Option<usize> {
    critical_section::with(|_| {
        for (w, word) in POOL.used.iter().enumerate() {
            let cur = word.load(Ordering::Acquire);
            let bit = cur.trailing_ones() as usize;
            if bit >= 32 {
                continue;
            }
            let index = w * 32 + bit;
            if index >= PACKET_BUF_COUNT {
                // Only the last word can have bits past the end. Everything
                // before it was full, so the pool is.
                return None;
            }
            word.store(cur | (1 << bit), Ordering::Relaxed);
            return Some(index);
        }
        None
    })
}

/// Give a slot back: clear its bit.
#[cfg(not(target_has_atomic = "32"))]
fn free_slot(index: usize) {
    critical_section::with(|_| {
        let word = &POOL.used[index / 32];
        word.store(word.load(Ordering::Relaxed) & !(1 << (index % 32)), Ordering::Release);
    })
}

/// An owned network packet buffer.
///
/// ```text
/// | headroom | data (len) | tailroom |
/// ```
pub struct PacketBuf {
    inner: NonNull<PacketBufInner>,
}

// SAFETY: a `PacketBuf` is the unique owner of its slot, like a `Box` of it.
unsafe impl Send for PacketBuf {}
unsafe impl Sync for PacketBuf {}

impl PacketBuf {
    /// Allocate a buffer.
    ///
    /// - Zero headroom, len.
    /// - Default metadata.
    /// - **Uninitialized** data.
    pub fn try_new() -> Option<Self> {
        let index = alloc_slot()?;
        let ptr = POOL.slots[index].get().cast::<PacketBufInner>();
        // SAFETY:
        // - the slot is ours (its bit is set), and nothing else points into it.
        // - `data` is valid thanks to `MaybeUninit::zeroed()`, we don't have to initialize it.
        // - We do initialize the header.
        unsafe {
            (&raw mut (*ptr).headroom).write(0);
            (&raw mut (*ptr).len).write(0);
            (&raw mut (*ptr).meta).write(PacketMeta::default());
            // Catch code that relies on fresh buffers being zeroed.
            #[cfg(test)]
            (*ptr).data.fill(0xa5);
        }
        Some(Self {
            // SAFETY: a pointer into a static is never null.
            inner: unsafe { NonNull::new_unchecked(ptr) },
        })
    }

    #[inline]
    fn inner(&self) -> &PacketBufInner {
        // SAFETY: we own the slot for as long as `self` exists.
        unsafe { self.inner.as_ref() }
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut PacketBufInner {
        // SAFETY: we own the slot for as long as `self` exists, and `&mut self`
        // makes this the only reference.
        unsafe { self.inner.as_mut() }
    }

    /// The packet's metadata.
    ///
    /// On a received packet this is what the driver attached to it. On a packet being
    /// sent it is what the application attached, and what the driver will see in
    /// [`Driver::transmit`](crate::Driver::transmit). It travels with the
    /// buffer through the whole stack, unaffected by header pushes and pulls.
    pub fn meta(&self) -> PacketMeta {
        self.inner().meta
    }

    /// Mutable reference to the packet's metadata.
    pub fn meta_mut(&mut self) -> &mut PacketMeta {
        &mut self.inner_mut().meta
    }

    /// Replace the packet's metadata.
    pub fn set_meta(&mut self, meta: PacketMeta) {
        self.inner_mut().meta = meta;
    }

    /// Total storage capacity of the buffer, in bytes.
    pub const fn capacity(&self) -> usize {
        PACKET_BUF_SIZE
    }

    /// Amount of free space in front of the payload.
    pub fn headroom(&self) -> usize {
        self.inner().headroom as usize
    }

    /// Length of the payload.
    pub fn len(&self) -> usize {
        self.inner().len as usize
    }

    /// Whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.inner().len == 0
    }

    /// Amount of free space behind the payload.
    pub fn tailroom(&self) -> usize {
        PACKET_BUF_SIZE - self.headroom() - self.len()
    }

    /// Set the headroom on an empty buffer, before writing a payload.
    ///
    /// # Panics
    /// Panics if the buffer is not empty, or if `headroom > capacity`.
    pub fn reserve(&mut self, headroom: usize) {
        assert!(self.inner().len == 0);
        assert!(headroom <= PACKET_BUF_SIZE);
        self.inner_mut().headroom = headroom as u16;
    }

    /// Grow the payload at the front by `n` bytes, taking them from the headroom.
    ///
    /// # Panics
    /// Panics if `n > headroom`.
    pub fn push_front(&mut self, n: usize) {
        assert!(n <= self.headroom());
        let inner = self.inner_mut();
        inner.headroom -= n as u16;
        inner.len += n as u16;
    }

    /// Shrink the payload at the front by `n` bytes, returning them to the headroom.
    ///
    /// # Panics
    /// Panics if `n > len`.
    pub fn pull_front(&mut self, n: usize) {
        assert!(n <= self.len());
        let inner = self.inner_mut();
        inner.headroom += n as u16;
        inner.len -= n as u16;
    }

    /// Make room for `headroom` bytes in front of the payload, moving the payload
    /// back if there isn't enough already.
    ///
    /// Returns `false` if the buffer can't fit `headroom` plus the payload, leaving
    /// it unchanged.
    pub fn ensure_headroom(&mut self, headroom: usize) -> bool {
        if self.headroom() >= headroom {
            return true;
        }
        let inner = self.inner_mut();
        let len = inner.len as usize;
        if headroom + len > PACKET_BUF_SIZE {
            return false;
        }
        let old = inner.headroom as usize;
        inner.data.copy_within(old..old + len, headroom);
        inner.headroom = headroom as u16;
        true
    }

    /// Set the payload length, growing or shrinking it at the back.
    ///
    /// # Panics
    /// Panics if `headroom + len > capacity`.
    pub fn set_len(&mut self, len: usize) {
        assert!(self.headroom() + len <= PACKET_BUF_SIZE);
        self.inner_mut().len = len as u16;
    }

    /// The whole underlying storage, ignoring headroom and length.
    ///
    /// The returned slice is aligned to [`PACKET_BUF_ALIGN`](crate::config::PACKET_BUF_ALIGN), and its length
    /// ([`PACKET_BUF_SIZE`]) is a multiple of it.
    pub fn storage_mut(&mut self) -> &mut [u8] {
        &mut self.inner_mut().data[..]
    }
}

impl Drop for PacketBuf {
    #[inline(never)] // helps code size
    fn drop(&mut self) {
        let base = POOL.slots.as_ptr() as usize;
        let index =
            (self.inner.as_ptr() as usize - base) / core::mem::size_of::<UnsafeCell<MaybeUninit<PacketBufInner>>>();
        free_slot(index);
    }
}

impl Deref for PacketBuf {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        let inner = self.inner();
        let start = inner.headroom as usize;
        let end = start + inner.len as usize;
        &inner.data[start..end]
    }
}
impl DerefMut for PacketBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let inner = self.inner_mut();
        let start = inner.headroom as usize;
        let end = start + inner.len as usize;
        &mut inner.data[start..end]
    }
}

impl fmt::Debug for PacketBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PacketBuf")
            .field("headroom", &self.headroom())
            .field("len", &self.len())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PacketBuf {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(f, "PacketBuf {{ headroom: {}, len: {} }}", self.headroom(), self.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PACKET_BUF_ALIGN;

    #[test]
    fn push_pull() {
        let mut buf = PacketBuf::try_new().unwrap();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.headroom(), 0);
        assert_eq!(buf.tailroom(), PACKET_BUF_SIZE);

        buf.reserve(42);
        assert_eq!(buf.headroom(), 42);
        buf.set_len(100);
        assert_eq!(buf.len(), 100);
        assert_eq!(buf.tailroom(), PACKET_BUF_SIZE - 142);
        buf.fill(0xaa);

        buf.push_front(20);
        assert_eq!(buf.headroom(), 22);
        assert_eq!(buf.len(), 120);
        assert_eq!(buf[20], 0xaa);

        buf.pull_front(20);
        assert_eq!(buf.headroom(), 42);
        assert_eq!(buf.len(), 100);
        assert_eq!(buf[0], 0xaa);
    }

    #[test]
    fn ensure_headroom() {
        let mut buf = PacketBuf::try_new().unwrap();
        buf.reserve(10);
        buf.set_len(4);
        buf.copy_from_slice(&[1, 2, 3, 4]);

        // Already enough: nothing moves.
        assert!(buf.ensure_headroom(4));
        assert_eq!(buf.headroom(), 10);
        assert_eq!(&*buf, &[1, 2, 3, 4]);

        // Not enough: the payload moves back, unchanged.
        assert!(buf.ensure_headroom(20));
        assert_eq!(buf.headroom(), 20);
        assert_eq!(buf.len(), 4);
        assert_eq!(&*buf, &[1, 2, 3, 4]);

        // The headroom overlapping the payload is fine, it's a move not a copy.
        assert!(buf.ensure_headroom(22));
        assert_eq!(&*buf, &[1, 2, 3, 4]);

        // Doesn't fit: the buffer is left alone.
        assert!(!buf.ensure_headroom(PACKET_BUF_SIZE - 3));
        assert_eq!(buf.headroom(), 22);
        assert_eq!(&*buf, &[1, 2, 3, 4]);
        assert!(buf.ensure_headroom(PACKET_BUF_SIZE - 4));
        assert_eq!(&*buf, &[1, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn push_beyond_headroom() {
        let mut buf = PacketBuf::try_new().unwrap();
        buf.push_front(1);
    }

    /// The storage a driver DMAs into must stay aligned to `PACKET_BUF_ALIGN` and
    /// a multiple of it long, whatever the metadata in front of it does to the
    /// layout.
    #[test]
    fn storage_is_dma_shaped() {
        let mut buf = PacketBuf::try_new().unwrap();
        assert_eq!(buf.storage_mut().as_ptr() as usize % PACKET_BUF_ALIGN, 0);
        assert_eq!(buf.storage_mut().len() % PACKET_BUF_ALIGN, 0);
        assert!(buf.storage_mut().len() >= PACKET_BUF_SIZE);
    }

    /// A fresh buffer starts out empty with default metadata, whatever its previous
    /// owner left behind. (Pool exhaustion and reuse are covered by xarxa's
    /// `packet_pool` integration test, which has a process's pool to itself.)
    #[test]
    fn fresh_buffer_is_reset() {
        let mut buf = PacketBuf::try_new().unwrap();
        buf.reserve(100);
        buf.set_len(200);
        buf.fill(0xff);
        drop(buf);

        let buf = PacketBuf::try_new().unwrap();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.headroom(), 0);
        assert_eq!(buf.meta(), PacketMeta::default());
    }

    /// Metadata rides along with the buffer, untouched by the header pushes and pulls
    /// the packet goes through on its way up or down the stack.
    #[cfg(feature = "packetmeta-id")]
    #[test]
    fn meta_travels_with_the_buffer() {
        let mut buf = PacketBuf::try_new().unwrap();
        assert_eq!(buf.meta(), PacketMeta::default());

        buf.meta_mut().id = 0xdead_beef;
        buf.reserve(20);
        buf.set_len(10);
        buf.push_front(20);
        buf.pull_front(4);
        assert_eq!(buf.meta().id, 0xdead_beef);

        buf.set_meta(PacketMeta::default());
        assert_eq!(buf.meta().id, 0);
    }
}
