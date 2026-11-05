
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Bkp",
        extends: None,
        description: Some("Backup registers"),
        items: &[
            BlockItem {
                name: "dr",
                description: Some("Data register"),
                array: Some(Array::Cursed(CursedArray {
                    offsets: &[
                        4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 64, 68, 72, 76, 80, 84, 88, 92, 96, 100, 104, 108, 112,
                        116, 120, 124, 128, 132, 136, 140, 144, 148, 152, 156, 160, 164, 168, 172, 176, 180, 184, 188,
                    ],
                })),
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dr"),
                }),
            },
            BlockItem {
                name: "rtccr",
                description: Some("RTC clock calibration register"),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rtccr"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x30,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "csr",
                description: Some("Control/status register"),
                array: None,
                byte_offset: 0x34,
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
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpe",
                    description: Some("Tamper pin enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpal",
                    description: Some("Tamper pin active level"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Tpal"),
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Control/status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cte",
                    description: Some("Clear Tamper event"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cti",
                    description: Some("Clear Tamper Interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpie",
                    description: Some("Tamper Pin interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tef",
                    description: Some("Tamper Event Flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some("Tamper Interrupt Flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some("Data register"),
            bit_size: 32,
            fields: &[Field {
                name: "d",
                description: Some("Backup data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Rtccr",
            extends: None,
            description: Some("RTC clock calibration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal",
                    description: Some("Calibration value"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cco",
                    description: Some("Calibration Clock Output"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asoe",
                    description: Some("Alarm or second output enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asos",
                    description: Some("Alarm or second output selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Asos"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Asos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALARM",
                    description: Some("RTC Alarm pulse output selected"),
                    value: 0,
                },
                EnumVariant {
                    name: "SECOND",
                    description: Some("RTC Second pulse output selected"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tpal",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
