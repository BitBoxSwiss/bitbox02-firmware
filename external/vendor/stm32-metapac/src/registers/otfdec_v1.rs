
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Otfdec",
            extends: None,
            description: Some(
                "On-The-Fly Decryption engine.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "OTFDEC control register.",
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
                    name: "privcfgr",
                    description: Some(
                        "OTFDEC_PRIVCFGR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "region",
                    description: None,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 48,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Region",
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "OTFDEC interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x300,
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
                        "OTFDEC interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x304,
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
                    name: "ier",
                    description: Some(
                        "OTFDEC interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x308,
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
            ],
        },
        Block {
            name: "Region",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "OTFDEC region 3 configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegionCfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "startaddr",
                    description: Some(
                        "OTFDEC region 3 start address register.",
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
                BlockItem {
                    name: "endaddr",
                    description: Some(
                        "OTFDEC region 3 end address register.",
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
                    name: "noncer",
                    description: Some(
                        "OTFDEC region 3 nonce register 0.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "OTFDEC region 3 key register 0.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x14,
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
            name: "Cr",
            extends: None,
            description: Some(
                "OTFDEC control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enc",
                    description: Some(
                        "Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Enc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some(
                "OTFDEC interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seif",
                    description: Some(
                        "Security error interrupt flag clear This bit is written by application, and always read as 0.",
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
                    name: "xoneif",
                    description: Some(
                        "Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0.",
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
                    name: "keif",
                    description: Some(
                        "Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again.",
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
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "OTFDEC interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seie",
                    description: Some(
                        "Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set.",
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
                    name: "xoneie",
                    description: Some(
                        "Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set.",
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
                    name: "keie",
                    description: Some(
                        "Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set.",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "OTFDEC interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seif",
                    description: Some(
                        "Security error interrupt flag status This bit is set by hardware and read only by application. This bit is set when at least one security error has been detected. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.",
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
                    name: "xoneif",
                    description: Some(
                        "Execute-only execute-never error interrupt flag status This bit is set by hardware and read only by application. This bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 11. Lastly, XONEIF is also set when an execute access is detected while encryption mode is enabled. This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.",
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
                    name: "keif",
                    description: Some(
                        "Key error interrupt flag status This bit is set by hardware and read only by application. The bit is set when a read access occurs on an encrypted region, while its key registers is null or not properly initialized (KEYCRC = 0x0). This bit is cleared when the application sets in OTFDEC_ICR the corresponding bit to 1. After KEIF is set any subsequent read to the region with bad key registers returns a zeroed value. This state remains until those key registers are properly initialized (KEYCRC not zero).",
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
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "OTFDEC_PRIVCFGR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv_",
                    description: Some(
                        "Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads.",
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
            name: "RegionCfgr",
            extends: None,
            description: Some(
                "OTFDEC region 3 configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reg_en",
                    description: Some(
                        "region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set.",
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
                    name: "configlock",
                    description: Some(
                        "region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1.",
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
                    name: "keylock",
                    description: Some(
                        "region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset.",
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
                    name: "mode",
                    description: Some(
                        "operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "keycrc",
                    description: Some(
                        "region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written.",
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
                    name: "reg_version",
                    description: Some(
                        "region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR.",
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
            name: "Enc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DECRYPTION",
                    description: Some(
                        "OTFDEC working in decryption mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENCRYPTION",
                    description: Some(
                        "OTFDEC working in encryption mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "All read accesses are decrypted (instruction or data).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ENHANCED",
                    description: Some(
                        "Enhanced encryption mode is activated, and only instruction accesses are decrypted",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
