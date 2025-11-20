// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Since we are targeting embedded we exclude the standard library by default
#![cfg_attr(
    not(any(
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    )),
    no_std
)]
// When compiling for testing we allow certain warnings.
#![cfg_attr(test, allow(unused_imports, dead_code))]

//use alloc::boxed::Box;
use bitbox02::ringbuffer::RingBuffer;
use bitbox02::uart::USART_0_BUFFER_SIZE;
use bitbox02::usb::USB_REPORT_SIZE;
use core::sync::atomic::{AtomicBool, Ordering};
//use core::task::Poll;
use util::log::log;

use async_channel::Receiver;
use bitbox02_executor::StaticExecutor;

mod pb {
    include!("./shiftcrypto.bitbox02.rs");
}
mod pb_backup {
    include!("./shiftcrypto.bitbox02.backups.rs");
}

#[macro_use]
pub mod general;
pub mod async_usb;
pub mod attestation;
pub mod backup;
pub mod bb02_async;
mod bip32;
pub mod bip39;
pub mod hal;
pub mod hash;
pub mod hww;
pub mod keystore;
pub mod salt;
pub mod secp256k1;
#[cfg(feature = "app-u2f")]
mod u2f;
mod version;
pub mod workflow;
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
mod xpubcache;

// for `format!`
#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate bitbox_aes;

#[cfg_attr(feature = "c-unit-testing", allow(unused))]
const UART_OUT_BUF_LEN: u32 = 2048;

static EXECUTOR: StaticExecutor = StaticExecutor::new();

#[cfg_attr(feature = "c-unit-testing", allow(unused))]
fn main_loop() -> ! {
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
            workflow::orientation_screen::orientation_screen().await;
            ORIENTATION_CHOSEN.store(true, Ordering::Relaxed);
        })
        .detach();

    EXECUTOR
        .spawn(async {
            util::log::log!("hello world");
        })
        .detach();

    let mut hww_data = None;
    let mut hww_frame = [0u8; USB_REPORT_SIZE as usize];

    #[cfg(feature = "app-u2f")]
    bitbox02::u2f_packet::init();
    #[cfg(feature = "app-u2f")]
    let mut u2f_data = None;
    #[cfg(feature = "app-u2f")]
    let mut u2f_frame = [0u8; USB_REPORT_SIZE as usize];

    if !bitbox02::memory::ble_enabled() {
        bitbox02::communication_mode::ble_disable();
    }

    loop {
        // Do UART I/O
        if bitbox02::communication_mode::ble_enabled() {
            if uart_read_buf_len < uart_read_buf.len() as u16 || uart_write_queue.len() > 0 {
                bitbox02::uart::poll(
                    Some(&mut uart_read_buf),
                    Some(&mut uart_read_buf_len),
                    Some(&mut uart_write_queue),
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
                if bitbox02::communication_mode::ble_enabled() {
                    // Enqueue a power down command to the da14531
                    bitbox02::da14531::power_down(&mut uart_write_queue);
                    // Flush out the power down command. This will be the last UART communication
                    // we do.
                    while uart_write_queue.len() > 0 {
                        bitbox02::uart::poll(None, None, Some(&mut uart_write_queue));
                    }
                    bitbox02::communication_mode::ble_disable();
                }
            } else {
                log!("usb_packet_process: invalid");
            }
        }
        #[cfg(feature = "app-u2f")]
        if u2f_data.is_none() && bitbox02::hid_u2f::read(&mut u2f_frame) {
            bitbox02::u2f_packet::process(&u2f_frame);
        }

        // Do UART Output
        if bitbox02::communication_mode::ble_enabled() {
            if hww_data.is_some() {
                log!("have hww data to send {:?}", hww_data);
            }
            if let Some(frame) = bitbox02::da14531_protocol::poll(
                &mut uart_read_buf,
                &mut uart_read_buf_len,
                &mut hww_data,
                &mut uart_write_queue,
            ) {
                log!("got frame, calling handler");
                bitbox02::da14531_handler::handler(frame, &mut uart_write_queue);
            }
        }

        // Do USB Output
        if let Some(data) = &mut hww_data
            && !bitbox02::communication_mode::ble_enabled()
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
        //unsafe { bitbox02_rust_c::workflow::rust_workflow_spin() }
        //async_usb::spin();

        // Run async exuecutor
        EXECUTOR.try_tick();

        if ORIENTATION_CHOSEN.swap(false, Ordering::Relaxed) {
            // hww handler in usb_process must be setup before we can allow ble connections
            if let Ok(bitbox02::memory::Platform::BitBox02Plus) = bitbox02::memory::get_platform() {
                let (product, product_len) = bitbox02::platform::product();
                bitbox02::da14531_handler::set_product(product, product_len);
                bitbox02::da14531::set_product(product, &mut uart_write_queue)
            }
            bitbox02::usb::start();
        }
    }
}

// Spawns a task and returns the receiving end of a one shot channel
pub fn spawn<T>(fut: impl Future<Output = T> + 'static) -> Receiver<T>
where
    T: 'static,
{
    let (sender, receiver) = async_channel::bounded(1);
    EXECUTOR
        .spawn(async move { sender.send(fut.await).await })
        .detach();
    receiver
}

//
// C interface
//

/// `private_key_out` must be 32 bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rust_noise_generate_static_private_key(
    mut private_key_out: util::bytes::BytesMut,
) {
    let key = bitbox02_noise::generate_static_private_key::<hww::noise::BB02Random32>();
    private_key_out.as_mut().copy_from_slice(&key[..]);
}

#[unsafe(no_mangle)]
#[cfg(not(feature = "c-unit-testing"))]
pub extern "C" fn rust_main_loop() -> ! {
    main_loop()
}
