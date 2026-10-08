
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Opamp",
        extends: None,
        description: Some("Operational amplifier"),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("Control/status register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "tcmr",
                description: Some("Control/status register"),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Tcmr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Control/status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
                    description: Some("Enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "force_vp",
                    description: Some("Force internal reference on VP (reserved for test)"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vp_sel",
                    description: Some("Non-inverting input selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VpSel"),
                },
                Field {
                    name: "usertrim",
                    description: Some("User trimming enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vm_sel",
                    description: Some("Inverting input selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VmSel"),
                },
                Field {
                    name: "opahsm",
                    description: Some("High-speed mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opaintoen",
                    description: Some("Internal output enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calon",
                    description: Some("Calibration mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calsel",
                    description: Some("Calibration selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Calsel"),
                },
                Field {
                    name: "pga_gain",
                    description: Some("Gain in PGA mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 5,
                    array: None,
                    enumm: Some("PgaGain"),
                },
                Field {
                    name: "trimoffsetp",
                    description: Some("Offset trimming value (PMOS)"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimoffsetn",
                    description: Some("Offset trimming value (NMOS)"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calout",
                    description: Some("OPAMP ouput status flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("LOCK"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tcmr",
            extends: None,
            description: Some("OPAMP timer controlled mode register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vms_sel",
                    description: Some("Inverting input secondary selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vps_sel",
                    description: Some("Non-inverting input secondary selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VpsSel"),
                },
                Field {
                    name: "t1cm_en",
                    description: Some("TIM1 controlled mux mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t8cm_en",
                    description: Some("TIM8 controlled mux mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t20cm_en",
                    description: Some("TIM20 controlled mux mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("Configure this register as read-only"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
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
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PERCENT3_3",
                    description: Some("VREFOPAMP = 3.3% VDDA"),
                    value: 0,
                },
                EnumVariant {
                    name: "PERCENT10",
                    description: Some("VREFOPAMP = 10% VDDA"),
                    value: 1,
                },
                EnumVariant {
                    name: "PERCENT50",
                    description: Some("VREFOPAMP = 50% VDDA"),
                    value: 2,
                },
                EnumVariant {
                    name: "PERCENT90",
                    description: Some("VREFOPAMP = 90% VDDA"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "GAIN2",
                    description: Some("Gain 2"),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some("Gain 4"),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some("Gain 8"),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some("Gain 16"),
                    value: 3,
                },
                EnumVariant {
                    name: "GAIN32",
                    description: Some("Gain 32"),
                    value: 4,
                },
                EnumVariant {
                    name: "GAIN64",
                    description: Some("Gain 64"),
                    value: 5,
                },
                EnumVariant {
                    name: "GAIN2_INPUT_VINM0",
                    description: Some("Gain 2, input/bias connected to VINM0 or inverting gain"),
                    value: 8,
                },
                EnumVariant {
                    name: "GAIN4_INPUT_VINM0",
                    description: Some("Gain 4, input/bias connected to VINM0 or inverting gain"),
                    value: 9,
                },
                EnumVariant {
                    name: "GAIN8_INPUT_VINM0",
                    description: Some("Gain 8, input/bias connected to VINM0 or inverting gain"),
                    value: 10,
                },
                EnumVariant {
                    name: "GAIN16_INPUT_VINM0",
                    description: Some("Gain 16, input/bias connected to VINM0 or inverting gain"),
                    value: 11,
                },
                EnumVariant {
                    name: "GAIN32_INPUT_VINM0",
                    description: Some("Gain 32, input/bias connected to VINM0 or inverting gain"),
                    value: 12,
                },
                EnumVariant {
                    name: "GAIN64_INPUT_VINM0",
                    description: Some("Gain 64, input/bias connected to VINM0 or inverting gain"),
                    value: 13,
                },
                EnumVariant {
                    name: "GAIN2_FILTERING_VINM0",
                    description: Some("Gain 2, with filtering on VINM0"),
                    value: 16,
                },
                EnumVariant {
                    name: "GAIN4_FILTERING_VINM0",
                    description: Some("Gain 4, with filtering on VINM0"),
                    value: 17,
                },
                EnumVariant {
                    name: "GAIN8_FILTERING_VINM0",
                    description: Some("Gain 8, with filtering on VINM0"),
                    value: 18,
                },
                EnumVariant {
                    name: "GAIN16_FILTERING_VINM0",
                    description: Some("Gain 16, with filtering on VINM0"),
                    value: 19,
                },
                EnumVariant {
                    name: "GAIN32_FILTERING_VINM0",
                    description: Some("Gain 32, with filtering on VINM0"),
                    value: 20,
                },
                EnumVariant {
                    name: "GAIN64_FILTERING_VINM0",
                    description: Some("Gain 64, with filtering on VINM0"),
                    value: 21,
                },
                EnumVariant {
                    name: "GAIN2_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "GAIN4_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "GAIN8_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "GAIN16_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "GAIN32_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "GAIN64_INPUT_VINM0FILTERING_VINM1",
                    description: Some(
                        "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 29,
                },
            ],
        },
        Enum {
            name: "VmSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VINM0",
                    description: Some("VINM0 connected to VINM input"),
                    value: 0,
                },
                EnumVariant {
                    name: "VINM1",
                    description: Some("VINM1 connected to VINM input"),
                    value: 1,
                },
                EnumVariant {
                    name: "PGA",
                    description: Some("Feedback resistor connected to VINM (PGA mode)"),
                    value: 2,
                },
                EnumVariant {
                    name: "OUTPUT",
                    description: Some("OpAmp output connected to VINM (Follower mode)"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VINP0",
                    description: Some("VINP0 connected to VINP input"),
                    value: 0,
                },
                EnumVariant {
                    name: "VINP1",
                    description: Some("VINP1 connected to VINP input"),
                    value: 1,
                },
                EnumVariant {
                    name: "VINP2",
                    description: Some("VINP2 connected to VINP input"),
                    value: 2,
                },
                EnumVariant {
                    name: "DAC3_CH1",
                    description: Some("DAC3_CH1 connected to VINP input"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VpsSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VINP0",
                    description: Some("VINP0 connected to VINP input"),
                    value: 0,
                },
                EnumVariant {
                    name: "VINP1",
                    description: Some("VINP1 connected to VINP input"),
                    value: 1,
                },
                EnumVariant {
                    name: "VINP2",
                    description: Some("VINP2 connected to VINP input"),
                    value: 2,
                },
                EnumVariant {
                    name: "DAC3_CH1",
                    description: Some("DAC3_CH1 connected to VINP input"),
                    value: 3,
                },
            ],
        },
    ],
};
