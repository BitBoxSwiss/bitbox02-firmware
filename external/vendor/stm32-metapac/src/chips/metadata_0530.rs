
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "h7rs",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INP18",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INP13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "INP6",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(8),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x40022300,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "h5",
            block: "ADC_COMMON",
            ir: &adccommon::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x40022100,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "h7rs",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INP18",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INP13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "INP6",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(9),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADF1",
        address: 0x4002f000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "ADFSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADFRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CCK0",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CCK1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SDI0",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(97),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLT0",
            interrupt: "ADF1_FLT0",
        }],
        afio: None,
    },
    Peripheral {
        name: "CEC",
        address: 0x40006c00,
        registers: Some(PeripheralRegisters {
            kind: "cec",
            version: "v2",
            block: "CEC",
            ir: &cec::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "CECSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "CECEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "CECRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CEC",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CEC",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CEC",
        }],
        afio: None,
    },
    Peripheral {
        name: "CORDIC",
        address: 0x48004400,
        registers: Some(PeripheralRegisters {
            kind: "cordic",
            version: "v1",
            block: "CORDIC",
            ir: &cordic::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "CORDICEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "CORDICRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(88),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CORDIC",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x58024c00,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v3",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "CRCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRS",
        address: 0x40008400,
        registers: Some(PeripheralRegisters {
            kind: "crs",
            version: "v1",
            block: "CRS",
            ir: &crs::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "CRSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PB3",
            signal: "SYNC",
            af: Some(10),
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRYP",
        address: 0x48020800,
        registers: Some(PeripheralRegisters {
            kind: "cryp",
            version: "v4",
            block: "CRYP",
            ir: &cryp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "CRYPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "CRYPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(3),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CRYP",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x5c001000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "h7",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DCMIPP",
        address: 0x50002000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "DCMIPPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "DCMIPPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "D0",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "HSYNC",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "VSYNC",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D14",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D15",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D4",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D6",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D7",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D9",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "HSYNC",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "PIXCLK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D15",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "D14",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "D9",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "D8",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D15",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "PIXCLK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "HSYNC",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "VSYNC",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMIPP",
        }],
        afio: None,
    },
    Peripheral {
        name: "DLYB_SDMMC1",
        address: 0x52008000,
        registers: Some(PeripheralRegisters {
            kind: "dlyb",
            version: "v1",
            block: "DLYB",
            ir: &dlyb::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DLYB_SDMMC2",
        address: 0x48002800,
        registers: Some(PeripheralRegisters {
            kind: "dlyb",
            version: "v1",
            block: "DLYB",
            ir: &dlyb::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA2D",
        address: 0x52001000,
        registers: Some(PeripheralRegisters {
            kind: "dma2d",
            version: "v2",
            block: "DMA2D",
            ir: &dma2d::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "DMA2DEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "DMA2DRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DMA2D",
        }],
        afio: None,
    },
    Peripheral {
        name: "DTS",
        address: 0x58006800,
        registers: Some(PeripheralRegisters {
            kind: "dts",
            version: "v1",
            block: "DTS",
            ir: &dts::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "TMPSENSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "TMPSENSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DTS",
        }],
        afio: None,
    },
    Peripheral {
        name: "ETH",
        address: 0x40028000,
        registers: Some(PeripheralRegisters {
            kind: "eth",
            version: "v2",
            block: "ETH",
            ir: &eth::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ETHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ETHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "COL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CRS_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX_ER",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PPS_OUT",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "PHY_INTN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CLK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "REF_CLK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "RX_CLK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "TXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "RXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "CRS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "TX_ER",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "RXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "PPS_OUT",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "PPS_OUT",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ETH",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "EXTI",
        address: 0x58000000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "h7",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI11",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI12",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI13",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI14",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI6",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI7",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI8",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FDCAN1",
        address: 0x4000a000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v1",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "FDCANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "FDCAN1_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "FDCAN1_IT1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FDCAN2",
        address: 0x4000a400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v1",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "FDCANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "FDCAN2_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "FDCAN2_IT1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM1",
        address: 0x4000ac00,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM2",
        address: 0x4000af50,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x52002000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "h7rs",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "FMC",
        address: 0x52004000,
        registers: Some(PeripheralRegisters {
            kind: "fmc",
            version: "v3x1",
            block: "FMC",
            ir: &fmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "FMCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "FMCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "FMCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "DA7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DA6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "DA5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NOE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DA3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "NOE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "DA8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "NE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "A20",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "NWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "D14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DA14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDCKE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "DA9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "NBL1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "A0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "A16",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "A1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "A17",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "A18",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "A2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "A19",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "A3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "A21",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "A5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "A22",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "A6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "A23",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "A7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D16",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "NE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "INT",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D18",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "NWAIT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NCE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NE2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "INT",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "DA8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "NBL0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "SDCLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "A15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "BA1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SDNRAS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SDNCAS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "SDNE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SDCKE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "DA14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "A20",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "A4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "A12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "A14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "BA0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "A8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "A9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "A22",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "A23",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "D19",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D20",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "A24",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "A25",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "A10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "A11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "A13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "A16",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CLE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "A17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "ALE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "A18",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "A19",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "A21",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D27",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D28",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "D29",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D30",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D31",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "NBL3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "D21",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "D22",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "D23",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "NBL2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D24",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "D25",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "D26",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN0",
                signal: "NE4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN1",
                signal: "NBL0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN10",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN10",
                signal: "DA6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN11",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN11",
                signal: "DA7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN2",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN2",
                signal: "DA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN3",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN3",
                signal: "DA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN4",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN4",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN5",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN5",
                signal: "DA3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN6",
                signal: "SDCLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN7",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN8",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "DA5",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMC",
        }],
        afio: None,
    },
    Peripheral {
        name: "GFXMMU",
        address: 0x52010000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "GFXMMUEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "GFXMMURST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GFXMMU",
        }],
        afio: None,
    },
    Peripheral {
        name: "GFXTIM",
        address: 0x50004000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "GFXTIMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "GFXTIMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "FCKCAL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "LCKCAL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "FCKCAL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "LCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "FCKCAL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "LCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "FCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "LCKCAL",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GFXTIM",
        }],
        afio: None,
    },
    Peripheral {
        name: "GPDMA1",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH10",
                interrupt: "GPDMA1_CHANNEL10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "GPDMA1_CHANNEL11",
            },
            PeripheralInterrupt {
                signal: "CH12",
                interrupt: "GPDMA1_CHANNEL12",
            },
            PeripheralInterrupt {
                signal: "CH13",
                interrupt: "GPDMA1_CHANNEL13",
            },
            PeripheralInterrupt {
                signal: "CH14",
                interrupt: "GPDMA1_CHANNEL14",
            },
            PeripheralInterrupt {
                signal: "CH15",
                interrupt: "GPDMA1_CHANNEL15",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA1_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "GPDMA1_CHANNEL8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "GPDMA1_CHANNEL9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x58020000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOB",
        address: 0x58020400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOC",
        address: 0x58020800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOD",
        address: 0x58020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIODRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOE",
        address: 0x58021000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOERST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOF",
        address: 0x58021400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOFRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOG",
        address: 0x58021800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOH",
        address: 0x58021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOM",
        address: 0x58023000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPION",
        address: 0x58023400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIONEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIONRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOO",
        address: 0x58023800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOORST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOP",
        address: 0x58023c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPU2D",
        address: 0x52014000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ER",
            interrupt: "GPU2D_ER",
        }],
        afio: None,
    },
    Peripheral {
        name: "HASH",
        address: 0x48020400,
        registers: Some(PeripheralRegisters {
            kind: "hash",
            version: "v3",
            block: "HASH",
            ir: &hash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "HASH",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(6),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "HPDMA1",
        address: 0x52000000,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "HPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "HPDMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "HPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "HPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH10",
                interrupt: "HPDMA1_CHANNEL10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "HPDMA1_CHANNEL11",
            },
            PeripheralInterrupt {
                signal: "CH12",
                interrupt: "HPDMA1_CHANNEL12",
            },
            PeripheralInterrupt {
                signal: "CH13",
                interrupt: "HPDMA1_CHANNEL13",
            },
            PeripheralInterrupt {
                signal: "CH14",
                interrupt: "HPDMA1_CHANNEL14",
            },
            PeripheralInterrupt {
                signal: "CH15",
                interrupt: "HPDMA1_CHANNEL15",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "HPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "HPDMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "HPDMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "HPDMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "HPDMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "HPDMA1_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "HPDMA1_CHANNEL8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "HPDMA1_CHANNEL9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "I2C1_I3C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C1_I3C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C1_I3C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(68),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(70),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C3",
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(72),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I3C1",
        address: 0x40005400,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(107),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(109),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I3C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I3C1_EV",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "I3C1_WKUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x58004800,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v3",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "IWDG",
        }],
        afio: None,
    },
    Peripheral {
        name: "JPEG",
        address: 0x52003000,
        registers: Some(PeripheralRegisters {
            kind: "jpeg",
            version: "v1",
            block: "JPEG",
            ir: &jpeg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "JPEGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "JPEGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(1),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "JPEG",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_h7",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "LPTIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "LPTIM1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "IN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "IN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(91),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x58002400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_h7",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH2",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(17),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(94),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM3",
        address: 0x58002800,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_h7",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM3RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "ETR",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(104),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(105),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM4",
        address: 0x58002c00,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_h7",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM4RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM4",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM5",
        address: 0x58003000,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_h7",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM5RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD15",
                signal: "OUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "ETR",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM5",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x58000c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB45PERCKSELR",
                field: "LPUART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(102),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LTDC",
        address: 0x50001000,
        registers: Some(PeripheralRegisters {
            kind: "ltdc",
            version: "v1",
            block: "LTDC",
            ir: &ltdc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "LTDCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "LTDCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "G3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "G2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "B3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "B2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "R5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "B7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "DE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "R3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "B7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "B1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "R4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "B6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "B2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "R4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "R3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "R2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "R0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "R1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "VSYNC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "R2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "G0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "G1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "G0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "G1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "R0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "R7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "R6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "G1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CLK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "B1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "G0",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC_ER",
            },
            PeripheralInterrupt {
                signal: "LO",
                interrupt: "LTDC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "MCE1",
        address: 0x5200b800,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCE1",
        }],
        afio: None,
    },
    Peripheral {
        name: "MCE2",
        address: 0x5200bc00,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCE2",
        }],
        afio: None,
    },
    Peripheral {
        name: "MCE3",
        address: 0x5200c000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCE3",
        }],
        afio: None,
    },
    Peripheral {
        name: "MDIOS",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "mdios",
            version: "v1",
            block: "MDIOS",
            ir: &mdios::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "MDIOSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "MDIOSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "MDIOS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "MDIOS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "PKA",
        address: 0x48022000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "PKAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "PKARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PKA",
        }],
        afio: None,
    },
    Peripheral {
        name: "PSSI",
        address: 0x48000400,
        registers: Some(PeripheralRegisters {
            kind: "pssi",
            version: "v1_h7rs",
            block: "PSSI",
            ir: &pssi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "PSSISEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "PSSIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "PSSIRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "D0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D8",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PDCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RDY",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D15",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DE",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D4",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D6",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "PDCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D15",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "PDCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "RDY",
                af: Some(9),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PSSI",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(100),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PSSI",
        }],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x58024800,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "h7rs",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP3",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x58024400,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "h7rs",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "MCO_2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "OSC_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "CRS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x48020000,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v3",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "RNGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
        afio: None,
    },
    Peripheral {
        name: "RTC",
        address: 0x58004000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_h7rs",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "BDCR",
                field: "RTCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "RTCAPBEN",
            }),
            reset: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAES",
        address: 0x48021000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "SAESEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "SAESRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(5),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAES",
        }],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x42005800,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4_4pdm",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "SAI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CK1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "MCLK_A",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(64),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI1_A",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI1_B",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI2",
        address: 0x42005c00,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4_4pdm",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "SAI2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "FS_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCK_A",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "MCLK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "FS_B",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(66),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI2_A",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI2_B",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SDMMC1",
        address: 0x52007000,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v2",
            block: "SDMMC",
            ir: &sdmmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "SDMMCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "SDMMC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "SDMMC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "D0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CDIR",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D123DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SDMMC2",
        address: 0x48002400,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v2",
            block: "SDMMC",
            ir: &sdmmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "SDMMCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "SDMMC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "SDMMC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CMD",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CMD",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "D0",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPDIFRX1",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "spdifrx",
            version: "h7",
            block: "SPDIFRX",
            ir: &spdifrx::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "SPDIFRXSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "SPDIFRXEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "SPDIFRXRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "IN0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "IN0",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "IN1",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "IN2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "IN3",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DAT",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "CTRL",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(96),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPDIF_RX",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x42003000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "SPI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RDY",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RDY",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "MISO",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(52),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "SPI23SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(54),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "SPI23SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SDO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(56),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI4",
        address: 0x42003400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "SPI45SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(58),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0x42005000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "SPI45SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PM11",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PM12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PM13",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PM14",
                signal: "MISO",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(60),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI6",
        address: 0x58001400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB45PERCKSELR",
                field: "SPI6SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "SPI6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "SPI6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "NSS",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(62),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x58000400,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "h7rs",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "SYSCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TAMP",
        address: 0x58004400,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB9",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT8",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "OUT7",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "OUT5",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "OUT6",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TAMP",
        }],
        afio: None,
    },
    Peripheral {
        name: "TIM1",
        address: 0x42000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH2N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "ETR",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(13),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM12",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM13",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM13EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM13RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM13",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM14",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM14RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM14",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM15",
        address: 0x42004000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_2CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM15EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM15RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(44),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM15",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM16",
        address: 0x42004400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM16EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM16RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1N",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(47),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM17",
        address: 0x42004800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM17EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM17RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(50),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(17),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(19),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(25),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(31),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM5",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(37),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(38),
        }],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(39),
        }],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM9",
        address: 0x42004c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(80),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(82),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART7",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PM8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PM9",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("HPDMA1"),
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(84),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART8",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(86),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
        afio: None,
    },
    Peripheral {
        name: "UCPD1",
        address: 0x4000ec00,
        registers: Some(PeripheralRegisters {
            kind: "ucpd",
            version: "v1",
            block: "UCPD",
            ir: &ucpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "UCPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "UCPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD13",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PM0",
                signal: "CC1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PM1",
                signal: "CC2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PM2",
                signal: "DBCC1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PM3",
                signal: "DBCC2",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
        }],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0x8fff800,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
            ir: &uid::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x42001000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB2PERCKSELR",
                field: "USART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "DE",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "RTS",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(74),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(76),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "APB1PERCKSELR",
                field: "USART234578SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(78),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
        afio: None,
    },
    Peripheral {
        name: "USB_OTG_FS",
        address: 0x40080000,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "USB_OTG_FSSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "USB_OTG_FSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "USB_OTG_FSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM11",
                signal: "DP",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM12",
                signal: "DM",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM13",
                signal: "ID",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM14",
                signal: "VBUS",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_FS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 0x40040000,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "USBPHYCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "USB_OTG_HSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "USB_OTG_HSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM5",
                signal: "DM",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM6",
                signal: "DP",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM8",
                signal: "VBUS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PM9",
                signal: "ID",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x58003c00,
        registers: Some(PeripheralRegisters {
            kind: "vrefbuf",
            version: "v2a1",
            block: "VREFBUF",
            ir: &vrefbuf::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v2",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "WWDGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "XSPI1",
        address: 0x52005000,
        registers: Some(PeripheralRegisters {
            kind: "xspi",
            version: "v1",
            block: "XSPI",
            ir: &xspi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "XSPI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "XSPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "XSPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "RX",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("HPDMA1"),
            request: Some(2),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XSPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "XSPI2",
        address: 0x5200a000,
        registers: Some(PeripheralRegisters {
            kind: "xspi",
            version: "v1",
            block: "XSPI",
            ir: &xspi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "AHBPERCKSELR",
                field: "XSPI2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "XSPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "XSPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "RX",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("HPDMA1"),
            request: Some(3),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XSPI2",
        }],
        afio: None,
    },
    Peripheral {
        name: "XSPIM",
        address: 0x5200b400,
        registers: Some(PeripheralRegisters {
            kind: "xspim",
            version: "v1",
            block: "XSPIM",
            ir: &xspim::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PN0",
                signal: "P2_DQS0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN1",
                signal: "P2_NCS1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN10",
                signal: "P2_IO6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN11",
                signal: "P2_IO7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN12",
                signal: "P2_NCS2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN2",
                signal: "P2_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN3",
                signal: "P2_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN4",
                signal: "P2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN5",
                signal: "P2_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN6",
                signal: "P2_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN7",
                signal: "P2_NCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN8",
                signal: "P2_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "P2_IO5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO0",
                signal: "P1_NCS1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO1",
                signal: "P1_NCS2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO2",
                signal: "P1_DQS0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO3",
                signal: "P1_DQS1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO4",
                signal: "P1_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PO5",
                signal: "P1_NCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP0",
                signal: "P1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP1",
                signal: "P1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP10",
                signal: "P1_IO10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP11",
                signal: "P1_IO11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP12",
                signal: "P1_IO12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP13",
                signal: "P1_IO13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP14",
                signal: "P1_IO14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP15",
                signal: "P1_IO15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP2",
                signal: "P1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP3",
                signal: "P1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP4",
                signal: "P1_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP5",
                signal: "P1_IO5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP6",
                signal: "P1_IO6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP7",
                signal: "P1_IO7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP8",
                signal: "P1_IO8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PP9",
                signal: "P1_IO9",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "PVD_PVM",
        number: 0,
    },
    Interrupt { name: "DTS", number: 2 },
    Interrupt {
        name: "IWDG",
        number: 3,
    },
    Interrupt {
        name: "WWDG",
        number: 4,
    },
    Interrupt { name: "RCC", number: 5 },
    Interrupt {
        name: "FLASH",
        number: 8,
    },
    Interrupt {
        name: "RAMECC",
        number: 9,
    },
    Interrupt {
        name: "FPU",
        number: 10,
    },
    Interrupt {
        name: "TAMP",
        number: 13,
    },
    Interrupt {
        name: "EXTI0",
        number: 16,
    },
    Interrupt {
        name: "EXTI1",
        number: 17,
    },
    Interrupt {
        name: "EXTI2",
        number: 18,
    },
    Interrupt {
        name: "EXTI3",
        number: 19,
    },
    Interrupt {
        name: "EXTI4",
        number: 20,
    },
    Interrupt {
        name: "EXTI5",
        number: 21,
    },
    Interrupt {
        name: "EXTI6",
        number: 22,
    },
    Interrupt {
        name: "EXTI7",
        number: 23,
    },
    Interrupt {
        name: "EXTI8",
        number: 24,
    },
    Interrupt {
        name: "EXTI9",
        number: 25,
    },
    Interrupt {
        name: "EXTI10",
        number: 26,
    },
    Interrupt {
        name: "EXTI11",
        number: 27,
    },
    Interrupt {
        name: "EXTI12",
        number: 28,
    },
    Interrupt {
        name: "EXTI13",
        number: 29,
    },
    Interrupt {
        name: "EXTI14",
        number: 30,
    },
    Interrupt {
        name: "EXTI15",
        number: 31,
    },
    Interrupt {
        name: "RTC",
        number: 32,
    },
    Interrupt {
        name: "SAES",
        number: 33,
    },
    Interrupt {
        name: "CRYP",
        number: 34,
    },
    Interrupt {
        name: "PKA",
        number: 35,
    },
    Interrupt {
        name: "HASH",
        number: 36,
    },
    Interrupt {
        name: "RNG",
        number: 37,
    },
    Interrupt {
        name: "ADC1_2",
        number: 38,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL0",
        number: 39,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 40,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 41,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 42,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 43,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 44,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 45,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 46,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 47,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 48,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 49,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 50,
    },
    Interrupt {
        name: "TIM2",
        number: 51,
    },
    Interrupt {
        name: "TIM3",
        number: 52,
    },
    Interrupt {
        name: "TIM4",
        number: 53,
    },
    Interrupt {
        name: "TIM5",
        number: 54,
    },
    Interrupt {
        name: "TIM6",
        number: 55,
    },
    Interrupt {
        name: "TIM7",
        number: 56,
    },
    Interrupt {
        name: "TIM9",
        number: 57,
    },
    Interrupt {
        name: "SPI1",
        number: 58,
    },
    Interrupt {
        name: "SPI2",
        number: 59,
    },
    Interrupt {
        name: "SPI3",
        number: 60,
    },
    Interrupt {
        name: "SPI4",
        number: 61,
    },
    Interrupt {
        name: "SPI5",
        number: 62,
    },
    Interrupt {
        name: "SPI6",
        number: 63,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL0",
        number: 64,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL1",
        number: 65,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL2",
        number: 66,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL3",
        number: 67,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL4",
        number: 68,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL5",
        number: 69,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL6",
        number: 70,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL7",
        number: 71,
    },
    Interrupt {
        name: "SAI1_A",
        number: 72,
    },
    Interrupt {
        name: "SAI1_B",
        number: 73,
    },
    Interrupt {
        name: "SAI2_A",
        number: 74,
    },
    Interrupt {
        name: "SAI2_B",
        number: 75,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 76,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 77,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 78,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 79,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 80,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 81,
    },
    Interrupt {
        name: "USART1",
        number: 82,
    },
    Interrupt {
        name: "USART2",
        number: 83,
    },
    Interrupt {
        name: "USART3",
        number: 84,
    },
    Interrupt {
        name: "UART4",
        number: 85,
    },
    Interrupt {
        name: "UART5",
        number: 86,
    },
    Interrupt {
        name: "UART7",
        number: 87,
    },
    Interrupt {
        name: "UART8",
        number: 88,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 89,
    },
    Interrupt {
        name: "I3C1_ER",
        number: 90,
    },
    Interrupt {
        name: "OTG_HS",
        number: 91,
    },
    Interrupt {
        name: "ETH",
        number: 92,
    },
    Interrupt {
        name: "CORDIC",
        number: 93,
    },
    Interrupt {
        name: "GFXTIM",
        number: 94,
    },
    Interrupt {
        name: "DCMIPP",
        number: 95,
    },
    Interrupt {
        name: "LTDC",
        number: 96,
    },
    Interrupt {
        name: "LTDC_ER",
        number: 97,
    },
    Interrupt {
        name: "DMA2D",
        number: 98,
    },
    Interrupt {
        name: "JPEG",
        number: 99,
    },
    Interrupt {
        name: "GFXMMU",
        number: 100,
    },
    Interrupt {
        name: "I3C1_WKUP",
        number: 101,
    },
    Interrupt {
        name: "MCE1",
        number: 102,
    },
    Interrupt {
        name: "MCE2",
        number: 103,
    },
    Interrupt {
        name: "MCE3",
        number: 104,
    },
    Interrupt {
        name: "XSPI1",
        number: 105,
    },
    Interrupt {
        name: "XSPI2",
        number: 106,
    },
    Interrupt {
        name: "FMC",
        number: 107,
    },
    Interrupt {
        name: "SDMMC1",
        number: 108,
    },
    Interrupt {
        name: "SDMMC2",
        number: 109,
    },
    Interrupt {
        name: "OTG_FS",
        number: 112,
    },
    Interrupt {
        name: "TIM12",
        number: 113,
    },
    Interrupt {
        name: "TIM13",
        number: 114,
    },
    Interrupt {
        name: "TIM14",
        number: 115,
    },
    Interrupt {
        name: "TIM15",
        number: 116,
    },
    Interrupt {
        name: "TIM16",
        number: 117,
    },
    Interrupt {
        name: "TIM17",
        number: 118,
    },
    Interrupt {
        name: "LPTIM1",
        number: 119,
    },
    Interrupt {
        name: "LPTIM2",
        number: 120,
    },
    Interrupt {
        name: "LPTIM3",
        number: 121,
    },
    Interrupt {
        name: "LPTIM4",
        number: 122,
    },
    Interrupt {
        name: "LPTIM5",
        number: 123,
    },
    Interrupt {
        name: "SPDIF_RX",
        number: 124,
    },
    Interrupt {
        name: "MDIOS",
        number: 125,
    },
    Interrupt {
        name: "ADF1_FLT0",
        number: 126,
    },
    Interrupt {
        name: "CRS",
        number: 127,
    },
    Interrupt {
        name: "UCPD1",
        number: 128,
    },
    Interrupt {
        name: "CEC",
        number: 129,
    },
    Interrupt {
        name: "PSSI",
        number: 130,
    },
    Interrupt {
        name: "LPUART1",
        number: 131,
    },
    Interrupt {
        name: "WAKEUP_PIN",
        number: 132,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL8",
        number: 133,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL9",
        number: 134,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL10",
        number: 135,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL11",
        number: 136,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL12",
        number: 137,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL13",
        number: 138,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL14",
        number: 139,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL15",
        number: 140,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL8",
        number: 141,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL9",
        number: 142,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL10",
        number: 143,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL11",
        number: 144,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL12",
        number: 145,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL13",
        number: 146,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL14",
        number: 147,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL15",
        number: 148,
    },
    Interrupt {
        name: "GPU2D",
        number: 149,
    },
    Interrupt {
        name: "GPU2D_ER",
        number: 150,
    },
    Interrupt {
        name: "ICACHE",
        number: 151,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 152,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 153,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 154,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 155,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "GPDMA1_CH0",
        dma: "GPDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH1",
        dma: "GPDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH2",
        dma: "GPDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH3",
        dma: "GPDMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH4",
        dma: "GPDMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH5",
        dma: "GPDMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH6",
        dma: "GPDMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH7",
        dma: "GPDMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH8",
        dma: "GPDMA1",
        channel: 8,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH9",
        dma: "GPDMA1",
        channel: 9,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH10",
        dma: "GPDMA1",
        channel: 10,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH11",
        dma: "GPDMA1",
        channel: 11,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH12",
        dma: "GPDMA1",
        channel: 12,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH13",
        dma: "GPDMA1",
        channel: 13,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH14",
        dma: "GPDMA1",
        channel: 14,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH15",
        dma: "GPDMA1",
        channel: 15,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH0",
        dma: "HPDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH1",
        dma: "HPDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH2",
        dma: "HPDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH3",
        dma: "HPDMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH4",
        dma: "HPDMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH5",
        dma: "HPDMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH6",
        dma: "HPDMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH7",
        dma: "HPDMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH8",
        dma: "HPDMA1",
        channel: 8,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH9",
        dma: "HPDMA1",
        channel: 9,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH10",
        dma: "HPDMA1",
        channel: 10,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH11",
        dma: "HPDMA1",
        channel: 11,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH12",
        dma: "HPDMA1",
        channel: 12,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH13",
        dma: "HPDMA1",
        channel: 13,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH14",
        dma: "HPDMA1",
        channel: 14,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "HPDMA1_CH15",
        dma: "HPDMA1",
        channel: 15,
        dmamux: None,
        dmamux_channel: None,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA0" },
    Pin { name: "PA1" },
    Pin { name: "PA2" },
    Pin { name: "PA3" },
    Pin { name: "PA4" },
    Pin { name: "PA5" },
    Pin { name: "PA6" },
    Pin { name: "PA7" },
    Pin { name: "PA8" },
    Pin { name: "PA9" },
    Pin { name: "PA10" },
    Pin { name: "PA11" },
    Pin { name: "PA12" },
    Pin { name: "PA13" },
    Pin { name: "PA14" },
    Pin { name: "PA15" },
    Pin { name: "PB0" },
    Pin { name: "PB1" },
    Pin { name: "PB2" },
    Pin { name: "PB3" },
    Pin { name: "PB4" },
    Pin { name: "PB5" },
    Pin { name: "PB6" },
    Pin { name: "PB7" },
    Pin { name: "PB8" },
    Pin { name: "PB9" },
    Pin { name: "PB10" },
    Pin { name: "PB11" },
    Pin { name: "PB12" },
    Pin { name: "PB13" },
    Pin { name: "PB14" },
    Pin { name: "PB15" },
    Pin { name: "PC0" },
    Pin { name: "PC1" },
    Pin { name: "PC2" },
    Pin { name: "PC3" },
    Pin { name: "PC4" },
    Pin { name: "PC5" },
    Pin { name: "PC6" },
    Pin { name: "PC7" },
    Pin { name: "PC8" },
    Pin { name: "PC9" },
    Pin { name: "PC10" },
    Pin { name: "PC11" },
    Pin { name: "PC12" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PD0" },
    Pin { name: "PD1" },
    Pin { name: "PD2" },
    Pin { name: "PD3" },
    Pin { name: "PD4" },
    Pin { name: "PD5" },
    Pin { name: "PD6" },
    Pin { name: "PD7" },
    Pin { name: "PD8" },
    Pin { name: "PD9" },
    Pin { name: "PD10" },
    Pin { name: "PD11" },
    Pin { name: "PD12" },
    Pin { name: "PD13" },
    Pin { name: "PD14" },
    Pin { name: "PD15" },
    Pin { name: "PE0" },
    Pin { name: "PE1" },
    Pin { name: "PE2" },
    Pin { name: "PE3" },
    Pin { name: "PE4" },
    Pin { name: "PE5" },
    Pin { name: "PE6" },
    Pin { name: "PE7" },
    Pin { name: "PE8" },
    Pin { name: "PE9" },
    Pin { name: "PE10" },
    Pin { name: "PE11" },
    Pin { name: "PE12" },
    Pin { name: "PE13" },
    Pin { name: "PE14" },
    Pin { name: "PE15" },
    Pin { name: "PF0" },
    Pin { name: "PF1" },
    Pin { name: "PF2" },
    Pin { name: "PF3" },
    Pin { name: "PF4" },
    Pin { name: "PF5" },
    Pin { name: "PF6" },
    Pin { name: "PF7" },
    Pin { name: "PF8" },
    Pin { name: "PF9" },
    Pin { name: "PF10" },
    Pin { name: "PF11" },
    Pin { name: "PF12" },
    Pin { name: "PF13" },
    Pin { name: "PF14" },
    Pin { name: "PF15" },
    Pin { name: "PG0" },
    Pin { name: "PG1" },
    Pin { name: "PG2" },
    Pin { name: "PG3" },
    Pin { name: "PG4" },
    Pin { name: "PG5" },
    Pin { name: "PG6" },
    Pin { name: "PG7" },
    Pin { name: "PG8" },
    Pin { name: "PG9" },
    Pin { name: "PG10" },
    Pin { name: "PG11" },
    Pin { name: "PG12" },
    Pin { name: "PG13" },
    Pin { name: "PG14" },
    Pin { name: "PG15" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
    Pin { name: "PM0" },
    Pin { name: "PM1" },
    Pin { name: "PM2" },
    Pin { name: "PM3" },
    Pin { name: "PM5" },
    Pin { name: "PM6" },
    Pin { name: "PM8" },
    Pin { name: "PM9" },
    Pin { name: "PM11" },
    Pin { name: "PM12" },
    Pin { name: "PM13" },
    Pin { name: "PM14" },
    Pin { name: "PN0" },
    Pin { name: "PN1" },
    Pin { name: "PN2" },
    Pin { name: "PN3" },
    Pin { name: "PN4" },
    Pin { name: "PN5" },
    Pin { name: "PN6" },
    Pin { name: "PN7" },
    Pin { name: "PN8" },
    Pin { name: "PN9" },
    Pin { name: "PN10" },
    Pin { name: "PN11" },
    Pin { name: "PN12" },
    Pin { name: "PO0" },
    Pin { name: "PO1" },
    Pin { name: "PO2" },
    Pin { name: "PO3" },
    Pin { name: "PO4" },
    Pin { name: "PO5" },
    Pin { name: "PP0" },
    Pin { name: "PP1" },
    Pin { name: "PP2" },
    Pin { name: "PP3" },
    Pin { name: "PP4" },
    Pin { name: "PP5" },
    Pin { name: "PP6" },
    Pin { name: "PP7" },
    Pin { name: "PP8" },
    Pin { name: "PP9" },
    Pin { name: "PP10" },
    Pin { name: "PP11" },
    Pin { name: "PP12" },
    Pin { name: "PP13" },
    Pin { name: "PP14" },
    Pin { name: "PP15" },
];
#[path = "../registers/adc_h7rs.rs"]
pub mod adc;
#[path = "../registers/adccommon_h5.rs"]
pub mod adccommon;
#[path = "../registers/can_fdcan_v1.rs"]
pub mod can;
#[path = "../registers/cec_v2.rs"]
pub mod cec;
#[path = "../registers/cordic_v1.rs"]
pub mod cordic;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/crs_v1.rs"]
pub mod crs;
#[path = "../registers/cryp_v4.rs"]
pub mod cryp;
#[path = "../registers/dbgmcu_h7.rs"]
pub mod dbgmcu;
#[path = "../registers/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../registers/dma2d_v2.rs"]
pub mod dma2d;
#[path = "../registers/dts_v1.rs"]
pub mod dts;
#[path = "../registers/eth_v2.rs"]
pub mod eth;
#[path = "../registers/exti_h7.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/flash_h7rs.rs"]
pub mod flash;
#[path = "../registers/fmc_v3x1.rs"]
pub mod fmc;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/hash_v3.rs"]
pub mod hash;
#[path = "../registers/i2c_v3.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../registers/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../registers/lptim_v1b_h7.rs"]
pub mod lptim;
#[path = "../registers/ltdc_v1.rs"]
pub mod ltdc;
#[path = "../registers/mdios_v1.rs"]
pub mod mdios;
#[path = "../registers/otg_v1.rs"]
pub mod otg;
#[path = "../registers/pssi_v1_h7rs.rs"]
pub mod pssi;
#[path = "../registers/pwr_h7rs.rs"]
pub mod pwr;
#[path = "../registers/rcc_h7rs.rs"]
pub mod rcc;
#[path = "../registers/rng_v3.rs"]
pub mod rng;
#[path = "../registers/rtc_v3_h7rs.rs"]
pub mod rtc;
#[path = "../registers/sai_v4_4pdm.rs"]
pub mod sai;
#[path = "../registers/sdmmc_v2.rs"]
pub mod sdmmc;
#[path = "../registers/spdifrx_h7.rs"]
pub mod spdifrx;
#[path = "../registers/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../registers/syscfg_h7rs.rs"]
pub mod syscfg;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/vrefbuf_v2a1.rs"]
pub mod vrefbuf;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
#[path = "../registers/xspi_v1.rs"]
pub mod xspi;
#[path = "../registers/xspim_v1.rs"]
pub mod xspim;
