// SPDX-License-Identifier: Apache-2.0

//! STM32U5A9 USB OTG HS adapter for Embassy, using the integrated UTMI PHY at full speed.
//!
//! The board configures the USB pins and supplies the PHY with a 16 MHz HSE clock.
//! AHB must run at >= 32 MHz. Startup and the system clocks remain owned by this platform.

use bitbox_mcu_stm32u5::pac::{self, interrupt};
use core::future::{Future, poll_fn};
use core::marker::PhantomData;
use core::pin::pin;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::{Context, Poll};
use cortex_m::peripheral::NVIC;
use embassy_usb_driver::{
    EndpointAddress, EndpointAllocError, EndpointError, EndpointType, Event, Unsupported,
};
use embassy_usb_synopsys_otg::{self as otg, otg_v1::Otg, otg_v1::vals::Dspd};

static STATE: otg::StateStorage<9> =
    otg::StateStorage::new(embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex::new());
static SETUP_GENERATION: AtomicU32 = AtomicU32::new(0);
static CONTROL_WAKER: embassy_sync::waitqueue::AtomicWaker =
    embassy_sync::waitqueue::AtomicWaker::new();

/// Board-specific VBUS sensing. Register the waker before sampling the pin.
pub trait VbusDetect {
    // USB has one board-level VBUS signal. Both the bus task and control transfers
    // must be able to observe it; a stalled control transfer must not hide unplug.
    fn poll_vbus(cx: &mut Context<'_>) -> bool;
}

fn regs() -> Otg {
    // SAFETY: this is the STM32U5A9 OTG_HS register block. The driver owns the PAC
    // peripheral; the interrupt handler only accesses it through Embassy's shared state.
    unsafe { Otg::from_ptr(pac::OTG_HS::PTR as *mut ()) }
}

#[interrupt]
fn OTG_HS() {
    if regs().doepint(0).read().stup() {
        SETUP_GENERATION.fetch_add(1, Ordering::Release);
    }
    // SAFETY: this interrupt is enabled only after the static driver state and
    // receive buffers have been installed. It is masked again when the bus is dropped.
    unsafe { otg::on_interrupt(regs(), &STATE.as_state()) };
    CONTROL_WAKER.wake();
}

pub struct Driver<V> {
    inner: otg::Driver<'static>,
    peripheral: pac::OTG_HS,
    vbus: V,
}

impl<V: VbusDetect> Driver<V> {
    /// Claim the USB controller. The static receive buffer also remains valid if
    /// endpoint handles outlive the USB device or a future is cancelled.
    pub fn new(peripheral: pac::OTG_HS, out_buffer: &'static mut [u8], vbus: V) -> Self {
        NVIC::mask(pac::Interrupt::OTG_HS);
        let instance = otg::OtgInstance {
            regs: regs(),
            state: STATE.as_state(),
            fifo_depth_words: 1024,
            tx_fifo_count: 9,
            extra_rx_fifo_words: 30,
            phy_type: otg::PhyType::InternalHighSpeed,
            calculate_trdt_fn: |speed| match speed {
                Dspd::HIGH_SPEED => 9,
                _ => 6, // Full speed, AHB >= 32 MHz (RM0456, GUSBCFG.TRDT).
            },
        };
        let mut config = otg::Config::default();
        config.vbus_valid_override = true;
        Self {
            inner: otg::Driver::new(out_buffer, instance, config),
            peripheral,
            vbus,
        }
    }
}

impl<V: VbusDetect + 'static> embassy_usb_driver::Driver<'static> for Driver<V> {
    type EndpointOut = otg::Endpoint<'static, otg::Out>;
    type EndpointIn = otg::Endpoint<'static, otg::In>;
    type ControlPipe = ControlPipe<V>;
    type Bus = Bus<V>;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.inner
            .alloc_endpoint_in(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.inner
            .alloc_endpoint_out(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let (inner, control) = self.inner.start(control_max_packet_size);
        (
            Bus {
                inner,
                _peripheral: self.peripheral,
                _vbus: self.vbus,
                powered: false,
                initialized: false,
                core_initialized: false,
            },
            ControlPipe {
                inner: control,
                generation: 0,
                setup_count: 0,
                _vbus: PhantomData,
            },
        )
    }
}

pub struct Bus<V> {
    inner: otg::Bus<'static>,
    _peripheral: pac::OTG_HS,
    _vbus: V,
    powered: bool,
    initialized: bool,
    core_initialized: bool,
}

/// Cancel control data/status stages when the host abandons the transfer. Embassy
/// services control requests inside its bus task, so waiting for that task to
/// process an unplug/reset would otherwise leave both tasks waiting on each other.
pub struct ControlPipe<V> {
    inner: otg::ControlPipe<'static>,
    generation: u32,
    setup_count: u8,
    _vbus: PhantomData<V>,
}

async fn control_transfer<V: VbusDetect, T>(
    generation: u32,
    setup_count: u8,
    future: impl Future<Output = Result<T, EndpointError>>,
) -> Result<T, EndpointError> {
    let mut future = pin!(future);
    poll_fn(|cx| {
        CONTROL_WAKER.register(cx.waker());
        let powered = V::poll_vbus(cx);
        let interrupts = regs().gintsts().read();
        if !powered
            || interrupts.usbrst()
            || interrupts.enumdne()
            || generation != SETUP_GENERATION.load(Ordering::Acquire)
            // SETUP can arrive between our ISR's generation check and Embassy's
            // handler. Also detect the hardware's decremented STUPCNT value.
            || regs().doeptsiz(0).read().rxdpid_stupcnt() != setup_count
        {
            return Poll::Ready(Err(EndpointError::Disabled));
        }
        future.as_mut().poll(cx)
    })
    .await
}

impl<V: VbusDetect> embassy_usb_driver::ControlPipe for ControlPipe<V> {
    fn max_packet_size(&self) -> usize {
        self.inner.max_packet_size()
    }

    async fn setup(&mut self) -> [u8; 8] {
        let mut setup = pin!(self.inner.setup());
        poll_fn(|cx| {
            // Read the request and its generation atomically with respect to the
            // ISR, including a new SETUP arriving immediately after this one.
            critical_section::with(|_| {
                let result = setup.as_mut().poll(cx);
                if result.is_ready() {
                    self.generation = SETUP_GENERATION.load(Ordering::Acquire);
                    self.setup_count = regs().doeptsiz(0).read().rxdpid_stupcnt();
                }
                result
            })
        })
        .await
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        first: bool,
        last: bool,
    ) -> Result<usize, EndpointError> {
        control_transfer::<V, _>(
            self.generation,
            self.setup_count,
            self.inner.data_out(buf, first, last),
        )
        .await
    }

    async fn data_in(&mut self, data: &[u8], first: bool, last: bool) -> Result<(), EndpointError> {
        control_transfer::<V, _>(
            self.generation,
            self.setup_count,
            self.inner.data_in(data, first, last),
        )
        .await
    }

    async fn accept(&mut self) {
        let _ = control_transfer::<V, _>(self.generation, self.setup_count, async {
            self.inner.accept().await;
            Ok(())
        })
        .await;
    }

    async fn reject(&mut self) {
        self.inner.reject().await;
    }

    async fn accept_set_address(&mut self, addr: u8) {
        log::debug!("USB: setting device address to {addr}");
        let _ = control_transfer::<V, _>(self.generation, self.setup_count, async {
            self.inner.accept_set_address(addr).await;
            Ok(())
        })
        .await;
    }
}

impl<V> Bus<V> {
    fn init(&mut self) {
        log::info!("USB: enabling power supplies");
        // SAFETY: shared clock/power registers are accessed in a critical section;
        // only USB-specific fields are changed. GPIO and the HSE clock are set up by the board.
        let (rcc, pwr, syscfg) = unsafe { (&*pac::RCC::PTR, &*pac::PWR::PTR, &*pac::SYSCFG::PTR) };
        assert!(rcc.cr().read().hserdy().is_ready());
        critical_section::with(|_| {
            rcc.ahb3enr().modify(|_, w| w.pwren().set_bit());
            rcc.apb3enr().modify(|_, w| w.syscfgen().set_bit());
            let _ = rcc.apb3enr().read();
            pwr.svmcr()
                .modify(|_, w| w.usv().set_bit().uvmen().set_bit());
        });
        wait_ready("VDDUSB", || pwr.svmsr().read().vddusbrdy().bit_is_set());
        critical_section::with(|_| {
            pwr.vosr()
                .modify(|_, w| w.usbpwren().set_bit().usbboosten().set_bit());
        });
        wait_ready("USB booster", || {
            pwr.vosr().read().usbboostrdy().bit_is_set()
        });
        log::info!("USB: enabling PHY and controller clocks");
        critical_section::with(|_| {
            rcc.ccipr2().modify(|_, w| w.otghssel().hse());
            // RM0456: CLKSEL=0b0011 selects the board's 16 MHz PHY reference clock.
            syscfg
                .otghsphycr()
                .modify(|_, w| unsafe { w.clksel().bits(3).pdctrl().clear_bit() });
            rcc.ahb2enr1()
                .modify(|_, w| w.otgen().set_bit().otghsphyen().set_bit());
            let _ = rcc.ahb2enr1().read();
            rcc.ahb2rstr1().modify(|_, w| w.otgrst().set_bit());
            rcc.ahb2rstr1().modify(|_, w| w.otgrst().clear_bit());
            syscfg.otghsphycr().modify(|_, w| w.en().set_bit());
        });
        wait_ready("AHB idle", || regs().grstctl().read().ahbidl());
        log::info!(
            "USB: selecting device mode (core {:#010x})",
            regs().cid().read().0
        );
        self.inner.configure_as_device();
        log::info!("USB: resetting core");
        self.inner.core_soft_reset();
        match regs().cid().read().0 {
            0x0000_5000 | 0x0000_6100 => self.inner.config_v5(),
            id => panic!("unsupported STM32U5 USB core: {id:#x}"),
        }
        regs().gccfg_v3().modify(|w| {
            w.set_vbvaloven(true);
            w.set_vbvaloval(false);
        });
        self.init_core();
        NVIC::unpend(pac::Interrupt::OTG_HS);
        // SAFETY: clocks, PHY, static state, and buffers are now ready for the ISR.
        unsafe { NVIC::unmask(pac::Interrupt::OTG_HS) };
        self.initialized = true;
        log::info!("USB: controller ready, waiting for VBUS");
    }

    fn init_core(&mut self) {
        if self.core_initialized {
            return;
        }
        regs().gccfg_v3().modify(|w| w.set_vbvaloval(false));
        self.inner.init_device();
        regs().dctl().modify(|w| w.set_sdis(true));
        // Use the HS controller's UTMI PHY at 12 Mbit/s. The HWW stack describes
        // a full-speed-only device; it does not supply other-speed descriptors.
        regs()
            .dcfg()
            .modify(|w| w.set_dspd(Dspd::FULL_SPEED_EXTERNAL));
        self.core_initialized = true;
    }
}

fn wait_ready(stage: &str, mut ready: impl FnMut() -> bool) {
    for _ in 0..100 {
        if ready() {
            return;
        }
        // At the board's 160 MHz system clock this allows at least 100 ms overall.
        cortex_m::asm::delay(160_000);
    }
    assert!(ready(), "USB: timed out waiting for {stage}");
}

impl<V: VbusDetect> embassy_usb_driver::Bus for Bus<V> {
    async fn poll(&mut self) -> Event {
        if !self.initialized {
            self.init();
        }
        self.init_core();
        let mut first_poll = true;
        let event = loop {
            let mut event = pin!(self.inner.poll());
            let event = poll_fn(|cx| {
                let powered = V::poll_vbus(cx);
                if first_poll || powered != self.powered {
                    log::debug!("USB: VBUS {}", if powered { "present" } else { "absent" });
                    first_poll = false;
                }
                if powered != self.powered {
                    self.powered = powered;
                    // Feed the board's VBUS state into the core's comparator override.
                    regs().gccfg_v3().modify(|w| w.set_vbvaloval(powered));
                    // Report power directly from the board's detector. Waiting for SRQINT
                    // here can leave the core soft-disconnected forever: the USB stack
                    // only calls enable() after receiving PowerDetected.
                    // PowerRemoved makes the stack call disable(), which also wakes
                    // endpoint transfers pending when the cable was removed.
                    return Poll::Ready(Some(if powered {
                        Event::PowerDetected
                    } else {
                        Event::PowerRemoved
                    }));
                }
                event.as_mut().poll(cx).map(|event| match event {
                    // Let the inner driver acknowledge session interrupts, but use
                    // the board's ADC as the sole source of power events.
                    Event::PowerDetected | Event::PowerRemoved => None,
                    event => Some(event),
                })
            })
            .await;
            if let Some(event) = event {
                break event;
            }
        };
        log::debug!("USB: bus event {event:?}");
        event
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        self.inner.endpoint_set_stalled(ep_addr, stalled);
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        self.inner.endpoint_is_stalled(ep_addr)
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        self.inner.endpoint_set_enabled(ep_addr, enabled);
    }

    async fn enable(&mut self) {
        self.init_core();
        if self.initialized && self.powered {
            regs().dctl().modify(|w| w.set_sdis(false));
            log::info!("USB: attached to host");
        }
    }

    async fn disable(&mut self) {
        if self.initialized {
            regs().dctl().modify(|w| w.set_sdis(true));
            self.inner.disable().await;
            self.core_initialized = false;
            self.powered = false;
        }
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}

impl<V> Drop for Bus<V> {
    fn drop(&mut self) {
        NVIC::mask(pac::Interrupt::OTG_HS);
        if self.initialized {
            regs().dctl().modify(|w| w.set_sdis(true));
            regs().gahbcfg().modify(|w| w.set_gint(false));
            self.inner.deinit_device();
        }
    }
}
