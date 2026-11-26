//! HAL for FMC peripheral used to access NAND Flash
//!

use core::cmp;
use core::marker::PhantomData;

use embedded_hal::delay::DelayNs;

use crate::fmc::{FmcBank, FmcRegisters};
use crate::FmcPeripheral;

use crate::ral::{fmc, modify_reg};

pub mod device;

/// FMC NAND Physical Interface Configuration
///
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NandConfiguration {
    /// Data path width in bits
    pub data_width: u8,
    /// Number of address bits used for the column address
    pub column_bits: u8,
}

/// FMC NAND Timing parameters
///
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NandTiming {
    /// nCE setup time tCS
    pub nce_setup_time: i32,
    /// Data setup time tDS
    pub data_setup_time: i32,
    /// ALE hold time
    pub ale_hold_time: i32,
    /// CLE hold time
    pub cle_hold_time: i32,
    /// ALE to nRE delay
    pub ale_to_nre_delay: i32,
    /// CLE to nRE delay
    pub cle_to_nre_delay: i32,
    /// nRE pulse width tRP
    pub nre_pulse_width_ns: i32,
    /// nWE pulse width tWP
    pub nwe_pulse_width_ns: i32,
    /// Read cycle time tRC
    pub read_cycle_time_ns: i32,
    /// Write cycle time tWC
    pub write_cycle_time_ns: i32,
    /// nWE high to busy tWB
    pub nwe_high_to_busy_ns: i32,
}

/// Respresents a model of NAND chip
pub trait NandChip {
    /// NAND controller configuration
    const CONFIG: NandConfiguration;
    /// Timing parameters
    const TIMING: NandTiming;
}

/// FMC Peripheral specialized as a NAND Controller. Not yet initialized.
#[allow(missing_debug_implementations)]
pub struct Nand<FMC, IC> {
    /// Parameters for the NAND IC
    _chip: PhantomData<IC>,
    /// FMC peripheral
    fmc: FMC,
    /// Register access
    regs: FmcRegisters,
}

/// Set of pins for a NAND
pub trait PinsNand {
    /// Number of data bus pins
    const N_DATA: usize;
}

impl<IC: NandChip, FMC: FmcPeripheral> Nand<FMC, IC> {
    /// New NAND instance
    ///
    /// `_pins` must be a set of pins connecting to an NAND on the FMC
    /// controller
    ///
    /// # Panics
    ///
    /// * Panics if there is a mismatch between the data lines in `PINS` and the
    /// NAND device
    pub fn new<PINS>(fmc: FMC, _pins: PINS, _chip: IC) -> Self
    where
        PINS: PinsNand,
    {
        assert!(
            PINS::N_DATA == IC::CONFIG.data_width as usize,
            "NAND Data Bus Width mismatch between IC and controller"
        );

        Nand {
            _chip: PhantomData,
            fmc,
            regs: FmcRegisters::new::<FMC>(),
        }
    }

    /// New NAND instance
    ///
    /// # Safety
    ///
    /// This method does not ensure that IO pins are configured
    /// correctly. Misconfiguration may result in a bus lockup or stall when
    /// attempting to initialise the NAND device.
    ///
    /// The pins are not checked against the requirements for the NAND
    /// chip. Using this method it is possible to initialise a NAND device
    /// without sufficient pins to access the whole memory
    ///
    pub unsafe fn new_unchecked(fmc: FMC, _chip: IC) -> Self {
        Nand {
            _chip: PhantomData,
            fmc,
            regs: FmcRegisters::new::<FMC>(),
        }
    }

    /// Initialise NAND instance. `delay` is used to wait 1µs after enabling the
    /// memory controller.
    ///
    /// Returns a [`NandDevice`](device::NandDevice) instance.
    ///
    /// # Panics
    ///
    /// * Panics if any setting in `IC::CONFIG` cannot be achieved
    /// * Panics if the FMC Kernel Clock is too fast to achieve the timing
    /// required by the NAND device
    pub fn init<D>(&mut self, delay: &mut D) -> device::NandDevice
    where
        D: DelayNs,
    {
        // calculate clock period, round down
        let fmc_source_ck_hz = self.fmc.source_clock_hz();
        let ker_clk_period_ns = 1_000_000_000u32 / fmc_source_ck_hz;

        // enable memory controller AHB register access
        self.fmc.enable();

        // device features and timing
        self.set_features_timings(IC::CONFIG, IC::TIMING, ker_clk_period_ns);

        // enable memory controller
        self.fmc.memory_controller_enable();
        delay.delay_us(1);

        // NOTE(unsafe): FMC controller has been initialized and enabled for
        // this bank
        unsafe {
            // Create device. NAND Flash is always on Bank 3
            let ptr = FmcBank::Bank3.ptr() as *mut u8;
            device::NandDevice::init(ptr, IC::CONFIG.column_bits as usize)
        }
    }

    /// Program memory device features and timings
    ///
    /// Timing calculations from AN4761 Section 4.2
    #[allow(non_snake_case)]
    fn set_features_timings(
        &mut self,
        config: NandConfiguration,
        timing: NandTiming,
        period_ns: u32,
    ) {
        let period_ns = period_ns as i32;
        let n_clock_periods = |time_ns: i32| {
            (time_ns + period_ns - 1) / period_ns // round up
        };
        let t_CS = timing.nce_setup_time;
        let t_DS = timing.data_setup_time;
        let t_ALH = timing.ale_hold_time;
        let t_CLH = timing.cle_hold_time;
        let t_AR = timing.ale_to_nre_delay;
        let t_CLR = timing.cle_to_nre_delay;
        let t_RP = timing.nre_pulse_width_ns;
        let t_WP = timing.nwe_pulse_width_ns;
        let t_RC = timing.read_cycle_time_ns;
        let t_WC = timing.write_cycle_time_ns;
        let t_WB = timing.nwe_high_to_busy_ns;

        // setup time before RE/WE assertion
        let setup_time = cmp::max(t_CS, cmp::max(t_AR, t_CLR));
        let set = cmp::max(n_clock_periods(setup_time - t_WP), 1) - 1;
        assert!(set < 255, "FMC ker clock too fast"); // 255 = reserved

        // RE/WE assertion time (minimum = 1)
        let wait = cmp::max(n_clock_periods(cmp::max(t_RP, t_WP)), 2) - 1;
        assert!(wait < 255, "FMC ker clock too fast"); // 255 = reserved

        // hold time after RE/WE deassertion (minimum = 1)
        let mut hold = cmp::max(n_clock_periods(cmp::max(t_ALH, t_CLH)), 1);
        // satisfy total cycle time
        let cycle_time = n_clock_periods(cmp::max(t_RC, t_WC));
        while wait + 1 + hold + set + 1 < cycle_time {
            hold += 1;
        }
        assert!(hold < 255, "FMC ker clock too fast"); // 255 = reserved

        // hold time to meet t_WB timing
        let atthold = cmp::max(n_clock_periods(t_WB), 2) - 1;
        let atthold = cmp::max(atthold, hold);
        assert!(atthold < 255, "FMC ker clock too fast"); // 255 = reserved

        // CS assertion to data setup
        let hiz = cmp::max(n_clock_periods(t_CS + t_WP - t_DS), 0);
        assert!(hiz < 255, "FMC ker clock too fast"); // 255 = reserved

        // ALE low to RE assert
        let ale_to_nre = n_clock_periods(t_AR);
        let tar = cmp::max(ale_to_nre - set - 2, 0);
        assert!(tar < 16, "FMC ker clock too fast");

        // CLE low to RE assert
        let clr_to_nre = n_clock_periods(t_CLR);
        let tclr = cmp::max(clr_to_nre - set - 2, 0);
        assert!(tclr < 16, "FMC ker clock too fast");

        let data_width = match config.data_width {
            8 => 0,
            16 => 1,
            _ => panic!("not possible"),
        };

        // PCR
        #[rustfmt::skip]
        modify_reg!(fmc, self.regs.global(), PCR,
                    TAR: tar as u32,
                    TCLR: tclr as u32,
                    ECCPS: 1,   // 0b1: 512 bytes
                    ECCEN: 0,   // 0b0: ECC computation disabled
                    PWID: data_width,
                    PTYP: 1,    // 0b1: NAND Flash
                    PWAITEN: 1  // 0b1: Wait feature enabled
        );

        // PMEM: Common memory space timing register
        #[rustfmt::skip]
        modify_reg!(fmc, self.regs.global(), PMEM,
                    MEMHIZ: hiz as u32,
                    MEMHOLD: hold as u32,
                    MEMWAIT: wait as u32,
                    MEMSET: set as u32);

        // PATT: Attribute memory space timing register
        #[rustfmt::skip]
        modify_reg!(fmc, self.regs.global(), PATT,
                    ATTHIZ: hiz as u32,
                    ATTHOLD: atthold as u32,
                    ATTWAIT: wait as u32,
                    ATTSET: set as u32);

        // Enable
        #[rustfmt::skip]
        modify_reg!(fmc, self.regs.global(), PCR,
                    PBKEN: 1);
    }
}
