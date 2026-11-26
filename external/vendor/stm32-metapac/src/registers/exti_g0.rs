
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Exti",
        extends: None,
        description: Some("External interrupt/event controller"),
        items: &[
            BlockItem {
                name: "rtsr",
                description: Some("Rising Trigger selection register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 40 })),
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "ftsr",
                description: Some("Falling Trigger selection register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 40 })),
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "swier",
                description: Some("Software interrupt event register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 40 })),
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "rpr",
                description: Some("Rising pending register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 40 })),
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "fpr",
                description: Some("Falling pending register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 40 })),
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "exticr",
                description: Some("Configuration register"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x60,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Exticr"),
                }),
            },
            BlockItem {
                name: "imr",
                description: Some("Interrupt mask register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                byte_offset: 0x80,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "emr",
                description: Some("Event mask register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                byte_offset: 0x84,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some("external interrupt configuration register 1"),
            bit_size: 32,
            fields: &[Field {
                name: "exti",
                description: Some("EXTI configuration bits"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: Some(Array::Regular(RegularArray { len: 4, stride: 8 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Lines",
            extends: None,
            description: Some("EXTI lines register, 1 bit per line"),
            bit_size: 32,
            fields: &[Field {
                name: "line",
                description: Some("EXTI line"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 32, stride: 1 })),
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
