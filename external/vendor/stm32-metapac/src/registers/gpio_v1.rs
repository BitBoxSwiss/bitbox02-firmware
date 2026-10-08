
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Gpio",
        extends: None,
        description: Some("General purpose I/O"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Port configuration register low (GPIOn_CRL)"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 4 })),
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "idr",
                description: Some("Port input data register (GPIOn_IDR)"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Idr"),
                }),
            },
            BlockItem {
                name: "odr",
                description: Some("Port output data register (GPIOn_ODR)"),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Odr"),
                }),
            },
            BlockItem {
                name: "bsrr",
                description: Some("Port bit set/reset register (GPIOn_BSRR)"),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Bsrr"),
                }),
            },
            BlockItem {
                name: "brr",
                description: Some("Port bit reset register (GPIOn_BRR)"),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Brr"),
                }),
            },
            BlockItem {
                name: "lckr",
                description: Some("Port configuration lock register"),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lckr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Brr",
            extends: None,
            description: Some("Port bit reset register (GPIOn_BRR)"),
            bit_size: 32,
            fields: &[Field {
                name: "br",
                description: Some("Reset bit"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Bsrr",
            extends: None,
            description: Some("Port bit set/reset register (GPIOn_BSRR)"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bs",
                    description: Some("Set bit"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "br",
                    description: Some("Reset bit"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Port configuration register (GPIOn_CRx)"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some("Port n mode bits"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: Some("Mode"),
                },
                Field {
                    name: "cnf_in",
                    description: Some("Port n configuration bits, for input mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: Some("CnfIn"),
                },
                Field {
                    name: "cnf_out",
                    description: Some("Port n configuration bits, for output mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: Some("CnfOut"),
                },
            ],
        },
        FieldSet {
            name: "Idr",
            extends: None,
            description: Some("Port input data register (GPIOn_IDR)"),
            bit_size: 32,
            fields: &[Field {
                name: "idr",
                description: Some("Port input data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                enumm: Some("Idr"),
            }],
        },
        FieldSet {
            name: "Lckr",
            extends: None,
            description: Some("Port configuration lock register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lck",
                    description: Some("Port configuration locked"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "lckk",
                    description: Some("Port configuration lock key active"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Odr",
            extends: None,
            description: Some("Port output data register (GPIOn_ODR)"),
            bit_size: 32,
            fields: &[Field {
                name: "odr",
                description: Some("Port output data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                enumm: Some("Odr"),
            }],
        },
    ],
    enums: &[
        Enum {
            name: "CnfIn",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some("Analog mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "FLOATING",
                    description: Some("Floating input (reset state)"),
                    value: 1,
                },
                EnumVariant {
                    name: "PULL",
                    description: Some("Input with pull-up/pull-down"),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "CnfOut",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PUSH_PULL",
                    description: Some("Push-Pull mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "OPEN_DRAIN",
                    description: Some("Open Drain-Mode"),
                    value: 1,
                },
                EnumVariant {
                    name: "ALT_PUSH_PULL",
                    description: Some("Alternate Function Push-Pull Mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "ALT_OPEN_DRAIN",
                    description: Some("Alternate Function Open-Drain Mode"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Idr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some("Input is logic low"),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("Input is logic high"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some("Input mode (reset state)"),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT10MHZ",
                    description: Some("Output mode 10 MHz"),
                    value: 1,
                },
                EnumVariant {
                    name: "OUTPUT2MHZ",
                    description: Some("Output mode 2 MHz"),
                    value: 2,
                },
                EnumVariant {
                    name: "OUTPUT50MHZ",
                    description: Some("Output mode 50 MHz"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Odr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some("Set output to logic low"),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("Set output to logic high"),
                    value: 1,
                },
            ],
        },
    ],
};
