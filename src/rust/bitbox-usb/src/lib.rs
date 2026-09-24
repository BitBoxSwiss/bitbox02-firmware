// SPDX-License-Identifier: Apache-2.0

//! HWW HID reports over USB. Wallet framing and command handling live above this layer.

#![no_std]

use core::cell::Cell;
use core::future::{Future, poll_fn};
use core::pin::pin;
use core::task::Poll;
use embassy_usb::class::hid::{self, HidReaderWriter};
use embassy_usb::driver::{Driver, EndpointError};
use embassy_usb::{Builder, Config, UsbDevice};

use bitbox_u2fhid::{U2fHid, VendorCommandHandler};

pub const REPORT_SIZE: usize = 64;
pub type Report = [u8; REPORT_SIZE];

// Keep the existing HWW usage page, usages, and unnumbered 64-byte reports from
// src/usb/class/usb_desc.h so host HID transports can use the same wire format.
const REPORT_DESCRIPTOR: &[u8] = &[
    0x06, 0xff, 0xff, // USAGE_PAGE (Vendor Defined)
    0x09, 0x01, // USAGE (HID Generic Device)
    0xa1, 0x01, // COLLECTION (Application)
    0x09, 0x20, // USAGE (Input Report Data)
    0x15, 0x00, // LOGICAL_MINIMUM (0)
    0x26, 0xff, 0x00, // LOGICAL_MAXIMUM (255)
    0x75, 0x08, // REPORT_SIZE (8)
    0x95, 0x40, // REPORT_COUNT (64)
    0x81, 0x02, // INPUT (Data,Var,Abs)
    0x09, 0x21, // USAGE (Output Report Data)
    0x15, 0x00, // LOGICAL_MINIMUM (0)
    0x26, 0xff, 0x00, // LOGICAL_MAXIMUM (255)
    0x75, 0x08, // REPORT_SIZE (8)
    0x95, 0x40, // REPORT_COUNT (64)
    0x91, 0x02, // OUTPUT (Data,Var,Abs)
    0xc0, // END_COLLECTION
];

/// Storage borrowed by the USB device for its entire lifetime. No heap is needed.
pub struct Buffers<'d> {
    config: [u8; 64],
    bos: [u8; 32],
    control: [u8; 256],
    hid: hid::State<'d>,
    session: Cell<u32>,
    handler: Option<SessionHandler<'d>>,
}

impl Default for Buffers<'_> {
    fn default() -> Self {
        Self {
            config: [0; 64],
            bos: [0; 32],
            control: [0; 256],
            hid: hid::State::new(),
            session: Cell::new(0),
            handler: None,
        }
    }
}

// Track lifecycle events even if reset and reconfiguration both happen between report polls.
// These callbacks run in UsbDevice::run(), on the same thread as report processing.
struct SessionHandler<'d>(&'d Cell<u32>);

impl embassy_usb::Handler for SessionHandler<'_> {
    fn reset(&mut self) {
        self.0.set(self.0.get().wrapping_add(1));
    }

    fn enabled(&mut self, enabled: bool) {
        if !enabled {
            self.reset();
        }
    }

    fn configured(&mut self, _configured: bool) {
        self.reset();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReadError {
    Disabled,
    InvalidReport,
}

pub struct HwwHid<'d, D: Driver<'d>> {
    inner: HidReaderWriter<'d, D, REPORT_SIZE, REPORT_SIZE>,
    session: &'d Cell<u32>,
}

/// Build a single HWW HID interface. Poll `UsbDevice::run()` concurrently with report I/O.
///
/// The serial string follows the existing BitBox convention of containing the firmware version.
pub fn new<'d, D: Driver<'d>>(
    driver: D,
    buffers: &'d mut Buffers<'d>,
    product: &'d str,
    version: &'d str,
) -> (UsbDevice<'d, D>, HwwHid<'d, D>) {
    // USB string descriptors have a one-byte length, including their two-byte header.
    assert!(product.encode_utf16().count() <= 126);
    assert!(version.encode_utf16().count() <= 126);
    let mut config = Config::new(0x03eb, 0x2403);
    config.bcd_usb = embassy_usb::UsbVersion::Two;
    config.max_speed = embassy_usb::UsbDeviceSpeed::Full;
    config.device_release = 0x0100;
    config.manufacturer = Some("bitbox.swiss");
    config.product = Some(product);
    config.serial_number = Some(version);
    config.device_class = 0;
    config.device_sub_class = 0;
    config.device_protocol = 0;
    config.composite_with_iads = false;
    config.max_packet_size_0 = REPORT_SIZE as u8;
    config.max_power = 100;
    config.supports_remote_wakeup = false;

    let mut builder = Builder::new(
        driver,
        config,
        &mut buffers.config,
        &mut buffers.bos,
        &mut [],
        &mut buffers.control,
    );
    buffers.handler = Some(SessionHandler(&buffers.session));
    builder.handler(buffers.handler.as_mut().unwrap());
    let inner = HidReaderWriter::new(
        &mut builder,
        &mut buffers.hid,
        hid::Config {
            report_descriptor: REPORT_DESCRIPTOR,
            request_handler: None,
            poll_ms: 4,
            max_packet_size: REPORT_SIZE as u16,
            hid_subclass: hid::HidSubclass::No,
            hid_boot_protocol: hid::HidBootProtocol::None,
        },
    );
    (
        builder.build(),
        HwwHid {
            inner,
            session: &buffers.session,
        },
    )
}

impl<'d, D: Driver<'d>> HwwHid<'d, D> {
    /// Wait for configuration by the host, including after a disconnect or reset.
    pub async fn ready(&mut self) {
        self.inner.ready().await;
    }

    /// Receive a complete report. Short packets never expose stale bytes to the caller.
    pub async fn read(&mut self) -> Result<Report, ReadError> {
        let mut report = [0; REPORT_SIZE];
        match self.inner.read(&mut report).await {
            Ok(REPORT_SIZE) => Ok(report),
            Err(hid::ReadError::Disabled) => Err(ReadError::Disabled),
            _ => Err(ReadError::InvalidReport),
        }
    }

    pub async fn write(&mut self, report: &Report) -> Result<(), EndpointError> {
        self.inner.write(report).await
    }

    /// Run HWW framing while USB is polled concurrently on the same thread.
    ///
    /// `now_ms` must be monotonic. `spin` polls the application's outstanding HWW task.
    /// This cooperatively polls timers and the task even while USB I/O is pending, as the
    /// existing HWW task executor does not register wakeups.
    pub async fn run<V: VendorCommandHandler>(
        &mut self,
        transport: &mut U2fHid<V>,
        mut now_ms: impl FnMut() -> u64,
        mut spin: impl FnMut(),
    ) -> ! {
        let session = self.session;
        loop {
            transport.reset();
            self.ready().await;
            let current_session = session.get();
            loop {
                let mut outgoing = None;
                let received = {
                    // A read is cancelled only for a session change or a timeout response.
                    // Each report fits in one endpoint packet, so no partial report is lost.
                    let mut read = pin!(self.read());
                    poll_fn(|cx| {
                        if session.get() != current_session {
                            return Poll::Ready(Err(ReadError::Disabled));
                        }
                        transport.tick(now_ms());
                        spin();
                        cx.waker().wake_by_ref();
                        outgoing = transport.pull_report();
                        if outgoing.is_some() {
                            return Poll::Ready(Ok(None));
                        }
                        read.as_mut().poll(cx).map(|result| result.map(Some))
                    })
                    .await
                };
                match received {
                    Ok(Some(report)) => transport.handle_report(&report, now_ms()),
                    Ok(None) => {}
                    Err(ReadError::InvalidReport) => continue,
                    Err(ReadError::Disabled) => break,
                }
                if let Some(report) = outgoing {
                    let mut write = pin!(self.write(&report));
                    let result = poll_fn(|cx| {
                        if session.get() != current_session {
                            return Poll::Ready(Err(EndpointError::Disabled));
                        }
                        transport.tick(now_ms());
                        spin();
                        cx.waker().wake_by_ref();
                        write.as_mut().poll(cx)
                    })
                    .await;
                    if result.is_err() {
                        // Never retry an interrupted response in the next USB session.
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
