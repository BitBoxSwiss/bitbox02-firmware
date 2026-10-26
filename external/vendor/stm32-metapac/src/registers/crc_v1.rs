
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some("Cyclic Redundancy Check calculation unit"),
        items: &[
            BlockItem {
                name: "dr",
                description: Some("Data register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "idr",
                description: Some("Independent Data register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Cr",
        extends: None,
        description: Some("Control register"),
        bit_size: 32,
        fields: &[Field {
            name: "reset",
            description: Some("RESET bit"),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 1,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
