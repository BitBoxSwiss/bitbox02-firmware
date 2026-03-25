#![no_std]
#![no_main]

extern crate bitbox02_rust_c as _;

#[cfg(feature = "platform-bitbox02plus")]
use bitbox_bytequeue::{ByteQueue, RustByteQueue};
#[cfg(feature = "platform-bitbox02plus")]
use bitbox_framed_serial_link::{ProtocolPacketType, protocol_format};
#[cfg(feature = "platform-bitbox02plus")]
use core::ffi::c_int;

unsafe extern "C" {
    static mut __stack_chk_guard: u32;
}

const SCB_VTOR_ADDR: *const u32 = 0xE000_ED08usize as *const u32;
// Must fit bond_db.
#[cfg(feature = "platform-bitbox02plus")]
const UART_OUT_BUF_LEN: usize = 2048;

#[cfg(feature = "platform-bitbox02plus")]
#[unsafe(no_mangle)]
pub static mut bootloader_pairing_request: c_int = 0;

#[cfg(feature = "platform-bitbox02plus")]
#[unsafe(no_mangle)]
pub static mut bootloader_pairing_code_bytes: [u8; 4] = [0; 4];

#[cfg(feature = "platform-bitbox02plus")]
#[unsafe(no_mangle)]
pub static mut uart_write_queue: *mut RustByteQueue = core::ptr::null_mut();

fn stack_chk_guard() -> u32 {
    let mut random = [0u8; 32];
    bitbox02::random::mcu_32_bytes(&mut random);
    u32::from_le_bytes(random[..4].try_into().unwrap())
}

#[cfg(feature = "platform-bitbox02plus")]
unsafe fn uart_write_queue_mut() -> &'static mut ByteQueue {
    unsafe { &mut *uart_write_queue.cast::<ByteQueue>() }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // When in bootloader mode, the vector table should be 0. If not, halt.
    if unsafe { core::ptr::read_volatile(SCB_VTOR_ADDR) } != 0 {
        loop {}
    }

    // Order is important
    bitbox02::system::init_mcu();
    bitbox02::bootloader::mpu_regions_init();
    bitbox02::bootloader::init();
    bitbox02::platform::init();
    unsafe {
        __stack_chk_guard = stack_chk_guard();
    }
    bitbox02::screen::init(
        bitbox02::oled::set_pixel,
        bitbox02::oled::mirror,
        bitbox02::oled::clear_buffer,
    );
    #[cfg(any(feature = "bootloader-devdevice", feature = "platform-bitbox02plus"))]
    bitbox02::qtouch::init();
    bitbox02::bootloader::jump();

    // If did not jump to firmware code, begin UART/USB processing
    let mut hww_data: Option<[u8; 64]> = None;
    let mut hww_frame = bitbox02::usb_packet::USB_FRAME::default();

    #[cfg(feature = "platform-bitbox02plus")]
    let mut uart_read_buf = [0u8; bitbox02::uart::USART_0_BUFFER_SIZE as usize];
    #[cfg(feature = "platform-bitbox02plus")]
    let mut uart_read_buf_len = 0u16;

    #[cfg(feature = "platform-bitbox02plus")]
    unsafe {
        uart_write_queue = bitbox_bytequeue::rust_bytequeue_init(UART_OUT_BUF_LEN);
        assert!(!uart_write_queue.is_null());
        let queue = uart_write_queue_mut();
        if !bitbox02::memory::ble_enabled() {
            bitbox02_rust_c::ble_disable();
        }

        // Set product to bootloader string, this is necessary if we have rebooted from firmware. Must
        // be done after usb_processing is initalized to avoid getting request from the app to early.
        let product = bitbox02::platform::product();
        bitbox02::da14531_handler::set_product(product);
        bitbox_da14531::set_product(product, queue);

        // Set device name, the MCU and BLE chip will probably not have the same name after a reset of
        // only the MCU.
        let name = bitbox02::memory::random_name();
        bitbox_da14531::set_name(&name, queue);

        // Ask for the current conection state
        bitbox_da14531::get_connection_state(queue);

        bitbox02::da14531_protocol::init();
    }
    bitbox02::usb_processing::init();

    loop {
        // Do UART I/O
        #[cfg(feature = "platform-bitbox02plus")]
        unsafe {
            if bitbox02_rust_c::ble_enabled() {
                let queue = uart_write_queue_mut();
                if uart_read_buf_len < uart_read_buf.len() as u16 || queue.num() > 0 {
                    bitbox02::uart::poll(
                        Some(&mut uart_read_buf),
                        Some(&mut uart_read_buf_len),
                        queue,
                    );
                }
            }
        }

        if hww_data.is_none() {
            hww_data = bitbox02::queue::pull_hww();
        }
        if hww_data.is_none() && bitbox02::hid_hww::read(&mut hww_frame) {
            bitbox02::usb_packet::process(&hww_frame);
            #[cfg(feature = "platform-bitbox02plus")]
            unsafe {
                if bitbox02_rust_c::ble_enabled() {
                    let queue = uart_write_queue_mut();
                    // Enqueue a power down command to the da14531
                    bitbox_da14531::power_down(queue);
                    // Flush out the power down command. This will be the last UART communication we do.
                    while queue.num() > 0 {
                        bitbox02::uart::poll(None, None, queue);
                    }
                    bitbox02_rust_c::ble_disable();
                    bitbox02::bootloader::render_default_screen();
                }
            }
        }

        #[cfg(feature = "platform-bitbox02plus")]
        unsafe {
            if bitbox02_rust_c::ble_enabled() {
                let queue = uart_write_queue_mut();
                if let Some(frame) = bitbox02::da14531_protocol::poll(
                    &mut uart_read_buf,
                    &mut uart_read_buf_len,
                    &mut hww_data,
                    queue,
                ) {
                    bitbox02::da14531_handler::handler(frame, queue);
                }
            }
        }

        #[cfg(feature = "platform-bitbox02plus")]
        let usb_can_write = !bitbox02_rust_c::ble_enabled();
        #[cfg(not(feature = "platform-bitbox02plus"))]
        let usb_can_write = true;
        if usb_can_write {
            if let Some(data) = hww_data.as_ref() {
                if bitbox02::hid_hww::write_poll(data) {
                    hww_data = None;
                }
            }
        }

        bitbox02::usb_processing::process_hww();

        #[cfg(feature = "platform-bitbox02plus")]
        unsafe {
            bitbox02::qtouch::process();
            if bootloader_pairing_request != 0 {
                if !bitbox02::qtouch::measurement_done() {
                    continue;
                }

                let top_slider = bitbox02::screen::top_slider();
                if bitbox02::qtouch::is_scroller_active(top_slider) {
                    let ok;
                    bitbox02::screen_clear();
                    if bitbox02::qtouch::get_scroller_position(top_slider) < 127 {
                        bitbox02::bootloader::render_default_screen();
                        ok = false;
                    } else {
                        bitbox02::bootloader::render_ble_confirm_screen(true);
                        ok = true;
                    }
                    let mut payload = [0u8; 18];
                    payload[0] = 11;
                    let pairing_code = bootloader_pairing_code_bytes;
                    payload[1..5].copy_from_slice(&pairing_code);
                    payload[17] = if ok { 1 } else { 0 };

                    let mut frame = [0u8; 32];
                    let frame_len =
                        protocol_format(&mut frame, ProtocolPacketType::CtrlData, &payload);
                    let queue = uart_write_queue_mut();
                    for &byte in &frame[..frame_len] {
                        queue.put(byte);
                    }
                    bootloader_pairing_request = 0;
                    bitbox02::ug_send_buffer();
                }
            }
        }
    }
}
