
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "Embedded Flash memory.",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "Access control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "FLASH control key register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Keyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "FLASH control register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                        "FLASH status register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "FLASH status register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "FLASH interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                        "FLASH interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                        "FLASH interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "crccr",
                    description: Some(
                        "FLASH CRC control register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcsaddr",
                    description: Some(
                        "FLASH CRC start address register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crceaddr",
                    description: Some(
                        "FLASH CRC end address register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crceaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcdatar",
                    description: Some(
                        "FLASH CRC data register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccsfaddr",
                    description: Some(
                        "FLASH ECC single error fail address.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccsfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdfaddr",
                    description: Some(
                        "FLASH ECC double error fail address.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "FLASH options key register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optcr",
                    description: Some(
                        "FLASH options control register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optisr",
                    description: Some(
                        "FLASH options interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opticr",
                    description: Some(
                        "FLASH options interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opticr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obkcr",
                    description: Some(
                        "FLASH option byte key control register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obkcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obkdr",
                    description: Some(
                        "FLASH option bytes key data register 0.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "nvsr",
                    description: Some(
                        "FLASH non-volatile status register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nvsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nvsrp",
                    description: Some(
                        "FLASH security status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nvsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotsr",
                    description: Some(
                        "FLASH RoT status register.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rotsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotsrp",
                    description: Some(
                        "FLASH RoT status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rotsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otplsr",
                    description: Some(
                        "FLASH OTP lock status register.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otplsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otplsrp",
                    description: Some(
                        "FLASH OTP lock status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otplsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpsr",
                    description: Some(
                        "FLASH write protection status register.",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpsrp",
                    description: Some(
                        "FLASH write protection status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdpsr",
                    description: Some(
                        "FLASH hide protection status register.",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdpsrp",
                    description: Some(
                        "FLASH hide protection status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdpsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "epochsr",
                    description: Some(
                        "FLASH epoch status register.",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Epochsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "epochsrp",
                    description: Some(
                        "FLASH RoT status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Epochsrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obw1sr",
                    description: Some(
                        "FLASH option byte word 1 status register.",
                    ),
                    array: None,
                    byte_offset: 0x260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obw1sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obw1srp",
                    description: Some(
                        "FLASH option byte word 1 status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obw1srp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obw2sr",
                    description: Some(
                        "FLASH option byte word 2 status register.",
                    ),
                    array: None,
                    byte_offset: 0x268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obw2sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obw2srp",
                    description: Some(
                        "FLASH option byte word 2 status register programming.",
                    ),
                    array: None,
                    byte_offset: 0x26c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obw2srp",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "Access control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. Please refer to Table 27 for details. ... Note: Embedded Flash does not verify that the configuration is correct.",
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
                    name: "wrhighfreq",
                    description: Some(
                        "Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to Table 27 for details. Note: Embedded Flash does not verify that the configuration is correct.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "FLASH control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "Configuration lock bit When this bit is set write to all other bits in this register, and to FLASH_IER register, are ignored. Clearing this bit requires the correct write sequence to FLASH_KEYR register (see this register for details). If a wrong sequence is executed, or if the unlock sequence is performed twice, this bit remains locked until the next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.",
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
                    name: "pg",
                    description: Some(
                        "Internal buffer control bit Setting this bit enables internal buffer for write operations. This allows preparing program operations even if a sector or bank erase is ongoing. When PG is cleared, the internal buffer is disabled for write operations, and all the data stored in the buffer but not sent to the operation queue are lost.",
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
                    name: "ser",
                    description: Some(
                        "Sector erase request Setting this bit requests a sector erase. Write protection error is triggered when a sector erase is required on at least one protected sector. BER has a higher priority than SER: if both bits are set, the embedded Flash memory executes a bank erase.",
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
                    name: "ber",
                    description: Some(
                        "Bank erase request Setting this bit requests a bank erase operation (user Flash memory only). Write protection error is triggered when a bank erase is required and some sectors are protected. BER has a higher priority than SER: if both are set, the embedded Flash memory executes a bank erase.",
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
                    name: "fw",
                    description: Some(
                        "Force write This bit forces a write operation even if the write buffer is not full. In this case all bits not written are set by hardware. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it will lead to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively).",
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
                    name: "start",
                    description: Some(
                        "Erase start control bit This bit is used to start a sector erase or a bank erase operation. The embedded Flash memory resets START when the corresponding operation has been acknowledged. The user application cannot access any embedded Flash memory register until the operation is acknowledged.",
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
                    name: "ssn",
                    description: Some(
                        "Sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pg_otp",
                    description: Some(
                        "Program Enable for OTP Area Set this bit to enable write operations to OTP area.",
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
                    name: "crc_en",
                    description: Some(
                        "CRC enable Setting this bit enables the CRC calculation. CRC_EN does not start CRC calculation but enables CRC configuration through FLASH_CRCCR register. When CRC calculation is performed it can be disabled by clearing CRC_EN bit. Doing so sets CRCDATA to 0x0, clears CRC configuration and resets the content of FLASH_CRCDATAR register.",
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
                    name: "all_banks",
                    description: Some(
                        "All banks select bit When this bit is set the erase is done on all Flash Memory sectors. ALL_BANKS is used only if a bank erase is required (BER=1). In all others operations, this control bit is ignored.",
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
            name: "Crccr",
            extends: None,
            description: Some(
                "FLASH CRC control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_sect",
                    description: Some(
                        "CRC sector number CRC_SECT is used to select one user Flash sectors to be added to the list of sectors on which the CRC is calculated. The CRC can be computed either between two addresses (using registers FLASH_CRCSADDR and FLASH_CRCEADDR) or on a list of sectors using this register. If this latter option is selected, it is possible to add a sector to the list of sectors by programming the sector number in CRC_SECT and then setting ADD_SECT bit. The list of sectors can be erased either by setting CLEAN_SECT bit or by disabling the CRC computation. ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crc_by_sect",
                    description: Some(
                        "CRC sector mode select bit When this bit is set the CRC calculation is performed at sector level, on the sectors present in the list of sectors. To add a sector to this list, use ADD_SECT and CRC_SECT bits. To clean the list, use CLEAN_SECT bit. When CRC_BY_SECT is cleared the CRC calculation is performed on all addresses defined between start and end addresses defined in FLASH_CRCSADDR and FLASH_CRCEADDR registers.",
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
                    name: "add_sect",
                    description: Some(
                        "CRC sector select bit When this bit is set the sector whose number is written in CRC_SECT is added to the list of sectors on which the CRC is calculated.",
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
                    name: "clean_sect",
                    description: Some(
                        "CRC sector list clear bit When this bit is set the list of sectors on which the CRC is calculated is cleared.",
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
                    name: "start_crc",
                    description: Some(
                        "CRC start bit START_CRC bit triggers a CRC calculation using the current configuration. No CRC calculation can launched when an option byte change operation is ongoing because all read accesses to embedded Flash memory registers are put on hold until the option byte change operation has completed. This bit is cleared when CRC computation starts.",
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
                    name: "clean_crc",
                    description: Some(
                        "CRC clear bit Setting CLEAN_CRC to 1 clears the current CRC result stored in the FLASH_CRCDATAR register.",
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
                    name: "crc_burst",
                    description: Some(
                        "CRC burst size CRC_BURST bits set the size of the bursts that are generated by the CRC calculation unit. A Flash word is 128-bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "CrcBurst",
                    ),
                },
                Field {
                    name: "all_sect",
                    description: Some(
                        "All sectors selection When this bit is set all the sectors in user Flash are added to list of sectors on which the CRC shall be calculated. This bit is cleared when CRC computation starts.",
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
            name: "Crcdatar",
            extends: None,
            description: Some(
                "FLASH CRC data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_data",
                    description: Some(
                        "CRC result This bitfield contains the result of the last CRC calculation. The value is valid only when CRC calculation completed (CRCENDF is set in FLASH_ISR register). CRC_DATA is cleared when CRC_EN is cleared in FLASH_CR register (CRC disabled), or when CLEAN_CRC bit is set in FLASH_CRCCR register.",
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
            name: "Crceaddr",
            extends: None,
            description: Some(
                "FLASH CRC end address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_end_addr",
                    description: Some(
                        "CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits [5:0] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crcsaddr",
            extends: None,
            description: Some(
                "FLASH CRC start address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_start_addr",
                    description: Some(
                        "CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits [5:0] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccdfaddr",
            extends: None,
            description: Some(
                "FLASH ECC double error fail address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ded_fadd",
                    description: Some(
                        "ECC double error detection fail address When a double ECC detection occurs during a read operation, the DED_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when the DBECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC double error detection error is saved in this register.",
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
            name: "Eccsfaddr",
            extends: None,
            description: Some(
                "FLASH ECC single error fail address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_fadd",
                    description: Some(
                        "ECC single error correction fail address When a single ECC error correction occurs during a read operation, the SEC_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when SNECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC single error correction error is saved in this register.",
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
            name: "Epochsr",
            extends: None,
            description: Some(
                "FLASH epoch status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epoch",
                    description: Some(
                        "Epoch This value is distributed by hardware to the SAES peripheral.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Epochsrp",
            extends: None,
            description: Some(
                "FLASH RoT status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epoch",
                    description: Some(
                        "Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fcr",
            extends: None,
            description: Some(
                "FLASH status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcheckf",
                    description: Some(
                        "Root code check flag clear Set this bit to clear RCHECKF bit in FLASH_SR.",
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
            ],
        },
        FieldSet {
            name: "Hdpsr",
            extends: None,
            description: Some(
                "FLASH hide protection status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp_area_start",
                    description: Some(
                        "Hide protection user Flash area start This option sets the start address that contains the first 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdp_area_end",
                    description: Some(
                        "Hide protection user Flash area end This option sets the end address that contains the last 256-byte block of the hide protection (HDP) area in user Flash area. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hdpsrp",
            extends: None,
            description: Some(
                "FLASH hide protection status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp_area_start",
                    description: Some(
                        "Hide protection user Flash area start programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdp_area_end",
                    description: Some(
                        "Hide protection user Flash area end programming Write to change corresponding option byte bits in FLASH_HDPSR. If HDP_AREA_END=HDP_AREA_START all the sectors are protected. If HDP_AREA_END<HDP_AREA_START no sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some(
                "FLASH interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eopf",
                    description: Some(
                        "End-of-program flag clear Setting this bit clears EOPF flag in FLASH_ISR register.",
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
                    name: "wrperrf",
                    description: Some(
                        "Write protection error flag clear Setting this bit clears WRPERRF flag in FLASH_ISR register.",
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
                    name: "pgserrf",
                    description: Some(
                        "Programming sequence error flag clear Setting this bit clears PGSERRF flag in FLASH_ISR register.",
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
                    name: "strberrf",
                    description: Some(
                        "Strobe error flag clear Setting this bit clears STRBERRF flag in FLASH_ISR register.",
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
                    name: "oblerrf",
                    description: Some(
                        "Option byte loading error flag clear Setting this bit clears OBLERRF flag in FLASH_ISR register.",
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
                    name: "incerrf",
                    description: Some(
                        "Inconsistency error flag clear Setting this bit clears INCERRF flag in FLASH_ISR register.",
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
                    name: "rdserrf",
                    description: Some(
                        "Read security error flag clear Setting this bit clears RDSERRF flag in FLASH_ISR register.",
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
                    name: "sneccerrf",
                    description: Some(
                        "ECC single error flag clear Setting this bit clears SNECCERRF flag in FLASH_ISR register. If the DBECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.",
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
                    name: "dbeccerrf",
                    description: Some(
                        "ECC double error flag clear Setting this bit clears DBECCERRF flag in FLASH_ISR register. If the SNECCERRF flag of FLASH_ISR register is also cleared, FLASH_ECCFAR register is reset to zero as well.",
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
                    name: "crcendf",
                    description: Some(
                        "CRC end flag clear Setting this bit clears CRCENDF flag in FLASH_ISR register.",
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
                    name: "crcrderrf",
                    description: Some(
                        "CRC error flag clear Setting this bit clears CRCRDERRF flag in FLASH_ISR register.",
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
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "FLASH interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eopie",
                    description: Some(
                        "End-of-program interrupt control bit.",
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
                    name: "wrperrie",
                    description: Some(
                        "Write protection error interrupt enable bit.",
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
                    name: "pgserrie",
                    description: Some(
                        "Programming sequence error interrupt enable bit.",
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
                    name: "strberrie",
                    description: Some(
                        "Strobe error interrupt enable bit.",
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
                    name: "oblerrie",
                    description: Some(
                        "Option byte loading error interrupt enable bit.",
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
                    name: "incerrie",
                    description: Some(
                        "Inconsistency error interrupt enable bit.",
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
                    name: "rdserrie",
                    description: Some(
                        "Read security error interrupt enable bit.",
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
                    name: "sneccerrie",
                    description: Some(
                        "ECC single correction error interrupt enable bit.",
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
                    name: "dbeccerrie",
                    description: Some(
                        "ECC double detection error interrupt enable bit.",
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
                    name: "crcendie",
                    description: Some(
                        "CRC end of calculation interrupt enable bit.",
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
                    name: "crcrderrie",
                    description: Some(
                        "CRC read error interrupt enable bit.",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "FLASH interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eopf",
                    description: Some(
                        "End-of-program flag This bit is set when a programming operation completes. An interrupt is generated if the EOPIE is set. It is not necessary to reset EOPF before starting a new operation. Setting EOPF bit in FLASH_ICR register clears this bit.",
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
                    name: "wrperrf",
                    description: Some(
                        "Write protection error flag This bit is set when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set. Setting WRPERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "pgserrf",
                    description: Some(
                        "Programming sequence error flag This bit is set when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set. Setting PGSERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "strberrf",
                    description: Some(
                        "Strobe error flag This bit is set when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set. Setting STRBERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "oblerrf",
                    description: Some(
                        "Option byte loading error flag This bit is set when an error is found during the option byte loading sequence. An interrupt is generated if OBLERRIE is set. Setting OBLERRF bit in the FLASH_ICR register clears this bit.",
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
                    name: "incerrf",
                    description: Some(
                        "Inconsistency error flag This bit is set when a inconsistency error occurs. An interrupt is generated if INCERRIE is set. Setting INCERRF bit in the FLASH_ICR register clears this bit.",
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
                    name: "rdserrf",
                    description: Some(
                        "Read security error flag This bit is set when a read security error occurs (read access to hide protected area with incorrect hide protection level). An interrupt is generated if RDSERRIE is set. Setting RDSERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "sneccerrf",
                    description: Some(
                        "ECC single error flag This bit is set when an ECC single correction error occurs during a read operation. An interrupt is generated if SNECCERRIE is set. Setting SNECCERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "dbeccerrf",
                    description: Some(
                        "ECC double error flag This bit is set when an ECC double detection error occurs during a read operation. An interrupt is generated if DBECCERRIE is set. Setting DBECCERRF bit in FLASH_ICR register clears this bit.",
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
                    name: "crcendf",
                    description: Some(
                        "CRC end flag This bit is set when the CRC computation has completed. An interrupt is generated if CRCENDIE is set. It is not necessary to reset CRCEND before restarting CRC computation. Setting CRCENDF bit in FLASH_ICR register clears this bit.",
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
                    name: "crcrderrf",
                    description: Some(
                        "CRC read error flag This bit is set when a word is found read protected during a CRC operation. An interrupt is generated if CRCRDIE is set. Setting CRCRDERRF bit in FLASH_ICR register clears this bit. This flag is valid only when CRCEND bit is set.",
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
            ],
        },
        FieldSet {
            name: "Keyr",
            extends: None,
            description: Some(
                "FLASH control key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cukey",
                    description: Some(
                        "Control unlock key Following values must be written to FLASH_KEYR consecutively to unlock FLASH_CR register: 1st key = 0x4567 0123 2nd key = 0xCDEF 89AB Reads to this register returns zero. If above sequence is wrong or performed twice, the FLASH_CR register is locked until the next system reset, and access to it generates a bus error.",
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
            name: "Nvsr",
            extends: None,
            description: Some(
                "FLASH non-volatile status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nvstate",
                    description: Some(
                        "Non-volatile state others: invalid configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "NvsrNvstate",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Nvsrp",
            extends: None,
            description: Some(
                "FLASH security status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nvstate",
                    description: Some(
                        "Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "NvsrpNvstate",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Obkcr",
            extends: None,
            description: Some(
                "FLASH option byte key control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "obkindex",
                    description: Some(
                        "Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them.",
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
                    name: "nextkl",
                    description: Some(
                        "Next key level 10 or 11: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Nextkl",
                    ),
                },
                Field {
                    name: "obksize",
                    description: Some(
                        "Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Obksize",
                    ),
                },
                Field {
                    name: "keyprog",
                    description: Some(
                        "Key program This bit must be set to write option byte keys (keys are read otherwise).",
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
                    name: "keystart",
                    description: Some(
                        "Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged.",
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
            ],
        },
        FieldSet {
            name: "Obw1sr",
            extends: None,
            description: Some(
                "FLASH option byte word 1 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bor_lev",
                    description: Some(
                        "Brownout level These bits reflects the power level that generates a system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "BorLev",
                    ),
                },
                Field {
                    name: "iwdg_hw",
                    description: Some(
                        "Independent watchdog HW Control.",
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
                    name: "nrst_stop",
                    description: Some(
                        "Reset on stop mode.",
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
                    name: "nrst_stby",
                    description: Some(
                        "Reset on standby mode.",
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
                    name: "octo1_hslv",
                    description: Some(
                        "XSPIM_P1 High-Speed at Low-Voltage.",
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
                    name: "octo2_hslv",
                    description: Some(
                        "XSPIM_P2 High-Speed at Low-Voltage.",
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
                    name: "iwdg_fz_stop",
                    description: Some(
                        "IWDG stop mode freeze When set the independent watchdog IWDG is frozen in system Stop mode.",
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
                    name: "iwdg_fz_sdby",
                    description: Some(
                        "IWDG standby mode freeze When set the independent watchdog IWDG is frozen in system Standby mode.",
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
                    name: "perso_ok",
                    description: Some(
                        "Personalization OK This bit is set on STMicroelectronics production line.",
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
                    name: "vddio_hslv",
                    description: Some(
                        "I/O High-Speed at Low-Voltage This bit indicates that the product operates below 2.5 V.",
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
            ],
        },
        FieldSet {
            name: "Obw1srp",
            extends: None,
            description: Some(
                "FLASH option byte word 1 status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bor_lev",
                    description: Some(
                        "Brownout level Write to change corresponding bits in FLASH_OBW1SR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_hw",
                    description: Some(
                        "Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "nrst_stop",
                    description: Some(
                        "Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "nrst_stby",
                    description: Some(
                        "Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "octo1_hslv",
                    description: Some(
                        "XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "octo2_hslv",
                    description: Some(
                        "XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "iwdg_fz_stop",
                    description: Some(
                        "IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "iwdg_fz_sdby",
                    description: Some(
                        "IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register.",
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
                    name: "vddio_hslv",
                    description: Some(
                        "I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.",
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
            ],
        },
        FieldSet {
            name: "Obw2sr",
            extends: None,
            description: Some(
                "FLASH option byte word 2 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itcm_axi_share",
                    description: Some(
                        "ITCM SRAM configuration.",
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
                    name: "dtcm_axi_share",
                    description: Some(
                        "DTCM SRAM configuration.",
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
                    name: "ecc_on_sram",
                    description: Some(
                        "ECC on SRAM.",
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
                    name: "i2c_ni3c",
                    description: Some(
                        "I2C Not I3C.",
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
            name: "Obw2srp",
            extends: None,
            description: Some(
                "FLASH option byte word 2 status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itcm_axi_share",
                    description: Some(
                        "ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes.",
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
                    name: "dtcm_axi_share",
                    description: Some(
                        "DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes.",
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
                    name: "ecc_on_sram",
                    description: Some(
                        "ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register.",
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
                    name: "i2c_ni3c",
                    description: Some(
                        "I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register.",
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
            name: "Optcr",
            extends: None,
            description: Some(
                "FLASH options control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optlock",
                    description: Some(
                        "Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.",
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
                    name: "pg_opt",
                    description: Some(
                        "Program options.",
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
                    name: "kveie",
                    description: Some(
                        "Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR.",
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
                    name: "kteie",
                    description: Some(
                        "Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR.",
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
                    name: "opterrie",
                    description: Some(
                        "Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change.",
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
            ],
        },
        FieldSet {
            name: "Opticr",
            extends: None,
            description: Some(
                "FLASH options interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "kvef",
                    description: Some(
                        "key valid error flag Set this bit to clear KVEF flag in FLASH_OPTISR register.",
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
                    name: "ktef",
                    description: Some(
                        "key transfer error flag Set this bit to clear KTEF flag in FLASH_OPTISR register.",
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
                    name: "opterrf",
                    description: Some(
                        "Option byte change error flag Set this bit to clear OPTERRF flag in FLASH_OPTISR register.",
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
            ],
        },
        FieldSet {
            name: "Optisr",
            extends: None,
            description: Some(
                "FLASH options interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "kvef",
                    description: Some(
                        "Key valid error flag This bit is set when loading an unknown or corrupted option byte key. More specifically: Embedded Flash did not find an option byte key that corresponds to the given OBKINDEX[4:0] and the requested HDPL (optionally modified by NEXTKL[1:0]). It can happen for example when requested key has not being provisioned. A double error detection was found when loading the requested option byte key. In this case, if this key is provisioned again the error should disappear. When KVEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KVEIE bit of FLASH_OPTCR register is set. Setting KVEF bit of register FLASH_OPTICR clears this bit.",
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
                    name: "ktef",
                    description: Some(
                        "Key transfer error flag This bit is set when embedded Flash signals an error to the SAES peripheral. It happens when the key size (128-bit or 256-bit) is not matching between embedded Flash OBKSIZE[1:0] and KEYSIZE bit in SAES_CR register. It also happen when an ECC dual error detection occurred while embedded Flash loaded an option byte key for the SAES peripheral. When KTEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KTEIE bit of FLASH_OPTCR register is set. Setting KTEF bit of register FLASH_OPTICR clears this bit.",
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
                    name: "opterrf",
                    description: Some(
                        "Option byte change error flag When OPTERRF is set, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTERRIE bit of FLASH_OPTCR register is set. Setting OPTERRF of register FLASH_OPTICR clears this bit.",
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
            ],
        },
        FieldSet {
            name: "Optkeyr",
            extends: None,
            description: Some(
                "FLASH options key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ocukey",
                    description: Some(
                        "Options configuration unlock key Following values must be written to FLASH_OPTKEYR consecutively to unlock FLASH_OPTCR register: 1st key = 0x0819 2A3B 2nd key = 0x4C5D 6E7F Reads to this register returns zero. If above sequence is performed twice locks up the corresponding register/bit until the next system reset, and generates a bus error.",
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
            name: "Otplsr",
            extends: None,
            description: Some(
                "FLASH OTP lock status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otpl",
                    description: Some(
                        "OTP lock n Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. OTPL[n] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. OTPL[n] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified.",
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
            name: "Otplsrp",
            extends: None,
            description: Some(
                "FLASH OTP lock status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otpl",
                    description: Some(
                        "OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared.",
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
            name: "Rotsr",
            extends: None,
            description: Some(
                "FLASH RoT status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oem_provd",
                    description: Some(
                        "OEM provisioned device Any other value: device is not provisioned by the OEM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "OemProvd",
                    ),
                },
                Field {
                    name: "dbg_auth",
                    description: Some(
                        "Debug authentication method Any other value: no authentication method selected (NotSet).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "DbgAuth",
                    ),
                },
                Field {
                    name: "irot_select",
                    description: Some(
                        "iRoT selection This option is ignored for STM32H7R devices (OEM iRoT is always selected). Any other value: OEM iRoT is selected at boot.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "IrotSelect",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rotsrp",
            extends: None,
            description: Some(
                "FLASH RoT status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oem_provd",
                    description: Some(
                        "OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1.",
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
                    name: "dbg_auth",
                    description: Some(
                        "Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0.",
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
                    name: "irot_select",
                    description: Some(
                        "iRoT selection This option is ignored for STM32H7R devices. Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1 and if NVSTATE is not 0xB4 (OPEN).",
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
            name: "Sr",
            extends: None,
            description: Some(
                "FLASH status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "Busy flag This bit is set when an effective write, erase or option byte change operation is ongoing. It is possible to know what type of operation is being executed reading the flags IS_PROGRAM, IS_ERASE and IS_OPTCHANGE. BUSY cannot be cleared by application. It is automatically reset by hardware every time a step in a write, erase or option byte change operation completes. It is not recommended to do software polling on BUSY to know when one operation completed because, depending of operation, more pulses are possible for one only operation. For software polling it is therefore better to use QW flag or to check the EOPF flag.",
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
                    name: "wbne",
                    description: Some(
                        "Write buffer not empty flag This bit is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_CR the embedded Flash memory detects an error that involves data loss the application software has disabled write operations This bit cannot be forced to 0. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.",
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
                    name: "qw",
                    description: Some(
                        "Wait queue flag This bit is set when a write, erase or option byte change operation is pending in the command queue buffer. It is not possible to know what type of programming operation is present in the queue. This flag is reset by hardware when all write, erase or option byte change operations have been executed and thus removed from the waiting queue(s). This bit cannot be forced to 0. It is reset after a deterministic time if no other operations are requested.",
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
                    name: "crc_busy",
                    description: Some(
                        "CRC busy flag This bit is set when a CRC calculation is ongoing. This bit cannot be forced to 0. The user must wait until the CRC calculation has completed or disable CRC computation using CRC_EN bit in FLASH_CR register.",
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
                    name: "is_program",
                    description: Some(
                        "Is a program This bit is set together with BUSY when a program operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an program operation can happen during an option change.",
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
                    name: "is_erase",
                    description: Some(
                        "Is an erase This bit is set together with BUSY when an erase operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_OPTCHANGE, because an erase operation can happen during an option change.",
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
                    name: "is_optchange",
                    description: Some(
                        "Is an option change This bit is set together with BUSY when an option change operation is ongoing. It is cleared when BUSY is cleared. This flag can also raise with IS_PROGRAM or IS_ERASE, because a program or erase step is ongoing during option change.",
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
                    name: "rcheckf",
                    description: Some(
                        "Root code check flag This bit returns the status of the root code check performed following the first access to the Flash. This bit is cleared with RCHECKF bit in FLASH_FCR (optional).",
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
            ],
        },
        FieldSet {
            name: "Wrpsr",
            extends: None,
            description: Some(
                "FLASH write protection status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrps",
                    description: Some(
                        "Write protection for sector n This bit reflects the write protection status of user Flash sector n.",
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
            name: "Wrpsrp",
            extends: None,
            description: Some(
                "FLASH write protection status register programming.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrps",
                    description: Some(
                        "Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR.",
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
    ],
    enums: &[
        Enum {
            name: "BorLev",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "BOR OFF, POR/PDR reset threshold level is applied.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "BOR Level 1, the threshold level is low (around 2.1 V).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "BOR Level 2, the threshold level is medium (around 2.4 V).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "BOR Level 3, the threshold level is high (around 2.7 V).",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CrcBurst",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "WORD4",
                    description: Some(
                        "every burst has a size of 4 Flash words (64 Bytes).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WORD16",
                    description: Some(
                        "every burst has a size of 16 Flash words (256 Bytes).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WORD64",
                    description: Some(
                        "every burst has a size of 64 Flash words (1 Kbytes).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WORD256",
                    description: Some(
                        "every burst has a size of 256 Flash words (4 Kbytes).",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "DbgAuth",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "ECDSA",
                    description: Some(
                        "Authentication method using ECDSA signature (NIST P256).",
                    ),
                    value: 81,
                },
                EnumVariant {
                    name: "DELEGATED",
                    description: Some(
                        "Delegated debug (to OEM iRoT code in user Flash).",
                    ),
                    value: 111,
                },
                EnumVariant {
                    name: "PASSWORD",
                    description: Some(
                        "Authentication method using password.",
                    ),
                    value: 138,
                },
                EnumVariant {
                    name: "LOCKED",
                    description: Some(
                        "Locked device (no debug allowed).",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "IrotSelect",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "SELECTED",
                    description: Some(
                        "ST iRoT is selected at boot.",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "Nextkl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLUS0",
                    description: Some(
                        "OBKINDEX represents the index of the option byte key stored for the hide protection level indicated in SBS_HDPLSR.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLUS1",
                    description: Some(
                        "OBKINDEX represents the index of the option byte key stored for the hide protection level indicated in SBS_HDPLSR plus one (e.g. if HDPL=1 in SBS_HDPLR the key of level 2 is selected).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "NvsrNvstate",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "CLOSED",
                    description: Some(
                        "CLOSED device.",
                    ),
                    value: 81,
                },
                EnumVariant {
                    name: "OPEN",
                    description: Some(
                        "OPEN device.",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "NvsrpNvstate",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "CLOSE",
                    description: Some(
                        "CLOSE.",
                    ),
                    value: 81,
                },
                EnumVariant {
                    name: "OPEN",
                    description: Some(
                        "OPEN.",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "Obksize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "Key size is 32 bits.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS64",
                    description: Some(
                        "Key size is 64 bits.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS128",
                    description: Some(
                        "Key size is 128 bits.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS256",
                    description: Some(
                        "Key size is 256 bits.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "OemProvd",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "PROVISIONED",
                    description: Some(
                        "Device has been provisioned by the OEM.",
                    ),
                    value: 180,
                },
            ],
        },
    ],
};
