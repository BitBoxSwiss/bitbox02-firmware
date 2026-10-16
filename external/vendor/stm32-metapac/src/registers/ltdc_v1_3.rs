
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Layer",
            extends: None,
            description: Some(
                "Cluster Layer%S",
            ),
            items: &[
                BlockItem {
                    name: "c0r",
                    description: Some(
                        "Layerx Configuration 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c1r",
                    description: Some(
                        "Layerx Configuration 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcr",
                    description: Some(
                        "Layerx Reload Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Layerx Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "whpcr",
                    description: Some(
                        "Layerx Window Horizontal Position Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Whpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wvpcr",
                    description: Some(
                        "Layerx Window Vertical Position Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wvpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckcr",
                    description: Some(
                        "Layerx Color Keying Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pfcr",
                    description: Some(
                        "Layerx Pixel Format Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cacr",
                    description: Some(
                        "Layerx Constant Alpha Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dccr",
                    description: Some(
                        "Layerx Default Color Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bfcr",
                    description: Some(
                        "Layerx Blending Factors Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "blcr",
                    description: Some(
                        "Layerx Burst Length Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Blcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcr",
                    description: Some(
                        "Layerx Planar Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfbar",
                    description: Some(
                        "Layerx Color Frame Buffer Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfblr",
                    description: Some(
                        "Layerx Color Frame Buffer Length Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfblr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfblnr",
                    description: Some(
                        "Layerx Color Frame Buffer Line Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfblnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afba0r",
                    description: Some(
                        "Layer1 Auxiliary Frame Buffer Address 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afba0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afba1r",
                    description: Some(
                        "Layer1 Auxiliary Frame Buffer Address 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afba1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afblr",
                    description: Some(
                        "Layer1 Auxiliary Frame Buffer Length Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afblr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afblnr",
                    description: Some(
                        "Layer1 Auxiliary Frame Buffer Line Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afblnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clutwr",
                    description: Some(
                        "Layerx Clut Write Register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clutwr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cyr0r",
                    description: Some(
                        "Layerx Conversion Ycbcr Rgb 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cyr0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cyr1r",
                    description: Some(
                        "Layerx Conversion Ycbcr Rgb 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cyr1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpf0r",
                    description: Some(
                        "Layerx Flexible Pixel Format 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpf0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpf1r",
                    description: Some(
                        "Layerx Flexible Pixel Format 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpf1r",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Ltdc",
            extends: None,
            description: Some(
                "LCD-TFT Controller",
            ),
            items: &[
                BlockItem {
                    name: "sscr",
                    description: Some(
                        "Ltdc Synchronization Size Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bpcr",
                    description: Some(
                        "Ltdc Back Porch Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awcr",
                    description: Some(
                        "Ltdc Active Width Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "twcr",
                    description: Some(
                        "Ltdc Total Width Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Twcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "Ltdc Global Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc1r",
                    description: Some(
                        "Ltdc Global Configuration 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gc1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc2r",
                    description: Some(
                        "Ltdc Global Configuration 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gc2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srcr",
                    description: Some(
                        "Ltdc Shadow Reload Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gccr",
                    description: Some(
                        "Ltdc Gamma Correction Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bccr",
                    description: Some(
                        "Ltdc Background Color Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "Ltdc Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Ltdc Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "Ltdc Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lipcr",
                    description: Some(
                        "Line Interrupt Position Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lipcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpsr",
                    description: Some(
                        "Ltdc Current Position Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdsr",
                    description: Some(
                        "Ltdc Current Display Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edcr",
                    description: Some(
                        "Ltdc External Display Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier2",
                    description: Some(
                        "Ltdc Interrupt Enable Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr2",
                    description: Some(
                        "Ltdc Interrupt Status Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr2",
                    description: Some(
                        "Ltdc Interrupt Clear Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lipcr2",
                    description: Some(
                        "Line Interrupt Position Configuration Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lipcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecrcr",
                    description: Some(
                        "Ltdc Expected Crc Register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccrcr",
                    description: Some(
                        "Ltdc Computed Crc Register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "futr",
                    description: Some(
                        "Ltdc Fifo Underrun Threshold Register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Futr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "layer",
                    description: Some(
                        "Cluster Layer%S",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Layer",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Afba0r",
            extends: None,
            description: Some(
                "Layer1 Auxiliary Frame Buffer Address 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afbadd0",
                    description: Some(
                        "Frame Buffer Start Address.",
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
            name: "Afba1r",
            extends: None,
            description: Some(
                "Layer1 Auxiliary Frame Buffer Address 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afbadd1",
                    description: Some(
                        "Auxiliary Frame Buffer Start Address.",
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
            name: "Afblnr",
            extends: None,
            description: Some(
                "Layer1 Auxiliary Frame Buffer Line Number Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afblnbr",
                    description: Some(
                        "Auxiliary Frame Buffer Line Number.",
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
            name: "Afblr",
            extends: None,
            description: Some(
                "Layer1 Auxiliary Frame Buffer Length Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afbll",
                    description: Some(
                        "Auxiliary Frame Buffer Line Length.",
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
                    name: "afbp",
                    description: Some(
                        "Auxiliary Frame Buffer Pitch In Bytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awcr",
            extends: None,
            description: Some(
                "Ltdc Active Width Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aah",
                    description: Some(
                        "Accumulated Active Height (In Units Of Horizontal Scan Line).",
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
                    name: "aaw",
                    description: Some(
                        "Accumulated Active Width (In Units Of Pixel Clock Period).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bccr",
            extends: None,
            description: Some(
                "Ltdc Background Color Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bcblue",
                    description: Some(
                        "Background Color Blue Value.",
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
                    name: "bcgreen",
                    description: Some(
                        "Background Color Green Value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcred",
                    description: Some(
                        "Background Color Red Value.",
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
            ],
        },
        FieldSet {
            name: "Bfcr",
            extends: None,
            description: Some(
                "Layerx Blending Factors Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bf2",
                    description: Some(
                        "Blending Factor 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Bf2",
                    ),
                },
                Field {
                    name: "bf1",
                    description: Some(
                        "Blending Factor 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Bf1",
                    ),
                },
                Field {
                    name: "bor",
                    description: Some(
                        "Blending Order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bor",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Blcr",
            extends: None,
            description: Some(
                "Layerx Burst Length Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bl",
                    description: Some(
                        "Burst Length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Bl",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bpcr",
            extends: None,
            description: Some(
                "Ltdc Back Porch Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avbp",
                    description: Some(
                        "Accumulated Vertical Back Porch (In Units Of Horizontal Scan Line).",
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
                    name: "ahbp",
                    description: Some(
                        "Accumulated Horizontal Back Porch (In Units Of Pixel Clock Period).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C0r",
            extends: None,
            description: Some(
                "Layerx Configuration 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckta",
                    description: Some(
                        "Color Key Transparency Ability.",
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
                    name: "cfbda",
                    description: Some(
                        "Color Frame Buffer Duplication Ability.",
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
                    name: "cfbpa",
                    description: Some(
                        "Color Frame Buffer Pitch Ability.",
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
                    name: "apa",
                    description: Some(
                        "Alpha Plane Ability.",
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
                    name: "dcp",
                    description: Some(
                        "Default Color Programmability.",
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
                    name: "wina",
                    description: Some(
                        "Windowing Ability.",
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
                    name: "cluta",
                    description: Some(
                        "Clut Ability.",
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
                    name: "ckra",
                    description: Some(
                        "Color Key Replace Ability.",
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
                    name: "f21",
                    description: Some(
                        "Blending Factor 2, Ability For: 1.0.",
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
                    name: "f20",
                    description: Some(
                        "Blending Factor 2, Ability For: 0.0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f2p",
                    description: Some(
                        "Blending Factor 2, Ability For: Pixel_Alpha.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f21p",
                    description: Some(
                        "Blending Factor 2, Ability For: 1.0 - Pixel_Alpha.",
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
                    name: "f2c",
                    description: Some(
                        "Blending Factor 2, Ability For: Constant_Alpha.",
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
                Field {
                    name: "f21c",
                    description: Some(
                        "Blending Factor 2, Ability For: 1.0 - Constant_Alpha.",
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
                    name: "f2pc",
                    description: Some(
                        "Blending Factor 2, Ability For: Pixel_Alpha * Constant_Alpha.",
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
                    name: "f21pc",
                    description: Some(
                        "Blending Factor 2, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha).",
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
                    name: "f11",
                    description: Some(
                        "Blending Factor 1, Ability For: 1.0.",
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
                    name: "f10",
                    description: Some(
                        "Blending Factor 1,Ability For: 0.0.",
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
                    name: "f1p",
                    description: Some(
                        "Blending Factor 1, Ability For: Pixel_Alpha.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f11p",
                    description: Some(
                        "Blending Factor 1, Ability For: 1.0 - Pixel_Alpha.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1c",
                    description: Some(
                        "Blending Factor 1, Ability For: Constant_Alpha.",
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
                    name: "f11c",
                    description: Some(
                        "Blending Factor 1, Ability For: 1.0 - Constant_Alpha.",
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
                    name: "f1pc",
                    description: Some(
                        "Blending Factor 1, Ability For: Pixel_Alpha * Constant_Alpha.",
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
                    name: "f11pc",
                    description: Some(
                        "Blending Factor 1, Ability For: 1.0 - (Pixel_Alpha * Constant_Alpha).",
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
                Field {
                    name: "ff",
                    description: Some(
                        "Flexible Pixel Format, Ability.",
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
                    name: "rgb888",
                    description: Some(
                        "Pixel Format, Ability For Rgb888.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bgr565",
                    description: Some(
                        "Pixel Format, Ability For Bgr565.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rgb565",
                    description: Some(
                        "Pixel Format, Ability For Rgb565.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bgra8888",
                    description: Some(
                        "Pixel Format, Ability For Bgra8888.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rgba8888",
                    description: Some(
                        "Pixel Format, Ability For Rgba8888.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abgr8888",
                    description: Some(
                        "Pixel Format, Ability For Abgr8888.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "argb8888",
                    description: Some(
                        "Pixel Format, Ability For Argb8888.",
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
            name: "C1r",
            extends: None,
            description: Some(
                "Layerx Configuration 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "yia",
                    description: Some(
                        "Ycbcr 422 Interleaved Ability For That Layer.",
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
                    name: "yspa",
                    description: Some(
                        "Ycbcr 420 Semi-Planar Ability For That Layer.",
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
                    name: "yfpa",
                    description: Some(
                        "Ycbcr 420 Full-Planar Ability For That Layer.",
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
                    name: "sca",
                    description: Some(
                        "Scaling Ability For That Layer.",
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
            name: "Cacr",
            extends: None,
            description: Some(
                "Layerx Constant Alpha Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "consta",
                    description: Some(
                        "Constant Alpha.",
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
            name: "Ccrcr",
            extends: None,
            description: Some(
                "Ltdc Computed Crc Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccrc",
                    description: Some(
                        "Computed Crc Of Frame.",
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
            name: "Cdsr",
            extends: None,
            description: Some(
                "Ltdc Current Display Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vdes",
                    description: Some(
                        "Vertical Data Enable Display Status.",
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
                    name: "hdes",
                    description: Some(
                        "Horizontal Data Enable Display Status.",
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
                    name: "vsyncs",
                    description: Some(
                        "Vertical Synchronization Display Status.",
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
                    name: "hsyncs",
                    description: Some(
                        "Horizontal Synchronization Display Status.",
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
            name: "Cfbar",
            extends: None,
            description: Some(
                "Layerx Color Frame Buffer Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfbadd",
                    description: Some(
                        "Color Frame Buffer Start Address.",
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
            name: "Cfblnr",
            extends: None,
            description: Some(
                "Layerx Color Frame Buffer Line Number Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfblnbr",
                    description: Some(
                        "Frame Buffer Line Number.",
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
            name: "Cfblr",
            extends: None,
            description: Some(
                "Layerx Color Frame Buffer Length Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfbll",
                    description: Some(
                        "Color Frame Buffer Line Length.",
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
                    name: "cfbp",
                    description: Some(
                        "Color Frame Buffer Pitch In Bytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ckcr",
            extends: None,
            description: Some(
                "Layerx Color Keying Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckblue",
                    description: Some(
                        "Color Key Blue Value.",
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
                    name: "ckgreen",
                    description: Some(
                        "Color Key Green Value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckred",
                    description: Some(
                        "Color Key Red Value.",
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
            ],
        },
        FieldSet {
            name: "Clutwr",
            extends: None,
            description: Some(
                "Layerx Clut Write Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue Value.",
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
                    name: "green",
                    description: Some(
                        "Green Value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "red",
                    description: Some(
                        "Red Value.",
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
                    name: "clutadd",
                    description: Some(
                        "Clut Address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpsr",
            extends: None,
            description: Some(
                "Ltdc Current Position Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cypos",
                    description: Some(
                        "Current Y Position.",
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
                    name: "cxpos",
                    description: Some(
                        "Current X Position.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Layerx Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Layer Enable.",
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
                    name: "cken",
                    description: Some(
                        "Color Keying Enable.",
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
                    name: "cluten",
                    description: Some(
                        "Color Look-Up Table Enable.",
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
                    name: "hmen",
                    description: Some(
                        "Horizontal Mirroring Enable.",
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
                    name: "dcben",
                    description: Some(
                        "Default Color Blending Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cyr0r",
            extends: None,
            description: Some(
                "Layerx Conversion Ycbcr Rgb 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr2r",
                    description: Some(
                        "Cr-To-Red Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cb2b",
                    description: Some(
                        "Cb-To-Blue Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cyr1r",
            extends: None,
            description: Some(
                "Layerx Conversion Ycbcr Rgb 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr2g",
                    description: Some(
                        "Cr-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cb2g",
                    description: Some(
                        "Cb-To-Green Coefficient, With Bits 9:8 As Positive Integer And 7:0 As Decimals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dccr",
            extends: None,
            description: Some(
                "Layerx Default Color Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcblue",
                    description: Some(
                        "Default Color Blue.",
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
                    name: "dcgreen",
                    description: Some(
                        "Default Color Green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcred",
                    description: Some(
                        "Default Color Red.",
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
                    name: "dcalpha",
                    description: Some(
                        "Default Color Alpha.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecrcr",
            extends: None,
            description: Some(
                "Ltdc Expected Crc Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecrc",
                    description: Some(
                        "Expected Crc Of Frame.",
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
            name: "Edcr",
            extends: None,
            description: Some(
                "Ltdc External Display Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ocyen",
                    description: Some(
                        "Output Conversion To Ycbcr 422 Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ocysel",
                    description: Some(
                        "Output Conversion To Ycbcr 422.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ocysel",
                    ),
                },
                Field {
                    name: "ocyco",
                    description: Some(
                        "Output Conversion To Ycbcr 422.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ocyco",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fpf0r",
            extends: None,
            description: Some(
                "Layerx Flexible Pixel Format 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "apos",
                    description: Some(
                        "Location Of The Alpha Component Inside The Pixel Memory Word (In Bits).",
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
                    name: "alen",
                    description: Some(
                        "Width Of The Alpha Component (In Bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpos",
                    description: Some(
                        "Location Of The Red Component Inside The Pixel Memory Word (In Bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rlen",
                    description: Some(
                        "Width Of The Red Component (In Bits).",
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
            name: "Fpf1r",
            extends: None,
            description: Some(
                "Layerx Flexible Pixel Format 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpos",
                    description: Some(
                        "Location Of The Green Component Inside The Pixel Memory Word (In Bits).",
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
                    name: "glen",
                    description: Some(
                        "Width Of The Green Component (In Bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bpos",
                    description: Some(
                        "Location Of The Blue Component Inside The Pixel Memory Word (In Bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blen",
                    description: Some(
                        "Width Of The Blue Component (In Bits).",
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
                Field {
                    name: "psize",
                    description: Some(
                        "Pixel Size (In Bytes).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Futr",
            extends: None,
            description: Some(
                "Ltdc Fifo Underrun Threshold Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thre",
                    description: Some(
                        "Threshold To Trigger A Fifo Underrun Interrupt (Per Fifo Word, 64 Bits).",
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
            name: "Gc1r",
            extends: None,
            description: Some(
                "Ltdc Global Configuration 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wbch",
                    description: Some(
                        "Width Of Blue Channel Output.",
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
                    name: "wgch",
                    description: Some(
                        "Width Of Green Channel Output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrch",
                    description: Some(
                        "Width Of Red Channel Output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prba",
                    description: Some(
                        "Precise Blending Ability.",
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
                Field {
                    name: "dt",
                    description: Some(
                        "Dithering Technique Implemented.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dt",
                    ),
                },
                Field {
                    name: "gct",
                    description: Some(
                        "Gamma Correction Technique Implemented.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Gct",
                    ),
                },
                Field {
                    name: "shra",
                    description: Some(
                        "Shadow Registers Ability.",
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
                    name: "bcp",
                    description: Some(
                        "Background Color Programmability (Unique Color Blended As Background).",
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
                    name: "bba",
                    description: Some(
                        "Background Blending Ability.",
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
                Field {
                    name: "lnip",
                    description: Some(
                        "Line-Irq: Line Position Programmability.",
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
                    name: "tp",
                    description: Some(
                        "Timing Programmability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spp",
                    description: Some(
                        "Sync Polarity Programmability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwp",
                    description: Some(
                        "Dither Width Programmability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stra",
                    description: Some(
                        "Status Register Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crma",
                    description: Some(
                        "Configuration Reading Mode Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bma",
                    description: Some(
                        "Blind Mode Ability.",
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
            name: "Gc2r",
            extends: None,
            description: Some(
                "Ltdc Global Configuration 2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bla",
                    description: Some(
                        "Background Layer Ability (Pixels Of Background Layer Are Read From Memory).",
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
                    name: "stsa",
                    description: Some(
                        "Slave Timings Synchronization Ability.",
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
                    name: "dva",
                    description: Some(
                        "Dual-View Ability.",
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
                    name: "dpa",
                    description: Some(
                        "Secondary Rgb Output Port Ability.",
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
                    name: "bw",
                    description: Some(
                        "Bus Width (Log2 Of Number Of Bytes).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Bw",
                    ),
                },
                Field {
                    name: "edca",
                    description: Some(
                        "External Display Control Ability.",
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
                    name: "oca",
                    description: Some(
                        "Output Conversion Ability (Rgb To Ycbcr).",
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
                    name: "axiida",
                    description: Some(
                        "Axiid Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rota",
                    description: Some(
                        "Rotation Support Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sisa",
                    description: Some(
                        "Second Interrupt Set Ability.",
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
                    name: "sfa",
                    description: Some(
                        "Single Frame Mode Ability.",
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
                Field {
                    name: "crca",
                    description: Some(
                        "Crc Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Crca",
                    ),
                },
                Field {
                    name: "boa",
                    description: Some(
                        "Blending Order Ability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Boa",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Gccr",
            extends: None,
            description: Some(
                "Ltdc Gamma Correction Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Address Of The R,G,B Table Where The Comp Component Is Written.",
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
                    name: "comp",
                    description: Some(
                        "Color Component To Be Written, In Either (Or All) The R,G,B Tables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blueen",
                    description: Some(
                        "Write Trigger To The Blue Table.",
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
                    name: "greenen",
                    description: Some(
                        "Write Trigger To The Green Table.",
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
                    name: "reden",
                    description: Some(
                        "Write Trigger To The Red Table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some(
                "Ltdc Global Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcen",
                    description: Some(
                        "Ltdc Global Enable.",
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
                    name: "gamen",
                    description: Some(
                        "Gamma Correction Enable.",
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
                    name: "dbw",
                    description: Some(
                        "Dither Blue Width.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dgw",
                    description: Some(
                        "Dither Green Width.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drw",
                    description: Some(
                        "Dither Red Width.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "den",
                    description: Some(
                        "Dither Enable.",
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
                    name: "crcen",
                    description: Some(
                        "Crc Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sfen",
                    description: Some(
                        "Single-Frame Mode: Mode Enable.",
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
                    name: "sfswtr",
                    description: Some(
                        "Single-Frame Mode: Software Trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sfswtr",
                    ),
                },
                Field {
                    name: "pcpol",
                    description: Some(
                        "Pixel Clock Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pcpol",
                    ),
                },
                Field {
                    name: "depol",
                    description: Some(
                        "Blanking (No Data/Pixel) Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Depol",
                    ),
                },
                Field {
                    name: "vspol",
                    description: Some(
                        "Vertical Synchronization Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vspol",
                    ),
                },
                Field {
                    name: "hspol",
                    description: Some(
                        "Horizontal Synchronization Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hspol",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some(
                "Ltdc Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clif",
                    description: Some(
                        "Clears The Line Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Clif",
                    ),
                },
                Field {
                    name: "cfuwif",
                    description: Some(
                        "Clears The Fifo Underrun Warning Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cfuwif",
                    ),
                },
                Field {
                    name: "cterrif",
                    description: Some(
                        "Clears The Transfer Error Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cterrif",
                    ),
                },
                Field {
                    name: "crrif",
                    description: Some(
                        "Clears Register Reload Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Crrif",
                    ),
                },
                Field {
                    name: "cfuif",
                    description: Some(
                        "Clears The Fifo Underrun Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cfuif",
                    ),
                },
                Field {
                    name: "ccrcif",
                    description: Some(
                        "Clears The Crc Error Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccrcif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Icr2",
            extends: None,
            description: Some(
                "Ltdc Interrupt Clear Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clif",
                    description: Some(
                        "Clears The Line Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Clif",
                    ),
                },
                Field {
                    name: "cfuwif",
                    description: Some(
                        "Clears The Fifo Underrun Warning Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cfuwif",
                    ),
                },
                Field {
                    name: "cterrif",
                    description: Some(
                        "Clears The Transfer Error Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cterrif",
                    ),
                },
                Field {
                    name: "crrif",
                    description: Some(
                        "Clears Register Reload Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Crrif",
                    ),
                },
                Field {
                    name: "cfuif",
                    description: Some(
                        "Clears The Fifo Underrun Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cfuif",
                    ),
                },
                Field {
                    name: "ccrcif",
                    description: Some(
                        "Clears The Crc Error Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccrcif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "Ltdc Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lie",
                    description: Some(
                        "Line Interrupt Enable.",
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
                    name: "fuwie",
                    description: Some(
                        "Fifo Underrun Warning Interrupt Enable.",
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
                    name: "terrie",
                    description: Some(
                        "Transfer Error Interrupt Enable.",
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
                    name: "rrie",
                    description: Some(
                        "Register Reload Interrupt Enable.",
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
                    name: "fuie",
                    description: Some(
                        "Fifo Underrun Interrupt Enable.",
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
                    name: "crcie",
                    description: Some(
                        "Crc Error Interrupt Enable.",
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
            ],
        },
        FieldSet {
            name: "Ier2",
            extends: None,
            description: Some(
                "Ltdc Interrupt Enable Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lie",
                    description: Some(
                        "Line Interrupt Enable.",
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
                    name: "fuwie",
                    description: Some(
                        "Fifo Underrun Warning Interrupt Enable.",
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
                    name: "terrie",
                    description: Some(
                        "Transfer Error Interrupt Enable.",
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
                    name: "rrie",
                    description: Some(
                        "Register Reload Interrupt Enable.",
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
                    name: "fuie",
                    description: Some(
                        "Fifo Underrun Interrupt Enable.",
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
                    name: "crcie",
                    description: Some(
                        "Crc Error Interrupt Enable.",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "Ltdc Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lif",
                    description: Some(
                        "Line Interrupt Flag.",
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
                    name: "fuwif",
                    description: Some(
                        "Fifo Underrun Warning Interrupt Flag.",
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
                    name: "terrif",
                    description: Some(
                        "Transfer Error Interrupt Flag.",
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
                    name: "rrif",
                    description: Some(
                        "Register Reload Interrupt Flag.",
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
                    name: "fuif",
                    description: Some(
                        "Fifo Underrun Interrupt Flag.",
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
                    name: "crcif",
                    description: Some(
                        "Crc Error Interrupt Flag.",
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
            ],
        },
        FieldSet {
            name: "Isr2",
            extends: None,
            description: Some(
                "Ltdc Interrupt Status Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lif",
                    description: Some(
                        "Line Interrupt Flag.",
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
                    name: "fuwif",
                    description: Some(
                        "Fifo Underrun Warning Interrupt Flag.",
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
                    name: "terrif",
                    description: Some(
                        "Transfer Error Interrupt Flag.",
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
                    name: "rrif",
                    description: Some(
                        "Register Reload Interrupt Flag.",
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
                    name: "fuif",
                    description: Some(
                        "Fifo Underrun Interrupt Flag.",
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
                    name: "crcif",
                    description: Some(
                        "Crc Error Interrupt Flag.",
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
            ],
        },
        FieldSet {
            name: "Lipcr",
            extends: None,
            description: Some(
                "Line Interrupt Position Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lipos",
                    description: Some(
                        "Line Interrupt Position.",
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
            name: "Lipcr2",
            extends: None,
            description: Some(
                "Line Interrupt Position Configuration Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lipos",
                    description: Some(
                        "Line Interrupt Position.",
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
            name: "Pcr",
            extends: None,
            description: Some(
                "Layerx Planar Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ycen",
                    description: Some(
                        "Ycbcr-To-Rgb Conversion Enable.",
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
                    name: "ycm",
                    description: Some(
                        "Ycbcr Conversion Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ycm",
                    ),
                },
                Field {
                    name: "yf",
                    description: Some(
                        "Y Component First.",
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
                    name: "cbf",
                    description: Some(
                        "Cb Component First.",
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
                    name: "of",
                    description: Some(
                        "Odd Pixel First.",
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
                    name: "yren",
                    description: Some(
                        "Y Rescale Enable For The Color Dynamic Range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pfcr",
            extends: None,
            description: Some(
                "Layerx Pixel Format Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pf",
                    description: Some(
                        "Pixel Format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rcr",
            extends: None,
            description: Some(
                "Layerx Reload Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imr",
                    description: Some(
                        "Immediate Reload Trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Imr",
                    ),
                },
                Field {
                    name: "vbr",
                    description: Some(
                        "Vertical Blanking Reload Request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbr",
                    ),
                },
                Field {
                    name: "grmsk",
                    description: Some(
                        "Shadow Reload Control, Global (Centralized) Reload Masked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Grmsk",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Srcr",
            extends: None,
            description: Some(
                "Ltdc Shadow Reload Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imr",
                    description: Some(
                        "Immediate Reload Trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Imr",
                    ),
                },
                Field {
                    name: "vbr",
                    description: Some(
                        "Vertical Blanking Reload Request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sscr",
            extends: None,
            description: Some(
                "Ltdc Synchronization Size Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsh",
                    description: Some(
                        "Vertical Synchronization Height (In Units Of Horizontal Scan Line).",
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
                    name: "hsw",
                    description: Some(
                        "Horizontal Synchronization Width (In Units Of Pixel Clock Period).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Twcr",
            extends: None,
            description: Some(
                "Ltdc Total Width Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "totalh",
                    description: Some(
                        "Total Height (In Units Of Horizontal Scan Line).",
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
                    name: "totalw",
                    description: Some(
                        "Total Width (In Units Of Pixel Clock Period).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Whpcr",
            extends: None,
            description: Some(
                "Layerx Window Horizontal Position Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "whstpos",
                    description: Some(
                        "Window Horizontal Start Position.",
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
                    name: "whsppos",
                    description: Some(
                        "Window Horizontal Stop Position.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wvpcr",
            extends: None,
            description: Some(
                "Layerx Window Vertical Position Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wvstpos",
                    description: Some(
                        "Window Vertical Start Position.",
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
                    name: "wvsppos",
                    description: Some(
                        "Window Vertical Stop Position.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Bf1",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CONSTANT",
                    description: Some(
                        "constant alpha.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PIXEL",
                    description: Some(
                        "pixel alpha x constant alpha.",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Bf2",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CONSTANT",
                    description: Some(
                        "1 - constant alpha.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PIXEL",
                    description: Some(
                        "1 - (pixel alpha x constant alpha).",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Bl",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "MAXIMUM",
                    description: Some(
                        "maximum burst length (16 words 64 bits, thus 128 Bytes).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WORD1",
                    description: Some(
                        "1 word (of 64 bits) per burst.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WORD16",
                    description: Some(
                        "16 words (of 64 bits) per burst.",
                    ),
                    value: 16,
                },
            ],
        },
        Enum {
            name: "Boa",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIXED",
                    description: Some(
                        "Blending Order Fixed.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONFIGURABLE",
                    description: Some(
                        "Blending Order Configurable.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bor",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BACKGROUND",
                    description: Some(
                        "layer set in background.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FOREGROUND",
                    description: Some(
                        "layer set in foreground.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bw",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BUS32",
                    description: Some(
                        "32-Bit Bus.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BUS64",
                    description: Some(
                        "64-Bit Bus.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BUS128",
                    description: Some(
                        "128-Bit Bus.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Ccrcif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Crcif Flag In Isrx.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cfuif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Fuif Flag In Isrx",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cfuwif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Fuwif Flag In Isrx.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Clif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Lif Flag In Isrx.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Crca",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_AVAILABLE",
                    description: Some(
                        "Crc No Computation Available.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AVAILABLE",
                    description: Some(
                        "Crc Computation Available.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Crrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Rrif Flag In Isrx.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cterrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears The Terrif Flag In Isrx.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Depol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "Blanking (No Data/Pixel) Polarity Is Active Low.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "Blanking (No Data/Pixel) Polarity Is Active High.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_DITHERING",
                    description: Some(
                        "No Dithering.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ORDERED",
                    description: Some(
                        "Ordered 4X4 Bayer.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PSEUDO_RANDOM",
                    description: Some(
                        "Pseudo-Random Lfsr.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Gct",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_GAMMA",
                    description: Some(
                        "No Gamma.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GAMMA_SAMPLES",
                    description: Some(
                        "Gamma With 256 Samples.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GAMMA_INTERPOLATED",
                    description: Some(
                        "Gamma With 8 Interpolated Segments.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Grmsk",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "global reload active for this layer (control from LTDC_SRCR enabled).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "global reload masked for this layer (control from LTDC_SRCR disabled).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hspol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "Horizontal Synchronization Polarity Is Active Low.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "Horizontal Synchronization Polarity Is Active High.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Imr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_EFFECT",
                    description: Some(
                        "No Effect",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: Some(
                        "The Shadow Registers Are Reloaded Immediately.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ocyco",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CB_FIRST",
                    description: Some(
                        "Cb Is Output First (Y0Cb, Then Y1Cr, Y2Cb And So On).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CR_FIRST",
                    description: Some(
                        "Cr Is Output First (Y0Cr, Then Y1Cb, Y2Cr And So On).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ocysel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BT601",
                    description: Some(
                        "Use Itu-R Bt.601 Set (For Typically Sdtv Analog-Like Displays).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BT709",
                    description: Some(
                        "Use Itu-R Bt.709 Set (For Typically Hdtv Digital-Like Displays).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "The Pixel And Sync Data Are Generated At The Rising-Edge Of The Output Lcd_Clk Clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "The Pixel And Sync Data Are Generated At The Falling-Edge Of The Output Lcd_Clk Clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pf",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "ARGB8888 (32 bpp).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ABGR8888",
                    description: Some(
                        "ABGR8888 (32 bpp).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGBA8888",
                    description: Some(
                        "RGBA8888 (32 bpp).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BGRA8888",
                    description: Some(
                        "BGRA8888 (32 bpp).",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "RGB565 (16 bpp, A = 255).",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BGR565",
                    description: Some(
                        "BGR565 (16 bpp, A = 255).",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "RGB888 (24 bpp packed, A = 255).",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FLEXIBLE",
                    description: Some(
                        "Flexible pixel format selected.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Sfswtr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_ACTION",
                    description: Some(
                        "No Action.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONE_FRAME",
                    description: Some(
                        "Triggers One Frame.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_EFFECT",
                    description: Some(
                        "No Effect",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: Some(
                        "The Shadow Registers Are Reloaded During The Vertical Blanking Period (At The Beginning Of The First Line After The Active Display Area).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vspol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "Vertical Synchronization Is Active Low.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "Vertical Synchronization Is Active High.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ycm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INTERLEAVED",
                    description: Some(
                        "interleaved 422 (Cb and Cr component are replicated horizontally for pixels P and P+1).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SEMI_PLANAR",
                    description: Some(
                        "semi-Planar 420: (Cb and Cr component are replicated horizontally and vertically.The layer main configuration defines the access to the Y buffer, and auxiliary registers define the access to the Cb and Cr buffers).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FULL_PLANAR",
                    description: Some(
                        "full-Planar 420: (Cb and Cr component are replicated horizontally and vertically. The layer main configuration defines the access to the Y buffer, and auxiliary registers define the access to the Cb and Cr buffers).",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
