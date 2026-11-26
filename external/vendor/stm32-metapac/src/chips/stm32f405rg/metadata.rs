include!("../metadata_0176.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F405RG",
    family: "STM32F4",
    line: "STM32F405/415",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 16384,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8010000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 65536,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 0x8020000,
            size: 917504,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "CCMRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x10000000,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x1fff7800,
            size: 528,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 114688,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x2001c000,
            size: 16384,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
