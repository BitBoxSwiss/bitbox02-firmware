
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RCC source control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hsicfgr",
                    description: Some(
                        "RCC HSI calibration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hsicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crrcr",
                    description: Some(
                        "RCC clock recovery RC register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csicfgr",
                    description: Some(
                        "RCC CSI calibration register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "RCC clock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdcfgr",
                    description: Some(
                        "RCC CPU domain clock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmcfgr",
                    description: Some(
                        "RCC AHB clock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbcfgr",
                    description: Some(
                        "RCC APB clocks configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllckselr",
                    description: Some(
                        "RCC PLLs clock source selection register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "RCC PLLs configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plldivr",
                    description: Some(
                        "RCC PLL dividers configuration register 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plldivr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllfracr",
                    description: Some(
                        "RCC PLL fractional divider register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllfracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbperckselr",
                    description: Some(
                        "RCC AHB peripheral kernel clock selection register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahbperckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1perckselr",
                    description: Some(
                        "RCC APB1 peripherals kernel clock selection register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1perckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2perckselr",
                    description: Some(
                        "RCC APB2 peripherals kernel clock selection register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2perckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb45perckselr",
                    description: Some(
                        "RCC APB4,5 peripherals kernel clock selection register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb45perckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock source interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cifr",
                    description: Some(
                        "RCC clock source interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cicr",
                    description: Some(
                        "RCC clock source interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RCC Backup domain control register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr",
                    description: Some(
                        "RCC clock control and status register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb5rstr",
                    description: Some(
                        "RCC AHB5 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr",
                    description: Some(
                        "RCC AHB2 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb4rstr",
                    description: Some(
                        "RCC AHB4 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb5rstr",
                    description: Some(
                        "RCC APB5 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb5rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr1",
                    description: Some(
                        "RCC APB1 peripheral reset register 1.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr2",
                    description: Some(
                        "RCC APB1 peripheral reset register 2.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "RCC APB2 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb4rstr",
                    description: Some(
                        "RCC APB4 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3rstr",
                    description: Some(
                        "RCC AHB3 peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckgdisr",
                    description: Some(
                        "RCC AXI clocks gating disable register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckgdisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plldivr2",
                    description: Some(
                        "RCC PLL dividers configuration register 2.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plldivr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllsscgr",
                    description: Some(
                        "RCC PLL Spread Spectrum Clock Generator register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllsscgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckprotr",
                    description: Some(
                        "RCC clock protection register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckprotr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsr",
                    description: Some(
                        "RCC Reset status register.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb5enr",
                    description: Some(
                        "RCC AHB5 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr",
                    description: Some(
                        "RCC AHB2 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb4enr",
                    description: Some(
                        "RCC AHB4 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb5enr",
                    description: Some(
                        "RCC APB5 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb5enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr1",
                    description: Some(
                        "RCC APB1 clock enable register 1.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr2",
                    description: Some(
                        "RCC APB1 clock enable register 2.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "RCC APB2 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb4enr",
                    description: Some(
                        "RCC APB4 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3enr",
                    description: Some(
                        "RCC AHB3 clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb5lpenr",
                    description: Some(
                        "RCC AHB5 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1lpenr",
                    description: Some(
                        "RCC AHB1 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2lpenr",
                    description: Some(
                        "RCC AHB2 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb4lpenr",
                    description: Some(
                        "RCC AHB4 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3lpenr",
                    description: Some(
                        "RCC AHB3 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lpenr1",
                    description: Some(
                        "RCC APB1 low-power clock enable register 1.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lpenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lpenr2",
                    description: Some(
                        "RCC APB1 low-power clock enable register 2.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lpenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "RCC APB2 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb4lpenr",
                    description: Some(
                        "RCC APB4 low-power clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb5lpenr",
                    description: Some(
                        "RCC APB5 sleep clock register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb5lpenr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "RCC AHB1 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 clock enable Set and reset by software.",
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
                    name: "adc12en",
                    description: Some(
                        "ADC1 and 2 peripheral clocks enable Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the hclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethen",
                    description: Some(
                        "ETH1 MAC peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethtxen",
                    description: Some(
                        "ETH1 transmission clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrxen",
                    description: Some(
                        "ETH1 reception clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsen",
                    description: Some(
                        "OTGHS clocks enable Set and reset by software.",
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
                    name: "usbphycen",
                    description: Some(
                        "USBPHYC clocks enable Set and reset by software.",
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
                    name: "usb_otg_fsen",
                    description: Some(
                        "OTGFS peripheral clocks enable Set and reset by software.",
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
                    name: "adfen",
                    description: Some(
                        "ADF clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "RCC AHB1 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1lpen",
                    description: Some(
                        "GPDMA1 clock enable in low-power mode Set and reset by software.",
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
                    name: "adc12lpen",
                    description: Some(
                        "ADC1 and 2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the ADC1 and 2 are the kernel clock selected by ADCSEL and provided to ADCx_CK input, and the rcc_hclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethlpen",
                    description: Some(
                        "ETH1 MAC peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethtxlpen",
                    description: Some(
                        "ETH1 transmission peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrxlpen",
                    description: Some(
                        "ETH1 reception peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbpdctrl",
                    description: Some(
                        "USBPHYC common block power-down control Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usbpdctrl",
                    ),
                },
                Field {
                    name: "usb_otg_hslpen",
                    description: Some(
                        "OTGHS peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "usbphyclpen",
                    description: Some(
                        "USBPHYC peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "usb_otg_fslpen",
                    description: Some(
                        "OTGFS clock enable in low-power mode Set and reset by software.",
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
                    name: "adflpen",
                    description: Some(
                        "ADF clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "RCC AHB1 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 blocks reset Set and reset by software.",
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
                    name: "adc12rst",
                    description: Some(
                        "ADC1 and 2 blocks reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrst",
                    description: Some(
                        "ETH1 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsrst",
                    description: Some(
                        "OTGHS block reset Set and reset by software.",
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
                    name: "usbphycrst",
                    description: Some(
                        "USBPHYC block reset Set and reset by software.",
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
                    name: "usb_otg_fsrst",
                    description: Some(
                        "OTGFS block reset Set and reset by software.",
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
                    name: "adfrst",
                    description: Some(
                        "ADF block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pssien",
                    description: Some(
                        "PSSI peripheral clocks enable Set and reset by software.",
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
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay clock enable Set and reset by software.",
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
                    name: "cordicen",
                    description: Some(
                        "CORDIC clock enable Set and reset by software.",
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
                    name: "sram1en",
                    description: Some(
                        "SRAM1 clock enable Set and reset by software.",
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
                    name: "sram2en",
                    description: Some(
                        "SRAM2 clock enable Set and reset by software.",
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
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "RCC AHB2 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pssilpen",
                    description: Some(
                        "PSSI peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "sdmmc2lpen",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay clock enable in low-power mode Set and reset by software.",
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
                    name: "cordiclpen",
                    description: Some(
                        "CORDIC clock enable in low-power mode Set and reset by software.",
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
                    name: "sram1lpen",
                    description: Some(
                        "SRAM1 clock enable in low-power mode Set and reset by software.",
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
                    name: "sram2lpen",
                    description: Some(
                        "SRAM2 clock enable in low-power mode Set and reset by software.",
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
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pssirst",
                    description: Some(
                        "PSSI block reset Set and reset by software.",
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
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay blocks reset Set and reset by software.",
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
                    name: "cordicrst",
                    description: Some(
                        "CORDIC reset Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Ahb3enr",
            extends: None,
            description: Some(
                "RCC AHB3 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rngen",
                    description: Some(
                        "RNG peripheral clocks enable Set and reset by software.",
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
                    name: "hashen",
                    description: Some(
                        "HASH peripheral clock enable Set and reset by software.",
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
                    name: "crypen",
                    description: Some(
                        "CRYP peripheral clock enable Set and reset by software.",
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
                    name: "saesen",
                    description: Some(
                        "SAES peripheral clock enable Set and reset by software. This bit controls the enable of the clock delivered to the SAES.",
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
                    name: "pkaen",
                    description: Some(
                        "PKA peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3lpenr",
            extends: None,
            description: Some(
                "RCC AHB3 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rnglpen",
                    description: Some(
                        "RNG peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "hashlpen",
                    description: Some(
                        "HASH peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "cryplpen",
                    description: Some(
                        "CRYP peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "saeslpen",
                    description: Some(
                        "SAES peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "pkalpen",
                    description: Some(
                        "PKA peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3rstr",
            extends: None,
            description: Some(
                "RCC AHB3 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rngrst",
                    description: Some(
                        "random number generator block reset Set and reset by software.",
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
                    name: "hashrst",
                    description: Some(
                        "HASH block reset Set and reset by software.",
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
                    name: "cryprst",
                    description: Some(
                        "CRYP block reset Set and reset by software.",
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
                    name: "saesrst",
                    description: Some(
                        "SAES block reset Set and reset by software.",
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
                    name: "pkarst",
                    description: Some(
                        "PKA block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb4enr",
            extends: None,
            description: Some(
                "RCC AHB4 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "GPIOA peripheral clock enable Set and reset by software.",
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
                    name: "gpioben",
                    description: Some(
                        "GPIOB peripheral clock enable Set and reset by software.",
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
                    name: "gpiocen",
                    description: Some(
                        "GPIOC peripheral clock enable Set and reset by software.",
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
                    name: "gpioden",
                    description: Some(
                        "GPIOD peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioeen",
                    description: Some(
                        "GPIOE peripheral clock enable Set and reset by software.",
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
                    name: "gpiofen",
                    description: Some(
                        "GPIOF peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogen",
                    description: Some(
                        "GPIOG peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "GPIOH peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiomen",
                    description: Some(
                        "GPIOM peripheral clock enable Set and reset by software.",
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
                    name: "gpionen",
                    description: Some(
                        "GPION peripheral clock enable Set and reset by software.",
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
                    name: "gpiooen",
                    description: Some(
                        "GPIOO peripheral clock enable Set and reset by software.",
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
                    name: "gpiopen",
                    description: Some(
                        "GPIOP peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramen",
                    description: Some(
                        "Backup RAM clock enable Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Ahb4lpenr",
            extends: None,
            description: Some(
                "RCC AHB4 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "GPIOA peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpioblpen",
                    description: Some(
                        "GPIOB peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpioclpen",
                    description: Some(
                        "GPIOC peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpiodlpen",
                    description: Some(
                        "GPIOD peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioelpen",
                    description: Some(
                        "GPIOE peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpioflpen",
                    description: Some(
                        "GPIOF peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioglpen",
                    description: Some(
                        "GPIOG peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohlpen",
                    description: Some(
                        "GPIOH peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiomlpen",
                    description: Some(
                        "GPIOM peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpionlpen",
                    description: Some(
                        "GPION peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpioolpen",
                    description: Some(
                        "GPIOO peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gpioplpen",
                    description: Some(
                        "GPIOP peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramlpen",
                    description: Some(
                        "Backup RAM clock enable in low-power mode Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Ahb4rstr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "GPIOA block reset Set and reset by software.",
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
                    name: "gpiobrst",
                    description: Some(
                        "GPIOB block reset Set and reset by software.",
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
                    name: "gpiocrst",
                    description: Some(
                        "GPIOC block reset Set and reset by software.",
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
                    name: "gpiodrst",
                    description: Some(
                        "GPIOD block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioerst",
                    description: Some(
                        "GPIOE block reset Set and reset by software.",
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
                    name: "gpiofrst",
                    description: Some(
                        "GPIOF block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogrst",
                    description: Some(
                        "GPIOG block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "GPIOH block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiomrst",
                    description: Some(
                        "GPIOM block reset Set and reset by software.",
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
                    name: "gpionrst",
                    description: Some(
                        "GPION block reset Set and reset by software.",
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
                    name: "gpioorst",
                    description: Some(
                        "GPIOO block reset Set and reset by software.",
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
                    name: "gpioprst",
                    description: Some(
                        "GPIOP block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb5enr",
            extends: None,
            description: Some(
                "RCC AHB5 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdma1en",
                    description: Some(
                        "HPDMA1 peripheral clock enable Set and reset by software.",
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
                    name: "dma2den",
                    description: Some(
                        "DMA2D peripheral clock enable Set and reset by software.",
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
                    name: "jpegen",
                    description: Some(
                        "JPEG peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmcen",
                    description: Some(
                        "FMC and MCE3 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.",
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
                    name: "xspi1en",
                    description: Some(
                        "XSPI1 and MCE1 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 and DB_SDMMC1 peripheral clocks enable Set and reset by software.",
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
                    name: "xspi2en",
                    description: Some(
                        "XSPI2 and MCE2 peripheral clocks enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
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
                    name: "iomngren",
                    description: Some(
                        "XSPIM peripheral clock enable Set and reset by software.",
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
                    name: "gfxmmuen",
                    description: Some(
                        "GFXMMU peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpuen",
                    description: Some(
                        "GPU peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb5lpenr",
            extends: None,
            description: Some(
                "RCC AHB5 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdma1lpen",
                    description: Some(
                        "HPDMA1 low-power peripheral clock enable Set and reset by software.",
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
                    name: "dma2dlpen",
                    description: Some(
                        "DMA2D low-power peripheral clock enable Set and reset by software.",
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
                    name: "flitflpen",
                    description: Some(
                        "FLITF low-power peripheral clock enable Set and reset by software.",
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
                    name: "jpeglpen",
                    description: Some(
                        "JPEG clock enable during Sleep mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmclpen",
                    description: Some(
                        "FMC and MCE3 peripheral clocks enable during Sleep mode Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL, and the hclk5 bus interface clock.",
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
                    name: "xspi1lpen",
                    description: Some(
                        "XSPI1 and MCE1 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1lpen",
                    description: Some(
                        "SDMMC1 and SDMMC1 delay low-power peripheral clock enable Set and reset by software.",
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
                    name: "xspi2lpen",
                    description: Some(
                        "XSPI2 and MCE2 low-power peripheral clock enable Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
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
                    name: "xspimlpen",
                    description: Some(
                        "XSPIM low-power peripheral clock enable Set and reset by software.",
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
                    name: "gfxmmulpen",
                    description: Some(
                        "GFXMMU low-power peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpulpen",
                    description: Some(
                        "GPU low-power peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcm1lpen",
                    description: Some(
                        "DTCM1 low-power peripheral clock enable Set and reset by software.",
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
                    name: "dtcm2lpen",
                    description: Some(
                        "DTCM2 low-power peripheral clock enable Set and reset by software.",
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
                    name: "itcmlpen",
                    description: Some(
                        "ITCM low-power peripheral clock enable Set and reset by software.",
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
                Field {
                    name: "axisramlpen",
                    description: Some(
                        "AXISRAM[4:1] low-power peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb5rstr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdma1rst",
                    description: Some(
                        "HPDMA1 block reset Set and reset by software.",
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
                    name: "dma2drst",
                    description: Some(
                        "DMA2D block reset Set and reset by software.",
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
                    name: "jpegrst",
                    description: Some(
                        "JPEG block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmcrst",
                    description: Some(
                        "FMC and MCE3 blocks reset Set and reset by software. The hardware prevents writing this bit if FMCCKP = 1.",
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
                    name: "xspi1rst",
                    description: Some(
                        "XSPI1 and MCE1 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 and DB_SDMMC1 blocks reset Set and reset by software.",
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
                    name: "xspi2rst",
                    description: Some(
                        "XSPI2 and MCE2 blocks reset Set and reset by software. The hardware prevents writing this bit if XSPICKP = 1.",
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
                    name: "iomngrrst",
                    description: Some(
                        "XSPIM reset Set and reset by software.",
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
                    name: "gfxmmurst",
                    description: Some(
                        "GFXMMU block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpurst",
                    description: Some(
                        "GPU block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahbperckselr",
            extends: None,
            description: Some(
                "RCC AHB peripheral kernel clock selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcsel",
                    description: Some(
                        "FMC kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fmcsel",
                    ),
                },
                Field {
                    name: "sdmmcsel",
                    description: Some(
                        "SDMMC1 and SDMMC2 kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "xspi1sel",
                    description: Some(
                        "XSPI1 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Xspisel",
                    ),
                },
                Field {
                    name: "xspi2sel",
                    description: Some(
                        "XSPI2 kernel clock source selection Set and reset by software. 1x: pll2_t_ck selected as kernel peripheral clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Xspisel",
                    ),
                },
                Field {
                    name: "usbrefcksel",
                    description: Some(
                        "USBPHYC kernel clock frequency selection Set and reset by software. This field is used to indicate to the USBPHYC, the frequency of the reference kernel clock provided to the USBPHYC. others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Usbrefcksel",
                    ),
                },
                Field {
                    name: "usbphycsel",
                    description: Some(
                        "USBPHYC kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbphycsel",
                    ),
                },
                Field {
                    name: "usb_otg_fssel",
                    description: Some(
                        "OTGFS kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "UsbOtgFssel",
                    ),
                },
                Field {
                    name: "eth_ref_clk_sel",
                    description: Some(
                        "Ethernet reference clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EthRefClkSel",
                    ),
                },
                Field {
                    name: "ethphy_clk_sel",
                    description: Some(
                        "Clock source selection for external Ethernet PHY Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "EthphyClkSel",
                    ),
                },
                Field {
                    name: "adfsel",
                    description: Some(
                        "ADF kernel clock source selection Set and reset by software. Note: I2S_CKIN is an external clock taken from a pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adfsel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
                Field {
                    name: "pssisel",
                    description: Some(
                        "PSSI kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pssisel",
                    ),
                },
                Field {
                    name: "persel",
                    description: Some(
                        "per_ck clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Persel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "RCC APB1 clock enable register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 peripheral clock enable Set and reset by software.",
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
                    name: "tim3en",
                    description: Some(
                        "TIM3 peripheral clock enable Set and reset by software.",
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
                    name: "tim4en",
                    description: Some(
                        "TIM4 peripheral clock enable Set and reset by software.",
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
                    name: "tim5en",
                    description: Some(
                        "TIM5 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6en",
                    description: Some(
                        "TIM6 peripheral clock enable Set and reset by software.",
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
                    name: "tim7en",
                    description: Some(
                        "TIM7 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12en",
                    description: Some(
                        "TIM12 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13en",
                    description: Some(
                        "TIM13 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14en",
                    description: Some(
                        "TIM14 peripheral clock enable Set and reset by software.",
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
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPTIM1 are the kernel clock selected by LPTIM1SEL and provided to clk_lpt input, and the rcc_pclk1 bus interface clock.",
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
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable Set by software, and reset by hardware when a system reset occurs.",
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
                    name: "spi2en",
                    description: Some(
                        "SPI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI2 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock.",
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
                    name: "spi3en",
                    description: Some(
                        "SPI3 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI3 are the kernel clock selected by I2S123SRC and provided to com_clk input, and the rcc_pclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxen",
                    description: Some(
                        "SPDIFRX peripheral clocks enable Set and reset by software. The peripheral clocks of the SPDIFRX are the kernel clock selected by SPDIFRXSEL and provided to SPDIFRX_CLK input, and the rcc_pclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART2peripheral clocks enable Set and reset by software. The peripheral clocks of the USART2 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART3 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART3 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4en",
                    description: Some(
                        "UART4 peripheral clocks enable Set and reset by software. The peripheral clocks of the UART4 are the kernel clock selected by USART234578SEL and provided to UCLK input, and the rcc_pclk1 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5en",
                    description: Some(
                        "UART5 peripheral clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_i3c1en",
                    description: Some(
                        "I2C1/I3C1 peripheral clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C2 peripheral clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3en",
                    description: Some(
                        "I2C3 peripheral clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cecen",
                    description: Some(
                        "HDMI-CEC peripheral clock enable Set and reset by software.",
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
                    name: "uart7en",
                    description: Some(
                        "UART7 peripheral clocks enable Set and reset by software.",
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
                Field {
                    name: "uart8en",
                    description: Some(
                        "UART8 peripheral clocks enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "RCC APB1 clock enable register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crsen",
                    description: Some(
                        "clock recovery system peripheral clock enable Set and reset by software.",
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
                    name: "mdiosen",
                    description: Some(
                        "MDIOS peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcanen",
                    description: Some(
                        "FDCAN peripheral clock enable Set and reset by software.",
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
                    name: "ucpden",
                    description: Some(
                        "UCPD peripheral clock enable Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb1lpenr1",
            extends: None,
            description: Some(
                "RCC APB1 low-power clock enable register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "tim3lpen",
                    description: Some(
                        "TIM3 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "tim4lpen",
                    description: Some(
                        "TIM4 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "tim5lpen",
                    description: Some(
                        "TIM5 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6lpen",
                    description: Some(
                        "TIM6 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "tim7lpen",
                    description: Some(
                        "TIM7 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12lpen",
                    description: Some(
                        "TIM12 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13lpen",
                    description: Some(
                        "TIM13 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14lpen",
                    description: Some(
                        "TIM14 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "lptim1lpen",
                    description: Some(
                        "LPTIM1 peripheral clocks enable in low-power mode Set and reset by software.",
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
                    name: "wwdglpen",
                    description: Some(
                        "WWDG clock enable in low-power mode Set and reset by software.",
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
                    name: "spi2lpen",
                    description: Some(
                        "SPI2 peripheral clocks enable in low-power mode Set and reset by software.",
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
                    name: "spi3lpen",
                    description: Some(
                        "SPI3 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxlpen",
                    description: Some(
                        "SPDIFRX peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2lpen",
                    description: Some(
                        "USART2 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3lpen",
                    description: Some(
                        "USART3 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4lpen",
                    description: Some(
                        "UART4 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5lpen",
                    description: Some(
                        "UART5 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_i3c1lpen",
                    description: Some(
                        "I2C1/I3C1 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2lpen",
                    description: Some(
                        "I2C2 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3lpen",
                    description: Some(
                        "I2C3 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ceclpen",
                    description: Some(
                        "HDMI-CEC peripheral clocks enable in low-power mode Set and reset by software.",
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
                    name: "uart7lpen",
                    description: Some(
                        "UART7 peripheral clocks enable in low-power mode Set and reset by software.",
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
                Field {
                    name: "uart8lpen",
                    description: Some(
                        "UART8 peripheral clocks enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lpenr2",
            extends: None,
            description: Some(
                "RCC APB1 low-power clock enable register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crslpen",
                    description: Some(
                        "clock recovery system peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "mdioslpen",
                    description: Some(
                        "MDIOS peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcanlpen",
                    description: Some(
                        "FDCAN peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "ucpdlpen",
                    description: Some(
                        "UCPD peripheral clock enable in low-power mode Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb1perckselr",
            extends: None,
            description: Some(
                "RCC APB1 peripherals kernel clock selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart234578sel",
                    description: Some(
                        "USART2,3, UART4,5,7,8 (APB1) kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usart234578sel",
                    ),
                },
                Field {
                    name: "spi23sel",
                    description: Some(
                        "SPI/I2S2 and SPI/I2S3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi123sel",
                    ),
                },
                Field {
                    name: "i2c23sel",
                    description: Some(
                        "I2C2, I2C3 kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i2c1_i3c1sel",
                    description: Some(
                        "I2C1 or I3C1 kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1I3c1sel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "fdcansel",
                    description: Some(
                        "FDCAN kernel clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "spdifrxsel",
                    description: Some(
                        "SPDIFRX kernel clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spdifrxsel",
                    ),
                },
                Field {
                    name: "cecsel",
                    description: Some(
                        "HDMI-CEC kernel clock source selection Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cecsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 block reset Set and reset by software.",
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
                    name: "tim3rst",
                    description: Some(
                        "TIM3 block reset Set and reset by software.",
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
                    name: "tim4rst",
                    description: Some(
                        "TIM4 block reset Set and reset by software.",
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
                    name: "tim5rst",
                    description: Some(
                        "TIM5 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6rst",
                    description: Some(
                        "TIM6 block reset Set and reset by software.",
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
                    name: "tim7rst",
                    description: Some(
                        "TIM7 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12rst",
                    description: Some(
                        "TIM12 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13rst",
                    description: Some(
                        "TIM13 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14rst",
                    description: Some(
                        "TIM14 block reset Set and reset by software.",
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
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 block reset Set and reset by software.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2S2 block reset Set and reset by software.",
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
                    name: "spi3rst",
                    description: Some(
                        "SPI2S3 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxrst",
                    description: Some(
                        "SPDIFRX block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART2 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART3 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4rst",
                    description: Some(
                        "UART4 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5rst",
                    description: Some(
                        "UART5 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_i3c1rst",
                    description: Some(
                        "I2C1/I3C1 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cecrst",
                    description: Some(
                        "HDMI-CEC block reset Set and reset by software.",
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
                    name: "uart7rst",
                    description: Some(
                        "UART7 block reset Set and reset by software.",
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
                Field {
                    name: "uart8rst",
                    description: Some(
                        "UART8 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crsrst",
                    description: Some(
                        "clock recovery system reset Set and reset by software.",
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
                    name: "mdiosrst",
                    description: Some(
                        "MDIOS block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcanrst",
                    description: Some(
                        "FDCAN block reset Set and reset by software.",
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
                    name: "ucpdrst",
                    description: Some(
                        "UCPD block reset Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "RCC APB2 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 peripheral clock enable Set and reset by software.",
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
                    name: "usart1en",
                    description: Some(
                        "USART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART1SEL, and the pclk2 bus interface clock.",
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
                    name: "spi1en",
                    description: Some(
                        "SPI2S1 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by SPI1SEL, and the pclk2 bus interface clock.",
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
                    name: "spi4en",
                    description: Some(
                        "SPI4 Peripheral Clocks Enable Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock.",
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
                    name: "tim15en",
                    description: Some(
                        "TIM15 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16en",
                    description: Some(
                        "TIM16 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17en",
                    description: Some(
                        "TIM17 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9en",
                    description: Some(
                        "TIM9 peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5en",
                    description: Some(
                        "SPI5 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1en",
                    description: Some(
                        "SAI1 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI1 are the kernel clock selected by SAI1SEL, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2en",
                    description: Some(
                        "SAI2 peripheral clocks enable Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI2SEL, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "RCC APB2 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "usart1lpen",
                    description: Some(
                        "USART1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART169SEL and provided to UCLK inputs, and the pclk2 bus interface clock.",
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
                    name: "spi1lpen",
                    description: Some(
                        "SPI2S1 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI2S1 are: the kernel clock selected by I2S1SEL and provided to spi_ker_ck input, and the pclk2 bus interface clock.",
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
                    name: "spi4lpen",
                    description: Some(
                        "SPI4 peripheral clock enable in low-power mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock.",
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
                    name: "tim15lpen",
                    description: Some(
                        "TIM15 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16lpen",
                    description: Some(
                        "TIM16 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17lpen",
                    description: Some(
                        "TIM17 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9lpen",
                    description: Some(
                        "TIM9 peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5lpen",
                    description: Some(
                        "SPI5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to com_clk input, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1lpen",
                    description: Some(
                        "SAI1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2lpen",
                    description: Some(
                        "SAI2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SAI2 are: the kernel clock selected by SAI2SEL and provided to SAI_CK_A and SAI_CK_B inputs, and the pclk2 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2perckselr",
            extends: None,
            description: Some(
                "RCC APB2 peripherals kernel clock selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "spi45sel",
                    description: Some(
                        "SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi45sel",
                    ),
                },
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI/I2S1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi123sel",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sai1sel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not possible to switch to another clock. Refer to Clock switches and gating on page 437 for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see Figure 51).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sai2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 block reset Set and reset by software.",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 block reset Set and reset by software.",
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
                    name: "spi1rst",
                    description: Some(
                        "SPI2S1 block reset Set and reset by software.",
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
                    name: "spi4rst",
                    description: Some(
                        "SPI4 block reset Set and reset by software.",
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
                    name: "tim15rst",
                    description: Some(
                        "TIM15 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16rst",
                    description: Some(
                        "TIM16 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17rst",
                    description: Some(
                        "TIM17 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9rst",
                    description: Some(
                        "TIM9 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5rst",
                    description: Some(
                        "SPI5 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1rst",
                    description: Some(
                        "SAI1 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2rst",
                    description: Some(
                        "SAI2 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb45perckselr",
            extends: None,
            description: Some(
                "RCC APB4,5 peripherals kernel clock selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
                Field {
                    name: "spi6sel",
                    description: Some(
                        "SPI/I2S6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi6sel",
                    ),
                },
                Field {
                    name: "lptim23sel",
                    description: Some(
                        "LPTIM2 and LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim45sel",
                    description: Some(
                        "LPTIM4, and LPTIM5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb4enr",
            extends: None,
            description: Some(
                "RCC APB4 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SBS peripheral clock enable Set and reset by software.",
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
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 peripheral clocks enable Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the pclk4 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6en",
                    description: Some(
                        "SPI/I2S6 peripheral clocks enable Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_clk input, and the pclk4 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 peripheral clocks enable Set and reset by software. The LPTIM2 kernel clock can be selected by LPTIM23SEL.",
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
                    name: "lptim3en",
                    description: Some(
                        "LPTIM3 peripheral clocks enable Set and reset by software. The LPTIM3 kernel clock can be selected by LPTIM23SEL.",
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
                    name: "lptim4en",
                    description: Some(
                        "LPTIM4 peripheral clocks enable Set and reset by software. The LPTIM4 kernel clock can be selected by LPTIM45SEL.",
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
                    name: "lptim5en",
                    description: Some(
                        "LPTIM5 peripheral clocks enable Set and reset by software. The LPTIM5 kernel clock can be selected by LPTIM45SEL.",
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
                    name: "vrefen",
                    description: Some(
                        "VREF peripheral clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "RTC APB clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmpsensen",
                    description: Some(
                        "Temperature Sensor peripheral clock enable Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb4lpenr",
            extends: None,
            description: Some(
                "RCC APB4 low-power clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfglpen",
                    description: Some(
                        "SBS peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "lpuart1lpen",
                    description: Some(
                        "LPUART1 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to UCLK input, and the rcc_pclk4 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6lpen",
                    description: Some(
                        "SPI/I2S6 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the SPI/I2S6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2lpen",
                    description: Some(
                        "LPTIM2 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock.",
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
                    name: "lptim3lpen",
                    description: Some(
                        "LPTIM3 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM23SEL and provided to clk_lpt input, and the pclk4 bus interface clock.",
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
                    name: "lptim4lpen",
                    description: Some(
                        "LPTIM4 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM4 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock.",
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
                    name: "lptim5lpen",
                    description: Some(
                        "LPTIM5 peripheral clocks enable in low-power mode Set and reset by software. The peripheral clocks of the LPTIM5 are the kernel clock selected by LPTIM45SEL and provided to clk_lpt input, and the pclk4 bus interface clock.",
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
                    name: "vreflpen",
                    description: Some(
                        "VREF peripheral clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapblpen",
                    description: Some(
                        "RTC APB clock enable in low-power mode Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmpsenslpen",
                    description: Some(
                        "temperature sensor peripheral clock enable in low-power mode Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb4rstr",
            extends: None,
            description: Some(
                "RCC APB4 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SBS block reset Set and reset by software.",
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
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6rst",
                    description: Some(
                        "SPI/I2S6 block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 block reset Set and reset by software.",
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
                    name: "lptim3rst",
                    description: Some(
                        "LPTIM3 block reset Set and reset by software.",
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
                    name: "lptim4rst",
                    description: Some(
                        "LPTIM4 block reset Set and reset by software.",
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
                    name: "lptim5rst",
                    description: Some(
                        "LPTIM5 block reset Set and reset by software.",
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
                    name: "vrefrst",
                    description: Some(
                        "VREF block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmpsensrst",
                    description: Some(
                        "TMPSENS block reset Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb5enr",
            extends: None,
            description: Some(
                "RCC APB5 clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LTDC peripheral clock enable Provides the pixel clock (ltdc_clk) to the LTDC block. Set and reset by software.",
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
                    name: "dcmippen",
                    description: Some(
                        "DCMIPP peripheral clock enable Set and reset by software.",
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
                    name: "gfxtimen",
                    description: Some(
                        "GFXTIM peripheral clock enable Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb5lpenr",
            extends: None,
            description: Some(
                "RCC APB5 sleep clock register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdclpen",
                    description: Some(
                        "LTDC peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "dcmipplpen",
                    description: Some(
                        "DCMIPP peripheral clock enable in low-power mode Set and reset by software.",
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
                    name: "gfxtimlpen",
                    description: Some(
                        "GFXTIM peripheral clock enable in low-power mode Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb5rstr",
            extends: None,
            description: Some(
                "RCC APB5 peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcrst",
                    description: Some(
                        "LTDC block reset Set and reset by software.",
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
                    name: "dcmipprst",
                    description: Some(
                        "DCMIPP block reset Set and reset by software.",
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
                    name: "gfxtimrst",
                    description: Some(
                        "GFXTIM block reset Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apbcfgr",
            extends: None,
            description: Some(
                "RCC APB clocks configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppre1",
                    description: Some(
                        "CPU domain APB1 prescaler Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE1 write. 0xx: rcc_pclk1 = sys_bus_ck (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "CPU domain APB2 prescaler Set and reset by software to control the division factor of rcc_pclk2. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE2 write. 0xx: rcc_pclk2 = sys_bus_ck (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre4",
                    description: Some(
                        "CPU domain APB4 prescaler Set and reset by software to control the division factor of rcc_pclk4. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE4 write. 0xx: rcc_pclk4 = sys_bus_ck (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre5",
                    description: Some(
                        "CPU domain APB5 prescaler Set and reset by software to control the division factor of rcc_pclk5. The clock is divided by the new prescaler factor from 1 to 16 cycles of sys_bus_ck after PPRE5 write. 0xx: rcc_pclk5 = sys_bus_ck (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup domain control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enabled Set and reset by software.",
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
                    name: "lserdy",
                    description: Some(
                        "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.",
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
                    name: "lsebyp",
                    description: Some(
                        "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1).",
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
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "lsecsson",
                    description: Some(
                        "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssd",
                    description: Some(
                        "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lseext",
                    description: Some(
                        "low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "lsecssra",
                    description: Some(
                        "Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details.",
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
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vswrst",
                    description: Some(
                        "VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bmcfgr",
            extends: None,
            description: Some(
                "RCC AHB clock configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmpre",
                    description: Some(
                        "Bus matrix clock prescaler Set and reset by software to control the division factor of rcc_hclk[5:1] and rcc_aclk. This group of clocks is also named sys_bus_ck. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: sys_bus_ck= sys_cpu_ck (default after reset) Note: The clocks are divided by the new prescaler factor from 1 to 16 periods of the slowest APB clock among rcc_pclk1,2,4,5 after BMPRE update. Note: Note also that frequency of rcc_hclk[5:1] = rcc_aclk = sys_bus_ck.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cdcfgr",
            extends: None,
            description: Some(
                "RCC CPU domain clock configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpre",
                    description: Some(
                        "CPU domain core prescaler Set and reset by software to control the CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "RCC clock configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock switch Set and reset by software to select system clock source (sys_ck). Set by hardware in order to force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode or in case of failure of the HSE when used directly or indirectly as system clock. others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "stopkerwuck",
                    description: Some(
                        "kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See Section 1.: Dividers values can be changed on-the-fly. All dividers provide have 50% duty-cycles. for details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopkerwuck",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. or 4, else it is equal to 4 x F<sub>rcc_pclkx_d2</sub> Refer to Table 64: Ratio between clock timer and pclk for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timpre",
                    ),
                },
                Field {
                    name: "mco1pre",
                    description: Some(
                        "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco1sel",
                    description: Some(
                        "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco1sel",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2sel",
                    description: Some(
                        "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock source interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.",
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
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.",
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
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.",
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
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyc",
                    description: Some(
                        "CSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.",
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
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssc",
                    description: Some(
                        "LSE clock security system interrupt clear Set by software to clear LSECSSF. Reset by hardware when clear done.",
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
                    name: "hsecssc",
                    description: Some(
                        "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.",
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
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock source interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.",
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
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyie",
                    description: Some(
                        "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.",
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
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssie",
                    description: Some(
                        "LSE clock security system interrupt enable Set and reset by software to enable/disable interrupt caused by the clock security system (CSS) on external 32 kHz oscillator.",
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
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock source interrupt flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.",
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
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set.",
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
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.",
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
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyf",
                    description: Some(
                        "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.",
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
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssf",
                    description: Some(
                        "LSE clock security system interrupt flag Reset by software by writing LSECSSC bit. Set by hardware when a failure is detected on the external 32 kHz oscillator and LSECSSIE is set.",
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
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure.",
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
            ],
        },
        FieldSet {
            name: "Ckgdisr",
            extends: None,
            description: Some(
                "RCC AXI clocks gating disable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "axickg",
                    description: Some(
                        "AXI interconnect matrix clock gating disable This bit is set and reset by software.",
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
                    name: "ahbmckg",
                    description: Some(
                        "AXI master AHB clock gating disable This bit is set and reset by software.",
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
                    name: "sdmmc1ckg",
                    description: Some(
                        "AXI master SDMMC1 clock gating disable This bit is set and reset by software.",
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
                    name: "hpdma1ckg",
                    description: Some(
                        "AXI master HPDMA1 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpuckg",
                    description: Some(
                        "AXI master CPU clock gating disable This bit is set and reset by software.",
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
                    name: "gpus0ckg",
                    description: Some(
                        "AXI master 0 GPU clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpus1ckg",
                    description: Some(
                        "AXI master 1 GPU clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpuclckg",
                    description: Some(
                        "AXI master cache GPU clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcmippckg",
                    description: Some(
                        "AXI master DCMIPP clock gating disable This bit is set and reset by software.",
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
                    name: "dma2dckg",
                    description: Some(
                        "AXI master DMA2D clock gating disable This bit is set and reset by software.",
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
                    name: "gfxmmusckg",
                    description: Some(
                        "AXI matrix slave GFXMMU clock gating disable This bit is set and reset by software.",
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
                    name: "ltdcckg",
                    description: Some(
                        "AXI master LTDC clock gating disable This bit is set and reset by software.",
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
                    name: "gfxmmumckg",
                    description: Some(
                        "AXI master GFXMMU clock gating disable This bit is set and reset by software.",
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
                    name: "ahbsckg",
                    description: Some(
                        "AXI slave AHB clock gating disable This bit is set and reset by software.",
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
                    name: "fmcckg",
                    description: Some(
                        "AXI slave FMC and MCE3 clock gating disable This bit is set and reset by software.",
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
                    name: "xspi1ckg",
                    description: Some(
                        "AXI slave XSPI1 and MCE1 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xspi2ckg",
                    description: Some(
                        "AXI slave XSPI2 and MCE2 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axiram4ckg",
                    description: Some(
                        "AXI matrix slave SRAM4 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axiram3ckg",
                    description: Some(
                        "AXI matrix slave SRAM3 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axiram2ckg",
                    description: Some(
                        "AXI slave SRAM2 clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axiram1ckg",
                    description: Some(
                        "AXI slave SRAM1 / error code correction (ECC) clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flitfckg",
                    description: Some(
                        "AXI slave Flash interface (FLIFT) clock gating disable This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extickg",
                    description: Some(
                        "EXTI clock gating disable This bit is set and reset by software.",
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
                Field {
                    name: "jtagckg",
                    description: Some(
                        "JTAG automatic clock gating disabling This bit is set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ckprotr",
            extends: None,
            description: Some(
                "RCC clock protection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xspickp",
                    description: Some(
                        "XSPI clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the XSPIs. The following fields cannot be modified when this bit is set to 1: PLL2ON, PLL2DIVSEN, PLL2DIVTEN, HSEON, HSION, CSION, XSPIxEN, XSPIxLPEN, XSPIxRST.",
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
                    name: "fmcckp",
                    description: Some(
                        "FMC clock protection Set and cleared by software. When set to 1, this bit prevents disabling accidentally the FMC. The following fields cannot be modified when this bit is set to 1: PLL1ON, PLL2ON, PLL1DIVQEN, PLL2DIVREN, HSEON, HSION, CSION, FMCEN, FMCLPEN, FMCRST.",
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
                    name: "xspi1swp",
                    description: Some(
                        "XSPI1 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Xspiswp",
                    ),
                },
                Field {
                    name: "xspi2swp",
                    description: Some(
                        "XSPI2 kernel clock switch position Set by hardware. This field can be used to verify the real position of XSPI2 kernel switch selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Xspiswp",
                    ),
                },
                Field {
                    name: "fmcswp",
                    description: Some(
                        "FMC kernel clock switch position Set by hardware. This field can be used to verify the real position of FMC kernel switch selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Fmcswp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "RCC source control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 0 or STOPKERWUCK = 0. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW switch) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1.",
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
                    name: "hsikeron",
                    description: Some(
                        "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION.",
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
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable.",
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
                    name: "hsidiv",
                    description: Some(
                        "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hsidiv",
                    ),
                },
                Field {
                    name: "hsidivf",
                    description: Some(
                        "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV. clock setting is completed).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csion",
                    description: Some(
                        "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdy",
                    description: Some(
                        "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request).",
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
                    name: "csikeron",
                    description: Some(
                        "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION.",
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
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode.",
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
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable.",
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
                    name: "hseon",
                    description: Some(
                        "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1) or if FMCCKP = 1, or if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseext",
                    description: Some(
                        "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hseext",
                    ),
                },
                Field {
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is set only (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3) or if FMCCKP = 1, or if XSPICKP = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crrcr",
            extends: None,
            description: Some(
                "RCC clock recovery RC register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "Internal RC 48 MHz clock calibration Set by hardware by option byte loading. Read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csicfgr",
            extends: None,
            description: Some(
                "RCC CSI calibration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csical",
                    description: Some(
                        "CSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csitrim",
                    description: Some(
                        "CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_opt) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_opt. Note: The reset value of the field is 0x20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "RCC clock control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable Set and reset by software.",
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
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready Set and reset by hardware to indicate when the low-speed internal RC oscillator is stable. This bit needs 3 cycles of lsi_ck clock to fall down after LSION has been set to 0. This bit can be set even when LSION is not enabled if there is a request for LSI clock by the clock security system on LSE or by the low-speed watchdog or by the RTC.",
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
            ],
        },
        FieldSet {
            name: "Hsicfgr",
            extends: None,
            description: Some(
                "RCC HSI calibration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration Set by hardware by option byte loading. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_opt) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_opt. Note: The reset value of the field is 0x40.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "RCC PLLs configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllvcosel",
                    description: Some(
                        "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllvcosel",
                    ),
                },
                Field {
                    name: "pllsscgen",
                    description: Some(
                        "PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "divpen",
                    description: Some(
                        "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divren",
                    description: Some(
                        "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divsen",
                    description: Some(
                        "PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divten",
                    description: Some(
                        "PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 11,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllckselr",
            extends: None,
            description: Some(
                "RCC PLLs clock source selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, PLLSRC must be set to 11.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "divm",
                    description: Some(
                        "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 6,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Plldivr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: Some(
                        "Plln",
                    ),
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Plldivr2",
            extends: None,
            description: Some(
                "RCC PLL1 dividers configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plls",
                    description: Some(
                        "PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Plldivst",
                    ),
                },
                Field {
                    name: "pllt",
                    description: Some(
                        "PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Plldivst",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pllfracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fracn",
                    description: Some(
                        "fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F<sub>ref1_ck</sub> x (DIVN1 + (FRACN / 2<sup>13</sup>)), with DIVN1 between 8 and 420 FRACN can be between 0 and 2<sup>13</sup>- 1 The input frequency F<sub>ref1_ck</sub> must be between 1 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACLE to 0. Write the new fractional value into FRACN. Set the bit PLL1FRACLE to 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllsscgr",
            extends: None,
            description: Some(
                "RCC PLL1 Spread Spectrum Clock Generator register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mod_per",
                    description: Some(
                        "Modulation Period Adjustment for PLL1 Set and reset by software to adjust the modulation period of the clock spreading generator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpdfn_dis1",
                    description: Some(
                        "Dithering TPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function.",
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
                    name: "rpdfn_dis1",
                    description: Some(
                        "Dithering RPDF noise control for PLL1 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function.",
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
                    name: "dwnspread1",
                    description: Some(
                        "Spread spectrum clock generator mode for PLL1 Set and reset by software to select the clock spreading mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dwnspread",
                    ),
                },
                Field {
                    name: "inc_step",
                    description: Some(
                        "Modulation Depth Adjustment for PLL1 Set and reset by software to adjust the modulation depth of the clock spreading generator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsr",
            extends: None,
            description: Some(
                "RCC Reset status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "remove reset flag Set and reset by software to reset the value of the reset flags.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oblrstf",
                    description: Some(
                        "Option byte loading reset flag <sup>(1)</sup> Reset by software by the RMVF bit. Set by hardware when a reset from the option byte loading occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "pin reset flag (NRST) <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "system reset from CPU reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7.",
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
                    name: "iwdgrstf",
                    description: Some(
                        "independent watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs.",
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
                    name: "wwdgrstf",
                    description: Some(
                        "window watchdog reset flag <sup>(1)</sup> Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs.",
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
                    name: "lpwrrstf",
                    description: Some(
                        "reset due to illegal Stop or Standby flag Reset by software by writing the RMVF bit. Set by hardware when the CPU goes erroneously in Stop or Standby mode,.",
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
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Adfsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK1",
                    description: Some(
                        "hclk1 selected as ADF kernel clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as ADF kernel clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cecsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Dwnspread",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CENTER_SPREAD",
                    description: Some(
                        "Center-spread modulation selected (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN_SPREAD",
                    description: Some(
                        "Down-spread modulation selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EthRefClkSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ETH_RMII_REF",
                    description: Some(
                        "PAD ETH_RMII_REF_CLK selected as kernel peripheral clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ker_ck selected as kernel peripheral clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ETH",
                    description: Some(
                        "eth_clk_fb selected as kernel peripheral clock.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "EthphyClkSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ker_ck selected as clock source (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_S",
                    description: Some(
                        "pll3_s_ck selected clock source.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fdcansel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Fmcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK5",
                    description: Some(
                        "hclk5 selected as kernel peripheral clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel peripheral clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r_ck selected as kernel peripheral clock.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker_ck selected as kernel peripheral clock.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Fmcswp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "The switch is in neutral mode and output clock is gated (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "The switch is selecting hclk5.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "The switch is selecting pll1_q_ck.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "The switch is selecting pll2_r_ck.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "The switch is selecting hsi_ker_ck.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "The switch is in recovery position (hclk5/4).",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: None,
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Hseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "HSE in analog mode (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIGITAL",
                    description: Some(
                        "HSE in digital mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsidiv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "division by 1, hsi(_ker)_ck = 64 MHz (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "division by 2, hsi(_ker)_ck = 32 MHz.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "division by 4, hsi(_ker)_ck = 16 MHz.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "division by 8, hsi(_ker)_ck = 8 MHz.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2c1I3c1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2csel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk1 selected as kernel clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lptimsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk_d4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_LOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM_HIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected for micro-controller clock output",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1_p selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected for micro-controller clock output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected for micro-controller clock output",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Divide by 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Divide by 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Divide by 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Divide by 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Divide by 5",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "Divide by 6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "Divide by 7",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Divide by 8",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "Divide by 9",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "Divide by 10",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "Divide by 11",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "Divide by 12",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "Divide by 13",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "Divide by 14",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "Divide by 15",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Persel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Plldiv",
            description: None,
            bit_size: 7,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "DIV33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "DIV34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "DIV35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "DIV36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "DIV37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "DIV38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "DIV39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "DIV40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "DIV41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "DIV42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "DIV43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "DIV44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "DIV45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "DIV46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "DIV47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "DIV48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "DIV49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "DIV50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "DIV51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "DIV52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "DIV53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "DIV54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "DIV55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "DIV56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "DIV57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "DIV58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "DIV59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "DIV60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "DIV61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "DIV62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "DIV63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "DIV64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "DIV65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "DIV66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "DIV67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "DIV68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "DIV69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "DIV70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "DIV71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "DIV72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "DIV73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "DIV74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "DIV75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "DIV76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "DIV77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "DIV78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "DIV79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "DIV80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "DIV81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "DIV82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "DIV83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "DIV84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "DIV85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "DIV86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "DIV87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "DIV88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "DIV89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "DIV90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "DIV91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "DIV92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "DIV93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "DIV94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "DIV95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "DIV96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "DIV97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "DIV98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "DIV99",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "DIV100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "DIV101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "DIV102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "DIV103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "DIV104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "DIV105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "DIV106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "DIV107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "DIV108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "DIV109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "DIV110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "DIV111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "DIV112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "DIV113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "DIV114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "DIV115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "DIV116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "DIV117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "DIV118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "DIV119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "DIV120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "DIV121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "DIV122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "DIV123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "DIV124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "DIV125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "DIV126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "DIV127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 127,
                },
            ],
        },
        Enum {
            name: "Plldivst",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "DIV18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "DIV19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "DIV20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "DIV21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "DIV22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "DIV23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "DIV24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "DIV25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "DIV26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "DIV27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "DIV28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "DIV29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "DIV30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "DIV31",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "DIV33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "DIV34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "DIV35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "DIV36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "DIV37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "DIV38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "DIV39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "DIV40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "DIV41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "DIV42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "DIV43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "DIV44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "DIV45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "DIV46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "DIV47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "DIV48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "DIV49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "DIV50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "DIV51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "DIV52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "DIV53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "DIV54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "DIV55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "DIV56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "DIV57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "DIV58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "DIV59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "DIV60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "DIV61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "DIV62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "DIV63",
                    description: None,
                    value: 63,
                },
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 9,
            variants: &[
                EnumVariant {
                    name: "MUL8",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "MUL9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "MUL10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "MUL11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "MUL12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "MUL13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "MUL14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "MUL15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "MUL16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "MUL17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "MUL18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "MUL19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "MUL20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "MUL21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "MUL22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "MUL23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "MUL24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "MUL25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "MUL26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "MUL27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "MUL28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "MUL29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "MUL30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "MUL31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "MUL32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "MUL33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "MUL34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "MUL35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "MUL36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "MUL37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "MUL38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "MUL39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "MUL40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "MUL41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "MUL42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "MUL43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "MUL44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "MUL45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "MUL46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "MUL47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "MUL48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "MUL49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "MUL50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "MUL51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "MUL52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "MUL53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "MUL54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "MUL55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "MUL56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "MUL57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "MUL58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "MUL59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "MUL60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "MUL61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "MUL62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "MUL63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "MUL64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "MUL65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "MUL66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "MUL67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "MUL68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "MUL69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "MUL70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "MUL71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "MUL72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "MUL73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "MUL74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "MUL75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "MUL76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "MUL77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "MUL78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "MUL79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "MUL80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "MUL81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "MUL82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "MUL83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "MUL84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "MUL85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "MUL86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "MUL87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "MUL88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "MUL89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "MUL90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "MUL91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "MUL92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "MUL93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "MUL94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "MUL95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "MUL96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "MUL97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "MUL98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "MUL99",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "MUL100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "MUL101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "MUL102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "MUL103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "MUL104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "MUL105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "MUL106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "MUL107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "MUL108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "MUL109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "MUL110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "MUL111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "MUL112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "MUL113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "MUL114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "MUL115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "MUL116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "MUL117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "MUL118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "MUL119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "MUL120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "MUL121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "MUL122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "MUL123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "MUL124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "MUL125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "MUL126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "MUL127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "MUL128",
                    description: None,
                    value: 127,
                },
                EnumVariant {
                    name: "MUL129",
                    description: None,
                    value: 128,
                },
                EnumVariant {
                    name: "MUL130",
                    description: None,
                    value: 129,
                },
                EnumVariant {
                    name: "MUL131",
                    description: None,
                    value: 130,
                },
                EnumVariant {
                    name: "MUL132",
                    description: None,
                    value: 131,
                },
                EnumVariant {
                    name: "MUL133",
                    description: None,
                    value: 132,
                },
                EnumVariant {
                    name: "MUL134",
                    description: None,
                    value: 133,
                },
                EnumVariant {
                    name: "MUL135",
                    description: None,
                    value: 134,
                },
                EnumVariant {
                    name: "MUL136",
                    description: None,
                    value: 135,
                },
                EnumVariant {
                    name: "MUL137",
                    description: None,
                    value: 136,
                },
                EnumVariant {
                    name: "MUL138",
                    description: None,
                    value: 137,
                },
                EnumVariant {
                    name: "MUL139",
                    description: None,
                    value: 138,
                },
                EnumVariant {
                    name: "MUL140",
                    description: None,
                    value: 139,
                },
                EnumVariant {
                    name: "MUL141",
                    description: None,
                    value: 140,
                },
                EnumVariant {
                    name: "MUL142",
                    description: None,
                    value: 141,
                },
                EnumVariant {
                    name: "MUL143",
                    description: None,
                    value: 142,
                },
                EnumVariant {
                    name: "MUL144",
                    description: None,
                    value: 143,
                },
                EnumVariant {
                    name: "MUL145",
                    description: None,
                    value: 144,
                },
                EnumVariant {
                    name: "MUL146",
                    description: None,
                    value: 145,
                },
                EnumVariant {
                    name: "MUL147",
                    description: None,
                    value: 146,
                },
                EnumVariant {
                    name: "MUL148",
                    description: None,
                    value: 147,
                },
                EnumVariant {
                    name: "MUL149",
                    description: None,
                    value: 148,
                },
                EnumVariant {
                    name: "MUL150",
                    description: None,
                    value: 149,
                },
                EnumVariant {
                    name: "MUL151",
                    description: None,
                    value: 150,
                },
                EnumVariant {
                    name: "MUL152",
                    description: None,
                    value: 151,
                },
                EnumVariant {
                    name: "MUL153",
                    description: None,
                    value: 152,
                },
                EnumVariant {
                    name: "MUL154",
                    description: None,
                    value: 153,
                },
                EnumVariant {
                    name: "MUL155",
                    description: None,
                    value: 154,
                },
                EnumVariant {
                    name: "MUL156",
                    description: None,
                    value: 155,
                },
                EnumVariant {
                    name: "MUL157",
                    description: None,
                    value: 156,
                },
                EnumVariant {
                    name: "MUL158",
                    description: None,
                    value: 157,
                },
                EnumVariant {
                    name: "MUL159",
                    description: None,
                    value: 158,
                },
                EnumVariant {
                    name: "MUL160",
                    description: None,
                    value: 159,
                },
                EnumVariant {
                    name: "MUL161",
                    description: None,
                    value: 160,
                },
                EnumVariant {
                    name: "MUL162",
                    description: None,
                    value: 161,
                },
                EnumVariant {
                    name: "MUL163",
                    description: None,
                    value: 162,
                },
                EnumVariant {
                    name: "MUL164",
                    description: None,
                    value: 163,
                },
                EnumVariant {
                    name: "MUL165",
                    description: None,
                    value: 164,
                },
                EnumVariant {
                    name: "MUL166",
                    description: None,
                    value: 165,
                },
                EnumVariant {
                    name: "MUL167",
                    description: None,
                    value: 166,
                },
                EnumVariant {
                    name: "MUL168",
                    description: None,
                    value: 167,
                },
                EnumVariant {
                    name: "MUL169",
                    description: None,
                    value: 168,
                },
                EnumVariant {
                    name: "MUL170",
                    description: None,
                    value: 169,
                },
                EnumVariant {
                    name: "MUL171",
                    description: None,
                    value: 170,
                },
                EnumVariant {
                    name: "MUL172",
                    description: None,
                    value: 171,
                },
                EnumVariant {
                    name: "MUL173",
                    description: None,
                    value: 172,
                },
                EnumVariant {
                    name: "MUL174",
                    description: None,
                    value: 173,
                },
                EnumVariant {
                    name: "MUL175",
                    description: None,
                    value: 174,
                },
                EnumVariant {
                    name: "MUL176",
                    description: None,
                    value: 175,
                },
                EnumVariant {
                    name: "MUL177",
                    description: None,
                    value: 176,
                },
                EnumVariant {
                    name: "MUL178",
                    description: None,
                    value: 177,
                },
                EnumVariant {
                    name: "MUL179",
                    description: None,
                    value: 178,
                },
                EnumVariant {
                    name: "MUL180",
                    description: None,
                    value: 179,
                },
                EnumVariant {
                    name: "MUL181",
                    description: None,
                    value: 180,
                },
                EnumVariant {
                    name: "MUL182",
                    description: None,
                    value: 181,
                },
                EnumVariant {
                    name: "MUL183",
                    description: None,
                    value: 182,
                },
                EnumVariant {
                    name: "MUL184",
                    description: None,
                    value: 183,
                },
                EnumVariant {
                    name: "MUL185",
                    description: None,
                    value: 184,
                },
                EnumVariant {
                    name: "MUL186",
                    description: None,
                    value: 185,
                },
                EnumVariant {
                    name: "MUL187",
                    description: None,
                    value: 186,
                },
                EnumVariant {
                    name: "MUL188",
                    description: None,
                    value: 187,
                },
                EnumVariant {
                    name: "MUL189",
                    description: None,
                    value: 188,
                },
                EnumVariant {
                    name: "MUL190",
                    description: None,
                    value: 189,
                },
                EnumVariant {
                    name: "MUL191",
                    description: None,
                    value: 190,
                },
                EnumVariant {
                    name: "MUL192",
                    description: None,
                    value: 191,
                },
                EnumVariant {
                    name: "MUL193",
                    description: None,
                    value: 192,
                },
                EnumVariant {
                    name: "MUL194",
                    description: None,
                    value: 193,
                },
                EnumVariant {
                    name: "MUL195",
                    description: None,
                    value: 194,
                },
                EnumVariant {
                    name: "MUL196",
                    description: None,
                    value: 195,
                },
                EnumVariant {
                    name: "MUL197",
                    description: None,
                    value: 196,
                },
                EnumVariant {
                    name: "MUL198",
                    description: None,
                    value: 197,
                },
                EnumVariant {
                    name: "MUL199",
                    description: None,
                    value: 198,
                },
                EnumVariant {
                    name: "MUL200",
                    description: None,
                    value: 199,
                },
                EnumVariant {
                    name: "MUL201",
                    description: None,
                    value: 200,
                },
                EnumVariant {
                    name: "MUL202",
                    description: None,
                    value: 201,
                },
                EnumVariant {
                    name: "MUL203",
                    description: None,
                    value: 202,
                },
                EnumVariant {
                    name: "MUL204",
                    description: None,
                    value: 203,
                },
                EnumVariant {
                    name: "MUL205",
                    description: None,
                    value: 204,
                },
                EnumVariant {
                    name: "MUL206",
                    description: None,
                    value: 205,
                },
                EnumVariant {
                    name: "MUL207",
                    description: None,
                    value: 206,
                },
                EnumVariant {
                    name: "MUL208",
                    description: None,
                    value: 207,
                },
                EnumVariant {
                    name: "MUL209",
                    description: None,
                    value: 208,
                },
                EnumVariant {
                    name: "MUL210",
                    description: None,
                    value: 209,
                },
                EnumVariant {
                    name: "MUL211",
                    description: None,
                    value: 210,
                },
                EnumVariant {
                    name: "MUL212",
                    description: None,
                    value: 211,
                },
                EnumVariant {
                    name: "MUL213",
                    description: None,
                    value: 212,
                },
                EnumVariant {
                    name: "MUL214",
                    description: None,
                    value: 213,
                },
                EnumVariant {
                    name: "MUL215",
                    description: None,
                    value: 214,
                },
                EnumVariant {
                    name: "MUL216",
                    description: None,
                    value: 215,
                },
                EnumVariant {
                    name: "MUL217",
                    description: None,
                    value: 216,
                },
                EnumVariant {
                    name: "MUL218",
                    description: None,
                    value: 217,
                },
                EnumVariant {
                    name: "MUL219",
                    description: None,
                    value: 218,
                },
                EnumVariant {
                    name: "MUL220",
                    description: None,
                    value: 219,
                },
                EnumVariant {
                    name: "MUL221",
                    description: None,
                    value: 220,
                },
                EnumVariant {
                    name: "MUL222",
                    description: None,
                    value: 221,
                },
                EnumVariant {
                    name: "MUL223",
                    description: None,
                    value: 222,
                },
                EnumVariant {
                    name: "MUL224",
                    description: None,
                    value: 223,
                },
                EnumVariant {
                    name: "MUL225",
                    description: None,
                    value: 224,
                },
                EnumVariant {
                    name: "MUL226",
                    description: None,
                    value: 225,
                },
                EnumVariant {
                    name: "MUL227",
                    description: None,
                    value: 226,
                },
                EnumVariant {
                    name: "MUL228",
                    description: None,
                    value: 227,
                },
                EnumVariant {
                    name: "MUL229",
                    description: None,
                    value: 228,
                },
                EnumVariant {
                    name: "MUL230",
                    description: None,
                    value: 229,
                },
                EnumVariant {
                    name: "MUL231",
                    description: None,
                    value: 230,
                },
                EnumVariant {
                    name: "MUL232",
                    description: None,
                    value: 231,
                },
                EnumVariant {
                    name: "MUL233",
                    description: None,
                    value: 232,
                },
                EnumVariant {
                    name: "MUL234",
                    description: None,
                    value: 233,
                },
                EnumVariant {
                    name: "MUL235",
                    description: None,
                    value: 234,
                },
                EnumVariant {
                    name: "MUL236",
                    description: None,
                    value: 235,
                },
                EnumVariant {
                    name: "MUL237",
                    description: None,
                    value: 236,
                },
                EnumVariant {
                    name: "MUL238",
                    description: None,
                    value: 237,
                },
                EnumVariant {
                    name: "MUL239",
                    description: None,
                    value: 238,
                },
                EnumVariant {
                    name: "MUL240",
                    description: None,
                    value: 239,
                },
                EnumVariant {
                    name: "MUL241",
                    description: None,
                    value: 240,
                },
                EnumVariant {
                    name: "MUL242",
                    description: None,
                    value: 241,
                },
                EnumVariant {
                    name: "MUL243",
                    description: None,
                    value: 242,
                },
                EnumVariant {
                    name: "MUL244",
                    description: None,
                    value: 243,
                },
                EnumVariant {
                    name: "MUL245",
                    description: None,
                    value: 244,
                },
                EnumVariant {
                    name: "MUL246",
                    description: None,
                    value: 245,
                },
                EnumVariant {
                    name: "MUL247",
                    description: None,
                    value: 246,
                },
                EnumVariant {
                    name: "MUL248",
                    description: None,
                    value: 247,
                },
                EnumVariant {
                    name: "MUL249",
                    description: None,
                    value: 248,
                },
                EnumVariant {
                    name: "MUL250",
                    description: None,
                    value: 249,
                },
                EnumVariant {
                    name: "MUL251",
                    description: None,
                    value: 250,
                },
                EnumVariant {
                    name: "MUL252",
                    description: None,
                    value: 251,
                },
                EnumVariant {
                    name: "MUL253",
                    description: None,
                    value: 252,
                },
                EnumVariant {
                    name: "MUL254",
                    description: None,
                    value: 253,
                },
                EnumVariant {
                    name: "MUL255",
                    description: None,
                    value: 254,
                },
                EnumVariant {
                    name: "MUL256",
                    description: None,
                    value: 255,
                },
                EnumVariant {
                    name: "MUL257",
                    description: None,
                    value: 256,
                },
                EnumVariant {
                    name: "MUL258",
                    description: None,
                    value: 257,
                },
                EnumVariant {
                    name: "MUL259",
                    description: None,
                    value: 258,
                },
                EnumVariant {
                    name: "MUL260",
                    description: None,
                    value: 259,
                },
                EnumVariant {
                    name: "MUL261",
                    description: None,
                    value: 260,
                },
                EnumVariant {
                    name: "MUL262",
                    description: None,
                    value: 261,
                },
                EnumVariant {
                    name: "MUL263",
                    description: None,
                    value: 262,
                },
                EnumVariant {
                    name: "MUL264",
                    description: None,
                    value: 263,
                },
                EnumVariant {
                    name: "MUL265",
                    description: None,
                    value: 264,
                },
                EnumVariant {
                    name: "MUL266",
                    description: None,
                    value: 265,
                },
                EnumVariant {
                    name: "MUL267",
                    description: None,
                    value: 266,
                },
                EnumVariant {
                    name: "MUL268",
                    description: None,
                    value: 267,
                },
                EnumVariant {
                    name: "MUL269",
                    description: None,
                    value: 268,
                },
                EnumVariant {
                    name: "MUL270",
                    description: None,
                    value: 269,
                },
                EnumVariant {
                    name: "MUL271",
                    description: None,
                    value: 270,
                },
                EnumVariant {
                    name: "MUL272",
                    description: None,
                    value: 271,
                },
                EnumVariant {
                    name: "MUL273",
                    description: None,
                    value: 272,
                },
                EnumVariant {
                    name: "MUL274",
                    description: None,
                    value: 273,
                },
                EnumVariant {
                    name: "MUL275",
                    description: None,
                    value: 274,
                },
                EnumVariant {
                    name: "MUL276",
                    description: None,
                    value: 275,
                },
                EnumVariant {
                    name: "MUL277",
                    description: None,
                    value: 276,
                },
                EnumVariant {
                    name: "MUL278",
                    description: None,
                    value: 277,
                },
                EnumVariant {
                    name: "MUL279",
                    description: None,
                    value: 278,
                },
                EnumVariant {
                    name: "MUL280",
                    description: None,
                    value: 279,
                },
                EnumVariant {
                    name: "MUL281",
                    description: None,
                    value: 280,
                },
                EnumVariant {
                    name: "MUL282",
                    description: None,
                    value: 281,
                },
                EnumVariant {
                    name: "MUL283",
                    description: None,
                    value: 282,
                },
                EnumVariant {
                    name: "MUL284",
                    description: None,
                    value: 283,
                },
                EnumVariant {
                    name: "MUL285",
                    description: None,
                    value: 284,
                },
                EnumVariant {
                    name: "MUL286",
                    description: None,
                    value: 285,
                },
                EnumVariant {
                    name: "MUL287",
                    description: None,
                    value: 286,
                },
                EnumVariant {
                    name: "MUL288",
                    description: None,
                    value: 287,
                },
                EnumVariant {
                    name: "MUL289",
                    description: None,
                    value: 288,
                },
                EnumVariant {
                    name: "MUL290",
                    description: None,
                    value: 289,
                },
                EnumVariant {
                    name: "MUL291",
                    description: None,
                    value: 290,
                },
                EnumVariant {
                    name: "MUL292",
                    description: None,
                    value: 291,
                },
                EnumVariant {
                    name: "MUL293",
                    description: None,
                    value: 292,
                },
                EnumVariant {
                    name: "MUL294",
                    description: None,
                    value: 293,
                },
                EnumVariant {
                    name: "MUL295",
                    description: None,
                    value: 294,
                },
                EnumVariant {
                    name: "MUL296",
                    description: None,
                    value: 295,
                },
                EnumVariant {
                    name: "MUL297",
                    description: None,
                    value: 296,
                },
                EnumVariant {
                    name: "MUL298",
                    description: None,
                    value: 297,
                },
                EnumVariant {
                    name: "MUL299",
                    description: None,
                    value: 298,
                },
                EnumVariant {
                    name: "MUL300",
                    description: None,
                    value: 299,
                },
                EnumVariant {
                    name: "MUL301",
                    description: None,
                    value: 300,
                },
                EnumVariant {
                    name: "MUL302",
                    description: None,
                    value: 301,
                },
                EnumVariant {
                    name: "MUL303",
                    description: None,
                    value: 302,
                },
                EnumVariant {
                    name: "MUL304",
                    description: None,
                    value: 303,
                },
                EnumVariant {
                    name: "MUL305",
                    description: None,
                    value: 304,
                },
                EnumVariant {
                    name: "MUL306",
                    description: None,
                    value: 305,
                },
                EnumVariant {
                    name: "MUL307",
                    description: None,
                    value: 306,
                },
                EnumVariant {
                    name: "MUL308",
                    description: None,
                    value: 307,
                },
                EnumVariant {
                    name: "MUL309",
                    description: None,
                    value: 308,
                },
                EnumVariant {
                    name: "MUL310",
                    description: None,
                    value: 309,
                },
                EnumVariant {
                    name: "MUL311",
                    description: None,
                    value: 310,
                },
                EnumVariant {
                    name: "MUL312",
                    description: None,
                    value: 311,
                },
                EnumVariant {
                    name: "MUL313",
                    description: None,
                    value: 312,
                },
                EnumVariant {
                    name: "MUL314",
                    description: None,
                    value: 313,
                },
                EnumVariant {
                    name: "MUL315",
                    description: None,
                    value: 314,
                },
                EnumVariant {
                    name: "MUL316",
                    description: None,
                    value: 315,
                },
                EnumVariant {
                    name: "MUL317",
                    description: None,
                    value: 316,
                },
                EnumVariant {
                    name: "MUL318",
                    description: None,
                    value: 317,
                },
                EnumVariant {
                    name: "MUL319",
                    description: None,
                    value: 318,
                },
                EnumVariant {
                    name: "MUL320",
                    description: None,
                    value: 319,
                },
                EnumVariant {
                    name: "MUL321",
                    description: None,
                    value: 320,
                },
                EnumVariant {
                    name: "MUL322",
                    description: None,
                    value: 321,
                },
                EnumVariant {
                    name: "MUL323",
                    description: None,
                    value: 322,
                },
                EnumVariant {
                    name: "MUL324",
                    description: None,
                    value: 323,
                },
                EnumVariant {
                    name: "MUL325",
                    description: None,
                    value: 324,
                },
                EnumVariant {
                    name: "MUL326",
                    description: None,
                    value: 325,
                },
                EnumVariant {
                    name: "MUL327",
                    description: None,
                    value: 326,
                },
                EnumVariant {
                    name: "MUL328",
                    description: None,
                    value: 327,
                },
                EnumVariant {
                    name: "MUL329",
                    description: None,
                    value: 328,
                },
                EnumVariant {
                    name: "MUL330",
                    description: None,
                    value: 329,
                },
                EnumVariant {
                    name: "MUL331",
                    description: None,
                    value: 330,
                },
                EnumVariant {
                    name: "MUL332",
                    description: None,
                    value: 331,
                },
                EnumVariant {
                    name: "MUL333",
                    description: None,
                    value: 332,
                },
                EnumVariant {
                    name: "MUL334",
                    description: None,
                    value: 333,
                },
                EnumVariant {
                    name: "MUL335",
                    description: None,
                    value: 334,
                },
                EnumVariant {
                    name: "MUL336",
                    description: None,
                    value: 335,
                },
                EnumVariant {
                    name: "MUL337",
                    description: None,
                    value: 336,
                },
                EnumVariant {
                    name: "MUL338",
                    description: None,
                    value: 337,
                },
                EnumVariant {
                    name: "MUL339",
                    description: None,
                    value: 338,
                },
                EnumVariant {
                    name: "MUL340",
                    description: None,
                    value: 339,
                },
                EnumVariant {
                    name: "MUL341",
                    description: None,
                    value: 340,
                },
                EnumVariant {
                    name: "MUL342",
                    description: None,
                    value: 341,
                },
                EnumVariant {
                    name: "MUL343",
                    description: None,
                    value: 342,
                },
                EnumVariant {
                    name: "MUL344",
                    description: None,
                    value: 343,
                },
                EnumVariant {
                    name: "MUL345",
                    description: None,
                    value: 344,
                },
                EnumVariant {
                    name: "MUL346",
                    description: None,
                    value: 345,
                },
                EnumVariant {
                    name: "MUL347",
                    description: None,
                    value: 346,
                },
                EnumVariant {
                    name: "MUL348",
                    description: None,
                    value: 347,
                },
                EnumVariant {
                    name: "MUL349",
                    description: None,
                    value: 348,
                },
                EnumVariant {
                    name: "MUL350",
                    description: None,
                    value: 349,
                },
                EnumVariant {
                    name: "MUL351",
                    description: None,
                    value: 350,
                },
                EnumVariant {
                    name: "MUL352",
                    description: None,
                    value: 351,
                },
                EnumVariant {
                    name: "MUL353",
                    description: None,
                    value: 352,
                },
                EnumVariant {
                    name: "MUL354",
                    description: None,
                    value: 353,
                },
                EnumVariant {
                    name: "MUL355",
                    description: None,
                    value: 354,
                },
                EnumVariant {
                    name: "MUL356",
                    description: None,
                    value: 355,
                },
                EnumVariant {
                    name: "MUL357",
                    description: None,
                    value: 356,
                },
                EnumVariant {
                    name: "MUL358",
                    description: None,
                    value: 357,
                },
                EnumVariant {
                    name: "MUL359",
                    description: None,
                    value: 358,
                },
                EnumVariant {
                    name: "MUL360",
                    description: None,
                    value: 359,
                },
                EnumVariant {
                    name: "MUL361",
                    description: None,
                    value: 360,
                },
                EnumVariant {
                    name: "MUL362",
                    description: None,
                    value: 361,
                },
                EnumVariant {
                    name: "MUL363",
                    description: None,
                    value: 362,
                },
                EnumVariant {
                    name: "MUL364",
                    description: None,
                    value: 363,
                },
                EnumVariant {
                    name: "MUL365",
                    description: None,
                    value: 364,
                },
                EnumVariant {
                    name: "MUL366",
                    description: None,
                    value: 365,
                },
                EnumVariant {
                    name: "MUL367",
                    description: None,
                    value: 366,
                },
                EnumVariant {
                    name: "MUL368",
                    description: None,
                    value: 367,
                },
                EnumVariant {
                    name: "MUL369",
                    description: None,
                    value: 368,
                },
                EnumVariant {
                    name: "MUL370",
                    description: None,
                    value: 369,
                },
                EnumVariant {
                    name: "MUL371",
                    description: None,
                    value: 370,
                },
                EnumVariant {
                    name: "MUL372",
                    description: None,
                    value: 371,
                },
                EnumVariant {
                    name: "MUL373",
                    description: None,
                    value: 372,
                },
                EnumVariant {
                    name: "MUL374",
                    description: None,
                    value: 373,
                },
                EnumVariant {
                    name: "MUL375",
                    description: None,
                    value: 374,
                },
                EnumVariant {
                    name: "MUL376",
                    description: None,
                    value: 375,
                },
                EnumVariant {
                    name: "MUL377",
                    description: None,
                    value: 376,
                },
                EnumVariant {
                    name: "MUL378",
                    description: None,
                    value: 377,
                },
                EnumVariant {
                    name: "MUL379",
                    description: None,
                    value: 378,
                },
                EnumVariant {
                    name: "MUL380",
                    description: None,
                    value: 379,
                },
                EnumVariant {
                    name: "MUL381",
                    description: None,
                    value: 380,
                },
                EnumVariant {
                    name: "MUL382",
                    description: None,
                    value: 381,
                },
                EnumVariant {
                    name: "MUL383",
                    description: None,
                    value: 382,
                },
                EnumVariant {
                    name: "MUL384",
                    description: None,
                    value: 383,
                },
                EnumVariant {
                    name: "MUL385",
                    description: None,
                    value: 384,
                },
                EnumVariant {
                    name: "MUL386",
                    description: None,
                    value: 385,
                },
                EnumVariant {
                    name: "MUL387",
                    description: None,
                    value: 386,
                },
                EnumVariant {
                    name: "MUL388",
                    description: None,
                    value: 387,
                },
                EnumVariant {
                    name: "MUL389",
                    description: None,
                    value: 388,
                },
                EnumVariant {
                    name: "MUL390",
                    description: None,
                    value: 389,
                },
                EnumVariant {
                    name: "MUL391",
                    description: None,
                    value: 390,
                },
                EnumVariant {
                    name: "MUL392",
                    description: None,
                    value: 391,
                },
                EnumVariant {
                    name: "MUL393",
                    description: None,
                    value: 392,
                },
                EnumVariant {
                    name: "MUL394",
                    description: None,
                    value: 393,
                },
                EnumVariant {
                    name: "MUL395",
                    description: None,
                    value: 394,
                },
                EnumVariant {
                    name: "MUL396",
                    description: None,
                    value: 395,
                },
                EnumVariant {
                    name: "MUL397",
                    description: None,
                    value: 396,
                },
                EnumVariant {
                    name: "MUL398",
                    description: None,
                    value: 397,
                },
                EnumVariant {
                    name: "MUL399",
                    description: None,
                    value: 398,
                },
                EnumVariant {
                    name: "MUL400",
                    description: None,
                    value: 399,
                },
                EnumVariant {
                    name: "MUL401",
                    description: None,
                    value: 400,
                },
                EnumVariant {
                    name: "MUL402",
                    description: None,
                    value: 401,
                },
                EnumVariant {
                    name: "MUL403",
                    description: None,
                    value: 402,
                },
                EnumVariant {
                    name: "MUL404",
                    description: None,
                    value: 403,
                },
                EnumVariant {
                    name: "MUL405",
                    description: None,
                    value: 404,
                },
                EnumVariant {
                    name: "MUL406",
                    description: None,
                    value: 405,
                },
                EnumVariant {
                    name: "MUL407",
                    description: None,
                    value: 406,
                },
                EnumVariant {
                    name: "MUL408",
                    description: None,
                    value: 407,
                },
                EnumVariant {
                    name: "MUL409",
                    description: None,
                    value: 408,
                },
                EnumVariant {
                    name: "MUL410",
                    description: None,
                    value: 409,
                },
                EnumVariant {
                    name: "MUL411",
                    description: None,
                    value: 410,
                },
                EnumVariant {
                    name: "MUL412",
                    description: None,
                    value: 411,
                },
                EnumVariant {
                    name: "MUL413",
                    description: None,
                    value: 412,
                },
                EnumVariant {
                    name: "MUL414",
                    description: None,
                    value: 413,
                },
                EnumVariant {
                    name: "MUL415",
                    description: None,
                    value: 414,
                },
                EnumVariant {
                    name: "MUL416",
                    description: None,
                    value: 415,
                },
                EnumVariant {
                    name: "MUL417",
                    description: None,
                    value: 416,
                },
                EnumVariant {
                    name: "MUL418",
                    description: None,
                    value: 417,
                },
                EnumVariant {
                    name: "MUL419",
                    description: None,
                    value: 418,
                },
                EnumVariant {
                    name: "MUL420",
                    description: None,
                    value: 419,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Frequency is between 1 and 2 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Frequency is between 2 and 4 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE4",
                    description: Some(
                        "Frequency is between 4 and 8 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE8",
                    description: Some(
                        "Frequency is between 8 and 16 MHz",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as PLL clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock sent to DIVMx dividers and PLLs",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllvcosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "WIDE_VCO",
                    description: Some(
                        "VCOH selected (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_VCO",
                    description: Some(
                        "VCOL selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "rcc_hclk not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "rcc_hclk divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "rcc_hclk divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "rcc_hclk divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "rcc_hclk divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pssisel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r_ck selected as kernel peripheral clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel peripheral clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock used as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock used as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by a prescaler used as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sai1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "I2S_CKIN",
                    description: Some(
                        "I2S_CKIN selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Sai2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "I2S_CKIN",
                    description: Some(
                        "I2S_CKIN selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SPDIFRX_SYMB",
                    description: Some(
                        "spdifrx_symb_ck selected as SAI2 kernel clock.",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL2_S",
                    description: Some(
                        "pll2_s_ck selected as kernel peripheral clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_T",
                    description: Some(
                        "pll2_t_ck selected as kernel peripheral clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spdifrxsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Spi123sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as SPI/I2S1 and 7 kernel clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as SPI/I2S1 and 7 kernel clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p_ck selected as SPI/I2S1 and 7 kernel clock.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "I2S_CKIN",
                    description: Some(
                        "I2S_CKIN selected as SPI/I2S1 and 7 kernel clock.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as SPI/I2S1,and 7 kernel clock.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Spi45sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "APB2 clock selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Spi6sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Stopkerwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as wake up clock from system Stop (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wake up clock from system Stop.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as wake up clock from system Stop",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wake up clock from system Stop",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Timpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DEFAULT_X2",
                    description: Some(
                        "Timer kernel clock equal to 2x pclk by default",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEFAULT_X4",
                    description: Some(
                        "Timer kernel clock equal to 4x pclk by default",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Usart234578sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "UsbOtgFssel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "hsi48_ker_ck (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q_ck.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ker_ck.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CLK48MOHCI",
                    description: Some(
                        "clk48mohci.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usbpdctrl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REMAIN_POWERED",
                    description: Some(
                        "In SUSPEND, PHY state machine, bias and USBPHYC PLL remain powered (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POWER_DOWN",
                    description: Some(
                        "In SUSPEND, PHY state machine, bias and USBPHYC PLL are powered down.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usbphycsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ker_ck (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE_DIV_2",
                    description: Some(
                        "hse_ker_ck / 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q_ck.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Usbrefcksel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "MHZ16",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 16 MHz.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MHZ19_2",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 19.2 MHz.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MHZ20",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 20MHz.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MHZ24",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 24 MHz (default after reset).",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MHZ32",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 32 MHz.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MHZ26",
                    description: Some(
                        "The kernel clock frequency provided to the USBPHYC is 26 MHz.",
                    ),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "Xspisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK5",
                    description: Some(
                        "hclk5 selected as kernel peripheral clock (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_S",
                    description: Some(
                        "pll2_s_ck selected as kernel peripheral clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Xspiswp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "The switch is in neutral mode and output clock is gated (default after reset).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "The switch is selecting hclk5.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "The switch is selecting pll2_s_ck.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "The switch is selecting pll2_t_ck.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "The switch is in recovery position (hclk5/4).",
                    ),
                    value: 4,
                },
            ],
        },
    ],
};
