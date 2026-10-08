
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("Comparator."),
        items: &[BlockItem {
            name: "csr",
            description: Some("Comparator control and status register."),
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
                description: Some("Enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "pwrmode",
                description: Some("Power Mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                bit_size: 2,
                array: None,
                enumm: Some("Pwrmode"),
            },
            Field {
                name: "inmsel",
                description: Some("Input minus selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 3,
                array: None,
                enumm: None,
            },
            Field {
                name: "inpsel",
                description: Some("Input plus selection bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 2,
                array: None,
                enumm: None,
            },
            Field {
                name: "polarity",
                description: Some("Polarity selection bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                bit_size: 1,
                array: None,
                enumm: Some("Polarity"),
            },
            Field {
                name: "hyst",
                description: Some("Hysteresis selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 2,
                array: None,
                enumm: Some("Hyst"),
            },
            Field {
                name: "blanking",
                description: Some("Blanking source selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 3,
                array: None,
                enumm: Some("Blanking"),
            },
            Field {
                name: "brgen",
                description: Some("Scaler bridge enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "scalen",
                description: Some("Voltage scaler enable bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "inmesel",
                description: Some("Input minus extended selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                bit_size: 2,
                array: None,
                enumm: None,
            },
            Field {
                name: "value",
                description: Some("Output status bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lock",
                description: Some("Register lock bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Blanking",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_BLANKING",
                    description: Some("No blanking."),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1OC5",
                    description: Some("TIM1 OC5 selected as blanking source."),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2OC3",
                    description: Some("TIM2 OC3 selected as blanking source."),
                    value: 2,
                },
            ],
        },
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
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Polarity",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_INVERTED",
                    description: Some("Output is not inverted."),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some("Output is inverted."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pwrmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HIGH_SPEED",
                    description: Some("High speed / full power."),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_SPEED",
                    description: Some("Medium speed / medium power."),
                    value: 1,
                },
                EnumVariant {
                    name: "LOW_SPEED",
                    description: Some("Low speed / low power."),
                    value: 2,
                },
                EnumVariant {
                    name: "VERY_LOW_SPEED",
                    description: Some("Very-low speed / ultra-low power."),
                    value: 3,
                },
            ],
        },
    ],
};
