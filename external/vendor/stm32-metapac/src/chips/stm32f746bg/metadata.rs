include!("../metadata_0258.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F746BG",
    family: "STM32F7",
    line: "STM32F7x6",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 32768,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8020000,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 0x8040000,
            size: 786432,
            settings: Some(FlashSettings {
                erase_size: 262144,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x8fff000,
            size: 2048,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "DTCM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20010000,
            size: 262144,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
