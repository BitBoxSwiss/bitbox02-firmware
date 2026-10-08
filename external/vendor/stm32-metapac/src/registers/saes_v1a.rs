
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Saes",
            extends: None,
            description: Some(
                "Secure advanced encryption standard hardware accelerator.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "SAES control register.",
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
                        "SAES status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "dinr",
                    description: Some(
                        "SAES data input register.",
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
                    name: "doutr",
                    description: Some(
                        "SAES data output register.",
                    ),
                    array: None,
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
                        "SAES key register 0.",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    32,
                                    36,
                                    40,
                                    44,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ivr",
                    description: Some(
                        "SAES initialization vector register 0.",
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
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "suspr",
                    description: Some(
                        "SAES suspend registers.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "SAES interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x300,
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
                        "SAES interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x304,
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
                        "SAES interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x308,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "SAES control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD[1:0] other than 00, use the IPRST bit rather than the bit EN.",
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
                    name: "datatype",
                    description: Some(
                        "Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Datatype",
                    ),
                },
                Field {
                    name: "mode",
                    description: Some(
                        "SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "chmod",
                    description: Some(
                        "Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Cursed(
                        CursedBitOffset {
                            ranges: &[
                                5..=6,
                                16..=16,
                            ],
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Chmod",
                    ),
                },
                Field {
                    name: "dmainen",
                    description: Some(
                        "DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE[1:0] bitfield. It is not effective for Mode 2 (key derivation).",
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
                    name: "dmaouten",
                    description: Some(
                        "DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE[1:0] bitfield. It is not effective for Mode 2 (key derivation).",
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
                    name: "gcmph",
                    description: Some(
                        "GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Gcmph",
                    ),
                },
                Field {
                    name: "keysize",
                    description: Some(
                        "Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD[1:0]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Keysize",
                    ),
                },
                Field {
                    name: "keyprot",
                    description: Some(
                        "Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
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
                        "Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...",
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
                        "Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0] bitfield. This sharing is valid only while KMOD[1:0]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD[1:0] other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.\\nAttempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
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
                    name: "kshareid",
                    description: Some(
                        "Key share identification This bitfield defines, at the end of a decryption process with KMOD[1:0]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Kshareid",
                    ),
                },
                Field {
                    name: "keysel",
                    description: Some(
                        "Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL[2:0] with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL[2:0] bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD[1:0] is other than zero, KEYSEL[2:0] is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Keysel",
                    ),
                },
                Field {
                    name: "iprst",
                    description: Some(
                        "SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.",
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
            name: "Icr",
            extends: None,
            description: Some(
                "SAES interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccf",
                    description: Some(
                        "Computation complete flag clear Setting this bit clears the CCF status bit of the SAES_SR and SAES_ISR registers.",
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
                    name: "rweif",
                    description: Some(
                        "Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the SAES_ISR register, and both RDERR and WRERR flags in the SAES_SR register.",
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
                        "Key error interrupt flag clear Setting this bit clears the KEIF status bit of the SAES_ISR register.",
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
                    name: "rngeif",
                    description: Some(
                        "RNG error interrupt flag clear Application must set this bit to clear the RNGEIF status bit in SAES_ISR register.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "SAES interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccfie",
                    description: Some(
                        "Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set.",
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
                    name: "rweie",
                    description: Some(
                        "Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set.",
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
                        "Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set.",
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
                    name: "rngeie",
                    description: Some(
                        "RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set.",
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
            name: "Isr",
            extends: None,
            description: Some(
                "SAES interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccf",
                    description: Some(
                        "Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the SAES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.",
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
                    name: "rweif",
                    description: Some(
                        "Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the SAES_SR register. RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the SAES_IER register. This flags has no meaning when key derivation mode is selected.",
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
                        "Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers or key register usage is forbidden. Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the SAES_IER register is set. KEIF is triggered upon any of the following errors: SAES fails to load the DHUK (KEYSEL = 001 or 100). SAES fails to load the BHK (KEYSEL = 010 or 100) respecting the correct order. AES fails to load the key shared by SAES peripheral (KMOD=10). When KEYVALID = 1 and (KEYPROT = 1 or KEYSEL is not 0x0), the security context of the application that loads the key (secure or non-secure) does not match the security attribute of the access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared. SAES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For KEYSIZE = 1, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.",
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
                    name: "rngeif",
                    description: Some(
                        "RNG error interrupt flag This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad entropy). RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing this bit triggers the reload of a new random number from RNG peripheral.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "SAES status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rderr",
                    description: Some(
                        "Read error flag This flag indicates the detection of an unexpected read operation from the SAES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected read returns zero.",
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
                    name: "wrerr",
                    description: Some(
                        "Write error This flag indicates the detection of an unexpected write operation to the SAES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected write is ignored.",
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
                    name: "busy",
                    description: Some(
                        "Busy This flag indicates whether SAES is idle or busy during GCM payload encryption phase: The flag is set upon SAES initialization, upon fetching random number from the RNG, or upon transferring a shared key to a target peripheral. When GCM encryption is selected, the flag must be at zero before selecting the GCM final phase.",
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
                    name: "keyvalid",
                    description: Some(
                        "Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in SAES_CR has been loaded in SAES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the SAES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by SAES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the SAES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the SAES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to.",
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
            name: "Chmod",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ECB",
                    description: Some(
                        "Electronic codebook",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CBC",
                    description: Some(
                        "Cipher-block chaining",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CTR",
                    description: Some(
                        "Counter mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "GCM_GMAC",
                    description: Some(
                        "Galois counter mode and Galois message authentication code",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CCM",
                    description: Some(
                        "Counter with CBC-MAC",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Datatype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "No swapping (32-bit data).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF_WORD",
                    description: Some(
                        "Half-word swapping (16-bit data)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTE",
                    description: Some(
                        "Byte swapping (8-bit data)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT",
                    description: Some(
                        "Bit-level swapping",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Gcmph",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INIT_PHASE",
                    description: Some(
                        "Initialization phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HEADER_PHASE",
                    description: Some(
                        "Header phase",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PAYLOAD_PHASE",
                    description: Some(
                        "Payload phase",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FINAL_PHASE",
                    description: Some(
                        "Final phase",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Keysel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SOFTWARE_KEY",
                    description: Some(
                        "Software key, loaded in key registers SAES_KEYx",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DHUK",
                    description: Some(
                        "Derived hardware unique key",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BHK",
                    description: Some(
                        "Boot hardware key",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "XOR_DHUK_BHK",
                    description: Some(
                        "XOR of DHUK and BHK",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Keysize",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BITS128",
                    description: Some(
                        "128-bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS256",
                    description: Some(
                        "256-bit",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Kmod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "AES peripheral",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WRAPPED_KEY",
                    description: Some(
                        "Wrapped key for SAES mode. Key loaded in key registers can only be used to encrypt or\ndecrypt AES keys. Hence, when a decryption is selected, read-as-zero SAES_DOUTR register is\nautomatically loaded into SAES key registers after a successful decryption process.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SHARED_KEY",
                    description: Some(
                        "Shared key mode. After a successful decryption process (unwrapping), SAES key registers are\nshared with the peripheral described in KSHAREID[1:0] bitfield. This sharing is valid only while\nKMOD[1:0] at 0x2 and KEYVALID=1. When a decryption is selected, read-as-zero SAES_DOUTR\nregister is automatically loaded into SAES key registers after a successful decryption process.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Kshareid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "AES",
                    description: Some(
                        "AES peripheral",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ENCRYPTION",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "KEY_DERIVATION",
                    description: Some(
                        "Key derivation (or key preparation), for ECB/CBC decryption only",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DECRYPTION",
                    description: None,
                    value: 2,
                },
            ],
        },
    ],
};
