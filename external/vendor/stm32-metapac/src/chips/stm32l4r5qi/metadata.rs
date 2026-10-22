include!("../metadata_0721.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32L4R5QI",
    family: "STM32L4+",
    line: "STM32L4R5/S5",
    memory: &[
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 2097152,
                settings: Some(FlashSettings {
                    erase_size: 8192,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "SRAM",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 655360,
                settings: None,
            },
        ],
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 1048576,
                settings: Some(FlashSettings {
                    erase_size: 4096,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "BANK_2",
                kind: MemoryRegionKind::Flash,
                address: 0x8100000,
                size: 1048576,
                settings: Some(FlashSettings {
                    erase_size: 4096,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "SRAM",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 655360,
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
