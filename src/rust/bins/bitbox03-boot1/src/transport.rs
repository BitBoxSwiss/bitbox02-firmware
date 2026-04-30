// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use bitbox_u2fhid::{ErrorCode, U2fHid, VendorCommandHandler};

use crate::commands::{Backend, BootloaderApi};
use crate::protocol::BOOTLOADER_CMD;

pub type BootloaderTransport<B> = U2fHid<BootloaderVendorHandler<B>>;

pub struct BootloaderVendorHandler<B> {
    api: BootloaderApi<B>,
}

impl<B> BootloaderVendorHandler<B> {
    pub fn new(api: BootloaderApi<B>) -> Self {
        Self { api }
    }
}

impl<B: Backend> VendorCommandHandler for BootloaderVendorHandler<B> {
    fn handle_vendor_command(
        &mut self,
        _cid: u32,
        cmd: u8,
        payload: &[u8],
        _now_ms: u64,
    ) -> Result<Vec<u8>, ErrorCode> {
        if cmd != BOOTLOADER_CMD {
            return Err(ErrorCode::InvalidCmd);
        }
        if payload.is_empty() {
            return Err(ErrorCode::InvalidLen);
        }
        Ok(self.api.handle(payload))
    }
}

pub fn bootloader_transport<B: Backend>(backend: B) -> BootloaderTransport<B> {
    U2fHid::new(BootloaderVendorHandler::new(BootloaderApi::new(backend)))
}

#[cfg(target_arch = "arm")]
pub fn bootloader_transport_arm() -> BootloaderTransport<crate::commands::FlashBackend> {
    bootloader_transport(crate::commands::FlashBackend)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::commands::Backend;
    use crate::protocol::{OP_STATUS_OK, OP_VERSIONS};
    use bitbox_boot_utils::FLASH_PAGE_SIZE;
    use bitbox_u2fhid::REPORT_SIZE;

    struct MockBackend;

    impl Backend for MockBackend {
        fn read(&self, _addr: usize, out: &mut [u8]) {
            out.fill(0xff);
        }

        fn write_page(&mut self, _addr: usize, _page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()> {
            Ok(())
        }

        fn reboot(&mut self) -> ! {
            panic!("reboot")
        }

        fn hardware_type(&self) -> u8 {
            1
        }
    }

    fn request_reports(cid: u32, cmd: u8, payload: &[u8]) -> Vec<[u8; REPORT_SIZE]> {
        let mut reports = Vec::new();
        let mut report = [0u8; REPORT_SIZE];
        report[..4].copy_from_slice(&cid.to_be_bytes());
        report[4] = cmd;
        report[5..7].copy_from_slice(&(payload.len() as u16).to_be_bytes());
        let first_len = core::cmp::min(payload.len(), REPORT_SIZE - 7);
        report[7..7 + first_len].copy_from_slice(&payload[..first_len]);
        reports.push(report);

        let mut offset = first_len;
        let mut seq = 0u8;
        while offset < payload.len() {
            let mut cont_report = [0u8; REPORT_SIZE];
            cont_report[..4].copy_from_slice(&cid.to_be_bytes());
            cont_report[4] = seq;
            let chunk_len = core::cmp::min(payload.len() - offset, REPORT_SIZE - 5);
            cont_report[5..5 + chunk_len].copy_from_slice(&payload[offset..offset + chunk_len]);
            reports.push(cont_report);
            offset += chunk_len;
            seq = seq.wrapping_add(1);
        }

        reports
    }

    fn parse_message(reports: &[[u8; REPORT_SIZE]]) -> (u32, u8, Vec<u8>) {
        let cid = u32::from_be_bytes(reports[0][..4].try_into().unwrap());
        let cmd = reports[0][4];
        let len = u16::from_be_bytes(reports[0][5..7].try_into().unwrap()) as usize;
        let mut payload = Vec::with_capacity(len);
        let first_len = core::cmp::min(len, REPORT_SIZE - 7);
        payload.extend_from_slice(&reports[0][7..7 + first_len]);

        let mut offset = first_len;
        let mut seq = 0u8;
        for report in &reports[1..] {
            assert_eq!(report[4], seq);
            let chunk_len = core::cmp::min(len - offset, REPORT_SIZE - 5);
            payload.extend_from_slice(&report[5..5 + chunk_len]);
            offset += chunk_len;
            seq = seq.wrapping_add(1);
        }

        (cid, cmd, payload)
    }

    #[test]
    fn test_bootloader_transport_versions_request() {
        let cid = 0x0102_0304;
        let mut transport = bootloader_transport(MockBackend);

        for report in request_reports(cid, BOOTLOADER_CMD, &[OP_VERSIONS]) {
            transport.handle_report(&report, 0);
        }

        let mut response_reports = Vec::new();
        while let Some(report) = transport.pull_report() {
            response_reports.push(report);
        }

        let (response_cid, response_cmd, payload) = parse_message(&response_reports);
        assert_eq!(response_cid, cid);
        assert_eq!(response_cmd, BOOTLOADER_CMD);
        assert_eq!(payload[0], OP_VERSIONS);
        assert_eq!(payload[1], OP_STATUS_OK);
    }
}
