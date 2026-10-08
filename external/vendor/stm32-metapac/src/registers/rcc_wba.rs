
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control",
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
                    name: "icscr3",
                    description: Some(
                        "RCC internal clock sources calibration register 3",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "RCC clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "cfgr2",
                    description: Some(
                        "RCC clock configuration register 2",
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
                    name: "cfgr3",
                    description: Some(
                        "RCC clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1cfgr",
                    description: Some(
                        "RCC PLL1 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1divr",
                    description: Some(
                        "RCC PLL1 dividers register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1divr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1fracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1fracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock interrupt enable register",
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
                        "RCC clock interrupt flag register",
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
                        "RCC clock interrupt clear register",
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
                        "RCC AHB1 peripheral reset register",
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
                    name: "ahb5rstr",
                    description: Some(
                        "RCC AHB5 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x70,
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
                    name: "apb1rstr1",
                    description: Some(
                        "RCC APB1 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 0x74,
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
                        "RCC APB1 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 0x78,
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
                    name: "apb7rstr",
                    description: Some(
                        "RCC APB7 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripheral clock enable register",
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
                        "RCC AHB2 peripheral clock enable register",
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
                        "RCC AHB4 peripheral clock enable register",
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
                    name: "ahb5enr",
                    description: Some(
                        "RCC AHB5 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x98,
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
                    name: "apb1enr1",
                    description: Some(
                        "RCC APB1 peripheral clock enable register 1",
                    ),
                    array: None,
                    byte_offset: 0x9c,
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
                        "RCC APB1 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 0xa0,
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
                        "RCC APB2 peripheral clock enable register",
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
                    name: "apb7enr",
                    description: Some(
                        "RCC APB7 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1smenr",
                    description: Some(
                        "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2smenr",
                    description: Some(
                        "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb4smenr",
                    description: Some(
                        "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb5smenr",
                    description: Some(
                        "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr1",
                    description: Some(
                        "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr2",
                    description: Some(
                        "RCC APB1 peripheral clocks enable in Sleep and Stop modes \tregister 2",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2smenr",
                    description: Some(
                        "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb7smenr",
                    description: Some(
                        "RCC APB7 peripheral clock enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC peripherals independent clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0xe0,
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
                        "RCC peripherals independent clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0xe4,
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
                        "RCC peripherals independent clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0xe8,
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
                    name: "bdcr",
                    description: Some(
                        "RCC backup domain control register",
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
                    name: "csr",
                    description: Some(
                        "RCC control/status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
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
                    name: "bdcr2",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr2",
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
                BlockItem {
                    name: "ascr",
                    description: Some(
                        "RCC audio synchronization control register",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "asier",
                    description: Some(
                        "RCC audio synchronization interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Asier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "assr",
                    description: Some(
                        "RCC audio synchronization status register",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Assr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascntr",
                    description: Some(
                        "RCC audio synchronization counter register",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "asarr",
                    description: Some(
                        "RCC audio synchronization auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x1d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Asarr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascar",
                    description: Some(
                        "RCC audio synchronization capture register",
                    ),
                    array: None,
                    byte_offset: 0x1d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascor",
                    description: Some(
                        "RCC audio synchronization compare register",
                    ),
                    array: None,
                    byte_offset: 0x1d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr4",
                    description: Some(
                        "RCC clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "radioenr",
                    description: Some(
                        "RCC RADIO peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Radioenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecscr1",
                    description: Some(
                        "RCC external clock sources calibration register 1",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecscr1",
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
                "RCC AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 bus clock enable\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "flashen",
                    description: Some(
                        "FLASH bus clock enable\r Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.\r Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "CRC bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tscen",
                    description: Some(
                        "Touch sensing controller bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gtzc1en",
                    description: Some(
                        "GTZC1 bus clock enable \r Set and reset by software.\r Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram1en",
                    description: Some(
                        "SRAM1 bus clock enable \r Set and reset by software.\r Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 reset\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "crcrst",
                    description: Some(
                        "CRC reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tscrst",
                    description: Some(
                        "TSC reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb1smenr",
            extends: None,
            description: Some(
                "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1smen",
                    description: Some(
                        "GPDMA1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "flashsmen",
                    description: Some(
                        "FLASH bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "crcsmen",
                    description: Some(
                        "CRC bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tscsmen",
                    description: Some(
                        "TSC bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                    name: "ramcfgsmen",
                    description: Some(
                        "RAMCFG bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gtzc1smen",
                    description: Some(
                        "GTZC1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "icachesmen",
                    description: Some(
                        "ICACHE bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "IO port A bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port B bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port C bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port D bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port E bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogen",
                    description: Some(
                        "IO port G bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port H bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hsen",
                    description: Some(
                        "USB OTG_HS bus and kernel clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hs_phyen",
                    description: Some(
                        "USB OTG_HS PHY kernel clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "aesen",
                    description: Some(
                        "AES bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HASH bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "RNG bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saesen",
                    description: Some(
                        "SAES bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsemen",
                    description: Some(
                        "HSEM bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkaen",
                    description: Some(
                        "PKA bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram2en",
                    description: Some(
                        "SRAM2 bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port B reset\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port C reset\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port D reset\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port E reset\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port H reset\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hsrst",
                    description: Some(
                        "USB OTG_HS reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "aesrst",
                    description: Some(
                        "AES hardware accelerator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Hash reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Random number generator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saesrst",
                    description: Some(
                        "SAES hardware accelerator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsemrst",
                    description: Some(
                        "HSEM hardware accelerator reset\r Set and cleared by software.\r Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkarst",
                    description: Some(
                        "PKA reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb2smenr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiobsmen",
                    description: Some(
                        "IO port B bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiocsmen",
                    description: Some(
                        "IO port C bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiodsmen",
                    description: Some(
                        "IO port D bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpioesmen",
                    description: Some(
                        "IO port E bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogsmen",
                    description: Some(
                        "IO port G bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiohsmen",
                    description: Some(
                        "IO port H bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hssmen",
                    description: Some(
                        "USB OTG_HS bus and kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hs_physmen",
                    description: Some(
                        "USB OTG_HS PHY kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "aessmen",
                    description: Some(
                        "AES bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hashsmen",
                    description: Some(
                        "HASH bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "rngsmen",
                    description: Some(
                        "Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saessmen",
                    description: Some(
                        "SAES accelerator bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkasmen",
                    description: Some(
                        "PKA bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb4enr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwren",
                    description: Some(
                        "PWR bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "adc4en",
                    description: Some(
                        "ADC4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "adc4rst",
                    description: Some(
                        "ADC4 reset\r Set and cleared by software.\r Access can be secred by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            ],
        },
        FieldSet {
            name: "Ahb4smenr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrsmen",
                    description: Some(
                        "PWR bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "adc4smen",
                    description: Some(
                        "ADC4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            ],
        },
        FieldSet {
            name: "Ahb5enr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radioen",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked.\r Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).",
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
                    name: "ptaconven",
                    description: Some(
                        "PTACONV bus clock enable",
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
            name: "Ahb5rstr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radiorst",
                    description: Some(
                        "2.4 GHz RADIO reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ptaconvrst",
                    description: Some(
                        "PTACONV reset.",
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
            name: "Ahb5smenr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radiosmen",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active.\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ptaconvsmen",
                    description: Some(
                        "PTACONV bus clock enable during Sleep and Stop modes.",
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
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "wwdgen",
                    description: Some(
                        "WWDG bus clock enable\r Set by software to enable the window watchdog bus clock. Reset by hardware system reset.\r This bit can also be set by hardware if the WWDG_SW option bit is reset.\r Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2en",
                    description: Some(
                        "USART2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                        "USART3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART3SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                    name: "i2c1en",
                    description: Some(
                        "I2C1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4en",
                    description: Some(
                        "I2C4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            ],
        },
        FieldSet {
            name: "Apb1rstr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM4 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2rst",
                    description: Some(
                        "USART2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "USART3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC UART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset.",
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
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            ],
        },
        FieldSet {
            name: "Apb1smenr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim3smen",
                    description: Some(
                        "TIM3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim4smen",
                    description: Some(
                        "TIM4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "wwdgsmen",
                    description: Some(
                        "Window watchdog bus clock enable during Sleep and Stop modes\r Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.\r Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi2smen",
                    description: Some(
                        "SPI2 bus and kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2smen",
                    description: Some(
                        "USART2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "usart3smen",
                    description: Some(
                        "USART3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "i2c2smen",
                    description: Some(
                        "I2C2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            name: "Apb1smenr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clocks enable in Sleep and Stop modes \tregister 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4smen",
                    description: Some(
                        "I2C4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lptim2smen",
                    description: Some(
                        "LPTIM2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart1en",
                    description: Some(
                        "USART1bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim16en",
                    description: Some(
                        "TIM16 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM17 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sai1en",
                    description: Some(
                        "SAI1 bus and kernel clocks enable",
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
                        "TIM1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim16rst",
                    description: Some(
                        "TIM16 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM17 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sai1rst",
                    description: Some(
                        "SAI1 reset.",
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
            name: "Apb2smenr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1smen",
                    description: Some(
                        "TIM1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi1smen",
                    description: Some(
                        "SPI1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "usart1smen",
                    description: Some(
                        "USART1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "tim16smen",
                    description: Some(
                        "TIM16 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim17smen",
                    description: Some(
                        "TIM17 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sai1smen",
                    description: Some(
                        "SAI1 bus and kernel clocks enable during Sleep and Stop modes.",
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
            name: "Apb7enr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG bus clock enable\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3en",
                    description: Some(
                        "SPI3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LPUART1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "compen",
                    description: Some(
                        "COMP bus clock enable",
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
                        "VREFBUF bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "RTC and TAMP bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb7rstr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SYSCFG reset\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LPUART1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "comprst",
                    description: Some(
                        "COMP reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC COMPSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "VREF reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb7smenr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral clock enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsmen",
                    description: Some(
                        "SYSCFG bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3smen",
                    description: Some(
                        "SPI3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lpuart1smen",
                    description: Some(
                        "LPUART1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lptim1smen",
                    description: Some(
                        "LPTIM1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "compsmen",
                    description: Some(
                        "COMP bus clock enable during Sleep and Stop modes.",
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
                    name: "vrefsmen",
                    description: Some(
                        "VREFBUF clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC and TAMP APB clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            name: "Asarr",
            extends: None,
            description: Some(
                "RCC audio synchronization auto-reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ar",
                    description: Some(
                        "Auto-reload value\r This field is set by software.\r CA[19:0] is the counter auto-reload value at which to restart the audio synchronization counter from value 0. It defines the counter period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascar",
            extends: None,
            description: Some(
                "RCC audio synchronization capture register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ca",
                    description: Some(
                        "Capture value\r This field is set by hardware.\r CA[26:20] is the capture period counter value loaded on the trigger event. CA[19:0] is the audio synchronization counter value loaded on the trigger event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascntr",
            extends: None,
            description: Some(
                "RCC audio synchronization counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value\r This field is set by hardware.\r CNT[19:0] is the counter value at the time this register is read.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascor",
            extends: None,
            description: Some(
                "RCC audio synchronization compare register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "co",
                    description: Some(
                        "Compare value\r This field is set by software.\r CO[19:0] is the value to be compared to the audio synchronization counter to generate an compare interrupt event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascr",
            extends: None,
            description: Some(
                "RCC audio synchronization control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable\r This bit is set and cleared by software.\r Clearing this bit will reset the audio synchronization counter and capture prescaler and all associated registers ASCR, ASIER, ASSR, ASCNTR, ASARR, ASCAR, and ASCOR.",
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
                    name: "psc",
                    description: Some(
                        "Clock prescaler\r This field is set and cleared by software.\r Counter clock frequency = f_audiosync_ker_ck / (PSC + 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cps",
                    description: Some(
                        "Capture prescaler\r This field is set and cleared by software.\r Capture period in number of counter periods. Capture period = counter period * (TPS + 1)",
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
            name: "Asier",
            extends: None,
            description: Some(
                "RCC audio synchronization interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caie",
                    description: Some(
                        "Capture trigger interrupt enable\r This bit is set and cleared by software.",
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
                    name: "coie",
                    description: Some(
                        "Comparer interrupt enable\r This field is set and cleared by software.",
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
                    name: "caeie",
                    description: Some(
                        "Capture error interrupt enable\r This field is set and cleared by software.",
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
            ],
        },
        FieldSet {
            name: "Assr",
            extends: None,
            description: Some(
                "RCC audio synchronization status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caf",
                    description: Some(
                        "Capture trigger interrupt flag\r This field is set by hardware, only when CAIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAIE is 0.",
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
                    name: "cof",
                    description: Some(
                        "Comparer interrupt flag\r This field is set by hardware, only when COIE is enabled. This bit is cleared by software by writing it to 0 or masked when COIE is 0.",
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
                    name: "caef",
                    description: Some(
                        "Capture error interrupt flag\r This field is set by hardware, only when CAEIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAEIE is 0.",
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
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable\r Set and cleared by software.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator ready\r Set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator bypass\r Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator drive capability\r Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in ‘Xtal’ mode.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.",
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
                        "Low speed external clock security enable\r Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable the LSECSSON bit.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Low speed external clock security, LSE failure Detection\r Set by hardware to indicate when a failure is detected by the LSECCS on the external 32�kHz oscillator.\r Reset when LSCSSON bit is cleared.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsesysen",
                    description: Some(
                        "LSE system clock (LSESYS) enable\r Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "RTC and TAMP kernel clock source enable and selection\r Set by software to enable and select the clock source for the RTC.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsesysrdy",
                    description: Some(
                        "LSE system clock (LSESYS) ready\r Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles.\r The LSE clock must be already enabled and stable (LSEON and LSERDY are set). \r When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsegfon",
                    description: Some(
                        "LSE clock glitch filter enable\r Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsetrim",
                    description: Some(
                        "LSE trimming\r These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value.\r Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY�= 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsetrim",
                    ),
                },
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "radiostsel",
                    description: Some(
                        "2.4 GHz RADIO sleep timer kernel clock enable and selection\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Radiostsel",
                    ),
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.\r Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Low-speed clock output selection\r Set and cleared by software.\r Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1on",
                    description: Some(
                        "LSI1 oscillator enable\r Set and cleared by software.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1rdy",
                    description: Some(
                        "LSI1 oscillator ready\r Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1prediv",
                    description: Some(
                        "LSI1 Low-speed clock divider configuration\r Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsiprediv",
                    ),
                },
                Field {
                    name: "lsi2on",
                    description: Some(
                        "LSI2 oscillator enable",
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
                    name: "lsi2rdy",
                    description: Some(
                        "LSI2 oscillator ready.",
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
            name: "Bdcr2",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi2mode",
                    description: Some(
                        "LSI2 oscillator operating mode configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lsi2mode",
                    ),
                },
                Field {
                    name: "lsi2cfg",
                    description: Some(
                        "LSI2 oscillator configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Lsi2cfg",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r This bits are used to select the USART1 kernel clock source.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 kernel clock source selection\r This bits are used to select the USART2 kernel clock source.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 kernel clock source selection\r This bits are used to select the USART3 kernel clock source.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection\r These bits are used to select the I2C1 kernel clock source.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1sel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 kernel clock source selection\r These bits are used to select the I2C2 kernel clock source.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1sel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 kernel clock source selection\r These bits are used to select the I2C4 kernel clock source.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1sel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r These bits are used to select the SPI2 kernel clock source.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi2sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "Low-power timer 2 kernel clock source selection\r These bits are used to select the LPTIM2 kernel clock source.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r These bits are used to select the SPI1 kernel clock source.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi1sel",
                    ),
                },
                Field {
                    name: "systicksel",
                    description: Some(
                        "SysTick clock source selection\r These bits are used to select the SysTick clock source.\r Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture \r When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI/256. \r When TIMICSEL is cleared, the HSI, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture.\r Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.",
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
                "RCC peripherals independent clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 kernel clock source selection\r These bits allow to select the SAI1 kernel clock source.\r Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sai1sel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNGSEL kernel clock source selection\r These bits allow to select the RNG kernel clock source.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "otghssel",
                    description: Some(
                        "USB OTG_HS PHY kernel clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Otghssel",
                    ),
                },
                Field {
                    name: "assel",
                    description: Some(
                        "RCC audio synchronization kernel clock source selection\r This bit allows to select the audio synchronization kernel clock source.\r Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Assel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r These bits are used to select the LPUART1 kernel clock source.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPUART1 is functional in Stop modes only when the kernel clock is HSI or LSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r These bits are used to select the SPI3 kernel clock source.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI3 is functional in Stop modes only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi3sel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 kernel clock source selection\r These bits are used to select the I2C3 kernel clock source.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C3 is functional in Stop modes only when the kernel clock is HSI",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c3sel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection\r These bits are used to select the LPTIM1 kernel clock source.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPTIM1 is functional in Stop modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "ADC4 kernel clock source selection\r These bits are used to select the ADC4 kernel clock source.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r others: reserved\r Note: The ADC4 is functional in Stop modes only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "RCC clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock switch\r Set and cleared by software to select system clock source (SYSCLK).\r Cleared by hardware when entering Stop and Standby modes\r When selecting HSE directly or indirectly as system clock and HSE oscillator clock security fails, cleared by hardware.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "system clock switch status\r Set and cleared by hardware to indicate which clock source is used as system clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "microcontroller clock output\r Set and cleared by software.\r others: reserved\r Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "mcopre",
                    description: Some(
                        "microcontroller clock output prescaler\r Set and cleared by software.\r It is highly recommended to change this prescaler before MCO output is enabled.\r others: not allowed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB1, AHB2 and AHB4 prescaler\r Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1).\r The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.\r 0xx: hclk1 = SYSCLK not divided",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB1 prescaler\r Set and cleared by software to control the division factor of the APB1 clock (pclk1).\r 0xx: pclk1 = hclk1 not divided",
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
                        "APB2 prescaler\r Set and cleared by software to control the division factor of the APB2 clock (pclk2).\r 0xx: pclk2 = hclk1 not divided",
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
            ],
        },
        FieldSet {
            name: "Cfgr3",
            extends: None,
            description: Some(
                "RCC clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppre7",
                    description: Some(
                        "APB7 prescaler\r Set and cleared by software to control the division factor of the APB7 clock (pclk7).\r 0xx: hclk1 not divided",
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
            ],
        },
        FieldSet {
            name: "Cfgr4",
            extends: None,
            description: Some(
                "RCC clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre5",
                    description: Some(
                        "AHB5 prescaler when SWS select PLL1\r Set and cleared by software to control the division factor of the AHB5 clock (hclk5).\r Must not be changed when SYSCLK source indicated by SWS is PLL1.\r When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account.\r When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs\r Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.\r 0xx: hclk5 = SYSCLK not divided",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hpre5",
                    ),
                },
                Field {
                    name: "hdiv5",
                    description: Some(
                        "AHB5 divider when SWS select HSI or HSE\r Set and reset by software.\r Set to 1 by hardware when entering Stop 1 mode.\r When SYSCLK source indicated by SWS is HSI or HSE: HDIV5 is taken into account\r When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account\r Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hdiv5",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyc",
                    description: Some(
                        "LSI1 ready interrupt clear\r Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt clear\r Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt clear\r Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.\\\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE ready interrupt clear\r Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pllrdyc",
                    description: Some(
                        "PLL1 ready interrupt clear\r Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecssc",
                    description: Some(
                        "High speed external clock security system interrupt clear\r Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyc",
                    description: Some(
                        "LSI2 ready interrupt clear.",
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
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyie",
                    description: Some(
                        "LSI1 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pllrdyie",
                    description: Some(
                        "PLL1 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by PLL1 lock.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyie",
                    description: Some(
                        "LSI2 ready interrupt enable",
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
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyf",
                    description: Some(
                        "LSI1 ready interrupt flag\r Set by hardware when the LSI1 clock becomes stable and LSI1RDYIE is set.\r Cleared by software setting the LSI1RDYC bit.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt flag\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.\r Cleared by software setting the LSERDYC bit.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt flag\r Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Cleared by software setting the HSIRDYC bit.",
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
                        "HSE ready interrupt flag\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.\r Cleared by software setting the HSERDYC bit.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pllrdyf",
                    description: Some(
                        "PLL1 ready interrupt flag\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.\r Cleared by software setting the PLL1RDYC bit.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag\r Set by hardware when a clock security failure is detected in the HSE oscillator.\r Cleared by software setting the HSECSSC bit.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyf",
                    description: Some(
                        "LSI2 ready interrupt flag.",
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
                        "HSI clock enable\r Set and cleared by software.\r Cleared by hardware when entering Stop and Standby modes. \r Set by hardware to force the HSI oscillator on when exiting Stop and Standby modes.\r Set by hardware to force the HSI oscillator on in case of clock security failure of the HSE crystal oscillator.\r This bit is set by hardware if the HSI is used directly or indirectly as system clock.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsikeron",
                    description: Some(
                        "HSI enable for some peripheral kernels\r Set and cleared by software to force HSI oscillator on even in Stop modes. \r Keeping the HSI oscillator on in Stop modes allows the communication speed not to be reduced by the HSI oscillator startup time. This bit has no effect on register bit HSION value.\r Cleared by hardware when entering Standby modes. \r Refer to Peripherals clock gating and autonomous mode for more details.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag\r Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles.",
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
                    name: "hseon",
                    description: Some(
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE clock for the CPU when entering Stop and Standby modes and on a HSECSS failure.\r When the HSE is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode.\r This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable. This bit is set both when HSE is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable\r Set by software to enable the HSE clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsepre",
                    description: Some(
                        "HSE clock for SYSCLK prescaler\r Set and cleared by software to control the division factor of the HSE clock for SYSCLK.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsepre",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable the main PLL.\r Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE is selected as sysclk, on a HSECSS failure.\r This bit cannot be reset if the PLL1 clock is used as the system clock.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "RCC control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag\r Set by software to clear the reset flags.\r Access can be secured by RCC RMVFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "oblrstf",
                    description: Some(
                        "Option byte loader reset flag\r Set by hardware when a reset from the option byte loading occurs.\r Cleared by writing to the RMVF bit.",
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
                    name: "pinrstf",
                    description: Some(
                        "NRST pin reset flag\r Set by hardware when a reset from the NRST pin occurs.\r Cleared by writing to the RMVF bit.",
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
                        "BOR flag\r Set by hardware when a BOR occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Software reset flag\r Set by hardware when a software reset occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Independent watchdog reset flag\r Set by hardware when an independent watchdog reset domain occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Window watchdog reset flag\r Set by hardware when a window watchdog reset occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Low-power reset flag\r Set by hardware when a reset occurs due to illegal Stop and Standby modes entry.\r Cleared by writing to the RMVF bit.",
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
            name: "Ecscr1",
            extends: None,
            description: Some(
                "RCC external clock sources calibration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsetrim",
                    description: Some(
                        "HSE clock trimming \r These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE oscillator frequency.",
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
            name: "Icscr3",
            extends: None,
            description: Some(
                "RCC internal clock sources calibration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r These bits are initialized at startup with the factory-programmed HSI calibration value. When HSITRIM[4:0] is written, HSICAL[11:0] is updated with the sum of HSITRIM[4:0] and the initial factory trim value.",
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
                        "HSI clock trimming \r These bits provide an additional user-programmable trimming value that is added to the HSICAL[11:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll1cfgr",
            extends: None,
            description: Some(
                "RCC PLL1 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL1 entry clock source\r Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled.\r Cleared by hardware when entering Stop or Standby modes. \r Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.",
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
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1.\r This bit must be written before enabling the PLL1.\r 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz",
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
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of PLL1FRACN into the ΣΔ modulator.\r In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set\u{a0}to\u{a0}0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).",
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
                    name: "pllm",
                    description: Some(
                        "Prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). \r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1pclk output of the PLL1.\r To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. \r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1qclk output of the PLL1.\r To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. \r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                        "PLL1 DIVR divider output enable\r Set and cleared by software to enable the pll1rclk output of the PLL1.\r To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                    name: "pllrclkpre",
                    description: Some(
                        "pll1rclk clock for SYSCLK prescaler division enable\r Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllrclkpre",
                    ),
                },
                Field {
                    name: "pllrclkprestep",
                    description: Some(
                        "pll1rclk clock for SYSCLK prescaler division step selection\r Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllrclkprestep",
                    ),
                },
                Field {
                    name: "pllrclkprerdy",
                    description: Some(
                        "pll1rclkpre not divided ready.\r Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre.",
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
            name: "Pll1divr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1 VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r others: reserved\r VCO output frequency = F<sub>ref1_ck</sub> x multiplication factor for PLL1 VCO, when fractional value 0 has been loaded into PLL1FRACN, with: \r Multiplication factor for PLL1 VCO between 4 and 512\r input frequency F<sub>ref1_ck</sub> between 4 and 16�MHz",
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
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1pclk clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
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
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the PLl1QCLK clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1rclk clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
            name: "Pll1fracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO.\r These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r VCO output frequency = F<sub>ref1_ck</sub> x [multiplication factor for PLL1 VCO + (PLL1FRACN / 2<sup>13</sup>)], with: \r Multiplication factor for PLL1 VCO must be between 4 and 512.\r PLL1FRACN can be between 0 and 2<sup>13</sup>- 1.\r The input frequency F<sub>ref1_ck</sub> must be between 4 and 16 MHz. \r To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows:\r Set the bit PLL1FRACEN to 0. \r Write the new fractional value into PLL1FRACN. \r Set the bit PLL1FRACEN to 1.",
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
                        "RCC secure functions privilege configuration\r Set and reset by software.\r This bit can be written only by a secure privileged access.",
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
                    name: "nspriv",
                    description: Some(
                        "RCC non-secure functions privilege configuration\r Set and reset by software.\r This bit can be written only by privileged access, secure or non-secure.",
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
            name: "Radioenr",
            extends: None,
            description: Some(
                "RCC RADIO peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bbclken",
                    description: Some(
                        "2.4 GHz RADIO baseband kernel clock (aclk) enable\r Set and cleared by software.\r Note: The HSE oscillator needs to be enabled by either HSEON or STRADIOCLKON.",
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
                    name: "stradioclkon",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable and HSE oscillator enable by 2.4 GHz RADIO sleep timer wakeup event\r Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event.\r Cleared by software writing zero to this bit.\r Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.",
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
                    name: "radioclkrdy",
                    description: Some(
                        "2.4 GHz RADIO bus clock ready.\r Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed.\r Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles.",
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
                        "HSE clock configuration bits, status bits and HSECSS security\r Set and reset by software.",
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
                        "SYSCLK selection, clock output on MCO configuration security\r Set and reset by software.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK4",
                    description: Some(
                        "hclk4 clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1pclk selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Assel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1pclk selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1qclk selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hdiv5",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "hclk5 = SYSCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "hclk5 = SYSCLK divided by 2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "hclk = SYSCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "hclk = SYSCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "hclk = SYSCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "hclk = SYSCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Hpre5",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "hclk5 = SYSCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "hclk5 = SYSCLK divided by 3",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "hclk5 = SYSCLK divided by 4",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "hclk5 = SYSCLK divided by 6",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Hsepre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HSE not divided, SYSCLK = HSE",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HSE divided, SYSCLK = HSE/2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "I2c3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK7",
                    description: Some(
                        "pclk7 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptim2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk7 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
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
            name: "Lsetrim",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "R5_4",
                    description: Some(
                        "current source resistance 5/4 x R",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "R",
                    description: Some(
                        "current source resistance R",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "R3_4",
                    description: Some(
                        "current source resistance 3/4 x R",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "R2_3",
                    description: Some(
                        "current source resistance 2/3 x R",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lsi2cfg",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "SENSITIVITY0AT80C",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +80 less thansup oless than/sup C.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SENSITIVITY0AT50C",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +50 less thansup oless than/sup C.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SENSITIVITY0AT20C",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +20 less thansup oless than/sup C.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lsi2mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NOMINAL_POWER",
                    description: Some(
                        "nominal-power, high accuracy.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_POWER",
                    description: Some(
                        "low-power, medium accuracy.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ULTRA_LOW_POWER",
                    description: Some(
                        "ultra-low-power, low accuracy.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lsiprediv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "LSI not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "LSI divided by 128",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "MCO divided by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "MCO divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "MCO divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "MCO divided by 8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "MCO divided by 16",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MCO output disabled, no clock on MCO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLKPRE",
                    description: Some(
                        "sysclkpre system clock after PLL1RCLKPRE division selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "pll1rclk clock selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1pclk clock selected",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1qclk clock selected",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "HCLK5",
                    description: Some(
                        "hclk5 clock selected",
                    ),
                    value: 10,
                },
            ],
        },
        Enum {
            name: "Otghssel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE32 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1pclk selected,.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE_DIV_2",
                    description: Some(
                        "HSE32 divided by 2 selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P_DIV_2",
                    description: Some(
                        "pll1pclk divided by 2 selected.",
                    ),
                    value: 3,
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
            name: "Pllrclkpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "pll1rclk not divided, sysclkpre = pll1rclk",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIVIDED",
                    description: Some(
                        "pll1rclk divided, sysclkpre = pll1rclk divided",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllrclkprestep",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STEP2",
                    description: Some(
                        "pll1rclk 2-step division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STEP3",
                    description: Some(
                        "pll1rclk 3-step division",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FREQ_4TO8MHZ",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FREQ_8TO16MHZ",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz",
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
                        "no clock sent to PLL1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected as PLL1 clock entry",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock after HSEPRE divider selected as PLL1 clock entry",
                    ),
                    value: 3,
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
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Radiostsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "no clock selected, 2.4 GHz RADIO sleep timer kernel clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by 1000 selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1qclk divide by 2 selected",
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
                        "no clock selected, RTC and TAMP kernel clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected, and enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock selected, and enabled",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by 32 selected, and enabled",
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
                    name: "PLL1_P",
                    description: Some(
                        "pll1pclk selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1qclk selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "input pin AUDIOCLK selected.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI16 clock selected.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Spi1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "pclk2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Spi2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Spi3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE or HSE/2, as defined by HSEPRE, selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "pll1rclk selected as system clock",
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
                        "hclk1 divided by 8 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
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
                    name: "HSI",
                    description: Some(
                        "HSI divider disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI_DIV_256",
                    description: Some(
                        "HSI/256 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "pclk2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
