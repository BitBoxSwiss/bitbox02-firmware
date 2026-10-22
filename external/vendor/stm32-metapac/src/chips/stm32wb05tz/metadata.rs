include!("../metadata_0853.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32WB05TZ",
    family: "STM32WB0",
    line: "STM32WBx5",
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
            size: 196608,
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
            size: 12264,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM1",
            kind: MemoryRegionKind::Ram,
            address: 0x20003000,
            size: 12288,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
