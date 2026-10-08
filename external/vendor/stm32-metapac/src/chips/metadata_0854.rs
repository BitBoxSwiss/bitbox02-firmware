
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x41006000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "wb0",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "VINM3",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "VINP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "VINM2",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "PDM_CLK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "PDM_DATA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "PDM_CLK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PDM_DATA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PGA_CAP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "PGA_CAP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "PDM_DATA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "PGA_VIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PDM_CLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PGA_VBIAS_MIC",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(5),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x48200000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v3",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
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
        name: "DMA1",
        address: 0x48700000,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x48800000,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
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
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "wb0",
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
        name: "GPIOA",
        address: 0x48000000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
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
        address: 0x48100000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
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
        name: "I2C1",
        address: 0x41000000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v1",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCL",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SDA",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SMBA",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SMBA",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(11),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C2",
        address: 0x41001000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v1",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SDA",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SMBA",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x41005000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "LPUARTEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "LPUARTRST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "DE",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CTS",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(17),
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
        name: "PKA",
        address: 0x48300000,
        registers: Some(PeripheralRegisters {
            kind: "pka",
            version: "v1c",
            block: "PKA",
            ir: &pka::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "PKAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
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
        name: "PWR",
        address: 0x48500000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "wb0",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP12",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "WKUP13",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "WKUP10",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "WKUP22",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "WKUP11",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "WKUP23",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "WKUP24",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "WKUP25",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "WKUP26",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "WKUP27",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "WKUP15",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "WKUP16",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "WKUP17",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "WKUP18",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "WKUP19",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "WKUP20",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "WKUP21",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "WKUP9",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "WKUP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "WKUP22",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "WKUP23",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "WKUP20",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "WKUP21",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PVD",
            interrupt: "PVD",
        }],
        afio: None,
    },
    Peripheral {
        name: "RADIO",
        address: 0x60000000,
        registers: Some(PeripheralRegisters {
            kind: "radio",
            version: "v1",
            block: "RADIO",
            ir: &radio::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "TX_SEQUENCE",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX_SEQUENCE",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX_SEQUENCE",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX_SEQUENCE",
                af: Some(1),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SEQ",
                interrupt: "RADIO_TXRX_SEQ",
            },
            PeripheralInterrupt {
                signal: "TXRX",
                interrupt: "RADIO_TXRX",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x48400000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "wb0",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "LCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "LCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "LCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "LCO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MCO",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x48600000,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "RNGRST",
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
        name: "RRM",
        address: 0x60001400,
        registers: Some(PeripheralRegisters {
            kind: "rrm",
            version: "v1",
            block: "RRM",
            ir: &rrm::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RTC",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_base",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK0",
            kernel_clock: Clock("PCLK0"),
            enable: Some(PeripheralRccRegister {
                register: "APB0ENR",
                field: "RTCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB0RSTR",
                field: "RTCRST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "OUT",
                af: Some(2),
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
        name: "SPI1",
        address: 0x41002000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "NSS",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MISO",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MOSI",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MOSI",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(7),
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
        address: 0x41003000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_MCK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_SD",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MISO",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_WS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "NSS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_MCK",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(9),
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
        address: 0x41007000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_MCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_SD",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MOSI",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_MCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_SCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MISO",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_WS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "NSS",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "wb0",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK0",
            kernel_clock: Clock("PCLK0"),
            enable: Some(PeripheralRccRegister {
                register: "APB0ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB0RSTR",
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
        name: "TIM1",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK0",
            kernel_clock: Clock("PCLK0_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB0ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB0RSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "ETR",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "BKIN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH2N",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(27),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x41004000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART1EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "DE",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(15),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "FLASH",
        number: 0,
    },
    Interrupt { name: "RCC", number: 1 },
    Interrupt { name: "PVD", number: 2 },
    Interrupt {
        name: "I2C1",
        number: 3,
    },
    Interrupt {
        name: "I2C2",
        number: 4,
    },
    Interrupt {
        name: "SPI1",
        number: 5,
    },
    Interrupt {
        name: "SPI2",
        number: 6,
    },
    Interrupt {
        name: "SPI3",
        number: 7,
    },
    Interrupt {
        name: "USART1",
        number: 8,
    },
    Interrupt {
        name: "LPUART1",
        number: 9,
    },
    Interrupt {
        name: "TIM1",
        number: 10,
    },
    Interrupt {
        name: "RTC",
        number: 11,
    },
    Interrupt {
        name: "ADC",
        number: 12,
    },
    Interrupt {
        name: "PKA",
        number: 13,
    },
    Interrupt {
        name: "UPCONV",
        number: 14,
    },
    Interrupt {
        name: "GPIOA",
        number: 15,
    },
    Interrupt {
        name: "GPIOB",
        number: 16,
    },
    Interrupt {
        name: "DMA",
        number: 17,
    },
    Interrupt {
        name: "RADIO_TXRX",
        number: 18,
    },
    Interrupt {
        name: "RADIO_TIMER_ERROR",
        number: 20,
    },
    Interrupt {
        name: "RADIO_TIMER_CPU_WKUP",
        number: 23,
    },
    Interrupt {
        name: "RADIO_TIMER_TXRX_WKUP",
        number: 24,
    },
    Interrupt {
        name: "RADIO_TXRX_SEQ",
        number: 25,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
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
];
#[path = "../registers/adc_wb0.rs"]
pub mod adc;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../registers/flash_wb0.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v1.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pka_v1c.rs"]
pub mod pka;
#[path = "../registers/pwr_wb0.rs"]
pub mod pwr;
#[path = "../registers/radio_v1.rs"]
pub mod radio;
#[path = "../registers/rcc_wb0.rs"]
pub mod rcc;
#[path = "../registers/rng_v1.rs"]
pub mod rng;
#[path = "../registers/rrm_v1.rs"]
pub mod rrm;
#[path = "../registers/rtc_v3_base.rs"]
pub mod rtc;
#[path = "../registers/spi_v3.rs"]
pub mod spi;
#[path = "../registers/syscfg_wb0.rs"]
pub mod syscfg;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
