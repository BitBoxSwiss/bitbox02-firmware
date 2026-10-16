
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "external interrupt configuration register 1",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exticr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some(
                        "Memory mapping selection bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "MemMode",
                    ),
                },
                Field {
                    name: "pa11_pa12_rmp",
                    description: Some(
                        "PA11 and PA12 remapping bit for small packages (28 and 20 pins)\n0: Pin pair PA9/PA10 mapped on the pins\n1: Pin pair PA11/PA12 mapped instead of PA9/PA10",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ir_mod",
                    description: Some(
                        "IR Modulation Envelope signal selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "IrMod",
                    ),
                },
                Field {
                    name: "adc_dma_rmp",
                    description: Some(
                        "ADC DMA remapping bit\n0: ADC DMA request mapped on DMA channel 1\n1: ADC DMA request mapped on DMA channel 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1_tx_dma_rmp",
                    description: Some(
                        "USART1_TX DMA remapping bit\n0: USART1_TX DMA request mapped on DMA channel 2\n1: USART1_TX DMA request mapped on DMA channel 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1_rx_dma_rmp",
                    description: Some(
                        "USART1_RX DMA request remapping bit\n0: USART1_RX DMA request mapped on DMA channel 3\n1: USART1_RX DMA request mapped on DMA channel 5",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16_dma_rmp",
                    description: Some(
                        "TIM16 DMA request remapping bit\n0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3\n1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17_dma_rmp",
                    description: Some(
                        "TIM17 DMA request remapping bit\n0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1\n1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16_dma_rmp2",
                    description: Some(
                        "TIM16 alternate DMA request remapping bit\n0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit\n1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17_dma_rmp2",
                    description: Some(
                        "TIM17 alternate DMA request remapping bit\n0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit\n1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pb6_fmp",
                    description: Some(
                        "Fast Mode Plus (FM plus) driving capability activation bits.\n0: PB6 pin operate in standard mode\n1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb7_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.\n0: PB7 pin operate in standard mode\n1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb8_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.\n0: PB8 pin operate in standard mode\n1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.\n0: PB9 pin operate in standard mode\n1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c1_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C1\n0: FM+ mode is controlled by I2C_Pxx_FMP bits only\n1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c2_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C2\n0: FM+ mode is controlled by I2C_Pxx_FMP bits only\n1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c_pa9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits\n0: PA9 pin operate in standard mode\n1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "i2c_pa10_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits\n0: PA10 pin operate in standard mode\n1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmp",
                    ),
                },
                Field {
                    name: "spi2_dma_rmp",
                    description: Some(
                        "SPI2 DMA request remapping bit\n0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively\n1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2_dma_rmp",
                    description: Some(
                        "USART2 DMA request remapping bit\n0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively\n1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3_dma_rmp",
                    description: Some(
                        "USART3 DMA request remapping bit\n0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)\n1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_dma_rmp",
                    description: Some(
                        "I2C1 DMA request remapping bit\n0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively\n1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_dma_rmp",
                    description: Some(
                        "TIM1 DMA request remapping bit\n0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively\n1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim2_dma_rmp",
                    description: Some(
                        "TIM2 DMA request remapping bit\n0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively\n1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3_dma_rmp",
                    description: Some(
                        "TIM3 DMA request remapping bit\n0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4\n1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex-M0 LOCKUP bit enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram_parity_lock",
                    description: Some(
                        "SRAM parity lock bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvd_lock",
                    description: Some(
                        "PVD lock enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram_pef",
                    description: Some(
                        "SRAM parity flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some(
                "external interrupt configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI configuration bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "Standard",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "FM+",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "IrMod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TIM16",
                    description: Some(
                        "TIM16 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USART1",
                    description: Some(
                        "USART1 selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USART4",
                    description: Some(
                        "USART4 selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "MemMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAIN_FLASH",
                    description: Some(
                        "Main Flash memory mapped at 0x0000_0000",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSTEM_FLASH",
                    description: Some(
                        "System Flash memory mapped at 0x0000_0000",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAIN_FLASH2",
                    description: Some(
                        "Main Flash memory mapped at 0x0000_0000",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "Embedded SRAM mapped at 0x0000_0000",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
