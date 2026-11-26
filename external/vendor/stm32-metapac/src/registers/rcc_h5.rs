
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock controller",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RCC clock control register",
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
                        "RCC HSI calibration register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                        "RCC clock recovery RC register",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                        "RCC CSI calibration register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                        "RCC clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "cfgr2",
                    description: Some(
                        "RCC CPU domain clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "RCC PLL clock source selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x28,
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
                        "RCC PLL1 dividers register",
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
                                "Plldivr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllfracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x38,
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
                    name: "cier",
                    description: Some(
                        "RCC clock source interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                        "RCC clock source interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x54,
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
                        "RCC clock source interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 0x58,
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
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 reset register",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                        "RCC AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                        "RCC AHB4 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
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
                    name: "apb1lrstr",
                    description: Some(
                        "RCC APB1 peripheral low reset register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hrstr",
                    description: Some(
                        "RCC APB1 peripheral high reset register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "RCC APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
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
                    name: "apb3rstr",
                    description: Some(
                        "RCC APB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripherals clock register",
                    ),
                    array: None,
                    byte_offset: 0x88,
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
                        "RCC AHB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
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
                        "RCC AHB4 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x94,
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
                    name: "apb1lenr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1henr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1henr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "RCC APB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
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
                    name: "apb3enr",
                    description: Some(
                        "RCC APB3 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1lpenr",
                    description: Some(
                        "RCC AHB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
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
                        "RCC AHB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
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
                        "RCC AHB4 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
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
                    name: "apb1llpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1llpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hlpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hlpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "RCC APB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
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
                    name: "apb3lpenr",
                    description: Some(
                        "RCC APB3 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr2",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr3",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr4",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr5",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 0xf0,
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
                    name: "rsr",
                    description: Some(
                        "RCC reset status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
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
                    name: "seccfgr",
                    description: Some(
                        "RCC secure configuration register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "RCC privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
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
                "RCC AHB1 peripherals clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 clock enable\r Set and reset by software.",
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
                    name: "gpdma2en",
                    description: Some(
                        "GPDMA2 clock enable\r Set and reset by software.",
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
                    name: "flitfen",
                    description: Some(
                        "Flash interface clock enable\r Set and reset by software.",
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
                    name: "crcen",
                    description: Some(
                        "CRC clock enable\r Set and reset by software.",
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
                    name: "cordicen",
                    description: Some(
                        "CORDIC clock enable\r Set and reset by software.",
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
                    name: "fmacen",
                    description: Some(
                        "FMAC clock enable\r Set and reset by software.",
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
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG clock enable\r Set and reset by software.",
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
                    name: "ethen",
                    description: Some(
                        "ETH clock enable\r Set and reset by software",
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
                    name: "ethtxen",
                    description: Some(
                        "ETHTX clock enable\r Set and reset by software",
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
                    name: "ethrxen",
                    description: Some(
                        "ETHRX clock enable\r Set and reset by software",
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
                    name: "tzsc1en",
                    description: Some(
                        "TZSC1 clock enable\r Set and reset by software",
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
                    name: "bkpramen",
                    description: Some(
                        "BKPRAM clock enable\r Set and reset by software",
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
                    name: "dcacheen",
                    description: Some(
                        "DCACHE clock enable\r Set and reset by software",
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
                    name: "sram1en",
                    description: Some(
                        "SRAM1 clock enable\r Set and reset by software.",
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
                "RCC AHB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1lpen",
                    description: Some(
                        "GPDMA1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "gpdma2lpen",
                    description: Some(
                        "GPDMA2 clock enable during sleep mode\r Set and reset by software.",
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
                        "Flash interface (FLITF) clock enable during sleep mode\r Set and reset by software.",
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
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "cordiclpen",
                    description: Some(
                        "CORDIC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "fmaclpen",
                    description: Some(
                        "FMAC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "ramcfglpen",
                    description: Some(
                        "RAMCFG clock enable during sleep mode\r Set and reset by software.",
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
                    name: "ethlpen",
                    description: Some(
                        "ETH clock enable during Sleep mode\r Set and reset by software",
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
                    name: "ethtxlpen",
                    description: Some(
                        "ETHTX clock enable during sleep mode\r Set and reset by software",
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
                    name: "ethrxlpen",
                    description: Some(
                        "ETHRX clock enable during sleep mode\r Set and reset by software",
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
                    name: "tzsc1lpen",
                    description: Some(
                        "TZSC1 clock enable during sleep mode\r Set and reset by software",
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
                    name: "bkpramlpen",
                    description: Some(
                        "BKPRAM clock enable during sleep mode\r Set and reset by software",
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
                    name: "icachelpen",
                    description: Some(
                        "ICACHE clock enable during sleep mode\r Set and reset by software",
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
                    name: "dcachelpen",
                    description: Some(
                        "DCACHE clock enable during sleep mode\r Set and reset by software",
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
                    name: "sram1lpen",
                    description: Some(
                        "SRAM1 clock enable during sleep mode\r Set and reset by software",
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
                "RCC AHB1 reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 block reset\r Set and reset by software.",
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
                    name: "gpdma2rst",
                    description: Some(
                        "GPDMA2 block reset\r Set and reset by software.",
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
                    name: "crcrst",
                    description: Some(
                        "CRC block reset Set and reset by software.",
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
                    name: "cordicrst",
                    description: Some(
                        "CORDIC block reset\r Set and reset by software.",
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
                    name: "fmacrst",
                    description: Some(
                        "FMAC block reset\r Set and reset by software.",
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
                    name: "ramcfgrst",
                    description: Some(
                        "RAMCFG block reset\r Set and reset by software.",
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
                    name: "ethrst",
                    description: Some(
                        "ETH1 block reset\r Set and reset by software",
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
                    name: "tzsc1rst",
                    description: Some(
                        "TZSC1 reset\r Set and reset by software",
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
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "GPIOA clock enable\r Set and reset by software.",
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
                        "GPIOB clock enable\r Set and reset by software.",
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
                        "GPIOC clock enable\r Set and reset by software.",
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
                        "GPIOD clock enable\r Set and reset by software.",
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
                        "GPIOE clock enable\r Set and reset by software.",
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
                        "GPIOF clock enable\r Set and reset by software.",
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
                        "GPIOG clock enable\r Set and reset by software.",
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
                        "GPIOH clock enable\r Set and reset by software.",
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
                    name: "gpioien",
                    description: Some(
                        "GPIOI clock enable\r Set and reset by software.",
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
                    name: "adc12en",
                    description: Some(
                        "ADC1 and 2 peripherals clock enabled\r Set and reset by software.",
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
                    name: "dac1en",
                    description: Some(
                        "DAC clock enable\r Set and reset by software.",
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
                    name: "dcmi_pssien",
                    description: Some(
                        "digital camera interface clock enable (DCMI or PSSI depending which interface is active)\r Set and reset by software.",
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
                    name: "aesen",
                    description: Some(
                        "AES clock enable\r Set and reset by software.",
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
                    name: "hashen",
                    description: Some(
                        "HASH clock enable\r Set and reset by software.",
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
                    name: "rngen",
                    description: Some(
                        "RNG clock enable\r Set and reset by software.",
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
                    name: "pkaen",
                    description: Some(
                        "PKA clock enable\r Set and reset by software.",
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
                    name: "saesen",
                    description: Some(
                        "SAES clock enable\r Set and reset by software.",
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
                    name: "sram3en",
                    description: Some(
                        "SRAM3 clock enable\r Set and reset by software.",
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
                    name: "sram2en",
                    description: Some(
                        "SRAM2 clock enable\r Set and reset by software.",
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
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "RCC AHB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "GPIOA clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOB clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOC clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOD clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOE clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOF clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOG clock enable during sleep mode\r Set and reset by software.",
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
                        "GPIOH clock enable during sleep mode\r Set and reset by software.",
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
                    name: "gpioilpen",
                    description: Some(
                        "GPIOI clock enable during sleep mode\r Set and reset by software.",
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
                    name: "adc12lpen",
                    description: Some(
                        "ADC1 and 2 peripherals clock enable during sleep mode\r Set and reset by software.",
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
                    name: "dac1lpen",
                    description: Some(
                        "DAC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "dcmi_pssilpen",
                    description: Some(
                        "digital camera interface clock enable during sleep mode (DCMI or PSSI depending which interface is active)\r Set and reset by software.",
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
                    name: "aeslpen",
                    description: Some(
                        "AES clock enable during sleep mode\r Set and reset by software.",
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
                    name: "hashlpen",
                    description: Some(
                        "HASH clock enable during sleep mode\r Set and reset by software.",
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
                    name: "rnglpen",
                    description: Some(
                        "RNG clock enable during sleep mode\r Set and reset by software.",
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
                    name: "pkalpen",
                    description: Some(
                        "PKA clock enable during sleep mode\r Set and reset by software.",
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
                    name: "saeslpen",
                    description: Some(
                        "SAES clock enable during sleep mode\r Set and reset by software.",
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
                    name: "sram2lpen",
                    description: Some(
                        "SRAM2 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "sram3lpen",
                    description: Some(
                        "SRAM3 clock enable during sleep mode\r Set and reset by software.",
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
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "GPIOA block reset\r Set and reset by software.",
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
                        "GPIOB block reset\r Set and reset by software.",
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
                        "GPIOC block reset\r Set and reset by software.",
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
                        "GPIOD block reset\r Set and reset by software.",
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
                        "GPIOE block reset\r Set and reset by software.",
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
                        "GPIOF block reset\r Set and reset by software.",
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
                        "GPIOG block reset\r Set and reset by software.",
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
                        "GPIOH block reset\r Set and reset by software.",
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
                    name: "gpioirst",
                    description: Some(
                        "GPIOI block reset\r Set and reset by software.",
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
                    name: "adc12rst",
                    description: Some(
                        "ADC1 and 2 blocks reset\r Set and reset by software.",
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
                    name: "dac1rst",
                    description: Some(
                        "DAC block reset\r Set and reset by software.",
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
                    name: "dcmi_pssirst",
                    description: Some(
                        "digital camera interface block reset (DCMI or PSSI depending which interface is active)\r Set and reset by software.",
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
                    name: "aesrst",
                    description: Some(
                        "AES block reset\r Set and reset by software.",
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
                    name: "hashrst",
                    description: Some(
                        "HASH block reset\r Set and reset by software.",
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
                    name: "rngrst",
                    description: Some(
                        "RNG block reset\r Set and reset by software.",
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
                    name: "pkarst",
                    description: Some(
                        "PKA block reset\r Set and reset by software.",
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
                    name: "saesrst",
                    description: Some(
                        "SAES block reset\r Set and reset by software.",
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
            name: "Ahb4enr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otfdec1en",
                    description: Some(
                        "OTFDEC1 clock enable\r Set and reset by software.",
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
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 and SDMMC1 delay peripheral clock enable reset",
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
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay peripheral clock enabled\r Set and reset by software.",
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
                    name: "fmcen",
                    description: Some(
                        "FMC clock enable\r Set and reset by software.",
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
                    name: "octospi1en",
                    description: Some(
                        "OCTOSPI1 clock enable\r Set and reset by software.",
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
            name: "Ahb4lpenr",
            extends: None,
            description: Some(
                "RCC AHB4 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otfdec1lpen",
                    description: Some(
                        "OTFDEC1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "sdmmc1lpen",
                    description: Some(
                        "SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode\r Set and reset by software",
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
                    name: "sdmmc2lpen",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode\r Set and reset by software.",
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
                    name: "fmclpen",
                    description: Some(
                        "FMC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "octospi1lpen",
                    description: Some(
                        "OCTOSPI1 clock enable during sleep mode\r Set and reset by software.",
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
            name: "Ahb4rstr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otfdec1rst",
                    description: Some(
                        "OTFDEC1 block reset\r Set and reset by software.",
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
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 and SDMMC1 delay blocks reset\r Set and reset by software.",
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
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay blocks reset\r Set and reset by software.",
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
                    name: "fmcrst",
                    description: Some(
                        "FMC block reset\r Set and reset by software.",
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
                    name: "octospi1rst",
                    description: Some(
                        "OCTOSPI1 block reset\r Set and reset by software.",
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
            name: "Apb1henr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uart9en",
                    description: Some(
                        "UART9 clock enable\r Set and reset by software.",
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
                    name: "uart12en",
                    description: Some(
                        "UART12 clock enable\r Set and reset by software.",
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
                    name: "dtsen",
                    description: Some(
                        "DTS clock enable\r Set and reset by software.",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable\r Set and reset by software.",
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
                    name: "fdcan12en",
                    description: Some(
                        "FDCAN1 and FDCAN2 peripheral clock enable\r Set and reset by software.",
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
                    name: "ucpden",
                    description: Some(
                        "UCPD clock enable\r Set and reset by software.",
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
            name: "Apb1hlpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uart9lpen",
                    description: Some(
                        "UART9 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "uart12lpen",
                    description: Some(
                        "UART12 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "dtslpen",
                    description: Some(
                        "DTS clock enable during sleep mode\r Set and reset by software.",
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
                    name: "lptim2lpen",
                    description: Some(
                        "LPTIM2 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "fdcan12lpen",
                    description: Some(
                        "FDCAN1 and FDCAN2 peripheral clock enable during sleep mode\r Set and reset by software.",
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
                    name: "ucpdlpen",
                    description: Some(
                        "UCPD clock enable during sleep mode\r Set and reset by software.",
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
            name: "Apb1hrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral high reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uart9rst",
                    description: Some(
                        "UART9 block reset\r Set and reset by software.",
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
                    name: "uart12rst",
                    description: Some(
                        "UART12 block reset\r Set and reset by software.",
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
                    name: "dtsrst",
                    description: Some(
                        "DTS block reset\r Set and reset by software.",
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
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 block reset\r Set and reset by software.",
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
                    name: "fdcan12rst",
                    description: Some(
                        "FDCAN1 and FDCAN2 blocks reset\r Set and reset by software.",
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
                    name: "ucpdrst",
                    description: Some(
                        "UCPD block reset\r Set and reset by software.",
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
            name: "Apb1lenr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable\r Set and reset by software.",
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
                        "TIM3 clock enable\r Set and reset by software.",
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
                        "TIM4 clock enable\r Set and reset by software.",
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
                        "TIM5 clock enable\r Set and reset by software.",
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
                        "TIM6 clock enable\r Set and reset by software.",
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
                        "TIM7 clock enable\r Set and reset by software.",
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
                        "TIM12 clock enable\r Set and reset by software.",
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
                        "TIM13 clock enable\r Set and reset by software.",
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
                        "TIM14 clock enable\r Set and reset by software.",
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
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable\r Set and reset by software.",
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
                        "SPI2 clock enable\r Set and reset by software.",
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
                        "SPI3 clock enable\r Set and reset by software.",
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
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable\r Set and reset by software.",
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
                        "USART3 clock enable\r Set and reset by software.",
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
                        "UART4 clock enable\r Set and reset by software.",
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
                        "UART5 clock enable\r Set and reset by software.",
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
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable\r Set and reset by software.",
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
                        "I2C2 clock enable\r Set and reset by software.",
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
                    name: "i3c1en",
                    description: Some(
                        "I3C1 clock enable\r Set and reset by software.",
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
                    name: "crsen",
                    description: Some(
                        "CRS clock enable\r Set and reset by software.",
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
                    name: "usart6en",
                    description: Some(
                        "USART6 clock enable\r Set and reset by software.",
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
                    name: "usart10en",
                    description: Some(
                        "USART10 clock enable\r Set and reset by software.",
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
                    name: "usart11en",
                    description: Some(
                        "USART11 clock enable",
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
                    name: "cecen",
                    description: Some(
                        "HDMI-CEC clock enable\r Set and reset by software.",
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
                    name: "uart7en",
                    description: Some(
                        "UART7 clock enable\r Set and reset by software.",
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
                        "UART8 clock enable\r Set and reset by software.",
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
            name: "Apb1llpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM3 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM4 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM5 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM6 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM7 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM12 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM13 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM14 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "wwdglpen",
                    description: Some(
                        "WWDG clock enable during sleep mode\r Set and reset by software.",
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
                        "SPI2 clock enable during sleep mode\r Set and reset by software.",
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
                        "SPI3 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usart2lpen",
                    description: Some(
                        "USART2 clock enable during sleep mode\r Set and reset by software.",
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
                        "USART3 clock enable during sleep mode\r Set and reset by software.",
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
                        "UART4 clock enable during sleep mode\r Set and reset by software.",
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
                        "UART5 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "i2c1lpen",
                    description: Some(
                        "I2C1 clock enable during sleep mode\r Set and reset by software.",
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
                        "I2C2 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "i3c1lpen",
                    description: Some(
                        "I3C1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "crslpen",
                    description: Some(
                        "CRS clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usart6lpen",
                    description: Some(
                        "USART6 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usart10lpen",
                    description: Some(
                        "USART10 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usart11lpen",
                    description: Some(
                        "USART11 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "ceclpen",
                    description: Some(
                        "HDMI-CEC clock enable during sleep mode\r Set and reset by software.",
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
                    name: "uart7lpen",
                    description: Some(
                        "UART7 clock enable during sleep mode\r Set and reset by software.",
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
                        "UART8 clock enable during sleep mode\r Set and reset by software.",
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
            name: "Apb1lrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral low reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 block reset\r Set and reset by software.",
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
                        "TIM3 block reset\r Set and reset by software.",
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
                        "TIM4 block reset\r Set and reset by software.",
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
                        "TIM5 block reset\r Set and reset by software.",
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
                        "TIM6 block reset\r Set and reset by software.",
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
                        "TIM7 block reset\r Set and reset by software.",
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
                        "TIM12 block reset\r Set and reset by software.",
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
                        "TIM13 block reset t\r Set and reset by software.",
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
                        "TIM14 block reset\r Set and reset by software.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 block reset\r Set and reset by software.",
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
                        "SPI3 block reset\r Set and reset by software.",
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
                    name: "usart2rst",
                    description: Some(
                        "USART2 block reset\r Set and reset by software.",
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
                        "USART3 block reset\r Set and reset by software.",
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
                        "UART4 block reset\r Set and reset by software.",
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
                        "UART5 block reset\r Set and reset by software.",
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
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 block reset\r Set and reset by software.",
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
                        "I2C2 block reset\r Set and reset by software.",
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
                    name: "i3c1rst",
                    description: Some(
                        "I3C1 block reset\r Set and reset by software.",
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
                    name: "crsrst",
                    description: Some(
                        "CRS block reset\r Set and reset by software.",
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
                    name: "usart6rst",
                    description: Some(
                        "USART6 block reset\r Set and reset by software.",
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
                    name: "usart10rst",
                    description: Some(
                        "USART10 block reset\r Set and reset by software.",
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
                    name: "usart11rst",
                    description: Some(
                        "USART11 block reset\r Set and reset by software.",
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
                    name: "cecrst",
                    description: Some(
                        "HDMI-CEC block reset\r Set and reset by software.",
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
                    name: "uart7rst",
                    description: Some(
                        "UART7 block reset\r Set and reset by software.",
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
                        "UART8 block reset\r Set and reset by software.",
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
            name: "Apb2enr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable\r Set and reset by software.",
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
                    name: "spi1en",
                    description: Some(
                        "SPI1 clock enable\r Set and reset by software.",
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
                    name: "tim8en",
                    description: Some(
                        "TIM8 clock enable\r Set and reset by software.",
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
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable\r Set and reset by software.",
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
                    name: "tim15en",
                    description: Some(
                        "TIM15 clock enable\r Set and reset by software.",
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
                        "TIM16 clock enable\r Set and reset by software.",
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
                        "TIM17 clock enable\r Set and reset by software.",
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
                    name: "spi4en",
                    description: Some(
                        "SPI4 clock enable\r Set and reset by software.",
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
                    name: "spi6en",
                    description: Some(
                        "SPI6 clock enable\r Set and reset by software.",
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
                        "SAI1 clock enable\r Set and reset by software.",
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
                    name: "sai2en",
                    description: Some(
                        "SAI2 clock enable\r Set and cleared by software.",
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
                    name: "usben",
                    description: Some(
                        "USB clock enable\r Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "RCC APB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "spi1lpen",
                    description: Some(
                        "SPI1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "tim8lpen",
                    description: Some(
                        "TIM8 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usart1lpen",
                    description: Some(
                        "USART1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "tim15lpen",
                    description: Some(
                        "TIM15 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM16 clock enable during sleep mode\r Set and reset by software.",
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
                        "TIM17 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "spi4lpen",
                    description: Some(
                        "SPI4 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "spi6lpen",
                    description: Some(
                        "SPI6 clock enable during sleep mode\r Set and reset by software.",
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
                        "SAI1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "sai2lpen",
                    description: Some(
                        "SAI2 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "usblpen",
                    description: Some(
                        "USB clock enable during sleep mode\r Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 block reset\r Set and reset by software.",
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
                    name: "spi1rst",
                    description: Some(
                        "SPI1 block reset\r Set and reset by software.",
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
                    name: "tim8rst",
                    description: Some(
                        "TIM8 block reset\r Set and reset by software.",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 block reset\r Set and reset by software.",
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
                    name: "tim15rst",
                    description: Some(
                        "TIM15 block reset\r Set and reset by software.",
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
                        "TIM16 block reset\r Set and reset by software.",
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
                        "TIM17 block reset\r Set and reset by software.",
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
                    name: "spi4rst",
                    description: Some(
                        "SPI4 block reset\r Set and reset by software.",
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
                    name: "spi6rst",
                    description: Some(
                        "SPI6 block reset\r Set and reset by software.",
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
                        "SAI1 block reset\r Set and reset by software.",
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
                    name: "sai2rst",
                    description: Some(
                        "SAI2 block reset\r Set and reset by software.",
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
                    name: "usbrst",
                    description: Some(
                        "USB block reset\r Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb3enr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SBS clock enable\r Set and reset by software.",
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
                    name: "spi5en",
                    description: Some(
                        "SPI5 clock enable\r Set and reset by software.",
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
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 clock enable\r Set and reset by software.",
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
                    name: "i2c3en",
                    description: Some(
                        "I2C3 clock enable\r Set and reset by software.",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable\r Set and reset by software.",
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
                        "LPTIM1 clock enable\r Set and reset by software.",
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
                    name: "lptim3en",
                    description: Some(
                        "LPTIM3 clock enable\r Set and reset by software.",
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
                    name: "lptim4en",
                    description: Some(
                        "LPTIM4 clock enable\r Set and reset by software.",
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
                    name: "lptim5en",
                    description: Some(
                        "LPTIM5 clock enable\r Set and reset by software.",
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
                    name: "lptim6en",
                    description: Some(
                        "LPTIM6 clock enable\r Set and reset by software.",
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
                    name: "vrefen",
                    description: Some(
                        "VREF clock enable\r Set and reset by software.",
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
                    name: "rtcapben",
                    description: Some(
                        "RTC APB interface clock enable\r Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb3lpenr",
            extends: None,
            description: Some(
                "RCC APB3 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfglpen",
                    description: Some(
                        "SBS clock enable during sleep mode\r Set and reset by software.",
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
                    name: "spi5lpen",
                    description: Some(
                        "SPI5 clock enable during Slsleepeep mode\r Set and reset by software.",
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
                    name: "lpuart1lpen",
                    description: Some(
                        "LPUART1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "i2c3lpen",
                    description: Some(
                        "I2C3 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "i2c4lpen",
                    description: Some(
                        "I2C4 clock enable during sleep mode\r Set and reset by software.",
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
                        "LPTIM1 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "lptim3lpen",
                    description: Some(
                        "LPTIM3 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "lptim4lpen",
                    description: Some(
                        "LPTIM4 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "lptim5lpen",
                    description: Some(
                        "LPTIM5 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "lptim6lpen",
                    description: Some(
                        "LPTIM6 clock enable during sleep mode\r Set and reset by software.",
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
                    name: "vreflpen",
                    description: Some(
                        "VREF clock enable during sleep mode\r Set and reset by software.",
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
                    name: "rtcapblpen",
                    description: Some(
                        "RTC APB interface clock enable during sleep mode\r Set and reset by software.",
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
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SBS block reset\r Set and reset by software.",
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
                    name: "spi5rst",
                    description: Some(
                        "SPI5 block reset\r Set and reset by software.",
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
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 block reset\r Set and reset by software.",
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
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 block reset\r Set and reset by software.",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 block reset\r Set and reset by software.",
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
                        "LPTIM1 block reset\r Set and reset by software.",
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
                    name: "lptim3rst",
                    description: Some(
                        "LPTIM3 block reset\r Set and reset by software.",
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
                    name: "lptim4rst",
                    description: Some(
                        "LPTIM4 block reset\r Set and reset by software.",
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
                    name: "lptim5rst",
                    description: Some(
                        "LPTIM5 block reset\r Set and reset by software.",
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
                    name: "lptim6rst",
                    description: Some(
                        "LPTIM6 block reset\r Set and reset by software.",
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
                    name: "vrefrst",
                    description: Some(
                        "VREF block reset\r Set and reset by software.",
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
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enabled\r Set and reset by software.",
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
                        "LSE oscillator ready\r Set and reset by hardware to indicate when the LSE is stable.\r This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.",
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
                        "LSE oscillator bypass\r Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)",
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
                        "LSE oscillator driving capability\r Set by software to select the driving capability of the LSE oscillator.\r These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0).",
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
                        "LSE clock security system enable\r Set by software to enable the clock security system on 32 kHz oscillator.\r LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON.",
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
                        "LSE clock security system failure detection\r Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.",
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
                        "low-speed external clock type in bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the LSEON bit, to be used by the device.\r The LSEEXT bit can be written only if the LSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lseext",
                    ),
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection\r Set by software to select the clock source for the RTC.\r These bits can be written only one time (except in case of failure detection on LSE).\r These bits must be written before LSECSSON is enabled.\r The VSWRST bit can be used to reset them, then it can be written one time again.\r If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).",
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
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable\r Set and reset by software.",
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
                        "VSwitch domain software reset\r Set and reset by software.",
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
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.",
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
                    name: "lscosel",
                    description: Some(
                        "Low-speed clock output selection\r Set and cleared by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lscosel",
                    ),
                },
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable\r Set and cleared by software.",
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
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready\r Set and cleared by hardware to indicate when the LSI oscillator is stable.\r After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles.\r This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.",
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
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
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
                    name: "usart2sel",
                    description: Some(
                        "USART2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart4sel",
                    description: Some(
                        "UART4 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart5sel",
                    description: Some(
                        "UART5 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart6sel",
                    description: Some(
                        "USART6 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart7sel",
                    description: Some(
                        "UART7 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart8sel",
                    description: Some(
                        "UART8 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart9sel",
                    description: Some(
                        "UART9 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart10sel",
                    description: Some(
                        "USART10 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "TIM12, TIM15 and LPTIM2 input capture source selection\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timicsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr2",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart11sel",
                    description: Some(
                        "USART11 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart12sel",
                    description: Some(
                        "USART12 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
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
                    name: "lptim2sel",
                    description: Some(
                        "LPTIM2 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "lptim3sel",
                    description: Some(
                        "LPTIM3 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim4sel",
                    description: Some(
                        "LPTIM4 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim5sel",
                    description: Some(
                        "LPTIM5 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim6sel",
                    description: Some(
                        "LPTIM6 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi1sel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi2sel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi3sel",
                    ),
                },
                Field {
                    name: "spi4sel",
                    description: Some(
                        "SPI4 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi4sel",
                    ),
                },
                Field {
                    name: "spi5sel",
                    description: Some(
                        "SPI5 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi5sel",
                    ),
                },
                Field {
                    name: "spi6sel",
                    description: Some(
                        "SPI6 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi6sel",
                    ),
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpusartsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr4",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "octospi1sel",
                    description: Some(
                        "OCTOSPI1 kernel clock source selection\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Octospisel",
                    ),
                },
                Field {
                    name: "systicksel",
                    description: Some(
                        "SYSTICK clock source selection\r Note: rcc_hclk frequency must be four times higher than\r lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "usbsel",
                    description: Some(
                        "USB kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbsel",
                    ),
                },
                Field {
                    name: "sdmmc1sel",
                    description: Some(
                        "SDMMC1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "sdmmc2sel",
                    description: Some(
                        "SDMMC2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c34sel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c34sel",
                    ),
                },
                Field {
                    name: "i3c1sel",
                    description: Some(
                        "I3C1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr5",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcdacsel",
                    description: Some(
                        "ADC and DAC kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcdacsel",
                    ),
                },
                Field {
                    name: "dacholdsel",
                    description: Some(
                        "DAC hold clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dacholdsel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNG kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "cecsel",
                    description: Some(
                        "HSMI-CEC kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cecsel",
                    ),
                },
                Field {
                    name: "fdcan12sel",
                    description: Some(
                        "FDCAN1 and FDCAN2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "persel",
                    description: Some(
                        "per_ck clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
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
            name: "Cfgr",
            extends: None,
            description: Some(
                "RCC clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock and trace clock switch\r Set and reset by software to select system clock and trace clock sources (sys_ck).\r Set by hardware in order to:\r -\tforce the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode\r -\tforce the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock\r others: reserved",
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
                        "system clock switch status\r Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset).\r others: reserved",
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
                        "system clock selection after a wakeup from system Stop\r Set and reset by software to select the system wakeup clock from system Stop.\r The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset)\r STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).",
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
                        "kernel clock selection after a wakeup from system Stop\r Set and reset by software to select the kernel wakeup clock from system Stop.",
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
                        "HSE division factor for RTC clock\r Set and cleared by software to divide the HSE to generate a clock for RTC.\r Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source.\r ...",
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
                        "timers clocks prescaler selection\r This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains.",
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
                        "MCO1 prescaler\r Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
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
                        "Microcontroller clock output 1\r Set and cleared by software. Clock source selection may generate glitches on MCO1.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
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
                        "MCO2 prescaler\r Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
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
                        "microcontroller clock output 2\r Set and cleared by software. Clock source selection may generate glitches on MCO2.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC CPU domain clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler\r Set and reset by software to control the division factor of rcc_hclk. Changing\r this division ratio has an impact on the frequency of all bus matrix clocks\r 0xxx: rcc_hclk = sys_ck (default after reset)",
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
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB low-speed prescaler (APB1)\r Set and reset by software to control the division factor of rcc_pclk1.\r The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write.\r 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)",
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
                    name: "ppre2",
                    description: Some(
                        "APB high-speed prescaler (APB2)\r Set and reset by software to control APB high-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write.\r 0xx: rcc_pclk2 = rcc_hclk1",
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
                    name: "ppre3",
                    description: Some(
                        "APB low-speed prescaler (APB3)\r Set and reset by software to control APB low-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write.\r 0xx: rcc_pclk3 = rcc_hclk1",
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
                Field {
                    name: "ahb1dis",
                    description: Some(
                        "AHB1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB1\r peripherals from RCC_AHB1ENR are used and when their clocks are disabled in\r RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from\r RCC_AHB1ENR are off.\r enable control bits",
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
                    name: "ahb2dis",
                    description: Some(
                        "AHB2 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB2\r peripherals from RCC_AHB2ENR are used and when their clocks are disabled in\r RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from\r RCC_AHB2ENR are off.\r enable control bits",
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
                    name: "ahb4dis",
                    description: Some(
                        "AHB4 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB4\r peripherals from RCC_AHB4ENR are used and when their clocks are disabled in\r RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from\r RCC_AHB4ENR are off.\r enable control bits",
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
                    name: "apb1dis",
                    description: Some(
                        "APB1 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB1\r peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR.\r When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.\r control bits",
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
                    name: "apb2dis",
                    description: Some(
                        "APB2 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB2\r peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is\r set, all the APB2 peripherals clocks are off.\r control bits",
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
                    name: "apb3dis",
                    description: Some(
                        "APB3 clock disable value.Set and cleared by software\r This bit can be set in order to further reduce power consumption, when none of the APB3\r peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is\r set, all the APB3 peripherals clocks are off.\r control bits",
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
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock source interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear\r Set by software to clear LSIRDYF.\r Reset by hardware when clear done.",
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
                        "LSE ready interrupt clear\r Set by software to clear LSERDYF.\r Reset by hardware when clear done.",
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
                    name: "csirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear CSIRDYF.\r Reset by hardware when clear done.",
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
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear HSIRDYF.\r Reset by hardware when clear done.",
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
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear\r Set by software to clear HSERDYF.\r Reset by hardware when clear done.",
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
                        "HSI48 ready interrupt clear\r Set by software to clear HSI48RDYF.\r Reset by hardware when clear done.",
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
                        "PLL1 ready interrupt clear\r Set by software to clear PLL1RDYF.\r Reset by hardware when clear done.",
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
                    name: "hsecssc",
                    description: Some(
                        "HSE clock security system interrupt clear\r Set by software to clear HSECSSF.\r Reset by hardware when clear done.",
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
                "RCC clock source interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.",
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
                        "LSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.",
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
                    name: "csirdyie",
                    description: Some(
                        "CSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.",
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
                        "HSI48 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.",
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
                        "PLL1 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by PLL1 lock.",
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
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock source interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag\r Reset by software by writing LSIRDYC bit.\r Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.",
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
                        "LSE ready interrupt flag\r Reset by software by writing LSERDYC bit.\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.",
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
                    name: "csirdyf",
                    description: Some(
                        "CSI ready interrupt flag\r Reset by software by writing CSIRDYC bit.\r Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.",
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
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag\r Reset by software by writing HSIRDYC bit.\r Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.",
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
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag\r Reset by software by writing HSERDYC bit.\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.",
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
                        "HSI48 ready interrupt flag\r Reset by software by writing HSI48RDYC bit.\r Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.",
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
                        "PLL1 ready interrupt flag\r Reset by software by writing PLL1RDYC bit.\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.",
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
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag\r Reset by software by writing HSECSSC bit.\r Set by hardware in case of HSE clock failure.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "RCC clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable\r Set and cleared by software.\r Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source.\r This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
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
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag\r Set by hardware to indicate that the HSI oscillator is stable.",
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
                    name: "hsikeron",
                    description: Some(
                        "HSI clock enable in Stop mode\r Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION.",
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
                        "HSI clock divider\r Set and reset by software.\r These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The\r HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.",
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
                        "HSI divider flag\r Set and reset by hardware.\r As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the\r current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV.",
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
                        "CSI clock enable\r Set and reset by software to enable/disable CSI clock for system and/or peripheral.\r Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
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
                    name: "csirdy",
                    description: Some(
                        "CSI clock ready flag\r Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request).",
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
                    name: "csikeron",
                    description: Some(
                        "CSI clock enable in Stop mode\r Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION.",
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
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable\r Set by software and cleared by software or by the hardware when the system enters to Stop\r or Standby mode.",
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
                        "HSI48 clock ready flag\r Set by hardware to indicate that the HSI48 oscillator is stable.",
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
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE when entering Stop or Standby mode.\r This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the\r HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
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
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable.",
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
                        "HSE clock bypass\r Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device.\r The HSEBYP bit can be written only if the HSE oscillator is disabled.",
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
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable\r Set by software to enable clock security system on HSE.\r This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected.",
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
                    name: "hseext",
                    description: Some(
                        "external high speed clock type in Bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hseext",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable PLL1.\r Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents\r writing this bit to 0, if the PLL1 output is used as the system clock.",
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
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.",
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
                "RCC clock recovery RC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "Internal RC 48 MHz clock calibration\r Set by hardware by option-byte loading during system reset NRESET. Read-only.",
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
                "RCC CSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csical",
                    description: Some(
                        "CSI clock calibration\r Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM.\r This field represents the sum of engineering option byte calibration value and CSITRIM bits value.",
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
                        "CSI clock trimming\r Set by software to adjust calibration.\r CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value.\r CSICAL = CSITRIM + FLASH_CSI_OPT.\r Note: The reset value of the field is 0x20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hsicfgr",
            extends: None,
            description: Some(
                "RCC HSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM.\r This field represents the sum of engineering option byte calibration value and HSITRIM bits value.",
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
                        "HSI clock trimming\r Set by software to adjust calibration.\r HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value.\r HSICAL = HSITRIM + FLASH_HSI_OPT.\r After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated\r Note: The reset value of the field is 0x40.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
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
                "RCC PLL clock source selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "DIVMx and PLLs clock source selection\r Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled.\r In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset).",
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
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator.\r In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator.",
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
                    name: "pllvcosel",
                    description: Some(
                        "PLL1 VCO selection\r Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllvcosel",
                    ),
                },
                Field {
                    name: "divm",
                    description: Some(
                        "prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1.\r The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1).\r In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0.\r ...\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1_p_ck output of the PLL1.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.",
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
                    name: "pllqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1_q_ck output of the PLL1.\r In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                    name: "pllren",
                    description: Some(
                        "PLL1 DIVR divider output enable\r Set and reset by software to enable the pll1_r_ck output of the PLL1.\r To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
            ],
        },
        FieldSet {
            name: "Plldivr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r Others: reserved",
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
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1_p_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
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
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the pll1_q_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1_r_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
            name: "Pllfracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is:\r * 128 to 560 MHz if PLL1VCOSEL = 0\r * \t150 to 420 MHz if PLL1VCOSEL = 1\r VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with\r * \tPLL1N between 8 and 420\r * \tPLL1FRACN can be between 0 and 213- 1\r * \tThe input frequency Fref1_ck must be between 1 and 16 MHz.\r To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r * \tSet the bit PLL1FRACEN to 0\r * \tWrite the new fractional value into PLL1FRACN\r * \tSet the bit PLL1FRACEN to 1",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "RCC privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "RCC secure functions privilege configuration\r Set and reset by software. This bit can be written only by a secure privileged access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Spriv",
                    ),
                },
                Field {
                    name: "nspriv",
                    description: Some(
                        "RCC non-secure functions privilege configuration\r Set and reset by software. This bit can be written only by privileged access, secure or non-secure.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nspriv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rsr",
            extends: None,
            description: Some(
                "RCC reset status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "remove reset flag\r Set and reset by software to reset the value of the reset flags.",
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
                    name: "pinrstf",
                    description: Some(
                        "pin reset flag (NRST)\r Reset by software by writing the RMVF bit.\r Set by hardware when a reset from pin occurs.",
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
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a BOR reset occurs (pwr_bor_rst).",
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
                    name: "sftrstf",
                    description: Some(
                        "system reset from CPU reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33.",
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
                    name: "iwdgrstf",
                    description: Some(
                        "independent watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when an independent watchdog reset occurs.",
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
                    name: "wwdgrstf",
                    description: Some(
                        "window watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a window watchdog reset occurs.",
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
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag\r Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared.\r Cleared by writing to the RMVF bit.",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "RCC secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsisec",
                    description: Some(
                        "HSI clock configuration and status bits security\r Set and reset by software.",
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
                    name: "hsesec",
                    description: Some(
                        "HSE clock configuration bits, status bits and HSE_CSS security\r Set and reset by software.",
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
                    name: "csisec",
                    description: Some(
                        "CSI clock configuration and status bits security\r Set and reset by software.",
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
                    name: "lsisec",
                    description: Some(
                        "LSI clock configuration and status bits security\r Set and reset by software.",
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
                    name: "lsesec",
                    description: Some(
                        "LSE clock configuration and status bits security\r Set and reset by software.",
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
                    name: "sysclksec",
                    description: Some(
                        "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security\r Set and reset by software.",
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
                    name: "prescsec",
                    description: Some(
                        "AHBx/APBx prescaler configuration bits security\r Set and reset by software.",
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
                    name: "pllsec",
                    description: Some(
                        "PLL1 clock configuration and status bits security\r Set and reset by software.",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hsi48sec",
                    description: Some(
                        "HSI48 clock configuration and status bits security\r Set and reset by software.",
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
                    name: "rmvfsec",
                    description: Some(
                        "Remove reset flag security\r Set and reset by software.",
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
                    name: "perselsec",
                    description: Some(
                        "per_ck selection security\r Set and reset by software.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcdacsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK2",
                    description: Some(
                        "rcc_hclk selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "sys_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 5,
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
                        "lse_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "lsi_ker_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CSI_DIV_122",
                    description: Some(
                        "csi_ker_ck/122 selected as kernel clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Dacholdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DAC_HOLD",
                    description: Some(
                        "dac_hold_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DAC_HOLD_2",
                    description: Some(
                        "dac_hold_ck selected as kernel clock",
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
                        "hse_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 2,
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
                    description: Some(
                        "sys_ck not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "sys_ck divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "sys_ck divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "sys_ck divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "sys_ck divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "sys_ck divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "sys_ck divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "sys_ck divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "sys_ck divided by 512",
                    ),
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
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Division by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2c34sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as peripheral clock",
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
            name: "Lptim2sel",
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
                    name: "PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as peripheral clock",
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
            name: "Lpusartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lscosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 1,
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
            name: "Lseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "LSE in analog mode (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIGITAL",
                    description: Some(
                        "LSE in digital mode (do not use if RTC is active).",
                    ),
                    value: 1,
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
            name: "Nspriv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Read and write to RCC non-secure functions can be done by privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Read and write to RCC non-secure functions can be done by privileged access only",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Octospisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK4",
                    description: Some(
                        "rcc_hclk selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel clock",
                    ),
                    value: 3,
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
                        "hsi_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ck selected as kernel clock",
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
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 9,
            variants: &[
                EnumVariant {
                    name: "MUL4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "MUL5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "MUL6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "MUL7",
                    description: None,
                    value: 6,
                },
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
                EnumVariant {
                    name: "MUL421",
                    description: None,
                    value: 420,
                },
                EnumVariant {
                    name: "MUL422",
                    description: None,
                    value: 421,
                },
                EnumVariant {
                    name: "MUL423",
                    description: None,
                    value: 422,
                },
                EnumVariant {
                    name: "MUL424",
                    description: None,
                    value: 423,
                },
                EnumVariant {
                    name: "MUL425",
                    description: None,
                    value: 424,
                },
                EnumVariant {
                    name: "MUL426",
                    description: None,
                    value: 425,
                },
                EnumVariant {
                    name: "MUL427",
                    description: None,
                    value: 426,
                },
                EnumVariant {
                    name: "MUL428",
                    description: None,
                    value: 427,
                },
                EnumVariant {
                    name: "MUL429",
                    description: None,
                    value: 428,
                },
                EnumVariant {
                    name: "MUL430",
                    description: None,
                    value: 429,
                },
                EnumVariant {
                    name: "MUL431",
                    description: None,
                    value: 430,
                },
                EnumVariant {
                    name: "MUL432",
                    description: None,
                    value: 431,
                },
                EnumVariant {
                    name: "MUL433",
                    description: None,
                    value: 432,
                },
                EnumVariant {
                    name: "MUL434",
                    description: None,
                    value: 433,
                },
                EnumVariant {
                    name: "MUL435",
                    description: None,
                    value: 434,
                },
                EnumVariant {
                    name: "MUL436",
                    description: None,
                    value: 435,
                },
                EnumVariant {
                    name: "MUL437",
                    description: None,
                    value: 436,
                },
                EnumVariant {
                    name: "MUL438",
                    description: None,
                    value: 437,
                },
                EnumVariant {
                    name: "MUL439",
                    description: None,
                    value: 438,
                },
                EnumVariant {
                    name: "MUL440",
                    description: None,
                    value: 439,
                },
                EnumVariant {
                    name: "MUL441",
                    description: None,
                    value: 440,
                },
                EnumVariant {
                    name: "MUL442",
                    description: None,
                    value: 441,
                },
                EnumVariant {
                    name: "MUL443",
                    description: None,
                    value: 442,
                },
                EnumVariant {
                    name: "MUL444",
                    description: None,
                    value: 443,
                },
                EnumVariant {
                    name: "MUL445",
                    description: None,
                    value: 444,
                },
                EnumVariant {
                    name: "MUL446",
                    description: None,
                    value: 445,
                },
                EnumVariant {
                    name: "MUL447",
                    description: None,
                    value: 446,
                },
                EnumVariant {
                    name: "MUL448",
                    description: None,
                    value: 447,
                },
                EnumVariant {
                    name: "MUL449",
                    description: None,
                    value: 448,
                },
                EnumVariant {
                    name: "MUL450",
                    description: None,
                    value: 449,
                },
                EnumVariant {
                    name: "MUL451",
                    description: None,
                    value: 450,
                },
                EnumVariant {
                    name: "MUL452",
                    description: None,
                    value: 451,
                },
                EnumVariant {
                    name: "MUL453",
                    description: None,
                    value: 452,
                },
                EnumVariant {
                    name: "MUL454",
                    description: None,
                    value: 453,
                },
                EnumVariant {
                    name: "MUL455",
                    description: None,
                    value: 454,
                },
                EnumVariant {
                    name: "MUL456",
                    description: None,
                    value: 455,
                },
                EnumVariant {
                    name: "MUL457",
                    description: None,
                    value: 456,
                },
                EnumVariant {
                    name: "MUL458",
                    description: None,
                    value: 457,
                },
                EnumVariant {
                    name: "MUL459",
                    description: None,
                    value: 458,
                },
                EnumVariant {
                    name: "MUL460",
                    description: None,
                    value: 459,
                },
                EnumVariant {
                    name: "MUL461",
                    description: None,
                    value: 460,
                },
                EnumVariant {
                    name: "MUL462",
                    description: None,
                    value: 461,
                },
                EnumVariant {
                    name: "MUL463",
                    description: None,
                    value: 462,
                },
                EnumVariant {
                    name: "MUL464",
                    description: None,
                    value: 463,
                },
                EnumVariant {
                    name: "MUL465",
                    description: None,
                    value: 464,
                },
                EnumVariant {
                    name: "MUL466",
                    description: None,
                    value: 465,
                },
                EnumVariant {
                    name: "MUL467",
                    description: None,
                    value: 466,
                },
                EnumVariant {
                    name: "MUL468",
                    description: None,
                    value: 467,
                },
                EnumVariant {
                    name: "MUL469",
                    description: None,
                    value: 468,
                },
                EnumVariant {
                    name: "MUL470",
                    description: None,
                    value: 469,
                },
                EnumVariant {
                    name: "MUL471",
                    description: None,
                    value: 470,
                },
                EnumVariant {
                    name: "MUL472",
                    description: None,
                    value: 471,
                },
                EnumVariant {
                    name: "MUL473",
                    description: None,
                    value: 472,
                },
                EnumVariant {
                    name: "MUL474",
                    description: None,
                    value: 473,
                },
                EnumVariant {
                    name: "MUL475",
                    description: None,
                    value: 474,
                },
                EnumVariant {
                    name: "MUL476",
                    description: None,
                    value: 475,
                },
                EnumVariant {
                    name: "MUL477",
                    description: None,
                    value: 476,
                },
                EnumVariant {
                    name: "MUL478",
                    description: None,
                    value: 477,
                },
                EnumVariant {
                    name: "MUL479",
                    description: None,
                    value: 478,
                },
                EnumVariant {
                    name: "MUL480",
                    description: None,
                    value: 479,
                },
                EnumVariant {
                    name: "MUL481",
                    description: None,
                    value: 480,
                },
                EnumVariant {
                    name: "MUL482",
                    description: None,
                    value: 481,
                },
                EnumVariant {
                    name: "MUL483",
                    description: None,
                    value: 482,
                },
                EnumVariant {
                    name: "MUL484",
                    description: None,
                    value: 483,
                },
                EnumVariant {
                    name: "MUL485",
                    description: None,
                    value: 484,
                },
                EnumVariant {
                    name: "MUL486",
                    description: None,
                    value: 485,
                },
                EnumVariant {
                    name: "MUL487",
                    description: None,
                    value: 486,
                },
                EnumVariant {
                    name: "MUL488",
                    description: None,
                    value: 487,
                },
                EnumVariant {
                    name: "MUL489",
                    description: None,
                    value: 488,
                },
                EnumVariant {
                    name: "MUL490",
                    description: None,
                    value: 489,
                },
                EnumVariant {
                    name: "MUL491",
                    description: None,
                    value: 490,
                },
                EnumVariant {
                    name: "MUL492",
                    description: None,
                    value: 491,
                },
                EnumVariant {
                    name: "MUL493",
                    description: None,
                    value: 492,
                },
                EnumVariant {
                    name: "MUL494",
                    description: None,
                    value: 493,
                },
                EnumVariant {
                    name: "MUL495",
                    description: None,
                    value: 494,
                },
                EnumVariant {
                    name: "MUL496",
                    description: None,
                    value: 495,
                },
                EnumVariant {
                    name: "MUL497",
                    description: None,
                    value: 496,
                },
                EnumVariant {
                    name: "MUL498",
                    description: None,
                    value: 497,
                },
                EnumVariant {
                    name: "MUL499",
                    description: None,
                    value: 498,
                },
                EnumVariant {
                    name: "MUL500",
                    description: None,
                    value: 499,
                },
                EnumVariant {
                    name: "MUL501",
                    description: None,
                    value: 500,
                },
                EnumVariant {
                    name: "MUL502",
                    description: None,
                    value: 501,
                },
                EnumVariant {
                    name: "MUL503",
                    description: None,
                    value: 502,
                },
                EnumVariant {
                    name: "MUL504",
                    description: None,
                    value: 503,
                },
                EnumVariant {
                    name: "MUL505",
                    description: None,
                    value: 504,
                },
                EnumVariant {
                    name: "MUL506",
                    description: None,
                    value: 505,
                },
                EnumVariant {
                    name: "MUL507",
                    description: None,
                    value: 506,
                },
                EnumVariant {
                    name: "MUL508",
                    description: None,
                    value: 507,
                },
                EnumVariant {
                    name: "MUL509",
                    description: None,
                    value: 508,
                },
                EnumVariant {
                    name: "MUL510",
                    description: None,
                    value: 509,
                },
                EnumVariant {
                    name: "MUL511",
                    description: None,
                    value: 510,
                },
                EnumVariant {
                    name: "MUL512",
                    description: None,
                    value: 511,
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
                    name: "DISABLE",
                    description: Some(
                        "no clock send to DIVMx divider and PLLs (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL clock (hsi_ck)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as PLL clock (csi_ck)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL clock (hse_ck)",
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
                        "VCO frequency range 192 to 836 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_VCO",
                    description: Some(
                        "VCO frequency range 150 to 420 MHz",
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
                        "rcc_pclk3 = rcc_hclk1 / 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "hsi48_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "lsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
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
                        "no clock (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE_DIV_RTCPRE",
                    description: Some(
                        "HSE divided by RTCPRE value selected as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Saisel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "AUDIOCLK selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spi1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "AUDIOCLK selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Spi2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "AUDIOCLK selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Spi3sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "AUDIOCLK selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "per_ck selected as kernel clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Spi4sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "rcc_pclk2 selected as kernel clock (default after reset)",
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
            name: "Spi5sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as kernel clock (default after reset)",
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
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Spriv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANY",
                    description: Some(
                        "Read and write to RCC secure functions can be done by privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRIVILEGED",
                    description: Some(
                        "Read and write to RCC secure functions can be done by privileged access only",
                    ),
                    value: 1,
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
                        "HSI selected as wakeup clock from system Stop (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
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
                    name: "CSI",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
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
            name: "Systicksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK1_DIV_8",
                    description: Some(
                        "rcc_hclk/8 selected as clock source (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "lsi_ker_ck[1] selected as clock source",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck[1] selected as clock source",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Timicsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No internal clock available for timers input capture (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture",
                    ),
                    value: 1,
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
                        "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEFAULT_X4",
                    description: Some(
                        "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2",
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
            name: "Usartsel",
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
            name: "Usbsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "Disable the kernel clock",
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
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
