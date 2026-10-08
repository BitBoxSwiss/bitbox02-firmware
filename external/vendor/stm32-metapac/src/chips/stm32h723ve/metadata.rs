include!("../metadata_0421.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32H723VE",
    family: "STM32H7",
    line: "STM32H723/733",
    memory: &[&[
        MemoryRegion {
            name: "ITCM",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "DTCM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 131072,
            settings: None,
        },
        MemoryRegion {
            name: "AXISRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x24000000,
            size: 327680,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x30000000,
            size: 16384,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM3",
            kind: MemoryRegionKind::Ram,
            address: 0x30004000,
            size: 16384,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM4",
            kind: MemoryRegionKind::Ram,
            address: 0x38000000,
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
