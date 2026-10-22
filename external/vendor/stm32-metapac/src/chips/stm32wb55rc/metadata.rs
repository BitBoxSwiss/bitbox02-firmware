include!("../metadata_0863.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32WB55RC",
    family: "STM32WB",
    line: "STM32WBx5",
    memory: &[&[
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
            size: 32768,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 65536,
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
            size: 32768,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
