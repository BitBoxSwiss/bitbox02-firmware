//! Tests SDRAM pin constraints apply correctly

mod dummy_pins;
use dummy_pins::*;

use stm32_fmc::*;

/// Dummy FmcPeripheral implementation for testing
struct DummyFMC;
unsafe impl FmcPeripheral for DummyFMC {
    const REGISTERS: *const () = 0 as *const ();
    fn enable(&mut self) {}
    fn source_clock_hz(&self) -> u32 {
        100_000_000
    }
}

macro_rules! fmc_pin_set {
    ($($p:ident),*) => {
        paste::item! {
            (
                $(
                    [< PinThats $p:upper>] {}
                ),*
            )
        }
    }
}

#[test]
/// SDRAM with 12 address pins, 4 banks
fn sdram_pins_12a_4b() {
    let fmc = DummyFMC {};
    let pins = fmc_pin_set!(
        // 12 address bits
        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,
        // 4 internal banks --------------------------------------
        BA0, BA1,
        // 32 bit data -------------------------------------------
        D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
        D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29,
        D30, D31,
        // NBL0-3 ------------------------------------------------
        NBL0, NBL1, NBL2, NBL3,
        // SDRAM Bank 0 ------------------------------------------
        SDCKE0, SDCLK, SDNCAS, SDNE0, SDNRAS, SDNWE
    );
    let chip = devices::is42s32800g_6::Is42s32800g {};

    // Check we can create a SDRAM
    Sdram::new(fmc, pins, chip);
}

#[test]
#[should_panic]
/// SDRAM with 12 address pins, 4 banks
fn sdram_pins_12a_4b_not_enough_adress_pins() {
    let fmc = DummyFMC {};
    let pins = fmc_pin_set!(
        // 11 bit address (!)
        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,
        // 4 internal banks --------------------------------------
        BA0, BA1,
        // 32 bit data -------------------------------------------
        D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
        D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29,
        D30, D31,
        // NBL0-3 ------------------------------------------------
        NBL0, NBL1, NBL2, NBL3,
        // SDRAM Bank 0 ------------------------------------------
        SDCKE0, SDCLK, SDNCAS, SDNE0, SDNRAS, SDNWE
    );
    let chip = devices::is42s32800g_6::Is42s32800g {};

    // Check we can create a SDRAM
    Sdram::new(fmc, pins, chip);
}

#[test]
#[should_panic]
/// SDRAM with 12 address pins, 4 banks
fn sdram_pins_12a_4b_not_enough_bank_pins() {
    let fmc = DummyFMC {};
    let pins = fmc_pin_set!(
        // 12 address bits
        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,
        // 2 internal banks (!) -----------------------------------
        BA0,
        // 32 bit data --------------------------------------------
        D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
        D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29,
        D30, D31,
        // NBL0-3 -------------------------------------------------
        NBL0, NBL1, NBL2, NBL3,
        // SDRAM Bank 0 -------------------------------------------
        SDCKE0, SDCLK, SDNCAS, SDNE0, SDNRAS, SDNWE
    );
    let chip = devices::is42s32800g_6::Is42s32800g {};

    // Check we can create a SDRAM
    Sdram::new(fmc, pins, chip);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DummyChip {}

const BURST_LENGTH_1: u16 = 0x0000;
const BURST_TYPE_SEQUENTIAL: u16 = 0x0000;
const CAS_LATENCY_3: u16 = 0x0030;
const OPERATING_MODE_STANDARD: u16 = 0x0000;
const WRITEBURST_MODE_SINGLE: u16 = 0x0200;

impl SdramChip for DummyChip {
    const MODE_REGISTER: u16 = BURST_LENGTH_1
        | BURST_TYPE_SEQUENTIAL
        | CAS_LATENCY_3
        | OPERATING_MODE_STANDARD
        | WRITEBURST_MODE_SINGLE;

    const CONFIG: stm32_fmc::SdramConfiguration = SdramConfiguration {
        column_bits: 9,
        row_bits: 12,
        memory_data_width: 32, // 32-bit
        internal_banks: 4,     // 4 internal banks
        cas_latency: 3,        // CAS latency = 3
        write_protection: false,
        read_burst: true,
        read_pipe_delay_cycles: 0,
    };

    const TIMING: stm32_fmc::SdramTiming = SdramTiming {
        startup_delay_ns: 100_000,    // 100 µs
        max_sd_clock_hz: 100_000_000, // 100 MHz
        refresh_period_ns: 15_625,    // 64ms / (4096 rows) = 15625ns
        mode_register_to_active: 2,   // tMRD = 2 cycles
        exit_self_refresh: 7,         // tXSR = 70ns
        active_to_precharge: 4,       // tRAS = 42ns
        row_cycle: 7,                 // tRC = 70ns
        row_precharge: 2,             // tRP = 18ns
        row_to_column: 2,             // tRCD = 18ns
    };
}

#[test]
/// Test that we can implement the SdramChip trait
fn sdram_chip_impl() {
    let fmc = DummyFMC {};
    let pins = fmc_pin_set!(
        // 12 address bits
        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,
        // 4 internal banks --------------------------------------
        BA0, BA1,
        // 32 bit data -------------------------------------------
        D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
        D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29,
        D30, D31,
        // NBL0-3 ------------------------------------------------
        NBL0, NBL1, NBL2, NBL3,
        // SDRAM Bank 0 ------------------------------------------
        SDCKE0, SDCLK, SDNCAS, SDNE0, SDNRAS, SDNWE
    );
    let chip = DummyChip {};

    // Check we can create a SDRAM
    Sdram::new(fmc, pins, chip);
}
