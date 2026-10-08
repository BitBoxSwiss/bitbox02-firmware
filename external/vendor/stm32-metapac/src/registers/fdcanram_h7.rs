
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Fdcanram",
        extends: None,
        description: Some("FDCAN Message RAM"),
        items: &[BlockItem {
            name: "ram",
            description: Some("FDCAN Message RAM"),
            array: Some(Array::Regular(RegularArray { len: 2560, stride: 4 })),
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
