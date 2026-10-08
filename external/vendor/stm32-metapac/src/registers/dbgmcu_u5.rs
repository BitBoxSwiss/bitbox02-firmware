
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "MCU debug component",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: Some(
                        "DBGMCU_IDCODE",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU configuration\r register",
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
                        "Debug MCU APB1L peripheral freeze\r register",
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
                        "Debug MCU APB1H peripheral freeze register",
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
                        "Debug MCU APB2 peripheral freeze register",
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
                    name: "apb3fzr",
                    description: Some(
                        "Debug MCU APB3 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1fzr",
                    description: Some(
                        "Debug MCU AHB1 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "ahb3fzr",
                    description: Some(
                        "Debug MCU AHB3 peripheral freeze register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgmcu_sr",
                    description: Some(
                        "DBGMCU status register",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgmcuSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgmcu_dbg_auth_host",
                    description: Some(
                        "DBGMCU debug host authentication register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgmcuDbgAuthHost",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgmcu_dbg_auth_device",
                    description: Some(
                        "DBGMCU debug device authentication register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgmcuDbgAuthDevice",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr4",
                    description: Some(
                        "Debug MCU CoreSight peripheral identity register 4",
                    ),
                    array: None,
                    byte_offset: 0xfd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight peripheral identity register 0",
                    ),
                    array: None,
                    byte_offset: 0xfe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight peripheral identity register 1",
                    ),
                    array: None,
                    byte_offset: 0xfe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight peripheral identity register 2",
                    ),
                    array: None,
                    byte_offset: 0xfe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight peripheral identity register 3",
                    ),
                    array: None,
                    byte_offset: 0xfec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight component identity register 0",
                    ),
                    array: None,
                    byte_offset: 0xff0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight component identity register 1",
                    ),
                    array: None,
                    byte_offset: 0xff4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight component identity register 2",
                    ),
                    array: None,
                    byte_offset: 0xff8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Debug MCU CoreSight component identity register 3",
                    ),
                    array: None,
                    byte_offset: 0xffc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                "Debug MCU AHB1 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_gpdma0_stop",
                    description: Some(
                        "GPDMA channel 0 stop in debug",
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
                    name: "dbg_gpdma1_stop",
                    description: Some(
                        "GPDMA channel 1 stop in debug",
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
                    name: "dbg_gpdma2_stop",
                    description: Some(
                        "GPDMA channel 2 stop in debug",
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
                    name: "dbg_gpdma3_stop",
                    description: Some(
                        "GPDMA channel 3 stop in debug",
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
                    name: "dbg_gpdma4_stop",
                    description: Some(
                        "GPDMA channel 4 stop in debug",
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
                    name: "dbg_gpdma5_stop",
                    description: Some(
                        "GPDMA channel 5 stop in debug",
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
                    name: "dbg_gpdma6_stop",
                    description: Some(
                        "GPDMA channel 6 stop in debug",
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
                    name: "dbg_gpdma7_stop",
                    description: Some(
                        "GPDMA channel 7 stop in debug",
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
                    name: "dbg_gpdma8_stop",
                    description: Some(
                        "GPDMA channel 8 stop in debug",
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
                    name: "dbg_gpdma9_stop",
                    description: Some(
                        "GPDMA channel 9 stop in debug",
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
                    name: "dbg_gpdma10_stop",
                    description: Some(
                        "GPDMA channel 10 stop in debug",
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
                    name: "dbg_gpdma11_stop",
                    description: Some(
                        "GPDMA channel 11 stop in debug",
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
                    name: "dbg_gpdma12_stop",
                    description: Some(
                        "GPDMA channel 12 stop in debug",
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
                    name: "dbg_gpdma13_stop",
                    description: Some(
                        "GPDMA channel 13 stop in debug",
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
                    name: "dbg_gpdma14_stop",
                    description: Some(
                        "GPDMA channel 14 stop in debug",
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
                    name: "dbg_gpdma15_stop",
                    description: Some(
                        "GPDMA channel 15 stop in debug",
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
            ],
        },
        FieldSet {
            name: "Ahb3fzr",
            extends: None,
            description: Some(
                "Debug MCU AHB3 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_lpdma0_stop",
                    description: Some(
                        "LPDMA channel 0 stop in debug",
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
                    name: "dbg_lpdma1_stop",
                    description: Some(
                        "LPDMA channel 1 stop in debug",
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
                    name: "dbg_lpdma2_stop",
                    description: Some(
                        "LPDMA channel 2 stop in debug",
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
                    name: "dbg_lpdma3_stop",
                    description: Some(
                        "LPDMA channel 3 stop in debug",
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
            ],
        },
        FieldSet {
            name: "Apb1hfzr",
            extends: None,
            description: Some(
                "Debug MCU APB1H peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_i2c4_stop",
                    description: Some(
                        "I2C4 stop in debug",
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
                    name: "dbg_lptim2_stop",
                    description: Some(
                        "LPTIM2 stop in debug",
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
                "Debug MCU APB1L peripheral freeze\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim2_stop",
                    description: Some(
                        "TIM2 stop in debug",
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
                        "TIM3 stop in debug",
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
                    name: "dbg_tim4_stop",
                    description: Some(
                        "TIM4 stop in debug",
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
                    name: "dbg_tim5_stop",
                    description: Some(
                        "TIM5 stop in debug",
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
                    name: "dbg_tim6_stop",
                    description: Some(
                        "TIM6 stop in debug",
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
                    name: "dbg_tim7_stop",
                    description: Some(
                        "TIM7 stop in debug",
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
                    name: "dbg_wwdg_stop",
                    description: Some(
                        "Window watchdog counter stop in debug",
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
                        "Independent watchdog counter stop in debug",
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
                        "I2C1 SMBUS timeout stop in debug",
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
                    name: "dbg_i2c2_stop",
                    description: Some(
                        "I2C2 SMBUS timeout stop in debug",
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
            name: "Apb2fzr",
            extends: None,
            description: Some(
                "Debug MCU APB2 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim1_stop",
                    description: Some(
                        "TIM1 counter stopped when core is\r halted",
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
                    name: "dbg_tim8_stop",
                    description: Some(
                        "TIM8 stop in debug",
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
                    name: "dbg_tim15_stop",
                    description: Some(
                        "TIM15 counter stopped when core is\r halted",
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
                    name: "dbg_tim16_stop",
                    description: Some(
                        "TIM16 counter stopped when core is\r halted",
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
                        "DBG_TIM17_STOP",
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
            name: "Apb3fzr",
            extends: None,
            description: Some(
                "Debug MCU APB3 peripheral freeze register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_i2c3_stop",
                    description: Some(
                        "I2C3 stop in debug",
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
                        "LPTIM1 stop in debug",
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
                    name: "dbg_lptim3_stop",
                    description: Some(
                        "LPTIM3 stop in debug",
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
                    name: "dbg_lptim4_stop",
                    description: Some(
                        "LPTIM4 stop in debug",
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
                    name: "dbg_rtc_stop",
                    description: Some(
                        "RTC stop in debug",
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
                "Debug MCU CoreSight component identity register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [7:0]",
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
                "Debug MCU CoreSight component identity register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [11:8]",
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
                        "component identification bits [15:12] - component class",
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
                "Debug MCU CoreSight component identity register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [23:16]",
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
                "Debug MCU CoreSight component identity register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [31:24]",
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
                "Debug MCU configuration\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_stop",
                    description: Some(
                        "Debug Stop mode",
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
                        "Debug Standby mode",
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
                    name: "trace_ioen",
                    description: Some(
                        "Trace pin assignment\r control",
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
                    name: "trace_en",
                    description: Some(
                        "trace port and clock\r enable",
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
                    name: "trace_mode",
                    description: Some(
                        "Trace pin assignment\r control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DbgmcuDbgAuthDevice",
            extends: None,
            description: Some(
                "DBGMCU debug device authentication register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_id",
                    description: Some(
                        "Device specific ID\r \tDevice specific ID used for RDP regression.",
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
            name: "DbgmcuDbgAuthHost",
            extends: None,
            description: Some(
                "DBGMCU debug host authentication register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_key",
                    description: Some(
                        "Device authentication key\r \tThe device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.",
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
            name: "DbgmcuSr",
            extends: None,
            description: Some(
                "DBGMCU status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap_present",
                    description: Some(
                        "Bit n identifies whether access port AP n is present in device\r \tBit n\u{a0}=\u{a0}0: APn absent\r \tBit n\u{a0}=\u{a0}1: APn present",
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
                    name: "ap_locked",
                    description: Some(
                        "DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)\r \tBit n\u{a0}=\u{a0}0: APn locked\r \tBit n\u{a0}=\u{a0}1: APn enabled",
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
            ],
        },
        FieldSet {
            name: "Idcode",
            extends: None,
            description: Some(
                "DBGMCU_IDCODE",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_id",
                    description: Some(
                        "Device dentification",
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
                        "Revision",
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
                "Debug MCU CoreSight peripheral identity register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "part number bits [7:0]",
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
                "Debug MCU CoreSight peripheral identity register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "part number bits [11:8]",
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
                "Debug MCU CoreSight peripheral identity register 2",
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
                        "component revision number",
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
                "Debug MCU CoreSight peripheral identity register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmod",
                    description: Some(
                        "customer modified",
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
                        "metal fix version",
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
                "Debug MCU CoreSight peripheral identity register 4",
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
                    name: "kcount_4",
                    description: Some(
                        "register file size",
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
    ],
    enums: &[],
};
