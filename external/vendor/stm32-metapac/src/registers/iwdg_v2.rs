
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Iwdg",
        extends: None,
        description: Some("Independent watchdog"),
        items: &[
            BlockItem {
                name: "kr",
                description: Some("Key register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Kr"),
                }),
            },
            BlockItem {
                name: "pr",
                description: Some("Prescaler register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pr"),
                }),
            },
            BlockItem {
                name: "rlr",
                description: Some("Reload register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rlr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "winr",
                description: Some("Window register"),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Winr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Kr",
            extends: None,
            description: Some("Key register"),
            bit_size: 32,
            fields: &[Field {
                name: "key",
                description: Some("Key value (write only, read 0000h)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: Some("Key"),
            }],
        },
        FieldSet {
            name: "Pr",
            extends: None,
            description: Some("Prescaler register"),
            bit_size: 32,
            fields: &[Field {
                name: "pr",
                description: Some("Prescaler divider"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 3,
                array: None,
                enumm: Some("Pr"),
            }],
        },
        FieldSet {
            name: "Rlr",
            extends: None,
            description: Some("Reload register"),
            bit_size: 32,
            fields: &[Field {
                name: "rl",
                description: Some("Watchdog counter reload value"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvu",
                    description: Some("Watchdog prescaler value update"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rvu",
                    description: Some("Watchdog counter reload value update"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wvu",
                    description: Some("Watchdog counter window value update"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Winr",
            extends: None,
            description: Some("Window register"),
            bit_size: 32,
            fields: &[Field {
                name: "win",
                description: Some("Watchdog counter window value"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Key",
            description: None,
            bit_size: 16,
            variants: &[
                EnumVariant {
                    name: "ENABLE",
                    description: Some("Enable access to PR, RLR and WINR registers (0x5555)"),
                    value: 21845,
                },
                EnumVariant {
                    name: "RESET",
                    description: Some("Reset the watchdog value (0xAAAA)"),
                    value: 43690,
                },
                EnumVariant {
                    name: "START",
                    description: Some("Start the watchdog (0xCCCC)"),
                    value: 52428,
                },
            ],
        },
        Enum {
            name: "Pr",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIVIDE_BY4",
                    description: Some("Divider /4"),
                    value: 0,
                },
                EnumVariant {
                    name: "DIVIDE_BY8",
                    description: Some("Divider /8"),
                    value: 1,
                },
                EnumVariant {
                    name: "DIVIDE_BY16",
                    description: Some("Divider /16"),
                    value: 2,
                },
                EnumVariant {
                    name: "DIVIDE_BY32",
                    description: Some("Divider /32"),
                    value: 3,
                },
                EnumVariant {
                    name: "DIVIDE_BY64",
                    description: Some("Divider /64"),
                    value: 4,
                },
                EnumVariant {
                    name: "DIVIDE_BY128",
                    description: Some("Divider /128"),
                    value: 5,
                },
                EnumVariant {
                    name: "DIVIDE_BY256",
                    description: Some("Divider /256"),
                    value: 6,
                },
                EnumVariant {
                    name: "DIVIDE_BY256BIS",
                    description: Some("Divider /256"),
                    value: 7,
                },
            ],
        },
    ],
};
