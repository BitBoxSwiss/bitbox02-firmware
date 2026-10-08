
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "AdcCommon",
            extends: None,
            description: Some(
                "Analog-to-Digital Converter.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "ADC common status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "ADC_CCR system control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                BlockItem {
                    name: "cdr",
                    description: Some(
                        "ADC common regular data register for dual mode.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdr2",
                    description: Some(
                        "ADC common regular data register for 32-bit dual mode.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdr2",
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
                "ADC_CCR system control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dual",
                    description: Some(
                        "Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Dual",
                    ),
                },
                Field {
                    name: "delay",
                    description: Some(
                        "Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "damdf",
                    description: Some(
                        "Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register CDR. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Damdf",
                    ),
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
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
                    name: "vsenseen",
                    description: Some(
                        "Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
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
                    name: "vbaten",
                    description: Some(
                        "VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
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
            name: "Cdr",
            extends: None,
            description: Some(
                "ADC common regular data register for dual mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata_mst",
                    description: Some(
                        "Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF[1:0]\u{a0}=\u{a0}11 mode, bits 15:8 contains SLV_ADC_DR[7:0], bits 7:0 contains MST_ADC_DR[7:0].",
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
                Field {
                    name: "rdata_slv",
                    description: Some(
                        "Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)).",
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
        FieldSet {
            name: "Cdr2",
            extends: None,
            description: Some(
                "ADC common regular data register for 32-bit dual mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata_alt",
                    description: Some(
                        "Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT).",
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
            name: "Csr",
            extends: None,
            description: Some(
                "ADC common status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy_mst",
                    description: Some(
                        "Master ADC ready This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register.",
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
                    name: "eosmp_mst",
                    description: Some(
                        "End of Sampling phase flag of the master ADC This bit is a copy of the EOSMP bit in the corresponding ADC_ISR register.",
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
                    name: "eoc_mst",
                    description: Some(
                        "End of regular conversion of the master ADC This bit is a copy of the EOC bit in the corresponding ADC_ISR register.",
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
                    name: "eos_mst",
                    description: Some(
                        "End of regular sequence flag of the master ADC This bit is a copy of the EOS bit in the corresponding ADC_ISR register.",
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
                    name: "ovr_mst",
                    description: Some(
                        "Overrun flag of the master ADC This bit is a copy of the OVR bit in the corresponding ADC_ISR register.",
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
                    name: "jeoc_mst",
                    description: Some(
                        "End of injected conversion flag of the master ADC This bit is a copy of the JEOC bit in the corresponding ADC_ISR register.",
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
                    name: "jeos_mst",
                    description: Some(
                        "End of injected sequence flag of the master ADC This bit is a copy of the JEOS bit in the corresponding ADC_ISR register.",
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
                    name: "awd_mst",
                    description: Some(
                        "Analog watchdog flags of the master ADC This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
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
                    name: "jqovf_mst",
                    description: Some(
                        "Injected context queue overflow flag of the master ADC.",
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
                    name: "ldordy_mst",
                    description: Some(
                        "ADC voltage regulator ready flag of the master ADC This bit is a copy of the LDORDY bit of the corresponding ADC_ISR register.",
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
                    name: "adrdy_slv",
                    description: Some(
                        "Slave ADC ready This bit is a copy of the ADRDY bit in the corresponding ADCx+1_ISR register.",
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
                    name: "eosmp_slv",
                    description: Some(
                        "End of Sampling phase flag of the slave ADC This bit is a copy of the EOSMP2 bit in the corresponding ADCx+1_ISR register.",
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
                    name: "eoc_slv",
                    description: Some(
                        "End of regular conversion of the slave ADC This bit is a copy of the EOC bit in the corresponding ADCx+1_ISR register.",
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
                    name: "eos_slv",
                    description: Some(
                        "End of regular sequence flag of the slave ADC This bit is a copy of the EOS bit in the corresponding ADCx+1_ISR register.",
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
                    name: "ovr_slv",
                    description: Some(
                        "Overrun flag of the slave ADC This bit is a copy of the OVR bit in the corresponding ADCx+1_ISR register.",
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
                    name: "jeoc_slv",
                    description: Some(
                        "End of injected conversion flag of the slave ADC This bit is a copy of the JEOC bit in the corresponding ADCx+1_ISR register.",
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
                    name: "jeos_slv",
                    description: Some(
                        "End of injected sequence flag of the slave ADC This bit is a copy of the JEOS bit in the corresponding ADCx+1_ISR register.",
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
                    name: "awd_slv",
                    description: Some(
                        "Analog watchdog 1 flag of the slave ADC This bit is a copy of the AWD1 bit in the corresponding ADCx+1_ISR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
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
                    name: "jqovf_slv",
                    description: Some(
                        "Injected context queue overflow flag of the slave ADC.",
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
                    name: "ldordy_slv",
                    description: Some(
                        "ADC voltage regulator ready flag of the slave ADC This bit is a copy of the LDORDY bit of the corresponding ADCx+1_ISR register.",
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
    ],
    enums: &[
        Enum {
            name: "Damdf",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_PACK",
                    description: Some(
                        "Without data packing, CDR/CDR2 not used",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORMAT32TO10",
                    description: Some(
                        "CDR formatted for 32-bit down to 10-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FORMAT8",
                    description: Some(
                        "CDR formatted for 8-bit resolution",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dual",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some(
                        "Independent mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DUAL_RJ",
                    description: Some(
                        "Dual, combined regular simultaneous + injected simultaneous mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DUAL_RA",
                    description: Some(
                        "Dual, combined regular simultaneous + alternate trigger mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DUAL_IJ",
                    description: Some(
                        "Dual, combined interleaved mode + injected simultaneous mode",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DUAL_J",
                    description: Some(
                        "Dual, injected simultaneous mode only",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DUAL_R",
                    description: Some(
                        "Dual, regular simultaneous mode only",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DUAL_I",
                    description: Some(
                        "Dual, interleaved mode only",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DUAL_A",
                    description: Some(
                        "Dual, alternate trigger mode only",
                    ),
                    value: 9,
                },
            ],
        },
    ],
};
