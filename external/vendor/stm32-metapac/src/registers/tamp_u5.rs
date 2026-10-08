
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tamp",
            extends: None,
            description: Some(
                "Tamper and backup registers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "TAMP control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "TAMP control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr3",
                    description: Some(
                        "TAMP control register 3",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltcr",
                    description: Some(
                        "TAMP filter control register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "atcr1",
                    description: Some(
                        "TAMP active tamper control register 1",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Atcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "atseedr",
                    description: Some(
                        "TAMP active tamper seed register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Atseedr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ator",
                    description: Some(
                        "TAMP active tamper output register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ator",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "atcr2",
                    description: Some(
                        "TAMP active tamper control register 2",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Atcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "TAMP secure mode register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "TAMP privilege mode control register",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "ier",
                    description: Some(
                        "TAMP interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    name: "sr",
                    description: Some(
                        "TAMP status register",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "misr",
                    description: Some(
                        "TAMP non-secure masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smisr",
                    description: Some(
                        "TAMP secure masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scr",
                    description: Some(
                        "TAMP status clear register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "countr",
                    description: Some(
                        "TAMP monotonic counter 1 register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Countr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ercfgr",
                    description: Some(
                        "TAMP erase configuration register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ercfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bkpr",
                    description: Some(
                        "TAMP backup X register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bkpr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Atcr1",
            extends: None,
            description: Some(
                "TAMP active tamper control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampam",
                    description: Some(
                        "Tamper X active mode",
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
                    name: "atosel",
                    description: Some(
                        "Active tamper shared output X selection. The selected output must be available in the package pinout",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
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
                    name: "atcksel",
                    description: Some(
                        "Active tamper RTC asynchronous prescaler clock selection. These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE.. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128.. .... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Atcksel",
                    ),
                },
                Field {
                    name: "atper",
                    description: Some(
                        "Active tamper output change period. The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to .",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atoshare",
                    description: Some(
                        "Active tamper output sharing. IN1 is compared with TAMPOUTSEL1. IN2 is compared with TAMPOUTSEL2. IN3 is compared with TAMPOUTSEL3. IN4 is compared with TAMPOUTSEL4. IN5 is compared with TAMPOUTSEL5. IN6 is compared with TAMPOUTSEL6. IN7 is compared with TAMPOUTSEL7. IN8 is compared with TAMPOUTSEL8",
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
                    name: "flten",
                    description: Some(
                        "Active tamper filter enable",
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
            name: "Atcr2",
            extends: None,
            description: Some(
                "TAMP active tamper control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "atosel",
                    description: Some(
                        "Active tamper shared output X selection. The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1[1:0] in the ATCR1, and so can also be read or. written through ATCR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ator",
            extends: None,
            description: Some(
                "TAMP active tamper output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prng",
                    description: Some(
                        "Pseudo-random generator value. This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode.",
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
                    name: "seedf",
                    description: Some(
                        "Seed running flag. This flag is set by hardware when a new seed is written in the ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set.",
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
                    name: "inits",
                    description: Some(
                        "Active tamper initialization status. This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled.",
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
            name: "Atseedr",
            extends: None,
            description: Some(
                "TAMP active tamper seed register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seed",
                    description: Some(
                        "Pseudo-random generator seed value. This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG.",
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
            name: "Bkpr",
            extends: None,
            description: Some(
                "TAMP backup register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkp",
                    description: Some(
                        "The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.",
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
            name: "Countr",
            extends: None,
            description: Some(
                "TAMP monotonic counter 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "count",
                    description: Some(
                        "This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value.",
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
            name: "Cr1",
            extends: None,
            description: Some(
                "TAMP control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampe",
                    description: Some(
                        "Tamper detection on INx enable",
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
                    name: "itampe",
                    description: Some(
                        "Internal tamper X enable",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "TAMP control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampnoer",
                    description: Some(
                        "Tamper X no erase",
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
                    name: "tampmsk",
                    description: Some(
                        "Tamper X mask. The tamper 1 interrupt must not be enabled when TAMP1MSK is set.",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bkblock",
                    description: Some(
                        "Backup registers and device secrets access blocked",
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
                    name: "bkerase",
                    description: Some(
                        "Backup registers and device secrets erase. Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0.",
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
                    name: "tamptrg",
                    description: Some(
                        "Active level for tamper 1 input.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
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
                    enumm: Some(
                        "Tamptrg",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "TAMP control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itampnoer",
                    description: Some(
                        "Internal Tamper X no erase",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ercfgr",
            extends: None,
            description: Some(
                "TAMP erase configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ercfg0",
                    description: Some(
                        "Configurable device secrets configuration",
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
            name: "Fltcr",
            extends: None,
            description: Some(
                "TAMP filter control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampfreq",
                    description: Some(
                        "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Tampfreq",
                    ),
                },
                Field {
                    name: "tampflt",
                    description: Some(
                        "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tampflt",
                    ),
                },
                Field {
                    name: "tampprch",
                    description: Some(
                        "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tampprch",
                    ),
                },
                Field {
                    name: "tamppudis",
                    description: Some(
                        "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "TAMP interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampie",
                    description: Some(
                        "Tamper X interrupt enable",
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
                    name: "itampie",
                    description: Some(
                        "Internal tamper X interrupt enable",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Misr",
            extends: None,
            description: Some(
                "TAMP non-secure masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampmf",
                    description: Some(
                        "TAMPx non-secure interrupt masked flag. This flag is set by hardware when the tamper X non-secure interrupt is raised.",
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
                    name: "itampmf",
                    description: Some(
                        "Internal tamper X non-secure interrupt masked flag. This flag is set by hardware when the internal tamper X non-secure interrupt is raised.",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "TAMP privilege mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt1priv",
                    description: Some(
                        "Monotonic counter 1 privilege protection",
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
                    name: "bkprwpriv",
                    description: Some(
                        "Backup registers zone 1 privilege protection",
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
                    name: "bkpwpriv",
                    description: Some(
                        "Backup registers zone 2 privilege protection",
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
                    name: "tamppriv",
                    description: Some(
                        "Tamper privilege protection (excluding backup registers). Note: Refer to for details on the read protection.",
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
            name: "Scr",
            extends: None,
            description: Some(
                "TAMP status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctampf",
                    description: Some(
                        "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the SR register.",
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
                    name: "citampf",
                    description: Some(
                        "Clear ITAMPx detection flag. Writing 1 in this bit clears the ITAMPxF bit in the SR register.",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "TAMP secure mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkprwsec",
                    description: Some(
                        "Backup registers read/write protection offset. Protection zone 1 is defined for backup registers from BKP0R to BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0:\tthe protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC[7:0] can be written only in privileged mode.",
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
                    name: "cnt1sec",
                    description: Some(
                        "Monotonic counter 1 secure protection",
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
                    name: "bkpwsec",
                    description: Some(
                        "Backup registers write protection offset. Protection zone 2 is defined for backup registers from BKPyR (y = BKPRWSEC, from 0 to 128) to BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSECBKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0:\tthe protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC[7:0] can be written only in privileged mode.",
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
                    name: "bhklock",
                    description: Some(
                        "Boot hardware key lock. This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.",
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
                    name: "tampsec",
                    description: Some(
                        "Tamper protection (excluding monotonic counters and backup registers). Note: Refer to for details on the read protection.",
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
            name: "Smisr",
            extends: None,
            description: Some(
                "TAMP secure masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampmf",
                    description: Some(
                        "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper X secure interrupt is raised.",
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
                    name: "itampmf",
                    description: Some(
                        "Internal tamper X secure interrupt masked flag. This flag is set by hardware when the internal tamper X secure interrupt is raised.",
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
                                len: 13,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "TAMP status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampf",
                    description: Some(
                        "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input.",
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
                    name: "itampf",
                    description: Some(
                        "Internal tamper X flag. This flag is set by hardware when a tamper detection event is detected on the internal tamper X.",
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
                                len: 13,
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
            name: "Atcksel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "RTCCLK is selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1st flip flop output)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2nd flip flop output)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7th flip flop output)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Tampflt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_FILTER",
                    description: Some(
                        "Tamper event is activated on edge of INx input transitions to the active level (no internal pull-up on INx input).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTER2",
                    description: Some(
                        "Tamper event is activated after 2 consecutive samples at the active level.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FILTER4",
                    description: Some(
                        "Tamper event is activated after 4 consecutive samples at the active level.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FILTER8",
                    description: Some(
                        "Tamper event is activated after 8 consecutive samples at the active level.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Tampfreq",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HZ_1",
                    description: Some(
                        "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HZ_2",
                    description: Some(
                        "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HZ_4",
                    description: Some(
                        "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HZ_8",
                    description: Some(
                        "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HZ_16",
                    description: Some(
                        "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HZ_32",
                    description: Some(
                        "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HZ_64",
                    description: Some(
                        "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "HZ_128",
                    description: Some(
                        "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Tampprch",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CYCLES1",
                    description: Some(
                        "1 RTCCLK cycle",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES2",
                    description: Some(
                        "2 RTCCLK cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES4",
                    description: Some(
                        "4 RTCCLK cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES8",
                    description: Some(
                        "8 RTCCLK cycles",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Tamptrg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FILTERED_LOW_OR_UNFILTERED_HIGH",
                    description: Some(
                        "If TAMPFLT 00 Tamper 2 input staying low triggers a tamper detection event.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTERED_HIGH_OR_UNFILTERED_LOW",
                    description: Some(
                        "If TAMPFLT 00 Tamper 2 input staying high triggers a tamper detection event.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
