// SPDX-License-Identifier: Apache-2.0

extern crate std;

use super::*;
use core::future::{pending, poll_fn};
use core::task::Poll;
use embassy_futures::poll_once;
use embassy_usb::driver::{
    Bus, ControlPipe, Direction, Endpoint, EndpointAddress, EndpointAllocError, EndpointIn,
    EndpointInfo, EndpointOut, EndpointType, Event, Unsupported,
};
use hex_lit::hex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{vec, vec::Vec};

#[derive(Default)]
struct State {
    events: VecDeque<Event>,
    setup: VecDeque<[u8; 8]>,
    control_in: Vec<u8>,
    stalled: bool,
    enabled: bool,
    reads: VecDeque<Result<Vec<u8>, EndpointError>>,
    writes: Vec<Vec<u8>>,
    write_blocked: bool,
}

#[derive(Clone)]
struct FakeDriver(Rc<RefCell<State>>);

struct FakeEndpoint {
    info: EndpointInfo,
    state: Rc<RefCell<State>>,
}

impl<'d> Driver<'d> for FakeDriver {
    type EndpointOut = FakeEndpoint;
    type EndpointIn = FakeEndpoint;
    type ControlPipe = Self;
    type Bus = Self;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        _ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<FakeEndpoint, EndpointAllocError> {
        Ok(FakeEndpoint {
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(1, Direction::In),
                ep_type,
                max_packet_size,
                interval_ms,
            },
            state: self.0.clone(),
        })
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        _ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<FakeEndpoint, EndpointAllocError> {
        Ok(FakeEndpoint {
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(1, Direction::Out),
                ep_type,
                max_packet_size,
                interval_ms,
            },
            state: self.0.clone(),
        })
    }

    fn start(self, control_max_packet_size: u16) -> (Self, Self) {
        assert_eq!(control_max_packet_size, 64);
        (self.clone(), self)
    }
}

impl Bus for FakeDriver {
    async fn enable(&mut self) {}
    async fn disable(&mut self) {
        self.0.borrow_mut().enabled = false;
    }
    async fn poll(&mut self) -> Event {
        poll_fn(|_| {
            let mut state = self.0.borrow_mut();
            let Some(event) = state.events.pop_front() else {
                return Poll::Pending;
            };
            if matches!(event, Event::Reset | Event::PowerRemoved) {
                state.enabled = false;
                state.reads.clear();
            }
            Poll::Ready(event)
        })
        .await
    }
    fn endpoint_set_enabled(&mut self, _ep: EndpointAddress, enabled: bool) {
        self.0.borrow_mut().enabled = enabled;
    }
    fn endpoint_set_stalled(&mut self, _ep: EndpointAddress, stalled: bool) {
        self.0.borrow_mut().stalled = stalled;
    }
    fn endpoint_is_stalled(&mut self, _ep: EndpointAddress) -> bool {
        self.0.borrow().stalled
    }
    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}

impl Endpoint for FakeEndpoint {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }
    async fn wait_enabled(&mut self) {
        poll_fn(|_| {
            if self.state.borrow().enabled {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl EndpointOut for FakeEndpoint {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let packet = poll_fn(|_| {
            let mut state = self.state.borrow_mut();
            if !state.enabled {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            state.reads.pop_front().map_or(Poll::Pending, Poll::Ready)
        })
        .await?;
        if packet.len() > buf.len() {
            return Err(EndpointError::BufferOverflow);
        }
        buf[..packet.len()].copy_from_slice(&packet);
        Ok(packet.len())
    }
}

impl EndpointIn for FakeEndpoint {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        poll_fn(|_| {
            let mut state = self.state.borrow_mut();
            if !state.enabled {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            if state.write_blocked {
                return Poll::Pending;
            }
            state.writes.push(buf.to_vec());
            Poll::Ready(Ok(()))
        })
        .await
    }
}

impl ControlPipe for FakeDriver {
    fn max_packet_size(&self) -> usize {
        64
    }
    async fn setup(&mut self) -> [u8; 8] {
        poll_fn(|_| {
            self.0
                .borrow_mut()
                .setup
                .pop_front()
                .map_or(Poll::Pending, Poll::Ready)
        })
        .await
    }
    async fn data_out(
        &mut self,
        _buf: &mut [u8],
        _first: bool,
        _last: bool,
    ) -> Result<usize, EndpointError> {
        pending().await
    }
    async fn data_in(
        &mut self,
        data: &[u8],
        _first: bool,
        _last: bool,
    ) -> Result<(), EndpointError> {
        self.0.borrow_mut().control_in.extend_from_slice(data);
        Ok(())
    }
    async fn accept(&mut self) {}
    async fn reject(&mut self) {
        self.0.borrow_mut().stalled = true;
    }
    async fn accept_set_address(&mut self, _addr: u8) {}
}

fn driver() -> FakeDriver {
    let state = State {
        events: VecDeque::from([Event::PowerDetected, Event::Reset]),
        ..State::default()
    };
    FakeDriver(Rc::new(RefCell::new(state)))
}

fn request(
    usb: &mut UsbDevice<'_, FakeDriver>,
    state: &Rc<RefCell<State>>,
    setup: [u8; 8],
) -> Vec<u8> {
    state.borrow_mut().setup.push_back(setup);
    state.borrow_mut().stalled = false;
    assert!(poll_once(usb.run()).is_pending());
    assert!(!state.borrow().stalled);
    core::mem::take(&mut state.borrow_mut().control_in)
}

#[async_test::test]
async fn test_new_descriptors() {
    let driver = driver();
    let state = driver.0.clone();
    let mut buffers = Buffers::default();
    let (mut usb, _) = new(driver, &mut buffers, "BitBox03");

    let device = request(&mut usb, &state, hex!("8006000100001200"));
    assert_eq!(&device[4..8], &hex!("00000040"));
    assert_eq!(&device[8..12], &hex!("eb030324"));
    assert_eq!(device[16], 0); // no serial number descriptor
    let config = request(&mut usb, &state, hex!("8006000200004000"));
    assert_eq!(config.len(), 41);
    assert_eq!(config[4], 1); // one interface
    assert_eq!(&config[13..16], &hex!("020300")); // two endpoints, HID, no subclass
    assert_eq!(&config[27..], &hex!("0705810340000407050103400004"));
    let report = request(&mut usb, &state, hex!("8106002200004000"));
    assert_eq!(report, REPORT_DESCRIPTOR);
    let product = request(&mut usb, &state, hex!("8006020309044000"));
    assert_eq!(&product[2..], &hex!("42006900740042006f00780030003300"));
    // Full-speed-only devices must stall requests for the high-speed qualifier.
    state.borrow_mut().setup.push_back(hex!("8006000600000a00"));
    assert!(poll_once(usb.run()).is_pending());
    assert!(state.borrow().stalled);
    // SET_CONFIGURATION enables the interrupt endpoints.
    request(&mut usb, &state, hex!("0009010000000000"));
    assert!(state.borrow().enabled);
    usb.disable().await;
}

#[async_test::test]
async fn test_new_maximum_string_descriptor() {
    let driver = driver();
    let state = driver.0.clone();
    let product = "x".repeat(126);
    let mut buffers = Buffers::default();
    let (mut usb, _) = new(driver, &mut buffers, &product);
    let response = request(&mut usb, &state, hex!("800602030904ff00"));
    assert_eq!(response.len(), 254);
    assert_eq!(&response[..2], &hex!("fe03"));
    usb.disable().await;
}

#[async_test::test]
async fn test_read_rejects_incomplete_reports() {
    let driver = driver();
    let state = driver.0.clone();
    let mut buffers = Buffers::default();
    let (_usb, mut hid) = new(driver, &mut buffers, "BitBox03");
    state.borrow_mut().enabled = true;
    state.borrow_mut().reads.extend([
        Ok(vec![0xaa; 64]),
        Ok(vec![0xbb; 1]),
        Ok(vec![0xcc; 63]),
        Ok(vec![0xdd; 65]),
        Err(EndpointError::Disabled),
        Ok(vec![0xee; 64]),
    ]);
    assert_eq!(hid.read().await.unwrap(), [0xaa; 64]);
    for _ in 0..3 {
        assert_eq!(hid.read().await, Err(ReadError::InvalidReport));
    }
    assert_eq!(hid.read().await, Err(ReadError::Disabled));
    assert_eq!(hid.read().await.unwrap(), [0xee; 64]);
}

#[async_test::test]
async fn test_write_disconnect() {
    let driver = driver();
    let state = driver.0.clone();
    let mut buffers = Buffers::default();
    let (_usb, mut hid) = new(driver, &mut buffers, "BitBox03");
    assert!(poll_once(hid.ready()).is_pending());
    assert_eq!(hid.write(&[0xaa; 64]).await, Err(EndpointError::Disabled));
    state.borrow_mut().enabled = true;
    hid.ready().await;
    hid.write(&[0xbb; 64]).await.unwrap();
    assert_eq!(state.borrow().writes, vec![vec![0xbb; 64]]);
}

#[derive(Default)]
struct VendorState {
    requests: Vec<Vec<u8>>,
    resets: usize,
    ticks: Vec<u64>,
}

struct EchoHandler(Rc<RefCell<VendorState>>);

impl VendorCommandHandler for EchoHandler {
    fn handle_vendor_command(
        &mut self,
        _cid: u32,
        cmd: u8,
        payload: &[u8],
        _now_ms: u64,
    ) -> Result<Vec<u8>, bitbox_u2fhid::ErrorCode> {
        assert_eq!(cmd, 0xc1);
        self.0.borrow_mut().requests.push(payload.to_vec());
        Ok(payload.to_vec())
    }

    fn tick(&mut self, now_ms: u64) {
        self.0.borrow_mut().ticks.push(now_ms);
    }

    fn reset(&mut self) {
        self.0.borrow_mut().resets += 1;
    }
}

fn reports() -> [Vec<u8>; 2] {
    // One 60-byte vendor message on channel 0x12345678, split across two reports.
    let mut initial = vec![0; REPORT_SIZE];
    initial[..7].copy_from_slice(&hex!("12345678c1003c"));
    initial[7..].fill(0xab);
    let mut continuation = vec![0; REPORT_SIZE];
    continuation[..8].copy_from_slice(&hex!("1234567800ababab"));
    [initial, continuation]
}

#[async_test::test]
async fn test_run_fragmented_message_and_backpressure() {
    let driver = driver();
    let state = driver.0.clone();
    let mut buffers = Buffers::default();
    let (mut usb, mut hid) = new(driver, &mut buffers, "BitBox03");
    request(&mut usb, &state, hex!("0009010000000000"));
    let vendor = Rc::new(RefCell::new(VendorState::default()));
    let mut transport = U2fHid::new(EchoHandler(vendor.clone()));
    let spins = Cell::new(0);
    let now = Cell::new(0);
    let mut run = pin!(hid.run(&mut transport, || now.get(), || spins.set(spins.get() + 1)));

    state.borrow_mut().reads.push_back(Ok(vec![0xff; 63]));
    state.borrow_mut().reads.extend(reports().map(Ok));
    state.borrow_mut().write_blocked = true;
    assert!(poll_once(run.as_mut()).is_pending());
    assert_eq!(vendor.borrow().requests, vec![vec![0xab; 60]]);
    assert!(state.borrow().writes.is_empty());
    let previous_spins = spins.get();
    now.set(100);
    assert!(poll_once(run.as_mut()).is_pending());
    assert!(spins.get() > previous_spins);
    assert_eq!(vendor.borrow().ticks.last(), Some(&100));

    state.borrow_mut().write_blocked = false;
    assert!(poll_once(run.as_mut()).is_pending());
    assert_eq!(state.borrow().writes, reports());
}

#[async_test::test]
async fn test_run_timeout_without_another_report() {
    let driver = driver();
    let state = driver.0.clone();
    let mut buffers = Buffers::default();
    let (mut usb, mut hid) = new(driver, &mut buffers, "BitBox03");
    request(&mut usb, &state, hex!("0009010000000000"));
    let vendor = Rc::new(RefCell::new(VendorState::default()));
    let mut transport = U2fHid::new(EchoHandler(vendor.clone()));
    let now = Cell::new(0);
    let mut run = pin!(hid.run(&mut transport, || now.get(), || {}));
    state.borrow_mut().reads.push_back(Ok(reports()[0].clone()));
    assert!(poll_once(run.as_mut()).is_pending());
    assert!(state.borrow().writes.is_empty());
    now.set(501);
    assert!(poll_once(run.as_mut()).is_pending());
    assert!(vendor.borrow().requests.is_empty());
    assert_eq!(&state.borrow().writes[0][..8], &hex!("12345678bf000105"));
}

#[async_test::test]
async fn test_run_reconnect_discards_requests_and_responses() {
    // Exercise both a partially received request and an interrupted multi-report response.
    for pending_response in [false, true] {
        for event in [Event::Reset, Event::PowerRemoved] {
            let driver = driver();
            let state = driver.0.clone();
            let mut buffers = Buffers::default();
            let (mut usb, mut hid) = new(driver, &mut buffers, "BitBox03");
            request(&mut usb, &state, hex!("0009010000000000"));
            let vendor = Rc::new(RefCell::new(VendorState::default()));
            let mut transport = U2fHid::new(EchoHandler(vendor.clone()));
            let mut run = pin!(hid.run(&mut transport, || 0, || {}));
            state.borrow_mut().reads.push_back(Ok(reports()[0].clone()));
            if pending_response {
                state.borrow_mut().reads.push_back(Ok(reports()[1].clone()));
                state.borrow_mut().write_blocked = true;
            }
            assert!(poll_once(run.as_mut()).is_pending());
            assert_eq!(vendor.borrow().resets, 1);

            state.borrow_mut().events.push_back(event);
            assert!(poll_once(usb.run()).is_pending());
            // Reconfigure before polling the HWW task. Endpoint status alone cannot detect this.
            state.borrow_mut().events.push_back(Event::PowerDetected);
            request(&mut usb, &state, hex!("0009010000000000"));
            state.borrow_mut().write_blocked = false;
            state.borrow_mut().reads.push_back(Ok(reports()[1].clone()));
            assert!(poll_once(run.as_mut()).is_pending());
            assert_eq!(vendor.borrow().resets, 2);
            assert!(state.borrow().writes.is_empty());
            assert_eq!(
                vendor.borrow().requests.len(),
                usize::from(pending_response)
            );

            state.borrow_mut().reads.extend(reports().map(Ok));
            assert!(poll_once(run.as_mut()).is_pending());
            assert_eq!(state.borrow().writes, reports());
        }
    }
}
