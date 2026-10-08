
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x42028000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "h5",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC1RST",
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
                pin: "PA5",
                signal: "INN18",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INP19",
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
                dma: Some("GPDMA2"),
                request: Some(0),
            },
        ],
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
            version: "h50",
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
        name: "COMP1",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "h5",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "COMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB15",
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
            interrupt: "COMP1",
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
                register: "AHB1ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
                register: "APB1LENR",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                register: "CCIPR5",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(3),
            },
        ],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM2_TRGO",
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
                signal: "DAC_CHX_TRG11",
                source: "LPTIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG12",
                source: "LPTIM2_TRGO",
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
        address: 0x44024000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "h5",
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
        address: 0x4600f000,
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
        address: 0x46008400,
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
        name: "DTS",
        address: 0x40008c00,
        registers: Some(PeripheralRegisters {
            kind: "dts",
            version: "v1",
            block: "DTS",
            ir: &dts::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "DTSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "DTSRST",
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
        name: "EXTI",
        address: 0x44022000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "h50",
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
                register: "CCIPR5",
                field: "FDCAN12SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "FDCAN12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "FDCAN12RST",
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
                pin: "PA8",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
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
                pin: "PB15",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
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
            PeripheralPin {
                pin: "PB7",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
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
            version: "h50",
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
        ],
        afio: None,
    },
    Peripheral {
        name: "GPDMA2",
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
                field: "GPDMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA2_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA2_CHANNEL7",
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
                register: "AHB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                register: "AHB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                register: "AHB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                register: "AHB2ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                register: "AHB2ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "h503",
            block: "GTZC1",
            ir: &gtzc::REGISTERS,
        }),
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
                register: "AHB2ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "HASHRST",
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
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(111),
            },
        ],
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
                register: "CCIPR4",
                field: "I2C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDA",
                af: Some(11),
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
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(13),
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
                register: "CCIPR4",
                field: "I2C2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SMBA",
                af: Some(11),
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
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(16),
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
                register: "CCIPR4",
                field: "I3C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I3C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I3C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDA",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
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
                request: Some(120),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(120),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(121),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(121),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(123),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(123),
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
        address: 0x44003000,
        registers: Some(PeripheralRegisters {
            kind: "i3c",
            version: "v1",
            block: "I3C",
            ir: &i3c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I3C2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I3C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "I3C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SDA",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SDA",
                af: Some(10),
            },
        ],
        dma_channels: &[],
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
            version: "v1_0crr",
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
        address: 0x44004400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
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
                pin: "PA0",
                signal: "ETR",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "ETR",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2",
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
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "IC2",
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
                dma: Some("GPDMA2"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "UE",
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
                dma: Some("GPDMA2"),
                request: Some(104),
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
                register: "CCIPR2",
                field: "LPTIM2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "IN2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN1",
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
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(14),
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
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(107),
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
        name: "LPUART1",
        address: 0x44002400,
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
                pin: "PA13",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
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
                signal: "RX",
                af: Some(8),
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
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(46),
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
        name: "OPAMP1",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v4",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "OPAMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM1",
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
        address: 0x44020800,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "h50",
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
                pin: "PA7",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP4",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "ramcfg",
            version: "h5",
            block: "RAMCFG",
            ir: &ramcfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "RAMCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
        address: 0x44020c00,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "h50",
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
                pin: "PB2",
                signal: "LSCO",
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
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v3",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "RNGSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x44007800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_u5",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "BDCR",
                field: "RTCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "RTCAPBEN",
            }),
            reset: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "OUT2",
                af: Some(0),
            },
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
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
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
                pin: "PA0",
                signal: "I2S_SDI",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "MISO",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_WS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "NSS",
                af: Some(4),
            },
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
                pin: "PA2",
                signal: "I2S_CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_SDI",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_SDO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: Some(4),
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
                signal: "I2S_MCK",
                af: Some(6),
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
                signal: "I2S_MCK",
                af: Some(4),
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
                pin: "PA8",
                signal: "I2S_CK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_SDI",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RDY",
                af: Some(4),
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
                pin: "PB6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_WS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "NSS",
                af: Some(12),
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
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "SPI2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_MCK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RDY",
                af: Some(11),
            },
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
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_SDI",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MISO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDI",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MISO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_WS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(11),
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
                pin: "PB1",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MOSI",
                af: Some(6),
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
                pin: "PB2",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_MCK",
                af: Some(7),
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
                pin: "PB5",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RDY",
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
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
                register: "CCIPR3",
                field: "SPI3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_WS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_CK",
                af: Some(10),
            },
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
                pin: "PA15",
                signal: "SCK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_SDI",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MISO",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_SDO",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MOSI",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_MCK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_SDO",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MOSI",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_MCK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MISO",
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
                pin: "PB6",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_MCK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SCK",
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
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
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
        address: 0x44000400,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "h50",
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
        address: 0x44007c00,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "h5",
            block: "TAMP",
            ir: &tamp::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT1",
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
                pin: "PA1",
                signal: "CH3",
                af: Some(14),
            },
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
                pin: "PA13",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH4N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2N",
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
                pin: "PA8",
                signal: "CH4N",
                af: Some(14),
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
                pin: "PB0",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(14),
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
                pin: "PB2",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BKIN2",
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
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(64),
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
                register: "APB1LENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                pin: "PA12",
                signal: "CH4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(14),
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
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(14),
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
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(76),
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
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM3RST",
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
                pin: "PA11",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                af: Some(2),
            },
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
                pin: "PA8",
                signal: "CH3",
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
                pin: "PB15",
                signal: "CH4",
                af: Some(14),
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
                pin: "PB6",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "ETR",
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
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(82),
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
                register: "APB1LENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(4),
            },
        ],
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
                register: "APB1LENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(5),
            },
        ],
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
                pin: "PA0",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(8),
            },
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
                pin: "PA11",
                signal: "RX",
                af: Some(8),
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
                pin: "PA12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RTS",
                af: Some(8),
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
                pin: "PB12",
                signal: "CK",
                af: Some(8),
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
                pin: "PB8",
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
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(22),
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
                register: "CCIPR1",
                field: "USART2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                signal: "CK",
                af: Some(9),
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
                pin: "PA11",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(9),
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
                pin: "PA5",
                signal: "CTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NSS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "NSS",
                af: Some(11),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "TX",
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
                dma: Some("GPDMA2"),
                request: Some(24),
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
                register: "CCIPR1",
                field: "USART3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RTS",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RTS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CK",
                af: Some(13),
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
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(26),
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
                register: "CCIPR4",
                field: "USBSEL",
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
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
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
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB_DRD_FS",
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
                register: "APB1LENR",
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
        name: "PVD_AVD",
        number: 1,
    },
    Interrupt { name: "RTC", number: 2 },
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
    Interrupt { name: "RCC", number: 9 },
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
        name: "GPDMA1_CHANNEL0",
        number: 27,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 28,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 29,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 30,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 31,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 32,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 33,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 34,
    },
    Interrupt {
        name: "IWDG",
        number: 35,
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
        name: "TIM6",
        number: 49,
    },
    Interrupt {
        name: "TIM7",
        number: 50,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 51,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 52,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 53,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 54,
    },
    Interrupt {
        name: "SPI1",
        number: 55,
    },
    Interrupt {
        name: "SPI2",
        number: 56,
    },
    Interrupt {
        name: "SPI3",
        number: 57,
    },
    Interrupt {
        name: "USART1",
        number: 58,
    },
    Interrupt {
        name: "USART2",
        number: 59,
    },
    Interrupt {
        name: "USART3",
        number: 60,
    },
    Interrupt {
        name: "LPUART1",
        number: 63,
    },
    Interrupt {
        name: "LPTIM1",
        number: 64,
    },
    Interrupt {
        name: "LPTIM2",
        number: 70,
    },
    Interrupt {
        name: "USB_DRD_FS",
        number: 74,
    },
    Interrupt {
        name: "CRS",
        number: 75,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL0",
        number: 90,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL1",
        number: 91,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL2",
        number: 92,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL3",
        number: 93,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL4",
        number: 94,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL5",
        number: 95,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL6",
        number: 96,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL7",
        number: 97,
    },
    Interrupt {
        name: "FPU",
        number: 103,
    },
    Interrupt {
        name: "ICACHE",
        number: 104,
    },
    Interrupt {
        name: "DTS",
        number: 113,
    },
    Interrupt {
        name: "RNG",
        number: 114,
    },
    Interrupt {
        name: "HASH",
        number: 117,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 123,
    },
    Interrupt {
        name: "I3C1_ER",
        number: 124,
    },
    Interrupt {
        name: "I3C2_EV",
        number: 131,
    },
    Interrupt {
        name: "I3C2_ER",
        number: 132,
    },
    Interrupt {
        name: "COMP1",
        number: 133,
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
        name: "GPDMA2_CH0",
        dma: "GPDMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH1",
        dma: "GPDMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH2",
        dma: "GPDMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH3",
        dma: "GPDMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH4",
        dma: "GPDMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH5",
        dma: "GPDMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH6",
        dma: "GPDMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH7",
        dma: "GPDMA2",
        channel: 7,
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
];
#[path = "../registers/adc_h5.rs"]
pub mod adc;
#[path = "../registers/adccommon_h50.rs"]
pub mod adccommon;
#[path = "../registers/can_fdcan_v1.rs"]
pub mod can;
#[path = "../registers/comp_h5.rs"]
pub mod comp;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/crs_v1.rs"]
pub mod crs;
#[path = "../registers/dac_v6.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_h5.rs"]
pub mod dbgmcu;
#[path = "../registers/dlyb_v1.rs"]
pub mod dlyb;
#[path = "../registers/dts_v1.rs"]
pub mod dts;
#[path = "../registers/exti_h50.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/flash_h50.rs"]
pub mod flash;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/gtzc_h503.rs"]
pub mod gtzc;
#[path = "../registers/hash_v3.rs"]
pub mod hash;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/i3c_v1.rs"]
pub mod i3c;
#[path = "../registers/icache_v1_0crr.rs"]
pub mod icache;
#[path = "../registers/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../registers/lptim_v2a.rs"]
pub mod lptim;
#[path = "../registers/opamp_v4.rs"]
pub mod opamp;
#[path = "../registers/pwr_h50.rs"]
pub mod pwr;
#[path = "../registers/ramcfg_h5.rs"]
pub mod ramcfg;
#[path = "../registers/rcc_h50.rs"]
pub mod rcc;
#[path = "../registers/rng_v3.rs"]
pub mod rng;
#[path = "../registers/rtc_v3_u5.rs"]
pub mod rtc;
#[path = "../registers/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../registers/syscfg_h50.rs"]
pub mod syscfg;
#[path = "../registers/tamp_h5.rs"]
pub mod tamp;
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
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
