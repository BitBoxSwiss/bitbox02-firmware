include!("../metadata_0636.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32L152ZD",
    family: "STM32L1",
    line: "STM32L151/152",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 196608,
            settings: Some(FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8030000,
            size: 196608,
            settings: Some(FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "EEPROM_BANK_1",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8080000,
            size: 6144,
            settings: None,
        },
        MemoryRegion {
            name: "EEPROM_BANK_2",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8081800,
            size: 6144,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 49152,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
