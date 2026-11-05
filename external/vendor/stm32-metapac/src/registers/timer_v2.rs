
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tim1ch",
            extends: Some(
                "TIM_CORE",
            ),
            description: Some(
                "1-channel timers",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1) (Dither mode disabled)",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tisel1ch",
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
                    byte_offset: 0x68,
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
            name: "Tim1chCmp",
            extends: Some(
                "TIM_1CH",
            ),
            description: Some(
                "1-channel with one complementary output timers",
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
                                "Cr21chCmp",
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
                                "Dier1chCmp",
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
                                "Sr1chCmp",
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
                                "Egr1chCmp",
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
                                "Ccer1chCmp",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcr",
                    description: Some(
                        "repetition counter register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Rcr1chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1) (Dither mode disabled)",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdtr",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdtr1chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtr2",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtr21chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tisel1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af1",
                    description: Some(
                        "alternate function register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af11chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af2",
                    description: Some(
                        "alternate function register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af21chCmp",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1chCmp",
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
                    byte_offset: 0x3e0,
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
                                "Smcr2ch",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1-2) (Dither mode disabled)",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1-2) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tisel2ch",
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
                    byte_offset: 0x68,
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
            name: "Tim2chCmp",
            extends: Some(
                "TIM_1CH_CMP",
            ),
            description: Some(
                "2-channel with one complementary output timers",
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
                                "Cr22chCmp",
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
                                "Smcr2chCmp",
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
                                "Dier2chCmp",
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
                                "Sr2chCmp",
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
                                "Egr2chCmp",
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
                                "Ccer2chCmp",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcr",
                    description: Some(
                        "repetition counter register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Rcr1chCmp",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1-2) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdtr",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdtr1chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtr2",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtr21chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tisel2ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af1",
                    description: Some(
                        "alternate function register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af11chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af2",
                    description: Some(
                        "alternate function register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af21chCmp",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1chCmp",
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
                    byte_offset: 0x3e0,
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
            name: "TimAdv",
            extends: Some(
                "TIM_2CH_CMP",
            ),
            description: Some(
                "Advanced Control timers",
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
                                "Cr2Adv",
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
                                "SmcrAdv",
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
                                "DierAdv",
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
                                "SrAdv",
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
                                "EgrAdv",
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
                                "CcerAdv",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcr",
                    description: Some(
                        "repetition counter register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "RcrAdv",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1-2) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdtr",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BdtrAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr5",
                    description: Some(
                        "capture/compare register 5 (Dither mode disabled)",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr5Adv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr5_dither",
                    description: Some(
                        "capture/compare register 5 (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr5DitherAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr6",
                    description: Some(
                        "capture/compare register 6 (Dither mode disabled)",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                    name: "ccr6_dither",
                    description: Some(
                        "capture/compare register 6 (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr3",
                    description: Some(
                        "capture/compare mode register 3",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccmr3Adv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtr2",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtr21chCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "encoder control register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TiselGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af1",
                    description: Some(
                        "alternate function register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af1Adv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af2",
                    description: Some(
                        "alternate function register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af2Adv",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1chCmp",
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
                    byte_offset: 0x3e0,
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
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
                        "auto-reload register (Dither mode disabled)",
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
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherCore",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1-4) (Dither mode disabled)",
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
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1-4) (Dither mode enabled)",
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
                                "CcrDither1ch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "encoder control register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TiselGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af1",
                    description: Some(
                        "alternate function register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af1Gp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af2",
                    description: Some(
                        "alternate function register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af21chCmp",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1chCmp",
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
                    byte_offset: 0x3e0,
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
            name: "TimGp32",
            extends: Some(
                "TIM_GP16",
            ),
            description: Some(
                "General purpose 32-bit timers",
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
                        "counter (Dither mode disabled)",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_dither",
                    description: Some(
                        "counter (Dither mode enbled)",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntDitherGp32",
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
                        "auto-reload register (Dither mode disabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "arr_dither",
                    description: Some(
                        "auto-reload register (Dither mode enabled)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ArrDitherGp32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register x (x=1-4) (Dither mode disabled)",
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
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ccr_dither",
                    description: Some(
                        "capture/compare register x (x=1-4) (Dither mode enabled)",
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
                                "CcrDitherGp32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "encoder control register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EcrGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tisel",
                    description: Some(
                        "input selection register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TiselGp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af1",
                    description: Some(
                        "alternate function register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af1Gp16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "af2",
                    description: Some(
                        "alternate function register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Af21chCmp",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1chCmp",
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
                    byte_offset: 0x3e0,
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
            name: "Af11chCmp",
            extends: None,
            description: Some(
                "alternate function register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkine",
                    description: Some(
                        "TIMx_BKIN input enable",
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
                    name: "bkcmpe",
                    description: Some(
                        "TIM_BRK_CMPx (x=1-8) enable",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bkinp",
                    description: Some(
                        "TIMx_BKIN input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bkinp",
                    ),
                },
                Field {
                    name: "bkcmpp",
                    description: Some(
                        "TIM_BRK_CMPx (x=1-4) input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
                    enumm: Some(
                        "Bkinp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Af1Adv",
            extends: Some(
                "AF1_1CH_CMP",
            ),
            description: Some(
                "alternate function register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkine",
                    description: Some(
                        "TIMx_BKIN input enable",
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
                    name: "bkcmpe",
                    description: Some(
                        "TIM_BRK_CMPx (x=1-8) enable",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bkinp",
                    description: Some(
                        "TIMx_BKIN input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bkinp",
                    ),
                },
                Field {
                    name: "bkcmpp",
                    description: Some(
                        "TIM_BRK_CMPx (x=1-4) input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
                    enumm: Some(
                        "Bkinp",
                    ),
                },
                Field {
                    name: "etrsel",
                    description: Some(
                        "etr_in source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Af1Gp16",
            extends: None,
            description: Some(
                "alternate function register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etrsel",
                    description: Some(
                        "etr_in source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Af21chCmp",
            extends: None,
            description: Some(
                "alternate function register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ocrsel",
                    description: Some(
                        "ocref_clr source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Af2Adv",
            extends: Some(
                "AF2_1CH_CMP",
            ),
            description: Some(
                "alternate function register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bk2ine",
                    description: Some(
                        "TIMx_BKIN2 input enable",
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
                    name: "bk2cmpe",
                    description: Some(
                        "TIM_BRK2_CMPx (x=1-8) enable",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bk2inp",
                    description: Some(
                        "TIMx_BK2IN input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bkinp",
                    ),
                },
                Field {
                    name: "bk2cmpp",
                    description: Some(
                        "TIM_BRK2_CMPx (x=1-4) input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
                    enumm: Some(
                        "Bkinp",
                    ),
                },
                Field {
                    name: "ocrsel",
                    description: Some(
                        "ocref_clr source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ArrCore",
            extends: None,
            description: Some(
                "auto-reload register (Dither mode disabled)",
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
            name: "ArrDitherCore",
            extends: None,
            description: Some(
                "auto-reload register (Dither mode enabled)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "Dither value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arr",
                    description: Some(
                        "Auto-reload value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ArrDitherGp32",
            extends: None,
            description: Some(
                "auto-reload register (Dither mode enabled)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "Dither value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arr",
                    description: Some(
                        "Auto-reload value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdtr1chCmp",
            extends: None,
            description: Some(
                "break and dead-time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtg",
                    description: Some(
                        "Dead-time generator setup",
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
                Field {
                    name: "lock",
                    description: Some(
                        "Lock configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lock",
                    ),
                },
                Field {
                    name: "ossi",
                    description: Some(
                        "Off-state selection for Idle mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossi",
                    ),
                },
                Field {
                    name: "ossr",
                    description: Some(
                        "Off-state selection for Run mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossr",
                    ),
                },
                Field {
                    name: "bke",
                    description: Some(
                        "Break x (x=1) enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 12,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bkp",
                    description: Some(
                        "Break x (x=1) polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 12,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Bkp",
                    ),
                },
                Field {
                    name: "aoe",
                    description: Some(
                        "Automatic output enable",
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
                    name: "moe",
                    description: Some(
                        "Main output enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkf",
                    description: Some(
                        "Break x (x=1) filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "FilterValue",
                    ),
                },
                Field {
                    name: "bkdsrm",
                    description: Some(
                        "Break x (x=1) Disarm",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
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
                    enumm: Some(
                        "Bkdsrm",
                    ),
                },
                Field {
                    name: "bkbid",
                    description: Some(
                        "Break x (x=1) bidirectional",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                    enumm: Some(
                        "Bkbid",
                    ),
                },
            ],
        },
        FieldSet {
            name: "BdtrAdv",
            extends: Some(
                "BDTR_1CH_CMP",
            ),
            description: Some(
                "break and dead-time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtg",
                    description: Some(
                        "Dead-time generator setup",
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
                Field {
                    name: "lock",
                    description: Some(
                        "Lock configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lock",
                    ),
                },
                Field {
                    name: "ossi",
                    description: Some(
                        "Off-state selection for Idle mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossi",
                    ),
                },
                Field {
                    name: "ossr",
                    description: Some(
                        "Off-state selection for Run mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossr",
                    ),
                },
                Field {
                    name: "bke",
                    description: Some(
                        "Break x (x=1,2) enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bkp",
                    description: Some(
                        "Break x (x=1,2) polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Bkp",
                    ),
                },
                Field {
                    name: "aoe",
                    description: Some(
                        "Automatic output enable",
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
                    name: "moe",
                    description: Some(
                        "Main output enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkf",
                    description: Some(
                        "Break x (x=1,2) filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "FilterValue",
                    ),
                },
                Field {
                    name: "bkdsrm",
                    description: Some(
                        "Break x (x=1,2) Disarm",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
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
                    enumm: Some(
                        "Bkdsrm",
                    ),
                },
                Field {
                    name: "bkbid",
                    description: Some(
                        "Break x (x=1,2) bidirectional",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                    enumm: Some(
                        "Bkbid",
                    ),
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
            name: "Ccer1chCmp",
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
                    name: "ccne",
                    description: Some(
                        "Capture/Compare x (x=1) complementary output enable",
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
            name: "Ccer2chCmp",
            extends: Some(
                "CCER_2CH",
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
                    name: "ccne",
                    description: Some(
                        "Capture/Compare x (x=1) complementary output enable",
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
            name: "CcerAdv",
            extends: Some(
                "CCER_2CH_CMP",
            ),
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare x (x=1-6) output enable",
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
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare x (x=1-6) output Polarity",
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
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccne",
                    description: Some(
                        "Capture/Compare x (x=1-4) complementary output enable",
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
            name: "Ccmr3Adv",
            extends: None,
            description: Some(
                "capture/compare mode register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ocfe",
                    description: Some(
                        "Output compare x (x=5,6) fast enable",
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
                        "Output compare x (x=5,6) preload enable",
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
                        "Output compare x (x=5,6) mode",
                    ),
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                16..=16,
                            ],
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
                        "Ocm",
                    ),
                },
                Field {
                    name: "occe",
                    description: Some(
                        "Output compare x (x=5,6) clear enable",
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                16..=16,
                            ],
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                16..=16,
                            ],
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                16..=16,
                            ],
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
                "capture/compare register x (x=1-4,6) (Dither mode disabled)",
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
            name: "Ccr5Adv",
            extends: Some(
                "CCR_1CH",
            ),
            description: Some(
                "capture/compare register 5 (Dither mode disabled)",
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
                Field {
                    name: "gc5c",
                    description: Some(
                        "Group channel 5 and channel x (x=1-3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Gc5c",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccr5DitherAdv",
            extends: Some(
                "CCR_DITHER_1CH",
            ),
            description: Some(
                "capture/compare register 5 (Dither mode enabled)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccr",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gc5c",
                    description: Some(
                        "Group channel 5 and channel x (x=1-3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Gc5c",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcrDither1ch",
            extends: None,
            description: Some(
                "capture/compare register x (x=1-4,6) (Dither mode enabled)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccr",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CcrDitherGp32",
            extends: None,
            description: Some(
                "capture/compare register x (x=1-4,6) (Dither mode enabled)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "Dither value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccr",
                    description: Some(
                        "capture/compare x (x=1-4,6) value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
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
            name: "CntDitherGp32",
            extends: None,
            description: Some(
                "counter (Dither mode enabled)",
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
                    bit_size: 31,
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
                Field {
                    name: "dithen",
                    description: Some(
                        "Dithering enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
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
                Field {
                    name: "dithen",
                    description: Some(
                        "Dithering enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
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
                Field {
                    name: "dithen",
                    description: Some(
                        "Dithering enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr21chCmp",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccpc",
                    description: Some(
                        "Capture/compare preloaded control",
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
                    name: "ccus",
                    description: Some(
                        "Capture/compare control update selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
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
                    name: "ois",
                    description: Some(
                        "Output Idle state x (x=1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "oisn",
                    description: Some(
                        "Output Idle state x (x=1)",
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
                                stride: 2,
                            },
                        ),
                    ),
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                25..=25,
                            ],
                        },
                    ),
                    bit_size: 4,
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
            name: "Cr22chCmp",
            extends: Some(
                "CR2_1CH_CMP",
            ),
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccpc",
                    description: Some(
                        "Capture/compare preloaded control",
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
                    name: "ccus",
                    description: Some(
                        "Capture/compare control update selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                25..=25,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                Field {
                    name: "ois",
                    description: Some(
                        "Output Idle state x (x=1,2)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "oisn",
                    description: Some(
                        "Output Idle state x (x=1)",
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
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2Adv",
            extends: Some(
                "CR2_2CH_CMP",
            ),
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccpc",
                    description: Some(
                        "Capture/compare preloaded control",
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
                    name: "ccus",
                    description: Some(
                        "Capture/compare control update selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                25..=25,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                Field {
                    name: "ois",
                    description: Some(
                        "Output Idle state x (x=1-6)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "oisn",
                    description: Some(
                        "Output Idle state x N x (x=1-4)",
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
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mms2",
                    description: Some(
                        "Master mode selection 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mms2",
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                25..=25,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                25..=25,
                            ],
                        },
                    ),
                    bit_size: 4,
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
            name: "Dcr1chCmp",
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
                Field {
                    name: "dbss",
                    description: Some(
                        "DMA burst source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Dbss",
                    ),
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
            name: "Dier1chCmp",
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
                Field {
                    name: "comie",
                    description: Some(
                        "COM interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bie",
                    description: Some(
                        "Break interrupt enable",
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
                        "Capture/Compare x (x=1) DMA request enable",
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
            name: "Dier2chCmp",
            extends: Some(
                "DIER_1CH_CMP",
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
                Field {
                    name: "comie",
                    description: Some(
                        "COM interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                    name: "bie",
                    description: Some(
                        "Break interrupt enable",
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
                        "Capture/Compare x (x=1) DMA request enable",
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
                Field {
                    name: "comde",
                    description: Some(
                        "COM DMA request enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
            name: "DierAdv",
            extends: Some(
                "DIER_2CH_CMP",
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
                    name: "comie",
                    description: Some(
                        "COM interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                    name: "bie",
                    description: Some(
                        "Break interrupt enable",
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
                        "Capture/Compare x (x=1) DMA request enable",
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
                    name: "comde",
                    description: Some(
                        "COM DMA request enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                Field {
                    name: "idxie",
                    description: Some(
                        "Index interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dirie",
                    description: Some(
                        "Direction change interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ierrie",
                    description: Some(
                        "Index error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrie",
                    description: Some(
                        "Transition error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
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
                Field {
                    name: "idxie",
                    description: Some(
                        "Index interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dirie",
                    description: Some(
                        "Direction change interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ierrie",
                    description: Some(
                        "Index error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrie",
                    description: Some(
                        "Transition error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtr21chCmp",
            extends: None,
            description: Some(
                "deadtime register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtgf",
                    description: Some(
                        "Dead-time falling edge generator setup",
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
                Field {
                    name: "dtae",
                    description: Some(
                        "Deadtime asymmetric enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dtae",
                    ),
                },
                Field {
                    name: "dtpe",
                    description: Some(
                        "Deadtime preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EcrGp16",
            extends: None,
            description: Some(
                "encoder control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ie",
                    description: Some(
                        "Index enable",
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
                    name: "idir",
                    description: Some(
                        "Index direction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Idir",
                    ),
                },
                Field {
                    name: "iblk",
                    description: Some(
                        "Index blanking",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Iblk",
                    ),
                },
                Field {
                    name: "fidx",
                    description: Some(
                        "First index",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fidx",
                    ),
                },
                Field {
                    name: "ipos",
                    description: Some(
                        "Index positioning",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pw",
                    description: Some(
                        "Pulse width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwprsc",
                    description: Some(
                        "Pulse width prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
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
            name: "Egr1chCmp",
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
                Field {
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bg",
                    description: Some(
                        "Break x (x=1) generation",
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
            name: "Egr2chCmp",
            extends: Some(
                "EGR_1CH_CMP",
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
                        "Capture/compare x (x=1,2) generation",
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
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                Field {
                    name: "bg",
                    description: Some(
                        "Break x (x=1) generation",
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
            name: "EgrAdv",
            extends: Some(
                "EGR_2CH_CMP",
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
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                Field {
                    name: "bg",
                    description: Some(
                        "Break x (x=1-2) generation",
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
                                stride: 1,
                            },
                        ),
                    ),
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
            name: "Rcr1chCmp",
            extends: None,
            description: Some(
                "repetition counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rep",
                    description: Some(
                        "Repetition counter value",
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
            name: "RcrAdv",
            extends: None,
            description: Some(
                "repetition counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rep",
                    description: Some(
                        "Repetition counter value",
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
            name: "Smcr2ch",
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                0..=2,
                                16..=16,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                20..=21,
                            ],
                        },
                    ),
                    bit_size: 5,
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
            ],
        },
        FieldSet {
            name: "Smcr2chCmp",
            extends: Some(
                "SMCR_2CH",
            ),
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                0..=2,
                                16..=16,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                20..=21,
                            ],
                        },
                    ),
                    bit_size: 5,
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
                    name: "smspe",
                    description: Some(
                        "SMS preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SmcrAdv",
            extends: Some(
                "SMCR_2CH_CMP",
            ),
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                0..=2,
                                16..=16,
                            ],
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Sms",
                    ),
                },
                Field {
                    name: "occs",
                    description: Some(
                        "OCREF clear selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Occs",
                    ),
                },
                Field {
                    name: "ts",
                    description: Some(
                        "Trigger selection",
                    ),
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                20..=21,
                            ],
                        },
                    ),
                    bit_size: 5,
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
                Field {
                    name: "smspe",
                    description: Some(
                        "SMS preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smsps",
                    description: Some(
                        "SMS preload source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smsps",
                    ),
                },
            ],
        },
        FieldSet {
            name: "SmcrGp16",
            extends: Some(
                "SMCR_2CH",
            ),
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                0..=2,
                                16..=16,
                            ],
                        },
                    ),
                    bit_size: 4,
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
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                4..=6,
                                20..=21,
                            ],
                        },
                    ),
                    bit_size: 5,
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
                Field {
                    name: "smspe",
                    description: Some(
                        "SMS preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smsps",
                    description: Some(
                        "SMS preload source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Smsps",
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
            name: "Sr1chCmp",
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
                    name: "comif",
                    description: Some(
                        "COM interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bif",
                    description: Some(
                        "Break x (x=1) interrupt flag",
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
            name: "Sr2chCmp",
            extends: Some(
                "SR_1CH_CMP",
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
                        "Capture/compare x (x=1,2) interrupt flag",
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
                    name: "comif",
                    description: Some(
                        "COM interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                    name: "bif",
                    description: Some(
                        "Break x (x=1) interrupt flag",
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
                        "Capture/Compare x (x=1,2) overcapture flag",
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
            name: "SrAdv",
            extends: Some(
                "SR_2CH_CMP",
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
                    name: "comif",
                    description: Some(
                        "COM interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                    name: "bif",
                    description: Some(
                        "Break x (x=1,2) interrupt flag",
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
                                stride: 1,
                            },
                        ),
                    ),
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
                Field {
                    name: "sbif",
                    description: Some(
                        "System break interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccif5",
                    description: Some(
                        "Capture/compare 5 interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccif6",
                    description: Some(
                        "Capture/compare 6 interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idxif",
                    description: Some(
                        "Index interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dirif",
                    description: Some(
                        "Direction change interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ierrif",
                    description: Some(
                        "Index error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrif",
                    description: Some(
                        "Transition error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                Field {
                    name: "idxif",
                    description: Some(
                        "Index interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dirif",
                    description: Some(
                        "Direction change interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ierrif",
                    description: Some(
                        "Index error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrif",
                    description: Some(
                        "Transition error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tisel1ch",
            extends: None,
            description: Some(
                "input selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tisel",
                    description: Some(
                        "Selects TIM_TIx (x=1) input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
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
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tisel2ch",
            extends: Some(
                "TISEL_1CH",
            ),
            description: Some(
                "input selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tisel",
                    description: Some(
                        "Selects TIM_TIx (x=1-2) input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
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
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TiselGp16",
            extends: None,
            description: Some(
                "input selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tisel",
                    description: Some(
                        "Selects TIM_TIx (x=1-4) input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
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
            name: "Bkbid",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some(
                        "Break input tim_brk in input mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIDIRECTIONAL",
                    description: Some(
                        "Break input tim_brk in bidirectional mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bkdsrm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ARMED",
                    description: Some(
                        "Break input tim_brk is armed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISARMED",
                    description: Some(
                        "Break input tim_brk is disarmed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bkinp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_INVERTED",
                    description: Some(
                        "input polarity is not inverted (active low if BKxP = 0, active high if BKxP = 1)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some(
                        "input polarity is inverted (active high if BKxP = 0, active low if BKxP = 1)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bkp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "Break input tim_brk is active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "Break input tim_brk is active high",
                    ),
                    value: 1,
                },
            ],
        },
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
            name: "Dbss",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "Update",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CC1",
                    description: Some(
                        "CC1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CC2",
                    description: Some(
                        "CC2",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CC3",
                    description: Some(
                        "CC3",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CC4",
                    description: Some(
                        "CC4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "COM",
                    description: Some(
                        "COM",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "TRIGGER",
                    description: Some(
                        "Trigger",
                    ),
                    value: 7,
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
            name: "Dtae",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IDENTICAL",
                    description: Some(
                        "Deadtime on rising and falling edges are identical, and defined with DTG[7:0] register",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISTINCT",
                    description: Some(
                        "Deadtime on rising edge is defined with DTG[7:0] register and deadtime on falling edge is defined with DTGF[7:0] bits.",
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
            name: "Fidx",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALWAYS_ACTIVE",
                    description: Some(
                        "Index is always active",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FIRST_ONLY",
                    description: Some(
                        "the first Index only resets the counter",
                    ),
                    value: 1,
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
            name: "Gc5c",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_EFFECT",
                    description: Some(
                        "No effect of TIM_OC5REF on TIM_OCxREFC (x=1-3)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOGICAL_AND",
                    description: Some(
                        "TIM_OCxREFC is the logical AND of TIM_OCxREF and TIM_OC5REF",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Iblk",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ALWAYS_ACTIVE",
                    description: Some(
                        "Index always active",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CC3P",
                    description: Some(
                        "Index disabled when tim_ti3 input is active, as per CC3P bitfield",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CC4P",
                    description: Some(
                        "Index disabled when tim_ti4 input is active, as per CC4P bitfield",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Idir",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BOTH",
                    description: Some(
                        "Index resets the counter whatever the direction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "UP",
                    description: Some(
                        "Index resets the counter when up-counting only",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Index resets the counter when down-counting only",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lock",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No bit is write protected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mms",
            description: None,
            bit_size: 4,
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
                EnumVariant {
                    name: "ENCODER_CLOCK_OUTPUT",
                    description: Some(
                        "The encoder clock signal is used as trigger output",
                    ),
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Mms2",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "The UG bit from the TIMx_EGR register is used as TRGO2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "The counter enable signal, CNT_EN, is used as TRGO2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "The update event is selected as TRGO2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "COMPARE_PULSE",
                    description: Some(
                        "TRGO2 send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "COMPARE_OC1",
                    description: Some(
                        "OC1REF signal is used as TRGO2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "COMPARE_OC2",
                    description: Some(
                        "OC2REF signal is used as TRGO2",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "COMPARE_OC3",
                    description: Some(
                        "OC3REF signal is used as TRGO2",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "COMPARE_OC4",
                    description: Some(
                        "OC4REF signal is used as TRGO2",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "COMPARE_OC5",
                    description: Some(
                        "OC5REF signal is used as TRGO2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "COMPARE_OC6",
                    description: Some(
                        "OC6REF signal is used as TRGO2",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC4",
                    description: Some(
                        "OC4REF rising or falling edges generate pulses on TRGO2",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC6",
                    description: Some(
                        "OC6REF rising or falling edges generate pulses on TRGO2",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC4_OR_OC6_RISING",
                    description: Some(
                        "OC4REF or OC6REF rising edges generate pulses on TRGO2",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC4_RISING_OR_OC6_FALLING",
                    description: Some(
                        "OC4REF rising or OC6REF falling edges generate pulses on TRGO2",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC5_OR_OC6_RISING",
                    description: Some(
                        "OC5REF or OC6REF rising edges generate pulses on TRGO2",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "COMPARE_PULSE_OC5_RISING_OR_OC6_FALLING",
                    description: Some(
                        "OC5REF rising or OC6REF falling edges generate pulses on TRGO2",
                    ),
                    value: 15,
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
            name: "Occs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some(
                        "tim_ocref_clr_int is connected to the tim_ocref_clr input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ETRF",
                    description: Some(
                        "tim_ocref_clr_int is connected to tim_etrf",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ocm",
            description: None,
            bit_size: 4,
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
                EnumVariant {
                    name: "RETRIGERRABLE_OPM_MODE_1",
                    description: Some(
                        "In up-counting mode, the channel is active until a trigger\nevent is detected (on tim_trgi signal). Then, a comparison is performed as in PWM\nmode 1 and the channels becomes active again at the next update. In down-counting\nmode, the channel is inactive until a trigger event is detected (on tim_trgi signal).\nThen, a comparison is performed as in PWM mode 1 and the channels becomes\ninactive again at the next update.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RETRIGERRABLE_OPM_MODE_2",
                    description: Some(
                        "In up-counting mode, the channel is inactive until a\ntrigger event is detected (on tim_trgi signal). Then, a comparison is performed as in\nPWM mode 2 and the channels becomes inactive again at the next update. In down-\ncounting mode, the channel is active until a trigger event is detected (on tim_trgi\nsignal). Then, a comparison is performed as in PWM mode 1 and the channels\nbecomes active again at the next update.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "_RESERVED1",
                    description: Some(
                        "_reserved1",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "_RESERVED2",
                    description: Some(
                        "_reserved2",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "COMBINED_PWM_MODE_1",
                    description: Some(
                        "tim_oc1ref has the same behavior as in PWM mode 1.\ntim_oc1refc is the logical OR between tim_oc1ref and tim_oc2ref.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "COMBINED_PWM_MODE_2",
                    description: Some(
                        "tim_oc1ref has the same behavior as in PWM mode 2.\ntim_oc1refc is the logical AND between tim_oc1ref and tim_oc2ref.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "ASYMMETRIC_PWM_MODE_1",
                    description: Some(
                        "tim_oc1ref has the same behavior as in PWM mode 1.\ntim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is\ncounting down.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "ASYMMETRIC_PWM_MODE_2",
                    description: Some(
                        "tim_oc1ref has the same behavior as in PWM mode 2.\ntim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is\ncounting down.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Ossi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "When inactive, OC/OCN outputs are disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLE_LEVEL",
                    description: Some(
                        "When inactive, OC/OCN outputs are forced to idle level",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ossr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "When inactive, OC/OCN outputs are disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLE_LEVEL",
                    description: Some(
                        "When inactive, OC/OCN outputs are enabled with their inactive level",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sms",
            description: None,
            bit_size: 4,
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
                EnumVariant {
                    name: "COMBINED_RESET_TRIGGER",
                    description: Some(
                        "Rising edge of the selected trigger input (tim_trgi) reinitializes the counter, generates an update of the registers and starts the counter.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "COMBINED_GATED_TRIGGER",
                    description: Some(
                        "The counter clock is enabled when the trigger input (tim_trgi) is high. The counter stops and is reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "ENCODER_UP_X2",
                    description: Some(
                        "Encoder mode, Clock plus direction, x2 mode.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "ENCODER_UP_X1",
                    description: Some(
                        "Encoder mode, Clock plus direction, x1 mode, tim_ti2fp2 edge sensitivity is set by CC2P",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "ENCODER_DIR_X2",
                    description: Some(
                        "Encoder mode, Directional Clock, x2 mode.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "ENCODER_DIR_X1",
                    description: Some(
                        "Encoder mode, Directional Clock, x1 mode, tim_ti1fp1 and tim_ti2fp2 edge sensitivity is set by CC1P and CC2P.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "QUADRATURE_ENCODER_MODE_X1_TI1PF1",
                    description: Some(
                        "Quadrature encoder mode, x1 mode, counting on tim_ti1fp1 edges only, edge sensitivity is set by CC1P.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "QUADRATURE_ENCODER_MODE_X1_TI2PF2",
                    description: Some(
                        "Quadrature encoder mode, x1 mode, counting on tim_ti2fp2 edges only, edge sensitivity is set by CC2P.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Smsps",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "The transfer is triggered by the Timer’s Update event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INDEX",
                    description: Some(
                        "The transfer is triggered by the Index event",
                    ),
                    value: 1,
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
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "ITR0",
                    description: Some(
                        "Internal Trigger 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ITR1",
                    description: Some(
                        "Internal Trigger 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ITR2",
                    description: Some(
                        "Internal Trigger 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ITR3",
                    description: Some(
                        "Internal Trigger 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TI1F_ED",
                    description: Some(
                        "TI1 Edge Detector",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TI1FP1",
                    description: Some(
                        "Filtered Timer Input 1",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TI2FP2",
                    description: Some(
                        "Filtered Timer Input 2",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "ETRF",
                    description: Some(
                        "External Trigger input",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "ITR4",
                    description: Some(
                        "Internal Trigger 4",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "ITR5",
                    description: Some(
                        "Internal Trigger 5",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "ITR6",
                    description: Some(
                        "Internal Trigger 6",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "ITR7",
                    description: Some(
                        "Internal Trigger 7",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "ITR8",
                    description: Some(
                        "Internal Trigger 8",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "ITR9",
                    description: Some(
                        "Internal Trigger 9",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "ITR10",
                    description: Some(
                        "Internal Trigger 10",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "ITR11",
                    description: Some(
                        "Internal Trigger 11",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "ITR12",
                    description: Some(
                        "Internal Trigger 12",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "ITR13",
                    description: Some(
                        "Internal Trigger 13",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "ITR14",
                    description: Some(
                        "Internal Trigger 14",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "ITR15",
                    description: Some(
                        "Internal Trigger 15",
                    ),
                    value: 19,
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
