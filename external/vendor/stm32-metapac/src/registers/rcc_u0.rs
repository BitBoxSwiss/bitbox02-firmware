
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "RCC address block description.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Clock control register.",
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
                    name: "icscr",
                    description: Some(
                        "Internal clock sources calibration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Clock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "pllcfgr",
                    description: Some(
                        "PLL configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "cier",
                    description: Some(
                        "Clock interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                        "Clock interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                        "Clock interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "ahbrstr",
                    description: Some(
                        "AHB peripheral reset register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahbrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpiorstr",
                    description: Some(
                        "I/O port reset register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpiorstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbrstr1",
                    description: Some(
                        "APB peripheral reset register 1.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbrstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbrstr2",
                    description: Some(
                        "APB peripheral reset register 2.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbrstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbenr",
                    description: Some(
                        "AHB peripheral clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahbenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpioenr",
                    description: Some(
                        "I/O port clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpioenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgcfgr",
                    description: Some(
                        "Debug configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbgcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbenr1",
                    description: Some(
                        "APB peripheral clock enable register 1.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbenr2",
                    description: Some(
                        "APB peripheral clock enable register 2.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbsmenr",
                    description: Some(
                        "AHB peripheral clock enable in Sleep/Stop mode register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahbsmenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpiosmenr",
                    description: Some(
                        "I/O port in Sleep mode clock enable register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpiosmenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbsmenr1",
                    description: Some(
                        "APB peripheral clock enable in Sleep/Stop mode register 1.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbsmenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apbsmenr2",
                    description: Some(
                        "APB peripheral clock enable in Sleep/Stop mode register 2.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apbsmenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr",
                    description: Some(
                        "Peripherals independent clock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RTC domain control register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
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
                        "Control/status register.",
                    ),
                    array: None,
                    byte_offset: 0x94,
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
                    name: "crrcr",
                    description: Some(
                        "RCC clock recovery RC register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahbenr",
            extends: None,
            description: Some(
                "AHB peripheral clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.",
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
                    name: "dma2en",
                    description: Some(
                        "DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.",
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
                    name: "flashen",
                    description: Some(
                        "Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.",
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
                        "CRC clock enable Set and cleared by software.",
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
                        "AES hardware accelerator Set and cleared by software.",
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
                    name: "rngen",
                    description: Some(
                        "Random number generator clock enable Set and cleared by software.",
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
                    name: "tscen",
                    description: Some(
                        "Touch sensing controller clock enable Set and cleared by software.",
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
            name: "Ahbrstr",
            extends: None,
            description: Some(
                "AHB peripheral reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA1 and DMAMUX reset Set and cleared by software.",
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
                    name: "dma2rst",
                    description: Some(
                        "DMA2 and DMAMUX reset Set and cleared by software.",
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
                    name: "flashrst",
                    description: Some(
                        "Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.",
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
                    name: "crcrst",
                    description: Some(
                        "CRC reset Set and cleared by software.",
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
                        "AES hardware accelerator reset Set and cleared by software.",
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
                    name: "rngrst",
                    description: Some(
                        "Random number generator reset Set and cleared by software.",
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
                    name: "tscrst",
                    description: Some(
                        "Touch sensing controller reset Set and cleared by software.",
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
            name: "Ahbsmenr",
            extends: None,
            description: Some(
                "AHB peripheral clock enable in Sleep/Stop mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1smen",
                    description: Some(
                        "DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.",
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
                    name: "dma2smen",
                    description: Some(
                        "DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.",
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
                    name: "flashsmen",
                    description: Some(
                        "Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.",
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
                    name: "sramsmen",
                    description: Some(
                        "SRAM clock enable during Sleep mode Set and cleared by software.",
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
                    name: "crcsmen",
                    description: Some(
                        "CRC clock enable during Sleep mode Set and cleared by software.",
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
                    name: "aessmen",
                    description: Some(
                        "AES hardware accelerator clock enable during Sleep mode Set and cleared by software.",
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
                    name: "rngsmen",
                    description: Some(
                        "RNG clock enable during Sleep and Stop mode Set and cleared by software.",
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
                    name: "tscsmen",
                    description: Some(
                        "TSC clock enable during Sleep and Stop mode Set and cleared by software.",
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
            name: "Apbenr1",
            extends: None,
            description: Some(
                "APB peripheral clock enable register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 timer clock enable Set and cleared by software.",
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
                        "TIM3 timer clock enable Set and cleared by software.",
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
                    name: "tim6en",
                    description: Some(
                        "TIM6 timer clock enable Set and cleared by software.",
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
                        "TIM7 timer clock enable Set and cleared by software.",
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
                    name: "lpuart2en",
                    description: Some(
                        "LPUART2 clock enable Set and cleared by software.",
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
                    name: "lcden",
                    description: Some(
                        "LCD clock enable<sup>(1)</sup> Set and cleared by software.",
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
                    name: "rtcapben",
                    description: Some(
                        "RTC APB clock enable Set and cleared by software.",
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
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.",
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
                    name: "lpuart3en",
                    description: Some(
                        "LPUART3 clock enable Set and cleared by software.",
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
                    name: "usben",
                    description: Some(
                        "USB clock enable<sup>(1)</sup> Set and cleared by software.",
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
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable Set and cleared by software.",
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
                        "SPI3 clock enable<sup>(1)</sup> Set and cleared by software.",
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
                    name: "crsen",
                    description: Some(
                        "CRS clock enable<sup>(1)</sup> Set and cleared by software.",
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
                        "USART2 clock enable Set and cleared by software.",
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
                        "USART3 clock enable Set and cleared by software.",
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
                    name: "usart4en",
                    description: Some(
                        "USART4 clock enable Set and cleared by software.",
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
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 clock enable Set and cleared by software.",
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
                        "I2C1 clock enable Set and cleared by software.",
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
                        "I2C2 clock enable Set and cleared by software.",
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
                        "I2C3 clock enable Set and cleared by software.",
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
                    name: "opampen",
                    description: Some(
                        "OPAMP clock enable Set and cleared by software.",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4EN clock enable<sup>(1)</sup> Set and cleared by software.",
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
                    name: "lptim3en",
                    description: Some(
                        "LPTIM3 clock enable Set and cleared by software.",
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
                    name: "pwren",
                    description: Some(
                        "Power interface clock enable Set and cleared by software.",
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
                    name: "dac1en",
                    description: Some(
                        "DAC1 interface clock enable Set and cleared by software.",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable Set and cleared by software.",
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
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 clock enable Set and cleared by software.",
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
            name: "Apbenr2",
            extends: None,
            description: Some(
                "APB peripheral clock enable register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.",
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
                    name: "tim1en",
                    description: Some(
                        "TIM1 timer clock enable Set and cleared by software.",
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
                        "SPI1 clock enable Set and cleared by software.",
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
                        "USART1 clock enable Set and cleared by software.",
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
                        "TIM15 timer clock enable Set and cleared by software.",
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
                        "TIM16 timer clock enable Set and cleared by software.",
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
                    name: "adcen",
                    description: Some(
                        "ADC clock enable Set and cleared by software.",
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
            name: "Apbrstr1",
            extends: None,
            description: Some(
                "APB peripheral reset register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 timer reset Set and cleared by software.",
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
                        "TIM3 timer reset Set and cleared by software.",
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
                    name: "tim6rst",
                    description: Some(
                        "TIM6 timer reset Set and cleared by software.",
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
                        "TIM7 timer reset Set and cleared by software.",
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
                    name: "lpuart2rst",
                    description: Some(
                        "LPUART2 reset Set and cleared by software.",
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
                    name: "lcdrst",
                    description: Some(
                        "LCD reset<sup>(1)</sup> Set and cleared by software.",
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
                    name: "lpuart3rst",
                    description: Some(
                        "LPUART3 reset<sup>(1)</sup> Set and cleared by software.",
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
                    name: "usbrst",
                    description: Some(
                        "USB reset<sup>(1)</sup> Set and cleared by software.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset Set and cleared by software.",
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
                        "SPI3 reset<sup>(1)</sup> Set and cleared by software.",
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
                    name: "crsrst",
                    description: Some(
                        "CRS reset<sup>(1)</sup> Set and cleared by software.",
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
                        "USART2 reset Set and cleared by software.",
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
                        "USART3 reset Set and cleared by software.",
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
                    name: "usart4rst",
                    description: Some(
                        "USART4 reset Set and cleared by software.",
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
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 reset Set and cleared by software.",
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
                        "I2C1 reset Set and cleared by software.",
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
                        "I2C2 reset Set and cleared by software.",
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
                        "I2C3 reset Set and cleared by software.",
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
                    name: "opamprst",
                    description: Some(
                        "OPAMP reset Set and cleared by software.",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset<sup>(1)</sup> Set and cleared by software.",
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
                    name: "lptim3rst",
                    description: Some(
                        "LPTIM3 reset Set and cleared by software.",
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
                    name: "pwrrst",
                    description: Some(
                        "Power interface reset Set and cleared by software.",
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
                    name: "dac1rst",
                    description: Some(
                        "DAC1 interface reset Set and cleared by software.",
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
                    name: "lptim2rst",
                    description: Some(
                        "Low Power Timer 2 reset Set and cleared by software.",
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
                    name: "lptim1rst",
                    description: Some(
                        "Low Power Timer 1 reset Set and cleared by software.",
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
            name: "Apbrstr2",
            extends: None,
            description: Some(
                "APB peripheral reset register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SYSCFG, COMP and VREFBUF reset Set and cleared by software.",
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
                    name: "tim1rst",
                    description: Some(
                        "TIM1 timer reset Set and cleared by software.",
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
                        "SPI1 reset Set and cleared by software.",
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
                        "USART1 reset Set and cleared by software.",
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
                        "TIM15 timer reset Set and cleared by software.",
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
                        "TIM16 timer reset Set and cleared by software.",
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
                    name: "adcrst",
                    description: Some(
                        "ADC reset Set and cleared by software.",
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
            name: "Apbsmenr1",
            extends: None,
            description: Some(
                "APB peripheral clock enable in Sleep/Stop mode register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 timer clock enable during Sleep mode Set and cleared by software.",
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
                        "TIM3 timer clock enable during Sleep mode Set and cleared by software.",
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
                    name: "tim6smen",
                    description: Some(
                        "TIM6 timer clock enable during Sleep mode Set and cleared by software.",
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
                    name: "tim7smen",
                    description: Some(
                        "TIM7 timer clock enable during Sleep mode Set and cleared by software.",
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
                    name: "lpuart2smen",
                    description: Some(
                        "LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "lcdsmen",
                    description: Some(
                        "LCD clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.",
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
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC APB clock enable during Sleep mode Set and cleared by software.",
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
                    name: "wwdgsmen",
                    description: Some(
                        "WWDG clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "lpuart3smen",
                    description: Some(
                        "LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "usbsmen",
                    description: Some(
                        "USB clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.",
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
                    name: "spi2smen",
                    description: Some(
                        "SPI2 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "spi3smen",
                    description: Some(
                        "SPI3 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.",
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
                    name: "crssmen",
                    description: Some(
                        "CRS clock enable during Sleep and Stop modes<sup>(1)</sup> Set and cleared by software.",
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
                    name: "usart2smen",
                    description: Some(
                        "USART2 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                        "USART3 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "usart4smen",
                    description: Some(
                        "USART4 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "lpuart1smen",
                    description: Some(
                        "LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                        "I2C2 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "opampsmen",
                    description: Some(
                        "OPAMP clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "i2c4smen",
                    description: Some(
                        "I2C4 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.",
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
                    name: "lptim3smen",
                    description: Some(
                        "Low power timer 3 clock enable during Sleep mode Set and cleared by software.",
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
                    name: "pwrsmen",
                    description: Some(
                        "Power interface clock enable during Sleep mode Set and cleared by software.",
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
                    name: "dac1smen",
                    description: Some(
                        "DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "lptim2smen",
                    description: Some(
                        "Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "lptim1smen",
                    description: Some(
                        "Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.",
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
            name: "Apbsmenr2",
            extends: None,
            description: Some(
                "APB peripheral clock enable in Sleep/Stop mode register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsmen",
                    description: Some(
                        "SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "tim1smen",
                    description: Some(
                        "TIM1 timer clock enable during Sleep mode Set and cleared by software.",
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
                        "SPI1 clock enable during Sleep mode Set and cleared by software.",
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
                        "USART1 clock enable during Sleep and Stop modes Set and cleared by software.",
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
                    name: "tim15smen",
                    description: Some(
                        "TIM15 timer clock enable during Sleep mode Set and cleared by software.",
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
                    name: "tim16smen",
                    description: Some(
                        "TIM16 timer clock enable during Sleep mode Set and cleared by software.",
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
                    name: "adcsmen",
                    description: Some(
                        "ADC clock enable during Sleep mode Set and cleared by software.",
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
                "RTC domain control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable Set and cleared by software to enable LSE oscillator:.",
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
                        "LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.",
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
                        "LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).",
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
                        "LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.",
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
                        "CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.",
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
                        "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):.",
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
                        "LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.",
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
                        "RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.",
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
                        "LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system.",
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
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.",
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
                    name: "bdrst",
                    description: Some(
                        "RTC domain software reset Set and cleared by software to reset the RTC domain:.",
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
                        "Low-speed clock output (LSCO) enable Set and cleared by software.",
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
                        "Low-speed clock output selection Set and cleared by software to select the low-speed output clock:.",
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
            ],
        },
        FieldSet {
            name: "Ccipr",
            extends: None,
            description: Some(
                "Peripherals independent clock configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:.",
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
                        "USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "lpuart3sel",
                    description: Some(
                        "LPUART3 clock source selection<sup>(1)</sup> This bitfield is controlled by software to select LPUART3 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuart3sel",
                    ),
                },
                Field {
                    name: "lpuart2sel",
                    description: Some(
                        "LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuart2sel",
                    ),
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuart1sel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:.",
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
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
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
                        "LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "lptim3sel",
                    description: Some(
                        "LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim3sel",
                    ),
                },
                Field {
                    name: "tim1sel",
                    description: Some(
                        "TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim1sel",
                    ),
                },
                Field {
                    name: "tim15sel",
                    description: Some(
                        "TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim15sel",
                    ),
                },
                Field {
                    name: "clk48sel",
                    description: Some(
                        "481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Clk48sel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "Clock configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.",
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
                        "System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved.",
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
                    name: "hpre",
                    description: Some(
                        "AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre",
                    description: Some(
                        "APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1.",
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
                    name: "stopwuck",
                    description: Some(
                        "Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).",
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
                    name: "mco2sel",
                    description: Some(
                        "Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.",
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
                        "Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "Clock interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.",
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
                        "LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.",
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
                    name: "msirdyc",
                    description: Some(
                        "MSI ready interrupt clear This bit is set by software to clear the MSIRDYF flag.",
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
                        "HSI ready interrupt clear This bit is set software to clear the HSIRDYF flag.",
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
                        "HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.",
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
                        "PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag.",
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
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.",
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
                    name: "lsecssc",
                    description: Some(
                        "LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.",
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
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag.",
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
                "Clock interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:.",
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
                        "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:.",
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
                    name: "msirdyie",
                    description: Some(
                        "MSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSI oscillator stabilization.",
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
                        "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization:.",
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
                        "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:.",
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
                        "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock:.",
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
                    name: "lsecssie",
                    description: Some(
                        "LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.",
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
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.",
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
            name: "Cifr",
            extends: None,
            description: Some(
                "Clock interrupt flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.",
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
                        "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.",
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
                    name: "msirdyf",
                    description: Some(
                        "MSI ready interrupt flag Set by hardware when the MSI clock becomes stable and MSIRDYDIE is set. Cleared by software setting the MSIRDYC bit.",
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
                        "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.",
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
                        "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.",
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
                        "PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYIE is set. Cleared by software setting the PLLRDYC bit.",
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
                    name: "cssf",
                    description: Some(
                        "HSE clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.",
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
                    name: "lsecssf",
                    description: Some(
                        "LSE clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.",
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
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to RCC clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit.",
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
                "Clock control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msion",
                    description: Some(
                        "MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock.",
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
                    name: "msirdy",
                    description: Some(
                        "MSI clock ready flag This bit is set by hardware to indicate that the MSI oscillator is stable. Note: Once the MSION bit is cleared, MSIRDY goes low after 6 MSI clock cycles.",
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
                    name: "msipllen",
                    description: Some(
                        "MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register).",
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
                    name: "msirgsel",
                    description: Some(
                        "MSI clock range selection Set by software to select the MSI clock range with MSIRANGE[3:0]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msirgsel",
                    ),
                },
                Field {
                    name: "msirange",
                    description: Some(
                        "MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msirange",
                    ),
                },
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable Set and cleared by software. Cleared by hardware to stop the HSI oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock).",
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
                        "HSI always enable for peripheral kernels. Set and cleared by software to force HSI ON even in Stop modes. The HSI can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI as kernel clock. Keeping the HSI ON in Stop mode allows avoiding to slow down the communication speed because of the HSI startup time. This bit has no effect on HSION value.",
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
                        "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI clock cycles.",
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
                    name: "hsiasfs",
                    description: Some(
                        "HSI automatic start from Stop Set and cleared by software. When the system wake-up clock is MSI, this bit is used to wake up the HSI is parallel of the system wake-up.",
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
                    name: "hseon",
                    description: Some(
                        "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.",
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
                        "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles.",
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
                        "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.",
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
                    name: "csson",
                    description: Some(
                        "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.",
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
                    name: "pllon",
                    description: Some(
                        "PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock.",
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
                        "PLL clock ready flag Set by hardware to indicate that the PLL is locked.",
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
            name: "Crrcr",
            extends: None,
            description: Some(
                "RCC clock recovery RC register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48on",
                    description: Some(
                        "HSI48 RC oscillator enable<sup>(1)</sup>.",
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
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag<sup>(1)</sup> The flag is set when the HSI48 clock is ready for use.",
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
                    name: "hsi48cal",
                    description: Some(
                        "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "Control/status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:.",
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
                        "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.",
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
                    name: "lsiprediv",
                    description: Some(
                        "Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsiprediv",
                    ),
                },
                Field {
                    name: "msisrange",
                    description: Some(
                        "MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE[3:0] can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE[3:0] does not change the current MSI frequency.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msisrange",
                    ),
                },
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flags Set by software to clear the reset flags.",
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
                        "Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.",
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
                        "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.",
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
                    name: "pwrrstf",
                    description: Some(
                        "BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.",
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
                        "Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.",
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
                        "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.",
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
                        "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.",
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
                        "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, nRST_STDBY or nRST_SHDW option bits are cleared.",
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
            name: "Dbgcfgr",
            extends: None,
            description: Some(
                "Debug configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbgen",
                    description: Some(
                        "Debug support clock enable Set and cleared by software.",
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
                    name: "dbgrst",
                    description: Some(
                        "Debug support reset Set and cleared by software.",
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
            name: "Gpioenr",
            extends: None,
            description: Some(
                "I/O port clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "I/O port A clock enable This bit is set and cleared by software.",
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
                        "I/O port B clock enable This bit is set and cleared by software.",
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
                        "I/O port C clock enable This bit is set and cleared by software.",
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
                        "I/O port D clock enable This bit is set and cleared by software.",
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
                        "I/O port E clock enable<sup>(1)</sup> This bit is set and cleared by software.",
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
                        "I/O port F clock enable This bit is set and cleared by software.",
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
            name: "Gpiorstr",
            extends: None,
            description: Some(
                "I/O port reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "I/O port A reset This bit is set and cleared by software.",
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
                        "I/O port B reset This bit is set and cleared by software.",
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
                        "I/O port C reset This bit is set and cleared by software.",
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
                        "I/O port D reset This bit is set and cleared by software.",
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
                        "I/O port E reset This bit is set and cleared by software.",
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
                        "I/O port F reset This bit is set and cleared by software.",
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
            name: "Gpiosmenr",
            extends: None,
            description: Some(
                "I/O port in Sleep mode clock enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "I/O port A clock enable during Sleep mode Set and cleared by software.",
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
                        "I/O port B clock enable during Sleep mode Set and cleared by software.",
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
                        "I/O port C clock enable during Sleep mode Set and cleared by software.",
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
                        "I/O port D clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.",
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
                        "I/O port E clock enable during Sleep mode Set and cleared by software.",
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
                    name: "gpiofsmen",
                    description: Some(
                        "I/O port F clock enable during Sleep mode Set and cleared by software.",
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
            name: "Icscr",
            extends: None,
            description: Some(
                "Internal clock sources calibration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msical",
                    description: Some(
                        "MSI clock calibration These bits are initialized at startup with the factory-programmed MSI calibration trim value. When MSITRIM is written, MSICAL is updated with the sum of MSITRIM and the factory trim value.",
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
                    name: "msitrim",
                    description: Some(
                        "MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL[7:0] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL[7:0] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI. The default value is 64 when added to the HSICAL value, trim the HSI to 161MHz 1 11%.",
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
                "PLL configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power.",
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
                    name: "pllm",
                    description: Some(
                        "Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "plln",
                    description: Some(
                        "PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f<sub>VCO</sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plln",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power.",
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
                    name: "pllp",
                    description: Some(
                        "PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power.",
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
                    name: "pllq",
                    description: Some(
                        "PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power.",
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
                    name: "pllr",
                    description: Some(
                        "PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
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
                    name: "SYS",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Clk48sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HSI48",
                    description: None,
                    value: 3,
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
            name: "I2c1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
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
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
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
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
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
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptim3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuart1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuart2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuart3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
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
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
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
            name: "Lsiprediv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 1,
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
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV64",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "DIV256",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV512",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV1024",
                    description: None,
                    value: 10,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "MSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "HSE",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "HSI48",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "RTC",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "RTC_WKUP",
                    description: None,
                    value: 10,
                },
            ],
        },
        Enum {
            name: "Msirange",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RANGE100K",
                    description: Some(
                        "range 0 around 100 kHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE200K",
                    description: Some(
                        "range 1 around 200 kHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE400K",
                    description: Some(
                        "range 2 around 400 kHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE800K",
                    description: Some(
                        "range 3 around 800 kHz",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RANGE1M",
                    description: Some(
                        "range 4 around 1 MHz",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RANGE2M",
                    description: Some(
                        "range 5 around 2 MHz",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "RANGE4M",
                    description: Some(
                        "range 6 around 4 MHz",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RANGE8M",
                    description: Some(
                        "range 7 around 8 MHz",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "RANGE16M",
                    description: Some(
                        "range 8 around 16 MHz",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RANGE24M",
                    description: Some(
                        "range 9 around 24 MHz",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "RANGE32M",
                    description: Some(
                        "range 10 around 32 MHz",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "RANGE48M",
                    description: Some(
                        "range 11 around 48 MHz",
                    ),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "Msirgsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CSR",
                    description: Some(
                        "MSI Range is provided by MSISRANGE[3:0] in RCC_CSR register",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CR",
                    description: Some(
                        "MSI Range is provided by MSIRANGE[3:0] in the RCC_CR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Msisrange",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RANGE_81MHZ",
                    description: None,
                    value: 4,
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
            bit_size: 7,
            variants: &[
                EnumVariant {
                    name: "MUL4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "MUL5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "MUL6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "MUL7",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "MUL8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "MUL9",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "MUL10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "MUL11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "MUL12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "MUL13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "MUL14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "MUL15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "MUL16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "MUL17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "MUL18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "MUL19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "MUL20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "MUL21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "MUL22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "MUL23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "MUL24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "MUL25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "MUL26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "MUL27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "MUL28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "MUL29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "MUL30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "MUL31",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "MUL32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "MUL33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "MUL34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "MUL35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "MUL36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "MUL37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "MUL38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "MUL39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "MUL40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "MUL41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "MUL42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "MUL43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "MUL44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "MUL45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "MUL46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "MUL47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "MUL48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "MUL49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "MUL50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "MUL51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "MUL52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "MUL53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "MUL54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "MUL55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "MUL56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "MUL57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "MUL58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "MUL59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "MUL60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "MUL61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "MUL62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "MUL63",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "MUL64",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "MUL65",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "MUL66",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "MUL67",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "MUL68",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "MUL69",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "MUL70",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "MUL71",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "MUL72",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "MUL73",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "MUL74",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "MUL75",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "MUL76",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "MUL77",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "MUL78",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "MUL79",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "MUL80",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "MUL81",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "MUL82",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "MUL83",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "MUL84",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "MUL85",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "MUL86",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "MUL87",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "MUL88",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "MUL89",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "MUL90",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "MUL91",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "MUL92",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "MUL93",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "MUL94",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "MUL95",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "MUL96",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "MUL97",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "MUL98",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "MUL99",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "MUL100",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "MUL101",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "MUL102",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "MUL103",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "MUL104",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "MUL105",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "MUL106",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "MUL107",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "MUL108",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "MUL109",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "MUL110",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "MUL111",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "MUL112",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "MUL113",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "MUL114",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "MUL115",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "MUL116",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "MUL117",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "MUL118",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "MUL119",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "MUL120",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "MUL121",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "MUL122",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "MUL123",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "MUL124",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "MUL125",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "MUL126",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "MUL127",
                    description: None,
                    value: 127,
                },
            ],
        },
        Enum {
            name: "Pllp",
            description: None,
            bit_size: 5,
            variants: &[
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
            ],
        },
        Enum {
            name: "Pllq",
            description: None,
            bit_size: 3,
            variants: &[
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
            name: "Pllr",
            description: None,
            bit_size: 3,
            variants: &[
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
            name: "Pllsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: None,
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
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MSI",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Tim15sel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PCLK1_TIM",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim1sel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PCLK1_TIM",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: None,
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
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usart2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: None,
                    value: 3,
                },
            ],
        },
    ],
};
