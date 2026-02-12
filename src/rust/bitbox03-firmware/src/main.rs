#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
unsafe fn main() -> ! {
    unsafe { st_drivers_sys::platform_init() };
    let tx_buf = b"hello, world\r\n";
    loop {
        unsafe {
            st_drivers_sys::HAL_UART_Transmit(
                &raw mut st_drivers_sys::huart1 as *mut _,
                tx_buf.as_ptr() as *const _,
                tx_buf.len() as u16,
                1000,
            );
            st_drivers_sys::HAL_Delay(1000);
        }
        cortex_m::asm::nop();
    }
}
