// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bitbox_usb_report_queue::UsbReportQueue;

pub const REPORT_SIZE: usize = 64;
const INIT_HEADER_SIZE: usize = 7;
const CONT_HEADER_SIZE: usize = 5;
const INIT_PAYLOAD_SIZE: usize = REPORT_SIZE - INIT_HEADER_SIZE;
const CONT_PAYLOAD_SIZE: usize = REPORT_SIZE - CONT_HEADER_SIZE;
pub const MAX_MESSAGE_SIZE: usize = INIT_PAYLOAD_SIZE + 128 * CONT_PAYLOAD_SIZE;
const MESSAGE_TIMEOUT_MS: u64 = 500;

const BROADCAST_CID: u32 = 0xffff_ffff;

const TYPE_INIT: u8 = 0x80;
// U2F-native commands such as PING/MSG/LOCK/WINK/SYNC are intentionally not implemented while the
// Rust USB port is focused on HWW. Reintroduce them when porting the U2F transport as well.
// const COMMAND_PING: u8 = TYPE_INIT | 0x01;
// const COMMAND_MSG: u8 = TYPE_INIT | 0x03;
// const COMMAND_LOCK: u8 = TYPE_INIT | 0x04;
// const COMMAND_WINK: u8 = TYPE_INIT | 0x08;
// const COMMAND_SYNC: u8 = TYPE_INIT | 0x3c;
//
// We keep the INIT command ID because `usb_frame.c` treats it specially during frame
// reassembly/resynchronization, even though completed INIT requests are currently rejected with
// INVALID_CMD on the HWW interface.
const COMMAND_INIT: u8 = TYPE_INIT | 0x06;
const COMMAND_ERROR: u8 = TYPE_INIT | 0x3f;
pub const COMMAND_VENDOR_FIRST: u8 = TYPE_INIT | 0x40;
const COMMAND_VENDOR_LAST: u8 = TYPE_INIT | 0x7f;

// U2F's INIT response reports interface/version/capabilities via a device-info structure. The
// Rust USB port is focused on HWW for now, so this stays commented out until the U2F transport is
// ported as well.
//
// pub const CAPABILITY_WINK: u8 = 0x01;
//
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub struct DeviceInfo {
//     pub interface_version: u8,
//     pub version_major: u8,
//     pub version_minor: u8,
//     pub version_build: u8,
//     pub capabilities: u8,
// }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ErrorCode {
    InvalidCmd = 0x01,
    InvalidPar = 0x02,
    InvalidLen = 0x03,
    InvalidSeq = 0x04,
    MsgTimeout = 0x05,
    ChannelBusy = 0x06,
    LockRequired = 0x0a,
    InvalidCid = 0x0b,
    Other = 0x7f,
}

pub trait VendorCommandHandler {
    fn handle_vendor_command(
        &mut self,
        cid: u32,
        cmd: u8,
        payload: &[u8],
        now_ms: u64,
    ) -> Result<Vec<u8>, ErrorCode>;

    fn tick(&mut self, _now_ms: u64) {}
}

struct ReceiveState {
    buffer: [u8; MAX_MESSAGE_SIZE],
    cid: u32,
    cmd: u8,
    total_len: usize,
    received_len: usize,
    next_seq: u8,
    deadline_ms: Option<u64>,
    initialized: bool,
}

impl ReceiveState {
    const fn new() -> Self {
        Self {
            buffer: [0; MAX_MESSAGE_SIZE],
            cid: 0,
            cmd: 0,
            total_len: 0,
            received_len: 0,
            next_seq: 0,
            deadline_ms: None,
            initialized: false,
        }
    }

    fn reset(&mut self) {
        self.cid = 0;
        self.cmd = 0;
        self.total_len = 0;
        self.received_len = 0;
        self.next_seq = 0;
        self.deadline_ms = None;
        self.initialized = false;
    }
}

pub struct U2fHid<V> {
    // U2F INIT would need this device info in order to answer INIT requests.
    // device_info: DeviceInfo,
    vendor_handler: V,
    out_queue: UsbReportQueue,
    receive_state: ReceiveState,
    // U2F INIT would also need channel allocation state.
    // next_cid: u32,
    sending_cid: Option<u32>,
}

impl<V: VendorCommandHandler> U2fHid<V> {
    // U2F INIT would take `device_info` as an additional constructor argument.
    // pub fn new(device_info: DeviceInfo, vendor_handler: V) -> Self {
    pub fn new(vendor_handler: V) -> Self {
        Self {
            vendor_handler,
            out_queue: UsbReportQueue::new(),
            receive_state: ReceiveState::new(),
            sending_cid: None,
        }
    }

    pub fn handle_report(&mut self, report: &[u8; REPORT_SIZE], now_ms: u64) {
        self.tick(now_ms);

        let cid = u32::from_be_bytes(report[..4].try_into().unwrap());
        let type_byte = report[4];
        if self.sending_cid.is_some() {
            self.handle_while_sending(cid, type_byte);
            return;
        }

        if type_byte & TYPE_INIT != 0 {
            self.handle_init_packet(report, cid, type_byte, now_ms);
        } else {
            self.handle_cont_packet(report, cid, type_byte, now_ms);
        }
    }

    pub fn tick(&mut self, now_ms: u64) {
        self.vendor_handler.tick(now_ms);

        let Some(deadline_ms) = self.receive_state.deadline_ms else {
            return;
        };
        if now_ms < deadline_ms {
            return;
        }
        let cid = self.receive_state.cid;
        self.receive_state.reset();
        self.enqueue_error(cid, ErrorCode::MsgTimeout);
    }

    pub fn pull_report(&mut self) -> Option<[u8; REPORT_SIZE]> {
        let report = self.out_queue.pull();
        if report.is_some() && self.out_queue.peek().is_none() {
            self.sending_cid = None;
        }
        report
    }

    pub fn handler(&self) -> &V {
        &self.vendor_handler
    }

    pub fn handler_mut(&mut self) -> &mut V {
        &mut self.vendor_handler
    }

    fn handle_while_sending(&mut self, cid: u32, type_byte: u8) {
        if type_byte & TYPE_INIT == 0 {
            return;
        }
        self.enqueue_error(cid, ErrorCode::ChannelBusy);
    }

    fn handle_init_packet(&mut self, report: &[u8; REPORT_SIZE], cid: u32, cmd: u8, now_ms: u64) {
        if self.receive_state.initialized {
            if cid != self.receive_state.cid && self.receive_state.cmd == COMMAND_INIT {
                self.enqueue_error(cid, ErrorCode::ChannelBusy);
                return;
            }
            if cid != self.receive_state.cid
                && cmd != COMMAND_INIT
                && self.receive_state.cmd != COMMAND_INIT
            {
                self.enqueue_error(cid, ErrorCode::ChannelBusy);
                return;
            }
            if cid == self.receive_state.cid && cmd != COMMAND_INIT {
                self.receive_state.reset();
                self.enqueue_error(cid, ErrorCode::InvalidSeq);
                return;
            }
        }

        let total_len = u16::from_be_bytes([report[5], report[6]]) as usize;
        if total_len > MAX_MESSAGE_SIZE {
            self.receive_state.reset();
            self.enqueue_error(cid, ErrorCode::InvalidLen);
            return;
        }

        self.receive_state.reset();
        self.receive_state.cid = cid;
        self.receive_state.cmd = cmd;
        self.receive_state.total_len = total_len;
        self.receive_state.initialized = true;

        let init_copy_len = core::cmp::min(total_len, INIT_PAYLOAD_SIZE);
        self.receive_state.buffer[..init_copy_len]
            .copy_from_slice(&report[INIT_HEADER_SIZE..INIT_HEADER_SIZE + init_copy_len]);
        self.receive_state.received_len = init_copy_len;
        self.receive_state.next_seq = 0;
        self.receive_state.deadline_ms = if init_copy_len < total_len {
            Some(now_ms.saturating_add(MESSAGE_TIMEOUT_MS))
        } else {
            None
        };

        if init_copy_len == total_len {
            self.finish_message(now_ms);
        }
    }

    fn handle_cont_packet(&mut self, report: &[u8; REPORT_SIZE], cid: u32, seq: u8, now_ms: u64) {
        if !self.receive_state.initialized {
            return;
        }
        if cid != self.receive_state.cid {
            self.enqueue_error(cid, ErrorCode::ChannelBusy);
            return;
        }
        if seq != self.receive_state.next_seq {
            self.receive_state.reset();
            self.enqueue_error(cid, ErrorCode::InvalidSeq);
            return;
        }
        if self.receive_state.received_len >= self.receive_state.total_len
            || self.receive_state.received_len + CONT_PAYLOAD_SIZE > MAX_MESSAGE_SIZE
        {
            self.receive_state.reset();
            self.enqueue_error(cid, ErrorCode::InvalidLen);
            return;
        }

        let copy_len = core::cmp::min(
            self.receive_state.total_len - self.receive_state.received_len,
            CONT_PAYLOAD_SIZE,
        );
        let start = self.receive_state.received_len;
        let end = start + copy_len;
        self.receive_state.buffer[start..end]
            .copy_from_slice(&report[CONT_HEADER_SIZE..CONT_HEADER_SIZE + copy_len]);
        self.receive_state.received_len = end;
        self.receive_state.next_seq = self.receive_state.next_seq.wrapping_add(1);
        self.receive_state.deadline_ms =
            if self.receive_state.received_len < self.receive_state.total_len {
                Some(now_ms.saturating_add(MESSAGE_TIMEOUT_MS))
            } else {
                None
            };

        if self.receive_state.received_len == self.receive_state.total_len {
            self.finish_message(now_ms);
        }
    }

    fn finish_message(&mut self, now_ms: u64) {
        let cid = self.receive_state.cid;
        let cmd = self.receive_state.cmd;
        let total_len = self.receive_state.total_len;
        let mut payload = Vec::with_capacity(total_len);
        payload.extend_from_slice(&self.receive_state.buffer[..total_len]);
        self.receive_state.reset();

        let response = self.handle_message(cid, cmd, payload.as_slice(), now_ms);
        match response {
            Ok(response_payload) => {
                if self
                    .enqueue_response(cmd, cid, response_payload.as_slice())
                    .is_err()
                {
                    self.out_queue.clear();
                    self.enqueue_error(cid, ErrorCode::Other);
                } else if self.out_queue.peek().is_some() {
                    self.sending_cid = Some(cid);
                }
            }
            Err(err) => self.enqueue_error(cid, err),
        }
    }

    fn handle_message(
        &mut self,
        cid: u32,
        cmd: u8,
        payload: &[u8],
        now_ms: u64,
    ) -> Result<Vec<u8>, ErrorCode> {
        match cmd {
            // The HWW-only port rejects the U2F-native commands for now.
            // COMMAND_INIT => self.handle_init(cid, payload),
            // COMMAND_PING => self.handle_ping(cid, payload),
            // COMMAND_WINK => self.handle_wink(cid, payload),
            // COMMAND_LOCK => Err(ErrorCode::InvalidCmd),
            // COMMAND_SYNC => Err(ErrorCode::InvalidCmd),
            // COMMAND_MSG | COMMAND_ERROR => Err(ErrorCode::InvalidCmd),
            COMMAND_VENDOR_FIRST..=COMMAND_VENDOR_LAST => {
                // Even in the HWW-only port we keep U2FHID's reserved CID checks intact.
                if cid == 0 || cid == BROADCAST_CID {
                    return Err(ErrorCode::InvalidCid);
                }
                self.vendor_handler
                    .handle_vendor_command(cid, cmd, payload, now_ms)
            }
            _ => Err(ErrorCode::InvalidCmd),
        }
    }

    // When the U2F transport is ported, restore these helpers together with `DeviceInfo`,
    // `next_cid`, and the constructor argument above.
    //
    // fn handle_init(&mut self, cid: u32, payload: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    //     if payload.len() != 8 {
    //         return Err(ErrorCode::InvalidLen);
    //     }
    //     if cid == 0 {
    //         return Err(ErrorCode::InvalidCid);
    //     }
    //
    //     let allocated_cid = if cid == BROADCAST_CID {
    //         self.allocate_cid()
    //     } else {
    //         cid
    //     };
    //
    //     let mut response = Vec::with_capacity(17);
    //     response.extend_from_slice(payload);
    //     response.extend_from_slice(&allocated_cid.to_be_bytes());
    //     response.push(self.device_info.interface_version);
    //     response.push(self.device_info.version_major);
    //     response.push(self.device_info.version_minor);
    //     response.push(self.device_info.version_build);
    //     response.push(self.device_info.capabilities);
    //     Ok(response)
    // }
    //
    // fn handle_ping(&self, cid: u32, payload: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    //     if cid == 0 || cid == BROADCAST_CID {
    //         return Err(ErrorCode::InvalidCid);
    //     }
    //     Ok(payload.to_vec())
    // }
    //
    // fn handle_wink(&self, cid: u32, payload: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    //     if cid == 0 || cid == BROADCAST_CID {
    //         return Err(ErrorCode::InvalidCid);
    //     }
    //     if !payload.is_empty() {
    //         return Err(ErrorCode::InvalidLen);
    //     }
    //     Ok(Vec::new())
    // }
    //
    // fn allocate_cid(&mut self) -> u32 {
    //     loop {
    //         let cid = self.next_cid;
    //         self.next_cid = self.next_cid.wrapping_add(1);
    //         if self.next_cid == 0 || self.next_cid == BROADCAST_CID {
    //             self.next_cid = 1;
    //         }
    //         if cid != 0 && cid != BROADCAST_CID {
    //             return cid;
    //         }
    //     }
    // }

    fn enqueue_error(&mut self, cid: u32, err: ErrorCode) {
        let _ = self.enqueue_response(COMMAND_ERROR, cid, &[err as u8]);
        if self.out_queue.peek().is_some() {
            self.sending_cid = Some(cid);
        }
    }

    fn enqueue_response(&mut self, cmd: u8, cid: u32, payload: &[u8]) -> Result<(), ()> {
        if payload.len() > MAX_MESSAGE_SIZE {
            return Err(());
        }

        let mut report = [0u8; REPORT_SIZE];
        report[..4].copy_from_slice(&cid.to_be_bytes());
        report[4] = cmd;
        report[5..7].copy_from_slice(&(payload.len() as u16).to_be_bytes());

        let first_len = core::cmp::min(payload.len(), INIT_PAYLOAD_SIZE);
        report[INIT_HEADER_SIZE..INIT_HEADER_SIZE + first_len]
            .copy_from_slice(&payload[..first_len]);
        if self.out_queue.push(&report)
            != bitbox_usb_report_queue::UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
        {
            return Err(());
        }

        let mut offset = first_len;
        let mut seq = 0u8;
        while offset < payload.len() {
            let mut cont_report = [0u8; REPORT_SIZE];
            cont_report[..4].copy_from_slice(&cid.to_be_bytes());
            cont_report[4] = seq;
            let chunk_len = core::cmp::min(payload.len() - offset, CONT_PAYLOAD_SIZE);
            cont_report[CONT_HEADER_SIZE..CONT_HEADER_SIZE + chunk_len]
                .copy_from_slice(&payload[offset..offset + chunk_len]);
            if self.out_queue.push(&cont_report)
                != bitbox_usb_report_queue::UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE
            {
                return Err(());
            }
            offset += chunk_len;
            seq = seq.wrapping_add(1);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use alloc::vec;

    const TEST_CID: u32 = 0x0102_0304;
    const TEST_VENDOR_CMD: u8 = COMMAND_VENDOR_FIRST + 1;

    #[derive(Default)]
    struct EchoVendorHandler {
        seen: Vec<(u32, u8, Vec<u8>)>,
        response: Vec<u8>,
        error: Option<ErrorCode>,
    }

    impl VendorCommandHandler for EchoVendorHandler {
        fn handle_vendor_command(
            &mut self,
            cid: u32,
            cmd: u8,
            payload: &[u8],
            _now_ms: u64,
        ) -> Result<Vec<u8>, ErrorCode> {
            self.seen.push((cid, cmd, payload.to_vec()));
            match self.error {
                Some(err) => Err(err),
                None => Ok(self.response.clone()),
            }
        }
    }

    fn make_transport() -> U2fHid<EchoVendorHandler> {
        U2fHid::new(EchoVendorHandler::default())
    }

    fn request_reports(cid: u32, cmd: u8, payload: &[u8]) -> Vec<[u8; REPORT_SIZE]> {
        let mut reports = Vec::new();
        let mut report = [0u8; REPORT_SIZE];
        report[..4].copy_from_slice(&cid.to_be_bytes());
        report[4] = cmd;
        report[5..7].copy_from_slice(&(payload.len() as u16).to_be_bytes());
        let first_len = core::cmp::min(payload.len(), INIT_PAYLOAD_SIZE);
        report[INIT_HEADER_SIZE..INIT_HEADER_SIZE + first_len]
            .copy_from_slice(&payload[..first_len]);
        reports.push(report);

        let mut offset = first_len;
        let mut seq = 0u8;
        while offset < payload.len() {
            let mut cont_report = [0u8; REPORT_SIZE];
            cont_report[..4].copy_from_slice(&cid.to_be_bytes());
            cont_report[4] = seq;
            let chunk_len = core::cmp::min(payload.len() - offset, CONT_PAYLOAD_SIZE);
            cont_report[CONT_HEADER_SIZE..CONT_HEADER_SIZE + chunk_len]
                .copy_from_slice(&payload[offset..offset + chunk_len]);
            reports.push(cont_report);
            offset += chunk_len;
            seq = seq.wrapping_add(1);
        }
        reports
    }

    fn drain_reports<V: VendorCommandHandler>(transport: &mut U2fHid<V>) -> Vec<[u8; REPORT_SIZE]> {
        let mut reports = Vec::new();
        while let Some(report) = transport.pull_report() {
            reports.push(report);
        }
        reports
    }

    fn parse_message(reports: &[[u8; REPORT_SIZE]]) -> (u32, u8, Vec<u8>) {
        assert!(!reports.is_empty());
        let cid = u32::from_be_bytes(reports[0][..4].try_into().unwrap());
        let cmd = reports[0][4];
        let len = u16::from_be_bytes(reports[0][5..7].try_into().unwrap()) as usize;
        let mut payload = Vec::with_capacity(len);
        let first_len = core::cmp::min(len, INIT_PAYLOAD_SIZE);
        payload.extend_from_slice(&reports[0][INIT_HEADER_SIZE..INIT_HEADER_SIZE + first_len]);
        let mut offset = first_len;
        for (expected_seq, report) in reports[1..].iter().enumerate() {
            assert_eq!(report[4], expected_seq as u8);
            let chunk_len = core::cmp::min(len - offset, CONT_PAYLOAD_SIZE);
            payload.extend_from_slice(&report[CONT_HEADER_SIZE..CONT_HEADER_SIZE + chunk_len]);
            offset += chunk_len;
        }
        (cid, cmd, payload)
    }

    #[test]
    fn test_u2f_native_commands_return_invalid_cmd() {
        const TEST_COMMAND_PING: u8 = TYPE_INIT | 0x01;
        const TEST_COMMAND_WINK: u8 = TYPE_INIT | 0x08;

        let mut transport = make_transport();
        for (cmd, payload) in [
            (COMMAND_INIT, b"12345678".as_slice()),
            (TEST_COMMAND_PING, b"hello".as_slice()),
            (TEST_COMMAND_WINK, b"".as_slice()),
        ] {
            transport.handle_report(&request_reports(TEST_CID, cmd, payload)[0], 0);
            let (_, response_cmd, response_payload) = parse_message(&drain_reports(&mut transport));
            assert_eq!(response_cmd, COMMAND_ERROR);
            assert_eq!(response_payload, vec![ErrorCode::InvalidCmd as u8]);
        }
    }

    #[test]
    fn test_max_reports() {
        const MAX_REPORTS: usize =
            1 + (MAX_MESSAGE_SIZE - INIT_PAYLOAD_SIZE).div_ceil(CONT_PAYLOAD_SIZE);

        assert_eq!(MAX_REPORTS, 129);
    }

    #[test]
    fn test_vendor_multi_packet_roundtrip() {
        let mut transport = make_transport();
        let payload = [0x42; 100];
        transport.handler_mut().response = payload.to_vec();
        for report in request_reports(TEST_CID, TEST_VENDOR_CMD, &payload) {
            transport.handle_report(&report, 0);
        }

        assert_eq!(
            transport.handler().seen,
            vec![(TEST_CID, TEST_VENDOR_CMD, payload.to_vec())]
        );
        let reports = drain_reports(&mut transport);
        let (cid, cmd, response_payload) = parse_message(&reports);
        assert_eq!(cid, TEST_CID);
        assert_eq!(cmd, TEST_VENDOR_CMD);
        assert_eq!(response_payload, payload);
    }

    #[test]
    fn test_unused_bytes_are_zeroed() {
        let mut transport = make_transport();
        transport.handler_mut().response = b"x".to_vec();
        transport.handle_report(
            &request_reports(TEST_CID, TEST_VENDOR_CMD, b"request")[0],
            0,
        );
        let report = transport.pull_report().unwrap();
        assert!(report[INIT_HEADER_SIZE + 1..].iter().all(|&byte| byte == 0));
    }

    #[test]
    fn test_invalid_sequence_returns_error() {
        let mut transport = make_transport();
        let mut reports = request_reports(TEST_CID, TEST_VENDOR_CMD, &[0x55; 100]);
        reports[1][4] = 1;
        transport.handle_report(&reports[0], 0);
        transport.handle_report(&reports[1], 0);

        let (_, cmd, payload) = parse_message(&drain_reports(&mut transport));
        assert_eq!(cmd, COMMAND_ERROR);
        assert_eq!(payload, vec![ErrorCode::InvalidSeq as u8]);
    }

    #[test]
    fn test_invalid_length_returns_error() {
        let mut transport = make_transport();
        let mut report = [0u8; REPORT_SIZE];
        report[..4].copy_from_slice(&TEST_CID.to_be_bytes());
        report[4] = TEST_VENDOR_CMD;
        report[5..7].copy_from_slice(&((MAX_MESSAGE_SIZE + 1) as u16).to_be_bytes());
        transport.handle_report(&report, 0);

        let (_, cmd, payload) = parse_message(&drain_reports(&mut transport));
        assert_eq!(cmd, COMMAND_ERROR);
        assert_eq!(payload, vec![ErrorCode::InvalidLen as u8]);
    }

    #[test]
    fn test_unsolicited_continuation_is_ignored() {
        let mut transport = make_transport();
        let mut report = [0u8; REPORT_SIZE];
        report[..4].copy_from_slice(&TEST_CID.to_be_bytes());
        report[4] = 0;
        transport.handle_report(&report, 0);
        assert!(transport.pull_report().is_none());
    }

    #[test]
    fn test_channel_busy_while_receiving() {
        let mut transport = make_transport();
        let first = request_reports(TEST_CID, TEST_VENDOR_CMD, &[0x11; 100]);
        let second = request_reports(0xaabb_ccdd, TEST_VENDOR_CMD, b"busy");
        transport.handle_report(&first[0], 0);
        transport.handle_report(&second[0], 0);

        let (cid, cmd, payload) = parse_message(&drain_reports(&mut transport));
        assert_eq!(cid, 0xaabb_ccdd);
        assert_eq!(cmd, COMMAND_ERROR);
        assert_eq!(payload, vec![ErrorCode::ChannelBusy as u8]);
    }

    #[test]
    fn test_message_timeout() {
        let mut transport = make_transport();
        let report = request_reports(TEST_CID, TEST_VENDOR_CMD, &[0x22; 100])[0];
        transport.handle_report(&report, 0);
        transport.tick(MESSAGE_TIMEOUT_MS + 1);

        let (_, cmd, payload) = parse_message(&drain_reports(&mut transport));
        assert_eq!(cmd, COMMAND_ERROR);
        assert_eq!(payload, vec![ErrorCode::MsgTimeout as u8]);
    }

    #[test]
    fn test_max_size_vendor_roundtrip() {
        let mut transport = make_transport();
        let payload = vec![0x5a; MAX_MESSAGE_SIZE];
        transport.handler_mut().response = payload.clone();
        for report in request_reports(TEST_CID, TEST_VENDOR_CMD, &payload) {
            transport.handle_report(&report, 0);
        }

        let reports = drain_reports(&mut transport);
        assert_eq!(reports.len(), 129);
        let (_, cmd, response_payload) = parse_message(&reports);
        assert_eq!(cmd, TEST_VENDOR_CMD);
        assert_eq!(response_payload, payload);
    }
}
