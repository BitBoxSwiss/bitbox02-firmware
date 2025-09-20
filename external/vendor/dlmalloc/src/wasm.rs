use crate::Allocator;
#[cfg(target_arch = "wasm32")]
use core::arch::wasm32 as wasm;
#[cfg(target_arch = "wasm64")]
use core::arch::wasm64 as wasm;
use core::ptr;

/// System setting for Wasm
pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

unsafe impl Allocator for System {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let pages = size.div_ceil(self.page_size());
        let prev = wasm::memory_grow(0, pages);

        // If the allocation failed, meaning `prev` is -1 or
        // `usize::max_value()`, then return null.
        if prev == usize::max_value() {
            return (ptr::null_mut(), 0, 0);
        }

        let prev_page = prev * self.page_size();
        let base_ptr = prev_page as *mut u8;
        let size = pages * self.page_size();

        // Additionally check to see if we just allocated the final bit of the
        // address space. In such a situation it's not valid in Rust for a
        // pointer to actually wrap around to from the top of the address space
        // to 0, so it's not valid to allocate the entire region. Fake the last
        // few bytes as being un-allocated meaning that the actual size of this
        // allocation won't be page aligned, which should be handled by
        // dlmalloc.
        if prev_page.wrapping_add(size) == 0 {
            return (base_ptr, size - 16, 0);
        }

        (base_ptr, size, 0)
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        // TODO: I think this can be implemented near the end?
        ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        false
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        64 * 1024
    }
}

#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    // single threaded, no need!
    assert!(!cfg!(target_feature = "atomics"));
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    // single threaded, no need!
    assert!(!cfg!(target_feature = "atomics"));
}

#[allow(missing_docs)]
#[cfg(feature = "global")]
pub unsafe fn enable_alloc_after_fork() {
    // single threaded, no need!
    assert!(!cfg!(target_feature = "atomics"));
}
