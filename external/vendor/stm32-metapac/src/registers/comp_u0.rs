
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
                name: "inmsel",
                description: Some("Input minus selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 4,
                array: None,
                enumm: None,
            },
            Field {
                name: "inpsel",
                description: Some("Input plus selection bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                bit_size: 3,
                array: None,
                enumm: None,
            },
            Field {
                name: "winmode",
                description: Some("Comparator 1 noninverting input selector for window mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                bit_size: 1,
                array: None,
                enumm: Some("WindowMode"),
            },
            Field {
                name: "winout",
                description: Some("Comparator 1 output selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                bit_size: 1,
                array: None,
                enumm: Some("WindowOut"),
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
                name: "pwrmode",
                description: Some("Power Mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 2,
                array: None,
                enumm: Some("Pwrmode"),
            },
            Field {
                name: "blanksel",
                description: Some("Blanking source selection bits."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                bit_size: 5,
                array: None,
                enumm: Some("Blanking"),
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
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "NO_BLANKING",
                    description: Some("No blanking."),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1OC4",
                    description: Some("TIM1 OC4 enabled as blanking source"),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM1OC5",
                    description: Some("TIM1 OC5 enabled as blanking source"),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2OC3",
                    description: Some("TIM5 OC3 enabled as blanking source"),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM3OC3",
                    description: Some("TIM3 OC3 enabled as blanking source"),
                    value: 8,
                },
                EnumVariant {
                    name: "TIM15OC2",
                    description: Some("TIM15 OC2 enabled as blanking source"),
                    value: 16,
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
                    description: Some("Very-low speed / ultra-low power."),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WindowMode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "THIS_INPSEL",
                    description: Some("Signal selected with INPSEL[2:0] bitfield of this register."),
                    value: 0,
                },
                EnumVariant {
                    name: "OTHER_INPSEL",
                    description: Some(
                        "Signal selected with INPSEL[2:0] bitfield of the other register (required for window mode).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WindowOut",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "COMP1_VALUE",
                    description: Some("Comparator 1 value."),
                    value: 0,
                },
                EnumVariant {
                    name: "COMP1_VALUE_XOR_COMP2_VALUE",
                    description: Some("Comparator 1 value XOR comparator 2 value (required for window mode)."),
                    value: 1,
                },
            ],
        },
    ],
};
