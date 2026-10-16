
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Uid",
        extends: None,
        description: Some("Device Factory programmed 96-bit unique device identifier"),
        items: &[BlockItem {
            name: "uid",
            description: Some("Factory programmed 96-bit unique device identifier word 0"),
            array: Some(Array::Regular(RegularArray { len: 3, stride: 4 })),
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::Read,
                bit_size: 32,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
