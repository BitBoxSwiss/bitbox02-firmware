
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("General purpose comparators."),
        items: &[BlockItem {
            name: "csr",
            description: Some("control and status register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Csr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Csr",
        extends: None,
        description: Some("control and status register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "en",
                description: Some("Comparator enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "inp_dac",
                description: Some("Comparator 1 non inverting input connection to DAC output. Only available on COMP1"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "mode",
                description: Some("Comparator mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                bit_size: 2,
                array: None,
                enumm: Some("Mode"),
            },
            Field {
                name: "insel",
                description: Some("Comparator inverting input selection."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 3,
                array: None,
                enumm: None,
            },
            Field {
                name: "wndwen",
                description: Some("Window mode enable. Only available on COMP2"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "outsel",
                description: Some("Comparator output selection."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                bit_size: 3,
                array: None,
                enumm: None,
            },
            Field {
                name: "pol",
                description: Some("Comparator output polarity."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "hyst",
                description: Some("Comparator hysteresis."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                bit_size: 2,
                array: None,
                enumm: Some("Hyst"),
            },
            Field {
                name: "out",
                description: Some("Comparator output."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lock",
                description: Some("Comparator lock."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Hyst",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some("Low hysteresis"),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some("Medium hysteresis"),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("High hysteresis"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HIGH_SPEED",
                    description: Some("High Speed mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_SPEED",
                    description: Some("Medium Speed mode"),
                    value: 1,
                },
                EnumVariant {
                    name: "LOW_SPEED",
                    description: Some("Low Speed mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "VERY_LOW_SPEED",
                    description: Some("Very Low  Speed mode"),
                    value: 3,
                },
            ],
        },
    ],
};
