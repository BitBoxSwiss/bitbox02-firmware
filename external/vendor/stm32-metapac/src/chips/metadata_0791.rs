
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x42028000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "u3",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(0),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x42028300,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "u3",
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
        address: 0x42028100,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "u3",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(1),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADF1",
        address: 0x40034000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADF1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR1",
                field: "ADF1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR1",
                field: "ADF1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "CCK0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDI0",
                af: Some(3),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "FLT0",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(98),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLT0",
            interrupt: "ADF1",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP1",
        address: 0x40045400,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "u3",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP2",
        address: 0x40045404,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "u3",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
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
                register: "AHB1ENR1",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR1",
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
        address: 0x40006000,
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
                register: "APB1ENR1",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "CRSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SYNC",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SYNC",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x42028400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v6",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
                field: "DAC1RST",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC1",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(2),
        }],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG5",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG6",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG8",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG11",
                source: "LPTIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG12",
                source: "LPTIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG13",
                source: "EXTI9_TRG",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0044000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "u3",
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
        name: "DLYB_OCTOSPI1",
        address: 0x420cf000,
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
        name: "DLYB_SDMMC1",
        address: 0x420c8400,
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
        name: "EXTI",
        address: 0x40032000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "u3",
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
                register: "CCIPR1",
                field: "FDCAN1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "FDCAN1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "FDCAN1RST",
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
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "u3",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR1",
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
        name: "GPDMA1",
        address: 0x40020000,
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
                register: "AHB1ENR1",
                field: "GPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR1",
                field: "GPDMA1RST",
            }),
            stop_mode: StopMode::Stop2,
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
        address: 0x42020000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x42020400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x42020800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x42020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x42021000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        name: "GPIOG",
        address: 0x42021800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x42021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        name: "GTZC",
        address: 0x40032400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HASH",
        address: 0x420c0400,
        registers: Some(PeripheralRegisters {
            kind: "hash",
            version: "v3",
            block: "HASH",
            ir: &hash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "IN",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(89),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "I2C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(4),
            },
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
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(14),
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
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "I2C2SEL",
            }),
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
                pin: "PA1",
                signal: "SMBA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(17),
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
        address: 0x40042800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "I2C3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
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
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(20),
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
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "i3c",
            version: "v1",
            block: "I3C",
            ir: &i3c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "I3C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I3C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I3C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "SDA",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(52),
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
        ],
        afio: None,
    },
    Peripheral {
        name: "I3C2",
        address: 0x40016c00,
        registers: Some(PeripheralRegisters {
            kind: "i3c",
            version: "v1",
            block: "I3C",
            ir: &i3c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "I3C2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "I3C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "I3C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "SDA",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(75),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I3C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I3C2_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "ICACHE",
        address: 0x40030400,
        registers: Some(PeripheralRegisters {
            kind: "icache",
            version: "v1_3crr",
            block: "ICACHE",
            ir: &icache::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ICACHE",
        }],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
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
        name: "LPTIM1",
        address: 0x40044400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "LPTIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
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
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(107),
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
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "LPTIM2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "IN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN2",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(110),
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
        address: 0x40044800,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM3RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(113),
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
        address: 0x40044c00,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM_BASIC",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM4RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM4",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40042400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "LPUART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
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
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(35),
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
        name: "OCTOSPI1",
        address: 0x420d1400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "OCTOSPISEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR2",
                field: "OCTOSPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR2",
                field: "OCTOSPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NCS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IO3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NCLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "NCLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "IO5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IO6",
                af: Some(10),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "OCTOSPI1",
            channel: None,
            dmamux: None,
            remap: &[],
            dma: Some("GPDMA1"),
            request: Some(40),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OCTOSPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v3",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "OPAMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "OPAMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OPAMP2",
        address: 0x40007010,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v3",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "OPAMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "OPAMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x40030800,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "u3",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR2",
                field: "PWREN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PVD",
                interrupt: "PWR",
            },
            PeripheralInterrupt {
                signal: "S3WU",
                interrupt: "PWR",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "PWR",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "ramcfg",
            version: "u5",
            block: "RAMCFG",
            ir: &ramcfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR1",
                field: "RAMCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR1",
                field: "RAMCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BKP",
                interrupt: "RAMCFG",
            },
            PeripheralInterrupt {
                signal: "ECC",
                interrupt: "RAMCFG",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x40030c00,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "u3",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MCO_2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MCO",
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
        address: 0x420c0800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "RNGSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR1",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR1",
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
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_u3",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "BDCR",
                field: "RTCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
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
                af: None,
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
        name: "SAI1",
        address: 0x40015400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
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
                pin: "PA10",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MCLK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FS_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MCLK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MCLK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FS_A",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "B",
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
                signal: "A",
                interrupt: "SAI1",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v6",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                pin: "PA1",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
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
                dma: Some("GPDMA1"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
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
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v6",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "SPI2SEL",
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
                pin: "PA9",
                signal: "SCK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
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
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
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
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v6",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "SPI3SEL",
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
                pin: "PA0",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RDY",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(11),
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
        name: "SYSCFG",
        address: 0x40040400,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "u3",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
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
        address: 0x40007c00,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN1",
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
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
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
                signal: "BKIN2",
                af: Some(2),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "CH4",
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
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(48),
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
        name: "TIM15",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
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
                pin: "PA1",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(81),
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
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
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
                pin: "PA6",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(83),
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
        address: 0x40014800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
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
                pin: "PA10",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(85),
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
            version: "v2",
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
                pin: "PA0",
                signal: "ETR",
                af: Some(14),
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
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(60),
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
            version: "v2",
            block: "TIM_GP32",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "TRG",
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
            version: "v2",
            block: "TIM_GP32",
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
                pin: "PA12",
                signal: "ETR",
                af: Some(2),
            },
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
                pin: "PH3",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(71),
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
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
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
            request: Some(4),
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
            version: "v2",
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
            request: Some(5),
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
        name: "TSC",
        address: 0x40024000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR1",
                field: "TSCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR1",
                field: "TSCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "G3_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G1_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G2_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "G2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "G2_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "G2_IO4",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSC",
        }],
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
                register: "CCIPR1",
                field: "UART4SEL",
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
                pin: "PB7",
                signal: "CTS",
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
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(31),
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
        name: "UID",
        address: 0xbfa0700,
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
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                pin: "PB3",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(7),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(25),
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
                register: "CCIPR1",
                field: "USART3SEL",
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
                pin: "PA15",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(29),
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
        name: "USB",
        address: 0x40016000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v4",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "ICLKSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
                pin: "PA13",
                signal: "NOE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_FS",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB_FS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "USBRAM",
        address: 0x40016400,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "32_2048",
            block: "USBRAM",
            ir: &usbram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x40007400,
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
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "PVD_PVM",
        number: 1,
    },
    Interrupt { name: "RTC", number: 2 },
    Interrupt {
        name: "RTC_S",
        number: 3,
    },
    Interrupt {
        name: "TAMP",
        number: 4,
    },
    Interrupt {
        name: "RAMCFG",
        number: 5,
    },
    Interrupt {
        name: "FLASH",
        number: 6,
    },
    Interrupt {
        name: "FLASH_S",
        number: 7,
    },
    Interrupt {
        name: "GTZC",
        number: 8,
    },
    Interrupt { name: "RCC", number: 9 },
    Interrupt {
        name: "RCC_S",
        number: 10,
    },
    Interrupt {
        name: "EXTI0",
        number: 11,
    },
    Interrupt {
        name: "EXTI1",
        number: 12,
    },
    Interrupt {
        name: "EXTI2",
        number: 13,
    },
    Interrupt {
        name: "EXTI3",
        number: 14,
    },
    Interrupt {
        name: "EXTI4",
        number: 15,
    },
    Interrupt {
        name: "EXTI5",
        number: 16,
    },
    Interrupt {
        name: "EXTI6",
        number: 17,
    },
    Interrupt {
        name: "EXTI7",
        number: 18,
    },
    Interrupt {
        name: "EXTI8",
        number: 19,
    },
    Interrupt {
        name: "EXTI9",
        number: 20,
    },
    Interrupt {
        name: "EXTI10",
        number: 21,
    },
    Interrupt {
        name: "EXTI11",
        number: 22,
    },
    Interrupt {
        name: "EXTI12",
        number: 23,
    },
    Interrupt {
        name: "EXTI13",
        number: 24,
    },
    Interrupt {
        name: "EXTI14",
        number: 25,
    },
    Interrupt {
        name: "EXTI15",
        number: 26,
    },
    Interrupt {
        name: "IWDG",
        number: 27,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL0",
        number: 29,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 30,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 31,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 32,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 33,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 34,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 35,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 36,
    },
    Interrupt {
        name: "ADC1",
        number: 37,
    },
    Interrupt {
        name: "DAC1",
        number: 38,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 39,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 40,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 41,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 42,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 43,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 44,
    },
    Interrupt {
        name: "TIM2",
        number: 45,
    },
    Interrupt {
        name: "TIM3",
        number: 46,
    },
    Interrupt {
        name: "TIM4",
        number: 47,
    },
    Interrupt {
        name: "TIM6",
        number: 49,
    },
    Interrupt {
        name: "TIM7",
        number: 50,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 53,
    },
    Interrupt {
        name: "I3C1_ER",
        number: 54,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 55,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 56,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 57,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 58,
    },
    Interrupt {
        name: "SPI1",
        number: 59,
    },
    Interrupt {
        name: "SPI2",
        number: 60,
    },
    Interrupt {
        name: "USART1",
        number: 61,
    },
    Interrupt {
        name: "USART3",
        number: 63,
    },
    Interrupt {
        name: "UART4",
        number: 64,
    },
    Interrupt {
        name: "UART5",
        number: 65,
    },
    Interrupt {
        name: "LPUART1",
        number: 66,
    },
    Interrupt {
        name: "LPTIM1",
        number: 67,
    },
    Interrupt {
        name: "LPTIM2",
        number: 68,
    },
    Interrupt {
        name: "TIM15",
        number: 69,
    },
    Interrupt {
        name: "TIM16",
        number: 70,
    },
    Interrupt {
        name: "TIM17",
        number: 71,
    },
    Interrupt {
        name: "COMP",
        number: 72,
    },
    Interrupt {
        name: "USB_FS",
        number: 73,
    },
    Interrupt {
        name: "CRS",
        number: 74,
    },
    Interrupt {
        name: "OCTOSPI1",
        number: 76,
    },
    Interrupt {
        name: "SDMMC1",
        number: 78,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL8",
        number: 80,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL9",
        number: 81,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL10",
        number: 82,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL11",
        number: 83,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 88,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 89,
    },
    Interrupt {
        name: "SAI1",
        number: 90,
    },
    Interrupt {
        name: "TSC",
        number: 92,
    },
    Interrupt {
        name: "RNG",
        number: 94,
    },
    Interrupt {
        name: "FPU",
        number: 95,
    },
    Interrupt {
        name: "HASH",
        number: 96,
    },
    Interrupt {
        name: "LPTIM3",
        number: 98,
    },
    Interrupt {
        name: "SPI3",
        number: 99,
    },
    Interrupt {
        name: "I3C2_EV",
        number: 100,
    },
    Interrupt {
        name: "I3C2_ER",
        number: 101,
    },
    Interrupt {
        name: "ICACHE",
        number: 107,
    },
    Interrupt {
        name: "LPTIM4",
        number: 110,
    },
    Interrupt {
        name: "ADF1",
        number: 112,
    },
    Interrupt {
        name: "ADC2",
        number: 113,
    },
    Interrupt {
        name: "PWR",
        number: 123,
    },
    Interrupt {
        name: "PWR_S",
        number: 124,
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
    Pin { name: "PB12" },
    Pin { name: "PB13" },
    Pin { name: "PB14" },
    Pin { name: "PB15" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
    Pin { name: "PH3" },
];
#[path = "../registers/adc_u3.rs"]
pub mod adc;
#[path = "../registers/adccommon_u3.rs"]
pub mod adccommon;
#[path = "../registers/can_fdcan_v1.rs"]
pub mod can;
#[path = "../registers/comp_u3.rs"]
pub mod comp;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/crs_v1.rs"]
pub mod crs;
#[path = "../registers/dac_v6.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_u3.rs"]
pub mod dbgmcu;
#[path = "../registers/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../registers/exti_u3.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/flash_u3.rs"]
pub mod flash;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/hash_v3.rs"]
pub mod hash;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/i3c_v1.rs"]
pub mod i3c;
#[path = "../registers/icache_v1_3crr.rs"]
pub mod icache;
#[path = "../registers/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../registers/lptim_v2a.rs"]
pub mod lptim;
#[path = "../registers/opamp_v3.rs"]
pub mod opamp;
#[path = "../registers/pwr_u3.rs"]
pub mod pwr;
#[path = "../registers/ramcfg_u5.rs"]
pub mod ramcfg;
#[path = "../registers/rcc_u3.rs"]
pub mod rcc;
#[path = "../registers/rtc_v3_u3.rs"]
pub mod rtc;
#[path = "../registers/spi_v6.rs"]
pub mod spi;
#[path = "../registers/syscfg_u3.rs"]
pub mod syscfg;
#[path = "../registers/timer_v2.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/usb_v4.rs"]
pub mod usb;
#[path = "../registers/usbram_32_2048.rs"]
pub mod usbram;
#[path = "../registers/vrefbuf_v2a1.rs"]
pub mod vrefbuf;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
