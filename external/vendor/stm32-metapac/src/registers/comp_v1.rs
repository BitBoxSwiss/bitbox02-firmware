
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("Comparator v1. (RM0444 18)"),
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
        description: Some("Comparator control and status register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "en",
                description: Some("COMP enable bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "inmsel",
                description: Some("Comparator signal selector for inverting input INM."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 4,
                array: None,
                enumm: None,
            },
            Field {
                name: "inpsel",
                description: Some("Comparator signal selector for non-inverting input INP."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                bit_size: 2,
                array: None,
                enumm: None,
            },
            Field {
                name: "winmode",
                description: Some("Comparator non-inverting input selector for window mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "winout",
                description: Some("Comparator output selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "polarity",
                description: Some("Comparator polarity selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                bit_size: 1,
                array: None,
                enumm: Some("Polarity"),
            },
            Field {
                name: "hyst",
                description: Some("Comparator hysteresis selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 2,
                array: None,
                enumm: Some("Hyst"),
            },
            Field {
                name: "pwrmode",
                description: Some("Comparator power mode selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 2,
                array: None,
                enumm: Some("Pwrmode"),
            },
            Field {
                name: "blanksel",
                description: Some("Comparator blanking source selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                bit_size: 5,
                array: None,
                enumm: Some("Blanksel"),
            },
            Field {
                name: "value_do_not_set",
                description: Some("Comparator output status. (READ ONLY)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lock",
                description: Some("CSR register lock."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Blanksel",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1OC4",
                    description: Some("TIM1 OC4"),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM1OC5",
                    description: Some("TIM1 OC5"),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2OC3",
                    description: Some("TIM2 OC3"),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM3OC3",
                    description: Some("TIM3 OC3"),
                    value: 8,
                },
                EnumVariant {
                    name: "TIM15OC2",
                    description: Some("TIM15 OC2"),
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
                    name: "NON_INVERTED",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: None,
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
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_SPEED",
                    description: None,
                    value: 1,
                },
            ],
        },
    ],
};
