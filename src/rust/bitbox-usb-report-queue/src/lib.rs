// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::cell::UnsafeCell;

const USB_REPORT_SIZE: usize = 64;
const USB_DATA_MAX_LEN: usize = 7609;
const USB_REPORT_QUEUE_NUM_REPORTS: usize = USB_DATA_MAX_LEN / USB_REPORT_SIZE;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum UsbReportQueueError {
    USB_REPORT_QUEUE_ERR_NONE = 0,
    USB_REPORT_QUEUE_ERR_FULL = 1,
}

type UsbReport = [u8; USB_REPORT_SIZE];

pub struct UsbReportQueue {
    start: usize,
    end: usize,
    reports: [UsbReport; USB_REPORT_QUEUE_NUM_REPORTS],
}

#[repr(C)]
pub struct RustUsbReportQueue {
    _private: [u8; 0],
}

struct QueueCell {
    value: UnsafeCell<UsbReportQueue>,
}

unsafe impl Sync for QueueCell {}

impl QueueCell {
    const fn new() -> Self {
        Self {
            value: UnsafeCell::new(UsbReportQueue::new()),
        }
    }

    fn get(&self) -> *mut UsbReportQueue {
        self.value.get()
    }
}

static HWW_QUEUE: QueueCell = QueueCell::new();
static U2F_QUEUE: QueueCell = QueueCell::new();

impl UsbReportQueue {
    const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            reports: [[0; USB_REPORT_SIZE]; USB_REPORT_QUEUE_NUM_REPORTS],
        }
    }

    fn clear(&mut self) {
        self.reports = [[0; USB_REPORT_SIZE]; USB_REPORT_QUEUE_NUM_REPORTS];
        self.start = 0;
        self.end = 0;
    }

    fn push(&mut self, report: &UsbReport) -> UsbReportQueueError {
        let next = (self.end + 1) % USB_REPORT_QUEUE_NUM_REPORTS;
        if next == self.start {
            return UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL;
        }
        self.reports[self.end] = *report;
        self.end = next;
        UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
    }

    fn pull(&mut self) -> Option<UsbReport> {
        if self.start == self.end {
            return None;
        }
        let report = self.reports[self.start];
        self.start = (self.start + 1) % USB_REPORT_QUEUE_NUM_REPORTS;
        Some(report)
    }

    fn peek(&self) -> Option<UsbReport> {
        if self.start == self.end {
            return None;
        }
        Some(self.reports[self.start])
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

fn with_queue_mut<T>(
    queue: *mut RustUsbReportQueue,
    f: impl FnOnce(&mut UsbReportQueue) -> T,
) -> Option<T> {
    critical_section(|| {
        let queue = unsafe { queue_mut(queue) }?;
        Some(f(queue))
    })
}

fn with_queue_ref<T>(
    queue: *const RustUsbReportQueue,
    f: impl FnOnce(&UsbReportQueue) -> T,
) -> Option<T> {
    critical_section(|| {
        let queue = unsafe { queue_ref(queue) }?;
        Some(f(queue))
    })
}

#[cfg(target_arch = "arm")]
fn critical_section<T>(f: impl FnOnce() -> T) -> T {
    cortex_m::interrupt::free(|_| f())
}

#[cfg(not(target_arch = "arm"))]
fn critical_section<T>(f: impl FnOnce() -> T) -> T {
    f()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_usb_report_queue_hww() -> *mut RustUsbReportQueue {
    HWW_QUEUE.get().cast::<RustUsbReportQueue>()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_usb_report_queue_u2f() -> *mut RustUsbReportQueue {
    U2F_QUEUE.get().cast::<RustUsbReportQueue>()
}

/// Clears the given USB report queue.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_hww`] or [`rust_usb_report_queue_u2f`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_report_queue_clear(queue: *mut RustUsbReportQueue) {
    let _ = with_queue_mut(queue, UsbReportQueue::clear);
}

/// Pushes one 64-byte USB report to the queue.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_hww`] or [`rust_usb_report_queue_u2f`].
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
    with_queue_mut(queue, |queue| queue.push(report))
        .unwrap_or(UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL)
}

/// Pulls one 64-byte USB report from the queue into `report_out`.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_hww`] or [`rust_usb_report_queue_u2f`].
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
    let Some(report) = with_queue_mut(queue, UsbReportQueue::pull).flatten() else {
        return false;
    };
    unsafe { *report_out.cast::<UsbReport>() = report };
    true
}

/// Copies the next 64-byte USB report into `report_out` without consuming it.
///
/// # Safety
/// `queue` must be null or a valid queue returned by
/// [`rust_usb_report_queue_hww`] or [`rust_usb_report_queue_u2f`].
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
    let Some(report) = with_queue_ref(queue, UsbReportQueue::peek).flatten() else {
        return false;
    };
    unsafe { *report_out.cast::<UsbReport>() = report };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::ptr;

    fn queue_handle(queue: &mut UsbReportQueue) -> *mut RustUsbReportQueue {
        (queue as *mut UsbReportQueue).cast::<RustUsbReportQueue>()
    }

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

        for i in 0..(USB_REPORT_QUEUE_NUM_REPORTS - 1) {
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

        for i in 0..(USB_REPORT_QUEUE_NUM_REPORTS - 1) {
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

        for i in 16..(USB_REPORT_QUEUE_NUM_REPORTS - 1) {
            assert_eq!(queue.pull().unwrap(), report((i % 251) as u8));
        }

        for i in 0..16 {
            assert_eq!(queue.pull().unwrap(), report((200 + i) as u8));
        }
    }

    #[test]
    fn test_null_ffi_arguments() {
        let mut out = [0u8; USB_REPORT_SIZE];
        assert!(matches!(
            unsafe { rust_usb_report_queue_push(ptr::null_mut(), report(7).as_ptr()) },
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL
        ));
        assert!(matches!(
            unsafe { rust_usb_report_queue_push(rust_usb_report_queue_hww(), ptr::null()) },
            UsbReportQueueError::USB_REPORT_QUEUE_ERR_FULL
        ));
        unsafe {
            rust_usb_report_queue_clear(ptr::null_mut());
        }
        assert!(!unsafe { rust_usb_report_queue_pull(ptr::null_mut(), out.as_mut_ptr()) });
        assert!(!unsafe {
            rust_usb_report_queue_pull(rust_usb_report_queue_hww(), ptr::null_mut())
        });
        assert!(!unsafe { rust_usb_report_queue_peek(ptr::null(), out.as_mut_ptr()) });
        assert!(!unsafe {
            rust_usb_report_queue_peek(rust_usb_report_queue_hww(), ptr::null_mut())
        });
    }

    #[test]
    fn test_hww_and_u2f_are_independent() {
        let hww = rust_usb_report_queue_hww();
        let u2f = rust_usb_report_queue_u2f();
        let mut out = [0u8; USB_REPORT_SIZE];

        unsafe {
            rust_usb_report_queue_clear(hww);
            rust_usb_report_queue_clear(u2f);

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

            rust_usb_report_queue_clear(hww);
            rust_usb_report_queue_clear(u2f);
        }
    }

    #[test]
    fn test_ffi_pull_and_peek() {
        let mut queue = UsbReportQueue::new();
        let queue = queue_handle(&mut queue);
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
}
