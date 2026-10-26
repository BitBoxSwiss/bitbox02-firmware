
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cryp",
            extends: None,
            description: Some(
                "Cryptographic processor.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register.",
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
                        "status register.",
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
                    name: "din",
                    description: Some(
                        "data input register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dout",
                    description: Some(
                        "data output register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dmacr",
                    description: Some(
                        "DMA control register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "imscr",
                    description: Some(
                        "interrupt mask set/clear register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Imscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "risr",
                    description: Some(
                        "raw interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Risr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misr",
                    description: Some(
                        "masked interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key",
                    description: Some(
                        "Cluster KEY%s, containing K?LR, K?RR.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Key",
                        },
                    ),
                },
                BlockItem {
                    name: "init",
                    description: Some(
                        "Cluster INIT%s, containing IV?LR, IV?RR.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Init",
                        },
                    ),
                },
                BlockItem {
                    name: "csgcmccmr",
                    description: Some(
                        "context swap register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "csgcmr",
                    description: Some(
                        "context swap register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x70,
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
            name: "Init",
            extends: None,
            description: Some(
                "Cluster INIT%s, containing IV?LR, IV?RR.",
            ),
            items: &[
                BlockItem {
                    name: "ivlr",
                    description: Some(
                        "initialization vector registers.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ivrr",
                    description: Some(
                        "initialization vector registers.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
            name: "Key",
            extends: None,
            description: Some(
                "Cluster KEY%s, containing K?LR, K?RR.",
            ),
            items: &[
                BlockItem {
                    name: "klr",
                    description: Some(
                        "key registers.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "krr",
                    description: Some(
                        "key registers.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
            name: "Cr",
            extends: None,
            description: Some(
                "control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "algodir",
                    description: Some(
                        "Algorithm direction.",
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
                    name: "algomode0",
                    description: Some(
                        "Algorithm mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datatype",
                    description: Some(
                        "Data type selection.",
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
                    name: "keysize",
                    description: Some(
                        "Key size selection (AES mode only).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fflush",
                    description: Some(
                        "FIFO flush.",
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
                    name: "crypen",
                    description: Some(
                        "Cryptographic processor enable.",
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
                    name: "gcm_ccmph",
                    description: Some(
                        "GCM_CCMPH.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "algomode3",
                    description: Some(
                        "ALGOMODE.",
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
                    name: "npblb",
                    description: Some(
                        "Number of Padding Bytes in Last Block of payload.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "kmod",
                    description: Some(
                        "Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD[1:0] different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Kmod",
                    ),
                },
                Field {
                    name: "iprst",
                    description: Some(
                        "CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers.",
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
            name: "Dmacr",
            extends: None,
            description: Some(
                "DMA control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dien",
                    description: Some(
                        "DMA input enable.",
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
                    name: "doen",
                    description: Some(
                        "DMA output enable.",
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
            name: "Imscr",
            extends: None,
            description: Some(
                "interrupt mask set/clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inim",
                    description: Some(
                        "Input FIFO service interrupt mask.",
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
                    name: "outim",
                    description: Some(
                        "Output FIFO service interrupt mask.",
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
            name: "Misr",
            extends: None,
            description: Some(
                "masked interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmis",
                    description: Some(
                        "Input FIFO service masked interrupt status.",
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
                    name: "outmis",
                    description: Some(
                        "Output FIFO service masked interrupt status.",
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
            name: "Risr",
            extends: None,
            description: Some(
                "raw interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inris",
                    description: Some(
                        "Input FIFO service raw interrupt status.",
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
                    name: "outris",
                    description: Some(
                        "Output FIFO service raw interrupt status.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ifem",
                    description: Some(
                        "Input FIFO empty.",
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
                    name: "ifnf",
                    description: Some(
                        "Input FIFO not full.",
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
                    name: "ofne",
                    description: Some(
                        "Output FIFO not empty.",
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
                    name: "offu",
                    description: Some(
                        "Output FIFO full.",
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
                    name: "busy",
                    description: Some(
                        "Busy bit.",
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
                    name: "kerf",
                    description: Some(
                        "Key error flag This read-only bit is set by hardware when key information failed to load into key registers. KERF is triggered upon any of the following errors: CRYP_KxR/LR register write does not respect the correct order (refer to Section 60.4.16: CRYP key registers for details). CRYP fails to load the key shared by SAES peripheral (KMOD = 0x2). KERF must be cleared by the application software, otherwise KEYVALID cannot be set. It can be done through IPRST bit of CRYP_CR, or when a correct key writing sequence starts.",
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
                    name: "keyvalid",
                    description: Some(
                        "Key valid flag This read-only bit is set by hardware when the key of size defined by KEYSIZE is loaded in CRYP_KxR/LR key registers. The CRYPEN bit can only be set when KEYVALID is set. In normal mode when KMOD[1:0] is at zero, the key must be written in the key registers in the correct sequence, otherwise the KERF flag is set and KEYVALID remains cleared. When KMOD[1:0] is different from zero, the BUSY flag is automatically set by CRYP. When the key is loaded successfully, BUSY is cleared and KEYVALID set. Upon an error, KERF is set, BUSY cleared and KEYVALID remains cleared. If set, KERF must be cleared, otherwise KEYVALID cannot be set. For further information on key loading, refer to Section 60.4.16: CRYP key registers.",
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
    ],
    enums: &[
        Enum {
            name: "Kmod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal-key mode. Key registers are freely usable.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SHARED",
                    description: Some(
                        "Shared-key mode. If shared-key mode is properly initialized in SAES peripheral, the CRYP peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in CRYP key registers when BUSY bit is cleared and KEYVALID is set in the CRYP_SR register. Key error flag KERF is set otherwise in the CRYP_SR register.",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
