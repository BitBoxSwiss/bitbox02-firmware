include!("../metadata_0356.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32G473QC",
    family: "STM32G4",
    line: "STM32G4x3",
    memory: &[
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 262144,
                settings: Some(FlashSettings {
                    erase_size: 4096,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "CCMRAM_ICODE",
                kind: MemoryRegionKind::Ram,
                address: 0x10000000,
                size: 32768,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM1",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 81920,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM2",
                kind: MemoryRegionKind::Ram,
                address: 0x20014000,
                size: 16384,
                settings: None,
            },
            MemoryRegion {
                name: "CCMRAM_DCODE",
                kind: MemoryRegionKind::Ram,
                address: 0x20018000,
                size: 32768,
                settings: None,
            },
        ],
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 131072,
                settings: Some(FlashSettings {
                    erase_size: 2048,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "BANK_2",
                kind: MemoryRegionKind::Flash,
                address: 0x8040000,
                size: 131072,
                settings: Some(FlashSettings {
                    erase_size: 2048,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "CCMRAM_ICODE",
                kind: MemoryRegionKind::Ram,
                address: 0x10000000,
                size: 32768,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM1",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 81920,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM2",
                kind: MemoryRegionKind::Ram,
                address: 0x20014000,
                size: 16384,
                settings: None,
            },
            MemoryRegion {
                name: "CCMRAM_DCODE",
                kind: MemoryRegionKind::Ram,
                address: 0x20018000,
                size: 32768,
                settings: None,
            },
        ],
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
