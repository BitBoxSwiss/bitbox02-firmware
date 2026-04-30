// SPDX-License-Identifier: Apache-2.0

use crate::ffi;
use bitbox_mcu_stm32u5::pac;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

const TIM6_CLOCK_ENABLE: u32 = 1 << 4;
const TARGET_TIMER_HZ: u32 = 100_000;
const MAX_ARR: u32 = 0x000f_ffff;

static TIM6_INTERRUPT_COUNT: AtomicU32 = AtomicU32::new(0);

unsafe extern "C" {
    static SystemCoreClock: u32;
}

pub struct Tim6 {
    one_shot_interrupts_before: u32,
    tick_remainder: u64,
}

impl Tim6 {
    pub fn new() -> Self {
        enable_tim6_clock();

        let tim6 = tim6();
        unsafe {
            tim6.cr1().write(|w| w.bits(0));
            tim6.dier().write(|w| w.bits(0));
            tim6.sr().write(|w| w.bits(0));
        }

        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM6);
        }

        Self {
            one_shot_interrupts_before: 0,
            tick_remainder: 0,
        }
    }

    pub fn start_timeout(&mut self, duration: Duration) {
        self.one_shot_interrupts_before = TIM6_INTERRUPT_COUNT.load(Ordering::Relaxed);
        self.start_one_shot(duration);
    }

    pub fn wait_event_or_timeout(&mut self) {
        unsafe {
            ffi::HAL_SuspendTick();
        }
        clear_stale_wfe_event();
        let wait_started_timer_ticks = self.elapsed_timer_ticks();
        if wait_started_timer_ticks < self.one_shot_timer_ticks() {
            cortex_m::asm::wfe();
        }
        let elapsed_timer_ticks = self.elapsed_timer_ticks();
        self.compensate_hal_tick_from_timer_ticks(
            elapsed_timer_ticks.saturating_sub(wait_started_timer_ticks),
        );
        unsafe {
            ffi::HAL_ResumeTick();
        }
        self.stop();
    }

    fn compensate_hal_tick_from_timer_ticks(&mut self, elapsed_timer_ticks: u32) -> u32 {
        let timer_hz = tim6_tick_hz(tim6_prescaler());
        assert!(timer_hz >= 1_000);

        let total = u64::from(elapsed_timer_ticks) * 1_000 + self.tick_remainder;
        let elapsed_ms = total / u64::from(timer_hz);
        self.tick_remainder = total % u64::from(timer_hz);

        if elapsed_ms > 0 {
            unsafe {
                ffi::uwTick = ffi::uwTick.wrapping_add(elapsed_ms as u32);
            }
        }

        elapsed_ms as u32
    }

    fn elapsed_timer_ticks(&self) -> u32 {
        let tim6 = tim6();
        let tim6_interrupts = TIM6_INTERRUPT_COUNT
            .load(Ordering::Relaxed)
            .wrapping_sub(self.one_shot_interrupts_before);
        let sr = tim6.sr().read().bits();
        if tim6_interrupts > 0 || sr & 1 != 0 {
            self.one_shot_timer_ticks()
        } else {
            tim6.cnt().read().bits()
        }
    }

    fn one_shot_timer_ticks(&self) -> u32 {
        tim6().arr().read().bits() + 1
    }

    fn start_one_shot(&mut self, duration: Duration) {
        let tim6 = tim6();
        let prescaler = tim6_prescaler();
        let ticks = duration_to_ticks(duration, tim6_tick_hz(prescaler));

        unsafe {
            tim6.cr1().write(|w| w.bits(0));
            tim6.dier().write(|w| w.bits(0));
            tim6.sr().write(|w| w.bits(0));
            tim6.cnt().write(|w| w.bits(0));

            // Set Autoreload
            tim6.arr().write(|w| w.bits(ticks - 1));
            // Set Prescaler
            tim6.psc().write(|w| w.bits(u32::from(prescaler)));

            // Set URS to disable interrupt on UG set
            tim6.cr1().write(|w| w.bits(1 << 2));
            // Set UG to generate interrupt on autoreload
            tim6.egr().write(|w| w.ug().set_bit());
            // Reset URS
            tim6.cr1().write(|w| w.bits(0));

            // Set UIE
            // Ensure interrupt flag is reset so that setting DIER doesn't immediately trigger
            // interrupt
            tim6.sr().write(|w| w.bits(0));
            tim6.dier().write(|w| w.bits(1));

            // Start timer
            tim6.cr1().write(|w| w.cen().set_bit());
        }
    }

    fn stop(&mut self) {
        let tim6 = tim6();
        unsafe {
            tim6.cr1().write(|w| w.bits(0));
            tim6.dier().write(|w| w.bits(0));
            tim6.sr().write(|w| w.bits(0));
        }
        cortex_m::peripheral::NVIC::unpend(pac::Interrupt::TIM6);
    }
}

impl Default for Tim6 {
    fn default() -> Self {
        Self::new()
    }
}

pub fn tim6_interrupt_handler() {
    TIM6_INTERRUPT_COUNT.fetch_add(1, Ordering::Relaxed);
    unsafe {
        tim6().sr().write(|w| w.bits(0));
    }
}

fn tim6() -> &'static pac::tim6::RegisterBlock {
    unsafe { &*pac::TIM6::ptr() }
}

fn clear_stale_wfe_event() {
    // WFE consumes a sticky event register before it actually waits. Clear it
    // before starting TIM6, otherwise a stale event looks like a zero-duration
    // sleep and can skew HAL tick compensation.
    cortex_m::asm::sev();
    cortex_m::asm::wfe();
}

fn enable_tim6_clock() {
    let rcc = unsafe { &*pac::RCC::PTR };

    unsafe {
        rcc.apb1enr1()
            .modify(|r, w| w.bits(r.bits() | TIM6_CLOCK_ENABLE));
        rcc.apb1smenr1()
            .modify(|r, w| w.bits(r.bits() | TIM6_CLOCK_ENABLE));
        rcc.apb1rstr1()
            .modify(|r, w| w.bits(r.bits() | TIM6_CLOCK_ENABLE));
        rcc.apb1rstr1()
            .modify(|r, w| w.bits(r.bits() & !TIM6_CLOCK_ENABLE));
    }
}

fn tim6_prescaler() -> u16 {
    let prescaler = (tim6_clock_hz() / TARGET_TIMER_HZ).saturating_sub(1);
    assert!(prescaler <= u32::from(u16::MAX));
    prescaler as u16
}

fn tim6_tick_hz(prescaler: u16) -> u32 {
    tim6_clock_hz() / (u32::from(prescaler) + 1)
}

fn tim6_clock_hz() -> u32 {
    let hclk = unsafe { SystemCoreClock };
    let apb1_div = apb1_prescaler();
    let pclk1 = hclk / apb1_div;

    if apb1_div == 1 { pclk1 } else { pclk1 * 2 }
}

fn apb1_prescaler() -> u32 {
    let rcc = unsafe { &*pac::RCC::PTR };
    match rcc.cfgr2().read().ppre1().bits() {
        4 => 2,
        5 => 4,
        6 => 8,
        7 => 16,
        _ => 1,
    }
}

fn duration_to_ticks(duration: Duration, timer_hz: u32) -> u32 {
    let ticks = duration
        .as_micros()
        .saturating_mul(u128::from(timer_hz))
        .saturating_add(999_999)
        / 1_000_000;

    ticks.clamp(1, u128::from(MAX_ARR)) as u32
}
