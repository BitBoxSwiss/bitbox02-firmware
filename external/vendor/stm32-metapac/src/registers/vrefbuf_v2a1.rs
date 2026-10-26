
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Vrefbuf",
            extends: None,
            description: Some(
                "Voltage reference buffer.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "VREFBUF control and status register.",
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
                    name: "ccr",
                    description: Some(
                        "VREFBUF calibration control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "VREFBUF calibration control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trim",
                    description: Some(
                        "Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM[5:0] is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM[5:0] is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM[5:0]: User can modify the TRIM[5:0] with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "VREFBUF control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "envr",
                    description: Some(
                        "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.",
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
                    name: "hiz",
                    description: Some(
                        "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hiz",
                    ),
                },
                Field {
                    name: "vrr",
                    description: Some(
                        "Voltage reference buffer ready.",
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
                    name: "vrs",
                    description: Some(
                        "Voltage reference scale These bits select the value generated by the voltage reference buffer. VRS = 000: VREFBUF0 voltage selected. VRS = 001: VREFBUF1 voltage selected. VRS = 010: VREFBUF2 voltage selected. VRS = 011: VREFBUF3 voltage selected. Others: Reserved Note: Refer to the product datasheet for each VREFBUFx voltage setting value. The software can program this bitfield only when the VREFBUF is disabled (ENVR=0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Vrs",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Hiz",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "VREF+ pin is internally connected to the voltage reference buffer output.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH_Z",
                    description: Some(
                        "VREF+ pin is high impedance.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vrs",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "VREF0",
                    description: Some(
                        "Voltage reference set to around 1.5 V.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "VREF1",
                    description: Some(
                        "Voltage reference set to around 1.8 V.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "VREF2",
                    description: Some(
                        "Voltage reference set to around 2.048 V.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VREF3",
                    description: Some(
                        "Voltage reference set to around 2.5 V (ADC, DAC are not compatible with this setting).",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
