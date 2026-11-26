
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g4",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "ADC12SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "INN9",
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
        triggers: &[
            PeripheralTrigger {
                signal: "ADC_EXT_TRG0",
                source: "TIM1_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG1",
                source: "TIM1_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG2",
                source: "TIM1_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG3",
                source: "TIM2_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG4",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG5",
                source: "TIM4_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG6",
                source: "EXTI11_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG7",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG8",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG9",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG10",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG11",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG12",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG13",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG14",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG15",
                source: "TIM3_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG18",
                source: "TIM20_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG19",
                source: "TIM20_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG20",
                source: "TIM20_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG21",
                source: "HRTIM_ADC_TRG1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG22",
                source: "HRTIM_ADC_TRG3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG23",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG24",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG25",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG26",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG27",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG28",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG30",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG0",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG1",
                source: "TIM1_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG2",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG3",
                source: "TIM2_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG4",
                source: "TIM3_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG6",
                source: "EXTI15_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG7",
                source: "TIM8_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG8",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG9",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG10",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG11",
                source: "TIM3_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG12",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG13",
                source: "TIM3_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG14",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG15",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG18",
                source: "TIM20_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG19",
                source: "HRTIM_ADC_TRG2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG20",
                source: "HRTIM_ADC_TRG4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG21",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG22",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG23",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG24",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG25",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG26",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG27",
                source: "TIM16_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG30",
                source: "TIM7_TRGO",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x50000300,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "g4",
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
        address: 0x50000100,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g4",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "ADC12SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INN14",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "INN9",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(36),
        }],
        triggers: &[
            PeripheralTrigger {
                signal: "ADC_EXT_TRG0",
                source: "TIM1_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG1",
                source: "TIM1_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG2",
                source: "TIM1_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG3",
                source: "TIM2_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG4",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG5",
                source: "TIM4_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG6",
                source: "EXTI11_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG7",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG8",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG9",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG10",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG11",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG12",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG13",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG14",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG15",
                source: "TIM3_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG18",
                source: "TIM20_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG19",
                source: "TIM20_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG20",
                source: "TIM20_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG21",
                source: "HRTIM_ADC_TRG1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG22",
                source: "HRTIM_ADC_TRG3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG23",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG24",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG25",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG26",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG27",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG28",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG30",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG0",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG1",
                source: "TIM1_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG2",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG3",
                source: "TIM2_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG4",
                source: "TIM3_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG6",
                source: "EXTI15_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG7",
                source: "TIM8_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG8",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG9",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG10",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG11",
                source: "TIM3_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG12",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG13",
                source: "TIM3_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG14",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG15",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG18",
                source: "TIM20_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG19",
                source: "HRTIM_ADC_TRG2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG20",
                source: "HRTIM_ADC_TRG4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG21",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG22",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG23",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG24",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG25",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG26",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG27",
                source: "TIM16_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG30",
                source: "TIM7_TRGO",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0x50000400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g4",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "ADC345SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC345EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC345RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "INN4",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC3",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(37),
        }],
        triggers: &[
            PeripheralTrigger {
                signal: "ADC_EXT_TRG0",
                source: "TIM3_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG1",
                source: "TIM2_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG2",
                source: "TIM1_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG3",
                source: "TIM8_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG4",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG5",
                source: "EXTI2_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG6",
                source: "TIM4_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG7",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG8",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG9",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG10",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG11",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG12",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG13",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG14",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG15",
                source: "TIM2_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG18",
                source: "TIM20_CC1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG19",
                source: "HRTIM_ADC_TRG2",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG20",
                source: "HRTIM_ADC_TRG4",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG21",
                source: "HRTIM_ADC_TRG1",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG22",
                source: "HRTIM_ADC_TRG3",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG23",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG24",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG25",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG26",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG27",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG28",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_EXT_TRG30",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG0",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG1",
                source: "TIM1_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG2",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG3",
                source: "TIM8_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG4",
                source: "TIM4_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG6",
                source: "TIM4_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG7",
                source: "TIM8_CC4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG8",
                source: "TIM1_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG9",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG10",
                source: "TIM8_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG11",
                source: "TIM1_CC3",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG12",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG13",
                source: "EXTI3_TRG",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG14",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG15",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG16",
                source: "TIM20_TRGO",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG17",
                source: "TIM20_TRGO2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG18",
                source: "TIM20_CC2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG19",
                source: "HRTIM_ADC_TRG2",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG20",
                source: "HRTIM_ADC_TRG4",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG21",
                source: "HRTIM_ADC_TRG5",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG22",
                source: "HRTIM_ADC_TRG6",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG23",
                source: "HRTIM_ADC_TRG7",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG24",
                source: "HRTIM_ADC_TRG8",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG25",
                source: "HRTIM_ADC_TRG9",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG26",
                source: "HRTIM_ADC_TRG10",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG27",
                source: "HRTIM_ADC_TRG1",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG28",
                source: "HRTIM_ADC_TRG3",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG29",
                source: "LPTIMOUT",
            },
            PeripheralTrigger {
                signal: "ADC_JEXT_TRG30",
                source: "TIM7_TRGO",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC345_COMMON",
        address: 0x50000700,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "g4",
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
        address: 0x40010200,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "v2",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP2",
        address: 0x40010204,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "v2",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP3",
        address: 0x40010208,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "v2",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "OUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "INM0",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP4",
        address: 0x4001020c,
        registers: Some(PeripheralRegisters {
            kind: "comp",
            version: "v2",
            block: "COMP",
            ir: &comp::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP4",
        }],
        afio: None,
    },
    Peripheral {
        name: "CORDIC",
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "cordic",
            version: "v1",
            block: "CORDIC",
            ir: &cordic::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CORDICEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CORDICRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(113),
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
        address: 0x40002000,
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
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SYNC",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x50000800,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v7",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
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
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(7),
            },
        ],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG15",
                source: "HRTIM_DAC_TRG1",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM15_TRGO",
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
            PeripheralTrigger {
                signal: "DAC_CHX_TRG7",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG8",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG9",
                source: "HRTIM_DAC_RESET_TRG1",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG10",
                source: "HRTIM_DAC_RESET_TRG2",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG11",
                source: "HRTIM_DAC_RESET_TRG3",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG12",
                source: "HRTIM_DAC_RESET_TRG4",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG13",
                source: "HRTIM_DAC_RESET_TRG5",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG14",
                source: "HRTIM_DAC_RESET_TRG6",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG1",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG3",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG4",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG6",
                source: "EXTI10_TRG",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG7",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG8",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG9",
                source: "HRTIM_DAC_STEP_TRG1",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG10",
                source: "HRTIM_DAC_STEP_TRG2",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG11",
                source: "HRTIM_DAC_STEP_TRG3",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG12",
                source: "HRTIM_DAC_STEP_TRG4",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG13",
                source: "HRTIM_DAC_STEP_TRG5",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG14",
                source: "HRTIM_DAC_STEP_TRG6",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC3",
        address: 0x50001000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v7",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(103),
            },
        ],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG15",
                source: "HRTIM_DAC_TRG3",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM15_TRGO",
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
            PeripheralTrigger {
                signal: "DAC_CHX_TRG7",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG8",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG9",
                source: "HRTIM_DAC_RESET_TRG1",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG10",
                source: "HRTIM_DAC_RESET_TRG2",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG11",
                source: "HRTIM_DAC_RESET_TRG3",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG12",
                source: "HRTIM_DAC_RESET_TRG4",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG13",
                source: "HRTIM_DAC_RESET_TRG5",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG14",
                source: "HRTIM_DAC_RESET_TRG6",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG1",
                source: "TIM1_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG3",
                source: "TIM15_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG4",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG6",
                source: "EXTI10_TRG",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG7",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG8",
                source: "TIM3_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG9",
                source: "HRTIM_DAC_STEP_TRG1",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG10",
                source: "HRTIM_DAC_STEP_TRG2",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG11",
                source: "HRTIM_DAC_STEP_TRG3",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG12",
                source: "HRTIM_DAC_STEP_TRG4",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG13",
                source: "HRTIM_DAC_STEP_TRG5",
            },
            PeripheralTrigger {
                signal: "DAC_INC_CHX_TRG14",
                source: "HRTIM_DAC_STEP_TRG6",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0042000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "g4",
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
                register: "AHB1ENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMA1RST",
            }),
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
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA1_CHANNEL8",
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
                register: "AHB1ENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMA2RST",
            }),
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
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA2_CHANNEL8",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DMAMUX1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMAMUX1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "OVR",
            interrupt: "DMAMUX_OVR",
        }],
        afio: None,
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
        name: "FDCAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v1",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v1",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "FDCANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
        address: 0x4000a400,
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
        address: 0x4000a750,
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
            version: "g4c4",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FLASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "FLASHRST",
            }),
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
        name: "FMAC",
        address: 0x40021400,
        registers: Some(PeripheralRegisters {
            kind: "fmac",
            version: "v1",
            block: "FMAC",
            ir: &fmac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FMACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "FMACRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(110),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(111),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMAC",
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
        address: 0x48000400,
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
        address: 0x48000800,
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
        address: 0x48000c00,
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
        name: "GPIOE",
        address: 0x48001000,
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
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x48001400,
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
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x48001800,
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
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                register: "CCIPR",
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
                pin: "PA13",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
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
                register: "CCIPR",
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
                pin: "PA10",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(19),
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
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "I2C3SEL",
            }),
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
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SDA",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(21),
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
        name: "LPTIM1",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1b_g4",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
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
                pin: "PA14",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "LPUART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
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
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
        name: "OPAMP1",
        address: 0x40010300,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v5",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP_SEC",
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
        address: 0x40010304,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v5",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OPAMP3",
        address: 0x40010308,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v5",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP_SEC",
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
                pin: "PB0",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OPAMP6",
        address: 0x40010314,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v5",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        rcc: None,
        pins: &[
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
                pin: "PA1",
                signal: "VINM_SEC",
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
            PeripheralPin {
                pin: "PB1",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP_SEC",
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
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "g4",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
        name: "QUADSPI1",
        address: 0xa0001000,
        registers: Some(PeripheralRegisters {
            kind: "quadspi",
            version: "v1",
            block: "QUADSPI",
            ir: &quadspi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "QUADSPIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "QUADSPIRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "BK1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BK1_IO3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "BK1_IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "BK1_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "BK1_IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "BK1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "BK2_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "BK2_IO3",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QUADSPI",
        }],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "g4",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
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
                pin: "PF0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "MCO",
                af: Some(0),
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
            PeripheralInterrupt {
                signal: "LSECSS",
                interrupt: "RTC_TAMP_LSECSS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x50060800,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
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
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_base",
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
                pin: "PA0",
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REFIN",
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
                signal: "TAMP1",
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
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC_TAMP_LSECSS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4_4pdm",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
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
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SD_A",
                af: Some(14),
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
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FS_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MCLK_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SD_B",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FS_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MCLK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FS_A",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(109),
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
            version: "v3_i2s",
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
            version: "v3_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PA10",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_MCK",
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
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(13),
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
            version: "v3_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PA9",
                signal: "I2S_MCK",
                af: Some(5),
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
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(6),
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
                signal: "MISO",
                af: Some(6),
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
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "g4",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "g4",
            block: "TAMP",
            ir: &tamp::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
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
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BKIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BKIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3N",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CH3N",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(48),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(81),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM15",
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
                pin: "PA12",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(83),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(10),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(85),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM17",
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
                pin: "PA10",
                signal: "CH4",
                af: Some(10),
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
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH3",
                af: Some(10),
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
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
        name: "TIM20",
        address: 0x40015000,
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
                field: "TIM20EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM20RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(94),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM20_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM20_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM20_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM20_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM20_UP",
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
                pin: "PA4",
                signal: "CH2",
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
                pin: "PB3",
                signal: "ETR",
                af: Some(10),
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
                pin: "PB7",
                signal: "CH4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
                pin: "PA11",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(8),
        }],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC",
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
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(9),
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
        name: "TIM8",
        address: 0x40013400,
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
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH4N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
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
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(55),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "UCPD1",
        address: 0x4000a000,
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
                field: "UCPD1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "UCPD1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "DBCC2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DBCC1",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CC1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(115),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
        }],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0x1fff7590,
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
                register: "CCIPR",
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
                pin: "PC4",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
                register: "CCIPR",
                field: "USART2SEL",
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
                pin: "PA14",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
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
                pin: "PB3",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(27),
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
                register: "CCIPR",
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
                pin: "PA13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "NSS",
                af: Some(7),
            },
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
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(7),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
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
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v3",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "CLK48SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_HP",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_LP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBWAKEUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "USBRAM",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "16x2_1024",
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
        address: 0x40010030,
        registers: Some(PeripheralRegisters {
            kind: "vrefbuf",
            version: "v2b",
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
        name: "VREFINTCAL",
        address: 0x1fff75aa,
        registers: Some(PeripheralRegisters {
            kind: "vrefintcal",
            version: "v1",
            block: "VREFINTCAL",
            ir: &vrefintcal::REGISTERS,
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
    Interrupt {
        name: "RTC_TAMP_LSECSS",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 3,
    },
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
        name: "USB_HP",
        number: 19,
    },
    Interrupt {
        name: "USB_LP",
        number: 20,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 21,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM15",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM16",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM17",
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
        name: "USBWAKEUP",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "ADC3",
        number: 47,
    },
    Interrupt {
        name: "LPTIM1",
        number: 49,
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
        name: "TIM6_DAC",
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
        name: "UCPD1",
        number: 63,
    },
    Interrupt {
        name: "COMP1_2_3",
        number: 64,
    },
    Interrupt {
        name: "COMP4",
        number: 65,
    },
    Interrupt {
        name: "CRS",
        number: 75,
    },
    Interrupt {
        name: "SAI1",
        number: 76,
    },
    Interrupt {
        name: "TIM20_BRK",
        number: 77,
    },
    Interrupt {
        name: "TIM20_UP",
        number: 78,
    },
    Interrupt {
        name: "TIM20_TRG_COM",
        number: 79,
    },
    Interrupt {
        name: "TIM20_CC",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 86,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 87,
    },
    Interrupt {
        name: "RNG",
        number: 90,
    },
    Interrupt {
        name: "LPUART1",
        number: 91,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 92,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 93,
    },
    Interrupt {
        name: "DMAMUX_OVR",
        number: 94,
    },
    Interrupt {
        name: "QUADSPI",
        number: 95,
    },
    Interrupt {
        name: "DMA1_CHANNEL8",
        number: 96,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 97,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 98,
    },
    Interrupt {
        name: "DMA2_CHANNEL8",
        number: 99,
    },
    Interrupt {
        name: "CORDIC",
        number: 100,
    },
    Interrupt {
        name: "FMAC",
        number: 101,
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
    DmaChannel {
        name: "DMA1_CH8",
        dma: "DMA1",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(12),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(13),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(14),
    },
    DmaChannel {
        name: "DMA2_CH8",
        dma: "DMA2",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(15),
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
    Pin { name: "PC4" },
    Pin { name: "PC6" },
    Pin { name: "PC10" },
    Pin { name: "PC11" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PF0" },
    Pin { name: "PF1" },
    Pin { name: "PG10" },
];
#[path = "../registers/adc_g4.rs"]
pub mod adc;
#[path = "../registers/adccommon_g4.rs"]
pub mod adccommon;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/can_fdcan_v1.rs"]
pub mod can;
#[path = "../registers/comp_v2.rs"]
pub mod comp;
#[path = "../registers/cordic_v1.rs"]
pub mod cordic;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/crs_v1.rs"]
pub mod crs;
#[path = "../registers/dac_v7.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_g4.rs"]
pub mod dbgmcu;
#[path = "../registers/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/flash_g4c4.rs"]
pub mod flash;
#[path = "../registers/fmac_v1.rs"]
pub mod fmac;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/lptim_v1b_g4.rs"]
pub mod lptim;
#[path = "../registers/opamp_v5.rs"]
pub mod opamp;
#[path = "../registers/pwr_g4.rs"]
pub mod pwr;
#[path = "../registers/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../registers/rcc_g4.rs"]
pub mod rcc;
#[path = "../registers/rng_v1.rs"]
pub mod rng;
#[path = "../registers/rtc_v3_base.rs"]
pub mod rtc;
#[path = "../registers/sai_v4_4pdm.rs"]
pub mod sai;
#[path = "../registers/spi_v3_i2s.rs"]
pub mod spi;
#[path = "../registers/syscfg_g4.rs"]
pub mod syscfg;
#[path = "../registers/tamp_g4.rs"]
pub mod tamp;
#[path = "../registers/timer_v2.rs"]
pub mod timer;
#[path = "../registers/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/usb_v3.rs"]
pub mod usb;
#[path = "../registers/usbram_16x2_1024.rs"]
pub mod usbram;
#[path = "../registers/vrefbuf_v2b.rs"]
pub mod vrefbuf;
#[path = "../registers/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
