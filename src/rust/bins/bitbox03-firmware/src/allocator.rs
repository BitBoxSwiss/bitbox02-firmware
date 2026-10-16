// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::{Cell, RefCell, UnsafeCell};
use core::ptr::null_mut;
use critical_section::Mutex;

const HEAP_SIZE: usize = 128 * 1024;

struct Region(UnsafeCell<[u64; HEAP_SIZE / 8]>);

// SAFETY: Only the global allocator obtains a pointer into this region, once under its lock.
// Keep it separate from dlmalloc's state so borrowing the allocator never borrows live allocations.
unsafe impl Sync for Region {}

static HEAP: Region = Region(UnsafeCell::new([0; HEAP_SIZE / 8]));

struct Memory {
    claimed: Cell<bool>,
}

// SAFETY: The sole dlmalloc instance lives in a static and accesses this memory only under
// ALLOCATOR's critical section. It receives the entire region once, and the region never moves
// or gets released. dlmalloc manages allocation and reuse within this fixed region.
unsafe impl dlmalloc::Allocator for Memory {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        if size > HEAP_SIZE || self.claimed.replace(true) {
            return (null_mut(), 0, 0);
        }
        (HEAP.0.get().cast(), HEAP_SIZE, 1)
    }

    fn remap(&self, _: *mut u8, _: usize, _: usize, _: bool) -> *mut u8 {
        null_mut()
    }

    fn free_part(&self, _: *mut u8, _: usize, _: usize) -> bool {
        false
    }

    fn free(&self, _: *mut u8, _: usize) -> bool {
        false
    }

    fn can_release_part(&self, _: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        4096
    }
}

struct Allocator(Mutex<RefCell<dlmalloc::Dlmalloc<Memory>>>);

// SAFETY: dlmalloc implements allocation with the requested size/alignment, and all access to
// its state is serialized by the critical section. Memory remains valid for the firmware lifetime.
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        critical_section::with(|cs| unsafe {
            self.0
                .borrow(cs)
                .borrow_mut()
                .malloc(layout.size(), layout.align())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        critical_section::with(|cs| unsafe {
            self.0
                .borrow(cs)
                .borrow_mut()
                .free(ptr, layout.size(), layout.align());
        });
    }
}

#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: Allocator = Allocator(Mutex::new(RefCell::new(
    dlmalloc::Dlmalloc::new_with_allocator(Memory {
        claimed: Cell::new(false),
    }),
)));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_alignment_exhaustion_and_reuse() {
        let small = Layout::from_size_align(65, 256).unwrap();
        let large = Layout::from_size_align(HEAP_SIZE / 2, 8).unwrap();
        // SAFETY: All successful allocations are accessed within their bounds and freed once
        // using their original layout. No pointers are dereferenced after deallocation.
        unsafe {
            for _ in 0..4 {
                let ptr = ALLOCATOR.alloc(small);
                assert!(!ptr.is_null());
                assert_eq!(ptr as usize % small.align(), 0);
                ptr.write_bytes(0xa5, small.size());
                let big = ALLOCATOR.alloc(large);
                assert!(!big.is_null());
                assert!(ALLOCATOR.alloc(large).is_null());
                assert_eq!(core::slice::from_raw_parts(ptr, small.size()), &[0xa5; 65]);
                ALLOCATOR.dealloc(big, large);
                ALLOCATOR.dealloc(ptr, small);

                let zeroed = ALLOCATOR.alloc_zeroed(small);
                assert!(!zeroed.is_null());
                assert_eq!(core::slice::from_raw_parts(zeroed, small.size()), &[0; 65]);
                ALLOCATOR.dealloc(zeroed, small);
            }
        }
    }
}
