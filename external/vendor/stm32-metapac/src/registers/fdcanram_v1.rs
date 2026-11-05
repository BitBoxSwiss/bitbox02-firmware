
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Fdcanram",
        extends: None,
        description: Some("FDCAN Message RAM"),
        items: &[
            BlockItem {
                name: "flssa",
                description: Some("11-bit filter"),
                array: Some(Array::Regular(RegularArray { len: 28, stride: 4 })),
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "flesa",
                description: Some("29-bit filter"),
                array: Some(Array::Regular(RegularArray { len: 16, stride: 4 })),
                byte_offset: 0x70,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "rxfifo0",
                description: Some("Rx FIFO 0"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 0xb0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "rxfifo1",
                description: Some("Rx FIFO 1"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 0x188,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "txefifo",
                description: Some("Tx event FIFO"),
                array: Some(Array::Regular(RegularArray { len: 6, stride: 4 })),
                byte_offset: 0x260,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "txbuf",
                description: Some("Tx buffer"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 0x278,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
