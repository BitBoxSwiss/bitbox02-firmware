include!("../metadata_0857.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32WB10CC",
    family: "STM32WB",
    line: "STM32WBx0 Value Line",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 327680,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "SRAM2A_ICODE",
            kind: MemoryRegionKind::Ram,
            address: 0x10000000,
            size: 32768,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2B_ICODE",
            kind: MemoryRegionKind::Ram,
            address: 0x10008000,
            size: 4096,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 12288,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2A",
            kind: MemoryRegionKind::Ram,
            address: 0x20030000,
            size: 32768,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2B",
            kind: MemoryRegionKind::Ram,
            address: 0x20038000,
            size: 4096,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
