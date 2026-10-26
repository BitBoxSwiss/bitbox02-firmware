
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
                        "control register 1",
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
                        "control register 2",
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
                            access: Access::Read,
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
                        "TAMP masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                    name: "scr",
                    description: Some(
                        "TAMP status clear register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                        "monotonic counter register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Countr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bkpr",
                    description: Some(
                        "TAMP backup register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
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
                        "BKP",
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
                "monotonic counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "count",
                    description: Some(
                        "COUNT",
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
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampe",
                    description: Some(
                        "Tamper detection on IN X enable",
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
                                len: 3,
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
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tampmsk",
                    description: Some(
                        "Tamper X mask. The tamper X interrupt must not be enabled when TAMPMSK is set.",
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
                    enumm: Some(
                        "Tampmsk",
                    ),
                },
                Field {
                    name: "bkerase",
                    description: Some(
                        "Backup registers erase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bkerase",
                    ),
                },
                Field {
                    name: "tamptrg",
                    description: Some(
                        "Active level for tamper X input",
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
                                len: 3,
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
                                len: 3,
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
            name: "Misr",
            extends: None,
            description: Some(
                "TAMP masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampmf",
                    description: Some(
                        "Tamper X interrupt masked flag",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampmf",
                    description: Some(
                        "Internal tamper X interrupt masked flag",
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
                        "Clear tamper X detection flag",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "citampf",
                    description: Some(
                        "Clear internal tamper X detection flag",
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
                        "Tamper X detection flag",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampf",
                    description: Some(
                        "Internal tamper X detection flag",
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
    ],
    enums: &[
        Enum {
            name: "Bkerase",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset backup registers",
                    ),
                    value: 1,
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
                        "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)\"",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTER2",
                    description: Some(
                        "Tamper event is activated after 2 consecutive samples at the active level\"",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FILTER4",
                    description: Some(
                        "Tamper event is activated after 4 consecutive samples at the active level\"",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FILTER8",
                    description: Some(
                        "Tamper event is activated after 8 consecutive samples at the active level\"",
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
            name: "Tampmsk",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET_BY_SOFTWARE",
                    description: Some(
                        "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RESET_BY_HARDWARE",
                    description: Some(
                        "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set",
                    ),
                    value: 1,
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
                        "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTERED_HIGH_OR_UNFILTERED_LOW",
                    description: Some(
                        "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
