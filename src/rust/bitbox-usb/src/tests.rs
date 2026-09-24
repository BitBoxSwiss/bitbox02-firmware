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
            self.0
                .borrow_mut()
                .events
                .pop_front()
                .map_or(Poll::Pending, Poll::Ready)
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
        let packet = self.state.borrow_mut().reads.pop_front().unwrap()?;
        if packet.len() > buf.len() {
            return Err(EndpointError::BufferOverflow);
        }
        buf[..packet.len()].copy_from_slice(&packet);
        Ok(packet.len())
    }
}

impl EndpointIn for FakeEndpoint {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        if !self.state.borrow().enabled {
            return Err(EndpointError::Disabled);
        }
        self.state.borrow_mut().writes.push(buf.to_vec());
        Ok(())
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
    let (mut usb, _) = new(driver, &mut buffers, "BitBox03", "0.1.0");

    let device = request(&mut usb, &state, hex!("8006000100001200"));
    assert_eq!(&device[4..8], &hex!("00000040"));
    assert_eq!(&device[8..12], &hex!("eb030324"));
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
    let (mut usb, _) = new(driver, &mut buffers, &product, "0.1.0");
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
    let (_usb, mut hid) = new(driver, &mut buffers, "BitBox03", "0.1.0");
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
    let (_usb, mut hid) = new(driver, &mut buffers, "BitBox03", "0.1.0");
    assert!(poll_once(hid.ready()).is_pending());
    assert_eq!(hid.write(&[0xaa; 64]).await, Err(EndpointError::Disabled));
    state.borrow_mut().enabled = true;
    hid.ready().await;
    hid.write(&[0xbb; 64]).await.unwrap();
    assert_eq!(state.borrow().writes, vec![vec![0xbb; 64]]);
}
