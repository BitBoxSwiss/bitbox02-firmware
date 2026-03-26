// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use bitbox_u2fhid::{
    CAPABILITY_WINK, COMMAND_VENDOR_FIRST, DeviceInfo, ErrorCode, U2fHid, VendorCommandHandler,
};

use crate::hal::{Hal, Memory, System};

const HWW_CMD: u8 = COMMAND_VENDOR_FIRST + 1;

const HWW_REQ_NEW: u8 = 0;
const HWW_REQ_RETRY: u8 = 1;
const HWW_REQ_CANCEL: u8 = 2;
const HWW_REQ_INFO: u8 = b'i';

const HWW_RSP_ACK: u8 = 0;
const HWW_RSP_NOT_READY: u8 = 1;
const HWW_RSP_BUSY: u8 = 2;
const HWW_RSP_NACK: u8 = 3;
const USB_OUTSTANDING_OP_TIMEOUT_MS: u64 = 500;

pub type HwwTransport<H> = U2fHid<HwwVendorHandler<H>>;

pub fn hww_transport<H>() -> HwwTransport<H>
where
    H: Hal + Default + 'static,
{
    U2fHid::new(device_info(), HwwVendorHandler::new(H::default()))
}

fn device_info() -> DeviceInfo {
    let (version_major, version_minor, version_build) =
        parse_version(crate::version::FIRMWARE_VERSION_SHORT);
    DeviceInfo {
        interface_version: 2,
        version_major,
        version_minor,
        version_build,
        capabilities: CAPABILITY_WINK,
    }
}

fn parse_version(version: &str) -> (u8, u8, u8) {
    let version = version.strip_prefix('v').unwrap_or(version);
    let mut parts = version.split('.');
    let parse_part = |part: Option<&str>| part.and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
    (
        parse_part(parts.next()),
        parse_part(parts.next()),
        parse_part(parts.next()),
    )
}

fn info_response<H: Hal>(hal: &mut H) -> Vec<u8> {
    let version = crate::version::FIRMWARE_VERSION_SHORT.as_bytes();
    let mut response = Vec::with_capacity(version.len() + 5);
    response.push(version.len() as u8);
    response.extend_from_slice(version);
    response.push(match hal.memory().get_platform() {
        Ok(crate::hal::memory::Platform::BitBox02Plus) => 0x02,
        _ => 0x00,
    });
    response.push(if hal.system().is_btconly() {
        0x01
    } else {
        0x00
    });
    response.push((!crate::keystore::is_locked()) as u8);
    response.push(hal.memory().is_initialized() as u8);
    response
}

async fn process_packet_with_hal<H>(usb_in: Vec<u8>) -> Vec<u8>
where
    H: Hal + Default,
{
    let mut hal = H::default();
    crate::hww::process_packet(&mut hal, usb_in).await
}

fn encode_hww_response(status: u8, payload: &[u8]) -> Vec<u8> {
    let mut response = Vec::with_capacity(payload.len() + 1);
    response.push(status);
    response.extend_from_slice(payload);
    response
}

pub struct HwwVendorHandler<H> {
    hal: H,
    deadline_ms: Option<u64>,
}

impl<H> HwwVendorHandler<H> {
    pub fn new(hal: H) -> Self {
        Self {
            hal,
            deadline_ms: None,
        }
    }

    fn refresh_timeout(&mut self, now_ms: u64) {
        self.deadline_ms = if crate::async_usb::is_idle() {
            None
        } else {
            Some(now_ms.saturating_add(USB_OUTSTANDING_OP_TIMEOUT_MS))
        };
    }
}

impl<H> VendorCommandHandler for HwwVendorHandler<H>
where
    H: Hal + Default + 'static,
{
    fn handle_vendor_command(
        &mut self,
        _cid: u32,
        cmd: u8,
        payload: &[u8],
        now_ms: u64,
    ) -> Result<Vec<u8>, ErrorCode> {
        if cmd != HWW_CMD {
            return Err(ErrorCode::InvalidCmd);
        }
        if payload.is_empty() {
            return Ok(vec![HWW_RSP_NACK]);
        }

        let request = payload[0];
        let body = &payload[1..];
        match request {
            HWW_REQ_INFO => Ok(info_response(&mut self.hal)),
            HWW_REQ_NEW => {
                if crate::async_usb::waiting_for_next_request() {
                    crate::async_usb::on_next_request(body);
                } else if !crate::async_usb::is_idle() {
                    return Ok(vec![HWW_RSP_BUSY]);
                } else {
                    crate::async_usb::spawn(process_packet_with_hal::<H>, body);
                }
                self.refresh_timeout(now_ms);
                crate::async_usb::spin();
                let mut response = vec![0u8; bitbox_u2fhid::MAX_MESSAGE_SIZE];
                let response = match crate::async_usb::copy_response(&mut response) {
                    Ok(len) => {
                        response.truncate(len);
                        Ok(encode_hww_response(HWW_RSP_ACK, response.as_slice()))
                    }
                    Err(crate::async_usb::CopyResponseErr::NotReady) => Ok(vec![HWW_RSP_NOT_READY]),
                    Err(crate::async_usb::CopyResponseErr::NotRunning) => Ok(vec![HWW_RSP_NACK]),
                };
                self.refresh_timeout(now_ms);
                response
            }
            HWW_REQ_RETRY => {
                let mut response = vec![0u8; bitbox_u2fhid::MAX_MESSAGE_SIZE];
                let response = match crate::async_usb::copy_response(&mut response) {
                    Ok(len) => {
                        response.truncate(len);
                        Ok(encode_hww_response(HWW_RSP_ACK, response.as_slice()))
                    }
                    Err(crate::async_usb::CopyResponseErr::NotReady) => Ok(vec![HWW_RSP_NOT_READY]),
                    Err(crate::async_usb::CopyResponseErr::NotRunning) => Ok(vec![HWW_RSP_NACK]),
                };
                self.refresh_timeout(now_ms);
                response
            }
            HWW_REQ_CANCEL => {
                let _ = crate::async_usb::cancel();
                self.refresh_timeout(now_ms);
                Ok(vec![HWW_RSP_NACK])
            }
            _ => Ok(vec![HWW_RSP_NACK]),
        }
    }

    fn tick(&mut self, now_ms: u64) {
        if crate::async_usb::is_idle() {
            self.deadline_ms = None;
            return;
        }
        if let Some(deadline_ms) = self.deadline_ms
            && now_ms > deadline_ms
        {
            let _ = crate::async_usb::cancel();
            self.deadline_ms = None;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::hal::{Memory, System, memory::Platform};
    use std::sync::{Mutex, MutexGuard};

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    async fn ready_task(_usb_in: Vec<u8>) -> Vec<u8> {
        vec![0xaa, 0xbb]
    }

    async fn pending_task(_usb_in: Vec<u8>) -> Vec<u8> {
        core::future::pending::<()>().await;
        unreachable!()
    }

    async fn next_request_task(usb_in: Vec<u8>) -> Vec<u8> {
        assert_eq!(usb_in, vec![0xaa]);
        let next_request = crate::async_usb::next_request(vec![0xbb]).await;
        assert_eq!(next_request, vec![0xcc]);
        vec![0xdd]
    }

    fn test_guard() -> MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap();
        let _ = crate::async_usb::cancel();
        guard
    }

    fn handler() -> HwwVendorHandler<TestingHal<'static>> {
        let _ = crate::async_usb::cancel();
        HwwVendorHandler::new(TestingHal::new())
    }

    #[test]
    fn test_req_info() {
        let _guard = test_guard();
        let mut handler = handler();
        handler.hal.memory.set_platform(Platform::BitBox02Plus);
        handler.hal.system.set_btconly(true);
        handler.hal.memory.set_initialized().unwrap();
        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_INFO], 0)
            .unwrap();
        assert!(!response.is_empty());
        assert_eq!(
            response[0] as usize,
            crate::version::FIRMWARE_VERSION_SHORT.len()
        );
        assert_eq!(response[response.len() - 4], 0x02);
        assert_eq!(response[response.len() - 3], 0x01);
        assert_eq!(response[response.len() - 1], 0x01);
    }

    #[test]
    fn test_req_cancel() {
        let _guard = test_guard();
        crate::async_usb::spawn(pending_task, &[]);
        let mut handler = handler();
        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_CANCEL], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_NACK]);
        assert!(crate::async_usb::is_idle());
    }

    #[test]
    fn test_req_retry_ack() {
        let _guard = test_guard();
        crate::async_usb::spawn(ready_task, &[]);
        crate::async_usb::spin();

        let mut handler = handler();
        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_RETRY], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_ACK, 0xaa, 0xbb]);
    }

    #[test]
    fn test_req_new_routes_follow_up_request() {
        let _guard = test_guard();
        let _ = crate::async_usb::cancel();
        crate::async_usb::spawn(next_request_task, &[0xaa]);
        crate::async_usb::spin();

        let mut handler = HwwVendorHandler::new(TestingHal::new());
        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_RETRY], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_ACK, 0xbb]);
        assert!(crate::async_usb::waiting_for_next_request());

        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_NEW, 0xcc], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_ACK, 0xdd]);
        assert!(crate::async_usb::is_idle());
    }

    #[test]
    fn test_req_new_while_busy_returns_busy() {
        let _guard = test_guard();
        let _ = crate::async_usb::cancel();
        crate::async_usb::spawn(pending_task, &[]);

        let mut handler = HwwVendorHandler::new(TestingHal::new());
        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_NEW, 0x00], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_BUSY]);
        let _ = crate::async_usb::cancel();
    }

    #[test]
    fn test_outstanding_request_times_out_without_retry() {
        let _guard = test_guard();
        crate::async_usb::spawn(pending_task, &[]);
        let mut handler = HwwVendorHandler::new(TestingHal::new());

        let response = handler
            .handle_vendor_command(1, HWW_CMD, &[HWW_REQ_RETRY], 0)
            .unwrap();
        assert_eq!(response, vec![HWW_RSP_NOT_READY]);
        assert!(!crate::async_usb::is_idle());

        bitbox_u2fhid::VendorCommandHandler::tick(&mut handler, USB_OUTSTANDING_OP_TIMEOUT_MS + 1);
        assert!(crate::async_usb::is_idle());
    }
}
