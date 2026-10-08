
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ramcfg",
            extends: None,
            description: Some(
                "RAMs configuration controller.",
            ),
            items: &[
                BlockItem {
                    name: "m1cr",
                    description: Some(
                        "RAMCFG memory 1 control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M1cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1isr",
                    description: Some(
                        "RAMCFG memory interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M1isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1erkeyr",
                    description: Some(
                        "RAMCFG memory 1 erase key register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M1erkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2cr",
                    description: Some(
                        "RAMCFG memory 2 control register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2ier",
                    description: Some(
                        "RAMCFG memory 2 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2isr",
                    description: Some(
                        "RAMCFG memory interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2sear",
                    description: Some(
                        "RAMCFG memory 2 ECC single error address register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2sear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2dear",
                    description: Some(
                        "RAMCFG memory 2 ECC double error address register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2dear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2icr",
                    description: Some(
                        "RAMCFG memory 2 interrupt clear register 2.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2wpr1",
                    description: Some(
                        "RAMCFG memory 2 write protection register 1.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2wpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2ecckeyr",
                    description: Some(
                        "RAMCFG memory 2 ECC key register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2ecckeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2erkeyr",
                    description: Some(
                        "RAMCFG memory 2 erase key register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2erkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3ier",
                    description: Some(
                        "RAMCFG memory 3 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3isr",
                    description: Some(
                        "RAMCFG memory interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3sear",
                    description: Some(
                        "RAMCFG memory 3 ECC single error address register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3sear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3dear",
                    description: Some(
                        "RAMCFG memory 3 ECC double error address register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3dear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3icr",
                    description: Some(
                        "RAMCFG memory 3 interrupt clear register 3.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3ecckeyr",
                    description: Some(
                        "RAMCFG memory 3 ECC key register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3ecckeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m3erkeyr",
                    description: Some(
                        "RAMCFG memory 3 erase key register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M3erkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m4erkeyr",
                    description: Some(
                        "RAMCFG memory 4 erase key register.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M4erkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5cr",
                    description: Some(
                        "RAMCFG memory 5 control register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5ier",
                    description: Some(
                        "RAMCFG memory 5 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5isr",
                    description: Some(
                        "RAMCFG memory interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5sear",
                    description: Some(
                        "RAMCFG memory 5 ECC single error address register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5sear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5dear",
                    description: Some(
                        "RAMCFG memory 5 ECC double error address register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5dear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5icr",
                    description: Some(
                        "RAMCFG memory 5 interrupt clear register 5.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5ecckeyr",
                    description: Some(
                        "RAMCFG memory 5 ECC key register.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5ecckeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m5erkeyr",
                    description: Some(
                        "RAMCFG memory 5 erase key register.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M5erkeyr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "M1cr",
            extends: None,
            description: Some(
                "RAMCFG memory 1 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecce",
                    description: Some(
                        "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ale",
                    description: Some(
                        "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sramer",
                    description: Some(
                        "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M1erkeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 1 erase key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erasekey",
                    description: Some(
                        "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY[7:0]. 2) Write 0x53 into ERASEKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M1isr",
            extends: None,
            description: Some(
                "RAMCFG memory interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sedc",
                    description: Some(
                        "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ded",
                    description: Some(
                        "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srambusy",
                    description: Some(
                        "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2cr",
            extends: None,
            description: Some(
                "RAMCFG memory 2 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecce",
                    description: Some(
                        "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ale",
                    description: Some(
                        "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sramer",
                    description: Some(
                        "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2dear",
            extends: None,
            description: Some(
                "RAMCFG memory 2 ECC double error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edea",
                    description: Some(
                        "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2ecckeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 2 ECC key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecckey",
                    description: Some(
                        "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY[7:0]. 2) Write 0x75 into ECCKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2erkeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 2 erase key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erasekey",
                    description: Some(
                        "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY[7:0]. 2) Write 0x53 into ERASEKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2icr",
            extends: None,
            description: Some(
                "RAMCFG memory 2 interrupt clear register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csedc",
                    description: Some(
                        "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cded",
                    description: Some(
                        "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2ier",
            extends: None,
            description: Some(
                "RAMCFG memory 2 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seie",
                    description: Some(
                        "ECC single error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "deie",
                    description: Some(
                        "ECC double error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccnmi",
                    description: Some(
                        "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2isr",
            extends: None,
            description: Some(
                "RAMCFG memory interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sedc",
                    description: Some(
                        "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ded",
                    description: Some(
                        "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srambusy",
                    description: Some(
                        "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2sear",
            extends: None,
            description: Some(
                "RAMCFG memory 2 ECC single error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "esea",
                    description: Some(
                        "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M2wpr1",
            extends: None,
            description: Some(
                "RAMCFG memory 2 write protection register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwp",
                    description: Some(
                        "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3dear",
            extends: None,
            description: Some(
                "RAMCFG memory 3 ECC double error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edea",
                    description: Some(
                        "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3ecckeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 3 ECC key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecckey",
                    description: Some(
                        "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY[7:0]. 2) Write 0x75 into ECCKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3erkeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 3 erase key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erasekey",
                    description: Some(
                        "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY[7:0]. 2) Write 0x53 into ERASEKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3icr",
            extends: None,
            description: Some(
                "RAMCFG memory 3 interrupt clear register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csedc",
                    description: Some(
                        "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cded",
                    description: Some(
                        "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3ier",
            extends: None,
            description: Some(
                "RAMCFG memory 3 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seie",
                    description: Some(
                        "ECC single error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "deie",
                    description: Some(
                        "ECC double error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccnmi",
                    description: Some(
                        "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3isr",
            extends: None,
            description: Some(
                "RAMCFG memory interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sedc",
                    description: Some(
                        "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ded",
                    description: Some(
                        "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srambusy",
                    description: Some(
                        "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M3sear",
            extends: None,
            description: Some(
                "RAMCFG memory 3 ECC single error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "esea",
                    description: Some(
                        "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M4erkeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 4 erase key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erasekey",
                    description: Some(
                        "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY[7:0]. 2) Write 0x53 into ERASEKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5cr",
            extends: None,
            description: Some(
                "RAMCFG memory 5 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecce",
                    description: Some(
                        "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ale",
                    description: Some(
                        "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sramer",
                    description: Some(
                        "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5dear",
            extends: None,
            description: Some(
                "RAMCFG memory 5 ECC double error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edea",
                    description: Some(
                        "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5ecckeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 5 ECC key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecckey",
                    description: Some(
                        "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY[7:0]. 2) Write 0x75 into ECCKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5erkeyr",
            extends: None,
            description: Some(
                "RAMCFG memory 5 erase key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erasekey",
                    description: Some(
                        "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY[7:0]. 2) Write 0x53 into ERASEKEY[7:0]. Note: Writing a wrong key reactivates the write protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5icr",
            extends: None,
            description: Some(
                "RAMCFG memory 5 interrupt clear register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csedc",
                    description: Some(
                        "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cded",
                    description: Some(
                        "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5ier",
            extends: None,
            description: Some(
                "RAMCFG memory 5 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seie",
                    description: Some(
                        "ECC single error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "deie",
                    description: Some(
                        "ECC double error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccnmi",
                    description: Some(
                        "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5isr",
            extends: None,
            description: Some(
                "RAMCFG memory interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sedc",
                    description: Some(
                        "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ded",
                    description: Some(
                        "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srambusy",
                    description: Some(
                        "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "M5sear",
            extends: None,
            description: Some(
                "RAMCFG memory 5 ECC single error address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "esea",
                    description: Some(
                        "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
