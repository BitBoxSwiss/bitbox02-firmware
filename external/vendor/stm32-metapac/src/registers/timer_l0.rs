
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tim1ch",
            extends: Some(
                "TIM_CORE",
            ),
            description: Some(
                "Virtual 1-channel timers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr11ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dier1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "Egr1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutput1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccer1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "Option register 1\nNote: Check Reference Manual to parse this register content",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tim2ch",
            extends: Some(
                "TIM_1CH",
            ),
            description: Some(
                "2-channel timers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr11ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr22ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcr",
                    description: Some(
                        "slave mode control register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SmcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dier2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "Egr2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutput2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccer2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1-2)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "Option register 1\nNote: Check Reference Manual to parse this register content",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimBasic",
            extends: Some(
                "TIM_BASIC_NO_CR2",
            ),
            description: Some(
                "Basic timers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr1Core",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Basic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierBasicNoCr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "EgrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimBasicNoCr2",
            extends: Some(
                "TIM_CORE",
            ),
            description: Some(
                "Virtual Basic timers without CR2 register for common part of TIM_BASIC and TIM_1CH_CMP",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr1Core",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierBasicNoCr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "EgrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimCore",
            extends: None,
            description: Some(
                "Virtual timer for common part of TIM_BASIC and TIM_1CH",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr1Core",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "EgrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimGp16",
            extends: Some(
                "TIM_2CH",
            ),
            description: Some(
                "General purpose 16-bit timers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr1Gp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Gp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcr",
                    description: Some(
                        "slave mode control register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SmcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "EgrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1-2 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1-2 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutputGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcerGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1-4)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmar",
                    description: Some(
                        "DMA address for full transfer",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmarGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "Option register 1\nNote: Check Reference Manual to parse this register content",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ArrCore",
            extends: None,
            description: Some(
                "auto-reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arr",
                    description: Some(
                        "Auto-reload value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccer1ch",
            extends: None,
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare x (x=1) output enable",
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
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare x (x=1) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare x (x=1) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccer2ch",
            extends: Some(
                "CCER_1CH",
            ),
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare x (x=1-2) output enable",
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
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare x (x=1-2) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare x (x=1-2) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CcerGp16",
            extends: None,
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare x (x=1-4) output enable",
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
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare x (x=1-4) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare x (x=1-4) output Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CcmrInput1ch",
            extends: None,
            description: Some(
                "capture/compare mode register x (x=1) (input mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare y selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrInputCcs",
                    ),
                },
                Field {
                    name: "icpsc",
                    description: Some(
                        "Input capture y prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "icf",
                    description: Some(
                        "Input capture y filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "FilterValue",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcmrInput2ch",
            extends: Some(
                "CCMR_Input_1CH",
            ),
            description: Some(
                "capture/compare mode register x (x=1) (input mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare y selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrInputCcs",
                    ),
                },
                Field {
                    name: "icpsc",
                    description: Some(
                        "Input capture y prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "icf",
                    description: Some(
                        "Input capture y filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "FilterValue",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcmrOutput1ch",
            extends: None,
            description: Some(
                "capture/compare mode register x (x=1) (output mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare y selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrOutputCcs",
                    ),
                },
                Field {
                    name: "ocfe",
                    description: Some(
                        "Output compare y fast enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpe",
                    description: Some(
                        "Output compare y preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocm",
                    description: Some(
                        "Output compare y mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ocm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcmrOutput2ch",
            extends: Some(
                "CCMR_Output_1CH",
            ),
            description: Some(
                "capture/compare mode register x (x=1) (output mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare y selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrOutputCcs",
                    ),
                },
                Field {
                    name: "ocfe",
                    description: Some(
                        "Output compare y fast enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpe",
                    description: Some(
                        "Output compare y preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocm",
                    description: Some(
                        "Output compare y mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ocm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcmrOutputGp16",
            extends: Some(
                "CCMR_Output_2CH",
            ),
            description: Some(
                "capture/compare mode register x (x=1-2) (output mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare y selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrOutputCcs",
                    ),
                },
                Field {
                    name: "ocfe",
                    description: Some(
                        "Output compare y fast enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpe",
                    description: Some(
                        "Output compare y preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocm",
                    description: Some(
                        "Output compare y mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ocm",
                    ),
                },
                Field {
                    name: "occe",
                    description: Some(
                        "Output compare y clear enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccr1ch",
            extends: None,
            description: Some(
                "capture/compare register x (x=1-4,6)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccr",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CntCore",
            extends: None,
            description: Some(
                "counter",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "counter value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uifcpy",
                    description: Some(
                        "UIF copy",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr11ch",
            extends: Some(
                "CR1_CORE",
            ),
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable",
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
                    name: "udis",
                    description: Some(
                        "Update disable",
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
                    name: "urs",
                    description: Some(
                        "Update request source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode enbaled",
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
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckd",
                    description: Some(
                        "Clock division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckd",
                    ),
                },
                Field {
                    name: "uifremap",
                    description: Some(
                        "UIF status bit remapping enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1Core",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable",
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
                    name: "udis",
                    description: Some(
                        "Update disable",
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
                    name: "urs",
                    description: Some(
                        "Update request source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode enbaled",
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
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uifremap",
                    description: Some(
                        "UIF status bit remapping enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1Gp16",
            extends: Some(
                "CR1_CORE",
            ),
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable",
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
                    name: "udis",
                    description: Some(
                        "Update disable",
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
                    name: "urs",
                    description: Some(
                        "Update request source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode enbaled",
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
                Field {
                    name: "dir",
                    description: Some(
                        "Direction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "cms",
                    description: Some(
                        "Center-aligned mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cms",
                    ),
                },
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckd",
                    description: Some(
                        "Clock division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckd",
                    ),
                },
                Field {
                    name: "uifremap",
                    description: Some(
                        "UIF status bit remapping enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr22ch",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
                Field {
                    name: "ti1s",
                    description: Some(
                        "TI1 selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ti1s",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2Basic",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2Gp16",
            extends: Some(
                "CR2_BASIC",
            ),
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccds",
                    description: Some(
                        "Capture/compare DMA selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccds",
                    ),
                },
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
                Field {
                    name: "ti1s",
                    description: Some(
                        "TI1 selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ti1s",
                    ),
                },
            ],
        },
        FieldSet {
            name: "DcrGp16",
            extends: None,
            description: Some(
                "DMA control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dba",
                    description: Some(
                        "DMA base address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbl",
                    description: Some(
                        "DMA burst length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dier1ch",
            extends: Some(
                "DIER_CORE",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
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
                    name: "ccie",
                    description: Some(
                        "Capture/Compare x (x=1) interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dier2ch",
            extends: Some(
                "DIER_1CH",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
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
                    name: "ccie",
                    description: Some(
                        "Capture/Compare x (x=1-2) interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tie",
                    description: Some(
                        "Trigger interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DierBasicNoCr2",
            extends: Some(
                "DIER_CORE",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
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
                    name: "ude",
                    description: Some(
                        "Update DMA request enable",
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
            name: "DierCore",
            extends: None,
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
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
            ],
        },
        FieldSet {
            name: "DierGp16",
            extends: Some(
                "DIER_BASIC_NO_CR2",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
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
                    name: "ccie",
                    description: Some(
                        "Capture/Compare x (x=1-4) interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tie",
                    description: Some(
                        "Trigger interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ude",
                    description: Some(
                        "Update DMA request enable",
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
                Field {
                    name: "ccde",
                    description: Some(
                        "Capture/Compare x (x=1-4) DMA request enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tde",
                    description: Some(
                        "Trigger DMA request enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmarGp16",
            extends: None,
            description: Some(
                "DMA address for full transfer",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmab",
                    description: Some(
                        "DMA register for burst accesses",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Egr1ch",
            extends: Some(
                "EGR_CORE",
            ),
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
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
                    name: "ccg",
                    description: Some(
                        "Capture/compare x (x=1) generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Egr2ch",
            extends: Some(
                "EGR_1CH",
            ),
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
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
                    name: "ccg",
                    description: Some(
                        "Capture/compare x (x=1-2) generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tg",
                    description: Some(
                        "Trigger generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EgrCore",
            extends: None,
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
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
            ],
        },
        FieldSet {
            name: "EgrGp16",
            extends: Some(
                "EGR_CORE",
            ),
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
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
                    name: "ccg",
                    description: Some(
                        "Capture/compare x (x=1-4) generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tg",
                    description: Some(
                        "Trigger generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SmcrGp16",
            extends: None,
            description: Some(
                "slave mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sms",
                    description: Some(
                        "Slave mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sms",
                    ),
                },
                Field {
                    name: "ts",
                    description: Some(
                        "Trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ts",
                    ),
                },
                Field {
                    name: "msm",
                    description: Some(
                        "Master/Slave mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msm",
                    ),
                },
                Field {
                    name: "etf",
                    description: Some(
                        "External trigger filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "FilterValue",
                    ),
                },
                Field {
                    name: "etps",
                    description: Some(
                        "External trigger prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Etps",
                    ),
                },
                Field {
                    name: "ece",
                    description: Some(
                        "External clock mode 2 enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etp",
                    description: Some(
                        "External trigger polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Etp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sr1ch",
            extends: Some(
                "SR_CORE",
            ),
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
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
                    name: "ccif",
                    description: Some(
                        "Capture/compare x (x=1) interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare x (x=1) overcapture flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr2ch",
            extends: Some(
                "SR_1CH",
            ),
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
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
                    name: "ccif",
                    description: Some(
                        "Capture/compare x (x=1-2) interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some(
                        "Trigger interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare x (x=1-2) overcapture flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SrCore",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
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
            ],
        },
        FieldSet {
            name: "SrGp16",
            extends: Some(
                "SR_CORE",
            ),
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
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
                    name: "ccif",
                    description: Some(
                        "Capture/compare x (x=1-4) interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some(
                        "Trigger interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare x (x=1-4) overcapture flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ccds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_COMPARE",
                    description: Some(
                        "CCx DMA request sent when CCx event occurs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_UPDATE",
                    description: Some(
                        "CCx DMA request sent when update event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CcmrInputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TI4",
                    description: Some(
                        "CCx channel is configured as input, normal mapping: ICx mapped to TIx",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TI3",
                    description: Some(
                        "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TRC",
                    description: Some(
                        "CCx channel is configured as input, ICx is mapped on TRC",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcmrOutputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "CCx channel is configured as output",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ckd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "t_DTS = t_CK_INT",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "t_DTS = 2 × t_CK_INT",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "t_DTS = 4 × t_CK_INT",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Cms",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EDGE_ALIGNED",
                    description: Some(
                        "The counter counts up or down depending on the direction bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CENTER_ALIGNED1",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CENTER_ALIGNED2",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CENTER_ALIGNED3",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UP",
                    description: Some(
                        "Counter used as upcounter",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Counter used as downcounter",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Etp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_INVERTED",
                    description: Some(
                        "ETR is noninverted, active at high level or rising edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some(
                        "ETR is inverted, active at low level or falling edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Etps",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Prescaler OFF",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "ETRP frequency divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "ETRP frequency divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "ETRP frequency divided by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "FilterValue",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NO_FILTER",
                    description: Some(
                        "No filter, sampling is done at fDTS",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FCK_INT_N2",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FCK_INT_N4",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FCK_INT_N8",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N6",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=6",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N8",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N6",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N8",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N6",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=6",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N8",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N5",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=5",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N6",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=6",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N8",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=8",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N5",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=5",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N6",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=6",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N8",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=8",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Mms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "The UG bit from the TIMx_EGR register is used as trigger output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "The counter enable signal, CNT_EN, is used as trigger output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "The update event is selected as trigger output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "COMPARE_PULSE",
                    description: Some(
                        "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "COMPARE_OC1",
                    description: Some(
                        "OC1REF signal is used as trigger output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "COMPARE_OC2",
                    description: Some(
                        "OC2REF signal is used as trigger output",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "COMPARE_OC3",
                    description: Some(
                        "OC3REF signal is used as trigger output",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "COMPARE_OC4",
                    description: Some(
                        "OC4REF signal is used as trigger output",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Msm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_SYNC",
                    description: Some(
                        "No action",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNC",
                    description: Some(
                        "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ocm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "FROZEN",
                    description: Some(
                        "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_ON_MATCH",
                    description: Some(
                        "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "INACTIVE_ON_MATCH",
                    description: Some(
                        "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TOGGLE",
                    description: Some(
                        "OCyREF toggles when TIMx_CNT=TIMx_CCRy",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FORCE_INACTIVE",
                    description: Some(
                        "OCyREF is forced low",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FORCE_ACTIVE",
                    description: Some(
                        "OCyREF is forced high",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PWM_MODE1",
                    description: Some(
                        "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PWM_MODE2",
                    description: Some(
                        "Inversely to PwmMode1",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Sms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENCODER_MODE_1",
                    description: Some(
                        "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ENCODER_MODE_2",
                    description: Some(
                        "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ENCODER_MODE_3",
                    description: Some(
                        "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RESET_MODE",
                    description: Some(
                        "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "GATED_MODE",
                    description: Some(
                        "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TRIGGER_MODE",
                    description: Some(
                        "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXT_CLOCK_MODE",
                    description: Some(
                        "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ti1s",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "The TIMx_CH1 pin is connected to TI1 input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "XOR",
                    description: Some(
                        "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ts",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ITR0",
                    description: Some(
                        "Internal Trigger 0 (ITR0)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ITR1",
                    description: Some(
                        "Internal Trigger 1 (ITR1)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ITR2",
                    description: Some(
                        "Internal Trigger 2 (ITR2)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ITR3",
                    description: Some(
                        "Internal Trigger 3 (ITR3)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TI1F_ED",
                    description: Some(
                        "TI1 Edge Detector (TI1F_ED)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TI1FP1",
                    description: Some(
                        "Filtered Timer Input 1 (TI1FP1)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TI2FP2",
                    description: Some(
                        "Filtered Timer Input 2 (TI2FP2)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "ETRF",
                    description: Some(
                        "External Trigger input (ETRF)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Urs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANY_EVENT",
                    description: Some(
                        "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "COUNTER_ONLY",
                    description: Some(
                        "Only counter overflow/underflow generates an update interrupt or DMA request",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
