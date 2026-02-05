// SPDX-License-Identifier: Apache-2.0

use bitbox02_sys::{ringbuffer, ringbuffer_init};
use core::marker::PhantomData;

/// A wrapper around ASF4 `ringbuffer` type
pub struct RingBuffer<'a> {
    pub(crate) inner: ringbuffer,
    _marker: PhantomData<&'a mut [u8]>,
}

impl<'a> RingBuffer<'a> {
    /// `buf` length must be a power of 2
    pub fn new(buf: &'a mut [u8]) -> Self {
        debug_assert!(buf.len().is_power_of_two());
        let mut inner = ringbuffer {
            buf: core::ptr::null_mut(),
            size: 0,
            read_index: 0,
            write_index: 0,
        };
        unsafe {
            ringbuffer_init(
                &mut inner as *mut _,
                buf as *mut _ as *mut _,
                buf.len() as u32,
            );
        };
        RingBuffer {
            inner,
            _marker: PhantomData,
        }
    }

    /// Bytes currently used
    pub fn len(&self) -> u32 {
        unsafe { bitbox02_sys::ringbuffer_num(&self.inner as *const _) }
    }
}

// These are currently only used in unit tests.
#[cfg(test)]
impl RingBuffer<'_> {
    pub fn put(&mut self, data: u8) -> Result<(), i32> {
        let result = unsafe { bitbox02_sys::ringbuffer_put(&mut self.inner as *mut _, data) };
        if result == 0 { Ok(()) } else { Err(result) }
    }

    pub fn get(&mut self) -> Result<u8, i32> {
        let mut out = 0u8;
        let result = unsafe { bitbox02_sys::ringbuffer_get(&mut self.inner as *mut _, &mut out) };
        if result == 0 { Ok(out) } else { Err(result) }
    }

    pub fn flush(&mut self) -> Result<(), u32> {
        let result = unsafe { bitbox02_sys::ringbuffer_flush(&mut self.inner as *mut _) };
        if result == 0 { Ok(()) } else { Err(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_len_is_zero() {
        let mut buf = [0u8; 8];
        let rb = RingBuffer::new(&mut buf);
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_put_get_len() {
        let mut buf = [0u8; 8];
        let mut rb = RingBuffer::new(&mut buf);

        rb.put(1).unwrap();
        assert_eq!(rb.len(), 1);

        rb.put(2).unwrap();
        assert_eq!(rb.len(), 2);

        let out = rb.get().unwrap();
        assert_eq!(out, 1);
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn test_overwrite_oldest() {
        // Buf len must be a power of 2, and the ringbuffer capacity is `buf.len()`.
        let mut buf = [0u8; 8];
        let mut rb = RingBuffer::new(&mut buf);

        for i in 0u8..9 {
            rb.put(i).unwrap();
        }
        assert_eq!(rb.len(), 8);

        let out = rb.get().unwrap();
        assert_eq!(out, 1);
    }

    #[test]
    fn test_get_empty_returns_error() {
        let mut buf = [0u8; 8];
        let mut rb = RingBuffer::new(&mut buf);

        let result = rb.get();
        assert!(result.is_err());
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_flush() {
        let mut buf = [0u8; 8];
        let mut rb = RingBuffer::new(&mut buf);

        rb.put(1).unwrap();
        rb.put(2).unwrap();
        assert_eq!(rb.len(), 2);

        rb.flush().unwrap();
        assert_eq!(rb.len(), 0);
    }
}
