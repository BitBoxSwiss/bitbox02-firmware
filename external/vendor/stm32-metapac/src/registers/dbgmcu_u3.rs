
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "DBGMCU Address block.",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: None,
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
                        "DBGMCU configuration register.",
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
                        "DBGMCU APB1L peripheral freeze register.",
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
                        "DBGMCU APB1H peripheral freeze register.",
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
                        "DBGMCU APB2 peripheral freeze register.",
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
                        "DBGMCU APB3 peripheral freeze register.",
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
                        "DBGMCU AHB1 peripheral freeze register.",
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
                    name: "dbgmcu_sr",
                    description: Some(
                        "DBGMCU status register.",
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
                        "DBGMCU debug host authentication register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                        "DBGMCU debug device authentication register.",
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
                        "DBGMCU CoreSight peripheral identity register 4.",
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
                        "DBGMCU CoreSight peripheral identity register 0.",
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
                        "DBGMCU CoreSight peripheral identity register 1.",
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
                        "DBGMCU CoreSight peripheral identity register 2.",
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
                        "DBGMCU CoreSight peripheral identity register 3.",
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
                        "DBGMCU CoreSight component identity register 0.",
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
                        "DBGMCU CoreSight component identity register 1.",
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
                        "DBGMCU CoreSight component identity register 2.",
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
                        "DBGMCU CoreSight component identity register 3.",
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
                "DBGMCU AHB1 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_gpdma0_stop",
                    description: Some(
                        "None 0: normal operation. GPDMA channel 0 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 0 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 1 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 1 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 2 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 2 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 3 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 3 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 4 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 4 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 5 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 5 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 6 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 6 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 7 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 7 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 8 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 8 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 9 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 9 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 10 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 10 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. GPDMA channel 11 continues to operate while CPU is in debug mode. 1: stop in debug. GPDMA channel 11 is frozen while CPU is in debug mode.",
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
            ],
        },
        FieldSet {
            name: "Apb1hfzr",
            extends: None,
            description: Some(
                "DBGMCU APB1H peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_lptim2_stop",
                    description: Some(
                        "None 0: normal operation. LPTIM2 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM2 is frozen while CPU is in debug mode.",
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
                "DBGMCU APB1L peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim2_stop",
                    description: Some(
                        "None 0: normal operation. TIM2 continues to operate while CPU is in debug mode. 1: stop in debug. TIM2 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. TIM3 continues to operate while CPU is in debug mode. 1: stop in debug. TIM3 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. TIM4 continues to operate while CPU is in debug mode. 1: stop in debug. TIM4 is frozen while CPU is in debug mode.",
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
                    name: "dbg_tim6_stop",
                    description: Some(
                        "None 0: normal operation. TIM6 continues to operate while CPU is in debug mode. 1: stop in debug. TIM6 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. TIM7 continues to operate while CPU is in debug mode. 1: stop in debug. TIM7 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. WWDG continues to operate while CPU is in debug mode. 1: stop in debug. WWDG is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. IWDG continues to operate while CPU is in debug mode. 1: stop in debug. IWDG is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode. 1: stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode.",
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
                    name: "dbg_i3c1_stop",
                    description: Some(
                        "None 0: normal operation. I3C1 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C1 timeout is frozen while CPU is in debug mode.",
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
                    name: "dbg_rtc_stop",
                    description: Some(
                        "None 0: normal operation. RTC continues to operate while CPU is in debug mode. 1: stop in debug. RTC is frozen while CPU is in debug mode.",
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
            name: "Apb2fzr",
            extends: None,
            description: Some(
                "DBGMCU APB2 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim1_stop",
                    description: Some(
                        "None 0: normal operation. TIM1 continues to operate while CPU is in debug mode. 1: stop in debug. TIM1 is frozen while CPU is in debug mode.",
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
                    name: "dbg_tim15_stop",
                    description: Some(
                        "None 0: normal operation. TIM15 continues to operate while CPU is in debug mode. 1: stop in debug. TIM15 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. TIM16 continues to operate while CPU is in debug mode. 1: stop in debug. TIM16 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. TIM17 continues to operate while CPU is in debug mode. 1: stop in debug. TIM17 is frozen while CPU is in debug mode.",
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
                    name: "dbg_i3c2_stop",
                    description: Some(
                        "None 0: normal operation. I3C2 timeout continues to operate while CPU is in debug mode. 1: stop in debug. I3C2 timeout is frozen while CPU is in debug mode.",
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
            name: "Apb3fzr",
            extends: None,
            description: Some(
                "DBGMCU APB3 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_i2c3_stop",
                    description: Some(
                        "None 0: normal operation. I2C3 continues to operate while CPU is in debug mode. 1: stop in debug. I2C3 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. LPTIM1 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM1 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. LPTIM3 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM3 is frozen while CPU is in debug mode.",
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
                        "None 0: normal operation. LPTIM4 continues to operate while CPU is in debug mode. 1: stop in debug. LPTIM4 is frozen while CPU is in debug mode.",
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
            name: "Cidr0",
            extends: None,
            description: Some(
                "DBGMCU CoreSight component identity register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "None 0x0D: common identification value.",
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
                "DBGMCU CoreSight component identity register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "None 0x0: common identification value.",
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
                        "None 0xF: Non-CoreSight component.",
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
                "DBGMCU CoreSight component identity register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "None 0x05: common identification value.",
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
                "DBGMCU CoreSight component identity register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "None 0xB1: common identification value.",
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
                "DBGMCU configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_stop",
                    description: Some(
                        "All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state. 0: normal operation 1: automatic clock stop disabled.",
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
                        "All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed. 0: normal operation 1: automatic clock stop/power down disabled.",
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
                        "None 0: disabled-trace pins not assigned 1: enabled-trace pins assigned according to the value of TRACE_MODE field.",
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
                        "This bit enables the trace port clock, TRACECK. 0: disabled 1: enabled.",
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
                        "None 0x0: trace pins assigned for asynchronous mode (TRACESWO) 0x1: trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0) 0x2: trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1) 0x3: trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3).",
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
                "DBGMCU debug device authentication register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_id",
                    description: Some(
                        "Device specific ID used for RDP regression.",
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
                "DBGMCU debug host authentication register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auth_key",
                    description: Some(
                        "The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.",
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
                "DBGMCU status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap_present",
                    description: Some(
                        "Bit n=0: APn absent Bit n=1: APn present.",
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
                        "Bit n=0: APn locked Bit n=1: APn enabled.",
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
            name: "Idcode",
            extends: None,
            description: Some(
                "DBGMCU identity code register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_id",
                    description: Some(
                        "None 0x454: STM32U375/385.",
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
                        "None 0x0001: revision A.",
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
                "DBGMCU CoreSight peripheral identity register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "None 0x00: DBGMCU part number.",
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
                "DBGMCU CoreSight peripheral identity register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "None 0x0: DBGMCU part number.",
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
                        "None 0x0: STMicroelectronics JEDEC code.",
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
                "DBGMCU CoreSight peripheral identity register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106id",
                    description: Some(
                        "None 0x2: STMicroelectronics JEDEC code.",
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
                        "None 0x1: designer identification specified by JEDEC.",
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
                        "None 0x0: r0p0.",
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
                "DBGMCU CoreSight peripheral identity register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmod",
                    description: Some(
                        "None 0x0: no customer modifications.",
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
                        "None 0x0: no metal fix.",
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
                "DBGMCU CoreSight peripheral identity register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106con",
                    description: Some(
                        "None 0x0: STMicroelectronics JEDEC code.",
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
                    name: "size",
                    description: Some(
                        "None 0x0: The register file occupies a single 4-Kbyte region.",
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
