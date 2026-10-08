
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Vrefintcal",
        extends: None,
        description: Some("VREFINT Factory Calibration"),
        items: &[BlockItem {
            name: "data",
            description: Some("Factory calibration"),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::Read,
                bit_size: 16,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
