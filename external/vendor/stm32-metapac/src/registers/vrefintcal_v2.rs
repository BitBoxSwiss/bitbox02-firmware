
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
                bit_size: 32,
                fieldset: Some("VrefintcalData"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "VrefintcalData",
        extends: None,
        description: Some("Factory calibration"),
        bit_size: 32,
        fields: &[Field {
            name: "vrefint_cal",
            description: Some("VREFINT calibration value"),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
            bit_size: 12,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
