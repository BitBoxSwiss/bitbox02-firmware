
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cpu",
            extends: None,
            description: Some("CPU-specific registers"),
            items: &[
                BlockItem {
                    name: "imr",
                    description: Some("CPU x interrupt mask register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
                BlockItem {
                    name: "emr",
                    description: Some("CPU x event mask register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
            ],
        },
        Block {
            name: "Exti",
            extends: None,
            description: Some("External interrupt/event controller"),
            items: &[
                BlockItem {
                    name: "rtsr",
                    description: Some("rising trigger selection register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
                BlockItem {
                    name: "ftsr",
                    description: Some("falling trigger selection register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
                BlockItem {
                    name: "swier",
                    description: Some("software interrupt event register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
                BlockItem {
                    name: "pr",
                    description: Some("EXTI pending register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Lines"),
                    }),
                },
                BlockItem {
                    name: "cpu",
                    description: Some("CPU specific registers"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 64 })),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Cpu" }),
                },
            ],
        },
    ],
    fieldsets: &[FieldSet {
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
    }],
    enums: &[],
};
