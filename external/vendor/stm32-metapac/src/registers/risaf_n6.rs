
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Risaf",
            extends: None,
            description: Some(
                "Resource isolation slave unit for address space protection.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RISAF configuration register.",
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
                    name: "iasr",
                    description: Some(
                        "RISAF illegal access status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Iasr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iacr",
                    description: Some(
                        "RISAF illegal access clear register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iaesr",
                    description: Some(
                        "RISAF illegal access error status register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Iaesr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iaddr",
                    description: Some(
                        "RISAF illegal address register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Iaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_cfgr",
                    description: Some(
                        "RISAF region configuration register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegCfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_startr",
                    description: Some(
                        "RISAF region start address register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegStartr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_endr",
                    description: Some(
                        "RISAF region end address register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegEndr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_cidcfgr",
                    description: Some(
                        "RISAF region CID configuration register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegCidcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "RISAF configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "glock",
                    description: Some(
                        "Global lock. When set, writes to RISAF registers are ignored except for IACR and region configuration registers.",
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
            name: "Iacr",
            extends: None,
            description: Some(
                "RISAF illegal access clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caef",
                    description: Some(
                        "Clear configuration access error flag.",
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
                    name: "iaef",
                    description: Some(
                        "Clear illegal access error flag.",
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
            name: "Iaddr",
            extends: None,
            description: Some(
                "RISAF illegal address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iadd",
                    description: Some(
                        "Illegal access address.",
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
            name: "Iaesr",
            extends: None,
            description: Some(
                "RISAF illegal access error status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacid",
                    description: Some(
                        "Illegal access compartment ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iapriv",
                    description: Some(
                        "Illegal access was privileged.",
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
                    name: "iasec",
                    description: Some(
                        "Illegal access was secure.",
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
                    name: "ianrw",
                    description: Some(
                        "Illegal access was a write (0 = read/fetch, 1 = write).",
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
            name: "Iasr",
            extends: None,
            description: Some(
                "RISAF illegal access status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caef",
                    description: Some(
                        "Configuration access error flag.",
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
                    name: "iaef",
                    description: Some(
                        "Illegal access error flag.",
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
            name: "RegCfgr",
            extends: None,
            description: Some(
                "RISAF region configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bren",
                    description: Some(
                        "Base region enable.",
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
                    name: "sec",
                    description: Some(
                        "Secure region. When set, only secure requests can access this region.",
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
                    name: "privc",
                    description: Some(
                        "Privileged access for compartment y. When set, compartment y can only access in privileged mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
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
            ],
        },
        FieldSet {
            name: "RegCidcfgr",
            extends: None,
            description: Some(
                "RISAF region CID configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdenc",
                    description: Some(
                        "Read enable for compartment y.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "wrenc",
                    description: Some(
                        "Write enable for compartment y.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
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
            ],
        },
        FieldSet {
            name: "RegEndr",
            extends: None,
            description: Some(
                "RISAF region end address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baddend",
                    description: Some(
                        "Base region end address.",
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
            name: "RegStartr",
            extends: None,
            description: Some(
                "RISAF region start address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baddstart",
                    description: Some(
                        "Base region start address.",
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
