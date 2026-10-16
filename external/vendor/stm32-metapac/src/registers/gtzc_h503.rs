
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gtzc1",
            extends: None,
            description: Some(
                "Global privilege controller.",
            ),
            items: &[
                BlockItem {
                    name: "tzsc_privcfgr1",
                    description: Some(
                        "GTZC1 TZSC privilege configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscPrivcfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_privcfgr2",
                    description: Some(
                        "GTZC1 TZSC privilege configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscPrivcfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_privcfgr3",
                    description: Some(
                        "GTZC1 TZSC privilege configuration register 3.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscPrivcfgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_mpcwm4acfgr",
                    description: Some(
                        "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscMpcwm4acfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_mpcwm4ar",
                    description: Some(
                        "GTZC1 TZSC BKPSRAM sub-region A watermark register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscMpcwm4ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_mpcwm4bcfgr",
                    description: Some(
                        "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscMpcwm4bcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzsc_mpcwm4br",
                    description: Some(
                        "GTZC1 TZSC BKPSRAM sub-region B watermark register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TzscMpcwm4br",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mpcbb1_privcfgr",
                    description: Some(
                        "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mpcbb1Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mpcbb2_privcfgr",
                    description: Some(
                        "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mpcbb2Privcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Mpcbb1Privcfgr",
            extends: None,
            description: Some(
                "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv0",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv1",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv2",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv3",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv4",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv5",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv6",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv7",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv8",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv9",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv10",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv11",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv12",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv13",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv14",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv15",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv16",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv17",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv18",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv19",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv20",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv21",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv22",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv23",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv24",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv25",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv26",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv27",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv28",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv29",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv30",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv31",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
            name: "Mpcbb2Privcfgr",
            extends: None,
            description: Some(
                "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv0",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv1",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv2",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv3",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv4",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv5",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv6",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv7",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv8",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv9",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv10",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv11",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv12",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv13",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv14",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv15",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv16",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv17",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv18",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv19",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv20",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv21",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv22",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv23",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv24",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv25",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv26",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv27",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv28",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv29",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv30",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
                    name: "priv31",
                    description: Some(
                        "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).",
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
            name: "TzscMpcwm4acfgr",
            extends: None,
            description: Some(
                "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sren",
                    description: Some(
                        "Sub-region z enable.",
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
                    name: "srlock",
                    description: Some(
                        "Sub-region z lock This bit, once set, can be cleared only by a system reset.",
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
                    name: "priv_",
                    description: Some(
                        "Privileged sub-region z This bit is taken into account only if SREN is set.",
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
            name: "TzscMpcwm4ar",
            extends: None,
            description: Some(
                "GTZC1 TZSC BKPSRAM sub-region A watermark register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "suba_start",
                    description: Some(
                        "Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "suba_length",
                    description: Some(
                        "Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in TZSC_MPCMWACFGR is cleared).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TzscMpcwm4bcfgr",
            extends: None,
            description: Some(
                "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sren",
                    description: Some(
                        "Sub-region z enable.",
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
                    name: "srlock",
                    description: Some(
                        "Sub-region z lock This bit, once set, can be cleared only by a system reset.",
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
                    name: "priv_",
                    description: Some(
                        "Privileged sub-region z This bit is taken into account only if SREN is set.",
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
            name: "TzscMpcwm4br",
            extends: None,
            description: Some(
                "GTZC1 TZSC BKPSRAM sub-region B watermark register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subb_start",
                    description: Some(
                        "Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "subb_length",
                    description: Some(
                        "Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in TZSC_MPCMWBCFGR is cleared).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TzscPrivcfgr1",
            extends: None,
            description: Some(
                "GTZC1 TZSC privilege configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2priv",
                    description: Some(
                        "privileged access mode for TIM2.",
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
                    name: "tim3priv",
                    description: Some(
                        "privileged access mode for TIM3.",
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
                    name: "tim6priv",
                    description: Some(
                        "privileged access mode for TIM6.",
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
                    name: "tim7priv",
                    description: Some(
                        "privileged access mode for TIM7.",
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
                    name: "wwdgpriv",
                    description: Some(
                        "privileged access mode for WWDG.",
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
                    name: "iwdgpriv",
                    description: Some(
                        "privileged access mode for IWDG.",
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
                    name: "spi2priv",
                    description: Some(
                        "privileged access mode for SPI2.",
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
                    name: "spi3priv",
                    description: Some(
                        "privileged access mode for SPI3.",
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
                    name: "usart2priv",
                    description: Some(
                        "privileged access mode for USART2.",
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
                    name: "usart3priv",
                    description: Some(
                        "privileged access mode for USART3.",
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
                    name: "i2c1priv",
                    description: Some(
                        "privileged access mode for I2C1.",
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
                    name: "i2c2priv",
                    description: Some(
                        "privileged access mode for I2C2.",
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
                    name: "i3c1priv",
                    description: Some(
                        "privileged access mode for I3C1.",
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
                    name: "crspriv",
                    description: Some(
                        "privileged access mode for CRS.",
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
                    name: "dac1priv",
                    description: Some(
                        "privileged access mode for DAC1.",
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
                    name: "dtspriv",
                    description: Some(
                        "privileged access mode for DTS.",
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
                    name: "lptim2priv",
                    description: Some(
                        "privileged access mode for LPTIM2.",
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
            name: "TzscPrivcfgr2",
            extends: None,
            description: Some(
                "GTZC1 TZSC privilege configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fdcan1priv",
                    description: Some(
                        "privileged access mode for FDCAN1.",
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
                    name: "opamppriv",
                    description: Some(
                        "privileged access mode for OPAMP.",
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
                    name: "comppriv",
                    description: Some(
                        "privileged access mode for COMP.",
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
                    name: "tim1priv",
                    description: Some(
                        "privileged access mode for TIM1.",
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
                    name: "spi1priv",
                    description: Some(
                        "privileged access mode for SPI1.",
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
                    name: "usart1priv",
                    description: Some(
                        "privileged access mode for USART1.",
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
                    name: "usbfspriv",
                    description: Some(
                        "privileged access mode for USBSF.",
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
                    name: "lpuart1priv",
                    description: Some(
                        "privileged access mode for LPUART.",
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
                    name: "lptim1priv",
                    description: Some(
                        "privileged access mode for LPTIM1.",
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
            name: "TzscPrivcfgr3",
            extends: None,
            description: Some(
                "GTZC1 TZSC privilege configuration register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i3c2priv",
                    description: Some(
                        "privileged access mode for I3C2.",
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
                    name: "crcpriv",
                    description: Some(
                        "privileged access mode for CRC.",
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
                    name: "icachepriv",
                    description: Some(
                        "privileged access mode for ICACHE.",
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
                    name: "adc1priv",
                    description: Some(
                        "privileged access mode for ADC1.",
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
                    name: "hashpriv",
                    description: Some(
                        "privileged access mode for HASH.",
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
                    name: "rngpriv",
                    description: Some(
                        "privileged access mode for RNG.",
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
                    name: "ramcfgpriv",
                    description: Some(
                        "privileged access mode for RAMSCFG.",
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
    ],
    enums: &[],
};
