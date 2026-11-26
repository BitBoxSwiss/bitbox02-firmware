
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "DBGMCU register block.",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: Some(
                        "DBGMCU device ID code register.",
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
                    name: "apb1fzr",
                    description: Some(
                        "DBGMCU APB1 freeze register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2fzr",
                    description: Some(
                        "DBG APB2 freeze register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "sr",
                    description: Some(
                        "DBGMCU status register.",
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
                        "DBGMCU debug authentication mailbox host register.",
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
                        "DBGMCU debug authentication mailbox device register.",
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
                    name: "pidr4",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 4.",
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
                        "DBGMCU CoreSight peripheral identity register 0.",
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
                        "DBGMCU CoreSight peripheral identity register 1.",
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
                        "DBGMCU CoreSight peripheral identity register 2.",
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
                        "DBGMCU CoreSight peripheral identity register 3.",
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
                        "DBGMCU CoreSight component identity register 0.",
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
                        "DBGMCU CoreSight component identity register 1.",
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
                        "DBGMCU CoreSight component identity register 2.",
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
                        "DBGMCU CoreSight component identity register 3.",
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
            name: "Apb1fzr",
            extends: None,
            description: Some(
                "DBGMCU APB1 freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim2_stop",
                    description: Some(
                        "TIM2 stop in debug.",
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
                        "TIM3 stop in debug.",
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
                    name: "dbg_tim6_stop",
                    description: Some(
                        "TIM6 stop in debug.",
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
                        "TIM7 stop in debug.",
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
                    name: "dbg_rtc_stop",
                    description: Some(
                        "RTC stop in debug.",
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
                    name: "dbg_wwdg_stop",
                    description: Some(
                        "WWDG stop in debug.",
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
                        "IWDG stop in debug.",
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
                    name: "dbg_lptim2_stop",
                    description: Some(
                        "LPTIM2 stop in debug.",
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
                    name: "dbg_lptim1_stop",
                    description: Some(
                        "LPTIM1 stop in debug.",
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
            name: "Apb2fzr",
            extends: None,
            description: Some(
                "DBG APB2 freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_tim1_stop",
                    description: Some(
                        "TIM1 stop in debug.",
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
                        "TIM15 stop in debug.",
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
                        "TIM16 stop in debug.",
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
                        "LPTIM3 stop in debug.",
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
                        "component identification bits [7:0].",
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
                        "component identification bits [11:8].",
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
                        "component identification bits [15:12] - component class.",
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
                        "component identification bits [23:16].",
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
                        "component identification bits [31:24].",
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
                        "Debug Stop mode Debug options in Stop mode.",
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
                        "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode.",
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
            name: "DbgAuthDevice",
            extends: None,
            description: Some(
                "DBGMCU debug authentication mailbox device register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "message",
                    description: Some(
                        "Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register.",
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
                "DBGMCU debug authentication mailbox host register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "message",
                    description: Some(
                        "Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register.",
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
                "DBGMCU device ID code register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_id",
                    description: Some(
                        "Device identifier This field indicates the device ID.",
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
                        "Revision identifier This field indicates the revision of the device.",
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
                        "part number bits [7:0].",
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
                        "part number bits [11:8].",
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
                        "JEP106 identity code bits [3:0].",
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
                        "JEP106 identity code bits [6:4].",
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
                        "JEDEC assigned value.",
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
                        "component revision number.",
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
                        "customer modified.",
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
                        "metal fix version.",
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
                        "JEP106 continuation code.",
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
                        "register file size.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "DBGMCU status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap1_present",
                    description: Some(
                        "Identifies whether access port AP1 is present in device.",
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
                    name: "ap0_present",
                    description: Some(
                        "Identifies whether access port AP0 is present in device.",
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
                    name: "ap1_enabled",
                    description: Some(
                        "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked).",
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
                    name: "ap0_enabled",
                    description: Some(
                        "Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked).",
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
    ],
    enums: &[],
};
