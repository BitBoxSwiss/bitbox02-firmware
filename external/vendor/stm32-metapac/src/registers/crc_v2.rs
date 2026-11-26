
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some("Cyclic Redundancy Check calculation unit"),
        items: &[
            BlockItem {
                name: "dr16",
                description: Some("Data register - half-word sized"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr32",
                description: Some("Data register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr8",
                description: Some("Data register - byte sized"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "idr",
                description: Some("Independent Data register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "init",
                description: Some("Initial CRC value"),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Cr",
        extends: None,
        description: Some("Control register"),
        bit_size: 32,
        fields: &[
            Field {
                name: "reset",
                description: Some("RESET bit"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "polysize",
                description: Some("Polynomial size"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 2,
                array: None,
                enumm: Some("Polysize"),
            },
            Field {
                name: "rev_in",
                description: Some("Reverse input data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                bit_size: 2,
                array: None,
                enumm: Some("RevIn"),
            },
            Field {
                name: "rev_out",
                description: Some("Reverse output data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 1,
                array: None,
                enumm: Some("RevOut"),
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Polysize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "POLYSIZE32",
                    description: Some("32-bit polynomial"),
                    value: 0,
                },
                EnumVariant {
                    name: "POLYSIZE16",
                    description: Some("16-bit polynomial"),
                    value: 1,
                },
                EnumVariant {
                    name: "POLYSIZE8",
                    description: Some("8-bit polynomial"),
                    value: 2,
                },
                EnumVariant {
                    name: "POLYSIZE7",
                    description: Some("7-bit polynomial"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "RevIn",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some("Bit order not affected"),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTE",
                    description: Some("Bit reversal done by byte"),
                    value: 1,
                },
                EnumVariant {
                    name: "HALF_WORD",
                    description: Some("Bit reversal done by half-word"),
                    value: 2,
                },
                EnumVariant {
                    name: "WORD",
                    description: Some("Bit reversal done by word"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "RevOut",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some("Bit order not affected"),
                    value: 0,
                },
                EnumVariant {
                    name: "REVERSED",
                    description: Some("Bit reversed output"),
                    value: 1,
                },
            ],
        },
    ],
};
