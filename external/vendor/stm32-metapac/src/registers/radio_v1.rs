
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Radio",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "aa0_dig_usr",
                    description: Some(
                        "AA0_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aa0DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aa1_dig_usr",
                    description: Some(
                        "AA1_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aa1DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aa2_dig_usr",
                    description: Some(
                        "AA2_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aa2DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aa3_dig_usr",
                    description: Some(
                        "AA3_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aa3DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dem_mod_dig_usr",
                    description: Some(
                        "DEM_MOD_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DemModDigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "radio_fsm_usr",
                    description: Some(
                        "RADIO_FSM_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RadioFsmUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phyctrl_dig_usr",
                    description: Some(
                        "PHYCTRL_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyctrlDigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afc1_dig_eng",
                    description: Some(
                        "AFC1_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afc1DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr0_dig_eng",
                    description: Some(
                        "CR0_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr0DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr0_lr",
                    description: Some(
                        "CR0_LR register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr0Lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vit_conf_dig_eng",
                    description: Some(
                        "VIT_CONF_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VitConfDigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lr_pd_thr_dig_eng",
                    description: Some(
                        "LR_PD_THR_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LrPdThrDigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lr_rssi_thr_dig_eng",
                    description: Some(
                        "LR_RSSI_THR_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LrRssiThrDigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lr_aac_thr_dig_eng",
                    description: Some(
                        "LR_AAC_THR_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LrAacThrDigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal0_dig_eng",
                    description: Some(
                        "SYNTHCAL0_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal0DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtb5_dig_eng",
                    description: Some(
                        "DTB5_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtb5DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxadc_ana_usr",
                    description: Some(
                        "RXADC_ANA_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxadcAnaUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ldo_ana_eng",
                    description: Some(
                        "LDO_ANA_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LdoAnaEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cbias0_ana_eng",
                    description: Some(
                        "CBIAS0_ANA_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cbias0AnaEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cbias1_ana_eng",
                    description: Some(
                        "CBIAS1_ANA_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cbias1AnaEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal0_dig_out",
                    description: Some(
                        "SYNTHCAL0_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal0DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal1_dig_out",
                    description: Some(
                        "SYNTHCAL1_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal1DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal2_dig_out",
                    description: Some(
                        "SYNTHCAL2_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal2DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal3_dig_out",
                    description: Some(
                        "SYNTHCAL3_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal3DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal4_dig_out",
                    description: Some(
                        "SYNTHCAL4_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal4DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synthcal5_dig_out",
                    description: Some(
                        "SYNTHCAL5_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Synthcal5DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fsm_status_dig_out",
                    description: Some(
                        "FSM_STATUS_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "FsmStatusDigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rssi0_dig_out",
                    description: Some(
                        "RSSI0_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rssi0DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rssi1_dig_out",
                    description: Some(
                        "RSSI1_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rssi1DigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc_dig_out",
                    description: Some(
                        "AGC_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "AgcDigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "demod_dig_out",
                    description: Some(
                        "DEMOD_DIG_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DemodDigOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc2_ana_tst",
                    description: Some(
                        "AGC2_ANA_TST register.",
                    ),
                    array: None,
                    byte_offset: 0x1bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc2AnaTst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc0_dig_eng",
                    description: Some(
                        "AGC0_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc0DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc1_dig_eng",
                    description: Some(
                        "AGC1_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc1DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc10_dig_eng",
                    description: Some(
                        "AGC10_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc10DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc11_dig_eng",
                    description: Some(
                        "AGC11_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc11DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc12_dig_eng",
                    description: Some(
                        "AGC12_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc12DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc13_dig_eng",
                    description: Some(
                        "AGC13_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc13DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc14_dig_eng",
                    description: Some(
                        "AGC14_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc14DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc15_dig_eng",
                    description: Some(
                        "AGC15_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc15DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc16_dig_eng",
                    description: Some(
                        "AGC16_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc16DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc17_dig_eng",
                    description: Some(
                        "AGC17_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc17DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc18_dig_eng",
                    description: Some(
                        "AGC18_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc18DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc19_dig_eng",
                    description: Some(
                        "AGC19_DIG_ENG register.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Agc19DigEng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxadc_hw_trim_out",
                    description: Some(
                        "RXADC_HW_TRIM_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxadcHwTrimOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cbias0_hw_trim_out",
                    description: Some(
                        "CBIAS0_HW_TRIM_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cbias0HwTrimOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "agc_hw_trim_out",
                    description: Some(
                        "AGC_HW_TRIM_OUT register.",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "AgcHwTrimOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "demod_iq2_dig_tst",
                    description: Some(
                        "DEMOD_IQ2_DIG_TST register.",
                    ),
                    array: None,
                    byte_offset: 0x23c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DemodIq2DigTst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "antsw0_dig_usr",
                    description: Some(
                        "ANTSW0_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Antsw0DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "antsw1_dig_usr",
                    description: Some(
                        "ANTSW1_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Antsw1DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "antsw2_dig_usr",
                    description: Some(
                        "ANTSW2_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Antsw2DigUsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "antsw3_dig_usr",
                    description: Some(
                        "ANTSW3_DIG_USR register.",
                    ),
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Antsw3DigUsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Aa0DigUsr",
            extends: None,
            description: Some(
                "AA0_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aa_7_0",
                    description: Some(
                        "Least significant byte of the Bluetooth LE Access Address code.",
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
            name: "Aa1DigUsr",
            extends: None,
            description: Some(
                "AA1_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aa_15_8",
                    description: Some(
                        "Next byte of the Bluetooth LE Access Address code.",
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
            name: "Aa2DigUsr",
            extends: None,
            description: Some(
                "AA2_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aa_23_16",
                    description: Some(
                        "Next byte of the Bluetooth LE Access Address code.",
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
            name: "Aa3DigUsr",
            extends: None,
            description: Some(
                "AA3_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aa_31_24",
                    description: Some(
                        "Most significant byte of the Bluetooth LE Access Address code.",
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
            name: "Afc1DigEng",
            extends: None,
            description: Some(
                "AFC1_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afc_delay_after",
                    description: Some(
                        "Set the decay factor of the AFC loop after Access Address detection.",
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
                    name: "afc_delay_before",
                    description: Some(
                        "Set the decay factor of the AFC loop before Access Address detection.",
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
            ],
        },
        FieldSet {
            name: "Agc0DigEng",
            extends: None,
            description: Some(
                "AGC0_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "agc_thr_high",
                    description: Some(
                        "High AGC threshold.",
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
                Field {
                    name: "agc_enable",
                    description: Some(
                        "Enable AGC.",
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
        FieldSet {
            name: "Agc10DigEng",
            extends: None,
            description: Some(
                "AGC10_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_0",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 0:.",
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
                    name: "att_lna_0",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 0:.",
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
                    name: "att_ant_0",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 0:.",
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
            name: "Agc11DigEng",
            extends: None,
            description: Some(
                "AGC11_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_1",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 1.",
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
                    name: "att_lna_1",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 1.",
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
                    name: "att_ant_1",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 1.",
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
            name: "Agc12DigEng",
            extends: None,
            description: Some(
                "AGC12_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_2",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 2.",
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
                    name: "att_lna_2",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 2.",
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
                    name: "att_ant_2",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 2.",
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
            name: "Agc13DigEng",
            extends: None,
            description: Some(
                "AGC13_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_3",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 3.",
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
                    name: "att_lna_3",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 3.",
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
                    name: "att_ant_3",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 3.",
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
            name: "Agc14DigEng",
            extends: None,
            description: Some(
                "AGC14_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_4",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 4.",
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
                    name: "att_lna_4",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 4.",
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
                    name: "att_ant_4",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 4.",
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
            name: "Agc15DigEng",
            extends: None,
            description: Some(
                "AGC15_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_5",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 5.",
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
                    name: "att_lna_5",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 5.",
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
                    name: "att_ant_5",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 5.",
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
            name: "Agc16DigEng",
            extends: None,
            description: Some(
                "AGC16_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_6",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 6.",
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
                    name: "att_lna_6",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 6.",
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
                    name: "att_ant_6",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 6.",
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
            name: "Agc17DigEng",
            extends: None,
            description: Some(
                "AGC17_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_7",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 7.",
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
                    name: "att_lna_7",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 7.",
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
                    name: "att_ant_7",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 7.",
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
            name: "Agc18DigEng",
            extends: None,
            description: Some(
                "AGC18_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_8",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 8.",
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
                    name: "att_lna_8",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 8.",
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
                    name: "att_ant_8",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 8.",
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
            name: "Agc19DigEng",
            extends: None,
            description: Some(
                "AGC19_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "att_if_9",
                    description: Some(
                        "Attenuation at IF Level for the AGC step 9.",
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
                    name: "att_lna_9",
                    description: Some(
                        "Attenuation at LNA Level for the AGC step 9.",
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
                    name: "att_ant_9",
                    description: Some(
                        "Attenuation at Antenna Level for the AGC step 9.",
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
            name: "Agc1DigEng",
            extends: None,
            description: Some(
                "AGC1_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "agc_thr_low_6",
                    description: Some(
                        "Low threshold for 6dB steps.",
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
                Field {
                    name: "agc_autolock",
                    description: Some(
                        "AGC locks when level is steady between high threshold and lock threshold.",
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
                    name: "agc_lock_sync",
                    description: Some(
                        "AGC locks when Access Address is detected (recommended).",
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
            name: "Agc2AnaTst",
            extends: None,
            description: Some(
                "AGC2_ANA_TST register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "agc2_ana_tst_sel",
                    description: Some(
                        "Selection:.",
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
                    name: "agc_antennae_usr_trim",
                    description: Some(
                        "the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AgcDigOut",
            extends: None,
            description: Some(
                "AGC_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "agc_att_out",
                    description: Some(
                        "AGC attenuation value.",
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
            ],
        },
        FieldSet {
            name: "AgcHwTrimOut",
            extends: None,
            description: Some(
                "AGC_HW_TRIM_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hw_agc_antennae_trim",
                    description: Some(
                        "AGC trim value (provided by the HW trimming, automatically loaded on POR).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Antsw0DigUsr",
            extends: None,
            description: Some(
                "ANTSW0_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_time_to_sample",
                    description: Some(
                        "specifies the exact timing of the first I/Q sampling in the reference period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Antsw1DigUsr",
            extends: None,
            description: Some(
                "ANTSW1_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_time_to_switch",
                    description: Some(
                        "specifies the exact timing of the antenna switching at receiver level (in AoA).",
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
            name: "Antsw2DigUsr",
            extends: None,
            description: Some(
                "ANTSW2_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_time_to_switch",
                    description: Some(
                        "specifies the exact timing of the antenna switching during transmission at LE_1M baud rate (in AoD).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Antsw3DigUsr",
            extends: None,
            description: Some(
                "ANTSW3_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_time_to_switch_2m",
                    description: Some(
                        "specifies the exact timing of the antenna switching during transmission at LE_2M baud rate (in AoD).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cbias0AnaEng",
            extends: None,
            description: Some(
                "CBIAS0_ANA_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfd_cbias_ibias_trim",
                    description: Some(
                        "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1).",
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
                    name: "rfd_cbias_iptat_trim",
                    description: Some(
                        "overloaded value for cbias current trimming (when CBIAS0_TRIM_TST_SEL = 1).",
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
            ],
        },
        FieldSet {
            name: "Cbias0HwTrimOut",
            extends: None,
            description: Some(
                "CBIAS0_HW_TRIM_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hw_cbias_ibias_trim",
                    description: Some(
                        "CBIAS current (provided by the HW trimming, automatically loaded on POR).",
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
                    name: "hw_cbias_iptat_trim",
                    description: Some(
                        "CBIAS current (provided by the HW trimming, automatically loaded on POR).",
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
            ],
        },
        FieldSet {
            name: "Cbias1AnaEng",
            extends: None,
            description: Some(
                "CBIAS1_ANA_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbias0_trim_tst_sel",
                    description: Some(
                        "When set, RFD_CBIAS_(IPTAT/IBIAS)_TRIM are used instead of HW trimmings.",
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
            name: "Cr0DigEng",
            extends: None,
            description: Some(
                "CR0_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr_gain_after",
                    description: Some(
                        "Set the gain of the clock recovery loop before Access Address detection to the value.",
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
                    name: "cr_gain_before",
                    description: Some(
                        "Set the gain of the clock recovery loop before Access Address detection to the value.",
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
            ],
        },
        FieldSet {
            name: "Cr0Lr",
            extends: None,
            description: Some(
                "CR0_LR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr_lr_gain_after",
                    description: Some(
                        "Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use.",
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
                    name: "cr_lr_gain_before",
                    description: Some(
                        "Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use.",
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
            ],
        },
        FieldSet {
            name: "DemModDigUsr",
            extends: None,
            description: Some(
                "DEM_MOD_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "channel_num",
                    description: Some(
                        "Index for internal lock up table in which the synthesizer setup is contained.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DemodDigOut",
            extends: None,
            description: Some(
                "DEMOD_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ci_field",
                    description: Some(
                        "CI field.",
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
                    name: "aac_found",
                    description: Some(
                        "aac_found.",
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
                    name: "pd_found",
                    description: Some(
                        "pd_found.",
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
                    name: "rx_end",
                    description: Some(
                        "rx_end.",
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
            ],
        },
        FieldSet {
            name: "DemodIq2DigTst",
            extends: None,
            description: Some(
                "DEMOD_IQ2_DIG_TST register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "extcfg_sampling_time",
                    description: Some(
                        "Defines the sampling time, when extended configuration is enabled:.",
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
                    name: "extcfg_trig_selection",
                    description: Some(
                        "Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:.",
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
            ],
        },
        FieldSet {
            name: "Dtb5DigEng",
            extends: None,
            description: Some(
                "DTB5_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxtx_start_sel",
                    description: Some(
                        "enable the possibility to control some signals by the other register bits instead of system design:.",
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
                    name: "tx_active",
                    description: Some(
                        "Force TX_ACTIVE signal.",
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
                    name: "rx_active",
                    description: Some(
                        "Force RX_ACTIVE signal.",
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
                    name: "initialize",
                    description: Some(
                        "Force INITIALIZE signal (emulate a token request of the IP_BLE).",
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
                    name: "port_selected_en",
                    description: Some(
                        "enable port selection.",
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
                    name: "port_selected_0",
                    description: Some(
                        "force port_selected[0] signal.",
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
            name: "FsmStatusDigOut",
            extends: None,
            description: Some(
                "FSM_STATUS_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "RF FSM state:.",
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
                    name: "synth_cal_error",
                    description: Some(
                        "PLL calibration error.",
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
            name: "LdoAnaEng",
            extends: None,
            description: Some(
                "LDO_ANA_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfd_rf_reg_bypass",
                    description: Some(
                        "RF_REG Bypass mode:.",
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
            name: "LrAacThrDigEng",
            extends: None,
            description: Some(
                "LR_AAC_THR_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lr_aac_thr",
                    description: Some(
                        "address coded correlation threshold.",
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
            name: "LrPdThrDigEng",
            extends: None,
            description: Some(
                "LR_PD_THR_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lr_pd_thr",
                    description: Some(
                        "preamble detect threshold value.",
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
            name: "LrRssiThrDigEng",
            extends: None,
            description: Some(
                "LR_RSSI_THR_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lr_rssi_thr",
                    description: Some(
                        "RSSI or peak threshold value.",
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
            name: "PhyctrlDigUsr",
            extends: None,
            description: Some(
                "PHYCTRL_DIG_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxtxphy",
                    description: Some(
                        "RXTXPHY selection.",
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
            ],
        },
        FieldSet {
            name: "RadioFsmUsr",
            extends: None,
            description: Some(
                "RADIO_FSM_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_calib_cbp",
                    description: Some(
                        "CBP calibration enable bit.",
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
                    name: "en_calib_synth",
                    description: Some(
                        "SYNTH calibration enable bit.",
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
                    name: "pa_power",
                    description: Some(
                        "PA Power coefficient.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rssi0DigOut",
            extends: None,
            description: Some(
                "RSSI0_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rssi_meas_out_7_0",
                    description: Some(
                        "Measure of the received signal strength.",
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
            name: "Rssi1DigOut",
            extends: None,
            description: Some(
                "RSSI1_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rssi_meas_out_15_8",
                    description: Some(
                        "Measure of the received signal strength.",
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
            name: "RxadcAnaUsr",
            extends: None,
            description: Some(
                "RXADC_ANA_USR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfd_rxadc_delaytrim_i",
                    description: Some(
                        "ADC loop delay control bits for I channel to apply when SW overload is enabled.",
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
                    name: "rfd_rxadc_delaytrim_q",
                    description: Some(
                        "ADC loop delay control bits for Q channel to apply when SW overload is enabled.",
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
                Field {
                    name: "rxadc_delaytrim_i_tst_sel",
                    description: Some(
                        "Enable the SW overload on RXADX delay trimming.",
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
                    name: "rxadc_delaytrim_q_tst_sel",
                    description: Some(
                        "Enable the SW overload on RXADX delay trimming.",
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
            name: "RxadcHwTrimOut",
            extends: None,
            description: Some(
                "RXADC_HW_TRIM_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hw_rxadc_delaytrim_i",
                    description: Some(
                        "control bits of the RX ADC loop delay for I channel (provided by the HW trimming, automatically loaded on POR).",
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
                    name: "hw_rxadc_delaytrim_q",
                    description: Some(
                        "control bits of the RX ADC loop delay for Q channel (provided by the HW trimming, automatically loaded on POR).",
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
            name: "Synthcal0DigEng",
            extends: None,
            description: Some(
                "SYNTHCAL0_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "synthcal_debug_bus_sel",
                    description: Some(
                        "for Debug purpose.",
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
                    name: "synth_if_freq_cal",
                    description: Some(
                        "Define the frequency applied on the PLL during calibration phase.",
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
            ],
        },
        FieldSet {
            name: "Synthcal0DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL0_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vco_calamp_out_6_0",
                    description: Some(
                        "VCO CALAMP value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Synthcal1DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL1_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vco_calamp_out_10_7",
                    description: Some(
                        "VCO CALAMP value.",
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
            ],
        },
        FieldSet {
            name: "Synthcal2DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL2_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vco_calfreq_out",
                    description: Some(
                        "VCO CALFREQ value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Synthcal3DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL3_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "synthcal_debug_bus",
                    description: Some(
                        "Calibration debug bus.",
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
            name: "Synthcal4DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL4_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mod_ref_dac_word_out",
                    description: Some(
                        "Calibration word.",
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
            name: "Synthcal5DigOut",
            extends: None,
            description: Some(
                "SYNTHCAL5_DIG_OUT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbp_calib_word",
                    description: Some(
                        "CBP Calibration word.",
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
            ],
        },
        FieldSet {
            name: "VitConfDigEng",
            extends: None,
            description: Some(
                "VIT_CONF_DIG_ENG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vit_en",
                    description: Some(
                        "Viterbi enable.",
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
                    name: "spare",
                    description: Some(
                        "spare.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
