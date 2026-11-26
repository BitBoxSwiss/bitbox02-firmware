
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Syscfg",
        extends: None,
        description: Some("System configuration controller"),
        items: &[
            BlockItem {
                name: "memrmp",
                description: Some("memory remap register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Memrmp"),
                }),
            },
            BlockItem {
                name: "pmc",
                description: Some("peripheral mode configuration register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pmc"),
                }),
            },
            BlockItem {
                name: "exticr",
                description: Some("external interrupt configuration register 1"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Exticr"),
                }),
            },
            BlockItem {
                name: "cmpcr",
                description: Some("Compensation cell control register"),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Cmpcr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cmpcr",
            extends: None,
            description: Some("Compensation cell control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_pd",
                    description: Some("Compensation cell power-down"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ready",
                    description: Some("Compensation cell ready flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some("external interrupt configuration register 1"),
            bit_size: 32,
            fields: &[Field {
                name: "exti",
                description: Some("EXTI x configuration (x = 0 to 3)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Memrmp",
            extends: None,
            description: Some("memory remap register"),
            bit_size: 32,
            fields: &[Field {
                name: "mem_mode",
                description: Some("Memory mapping selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 2,
                array: None,
                enumm: Some("MemMode"),
            }],
        },
        FieldSet {
            name: "Pmc",
            extends: None,
            description: Some("peripheral mode configuration register"),
            bit_size: 32,
            fields: &[Field {
                name: "mii_rmii_sel",
                description: Some("Ethernet PHY interface selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[Enum {
        name: "MemMode",
        description: None,
        bit_size: 2,
        variants: &[
            EnumVariant {
                name: "MAIN_FLASH",
                description: Some("Main Flash memory mapped at 0x0000_0000"),
                value: 0,
            },
            EnumVariant {
                name: "SYSTEM_FLASH",
                description: Some("System Flash memory mapped at 0x0000_0000"),
                value: 1,
            },
            EnumVariant {
                name: "FSMC",
                description: Some("FSMC Bank1 (NOR/PSRAM 1 and 2) mapped at 0x0000_0000"),
                value: 2,
            },
            EnumVariant {
                name: "SRAM",
                description: Some("Embedded SRAM mapped at 0x0000_0000"),
                value: 3,
            },
        ],
    }],
};
