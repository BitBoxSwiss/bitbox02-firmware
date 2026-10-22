
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gfxmmu",
            extends: None,
            description: Some(
                "GFXMMU.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "GFXMMU configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "sr",
                    description: Some(
                        "GFXMMU status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fcr",
                    description: Some(
                        "GFXMMU flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dvr",
                    description: Some(
                        "GFXMMU default value register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bcr",
                    description: Some(
                        "GFXMMU buffer 0 configuration register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutl",
                    description: Some(
                        "GFXMMU LUT entry 0 low.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1024,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "luth",
                    description: Some(
                        "GFXMMU LUT entry 0 high.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1024,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x1004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Luth",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcr",
            extends: None,
            description: Some(
                "GFXMMU buffer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pbo",
                    description: Some(
                        "Physical buffer offset. Offset of the physical buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pbba",
                    description: Some(
                        "Physical buffer base address. Base address MSB of the physical buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "GFXMMU configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boie",
                    description: Some(
                        "Buffer overflow interrupt enable. This bit enables the buffer 0 overflow interrupt.",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ameie",
                    description: Some(
                        "AHB master error interrupt enable. This bit enables the AHB master error interrupt.",
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
                    name: "bm",
                    description: Some(
                        "192 Block mode. This bit defines the number of blocks per line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 0,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Bm192",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dvr",
            extends: None,
            description: Some(
                "GFXMMU default value register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dv",
                    description: Some(
                        "Default value. This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped.",
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
            name: "Fcr",
            extends: None,
            description: Some(
                "GFXMMU flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbof",
                    description: Some(
                        "Clear buffer overflow flag. Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register.",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "camef",
                    description: Some(
                        "Clear AHB master error flag. Writing 1 clears the AHB master error flag in the GFXMMU_SR register.",
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
            ],
        },
        FieldSet {
            name: "Luth",
            extends: None,
            description: Some(
                "GFXMMU LUT entry high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset. Line offset of line number x (i.e. offset of block 0 of line x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lutl",
            extends: None,
            description: Some(
                "GFXMMU LUT entry low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Line enable.",
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
                    name: "fvb",
                    description: Some(
                        "First Valid Block. Number of the first valid block of line number x.",
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
                    name: "lvb",
                    description: Some(
                        "Last Valid Block. Number of the last valid block of line number X.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "GFXMMU status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bof",
                    description: Some(
                        "Buffer overflow flag. This bit is set when an overflow occurs during the offset calculation of the buffer 0. It is cleared by writing 1 to CB0OF.",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "amef",
                    description: Some(
                        "AHB master error flag. This bit is set when an AHB error happens during a transaction. It is cleared by writing 1 to CAMEF.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Bm192",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "_256BLOCKS_PER_LINE",
                    description: Some(
                        "256 blocks per line.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_192BLOCKS_PER_LINE",
                    description: Some(
                        "192 blocks per line.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
