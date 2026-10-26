
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Dlyb",
        extends: None,
        description: Some("Delay Block for specified pheripheral"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("DLYB control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cfgr",
                description: Some("DLYB configuration register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some("DLYB configuration register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sel",
                    description: Some("Select the phase for the Output clock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "unit",
                    description: Some("Delay Defines the delay of a Unit delay cell."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lng",
                    description: Some("Delay line length value."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lngf",
                    description: Some("Length valid flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("DLYB control register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den",
                    description: Some("Delay block enable bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sen",
                    description: Some("Sampler length enable bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
