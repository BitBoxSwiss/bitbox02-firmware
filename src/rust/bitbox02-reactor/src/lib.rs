#![no_std]

extern crate alloc;

use alloc::boxed::Box;
//use alloc::{sync::Arc, task::Wake};
use core::ffi::c_void;
use core::pin::Pin;
use core::task::{Context, Poll};
use embedded_io_async::{Error, ErrorKind, ErrorType, Read as AsyncRead};

#[derive(Debug)]
struct ASF4UsbError {}

impl core::fmt::Display for ASF4UsbError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ASF4UsbError")
    }
}

impl core::error::Error for ASF4UsbError {}

impl Error for ASF4UsbError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

struct USBReportRead<'a> {
    buf: &'a mut [u8],
}

impl ErrorType for USBReportRead<'_> {
    type Error = ASF4UsbError;
}

impl Future for USBReportRead<'_> {
    type Output = Result<usize, <Self as ErrorType>::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker().clone();
        hid_hww_read_inner(self.buf, || waker.wake_by_ref());
        Poll::Ready(Ok(0))
    }
}

struct ASF4UsbDevice {}

impl ErrorType for ASF4UsbDevice {
    type Error = ASF4UsbError;
}

impl AsyncRead for ASF4UsbDevice {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let size = USBReportRead { buf }.await.unwrap();
        Ok(size)
    }
}

unsafe extern "C" {
    fn hid_hww_read(
        buf: *mut c_void,
        callback: unsafe extern "C" fn(user_data: *const c_void),
        user_data: *const c_void,
    );
}

// Initiate a read
fn hid_hww_read_inner<CB>(buf: &mut [u8], read_cb: CB)
where
    CB: FnOnce() -> (),
{
    // This callback is executed in interrupt context so be careful
    unsafe extern "C" fn c_read_cb<CB>(user_data: *const c_void)
    where
        CB: FnOnce() -> (),
    {
        let callback = unsafe { Box::from_raw(user_data as *mut CB) };
        (*callback)();
    }

    unsafe {
        hid_hww_read(
            buf.as_ptr() as _,
            c_read_cb::<CB>,
            Box::into_raw(Box::new(read_cb)) as *mut c_void,
        );
    }
}
//
//struct Inner {}
//
//struct Waker {
//    inner: Arc<Inner>,
//}
//
//impl From<Waker> for core::task::Waker {
//    fn from(waker: Waker) -> Self {
//        core::task::Waker::from(waker.inner)
//    }
//}
//
//impl Wake for Inner {
//    fn wake(self: Arc<Self>) {}
//}
