// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

use core::cmp::min;
use core::ptr;

pub const REPORT_SIZE: usize = 64;
pub const INIT_HEADER_SIZE: usize = 7;
pub const CONT_HEADER_SIZE: usize = 5;
pub const INIT_DATA_SIZE: usize = REPORT_SIZE - INIT_HEADER_SIZE;
pub const CONT_DATA_SIZE: usize = REPORT_SIZE - CONT_HEADER_SIZE;
pub const MAX_MESSAGE_LEN: usize = INIT_DATA_SIZE + 128 * CONT_DATA_SIZE;

pub const BROADCAST_CID: u32 = 0xffff_ffff;

pub const TYPE_MASK: u8 = 0x80;
pub const TYPE_INIT: u8 = 0x80;
pub const TYPE_CONT: u8 = 0x00;

pub const PING: u8 = TYPE_INIT | 0x01;
pub const MSG: u8 = TYPE_INIT | 0x03;
pub const LOCK: u8 = TYPE_INIT | 0x04;
pub const INIT: u8 = TYPE_INIT | 0x06;
pub const WINK: u8 = TYPE_INIT | 0x08;
pub const SYNC: u8 = TYPE_INIT | 0x3c;
pub const ERROR: u8 = TYPE_INIT | 0x3f;
pub const VENDOR_FIRST: u8 = TYPE_INIT | 0x40;
pub const VENDOR_LAST: u8 = TYPE_INIT | 0x7f;

pub const IF_VERSION: u8 = 2;
pub const FRAME_TIMEOUT_MS: u16 = 500;
pub const TRANS_TIMEOUT_MS: u16 = 3000;

pub const CAPFLAG_WINK: u8 = 0x01;
pub const CAPFLAG_LOCK: u8 = 0x02;

pub const ERR_NONE: u8 = 0x00;
pub const ERR_INVALID_CMD: u8 = 0x01;
pub const ERR_INVALID_PAR: u8 = 0x02;
pub const ERR_INVALID_LEN: u8 = 0x03;
pub const ERR_INVALID_SEQ: u8 = 0x04;
pub const ERR_MSG_TIMEOUT: u8 = 0x05;
pub const ERR_CHANNEL_BUSY: u8 = 0x06;
pub const ERR_LOCK_REQUIRED: u8 = 0x0a;
pub const ERR_INVALID_CID: u8 = 0x0b;
pub const ERR_OTHER: u8 = 0x7f;
pub const ERR_IGNORE: u8 = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Packet {
    pub cid: u32,
    pub payload: [u8; REPORT_SIZE - 4],
}

impl Packet {
    pub const fn zeroed() -> Self {
        Self {
            cid: 0,
            payload: [0; REPORT_SIZE - 4],
        }
    }

    #[inline]
    pub fn packet_type(&self) -> u8 {
        self.payload[0] & TYPE_MASK
    }

    #[inline]
    pub fn header_byte(&self) -> u8 {
        self.payload[0]
    }

    #[inline]
    pub fn command(&self) -> u8 {
        self.payload[0] & !TYPE_MASK
    }

    #[inline]
    pub fn sequence(&self) -> u8 {
        self.payload[0] & !TYPE_MASK
    }

    #[inline]
    pub fn message_len(&self) -> usize {
        ((self.payload[1] as usize) << 8) | self.payload[2] as usize
    }

    #[inline]
    pub fn init_data(&self) -> &[u8] {
        &self.payload[3..]
    }

    #[inline]
    pub fn cont_data(&self) -> &[u8] {
        &self.payload[1..]
    }

    pub fn set_init(&mut self, cid: u32, cmd: u8, len: usize, data: &[u8]) {
        debug_assert!(len <= u16::MAX as usize);
        debug_assert!(data.len() <= INIT_DATA_SIZE);
        *self = Self::zeroed();
        self.cid = cid;
        self.payload[0] = cmd;
        self.payload[1] = (len >> 8) as u8;
        self.payload[2] = len as u8;
        self.payload[3..3 + data.len()].copy_from_slice(data);
    }

    pub fn set_cont(&mut self, cid: u32, seq: u8, data: &[u8]) {
        debug_assert!(data.len() <= CONT_DATA_SIZE);
        *self = Self::zeroed();
        self.cid = cid;
        self.payload[0] = seq;
        self.payload[1..1 + data.len()].copy_from_slice(data);
    }
}

const _: [u8; REPORT_SIZE] = [0; core::mem::size_of::<Packet>()];

#[repr(C)]
pub struct State {
    pub data: [u8; MAX_MESSAGE_LEN],
    pub buf_ptr: *mut u8,
    pub len: u32,
    pub seq: u8,
    pub cmd: u8,
    pub cid: u32,
    pub initialized: u8,
}

impl State {
    pub const fn new() -> Self {
        Self {
            data: [0; MAX_MESSAGE_LEN],
            buf_ptr: ptr::null_mut(),
            len: 0,
            seq: 0,
            cmd: 0,
            cid: 0,
            initialized: 0,
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn bytes_read(&self) -> usize {
        if self.buf_ptr.is_null() {
            0
        } else {
            // SAFETY: `buf_ptr` always points inside `data` while initialized.
            unsafe { self.buf_ptr.offset_from(self.data.as_ptr()) as usize }
        }
    }

    pub fn needs_more_data(&self) -> bool {
        self.bytes_read() < self.len as usize
    }

    pub fn message_data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }

    fn append(&mut self, data: &[u8]) {
        let offset = self.bytes_read();
        self.data[offset..offset + data.len()].copy_from_slice(data);
        // SAFETY: `offset + data.len()` stays within `data` due to checked callers.
        self.buf_ptr = unsafe { self.data.as_mut_ptr().add(offset + data.len()) };
    }

    fn start_message(&mut self, packet: &Packet) {
        let len = packet.message_len();
        *self = Self::new();
        self.seq = 0;
        self.buf_ptr = self.data.as_mut_ptr();
        self.len = len as u32;
        self.cmd = packet.header_byte();
        self.cid = packet.cid;
        self.initialized = 1;
        self.append(&packet.init_data()[..min(len, INIT_DATA_SIZE)]);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub fn process_packet(packet: &Packet, state: &mut State) -> i32 {
    match packet.packet_type() {
        TYPE_INIT => process_init(packet, state),
        TYPE_CONT => process_cont(packet, state),
        _ => ERR_INVALID_CMD as i32,
    }
}

fn process_init(packet: &Packet, state: &mut State) -> i32 {
    if state.initialized != 0 && packet.cid != state.cid && state.cmd == INIT {
        return ERR_CHANNEL_BUSY as i32;
    }

    if state.initialized != 0
        && packet.cid != state.cid
        && packet.header_byte() != INIT
        && state.cmd != INIT
    {
        return ERR_CHANNEL_BUSY as i32;
    }

    if state.initialized != 0 && packet.cid == state.cid && packet.header_byte() != INIT {
        return ERR_INVALID_SEQ as i32;
    }

    if packet.message_len() > MAX_MESSAGE_LEN {
        return ERR_INVALID_LEN as i32;
    }

    state.start_message(packet);
    ERR_NONE as i32
}

fn process_cont(packet: &Packet, state: &mut State) -> i32 {
    if state.initialized == 0 {
        return ERR_IGNORE as i32;
    }

    if state.cid != packet.cid {
        return ERR_CHANNEL_BUSY as i32;
    }

    if state.seq != packet.sequence() {
        return ERR_INVALID_SEQ as i32;
    }

    let already_read = state.bytes_read();
    if already_read >= state.len as usize || already_read + CONT_DATA_SIZE > state.data.len() {
        return ERR_INVALID_LEN as i32;
    }

    state.seq = state.seq.wrapping_add(1);
    let remaining = state.len as usize - already_read;
    state.append(&packet.cont_data()[..min(remaining, CONT_DATA_SIZE)]);
    ERR_NONE as i32
}

pub fn fragment_message<E>(
    cmd: u8,
    data: &[u8],
    cid: u32,
    mut push: impl FnMut(&Packet) -> Result<(), E>,
) -> Result<(), E> {
    debug_assert!(data.len() <= MAX_MESSAGE_LEN);
    debug_assert!(data.len() <= u16::MAX as usize);

    let mut sent = 0usize;
    let mut packet = Packet::zeroed();
    let init_len = min(data.len(), INIT_DATA_SIZE);
    packet.set_init(cid, cmd, data.len(), &data[..init_len]);
    push(&packet)?;
    sent += init_len;

    let mut seq = 0u8;
    while sent < data.len() {
        let cont_len = min(data.len() - sent, CONT_DATA_SIZE);
        packet.set_cont(cid, seq, &data[sent..sent + cont_len]);
        push(&packet)?;
        sent += cont_len;
        seq = seq.wrapping_add(1);
    }
    Ok(())
}

pub fn fragment_error<E>(
    err: u8,
    cid: u32,
    push: impl FnMut(&Packet) -> Result<(), E>,
) -> Result<(), E> {
    fragment_message(ERROR, &[err], cid, push)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;
    use std::vec::Vec;

    fn packet_bytes(packet: &Packet) -> [u8; REPORT_SIZE] {
        let mut out = [0u8; REPORT_SIZE];
        out[..4].copy_from_slice(&packet.cid.to_ne_bytes());
        out[4..].copy_from_slice(&packet.payload);
        out
    }

    #[test]
    fn test_fragment_single_packet() {
        let mut packets = Vec::<Packet>::new();
        fragment_message(PING, &hex!("01020304"), 0x11223344, |packet| {
            packets.push(*packet);
            Ok::<(), ()>(())
        })
        .unwrap();

        assert_eq!(packets.len(), 1);
        assert_eq!(
            packet_bytes(&packets[0])[..11],
            hex!("4433221181000401020304")
        );
    }

    #[test]
    fn test_fragment_multi_packet() {
        let payload = [0x55u8; INIT_DATA_SIZE + 3];
        let mut packets = Vec::<Packet>::new();
        fragment_message(PING, &payload, 7, |packet| {
            packets.push(*packet);
            Ok::<(), ()>(())
        })
        .unwrap();

        assert_eq!(packets.len(), 2);
        assert_eq!(packets[0].header_byte(), PING);
        assert_eq!(packets[1].sequence(), 0);
        assert_eq!(&packets[1].cont_data()[..3], &[0x55, 0x55, 0x55]);
    }

    #[test]
    fn test_process_single_packet_message() {
        let mut state = State::new();
        let mut packet = Packet::zeroed();
        packet.set_init(0x01020304, PING, 4, &hex!("aabbccdd"));

        assert_eq!(process_packet(&packet, &mut state), ERR_NONE as i32);
        assert!(!state.needs_more_data());
        assert_eq!(state.cmd, PING);
        assert_eq!(state.cid, 0x01020304);
        assert_eq!(state.message_data(), &hex!("aabbccdd"));
    }

    #[test]
    fn test_process_continuation_message() {
        let payload = [0x11u8; INIT_DATA_SIZE + 2];
        let mut state = State::new();
        let mut init = Packet::zeroed();
        init.set_init(9, PING, payload.len(), &payload[..INIT_DATA_SIZE]);
        let mut cont = Packet::zeroed();
        cont.set_cont(9, 0, &payload[INIT_DATA_SIZE..]);

        assert_eq!(process_packet(&init, &mut state), ERR_NONE as i32);
        assert!(state.needs_more_data());
        assert_eq!(process_packet(&cont, &mut state), ERR_NONE as i32);
        assert!(!state.needs_more_data());
        assert_eq!(state.message_data(), payload.as_slice());
    }

    #[test]
    fn test_process_unsolicited_continuation_ignored() {
        let mut state = State::new();
        let mut packet = Packet::zeroed();
        packet.set_cont(1, 0, &hex!("0102"));
        assert_eq!(process_packet(&packet, &mut state), ERR_IGNORE as i32);
    }

    #[test]
    fn test_process_invalid_sequence() {
        let payload = [0x22u8; INIT_DATA_SIZE + 1];
        let mut state = State::new();
        let mut init = Packet::zeroed();
        init.set_init(9, PING, payload.len(), &payload[..INIT_DATA_SIZE]);
        let mut cont = Packet::zeroed();
        cont.set_cont(9, 1, &payload[INIT_DATA_SIZE..]);

        assert_eq!(process_packet(&init, &mut state), ERR_NONE as i32);
        assert_eq!(process_packet(&cont, &mut state), ERR_INVALID_SEQ as i32);
    }

    #[test]
    fn test_process_max_len_and_overflow() {
        let mut max_packet = Packet::zeroed();
        max_packet.set_init(1, PING, MAX_MESSAGE_LEN, &[]);
        assert_eq!(
            process_packet(&max_packet, &mut State::new()),
            ERR_NONE as i32
        );

        let mut packet = Packet::zeroed();
        packet.set_init(1, PING, MAX_MESSAGE_LEN + 1, &[]);
        assert_eq!(
            process_packet(&packet, &mut State::new()),
            ERR_INVALID_LEN as i32
        );
    }

    #[test]
    fn test_fragment_error() {
        let mut packets = Vec::<Packet>::new();
        fragment_error(ERR_INVALID_CMD, 5, |packet| {
            packets.push(*packet);
            Ok::<(), ()>(())
        })
        .unwrap();

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].header_byte(), ERROR);
        assert_eq!(packets[0].message_len(), 1);
        assert_eq!(packets[0].init_data()[0], ERR_INVALID_CMD);
    }
}
