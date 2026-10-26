// SPDX-License-Identifier: Apache-2.0

//! USB-C connector on the discovery board: PA11/PA12 data, PG1/ADC4_IN8 VBUS sense,
//! and a 16 MHz HSE crystal. Call after `board::init()`.

use bitbox_mcu_stm32u5::pac::{self, interrupt};
use bitbox_platform_stm32u5::usb::{Driver, VbusDetect};
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Context;
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;

static VBUS_WAKER: AtomicWaker = AtomicWaker::new();
static VBUS_PRESENT: AtomicBool = AtomicBool::new(false);

pub struct Vbus {
    _sense_pin: pac::GPIOG,
    _data_pins: pac::GPIOA,
    adc: pac::ADC4,
}

impl VbusDetect for Vbus {
    fn poll_vbus(cx: &mut Context<'_>) -> bool {
        VBUS_WAKER.register(cx.waker());
        VBUS_PRESENT.load(Ordering::Acquire)
    }
}

impl Drop for Vbus {
    fn drop(&mut self) {
        NVIC::mask(pac::Interrupt::ADC4);
        self.adc.ier().reset();
        // SAFETY: Vbus owns ADC4. Reset stops its conversions before disabling its
        // clock. Other users of the analog supply and kernel clock are unaffected.
        critical_section::with(|_| {
            let rcc = unsafe { &*pac::RCC::PTR };
            rcc.ahb3rstr().modify(|_, w| w.adc4rst().set_bit());
            rcc.ahb3rstr().modify(|_, w| w.adc4rst().clear_bit());
            rcc.ahb3enr().modify(|_, w| w.adc4en().clear_bit());
        });
        VBUS_PRESENT.store(false, Ordering::Release);
        VBUS_WAKER.wake();
    }
}

#[interrupt]
fn ADC4() {
    // SAFETY: Vbus owns ADC4. Only this handler reads samples and changes the
    // watchdog window after initialization; Drop masks the IRQ before teardown.
    let adc = unsafe { &*pac::ADC4::PTR };
    if adc.isr().read().awd1().bit_is_set() {
        let sample = adc.dr().read().data().bits();
        let powered = crate::usb_vbus::present(sample, VBUS_PRESENT.load(Ordering::Relaxed));
        set_watchdog_window(adc, powered);
        adc.isr().write(|w| w.awd1().clear());
        VBUS_PRESENT.store(powered, Ordering::Release);
        VBUS_WAKER.wake();
    }
}

fn set_watchdog_window(adc: &pac::adc4::RegisterBlock, powered: bool) {
    let (low, high) = crate::usb_vbus::watchdog_window(powered);
    // SAFETY: both thresholds fit the 12-bit fields and low <= high.
    adc.awd1tr()
        .write(|w| unsafe { w.lt1().bits(low).ht1().bits(high) });
}

/// Reserve the controller, ADC4, and GPIO ports for USB. Only PA11/PA12 and PG1 are changed.
pub fn new(
    peripheral: pac::OTG_HS,
    gpioa: pac::GPIOA,
    gpiog: pac::GPIOG,
    adc: pac::ADC4,
    out_buffer: &'static mut [u8],
) -> Driver<Vbus> {
    NVIC::mask(pac::Interrupt::ADC4);
    // SAFETY: only HSE, USB GPIO clock gates, and ADC4 resources are changed; all
    // shared register read/modify/write operations are protected from interrupts.
    let (rcc, pwr) = unsafe { (&*pac::RCC::PTR, &*pac::PWR::PTR) };
    critical_section::with(|_| {
        // HSE is not the system clock source in board::init(), so enabling it
        // here leaves the existing 160 MHz MSI/PLL system clock unchanged.
        rcc.cr().modify(|_, w| w.hseon().enabled());
        rcc.ahb2enr1()
            .modify(|_, w| w.gpioaen().set_bit().gpiogen().set_bit());
        let _ = rcc.ahb2enr1().read();
        // The U5 internal HS PHY requires analog mode, not an alternate function.
        gpioa
            .moder()
            .modify(|_, w| w.mode11().analog().mode12().analog());
        gpioa
            .pupdr()
            .modify(|_, w| w.pupd11().floating().pupd12().floating());
        // PG1 is behind a 330k/27k divider: 5 V VBUS is only 0.38 V here.
        // GPIO/EXTI sensing never sees a digital high. Use ADC4 channel 8.
        gpiog.moder().modify(|_, w| w.mode1().analog());
        gpiog.pupdr().modify(|_, w| w.pupd1().floating());
        rcc.ahb3enr()
            .modify(|_, w| w.pwren().set_bit().adc4en().set_bit());
        let _ = rcc.ahb3enr().read();
        rcc.ahb3rstr().modify(|_, w| w.adc4rst().set_bit());
        rcc.ahb3rstr().modify(|_, w| w.adc4rst().clear_bit());
        pwr.svmcr().modify(|_, w| w.avm1en().enabled());
    });
    for _ in 0..100 {
        if rcc.cr().read().hserdy().is_ready() {
            break;
        }
        cortex_m::asm::delay(160_000);
    }
    assert!(
        rcc.cr().read().hserdy().is_ready(),
        "USB requires the board's 16 MHz HSE crystal"
    );
    // Keep the board's shared ADC/DAC clock selection unchanged (HCLK = 160 MHz).
    assert!(rcc.ccipr3().read().adcdacsel().is_hclk());
    wait_ready("VDDA", || pwr.svmsr().read().vdda1rdy().bit_is_set());
    critical_section::with(|_| pwr.svmcr().modify(|_, w| w.asv().set_bit()));
    adc.ccr().write(|w| w.presc().div16()); // 10 MHz ADC clock
    adc.cr().write(|w| w.advregen().enabled());
    wait_ready("ADC4 regulator", || adc.isr().read().ldordy().bit_is_set());
    adc.cr().modify(|_, w| w.adcal().start_calibration());
    wait_ready("ADC4 calibration", || {
        adc.cr().read().adcal().is_not_calibrating()
    });
    // Allow at least four ADC clock cycles between calibration and enable.
    cortex_m::asm::delay(160);
    adc.smpr().write(|w| w.smp1().cycles814_5());
    adc.chselr0().write(|w| w.chsel8().set_bit());
    // RM0456: ADC4 CFGR2.LFTRIG must be set; its reset value is reserved.
    adc.cfgr2().write(|w| w.lftrig().enabled());
    adc.cfgr1().write(|w| {
        w.cont()
            .continuous()
            .ovrmod()
            .overwrite()
            .awd1en()
            .enabled()
    });
    adc.isr().write(|w| w.adrdy().clear());
    adc.cr().modify(|_, w| w.aden().enabled());
    wait_ready("ADC4 ready", || adc.isr().read().adrdy().bit_is_set());
    adc.cr().modify(|_, w| w.adstart().start());
    wait_ready("ADC4 sample", || adc.isr().read().eoc().bit_is_set());
    let sample = adc.dr().read().data().bits();
    let powered = crate::usb_vbus::present(sample, false);
    VBUS_PRESENT.store(powered, Ordering::Release);
    set_watchdog_window(&adc, powered);
    adc.isr().write(|w| w.awd1().clear());
    adc.ier().write(|w| w.awd1ie().enabled());
    NVIC::unpend(pac::Interrupt::ADC4);
    // SAFETY: ADC4, the watchdog window, and the static state are ready.
    unsafe { NVIC::unmask(pac::Interrupt::ADC4) };
    log::info!("USB: VBUS ADC sample {sample}, present={powered}");
    Driver::new(
        peripheral,
        out_buffer,
        Vbus {
            _sense_pin: gpiog,
            _data_pins: gpioa,
            adc,
        },
    )
}

fn wait_ready(stage: &str, mut ready: impl FnMut() -> bool) {
    for _ in 0..100 {
        if ready() {
            return;
        }
        cortex_m::asm::delay(160_000);
    }
    assert!(ready(), "USB: timed out waiting for {stage}");
}
