
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opamp",
            extends: None,
            description: Some(
                "Operational amplifier",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "Control/status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otr",
                    description: Some(
                        "Offset trimming register in normal mode",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpotr",
                    description: Some(
                        "Offset trimming register in low-power mode",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpotr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "Control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
                    description: Some(
                        "Enable",
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
                    name: "opalpm",
                    description: Some(
                        "Low-power mode enable. The operational amplifier must be disabled to change this configuration.",
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
                    name: "opamode",
                    description: Some(
                        "PGA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Opamode",
                    ),
                },
                Field {
                    name: "pga_gain",
                    description: Some(
                        "Gain in PGA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PgaGain",
                    ),
                },
                Field {
                    name: "vm_sel",
                    description: Some(
                        "Inverting input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "VmSel",
                    ),
                },
                Field {
                    name: "vp_sel",
                    description: Some(
                        "Non inverted input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "VpSel",
                    ),
                },
                Field {
                    name: "calon",
                    description: Some(
                        "Calibration mode enable",
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
                    name: "calsel",
                    description: Some(
                        "Calibration selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calsel",
                    ),
                },
                Field {
                    name: "usertrim",
                    description: Some(
                        "User trimming enable",
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
                    name: "calout",
                    description: Some(
                        "Calibration output",
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
                    name: "opa_range",
                    description: Some(
                        "Power supply range for stability",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OpaRange",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Lpotr",
            extends: None,
            description: Some(
                "Offset trimming register in low-power mode",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimlpoffsetn",
                    description: Some(
                        "Offset trimming value (NMOS)",
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
                    name: "trimlpoffsetp",
                    description: Some(
                        "Offset trimming value (PMOS)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Otr",
            extends: None,
            description: Some(
                "Offset trimming register in normal mode",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimoffsetn",
                    description: Some(
                        "Offset trimming value (NMOS)",
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
                    name: "trimoffsetp",
                    description: Some(
                        "Offset trimming value (PMOS)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Calsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NMOS",
                    description: Some(
                        "NMOS calibration, 0.2 V applied to OPAMP inputs during calibration",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PMOS",
                    description: Some(
                        "PMOS calibration, VDDA - 0.2 V applied to OPAMP inputs during calibration",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OpaRange",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low range (VDDA < 2.4 V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High range (VDDA > 2.4 V)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Opamode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "Internal PGA disable",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLE2",
                    description: Some(
                        "Internal PGA disable (duplicate)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "Internal PGA enable, gain programmed in PGA_GAIN",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FOLLOWER",
                    description: Some(
                        "Internal follower",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GAIN2",
                    description: Some(
                        "Gain 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some(
                        "Gain 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some(
                        "Gain 8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some(
                        "Gain 16",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VmSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GPIO",
                    description: Some(
                        "GPIO connected to VINM (valid also in PGA mode for filtering)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_LEAKAGE",
                    description: Some(
                        "Low leakage inputs connected (only available in certain packages)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NOT_CONNECTED",
                    description: Some(
                        "VINM not externally connected, valid only in PGA mode",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "GPIO",
                    description: Some(
                        "GPIO connected to VINP",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DAC",
                    description: Some(
                        "DAC connected to VINP",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
