// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::VecDeque;

const USB_REPORT_SIZE: usize = 64;
// Keep this in sync with USB_DATA_MAX_LEN in src/usb/usb_frame.h.
const USB_DATA_MAX_LEN: usize = 7609;
const USB_REPORT_QUEUE_NUM_REPORTS: usize = USB_DATA_MAX_LEN / USB_REPORT_SIZE;
// Preserve the previous effective capacity of the manual ring buffer, which
// kept one slot empty to distinguish full from empty.
const USB_REPORT_QUEUE_MAX_LEN: usize = USB_REPORT_QUEUE_NUM_REPORTS - 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum UsbReportQueueError {
    USB_REPORT_QUEUE_ERR_NONE = 0,
    USB_REPORT_QUEUE_ERR_FULL = 1,
}

type UsbReport = [u8; USB_REPORT_SIZE];

pub struct UsbReportQueue {
    reports: VecDeque<UsbReport>,
}

#[repr(C)]
pub struct RustUsbReportQueue {
    _private: [u8; 0],
}

impl UsbReportQueue {
    pub const fn new() -> Self {
        Self {
            reports: VecDeque::new(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut RustUsbReportQueue {
        (self as *mut Self).cast::<RustUsbReportQueue>()
    }

    pub fn clear(&mut self) {
        self.reports.clear();
    }

    pub fn push(&mut self, report: &[u8; USB_REPORT_SIZE]) -> UsbReportQueueError {
        if self.reports.len() >= USB_REPORT_QUEUE_MAX_LEN {
            return UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL;
        }
        self.reports.push_back(*report);
        UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
    }

    pub fn pull(&mut self) -> Option<[u8; USB_REPORT_SIZE]> {
        self.reports.pop_front()
    }

    pub fn peek(&self) -> Option<[u8; USB_REPORT_SIZE]> {
        self.reports.front().copied()
    }
}

impl Default for UsbReportQueue {
    fn default() -> Self {
        Self::new()
    }
}

unsafe fn queue_mut<'a>(queue: *mut RustUsbReportQueue) -> Option<&'a mut UsbReportQueue> {
    if queue.is_null() {
        return None;
    }
    Some(unsafe { &mut *queue.cast::<UsbReportQueue>() })
}

unsafe fn queue_ref<'a>(queue: *const RustUsbReportQueue) -> Option<&'a UsbReportQueue> {
    if queue.is_null() {
        return None;
    }
    Some(unsafe { &*queue.cast::<UsbReportQueue>() })
}

/// Allocates a new USB report queue and returns an opaque handle.
///
/// The returned pointer must be freed with [`rust_usb_report_queue_free`].
#[unsafe(no_mangle)]
pub extern "C" fn rust_usb_report_queue_init() -> *mut RustUsbReportQueue {
    Box::into_raw(Box::new(UsbReportQueue::new())).cast::<RustUsbReportQueue>()
}

/// Frees a USB report queue previously created by [`rust_usb_report_queue_init`].
///
/// # Safety
/// If `queue` is non-null, it must be a pointer returned by
/// [`rust_usb_report_queue_init`] that has not already been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_free(queue: *mut RustUsbReportQueue) -> bool {
    if queue.is_null() {
        return false;
    }

    unsafe { drop(Box::from_raw(queue.cast::<UsbReportQueue>())) };
    true
}

/// Clears the given USB report queue.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_init`] or [`UsbReportQueue::as_mut_ptr`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_clear(queue: *mut RustUsbReportQueue) {
    if let Some(queue) = unsafe { queue_mut(queue) } {
        queue.clear();
    }
}

/// Pushes one 64-byte USB report to the queue.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_init`] or [`UsbReportQueue::as_mut_ptr`].
///
/// `report` must point to 64 readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_push(
    queue: *mut RustUsbReportQueue,
    report: *const u8,
) -> UsbReportQueueError {
    if report.is_null() {
        return UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL;
    }
    let report = unsafe { &*report.cast::<UsbReport>() };
    unsafe { queue_mut(queue) }
        .map(|queue| queue.push(report))
        .unwrap_or(UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL)
}

/// Pulls one 64-byte USB report from the queue into `report_out`.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_init`] or [`UsbReportQueue::as_mut_ptr`].
///
/// `report_out` must point to 64 writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_pull(
    queue: *mut RustUsbReportQueue,
    report_out: *mut u8,
) -> bool {
    if report_out.is_null() {
        return false;
    }
    let Some(report) = unsafe { queue_mut(queue) }.and_then(UsbReportQueue::pull) else {
        return false;
    };
    unsafe { *report_out.cast::<UsbReport>() = report };
    true
}

/// Copies the next 64-byte USB report into `report_out` without consuming it.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_init`] or [`UsbReportQueue::as_mut_ptr`].
///
/// `report_out` must point to 64 writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_peek(
    queue: *const RustUsbReportQueue,
    report_out: *mut u8,
) -> bool {
    if report_out.is_null() {
        return false;
    }
    let Some(report) = unsafe { queue_ref(queue) }.and_then(UsbReportQueue::peek) else {
        return false;
    };
    unsafe { *report_out.cast::<UsbReport>() = report };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::ptr;

    fn report(fill: u8) -> UsbReport {
        [fill; USB_REPORT_SIZE]
    }

    #[test]
    fn test_push_pull_fifo() {
        let mut queue = UsbReportQueue::new();

        assert!(matches!(
            queue.push(&report(1)),
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        ));
        assert!(matches!(
            queue.push(&report(2)),
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        ));
        assert_eq!(queue.pull().unwrap(), report(1));
        assert_eq!(queue.pull().unwrap(), report(2));
        assert!(queue.pull().is_none());
    }

    #[test]
    fn test_pull_empty() {
        let mut queue = UsbReportQueue::new();
        assert!(queue.pull().is_none());
    }

    #[test]
    fn test_peek_does_not_consume() {
        let mut queue = UsbReportQueue::new();
        assert!(matches!(
            queue.push(&report(3)),
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        ));
        assert_eq!(queue.peek().unwrap(), report(3));
        assert_eq!(queue.pull().unwrap(), report(3));
    }

    #[test]
    fn test_clear_empties_queue() {
        let mut queue = UsbReportQueue::new();
        assert!(matches!(
            queue.push(&report(4)),
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        ));
        queue.clear();
        assert!(queue.pull().is_none());
    }

    #[test]
    fn test_overflow_returns_full() {
        let mut queue = UsbReportQueue::new();

        for i in 0..USB_REPORT_QUEUE_MAX_LEN {
            assert!(matches!(
                queue.push(&report((i % 251) as u8)),
                UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            ));
        }

        assert!(matches!(
            queue.push(&report(0xff)),
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL
        ));
    }

    #[test]
    fn test_wraparound_fifo_order() {
        let mut queue = UsbReportQueue::new();

        for i in 0..USB_REPORT_QUEUE_MAX_LEN {
            assert!(matches!(
                queue.push(&report((i % 251) as u8)),
                UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            ));
        }

        for i in 0..16 {
            assert_eq!(queue.pull().unwrap(), report((i % 251) as u8));
        }

        for i in 0..16 {
            assert!(matches!(
                queue.push(&report((200 + i) as u8)),
                UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            ));
        }

        for i in 16..USB_REPORT_QUEUE_MAX_LEN {
            assert_eq!(queue.pull().unwrap(), report((i % 251) as u8));
        }

        for i in 0..16 {
            assert_eq!(queue.pull().unwrap(), report((200 + i) as u8));
        }
    }

    #[test]
    fn test_null_ffi_arguments() {
        let mut out = [0u8; USB_REPORT_SIZE];
        let mut queue = UsbReportQueue::new();
        let queue = queue.as_mut_ptr();
        assert!(matches!(
            unsafe { rust_usb_report_queue_push(ptr::null_mut(), report(7).as_ptr()) },
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL
        ));
        assert!(matches!(
            unsafe { rust_usb_report_queue_push(queue, ptr::null()) },
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL
        ));
        unsafe {
            rust_usb_report_queue_clear(ptr::null_mut());
        }
        assert!(!unsafe { rust_usb_report_queue_pull(ptr::null_mut(), out.as_mut_ptr()) });
        assert!(!unsafe { rust_usb_report_queue_pull(queue, ptr::null_mut()) });
        assert!(!unsafe { rust_usb_report_queue_peek(ptr::null(), out.as_mut_ptr()) });
        assert!(!unsafe { rust_usb_report_queue_peek(queue, ptr::null_mut()) });
    }

    #[test]
    fn test_hww_and_u2f_are_independent() {
        let mut hww_queue = UsbReportQueue::new();
        let mut u2f_queue = UsbReportQueue::new();
        let hww = hww_queue.as_mut_ptr();
        let u2f = u2f_queue.as_mut_ptr();
        let mut out = [0u8; USB_REPORT_SIZE];

        unsafe {
            assert!(matches!(
                rust_usb_report_queue_push(hww, report(0x11).as_ptr()),
                UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            ));
            assert!(matches!(
                rust_usb_report_queue_push(u2f, report(0x22).as_ptr()),
                UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            ));

            assert!(rust_usb_report_queue_pull(hww, out.as_mut_ptr()));
            assert_eq!(out, report(0x11));

            assert!(rust_usb_report_queue_pull(u2f, out.as_mut_ptr()));
            assert_eq!(out, report(0x22));
        }
    }

    #[test]
    fn test_ffi_pull_and_peek() {
        let mut queue = UsbReportQueue::new();
        let queue = queue.as_mut_ptr();
        let pushed = report(9);
        let mut out = [0u8; USB_REPORT_SIZE];

        assert!(matches!(
            unsafe { rust_usb_report_queue_push(queue, pushed.as_ptr()) },
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        ));
        assert!(unsafe { rust_usb_report_queue_peek(queue, out.as_mut_ptr()) });
        assert_eq!(out, pushed);
        out = [0; USB_REPORT_SIZE];
        assert!(unsafe { rust_usb_report_queue_pull(queue, out.as_mut_ptr()) });
        assert_eq!(out, pushed);
        assert!(!unsafe { rust_usb_report_queue_pull(queue, out.as_mut_ptr()) });
    }

    #[test]
    fn test_rust_usb_report_queue_init_free() {
        let queue = rust_usb_report_queue_init();
        assert!(!queue.is_null());
        assert!(unsafe { rust_usb_report_queue_free(queue) });
        assert!(!unsafe { rust_usb_report_queue_free(ptr::null_mut()) });
    }
}
