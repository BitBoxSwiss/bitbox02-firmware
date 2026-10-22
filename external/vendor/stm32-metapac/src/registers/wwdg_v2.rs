
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Wwdg",
        extends: None,
        description: Some("Window watchdog"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cfr",
                description: Some("Configuration register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfr",
            extends: None,
            description: Some("Configuration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "w",
                    description: Some("7-bit window value"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewi",
                    description: Some("Early wakeup interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgtb",
                    description: Some("Timer base"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Wdgtb"),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some("7-bit counter (MSB to LSB)"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdga",
                    description: Some("Activation bit (true is enabled, false is disabled)"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[Field {
                name: "ewif",
                description: Some("Early wakeup interrupt flag"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[Enum {
        name: "Wdgtb",
        description: None,
        bit_size: 3,
        variants: &[
            EnumVariant {
                name: "DIV1",
                description: Some("Counter clock (PCLK1 div 4096) div 1"),
                value: 0,
            },
            EnumVariant {
                name: "DIV2",
                description: Some("Counter clock (PCLK1 div 4096) div 2"),
                value: 1,
            },
            EnumVariant {
                name: "DIV4",
                description: Some("Counter clock (PCLK1 div 4096) div 4"),
                value: 2,
            },
            EnumVariant {
                name: "DIV8",
                description: Some("Counter clock (PCLK1 div 4096) div 8"),
                value: 3,
            },
            EnumVariant {
                name: "DIV16",
                description: Some("Counter clock (PCLK1 div 4096) div 16"),
                value: 4,
            },
            EnumVariant {
                name: "DIV32",
                description: Some("Counter clock (PCLK1 div 4096) div 32"),
                value: 5,
            },
            EnumVariant {
                name: "DIV64",
                description: Some("Counter clock (PCLK1 div 4096) div 64"),
                value: 6,
            },
            EnumVariant {
                name: "DIV128",
                description: Some("Counter clock (PCLK1 div 4096) div 128"),
                value: 7,
            },
        ],
    }],
};
