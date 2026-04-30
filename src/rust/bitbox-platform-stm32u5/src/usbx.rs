// SPDX-License-Identifier: Apache-2.0

use core::cell::RefCell;
use core::future::poll_fn;
use core::task::{Context, Poll, Waker};

const CUSTOM_HID_OUT_ADDR: u8 = 0x02;
const CUSTOM_HID_IN_ADDR: u8 = 0x81;
const CUSTOM_HID_MAX_PACKET_SIZE: u16 = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndpointInfo {
    pub addr: u8,
    pub max_packet_size: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointError {
    BufferOverflow,
    Disabled,
}

#[allow(async_fn_in_trait)]
pub trait Endpoint {
    fn info(&self) -> &EndpointInfo;

    async fn wait_enabled(&mut self);
}

#[allow(async_fn_in_trait)]
pub trait EndpointOut: Endpoint {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError>;
}

#[allow(async_fn_in_trait)]
pub trait EndpointIn: Endpoint {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError>;
}

pub struct CustomHidOut {
    info: EndpointInfo,
}

pub struct CustomHidIn {
    info: EndpointInfo,
}

pub fn custom_hid() -> (CustomHidOut, CustomHidIn) {
    (
        CustomHidOut {
            info: EndpointInfo {
                addr: CUSTOM_HID_OUT_ADDR,
                max_packet_size: CUSTOM_HID_MAX_PACKET_SIZE,
            },
        },
        CustomHidIn {
            info: EndpointInfo {
                addr: CUSTOM_HID_IN_ADDR,
                max_packet_size: CUSTOM_HID_MAX_PACKET_SIZE,
            },
        },
    )
}

#[derive(Default)]
struct Waiters {
    enabled: Option<Waker>,
    read: Option<Waker>,
    write: Option<Waker>,
}

struct SafeWaiters(RefCell<Waiters>);

unsafe impl Sync for SafeWaiters {}

static WAITERS: SafeWaiters = SafeWaiters(RefCell::new(Waiters {
    enabled: None,
    read: None,
    write: None,
}));

fn register_waker(slot: &mut Option<Waker>, cx: &Context<'_>) {
    if slot
        .as_ref()
        .is_none_or(|registered| !registered.will_wake(cx.waker()))
    {
        *slot = Some(cx.waker().clone());
    }
}

fn wake(slot: &mut Option<Waker>) {
    if let Some(waker) = slot.take() {
        waker.wake();
    }
}

pub fn poll() {
    let enabled = imp::enabled();
    let read_ready = enabled && imp::read_ready();
    let can_write = enabled && imp::can_write();

    let mut waiters = WAITERS.0.borrow_mut();
    if enabled {
        wake(&mut waiters.enabled);
    }
    if !enabled || read_ready {
        wake(&mut waiters.read);
    }
    if !enabled || can_write {
        wake(&mut waiters.write);
    }
}

pub fn process() {
    unsafe {
        crate::ffi::USBX_Device_Process();
    }
}

impl Endpoint for CustomHidOut {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        poll_fn(|cx| {
            if imp::enabled() {
                Poll::Ready(())
            } else {
                register_waker(&mut WAITERS.0.borrow_mut().enabled, cx);
                Poll::Pending
            }
        })
        .await
    }
}

impl EndpointOut for CustomHidOut {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        poll_fn(|cx| match imp::read(buf) {
            ReadResult::Ok(len) => Poll::Ready(Ok(len)),
            ReadResult::Empty => {
                register_waker(&mut WAITERS.0.borrow_mut().read, cx);
                Poll::Pending
            }
            ReadResult::Overflow => Poll::Ready(Err(EndpointError::BufferOverflow)),
            ReadResult::Disabled => Poll::Ready(Err(EndpointError::Disabled)),
        })
        .await
    }
}

impl Endpoint for CustomHidIn {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        poll_fn(|cx| {
            if imp::enabled() {
                Poll::Ready(())
            } else {
                register_waker(&mut WAITERS.0.borrow_mut().enabled, cx);
                Poll::Pending
            }
        })
        .await
    }
}

impl EndpointIn for CustomHidIn {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        if buf.is_empty() || buf.len() > CUSTOM_HID_MAX_PACKET_SIZE as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let mut submitted = false;
        poll_fn(|cx| {
            if !imp::enabled() {
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            if !submitted {
                if !imp::can_write() {
                    register_waker(&mut WAITERS.0.borrow_mut().write, cx);
                    return Poll::Pending;
                }

                match imp::write(buf) {
                    WriteResult::Submitted => {
                        submitted = true;
                    }
                    WriteResult::Busy => {
                        register_waker(&mut WAITERS.0.borrow_mut().write, cx);
                        return Poll::Pending;
                    }
                    WriteResult::Disabled => {
                        return Poll::Ready(Err(EndpointError::Disabled));
                    }
                }
            }

            if !imp::enabled() {
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            if imp::can_write() {
                Poll::Ready(Ok(()))
            } else {
                register_waker(&mut WAITERS.0.borrow_mut().write, cx);
                Poll::Pending
            }
        })
        .await
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReadResult {
    Ok(usize),
    Empty,
    Overflow,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WriteResult {
    Submitted,
    Busy,
    Disabled,
}

#[cfg(not(test))]
mod imp {
    use super::{ReadResult, WriteResult};
    use crate::ffi;

    pub fn enabled() -> bool {
        unsafe { ffi::bitbox_usbx_custom_hid_enabled() != 0 }
    }

    pub fn read_ready() -> bool {
        unsafe { ffi::bitbox_usbx_custom_hid_read_ready() != 0 }
    }

    pub fn can_write() -> bool {
        unsafe { ffi::bitbox_usbx_custom_hid_can_write() != 0 }
    }

    pub fn read(buf: &mut [u8]) -> ReadResult {
        let mut out_length = 0;
        match unsafe {
            ffi::bitbox_usbx_custom_hid_read(buf.as_mut_ptr(), buf.len() as _, &mut out_length)
        } {
            status if status == ffi::BITBOX_USBX_CUSTOM_HID_READ_OK => {
                ReadResult::Ok(out_length as usize)
            }
            status if status == ffi::BITBOX_USBX_CUSTOM_HID_READ_EMPTY => ReadResult::Empty,
            status if status == ffi::BITBOX_USBX_CUSTOM_HID_READ_OVERFLOW => ReadResult::Overflow,
            _ => ReadResult::Disabled,
        }
    }

    pub fn write(buf: &[u8]) -> WriteResult {
        if !enabled() {
            return WriteResult::Disabled;
        }
        if unsafe { ffi::bitbox_usbx_custom_hid_write(buf.as_ptr(), buf.len() as _) }
            == ffi::UX_SUCCESS
        {
            WriteResult::Submitted
        } else if enabled() {
            WriteResult::Busy
        } else {
            WriteResult::Disabled
        }
    }
}

#[cfg(test)]
mod imp {
    use super::{ReadResult, WriteResult};
    use core::cell::RefCell;

    struct MockState {
        enabled: bool,
        read_buf: [u8; 64],
        read_len: usize,
        read_ready: bool,
        can_write: bool,
        write_calls: usize,
    }

    struct SafeMockState(RefCell<MockState>);

    unsafe impl Sync for SafeMockState {}

    static STATE: SafeMockState = SafeMockState(RefCell::new(MockState {
        enabled: false,
        read_buf: [0; 64],
        read_len: 0,
        read_ready: false,
        can_write: true,
        write_calls: 0,
    }));

    pub fn enabled() -> bool {
        STATE.0.borrow().enabled
    }

    pub fn read_ready() -> bool {
        let state = STATE.0.borrow();
        state.enabled && state.read_ready
    }

    pub fn can_write() -> bool {
        let state = STATE.0.borrow();
        state.enabled && state.can_write
    }

    pub fn read(buf: &mut [u8]) -> ReadResult {
        let mut state = STATE.0.borrow_mut();
        if !state.enabled {
            return ReadResult::Disabled;
        }
        if !state.read_ready {
            return ReadResult::Empty;
        }
        state.read_ready = false;
        if buf.len() < state.read_len {
            return ReadResult::Overflow;
        }
        buf[..state.read_len].copy_from_slice(&state.read_buf[..state.read_len]);
        ReadResult::Ok(state.read_len)
    }

    pub fn write(_buf: &[u8]) -> WriteResult {
        let mut state = STATE.0.borrow_mut();
        if !state.enabled {
            return WriteResult::Disabled;
        }
        if !state.can_write {
            return WriteResult::Busy;
        }
        state.can_write = false;
        state.write_calls += 1;
        WriteResult::Submitted
    }

    #[cfg(test)]
    pub fn reset() {
        *STATE.0.borrow_mut() = MockState {
            enabled: false,
            read_buf: [0; 64],
            read_len: 0,
            read_ready: false,
            can_write: true,
            write_calls: 0,
        };
    }

    #[cfg(test)]
    pub fn set_enabled(value: bool) {
        STATE.0.borrow_mut().enabled = value;
    }

    #[cfg(test)]
    pub fn push_read(data: &[u8]) {
        let mut state = STATE.0.borrow_mut();
        state.read_buf[..data.len()].copy_from_slice(data);
        state.read_len = data.len();
        state.read_ready = true;
    }

    #[cfg(test)]
    pub fn complete_write() {
        STATE.0.borrow_mut().can_write = true;
    }

    #[cfg(test)]
    pub fn write_calls() -> usize {
        STATE.0.borrow().write_calls
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::{CustomHidIn, CustomHidOut, Endpoint, EndpointError, EndpointIn, EndpointOut, imp};
    use core::pin::pin;
    use core::task::{Context, Poll};
    use std::boxed::Box;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    struct CountingWake {
        wake_count: AtomicUsize,
    }

    impl CountingWake {
        fn new() -> Self {
            Self {
                wake_count: AtomicUsize::new(0),
            }
        }

        fn wake_count(&self) -> usize {
            self.wake_count.load(Ordering::SeqCst)
        }
    }

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.wake_count.fetch_add(1, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.wake_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn context() -> (Arc<CountingWake>, Context<'static>) {
        let wake = Arc::new(CountingWake::new());
        let waker = std::task::Waker::from(Arc::clone(&wake));
        let leaked = Box::leak(Box::new(waker));
        (wake, Context::from_waker(leaked))
    }

    #[test]
    fn test_wait_enabled() {
        imp::reset();
        let mut ep = CustomHidOut {
            info: super::EndpointInfo {
                addr: 0x02,
                max_packet_size: 64,
            },
        };
        let mut future = pin!(ep.wait_enabled());
        let (wake, mut cx) = context();

        assert!(matches!(future.as_mut().poll(&mut cx), Poll::Pending));
        assert_eq!(wake.wake_count(), 0);

        imp::set_enabled(true);
        super::poll();
        assert_eq!(wake.wake_count(), 1);
        assert!(matches!(future.as_mut().poll(&mut cx), Poll::Ready(())));
    }

    #[test]
    fn test_read() {
        imp::reset();
        imp::set_enabled(true);
        imp::push_read(&[1, 2, 3]);

        let mut ep = CustomHidOut {
            info: super::EndpointInfo {
                addr: 0x02,
                max_packet_size: 64,
            },
        };
        let mut buf = [0u8; 8];
        let (_wake, mut cx) = context();

        let result = {
            let mut future = pin!(ep.read(&mut buf));
            future.as_mut().poll(&mut cx)
        };
        assert!(matches!(result, Poll::Ready(Ok(3))));
        assert_eq!(&buf[..3], &[1, 2, 3]);
    }

    #[test]
    fn test_read_overflow() {
        imp::reset();
        imp::set_enabled(true);
        imp::push_read(&[1, 2, 3]);

        let mut ep = CustomHidOut {
            info: super::EndpointInfo {
                addr: 0x02,
                max_packet_size: 64,
            },
        };
        let mut buf = [0u8; 2];
        let mut future = pin!(ep.read(&mut buf));
        let (_wake, mut cx) = context();

        assert!(matches!(
            future.as_mut().poll(&mut cx),
            Poll::Ready(Err(EndpointError::BufferOverflow))
        ));
    }

    #[test]
    fn test_write_waits_for_completion() {
        imp::reset();
        imp::set_enabled(true);

        let mut ep = CustomHidIn {
            info: super::EndpointInfo {
                addr: 0x81,
                max_packet_size: 64,
            },
        };
        let mut future = pin!(ep.write(&[1, 2, 3]));
        let (wake, mut cx) = context();

        assert!(matches!(future.as_mut().poll(&mut cx), Poll::Pending));
        assert_eq!(imp::write_calls(), 1);

        imp::complete_write();
        super::poll();
        assert_eq!(wake.wake_count(), 1);
        assert!(matches!(future.as_mut().poll(&mut cx), Poll::Ready(Ok(()))));
    }

    #[test]
    fn test_write_disabled() {
        imp::reset();

        let mut ep = CustomHidIn {
            info: super::EndpointInfo {
                addr: 0x81,
                max_packet_size: 64,
            },
        };
        let mut future = pin!(ep.write(&[1, 2, 3]));
        let (_wake, mut cx) = context();

        assert!(matches!(
            future.as_mut().poll(&mut cx),
            Poll::Ready(Err(EndpointError::Disabled))
        ));
    }

    #[test]
    fn test_pending_read_returns_disabled() {
        imp::reset();
        imp::set_enabled(true);

        let mut ep = CustomHidOut {
            info: super::EndpointInfo {
                addr: 0x02,
                max_packet_size: 64,
            },
        };
        let mut buf = [0u8; 8];
        let mut future = pin!(ep.read(&mut buf));
        let (wake, mut cx) = context();

        assert!(matches!(future.as_mut().poll(&mut cx), Poll::Pending));

        imp::set_enabled(false);
        super::poll();
        assert_eq!(wake.wake_count(), 1);
        assert!(matches!(
            future.as_mut().poll(&mut cx),
            Poll::Ready(Err(EndpointError::Disabled))
        ));
    }
}
