
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "CR register.",
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
                    name: "cfgr",
                    description: Some(
                        "CFGR register.",
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
                    name: "cier",
                    description: Some(
                        "CIER register.",
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
                        "CIFR register.",
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
                    name: "cscmdr",
                    description: Some(
                        "CSCMDR register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cscmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbrstr",
                    description: Some(
                        "AHBRSTR register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "apb0rstr",
                    description: Some(
                        "APB0RSTR register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb0rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr",
                    description: Some(
                        "APB1RSTR register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "APB2RSTR register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                    name: "ahbenr",
                    description: Some(
                        "AHBENR register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                    name: "apb0enr",
                    description: Some(
                        "APB0ENR register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb0enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr",
                    description: Some(
                        "APB1ENR register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2ENR register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                    name: "csr",
                    description: Some(
                        "CSR register.",
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
                    name: "rfswhsecr",
                    description: Some(
                        "RFSWHSECR register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfswhsecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfhsecr",
                    description: Some(
                        "RFHSECR register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfhsecr",
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
                "AHBENR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaen",
                    description: Some(
                        "DMA and DMAMUX enable Set and enable by software.",
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
                    name: "gpioaen",
                    description: Some(
                        "GPIOA enable. It must be enabled by default.",
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
                    name: "gpioben",
                    description: Some(
                        "GPIOB enable. It must be enabled by default.",
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
                    name: "crcen",
                    description: Some(
                        "CRC enable Set and enable by software.",
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
                    name: "pkaen",
                    description: Some(
                        "PKA clock enable Set and enable by software.",
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
                        "RNG clock enable Set and enable by software.",
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
            name: "Ahbrstr",
            extends: None,
            description: Some(
                "AHBRSTR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmarst",
                    description: Some(
                        "DMA and DMAMUX reset Set and reset by software.",
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
                    name: "gpioarst",
                    description: Some(
                        "GPIOA reset Set and reset by software.",
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
                    name: "gpiobrst",
                    description: Some(
                        "GPIOB reset Set and reset by software.",
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
                    name: "crcrst",
                    description: Some(
                        "CRC reset Set and reset by software.",
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
                    name: "pkarst",
                    description: Some(
                        "PKA reset Set and reset by software.",
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
                        "RNG reset Set and reset by software.",
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
            name: "Apb0enr",
            extends: None,
            description: Some(
                "APB0ENR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 enable.",
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
                    name: "tim2en",
                    description: Some(
                        "TIM2: Advanced Timer clock enable Set and enable by software.",
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
                    name: "tim16en",
                    description: Some(
                        "TIM16 enable.",
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
                    name: "tim17en",
                    description: Some(
                        "TIM17 enable.",
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
                    name: "syscfgen",
                    description: Some(
                        "SYSTEM CONFIG enable Set and enable by software.",
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
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable Set and enable by software. Reset source only for this field: PORESETn.",
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
                    name: "wdgen",
                    description: Some(
                        "Watchdog clock enable. Set and enable by software.",
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
            name: "Apb0rstr",
            extends: None,
            description: Some(
                "APB0RSTR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1: Advanced Timer reset Set and reset by software.",
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
                    name: "tim16rst",
                    description: Some(
                        "TIM16 reset.",
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
                    name: "tim17rst",
                    description: Some(
                        "TIM17 reset.",
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
                    name: "syscfgrst",
                    description: Some(
                        "SYSTEM CONFIG reset Set and reset by software.",
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
                    name: "rtcrst",
                    description: Some(
                        "RTC reset Set and reset by software.",
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
                    name: "wdgrst",
                    description: Some(
                        "WATCHDOG reset Set and reset by software.",
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
                    name: "wdrst",
                    description: Some(
                        "WATCHDOG reset Set and reset by software.",
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
            name: "Apb1enr",
            extends: None,
            description: Some(
                "APB1ENR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1en",
                    description: Some(
                        "SPI1 enable.",
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
                    name: "adcdigen",
                    description: Some(
                        "AUXADC clock enable for Aux-ADC digital clock Set and enable by software.",
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
                    name: "adcanaen",
                    description: Some(
                        "ADC clock enable for Aux-ADC analog clock Set and enable by software.",
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
                    name: "lpuarten",
                    description: Some(
                        "LPUART clock enable Set and enable by software.",
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
                    name: "usart1en",
                    description: Some(
                        "USART clock enable Set and enable by software.",
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
                    name: "spi2en",
                    description: Some(
                        "SPI2 enable.",
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
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable Set and enable by software.",
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
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable Set and enable by software.",
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
                        "I2C2 enable.",
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
            name: "Apb1rstr",
            extends: None,
            description: Some(
                "APB1RSTR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI1 reset.",
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
                    name: "adcrst",
                    description: Some(
                        "ADC reset.",
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
                    name: "auxadcrst",
                    description: Some(
                        "AUXADC reset for Aux-ADC digital clock Set and reset by software.",
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
                    name: "lpuartrst",
                    description: Some(
                        "LPUART reset Set and reset by software.",
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
                    name: "usartrst",
                    description: Some(
                        "USART reset Set and reset by software.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset.",
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
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset Set and reset by software.",
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
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset Set and reset by software.",
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
                    name: "i2c21rst",
                    description: Some(
                        "I2C1 reset Set and reset by software.",
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
                        "2C2 reset.",
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
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2ENR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mrbleen",
                    description: Some(
                        "MR_BLE enable.",
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
                    name: "clkblediv",
                    description: Some(
                        "MR_BLE clock frequency selection when RCC_APB2ENR.MRBLEEN=1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Clkblediv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "APB2RSTR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blerst",
                    description: Some(
                        "BLE reset.",
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
                    name: "mrblerst",
                    description: Some(
                        "MR_BLE (Bluetooth radio) reset.",
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
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "CFGR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpsinv",
                    description: Some(
                        "bit to control inversion of the SMPS clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smpsinv",
                    ),
                },
                Field {
                    name: "hsesel",
                    description: Some(
                        "Clock source selection request:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsesel",
                    ),
                },
                Field {
                    name: "stophsi",
                    description: Some(
                        "Stop HSI clock source request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stophsi",
                    ),
                },
                Field {
                    name: "hsesel_status",
                    description: Some(
                        "Clock source selection Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "HseselStatus",
                    ),
                },
                Field {
                    name: "clksysdiv",
                    description: Some(
                        "CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clksysdiv_status",
                    description: Some(
                        "CLKSYSDIV_STATUS: system clock frequency status Set and cleared by hardware to indicate the actual system clock frequency. This register must be read to be sure that the new frequency, selected by CLKSYSDIV, has been applied. 000: system clock frequency is 64 MHz 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz 100: system clock frequency is 4 MHz 101: system clock frequency is 2 MHz 110: system clock frequency is 1 MHz 111: not used. The actual clock frequency switching can be delayed of up to 128 system clock cycles, depending on the RCC internal counter status at the moment the new CLKSYSDIV is applied.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpsdiv",
                    description: Some(
                        "SMPS clock prescaling factor to generate 4MHz or 8MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smpsdiv",
                    ),
                },
                Field {
                    name: "lpuclksel",
                    description: Some(
                        "Selection of LPUART clock:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpuclksel",
                    ),
                },
                Field {
                    name: "clkslowsel",
                    description: Some(
                        "slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Clkslowsel",
                    ),
                },
                Field {
                    name: "ioboosten",
                    description: Some(
                        "IO BOOSTER enable Set and reset by software.",
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
                    name: "ioboostclkexten",
                    description: Some(
                        "IO BOOSTER clock enable as external clock Set and reset by software.",
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
                    name: "lcoen",
                    description: Some(
                        "LCO output enable.",
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
                    name: "spi3i2sclksel",
                    description: Some(
                        "Selection of I2S1 clock: 1x:64MHz peripheral clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spiisclksel",
                    ),
                },
                Field {
                    name: "spi2i2sclksel",
                    description: Some(
                        "Selection of I2S clock: 1x:64MHz peripheral clock.",
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
                    name: "lcosel",
                    description: Some(
                        "Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lcosel",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "ccopre",
                    description: Some(
                        "Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ccopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "CIER register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization.",
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
                        "LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization.",
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
                        "HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization.",
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
                        "HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization.",
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
                    name: "hsipllrdyie",
                    description: Some(
                        "HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE.",
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
                    name: "hsipllunlockdetie",
                    description: Some(
                        "HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock.",
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
                    name: "rtcrstie",
                    description: Some(
                        "RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end.",
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
                    name: "wdgrstie",
                    description: Some(
                        "WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end.",
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
                    name: "lpurstie",
                    description: Some(
                        "LPURSTIE: LPUART reset release interrupt enable.",
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
                "CIFR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyif",
                    description: Some(
                        "LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsirdyif",
                    ),
                },
                Field {
                    name: "lserdyif",
                    description: Some(
                        "LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lserdyif",
                    ),
                },
                Field {
                    name: "hsirdyif",
                    description: Some(
                        "HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsirdyif",
                    ),
                },
                Field {
                    name: "hserdyif",
                    description: Some(
                        "HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hserdyif",
                    ),
                },
                Field {
                    name: "hsipllrdyif",
                    description: Some(
                        "HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsipllrdyif",
                    ),
                },
                Field {
                    name: "hsipllunlockdetif",
                    description: Some(
                        "HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag.",
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
                    name: "rtcrstif",
                    description: Some(
                        "RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock.",
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
                    name: "wdgrstif",
                    description: Some(
                        "WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock.",
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
                    name: "lpurstf",
                    description: Some(
                        "LPUART reset release flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpurstf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn.",
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
                    name: "lsirdy",
                    description: Some(
                        "Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsirdy",
                    ),
                },
                Field {
                    name: "lseon",
                    description: Some(
                        "External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn.",
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
                    name: "lserdy",
                    description: Some(
                        "External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lserdy",
                    ),
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsebyp",
                    ),
                },
                Field {
                    name: "lockdet_nstop",
                    description: Some(
                        "Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsirdy",
                    ),
                },
                Field {
                    name: "hsepllbufon",
                    description: Some(
                        "External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software.",
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
                    name: "hsipllon",
                    description: Some(
                        "Internal High Speed Clock PLL enable.",
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
                    name: "hsipllrdy",
                    description: Some(
                        "Internal High Speed Clock PLL ready flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsipllrdy",
                    ),
                },
                Field {
                    name: "fmrat",
                    description: Some(
                        "Force MR_BLE active transmission status (for debug purpose).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmrat",
                    ),
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off.",
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
                        "External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hserdy",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cscmdr",
            extends: None,
            description: Some(
                "CSCMDR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "request",
                    description: Some(
                        "Request for system clock switching Cleared by hardware when system clock frequency switch is done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Request",
                    ),
                },
                Field {
                    name: "clksysdiv_req",
                    description: Some(
                        "system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "ClksysdivReq",
                    ),
                },
                Field {
                    name: "status",
                    description: Some(
                        "Status of clock switch sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Status",
                    ),
                },
                Field {
                    name: "eofseq_ie",
                    description: Some(
                        "End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch.",
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
                    name: "eofseq_irq",
                    description: Some(
                        "End of Sequence flag Set by hardware when clock system swtich is ended.",
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
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "CSR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rmvf",
                    ),
                },
                Field {
                    name: "padrstf",
                    description: Some(
                        "SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Padrstf",
                    ),
                },
                Field {
                    name: "porrstf",
                    description: Some(
                        "POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Porrstf",
                    ),
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sftrstf",
                    ),
                },
                Field {
                    name: "wdgrstf",
                    description: Some(
                        "Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wdgrstf",
                    ),
                },
                Field {
                    name: "lockuprstf",
                    description: Some(
                        "LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lockuprstf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rfhsecr",
            extends: None,
            description: Some(
                "RFHSECR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xotune",
                    description: Some(
                        "RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rfswhsecr",
            extends: None,
            description: Some(
                "RFSWHSECR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "satrg",
                    description: Some(
                        "Sense Amplifier threshold Set by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Satrg",
                    ),
                },
                Field {
                    name: "gmc",
                    description: Some(
                        "High Speed External XO current control Set by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Gmc",
                    ),
                },
                Field {
                    name: "swxotuneen",
                    description: Some(
                        "RF-HSE capacitor bank tuning by SW enable Set by software.",
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
                    name: "swxotune",
                    description: Some(
                        "RF-HSE capacitor bank tuning value by SW Set by software.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ccopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CCO clock is divided by 1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CCO clock is divided by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "CCO clock is divided by 4.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "CCO clock is divided by 8.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "CCO clock is divided by 16.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Clkblediv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "32MHz.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16MHz.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Clkslowsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "LSILMPU oscillator clock (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LSE oscillator clock used as slow clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "LSI oscillator clock used as slow clock.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "HSI_64M divided by 2048 used as slow clock.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ClksysdivReq",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "div 1 (sys clock 64M).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "div 2 (sys clock 32M).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "div 4 (sys clock 16M).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "div 8 (sys clock 8M).",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "div 16 (sys clock 4M).",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "div 32 (sys clock 2M).",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "div 64 (sys clock 1M).",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Fmrat",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "active_transmission is force to '1' whatever the HSIPLLRDY status.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Gmc",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "max 0.0 001: max 0.57 mA/V.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "max 0.78 mA/V.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "max 1.13 mA/V (Default).",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "max 0.61 mA/V.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "max 1.65 mA/V.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "max 2.12 mA/V.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "max 2.84 mA/V.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Hserdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "HSE oscillator not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "HSE oscillator ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hserdyif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No clock ready interrupt caused by the HSE oscillator.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clock ready interrupt caused by the HSE oscillator.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsesel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "HSI clock source is requested (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "HSE clock source is requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "HseselStatus",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "HSI clock source is requested (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "HSE clock source is requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsipllrdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "PLL is unlocked.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "PLL is locked.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsipllrdyif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No clock ready interrupt caused by the HSI PLL64 MHz oscillator.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clock ready interrupt caused by the HSI PLL64 MHz oscillator.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsirdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "internal RC 64 MHz oscillator not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "internal RC 64 MHz oscillator ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsirdyif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No clock ready interrupt caused by the HSI oscillator.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clock ready interrupt caused by the HSI oscillator.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lcosel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "LCO output disabled, no clock on LCO.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "internal 32 KHz (LSI_LPMU) oscillator clock selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "internal 32 KHz (LSI) oscillator clock selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "external 32 KHz (LSE) oscillator clock selected.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lockuprstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No lockup reset occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "lockup reset occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpuclksel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "16MHz peripheral clock (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LSE clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpurstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no LPUART reset release event occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LPUART reset release event occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lsebyp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "LSE oscillator bypass OFF.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LSE oscillator bypass ON.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lserdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "LSE oscillator not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LSE oscillator ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lserdyif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No clock ready interrupt caused by the LSE oscillator.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clock ready interrupt caused by the LSE oscillator.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lsirdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "LSI RC oscillator not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "LSI RC oscillator ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lsirdyif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No clock ready interrupt caused by the internal RC 32 KHz oscillator.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clock ready interrupt caused by the internal RC 32 kHz oscillator.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "MCO output disabled, no clock on MCO.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "system clock selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "na.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "internal RC 64 MHz (HSI) oscillator clock selected.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "external oscillator (HSE) clock selected.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "SMPS clock selected.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "AUX ADC ANA clock selected.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Padrstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No reset from pad occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Reset from pad occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Porrstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No POWER reset occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "POWER reset occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Request",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "To cancel an ongiong request - still possible until IRQ assertion.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "To update the system clock frequency.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rmvf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Nothing done.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Reset the value of the reset flags.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Satrg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "the bias current is confronted to a reference current with a ratio of 1/2.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "the bias current is confronted to a reference current with a ratio of 3/4.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sftrstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No software reset occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Software reset occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smpsdiv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "div 2 when ANADIV=2 or 4 (default ).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "div 4 when ANADIV=1 or 2.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smpsinv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SMPS clock not inverted (default value).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SMPS clock inverted (for debug).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spiisclksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "16MHz peripheral clock (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "32MHz peripheral clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Status",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "IDLE no switch requested.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "ONGOING clock frequency switch is ongoing.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "DONE clock frequency switch done.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Stophsi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "HSI is enabled (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "disable HSI is requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wdgrstf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No watchdog reset occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Watchdog reset occurred.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
