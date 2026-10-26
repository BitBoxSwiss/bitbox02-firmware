
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Icache",
            extends: None,
            description: Some(
                "Instruction Cache Control Registers.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "ICACHE control register.",
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
                    name: "sr",
                    description: Some(
                        "ICACHE status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "ICACHE interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fcr",
                    description: Some(
                        "ICACHE flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hmonr",
                    description: Some(
                        "ICACHE hit monitor register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mmonr",
                    description: Some(
                        "ICACHE miss monitor register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmonr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crr",
                    description: Some(
                        "Cluster CRR%s, container region configuration registers.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "ICACHE control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "EN.",
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
                    name: "cacheinv",
                    description: Some(
                        "Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect.",
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
                    name: "waysel",
                    description: Some(
                        "This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waysel",
                    ),
                },
                Field {
                    name: "hitmen",
                    description: Some(
                        "Hit monitor enable.",
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
                    name: "missmen",
                    description: Some(
                        "Miss monitor enable.",
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
                    name: "hitmrst",
                    description: Some(
                        "Hit monitor reset.",
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
                    name: "missmrst",
                    description: Some(
                        "Miss monitor reset.",
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
            name: "Crr",
            extends: None,
            description: Some(
                "ICACHE region configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baseaddr",
                    description: Some(
                        "base address for region.",
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
                    name: "rsize",
                    description: Some(
                        "size for region.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Rsize",
                    ),
                },
                Field {
                    name: "ren",
                    description: Some(
                        "enable for region.",
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
                    name: "remapaddr",
                    description: Some(
                        "remapped address for region.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstsel",
                    description: Some(
                        "AHB cache master selection for region.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mstsel",
                    ),
                },
                Field {
                    name: "hburst",
                    description: Some(
                        "output burst type for region.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hburst",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fcr",
            extends: None,
            description: Some(
                "ICACHE flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbsyendf",
                    description: Some(
                        "Clear busy end flag.",
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
                    name: "cerrf",
                    description: Some(
                        "Clear ERRF flag in SR.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "ICACHE interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsyendie",
                    description: Some(
                        "Interrupt enable on busy end.",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt on cache error.",
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
            name: "Mmonr",
            extends: None,
            description: Some(
                "ICACHE miss monitor register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "missmon",
                    description: Some(
                        "Miss monitor register.",
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
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "ICACHE status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busyf",
                    description: Some(
                        "cache busy executing a full invalidate CACHEINV operation.",
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
                    name: "bsyendf",
                    description: Some(
                        "full invalidate CACHEINV operation finished.",
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
                    name: "errf",
                    description: Some(
                        "an error occurred during the operation.",
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
    ],
    enums: &[
        Enum {
            name: "Hburst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "WRAP",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "INCREMENT",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mstsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MASTER1SELECTED",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MASTER2SELECTED",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rsize",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MEGA_BYTES2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "MEGA_BYTES4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "MEGA_BYTES8",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "MEGA_BYTES16",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "MEGA_BYTES32",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "MEGA_BYTES64",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "MEGA_BYTES128",
                    description: None,
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Waysel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIRECT_MAPPED",
                    description: Some(
                        "direct mapped cache (1-way cache)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NWAY_SET_ASSOCIATIVE",
                    description: Some(
                        "n-way set associative cache (reset value)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
