// SPDX-License-Identifier: Apache-2.0

use bitbox_u2fhid::{
    ERR_CHANNEL_BUSY, ERR_IGNORE, ERR_INVALID_LEN, ERR_INVALID_SEQ, ERR_NONE, ERR_OTHER, Packet,
    State,
};
#[cfg(feature = "app-u2f")]
use bitbox_u2fhid::{ERR_MSG_TIMEOUT, TYPE_INIT, VENDOR_FIRST};
use bitbox_usb_report_queue::{RustUsbReportQueue, UsbReportQueueError};
use core::ffi::c_void;

/// cbindgen:ignore
mod c_ffi {
    use bitbox_usb_report_queue::RustUsbReportQueue;

    #[repr(C)]
    pub struct UsbProcessing {
        _private: [u8; 0],
    }

    unsafe extern "C" {
        pub fn usb_processing_enqueue(
            ctx: *mut UsbProcessing,
            buf: *const u8,
            length: usize,
            cmd: u8,
            cid: u32,
        ) -> bool;
        pub fn usb_processing_hww() -> *mut UsbProcessing;
        pub fn usb_processing_out_queue(ctx: *mut UsbProcessing) -> *mut RustUsbReportQueue;
        #[cfg(feature = "app-u2f")]
        pub fn usb_processing_u2f() -> *mut UsbProcessing;
    }
}

#[cfg(feature = "app-u2f")]
use c_ffi::usb_processing_u2f;
use c_ffi::{UsbProcessing, usb_processing_enqueue, usb_processing_hww, usb_processing_out_queue};

static mut HWW_IN_STATE: State = State::new();

#[cfg(feature = "app-u2f")]
static mut U2F_IN_STATE: State = State::new();

#[cfg(feature = "app-u2f")]
#[derive(Copy, Clone)]
struct FrameCounter {
    cid: u32,
    counter: u8,
}

#[cfg(feature = "app-u2f")]
const NUM_TIMEOUT_COUNTERS: usize = 3;

#[cfg(feature = "app-u2f")]
static mut U2F_TIMEOUT_COUNTERS: [FrameCounter; NUM_TIMEOUT_COUNTERS] =
    [FrameCounter { cid: 0, counter: 0 }; NUM_TIMEOUT_COUNTERS];

enum TimeoutMode {
    None,
    #[cfg(feature = "app-u2f")]
    U2f,
}

unsafe fn packet_ref<'a>(frame: *const c_void) -> &'a Packet {
    // SAFETY: the C caller passes a pointer to a `USB_FRAME`, which has the same layout as `Packet`.
    unsafe { &*frame.cast::<Packet>() }
}

unsafe fn state_mut<'a>(state: *mut c_void) -> &'a mut State {
    // SAFETY: the C caller passes a pointer to a `State`, which has the same layout as
    // `bitbox_u2fhid::State`.
    unsafe { &mut *state.cast::<State>() }
}

fn push_packet(queue: *mut RustUsbReportQueue, packet: &Packet) -> Result<(), UsbReportQueueError> {
    let err = unsafe {
        bitbox_usb_report_queue::rust_usb_report_queue_push(queue, packet as *const _ as *const u8)
    };
    if err == UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE {
        Ok(())
    } else {
        Err(err)
    }
}

fn frame_reply(
    cmd: u8,
    data: &[u8],
    len: u32,
    cid: u32,
    queue: *mut RustUsbReportQueue,
) -> UsbReportQueueError {
    let len = len as usize;
    debug_assert_eq!(data.len(), len);
    match bitbox_u2fhid::fragment_message(cmd, data, cid, |packet| push_packet(queue, packet)) {
        Ok(()) => UsbReportQueueError::USB_REPORT_QUEUE_ERR_NONE,
        Err(err) => err,
    }
}

fn queue_err(err: u8, cid: u32, queue: *mut RustUsbReportQueue) {
    let _ = bitbox_u2fhid::fragment_error(err, cid, |packet| push_packet(queue, packet));
}

fn clear_queue(queue: *mut RustUsbReportQueue) {
    unsafe {
        bitbox_usb_report_queue::rust_usb_report_queue_clear(queue);
    }
}

#[cfg(feature = "app-u2f")]
fn timeout_reset(cid: u32) {
    unsafe {
        let counters = core::ptr::addr_of_mut!(U2F_TIMEOUT_COUNTERS).cast::<FrameCounter>();
        for i in 0..NUM_TIMEOUT_COUNTERS {
            let counter = &mut *counters.add(i);
            if counter.cid == cid {
                counter.counter = 0;
            }
        }
    }
}

#[cfg(feature = "app-u2f")]
fn timeout_disable(cid: u32) {
    unsafe {
        let counters = core::ptr::addr_of_mut!(U2F_TIMEOUT_COUNTERS).cast::<FrameCounter>();
        for i in 0..NUM_TIMEOUT_COUNTERS {
            let counter = &mut *counters.add(i);
            if counter.cid == cid {
                counter.cid = 0;
                counter.counter = 0;
            }
        }
    }
}

#[cfg(feature = "app-u2f")]
fn timeout_enable(cid: u32) {
    unsafe {
        let counters = core::ptr::addr_of_mut!(U2F_TIMEOUT_COUNTERS).cast::<FrameCounter>();
        for i in 0..NUM_TIMEOUT_COUNTERS {
            let counter = &mut *counters.add(i);
            if counter.cid == 0 {
                counter.cid = cid;
                counter.counter = 0;
                return;
            }
        }
    }
}

#[cfg_attr(not(feature = "app-u2f"), allow(unused_variables))]
fn reset_state(state: &mut State, queue: *mut RustUsbReportQueue, timeout_mode: TimeoutMode) {
    clear_queue(queue);
    #[cfg(feature = "app-u2f")]
    if matches!(timeout_mode, TimeoutMode::U2f) {
        timeout_disable(state.cid);
    }
    state.clear();
    #[cfg(feature = "app-u2f")]
    if matches!(timeout_mode, TimeoutMode::U2f) {
        state.buf_ptr = state.data.as_mut_ptr();
    }
}

fn process_received_packet(
    packet: &Packet,
    state: &mut State,
    ctx: *mut UsbProcessing,
    complete_result: bool,
    timeout_mode: TimeoutMode,
) -> bool {
    let queue = unsafe { usb_processing_out_queue(ctx) };
    match bitbox_u2fhid::process_packet(packet, state) as u8 {
        ERR_IGNORE => false,
        ERR_INVALID_SEQ => {
            reset_state(state, queue, timeout_mode);
            queue_err(ERR_INVALID_SEQ, packet.cid, queue);
            false
        }
        ERR_CHANNEL_BUSY => {
            queue_err(ERR_CHANNEL_BUSY, packet.cid, queue);
            false
        }
        ERR_INVALID_LEN => {
            reset_state(state, queue, timeout_mode);
            queue_err(ERR_INVALID_LEN, packet.cid, queue);
            false
        }
        ERR_NONE => {
            #[cfg(feature = "app-u2f")]
            if matches!(timeout_mode, TimeoutMode::U2f) {
                if packet.packet_type() == TYPE_INIT && packet.header_byte() < VENDOR_FIRST {
                    timeout_enable(packet.cid);
                }
                timeout_reset(packet.cid);
            }

            if state.needs_more_data() {
                return true;
            }

            if unsafe {
                usb_processing_enqueue(
                    ctx,
                    state.data.as_ptr(),
                    state.len as usize,
                    state.cmd,
                    state.cid,
                )
            } {
                reset_state(state, queue, timeout_mode);
                return complete_result;
            }

            #[cfg(feature = "app-u2f")]
            if matches!(timeout_mode, TimeoutMode::U2f) {
                timeout_disable(packet.cid);
            }
            reset_state(state, queue, timeout_mode);
            queue_err(ERR_CHANNEL_BUSY, packet.cid, queue);
            false
        }
        _ => {
            reset_state(state, queue, timeout_mode);
            queue_err(ERR_OTHER, packet.cid, queue);
            false
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2fhid_frame_reply(
    cmd: u8,
    data: *const u8,
    len: u32,
    cid: u32,
    queue: *mut RustUsbReportQueue,
) -> UsbReportQueueError {
    let data = if len == 0 {
        &[]
    } else {
        // SAFETY: `data` points to `len` readable bytes for the duration of the call.
        unsafe { core::slice::from_raw_parts(data, len as usize) }
    };
    frame_reply(cmd, data, len, cid, queue)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2fhid_frame_prepare_err(
    err: u8,
    cid: u32,
    queue: *mut RustUsbReportQueue,
) -> UsbReportQueueError {
    frame_reply(bitbox_u2fhid::ERROR, &[err], 1, cid, queue)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2fhid_frame_process(
    frame: *const c_void,
    state: *mut c_void,
) -> i32 {
    let frame = unsafe { packet_ref(frame) };
    let state = unsafe { state_mut(state) };
    bitbox_u2fhid::process_packet(frame, state)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_usb_packet_process(frame: *const c_void) -> bool {
    let frame = unsafe { packet_ref(frame) };
    // SAFETY: single-threaded firmware access, mirroring the old C static state.
    let state = unsafe { &mut *core::ptr::addr_of_mut!(HWW_IN_STATE) };
    process_received_packet(
        frame,
        state,
        unsafe { usb_processing_hww() },
        true,
        TimeoutMode::None,
    )
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_process(frame: *const c_void) -> bool {
    let frame = unsafe { packet_ref(frame) };
    // SAFETY: single-threaded firmware access, mirroring the old C static state.
    let state = unsafe { &mut *core::ptr::addr_of_mut!(U2F_IN_STATE) };
    process_received_packet(
        frame,
        state,
        unsafe { usb_processing_u2f() },
        false,
        TimeoutMode::U2f,
    )
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_init() {
    // SAFETY: single-threaded firmware access, mirroring the old C static state.
    let state = unsafe { &mut *core::ptr::addr_of_mut!(U2F_IN_STATE) };
    state.clear();
    state.buf_ptr = state.data.as_mut_ptr();
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_timeout_enable(cid: u32) {
    timeout_enable(cid);
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_timeout_get(cid_out: *mut u32) -> bool {
    if cid_out.is_null() {
        return false;
    }
    unsafe {
        let counters = core::ptr::addr_of!(U2F_TIMEOUT_COUNTERS).cast::<FrameCounter>();
        for i in 0..NUM_TIMEOUT_COUNTERS {
            let counter = &*counters.add(i);
            *cid_out = counter.cid;
            if counter.cid != 0 && counter.counter >= 5 {
                return true;
            }
        }
    }
    false
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_timeout_tick() {
    unsafe {
        let counters = core::ptr::addr_of_mut!(U2F_TIMEOUT_COUNTERS).cast::<FrameCounter>();
        for i in 0..NUM_TIMEOUT_COUNTERS {
            let counter = &mut *counters.add(i);
            if counter.cid != 0 {
                counter.counter = counter.counter.saturating_add(1);
            }
        }
    }
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_u2f_packet_timeout(cid: u32) {
    timeout_disable(cid);

    // SAFETY: single-threaded firmware access, mirroring the old C static state.
    let state = unsafe { &mut *core::ptr::addr_of_mut!(U2F_IN_STATE) };
    let queue = unsafe { usb_processing_out_queue(usb_processing_u2f()) };
    if cid == state.cid {
        reset_state(state, queue, TimeoutMode::U2f);
    }
    queue_err(ERR_MSG_TIMEOUT, cid, queue);
}
