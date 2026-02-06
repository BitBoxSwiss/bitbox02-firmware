// SPDX-License-Identifier: Apache-2.0

use bitbox_executor::Executor;
use bitbox02::ringbuffer::RingBuffer;
use bitbox02::uart::USART_0_BUFFER_SIZE;
use bitbox02::usb_packet::USB_FRAME;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

const UART_OUT_BUF_LEN: u32 = 2048;

static EXECUTOR: Executor = Executor::new();

fn main_loop(hal: &mut impl crate::hal::Hal) -> ! {
    static ORIENTATION_CHOSEN: AtomicBool = AtomicBool::new(false);

    // Set the size of uart_read_buf to the size of the ringbuffer in the UART driver so we can read
    // out all bytes
    let mut uart_read_buf = [0u8; USART_0_BUFFER_SIZE as usize];
    let mut uart_read_buf_len = 0u16;

    let mut uart_write_buf = [0u8; UART_OUT_BUF_LEN as usize];
    let mut uart_write_queue = RingBuffer::new(&mut uart_write_buf);

    // If the bootloader has booted the BLE chip, the BLE chip isn't aware of the name according to
    // the fw. Send it over.
    let device_name = bitbox02::memory::get_device_name();
    bitbox02::da14531::set_name(&device_name, &mut uart_write_queue);

    // This starts the async orientation screen workflow, which is processed by the loop below.
    EXECUTOR
        .spawn(async {
            crate::workflow::orientation_screen::orientation_screen().await;
            ORIENTATION_CHOSEN.store(true, Ordering::Relaxed);
        })
        .detach();

    let mut hww_data = None;
    let mut hww_frame: USB_FRAME = unsafe { MaybeUninit::zeroed().assume_init() };

    #[cfg(feature = "app-u2f")]
    bitbox02::u2f_packet::init();
    #[cfg(feature = "app-u2f")]
    let mut u2f_data = None;
    #[cfg(feature = "app-u2f")]
    let mut u2f_frame: USB_FRAME = unsafe { MaybeUninit::zeroed().assume_init() };

    if !bitbox02::memory::ble_enabled() {
        crate::communication_mode::ble_disable();
    }

    loop {
        // Do UART I/O
        if crate::communication_mode::ble_enabled(hal) {
            if uart_read_buf_len < uart_read_buf.len() as u16 || uart_write_queue.len() > 0 {
                bitbox02::uart::poll(
                    Some(&mut uart_read_buf),
                    Some(&mut uart_read_buf_len),
                    &mut uart_write_queue,
                )
            }
        }

        // Check if there is outgoing data
        if hww_data.is_none() {
            hww_data = bitbox02::queue::pull_hww();
        }

        // Generate u2f timeout packets
        #[cfg(feature = "app-u2f")]
        {
            // Generate timeout packets
            let mut timeout_cid = 0u32;
            while bitbox02::u2f_packet::timeout_get(&mut timeout_cid) {
                bitbox02::u2f_packet::timeout(timeout_cid);
            }
            if u2f_data.is_none() {
                u2f_data = bitbox02::queue::pull_u2f();
                // If USB stack was locked and there is no more messages to send out, time to
                // unlock it.
                if u2f_data.is_none() && bitbox02::usb_processing::locked_u2f() {
                    bitbox02::usb_processing::unlock();
                }
            }
        }

        // Do USB Input
        if hww_data.is_none() && bitbox02::hid_hww::read(&mut hww_frame) {
            if bitbox02::usb_packet::process(&hww_frame) {
                if crate::communication_mode::ble_enabled(hal) {
                    // Enqueue a power down command to the da14531
                    bitbox02::da14531::power_down(&mut uart_write_queue);
                    // Flush out the power down command. This will be the last UART communication
                    // we do.
                    while uart_write_queue.len() > 0 {
                        bitbox02::uart::poll(None, None, &mut uart_write_queue);
                    }
                    crate::communication_mode::ble_disable();
                }
            } else {
                // log!("usb_packet_process: invalid");
            }
        }
        #[cfg(feature = "app-u2f")]
        if u2f_data.is_none() && bitbox02::hid_u2f::read(&mut u2f_frame) {
            bitbox02::u2f_packet::process(&u2f_frame);
        }

        // Do UART Output
        if crate::communication_mode::ble_enabled(hal) {
            if let Some(frame) = bitbox02::da14531_protocol::poll(
                &mut uart_read_buf,
                &mut uart_read_buf_len,
                &mut hww_data,
                &mut uart_write_queue,
            ) {
                // log!("got frame, calling handler");
                bitbox02::da14531_handler::handler(frame, &mut uart_write_queue);
            }
        }

        // Do USB Output
        if let Some(data) = &mut hww_data
            && !crate::communication_mode::ble_enabled(hal)
        {
            if bitbox02::hid_hww::write_poll(data) {
                hww_data = None;
            }
        }
        #[cfg(feature = "app-u2f")]
        if let Some(data) = &mut u2f_data {
            if bitbox02::hid_u2f::write_poll(data) {
                u2f_data = None;
            }
        }

        /* First, process all the incoming USB traffic. */
        bitbox02::usb_processing::process_hww();
        #[cfg(feature = "app-u2f")]
        bitbox02::usb_processing::process_u2f();

        /*
         * If USB has generated events at the application level,
         * process them now.
         */
        #[cfg(feature = "app-u2f")]
        bitbox02::u2f::process();

        bitbox02::screen::process();

        /* And finally, run the high-level event processing. */
        #[cfg(feature = "app-u2f")]
        crate::workflow::u2f_c_api::workflow_spin();

        crate::async_usb::spin();

        // Run async executor
        EXECUTOR.try_tick();

        if ORIENTATION_CHOSEN.swap(false, Ordering::Relaxed) {
            // hww handler in usb_process must be setup before we can allow ble connections
            if let Ok(bitbox02::memory::Platform::BitBox02Plus) = bitbox02::memory::get_platform() {
                let product = bitbox02::platform::product();
                bitbox02::da14531_handler::set_product(product);
                bitbox02::da14531::set_product(product, &mut uart_write_queue)
            }
            bitbox02::usb::start();
        }
    }
}

//
// C interface
//

#[unsafe(no_mangle)]
pub extern "C" fn rust_main_loop() -> ! {
    main_loop(&mut crate::hal::BitBox02Hal::new())
}
