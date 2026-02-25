// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::VecDeque;

pub struct ByteQueue {
    queue: VecDeque<u8>,
    initial_capacity: usize,
}

/// Opaque C handle for a [`ByteQueue`] allocated and owned by Rust.
#[repr(C)]
pub struct RustByteQueue {
    _private: [u8; 0],
}

impl ByteQueue {
    /// Creates a bytequeue with a fixed maximum number of elements.
    ///
    /// The buffer panics on [`put`](Self::put) once it reaches `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            initial_capacity: capacity,
        }
    }

    /// Pushes one byte to the back of the queue.
    ///
    /// Panics if inserting would exceed the initial capacity.
    pub fn put(&mut self, data: u8) {
        if self.queue.len() >= self.initial_capacity {
            panic!("bytequeue overflow");
        }
        self.queue.push_back(data);
    }

    /// Pops one byte from the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    pub fn get(&mut self) -> Option<u8> {
        self.queue.pop_front()
    }

    /// Returns the number of queued bytes.
    ///
    /// Panics if the internal length does not fit into `u32`.
    pub fn num(&self) -> u32 {
        u32::try_from(self.queue.len()).unwrap()
    }

    /// Removes all queued bytes.
    pub fn flush(&mut self) {
        self.queue.clear();
    }
}

unsafe fn bytequeue_mut<'a>(rb: *mut RustByteQueue) -> Option<&'a mut ByteQueue> {
    if rb.is_null() {
        return None;
    }
    Some(unsafe { &mut *(rb.cast::<ByteQueue>()) })
}

unsafe fn bytequeue_ref<'a>(rb: *const RustByteQueue) -> Option<&'a ByteQueue> {
    if rb.is_null() {
        return None;
    }
    Some(unsafe { &*(rb.cast::<ByteQueue>()) })
}

/// Allocates a new bytequeue and returns an opaque handle.
///
/// The returned pointer must be freed with [`rust_bytequeue_free`].
#[unsafe(no_mangle)]
pub extern "C" fn rust_bytequeue_init(capacity: usize) -> *mut RustByteQueue {
    Box::into_raw(Box::new(ByteQueue::with_capacity(capacity))).cast::<RustByteQueue>()
}

/// Frees a bytequeue previously created by [`rust_bytequeue_init`].
///
/// # Safety
/// If `rb` is non-null, it must be a pointer returned by
/// [`rust_bytequeue_init`] that has not already been freed. Passing any other
/// pointer, or freeing the same pointer more than once, is undefined behavior.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bytequeue_free(rb: *mut RustByteQueue) -> bool {
    if rb.is_null() {
        return false;
    }

    unsafe { drop(Box::from_raw(rb.cast::<ByteQueue>())) };
    true
}

/// Pops one byte from the front of the queue into `data_out`.
///
/// # Safety
/// `rb` must be either null or a valid pointer to a [`ByteQueue`] for the
/// duration of this call. If non-null, the pointed-to bytequeue must not be
/// aliased for mutable access elsewhere.
///
/// `data_out` must be non-null and valid for writing one byte.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bytequeue_get(rb: *mut RustByteQueue, data_out: *mut u8) -> bool {
    if data_out.is_null() {
        return false;
    }
    let Some(rb) = (unsafe { bytequeue_mut(rb) }) else {
        return false;
    };
    let Some(data) = rb.get() else {
        return false;
    };
    unsafe { *data_out = data };
    true
}

/// Pushes one byte to the back of the queue.
///
/// # Safety
/// `rb` must be either null or a valid pointer to a [`ByteQueue`] for the
/// duration of this call. If non-null, the pointed-to bytequeue must not be
/// aliased for mutable access elsewhere.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bytequeue_put(rb: *mut RustByteQueue, data: u8) {
    if let Some(rb) = unsafe { bytequeue_mut(rb) } {
        rb.put(data);
    }
}

/// Returns the current number of queued bytes.
///
/// # Safety
/// `rb` must be either null or a valid pointer to a [`ByteQueue`] for the
/// duration of this call.
///
/// If non-null, the pointed-to bytequeue must not be aliased for mutable
/// access elsewhere.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bytequeue_num(rb: *const RustByteQueue) -> u32 {
    let Some(rb) = (unsafe { bytequeue_ref(rb) }) else {
        return 0;
    };
    rb.num()
}

/// Removes all queued bytes.
///
/// # Safety
/// `rb` must be either null or a valid pointer to a [`ByteQueue`] for the
/// duration of this call. If non-null, the pointed-to bytequeue must not be
/// aliased for mutable access elsewhere.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bytequeue_flush(rb: *mut RustByteQueue) {
    if let Some(rb) = unsafe { bytequeue_mut(rb) } {
        rb.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::ptr;

    struct TestByteQueue {
        ptr: *mut RustByteQueue,
    }

    impl TestByteQueue {
        fn new() -> Self {
            Self::with_capacity(16)
        }

        fn with_capacity(capacity: usize) -> Self {
            let ptr = rust_bytequeue_init(capacity);
            assert!(!ptr.is_null());
            Self { ptr }
        }
    }

    impl Drop for TestByteQueue {
        fn drop(&mut self) {
            unsafe {
                rust_bytequeue_free(self.ptr);
            }
        }
    }

    #[test]
    fn test_bytequeue() {
        let mut rb = ByteQueue::with_capacity(2);
        assert_eq!(rb.num(), 0);

        rb.put(1);
        rb.put(2);
        assert_eq!(rb.num(), 2);
        assert_eq!(rb.get(), Some(1));
        assert_eq!(rb.get(), Some(2));
        assert_eq!(rb.get(), None);
        assert_eq!(rb.num(), 0);
    }

    #[test]
    fn test_bytequeue_flush() {
        let mut rb = ByteQueue::with_capacity(2);
        rb.put(1);
        rb.put(2);
        rb.flush();
        assert_eq!(rb.num(), 0);
        assert_eq!(rb.get(), None);
    }

    #[test]
    #[should_panic]
    fn test_bytequeue_put_overflow_panics() {
        let mut rb = ByteQueue::with_capacity(1);
        rb.put(1);
        rb.put(2);
    }

    #[test]
    fn test_rust_bytequeue_init_free() {
        let rb = rust_bytequeue_init(0);
        assert!(!rb.is_null());
        assert!(unsafe { rust_bytequeue_free(rb) });
    }

    #[test]
    fn test_rust_bytequeue_put_get_fifo() {
        let rb = TestByteQueue::new();
        unsafe {
            rust_bytequeue_put(rb.ptr, 1);
            rust_bytequeue_put(rb.ptr, 2);
            rust_bytequeue_put(rb.ptr, 3);

            let mut out = 0;
            assert!(rust_bytequeue_get(rb.ptr, &mut out));
            assert_eq!(out, 1);

            assert!(rust_bytequeue_get(rb.ptr, &mut out));
            assert_eq!(out, 2);

            assert!(rust_bytequeue_get(rb.ptr, &mut out));
            assert_eq!(out, 3);
        }
    }

    #[test]
    fn test_rust_bytequeue_get_empty() {
        let rb = TestByteQueue::new();
        let mut out = 0;
        assert!(!unsafe { rust_bytequeue_get(rb.ptr, &mut out) });
    }

    #[test]
    fn test_rust_bytequeue_num() {
        let rb = TestByteQueue::new();
        unsafe {
            assert_eq!(rust_bytequeue_num(rb.ptr), 0);
            rust_bytequeue_put(rb.ptr, 0x11);
            rust_bytequeue_put(rb.ptr, 0x22);
            assert_eq!(rust_bytequeue_num(rb.ptr), 2);

            let mut out = 0;
            assert!(rust_bytequeue_get(rb.ptr, &mut out));
            assert_eq!(out, 0x11);
            assert_eq!(rust_bytequeue_num(rb.ptr), 1);
        }
    }

    #[test]
    fn test_rust_bytequeue_flush() {
        let rb = TestByteQueue::new();
        unsafe {
            rust_bytequeue_put(rb.ptr, 1);
            rust_bytequeue_put(rb.ptr, 2);
            assert_eq!(rust_bytequeue_num(rb.ptr), 2);

            rust_bytequeue_flush(rb.ptr);
            assert_eq!(rust_bytequeue_num(rb.ptr), 0);

            let mut out = 0;
            assert!(!rust_bytequeue_get(rb.ptr, &mut out));
        }
    }

    #[test]
    fn test_rust_bytequeue_dynamic_growth() {
        let rb = TestByteQueue::with_capacity(4096);
        unsafe {
            for i in 0..4096usize {
                rust_bytequeue_put(rb.ptr, (i % 251) as u8);
            }
            assert_eq!(rust_bytequeue_num(rb.ptr), 4096);

            for i in 0..4096usize {
                let mut out = 0;
                assert!(rust_bytequeue_get(rb.ptr, &mut out));
                assert_eq!(out, (i % 251) as u8);
            }
            assert_eq!(rust_bytequeue_num(rb.ptr), 0);
        }
    }

    #[test]
    fn test_rust_bytequeue_null_pointer_handling() {
        unsafe {
            rust_bytequeue_put(ptr::null_mut(), 1);
            rust_bytequeue_flush(ptr::null_mut());
            assert_eq!(rust_bytequeue_num(ptr::null()), 0);

            let mut out = 0;
            assert!(!rust_bytequeue_get(ptr::null_mut(), &mut out));
            assert!(!rust_bytequeue_get(ptr::null_mut(), ptr::null_mut()));
            assert!(!rust_bytequeue_free(ptr::null_mut()));
        }

        let rb = TestByteQueue::new();
        assert!(!unsafe { rust_bytequeue_get(rb.ptr, ptr::null_mut()) });
    }
}
