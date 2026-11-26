
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Pwr",
        extends: None,
        description: Some("Power control"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("power control register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "csr",
                description: Some("power control/status register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("power control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpds",
                    description: Some("Low-power deep sleep"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdds",
                    description: Some("Power down deepsleep"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Pdds"),
                },
                Field {
                    name: "cwuf",
                    description: Some("Clear wakeup flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csbf",
                    description: Some("Clear standby flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbp",
                    description: Some("Disable backup domain write protection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("power control/status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some("Wakeup flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbf",
                    description: Some("Standby flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewup",
                    description: Some("Enable WKUP pin 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[Enum {
        name: "Pdds",
        description: None,
        bit_size: 1,
        variants: &[
            EnumVariant {
                name: "STOP_MODE",
                description: Some("Enter Stop mode when the CPU enters deepsleep"),
                value: 0,
            },
            EnumVariant {
                name: "STANDBY_MODE",
                description: Some("Enter Standby mode when the CPU enters deepsleep"),
                value: 1,
            },
        ],
    }],
};
