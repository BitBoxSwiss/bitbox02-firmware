#![no_std]
#![no_main]

#[cfg(all(feature = "multi", feature = "btc-only"))]
compile_error!("Only one firmware variant can be enabled at a time.");

#[cfg(not(any(feature = "multi", feature = "btc-only")))]
compile_error!("One firmware variant must be enabled.");

use bitbox02::memory::Platform;

#[unsafe(no_mangle)]
static mut __stack_chk_guard: u32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    bitbox02::system::init_mcu();
    bitbox02::system::init();
    bitbox02::platform::init();
    unsafe {
        __stack_chk_guard = bitbox02::system::common_stack_chk_guard();
    }
    bitbox02::screen::init(
        bitbox02::oled::set_pixel,
        bitbox02::oled::mirror,
        bitbox02::oled::clear_buffer,
    );
    bitbox02::screen::splash();
    bitbox02::qtouch::init();
    bitbox02::system::common_main();
    bitbox02::smarteeprom::init();
    if matches!(bitbox02::memory::get_platform(), Ok(Platform::BitBox02Plus)) {
        bitbox02::da14531_protocol::init();
    }
    bitbox02::usb_processing::init();
    // Setup usb_processing handlers
    bitbox02::hww::setup();
    #[cfg(feature = "multi")]
    bitbox02::u2f::device_setup();
    bitbox02_rust_c::main_loop()
}
