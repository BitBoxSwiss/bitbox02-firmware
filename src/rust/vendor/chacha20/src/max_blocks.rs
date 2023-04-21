//! Maximum number of blocks that can be encrypted with ChaCha before the
//! counter overflows.

/// Marker type for the maximum ChaCha counter value
pub trait MaxCounter: Copy {
    const MAX_BLOCKS: Option<u64>;
}

/// 32-bit counter
#[derive(Copy, Clone)]
pub struct C32;

impl MaxCounter for C32 {
    const MAX_BLOCKS: Option<u64> = Some(u32::MAX as u64);
}

/// 64-bit counter
#[derive(Copy, Clone)]
pub struct C64;

impl MaxCounter for C64 {
    const MAX_BLOCKS: Option<u64> = None;
}
