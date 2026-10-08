
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rng",
            extends: None,
            description: Some(
                "Random number generator",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
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
                        "status register",
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
                    name: "dr",
                    description: Some(
                        "data register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "htcr",
                    description: Some(
                        "health test control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htcr",
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
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rngen",
                    description: Some(
                        "Random number generator enable",
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
                    name: "ie",
                    description: Some(
                        "Interrupt enable",
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
                    name: "ced",
                    description: Some(
                        "Clock error detection",
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
                    name: "rng_config3",
                    description: Some(
                        "RNG configuration 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "RngConfig3",
                    ),
                },
                Field {
                    name: "nistc",
                    description: Some(
                        "Non NIST compliant",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nistc",
                    ),
                },
                Field {
                    name: "rng_config2",
                    description: Some(
                        "RNG configuration 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "RngConfig2",
                    ),
                },
                Field {
                    name: "clkdiv",
                    description: Some(
                        "Clock divider factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Clkdiv",
                    ),
                },
                Field {
                    name: "rng_config1",
                    description: Some(
                        "RNG configuration 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "RngConfig1",
                    ),
                },
                Field {
                    name: "condrst",
                    description: Some(
                        "Conditioning soft reset",
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
                    name: "configlock",
                    description: Some(
                        "Config Lock",
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
            name: "Htcr",
            extends: None,
            description: Some(
                "Health test control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htcfg",
                    description: Some(
                        "Health test configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Htcfg",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "drdy",
                    description: Some(
                        "Data ready",
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
                    name: "cecs",
                    description: Some(
                        "Clock error current status",
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
                    name: "secs",
                    description: Some(
                        "Seed error current status",
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
                    name: "ceis",
                    description: Some(
                        "Clock error interrupt status",
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
                    name: "seis",
                    description: Some(
                        "Seed error interrupt status",
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
    ],
    enums: &[
        Enum {
            name: "Clkdiv",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NO_DIV",
                    description: Some(
                        "Internal RNG clock after divider is similar to incoming RNG clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV_2_1",
                    description: Some(
                        "Divide RNG clock by 2^1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV_2_2",
                    description: Some(
                        "Divide RNG clock by 2^2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV_2_3",
                    description: Some(
                        "Divide RNG clock by 2^3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV_2_4",
                    description: Some(
                        "Divide RNG clock by 2^4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV_2_5",
                    description: Some(
                        "Divide RNG clock by 2^5",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV_2_6",
                    description: Some(
                        "Divide RNG clock by 2^6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV_2_7",
                    description: Some(
                        "Divide RNG clock by 2^7",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV_2_8",
                    description: Some(
                        "Divide RNG clock by 2^8",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV_2_9",
                    description: Some(
                        "Divide RNG clock by 2^9",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV_2_10",
                    description: Some(
                        "Divide RNG clock by 2^10",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV_2_11",
                    description: Some(
                        "Divide RNG clock by 2^11",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV_2_12",
                    description: Some(
                        "Divide RNG clock by 2^12",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV_2_13",
                    description: Some(
                        "Divide RNG clock by 2^13",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV_2_14",
                    description: Some(
                        "Divide RNG clock by 2^14",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV_2_15",
                    description: Some(
                        "Divide RNG clock by 2^15",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Htcfg",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "RECOMMENDED",
                    description: Some(
                        "Recommended value for RNG certification (0x0000_AA74)",
                    ),
                    value: 43636,
                },
                EnumVariant {
                    name: "MAGIC",
                    description: Some(
                        "Magic number to be written before any write (0x1759_0ABC)",
                    ),
                    value: 391711420,
                },
            ],
        },
        Enum {
            name: "Nistc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DEFAULT",
                    description: Some(
                        "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CUSTOM",
                    description: Some(
                        "Custom values for NIST compliant RNG",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RngConfig1",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "CONFIG_A",
                    description: Some(
                        "Recommended value for config A (NIST certifiable)",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "CONFIG_B",
                    description: Some(
                        "Recommended value for config B (not NIST certifiable)",
                    ),
                    value: 24,
                },
            ],
        },
        Enum {
            name: "RngConfig2",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CONFIG_A_B",
                    description: Some(
                        "Recommended value for config A and B",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "RngConfig3",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "CONFIG_B",
                    description: Some(
                        "Recommended value for config B (not NIST certifiable)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONFIG_A",
                    description: Some(
                        "Recommended value for config A (NIST certifiable)",
                    ),
                    value: 13,
                },
            ],
        },
    ],
};
