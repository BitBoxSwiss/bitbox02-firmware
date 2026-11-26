include!("../metadata_0854.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32WB06CC",
    family: "STM32WB0",
    line: "STM32WBx6",
    memory: &[&[
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x10001800,
            size: 1024,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x10040000,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM0",
            kind: MemoryRegionKind::Ram,
            address: 0x20000018,
            size: 16360,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM1",
            kind: MemoryRegionKind::Ram,
            address: 0x20004000,
            size: 16384,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
