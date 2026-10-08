
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "Microcontroller debug unit",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: Some(
                        "identity code register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idcode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "status and configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "apb1lfzr",
                    description: Some(
                        "APB1L peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lfzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hfzr",
                    description: Some(
                        "APB1H peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hfzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2fzr",
                    description: Some(
                        "APB2 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb7fzr",
                    description: Some(
                        "APB7 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1fzr",
                    description: Some(
                        "AHB1 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbg_auth_host",
                    description: Some(
                        "debug host authentication register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgAuthHost",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbg_auth_device",
                    description: Some(
                        "debug device authentication register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgAuthDevice",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pncr",
                    description: Some(
                        "part number codification register",
                    ),
                    array: None,
                    byte_offset: 0x7dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pncr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr4",
                    description: Some(
                        "CoreSight peripheral identity register 4",
                    ),
                    array: None,
                    byte_offset: 0xfd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr0",
                    description: Some(
                        "CoreSight peripheral identity register 0",
                    ),
                    array: None,
                    byte_offset: 0xfe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr1",
                    description: Some(
                        "CoreSight peripheral identity register 1",
                    ),
                    array: None,
                    byte_offset: 0xfe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr2",
                    description: Some(
                        "CoreSight peripheral identity register 2",
                    ),
                    array: None,
                    byte_offset: 0xfe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr3",
                    description: Some(
                        "CoreSight peripheral identity register 3",
                    ),
                    array: None,
                    byte_offset: 0xfec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr0",
                    description: Some(
                        "CoreSight component identity register 0",
                    ),
                    array: None,
                    byte_offset: 0xff0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr1",
                    description: Some(
                        "CoreSight peripheral identity register 1",
                    ),
                    array: None,
                    byte_offset: 0xff4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr2",
                    description: Some(
                        "CoreSight component identity register 2",
                    ),
                    array: None,
                    byte_offset: 0xff8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr3",
                    description: Some(
                        "CoreSight component identity register 3",
                    ),
                    array: None,
                    byte_offset: 0xffc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb1fzr",
            extends: None,
            description: Some(
                "AHB1 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_gpdma1_ch0_stop",
                    description: Some(
                        "GPDMA 1 channel 0 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC0.",
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
                    name: "dbg_gpdma1_ch1_stop",
                    description: Some(
                        "GPDMA 1 channel 1 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC1.",
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
                    name: "dbg_gpdma1_ch2_stop",
                    description: Some(
                        "GPDMA 1 channel 2 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC2.",
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
                    name: "dbg_gpdma1_ch3_stop",
                    description: Some(
                        "GPDMA 1 channel 3 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC3.",
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
                    name: "dbg_gpdma1_ch4_stop",
                    description: Some(
                        "GPDMA 1 channel 4 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC4.",
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
                    name: "dbg_gpdma1_ch5_stop",
                    description: Some(
                        "GPDMA 1 channel 5 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC5.",
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
                    name: "dbg_gpdma1_ch6_stop",
                    description: Some(
                        "GPDMA 1 channel 6 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC6.",
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
                    name: "dbg_gpdma1_ch7_stop",
                    description: Some(
                        "GPDMA 1 channel 7 stop in CPU debug\r Write access can be protected by GPDMA_SECCFGR.SEC7.",
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
            name: "Apb1hfzr",
            extends: None,
            description: Some(
                "APB1H peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_lptim2_stop",
                    description: Some(
                        "LPTIM2 stop in CPU debug\r Write access can be protected by GTZC_TZSC.LPTIM2SEC.",
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
            name: "Apb1lfzr",
            extends: None,
            description: Some(
                "APB1L peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim2_stop",
                    description: Some(
                        "TIM2 stop in CPU debug\r Write access can be protected by GTZC_TZSC.TIM2SEC.",
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
                    name: "dbg_tim3_stop",
                    description: Some(
                        "TIM3 stop in CPU debug\r Write access can be protected by GTZC_TZSC.TIM3SEC.",
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
                    name: "dbg_wwdg_stop",
                    description: Some(
                        "WWDG stop in CPU debug\r Write access can be protected by GTZC_TZSC.WWDGSEC",
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
                    name: "dbg_iwdg_stop",
                    description: Some(
                        "IWDG stop in CPU debug\r Write access can be protected by GTZC_TZSC.IWDGSEC.",
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
                    name: "dbg_i2c1_stop",
                    description: Some(
                        "I2C1 SMBUS timeout stop in CPU debug\r Write access can be protected by GTZC_TZSC.I2C1SEC.",
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
            name: "Apb2fzr",
            extends: None,
            description: Some(
                "APB2 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim1_stop",
                    description: Some(
                        "TIM1 stop in CPU debug\r Write access can be protected by GTZC_TZSC.TIM1SEC.",
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
                    name: "dbg_tim16_stop",
                    description: Some(
                        "TIM16 stop in CPU debug\r Write access can be protected by GTZC_TZSC.TIM16SEC.",
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
                    name: "dbg_tim17_stop",
                    description: Some(
                        "TIM17 stop in CPU debug\r Write access can be protected by GTZC_TZSC.TIM17SEC.",
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
            name: "Apb7fzr",
            extends: None,
            description: Some(
                "APB7 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_i2c3_stop",
                    description: Some(
                        "I2C3 stop in CPU debug\r Access can be protected by GTZC_TZSC.I2C3SEC.",
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
                    name: "dbg_lptim1_stop",
                    description: Some(
                        "LPTIM1 stop in CPU debug\r Access can be protected by GTZC_TZSC.LPTIM1SEC.",
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
                    name: "dbg_rtc_stop",
                    description: Some(
                        "RTC stop in CPU debug\r Access can be protected by GTZC_TZSC.TIM17SEC.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure.",
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
            name: "Cidr0",
            extends: None,
            description: Some(
                "CoreSight component identity register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "Component ID bits [7:0]",
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
            ],
        },
        FieldSet {
            name: "Cidr1",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "Component ID bits [11:8]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "class",
                    description: Some(
                        "Component ID bits [15:12] - component class",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cidr2",
            extends: None,
            description: Some(
                "CoreSight component identity register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "Component ID bits [23:16]",
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
            ],
        },
        FieldSet {
            name: "Cidr3",
            extends: None,
            description: Some(
                "CoreSight component identity register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "Component ID bits [31:24]",
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
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "status and configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_stop",
                    description: Some(
                        "Allows debug in Stop mode\r Write access can be protected by PWR_SECCFGR.LPMSEC.\r The CPU debug and clocks remain active and the HSI oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.",
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
                    name: "dbg_standby",
                    description: Some(
                        "Allows debug in Standby mode\r Write access can be protected by PWR_SECCFGR.LPMSEC.\r The CPU debug and clocks remain active and the HSI oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed.",
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
                    name: "lpms",
                    description: Some(
                        "Device low power mode selected\r 10x: Standby mode\r others reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stopf",
                    description: Some(
                        "Device Stop flag",
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
                    name: "sbf",
                    description: Some(
                        "Device Standby flag",
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
                    name: "cs",
                    description: Some(
                        "CPU Sleep",
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
                    name: "cds",
                    description: Some(
                        "CPU DeepSleep",
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
            name: "DbgAuthDevice",
            extends: None,
            description: Some(
                "debug device authentication register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_id",
                    description: Some(
                        "Device specific ID\r Device specific ID used for RDP regression.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DbgAuthHost",
            extends: None,
            description: Some(
                "debug host authentication register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_key",
                    description: Some(
                        "Device authentication key\r The device specific 64-bit authentication key (OEMn key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idcode",
            extends: None,
            description: Some(
                "identity code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_id",
                    description: Some(
                        "Device ID",
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
                    name: "rev_id",
                    description: Some(
                        "Revision ID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pidr0",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "Part number bits [7:0]",
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
            ],
        },
        FieldSet {
            name: "Pidr1",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "Part number bits [11:8]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jep106id",
                    description: Some(
                        "JEP106 identity code bits [3:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pidr2",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106id",
                    description: Some(
                        "JEP106 identity code bits [6:4]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jedec",
                    description: Some(
                        "JEDEC assigned value",
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
                    name: "revision",
                    description: Some(
                        "Component revision number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pidr3",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmod",
                    description: Some(
                        "Customer modified",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "revand",
                    description: Some(
                        "Metal fix version",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pidr4",
            extends: None,
            description: Some(
                "CoreSight peripheral identity register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106con",
                    description: Some(
                        "JEP106 continuation code",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f4kcount",
                    description: Some(
                        "Register file size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pncr",
            extends: None,
            description: Some(
                "part number codification register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "codification",
                    description: Some(
                        "Part number codification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap_present",
                    description: Some(
                        "Bit n identifies whether access port APn is present in device \r Bit n�=�0: APn absent \r Bit n�=�1: APn present",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ap_enabled",
                    description: Some(
                        "Bit n identifies whether access port APn is open (can be accessed via the debug port) or locked (debug access to the APn is blocked, except for access) \r Bit n�=�0: APn locked (except for access to DBGMCU)\r Bit n�=�1: APn enabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
