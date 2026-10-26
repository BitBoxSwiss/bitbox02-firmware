
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f1",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "ADC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            remap: &[],
            dma: None,
            request: None,
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x40012400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x40012800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f1",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "ADC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "ADC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "f1",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "AFIORST",
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
        name: "BKP",
        address: 0x40006c00,
        registers: Some(PeripheralRegisters {
            kind: "bkp",
            version: "v1",
            block: "BKP",
            ir: &bkp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "BKPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "BKPRST",
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
        name: "CAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "CAN1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "CAN1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
            },
        ],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "CAN1_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA11", "PA12"],
                },
                PeripheralAfioValue {
                    value: 2,
                    pins: &["PB8", "PB9"],
                },
                PeripheralAfioValue {
                    value: 3,
                    pins: &["PD0", "PD1"],
                },
            ],
        }),
    },
    Peripheral {
        name: "CAN2",
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "CAN2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "CAN2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN2_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN2_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN2_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN2_TX",
            },
        ],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "CAN2_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PB12", "PB13"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PB5", "PB6"],
                },
            ],
        }),
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v1",
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
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v1",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "DACRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG0",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM5_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG4",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG6",
                source: "EXTI9_TRG",
            },
        ],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0042000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f1",
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
        name: "DMA1",
        address: 0x40020000,
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
                field: "DMA1EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "DMA2",
        address: 0x40020400,
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
                field: "DMA2EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "ETH",
        address: 0x40028000,
        registers: Some(PeripheralRegisters {
            kind: "eth",
            version: "v1a",
            block: "ETH",
            ir: &eth::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "ETHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "ETHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CRS",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REF_CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX_CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "COL",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CRS_DV",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX_DV",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RXD2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RXD3",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX_ER",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX_EN",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD0",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PPS_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TXD3",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TXD2",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX_CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RXD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RXD1",
                af: None,
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
                interrupt: "ETH_WKUP",
            },
        ],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "ETH_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &[
                        "PA0", "PA1", "PA2", "PA3", "PC1", "PC2", "PC3", "PB5", "PB8", "PB10", "PB11", "PB12", "PB13",
                        "PA7", "PC4", "PC5", "PB0", "PB1",
                    ],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &[
                        "PA0", "PA1", "PA2", "PA3", "PC1", "PC2", "PC3", "PB5", "PB8", "PB10", "PB11", "PB12", "PB13",
                        "PD8", "PD9", "PD10", "PD11", "PD12",
                    ],
                },
            ],
        }),
    },
    Peripheral {
        name: "EXTI",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "v1",
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
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
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
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f1",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
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
        address: 0x40010800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40010c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40011400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40011800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        name: "I2C1",
        address: 0x40005400,
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
                pin: "PB5",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "I2C1_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PB6", "PB7"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PB8", "PB9"],
                },
            ],
        }),
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v1",
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "f1",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "PWRRST",
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
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "f1cl",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: None,
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
                pin: "PD0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "OSC_OUT",
                af: None,
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
        name: "RTC",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v1",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMPER",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_ALARM",
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
                interrupt: "TAMPER",
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
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v1_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
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
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "SPI1_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA4", "PA5", "PA6", "PA7"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PA15", "PB3", "PB4", "PB5"],
                },
            ],
        }),
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v1_i2s",
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
                pin: "PB12",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
            version: "v1_i2s",
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
                pin: "PA15",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SD",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "SPI3_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PC7", "PA15", "PB3", "PB4", "PB5"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PC7", "PA4", "PC10", "PC11", "PC12"],
                },
            ],
        }),
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "TIM1_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA12", "PA8", "PA9", "PA10", "PA11", "PB12", "PB13", "PB14", "PB15"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PA12", "PA8", "PA9", "PA10", "PA11", "PA6", "PA7", "PB0", "PB1"],
                },
                PeripheralAfioValue {
                    value: 3,
                    pins: &["PE7", "PE9", "PE11", "PE13", "PE14", "PE15", "PE8", "PE10", "PE12"],
                },
            ],
        }),
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "TIM2_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA0", "PA1", "PA2", "PA3"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PA15", "PB3", "PA2", "PA3"],
                },
                PeripheralAfioValue {
                    value: 2,
                    pins: &["PA0", "PA1", "PB10", "PB11"],
                },
                PeripheralAfioValue {
                    value: 3,
                    pins: &["PA15", "PB3", "PB10", "PB11"],
                },
            ],
        }),
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "TIM3_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PD2", "PA6", "PA7", "PB0", "PB1"],
                },
                PeripheralAfioValue {
                    value: 2,
                    pins: &["PD2", "PB4", "PB5", "PB0", "PB1"],
                },
                PeripheralAfioValue {
                    value: 3,
                    pins: &["PD2", "PC6", "PC7", "PC8", "PC9"],
                },
            ],
        }),
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "TIM4_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PE0", "PB6", "PB7", "PB8", "PB9"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PE0", "PD12", "PD13", "PD14", "PD15"],
                },
            ],
        }),
    },
    Peripheral {
        name: "TIM5",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH3"),
            dmamux: None,
            remap: &[],
            dma: None,
            request: None,
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
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            remap: &[],
            dma: None,
            request: None,
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
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
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
            version: "v1",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0x1ffff7e8,
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
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
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
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "USART1_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA11", "PA12", "PA8", "PA9", "PA10"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PA11", "PA12", "PA8", "PB6", "PB7"],
                },
            ],
        }),
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "USART2_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PA0", "PA1", "PA2", "PA3", "PA4"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PD3", "PD4", "PD5", "PD6", "PD7"],
                },
            ],
        }),
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: None,
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
        afio: Some(PeripheralAfio {
            register: "MAPR",
            field: "USART3_REMAP",
            values: &[
                PeripheralAfioValue {
                    value: 0,
                    pins: &["PB10", "PB11", "PB12", "PB13", "PB14"],
                },
                PeripheralAfioValue {
                    value: 1,
                    pins: &["PC10", "PC11", "PC12", "PB13", "PB14"],
                },
                PeripheralAfioValue {
                    value: 3,
                    pins: &["PD8", "PD9", "PD10", "PD11", "PD12"],
                },
            ],
        }),
    },
    Peripheral {
        name: "USB_OTG_FS",
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("USB"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "USB_OTG_FSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "USB_OTG_FSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "VBUS",
                af: None,
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
                interrupt: "OTG_FS_WKUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v1",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "WWDGRST",
            }),
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
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt { name: "PVD", number: 1 },
    Interrupt {
        name: "TAMPER",
        number: 2,
    },
    Interrupt { name: "RTC", number: 3 },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt { name: "RCC", number: 5 },
    Interrupt {
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC1_2",
        number: 18,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
        number: 30,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "USART3",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "OTG_FS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 60,
    },
    Interrupt {
        name: "ETH",
        number: 61,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 62,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 66,
    },
    Interrupt {
        name: "OTG_FS",
        number: 67,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
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
];
#[path = "../registers/adc_f1.rs"]
pub mod adc;
#[path = "../registers/afio_f1.rs"]
pub mod afio;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/bkp_v1.rs"]
pub mod bkp;
#[path = "../registers/can_bxcan.rs"]
pub mod can;
#[path = "../registers/crc_v1.rs"]
pub mod crc;
#[path = "../registers/dac_v1.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_f1.rs"]
pub mod dbgmcu;
#[path = "../registers/eth_v1a.rs"]
pub mod eth;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/flash_f1.rs"]
pub mod flash;
#[path = "../registers/gpio_v1.rs"]
pub mod gpio;
#[path = "../registers/i2c_v1.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../registers/otg_v1.rs"]
pub mod otg;
#[path = "../registers/pwr_f1.rs"]
pub mod pwr;
#[path = "../registers/rcc_f1cl.rs"]
pub mod rcc;
#[path = "../registers/rtc_v1.rs"]
pub mod rtc;
#[path = "../registers/spi_v1_i2s.rs"]
pub mod spi;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v1.rs"]
pub mod usart;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;
