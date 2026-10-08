
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "ADC address block description.",
            ),
            items: &[
                BlockItem {
                    name: "version_id",
                    description: Some(
                        "VERSION_ID register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "VersionId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "conf",
                    description: Some(
                        "CONF register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Conf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "CTRL register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ocm_ctrl",
                    description: Some(
                        "Occasionnal mode control register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OcmCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pga_conf",
                    description: Some(
                        "PGA configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PgaConf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "switch",
                    description: Some(
                        "SWITCH register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Switch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "df_conf",
                    description: Some(
                        "Decimation filter configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfConf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ds_conf",
                    description: Some(
                        "DS_CONF register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DsConf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_1",
                    description: Some(
                        "SEQ_1 register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seq1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_2",
                    description: Some(
                        "SEQ_2 register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seq2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_1",
                    description: Some(
                        "COMP_1 register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_2",
                    description: Some(
                        "COMP_2 register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_3",
                    description: Some(
                        "COMP_3 register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comp3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_4",
                    description: Some(
                        "COMP_4 register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comp4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_sel",
                    description: Some(
                        "COMP_SEL register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CompSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd_th",
                    description: Some(
                        "WD_TH register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdTh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd_conf",
                    description: Some(
                        "WD_CONF register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdConf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ds_dataout",
                    description: Some(
                        "DS_DATAOUT register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DsDataout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "df_dataout",
                    description: Some(
                        "Decimation filter Data output register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfDataout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_status",
                    description: Some(
                        "IRQ_STATUS register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_enable",
                    description: Some(
                        "IRQ_ENABLE register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer_conf",
                    description: Some(
                        "Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TimerConf",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Comp1",
            extends: None,
            description: Some(
                "COMP_1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gain1",
                    description: Some(
                        "GAIN1[11:0]: first calibration point: gain AUXADC_GAIN_1V2[11:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset1",
                    description: Some(
                        "OFFSET1[7:0]: first calibration point: offset compensation[7:0] with sign.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Comp2",
            extends: None,
            description: Some(
                "COMP_2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gain2",
                    description: Some(
                        "GAIN2[11:0]: second calibration point: gain AUXADC_GAIN_1V2[11:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset2",
                    description: Some(
                        "OFFSET2[7:0]: second calibration point: offset compensation[7:0] with sign.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Comp3",
            extends: None,
            description: Some(
                "COMP_3 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gain3",
                    description: Some(
                        "GAIN3[11:0]: third calibration point: gain AUXADC_GAIN_1V2[11:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset3",
                    description: Some(
                        "OFFSET3[7:0]: third calibration point: offset compensation[7:0] with sign.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Comp4",
            extends: None,
            description: Some(
                "COMP_4 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gain4",
                    description: Some(
                        "GAIN4[11:0]: fourth calibration point: gain AUXADC_GAIN_1V2[11:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset4",
                    description: Some(
                        "OFFSET4[7:0]: fourth calibration point: offset compensation[7:0] with sign.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CompSel",
            extends: None,
            description: Some(
                "COMP_SEL register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gain_offset0",
                    description: Some(
                        "gain / offset used in ADC single negative mode with Vinput range = 1.2V.",
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
                    name: "offset_gain0",
                    description: Some(
                        "OFFSET_GAIN0[1:0]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
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
                    name: "gain_offset1",
                    description: Some(
                        "gain / offset used in ADC single positive mode with Vinput range = 1.2V.",
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
                    name: "offset_gain1",
                    description: Some(
                        "OFFSET_GAIN1[1:0]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
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
                    name: "gain_offset2",
                    description: Some(
                        "gain / offset used in ADC differential mode with Vinput range = 1.2V.",
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
                Field {
                    name: "offset_gain2",
                    description: Some(
                        "OFFSET_GAIN2[1:0]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
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
                Field {
                    name: "gain_offset3",
                    description: Some(
                        "gain / offset used in ADC single negative mode with Vinput range = 2.4V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain3",
                    description: Some(
                        "OFFSET_GAIN3[1:0]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gain_offset4",
                    description: Some(
                        "gain / offset used in ADC single positive mode with Vinput range = 2.4V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain4",
                    description: Some(
                        "OFFSET_GAIN4[1:0]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gain_offset5",
                    description: Some(
                        "gain / offset used in ADC differential mode with Vinput range = 2.4V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain5",
                    description: Some(
                        "OFFSET_GAIN5[1:0]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gain_offset6",
                    description: Some(
                        "gain / offset used in ADC single negative mode with Vinput range = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain6",
                    description: Some(
                        "OFFSET_GAIN6[1:0]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gain_offset7",
                    description: Some(
                        "gain / offset used in ADC single positive mode with Vinput range = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain7",
                    description: Some(
                        "OFFSET_GAIN7[1:0]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gain_offset8",
                    description: Some(
                        "gain / offset used in ADC differential mode with Vinput range = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset_gain8",
                    description: Some(
                        "OFFSET_GAIN8[1:0]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Conf",
            extends: None,
            description: Some(
                "CONF register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cont",
                    description: Some(
                        "CONT: regular sequence runs continuously when ADC mode is enabled: 0: enable the single conversion: when the sequence is over, the conversion stops 1: enable the continuous conversion: when the sequence is over, the sequence starts again until the software sets the CTRL.STOP_OP_MODE bit.",
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
                    name: "sequence",
                    description: Some(
                        "SEQUENCE: enable the sequence mode (active by default): 0: sequence mode is disabled, only SEQ0 is selected 1: sequence mode is enabled, conversions from SEQ0 to SEQx with x=SEQ_LEN Note: clearing this bit is equivalent to SEQUENCE=1 and SEQ_LEN=0000. Ideally, this bit can be kept high as redundant with keeping high and setting SEQ_LEN=0000.",
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
                    name: "seq_len",
                    description: Some(
                        "SEQ_LEN[3:0]: number of conversions in a regular sequence: 0000: 1 conversion, starting from SEQ0 0001: 2 conversions, starting from SEQ0 ... 1111: 16 conversions, starting from SEQ0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smps_synchro_ena",
                    description: Some(
                        "SMPS_SYNCHRO_ENA: synchronize the ADC start conversion with a pulse generated by the SMPS: 0: SMPS synchronization is disabled for all ADC clock frequencies 1: SMPS synchronization is enabled (only when ADC clock is 8 MHz or 16 MHz) Note: SMPS_SYNCHRO_ENA must be 0 when the ADC analog clock is 32 MHz or when PWRC_CR5.NOSMPS = 1.",
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
                    name: "op_mode",
                    description: Some(
                        "ADC mode selection (= data path selection).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sample_rate_lsb",
                    description: Some(
                        "SAMPLE_RATE_LSB: Sample Rate LSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description. When this field is set to a value different than 0, SMPS synchronization is not feasible. This value is hidden to the user.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sample_rate",
                    description: Some(
                        "SAMPLE_RATE[1:0]: conversion rate of ADC (F_ADC): F_ADC = F_ADC_CLK/(16 + 16*SAMPLE_RATE_MSB + 4*SAMPLE_RATE + SAMPLE_RATE_LSB),where F_ADC_CLK is the analog ADC clock frequency. By default F_ADC_CLK is 16MHz frequency.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma_ds_ena",
                    description: Some(
                        "DMA_DS_EN: enable the DMA mode for the Down Sampler data path: 0: DMA mode is disabled 1: DMA mode is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma_df_ena",
                    description: Some(
                        "enable DMA mode for Decimation Filter data path.",
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
                    name: "ovr_ds_cfg",
                    description: Some(
                        "OVR_DS_CFG: Down Sampler overrun configuration: 0: the previous data is kept, the new one is lost 1: the previous data is lost, the new one is kept.",
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
                    name: "ovr_df_cfg",
                    description: Some(
                        "decimation overrun configuration.",
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
                    name: "bit_invert_sn",
                    description: Some(
                        "BIT_INVERT_SN: invert bit to bit the ADC data output (1's complement) when a single negative input is connected to the ADC: 0: no inversion (default) 1: enable the inversion.",
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
                    name: "bit_invert_diff",
                    description: Some(
                        "BIT_INVERT_DIFF: invert bit to bit the ADC data output (1's complement) when a differential input is connected to the ADC: 0: no inversion (default) 1: enable the inversion.",
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
                    name: "adc_cont_1v2",
                    description: Some(
                        "ADC_CONT_1V2: select the input sampling method: 0: sampling only at conversion start (default) 1: sampling starts at the end of conversion.",
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
                    name: "vbias_prech_force",
                    description: Some(
                        "possibility to keep the VBIAS_PRECH enabled to deactivate the filter.",
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
                    name: "sample_rate_msb",
                    description: Some(
                        "SAMPLE_RATE_MSB: Sample Rate MSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "CTRL register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_on_off",
                    description: Some(
                        "ADC_ON_OFF: 0: power off the ADC 1: power on the ADC.",
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
                    name: "start_con",
                    description: Some(
                        "generate a start pulse to initiate an ADC conversion.",
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
                    name: "start_conv",
                    description: Some(
                        "START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware.",
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
                    name: "stop_op_mod",
                    description: Some(
                        "stop the on-going OP_MODE (ADC mode, Analog audio mode, Full.",
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
                    name: "stop_op_mode",
                    description: Some(
                        "STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit.",
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
                    name: "dig_aud_mode",
                    description: Some(
                        "enable the digital audio mode (the data path uses. the decimation filter).",
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
                    name: "test_mode",
                    description: Some(
                        "TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration).",
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
                    name: "adc_ldo_ena",
                    description: Some(
                        "ADC_LDO_ENA: enable the LDO associated to the ADC block: 0: disable the ADC LDO 1: enable the ADC LDO.",
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
            ],
        },
        FieldSet {
            name: "DfConf",
            extends: None,
            description: Some(
                "Decimation filter configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "df_cic_dec_factor",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "df_cic_dhf",
                    description: Some(
                        "CIC filter decimator half factor.",
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
                    name: "df_itp1p2",
                    description: Some(
                        "1.2 fractional interpolator enable.",
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
                    name: "df_i_u2s",
                    description: Some(
                        "select signed/unsigned format for input.",
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
                    name: "df_o_s2u",
                    description: Some(
                        "select signed/unsigned format for data output.",
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
                    name: "pdm_rate",
                    description: Some(
                        "select the PDM clock rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "df_microl_rn",
                    description: Some(
                        "left/right channel selection on digital microphone.",
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
                    name: "df_hpf_en",
                    description: Some(
                        "high pass filter enable.",
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
                    name: "df_half_d_en",
                    description: Some(
                        "half dynamic enable.",
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
            ],
        },
        FieldSet {
            name: "DfDataout",
            extends: None,
            description: Some(
                "Decimation filter Data output register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "df_data",
                    description: Some(
                        "contain the converted data at the output of the. decimation filter.",
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
            name: "DsConf",
            extends: None,
            description: Some(
                "DS_CONF register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ds_ratio",
                    description: Some(
                        "DS_RATIO[2:0]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128.",
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
                    name: "ds_width",
                    description: Some(
                        "DS_WIDTH[2:0]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DsDataout",
            extends: None,
            description: Some(
                "DS_DATAOUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ds_data",
                    description: Some(
                        "DS_DATA[15:0]: contain the converted data at the output of the Down Sampler.",
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
            name: "IrqEnable",
            extends: None,
            description: Some(
                "IRQ_ENABLE register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eoc_irq",
                    description: Some(
                        "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "eoc_irq_ena",
                    description: Some(
                        "(Used in test mode only): End of ADC conversion interrupt enable.",
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
                    name: "eods_irq",
                    description: Some(
                        "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "eods_irq_ena",
                    description: Some(
                        "End of conversion interrupt enable for the Down Sampler output.",
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
                    name: "eodf_irq_ena",
                    description: Some(
                        "End of conversion interrupt enable for the decimation filter output.",
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
                    name: "eos_irq",
                    description: Some(
                        "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "eos_irq_ena",
                    description: Some(
                        "End of regular sequence interrupt enable.",
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
                    name: "awd_irq",
                    description: Some(
                        "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "awd_irq_ena",
                    description: Some(
                        "analog watchdog interrupt enable.",
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
                    name: "ovr_ds_irq",
                    description: Some(
                        "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "ovr_ds_irq_ena",
                    description: Some(
                        "Down Sampler overrun interrupt enable.",
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
                    name: "ovr_df_irq_ena",
                    description: Some(
                        "decimation filter overrun interrupt enable.",
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
                    name: "df_ovrfl_irq_ena",
                    description: Some(
                        "decimation filter saturation interrupt enable.",
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
            name: "IrqStatus",
            extends: None,
            description: Some(
                "IRQ_STATUS register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eoc_irq",
                    description: Some(
                        "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "eods_irq",
                    description: Some(
                        "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "eodf_irq",
                    description: Some(
                        "set when the decimation filter conversion is completed.",
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
                    name: "eos_irq",
                    description: Some(
                        "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "awd_irq",
                    description: Some(
                        "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "ovr_ds_irq",
                    description: Some(
                        "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt.",
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
                    name: "ovr_df_irq",
                    description: Some(
                        "set to indicate a decimation filter overrun (a data is lost).",
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
                    name: "df_ovrfl_irq",
                    description: Some(
                        "set to indicate the decimation filter is saturated.",
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
            name: "OcmCtrl",
            extends: None,
            description: Some(
                "Occasionnal mode control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ocm_src",
                    description: Some(
                        "select the occasional conversion source.",
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
                    name: "ocm_ena",
                    description: Some(
                        "start occasional conversion in analog audio and full. modes.",
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
            ],
        },
        FieldSet {
            name: "PgaConf",
            extends: None,
            description: Some(
                "PGA configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pga_gain",
                    description: Some(
                        "from 6 to 30 dB.",
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
                    name: "pga_bias",
                    description: Some(
                        "set the microphone bias voltage.",
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
            ],
        },
        FieldSet {
            name: "Seq1",
            extends: None,
            description: Some(
                "SEQ_1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seq0",
                    description: Some(
                        "SEQ0[3:0]: channel number code for first conversion of the sequence 0000: VINM[0] to ADC single negative input 0001: VINM[1] to ADC single negative input 0010: VINM[2] to ADC single negative input 0011: VINM[3] to ADC single negative input 0100: VINP[0] to ADC single positive input 0101: VINP[1] to ADC single positive input 0110: VINP[2] to ADC single positive input 0111: VINP[3] to ADC single positive input 1000: VINP[0]-VINM[0] to ADC differential input 1001: VINP[1]-VINM[1] to ADC differential input 1010: VINP[2]-VINM[2] to ADC differential input 1011: VINP[3]-VINM[3] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved.",
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
                    name: "seq1",
                    description: Some(
                        "SEQ1[3:0]: channel number code for second conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq2",
                    description: Some(
                        "SEQ2[3:0]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail.",
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
                    name: "seq3",
                    description: Some(
                        "SEQ3[3:0]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq4",
                    description: Some(
                        "SEQ4[3:0]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq5",
                    description: Some(
                        "SEQ5[3:0]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail.",
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
                    name: "seq6",
                    description: Some(
                        "SEQ6[3:0]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq7",
                    description: Some(
                        "SEQ7[3:0]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seq2",
            extends: None,
            description: Some(
                "SEQ_2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seq8",
                    description: Some(
                        "SEQ8[3:0]: channel number code for 9th conversion of the sequence 0000: VINM[0] to ADC single negative input 0001: VINM[1] to ADC single negative input 0010: VINM[2] to ADC single negative input 0011: VINM[3] to ADC single negative input 0100: VINP[0] to ADC single positive input 0101: VINP[1] to ADC single positive input 0110: VINP[2] to ADC single positive input 0111: VINP[3] to ADC single positive input 1000: VINP[0]-VINM[0] to ADC differential input 1001: VINP[1]-VINM[1] to ADC differential input 1010: VINP[2]-VINM[2] to ADC differential input 1011: VINP[3]-VINM[3] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved.",
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
                    name: "seq9",
                    description: Some(
                        "SEQ9[3:0]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq10",
                    description: Some(
                        "SEQ10[3:0]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail.",
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
                    name: "seq11",
                    description: Some(
                        "SEQ11[3:0]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq12",
                    description: Some(
                        "SEQ12[3:0]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq13",
                    description: Some(
                        "SEQ13[3:0]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail.",
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
                    name: "seq14",
                    description: Some(
                        "SEQ14[3:0]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq15",
                    description: Some(
                        "SEQ15[3:0]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Switch",
            extends: None,
            description: Some(
                "SWITCH register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "se_vin_0",
                    description: Some(
                        "SE_VIN_0[1:0]: input voltage for VINM[0] / VINP[0]-VINM[0] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
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
                    name: "se_vin_1",
                    description: Some(
                        "SE_VIN_1[1:0]: input voltage for VINM[1] / VINP[1]-VINM[1] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
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
                    name: "se_vin_2",
                    description: Some(
                        "SE_VIN_2[1:0]: input voltage for VINM[2] / VINP[2]-VINM[2] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
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
                Field {
                    name: "se_vin_3",
                    description: Some(
                        "SE_VIN_3[1:0]: input voltage for VINM[3] / VINP[3]-VINM[3] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "se_vin_4",
                    description: Some(
                        "SE_VIN_4[1:0]: input voltage for VINP[0] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "se_vin_5",
                    description: Some(
                        "SE_VIN_5[1:0]: input voltage for VINP[1] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "se_vin_6",
                    description: Some(
                        "SE_VIN_6[1:0]: input voltage for VINP[2] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "se_vin_7",
                    description: Some(
                        "SE_VIN_7[1:0]: input voltage for VINP[3] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TimerConf",
            extends: None,
            description: Some(
                "Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_ldo_delay",
                    description: Some(
                        "define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion.",
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
                    name: "vbias_prech_delay",
                    description: Some(
                        "define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration.",
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
                    name: "prech_delay_sel",
                    description: Some(
                        "Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer.",
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
            ],
        },
        FieldSet {
            name: "VersionId",
            extends: None,
            description: Some(
                "VERSION_ID register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version_id",
                    description: Some(
                        "VERSION_ID[7:0]: version of the embedded IP.",
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
            name: "WdConf",
            extends: None,
            description: Some(
                "WD_CONF register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd_chx",
                    description: Some(
                        "AWD_CHX[15:0]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM[0] to ADC negative input Bit1: VINM[1] to ADC negative input Bit2: VINM[2] to ADC negative input Bit3: VINM[3] to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP[0] to ADC positive input Bit9: VINP[1] to ADC positive input Bit10: VINP[2] to ADC positive input Bit11: VINP[3] to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input.",
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
            name: "WdTh",
            extends: None,
            description: Some(
                "WD_TH register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd_lt",
                    description: Some(
                        "WD_LT[11:0]: analog watchdog low level threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wd_ht",
                    description: Some(
                        "WD_HT[11:0]: analog watchdog high level threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
