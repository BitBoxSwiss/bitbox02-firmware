
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Vrefbuf",
        extends: None,
        description: Some("Voltage reference buffer."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("VREFBUF Control and Status Register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "ccr",
                description: Some("VREFBUF Calibration Control Register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ccr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some("VREFBUF Calibration Control Register."),
            bit_size: 32,
            fields: &[Field {
                name: "trim",
                description: Some("Trimming code."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 6,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("VREFBUF Control and Status Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "envr",
                    description: Some("Enable Voltage Reference."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hiz",
                    description: Some("High impedence mode for the VREFBUF."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Hiz"),
                },
                Field {
                    name: "vrr",
                    description: Some("Voltage reference buffer ready."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrs",
                    description: Some("Voltage reference scale."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Vrs"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Hiz",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONNECTED",
                    description: Some("VREF+ pin is internally connected to the voltage reference buffer output."),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH_Z",
                    description: Some("VREF+ pin is high impedance."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vrs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VREF0",
                    description: Some("Voltage reference set to around 2.048 V."),
                    value: 0,
                },
                EnumVariant {
                    name: "VREF1",
                    description: Some("Voltage reference set to around 2.5 V."),
                    value: 1,
                },
                EnumVariant {
                    name: "VREF2",
                    description: Some("Voltage reference set to around 2.9 V."),
                    value: 2,
                },
            ],
        },
    ],
};
