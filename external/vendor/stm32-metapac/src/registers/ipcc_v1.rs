
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ipcc",
            extends: None,
            description: Some("IPCC"),
            items: &[BlockItem {
                name: "cpu",
                description: Some("CPU specific registers"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                byte_offset: 0x0,
                inner: BlockItemInner::Block(BlockItemBlock { block: "IpccCpu" }),
            }],
        },
        Block {
            name: "IpccCpu",
            extends: None,
            description: Some("IPCC"),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some("Control register CPUx"),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("CxCr"),
                    }),
                },
                BlockItem {
                    name: "mr",
                    description: Some("Mask register CPUx"),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("CxMr"),
                    }),
                },
                BlockItem {
                    name: "scr",
                    description: Some("Status Set or Clear register CPUx"),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Write,
                        bit_size: 32,
                        fieldset: Some("CxScr"),
                    }),
                },
                BlockItem {
                    name: "sr",
                    description: Some("CPUx to CPUy status register"),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("CxToySr"),
                    }),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CxCr",
            extends: None,
            description: Some("Control register CPUx"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxoie",
                    description: Some("processor x Receive channel occupied interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfie",
                    description: Some("processor x Transmit channel free interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CxMr",
            extends: None,
            description: Some("Mask register CPUx"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chom",
                    description: Some("processor x Receive channel y occupied interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chfm",
                    description: Some("processor x Transmit channel y free interrupt mask"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CxScr",
            extends: None,
            description: Some("Status Set or Clear register CPUx"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chc",
                    description: Some("processor x Receive channel y status clear"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chs",
                    description: Some("processor x Transmit channel y status set"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CxToySr",
            extends: None,
            description: Some("CPUx to CPUy status register"),
            bit_size: 32,
            fields: &[Field {
                name: "chf",
                description: Some("processor x transmit to process y Receive channel z status flag"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
