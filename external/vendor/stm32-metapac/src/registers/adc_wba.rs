
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc4",
            extends: None,
            description: Some(
                "ADC register block.",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "ADC interrupt and status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "ier",
                    description: Some(
                        "ADC interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cr",
                    description: Some(
                        "ADC control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "cfgr1",
                    description: Some(
                        "ADC configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "ADC configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr",
                    description: Some(
                        "ADC sampling time register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd1tr",
                    description: Some(
                        "ADC watchdog threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd1tr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd2tr",
                    description: Some(
                        "ADC watchdog threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd2tr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chselr",
                    description: Some(
                        "ADC channel selection register [alternate].",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chselr_alternate",
                    description: Some(
                        "ADC channel selection register [alternate].",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChselrAlternate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd3tr",
                    description: Some(
                        "ADC watchdog threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd3tr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "ADC data register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwr",
                    description: Some(
                        "ADC Power register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd2cr",
                    description: Some(
                        "ADC Analog Watchdog 2 Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd3cr",
                    description: Some(
                        "ADC Analog Watchdog 3 Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd3cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact",
                    description: Some(
                        "ADC Calibration factor.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfact",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "ADC common configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x308,
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
            name: "Awd1tr",
            extends: None,
            description: Some(
                "ADC watchdog threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt1",
                    description: Some(
                        "Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
                    name: "ht1",
                    description: Some(
                        "Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
        FieldSet {
            name: "Awd2cr",
            extends: None,
            description: Some(
                "ADC Analog Watchdog 2 Configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd2ch",
                    description: Some(
                        "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through AWD2CR must be also configured into the ADC_CHSELR registers. Refer to SQ8[3:0] for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
                                len: 14,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd2tr",
            extends: None,
            description: Some(
                "ADC watchdog threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt2",
                    description: Some(
                        "Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
                    name: "ht2",
                    description: Some(
                        "Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
        FieldSet {
            name: "Awd3cr",
            extends: None,
            description: Some(
                "ADC Analog Watchdog 3 Configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd3ch",
                    description: Some(
                        "Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8[3:0] for a definition of channel selection. The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
                                len: 14,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd3tr",
            extends: None,
            description: Some(
                "ADC watchdog threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt3",
                    description: Some(
                        "Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
                    name: "ht3",
                    description: Some(
                        "Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.",
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
        FieldSet {
            name: "Calfact",
            extends: None,
            description: Some(
                "ADC Calibration factor.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calfact",
                    description: Some(
                        "Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new calibration is launched. Just after a calibration is complete, DATA[6:0] contains the calibration factor. Note: Software can write these bits only when ADEN = 1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).",
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
            name: "Ccr",
            extends: None,
            description: Some(
                "ADC common configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "presc",
                    description: Some(
                        "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Presc",
                    ),
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VsubREFINT/sub enable This bit is set and cleared by software to enable/disable the VsubREFINT/sub buffer. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
                    name: "vsensesel",
                    description: Some(
                        "Temperature sensor selection This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "ADC configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaen",
                    description: Some(
                        "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the automatic management of the converted data by the DMA controller. For more details, refer to Section : Managing converted data using the DMA on page 632. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "dmacfg",
                    description: Some(
                        "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing converted data using the DMA on page 632 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dmacfg",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Res",
                    ),
                },
                Field {
                    name: "scandir",
                    description: Some(
                        "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELRMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Scandir",
                    ),
                },
                Field {
                    name: "align",
                    description: Some(
                        "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Align",
                    ),
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "External trigger selection These bits select the external event used to trigger the start of conversion (refer to table ADC interconnection in Section 20.4.2: ADC pins and internal signals for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Extsel",
                    ),
                },
                Field {
                    name: "exten",
                    description: Some(
                        "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Exten",
                    ),
                },
                Field {
                    name: "ovrmod",
                    description: Some(
                        "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ovrmod",
                    ),
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cont",
                    ),
                },
                Field {
                    name: "wait",
                    description: Some(
                        "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.sup./sup Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "discen",
                    description: Some(
                        "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "chselrmod",
                    description: Some(
                        "Mode selection of the CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Chselrmod",
                    ),
                },
                Field {
                    name: "awd1sgl",
                    description: Some(
                        "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH[4:0] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Awd1sgl",
                    ),
                },
                Field {
                    name: "awd1en",
                    description: Some(
                        "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "awd1ch",
                    description: Some(
                        "Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved The channel selected by the AWDCH[4:0] bits must be also set into the CHSELR register. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "ADC configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovse",
                    description: Some(
                        "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
                    name: "ovsr",
                    description: Some(
                        "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "OversamplingRatio",
                    ),
                },
                Field {
                    name: "ovss",
                    description: Some(
                        "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Ovss",
                    ),
                },
                Field {
                    name: "tovs",
                    description: Some(
                        "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tovs",
                    ),
                },
                Field {
                    name: "lftrig",
                    description: Some(
                        "Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
            ],
        },
        FieldSet {
            name: "Chselr",
            extends: None,
            description: Some(
                "ADC channel selection register [alternate].",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chsel0",
                    description: Some(
                        "Channel x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
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
                                len: 14,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ChselrAlternate",
            extends: None,
            description: Some(
                "ADC channel selection register [alternate].",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "1st conversion of the sequence These bits are programmed by software with the channel number assigned to the 1st conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8[3:0] for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "ADC control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aden",
                    description: Some(
                        "ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0).",
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
                    name: "addis",
                    description: Some(
                        "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to 1 is only effective when ADEN = 1 and ADSTART = 0 (which ensures that no conversion is ongoing).",
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
                    name: "adstart",
                    description: Some(
                        "ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN [1:0] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT=0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC).",
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
                    name: "adstp",
                    description: Some(
                        "ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: To clear the A/D converter state, ADSTP must be set to 1 even if ADSTART is cleared to 0 after the software trigger A/D conversion. It is recommended to set ADSTP to 1 whenever the configuration needs to be modified.",
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
                    name: "advregen",
                    description: Some(
                        "ADC voltage regulator enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tsubADCVREG_SETUP/sub. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).",
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
                Field {
                    name: "adcal",
                    description: Some(
                        "ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing CALFACT only when ADEN is set to 1 and ADSTART is cleared to 0 by writing ADSTP to 1 (ADC enabled and no conversion is ongoing).",
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
            name: "Dr",
            extends: None,
            description: Some(
                "ADC data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in Figure 78: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 631. Just after a calibration is complete, DATA[6:0] contains the calibration factor.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "ADC interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdyie",
                    description: Some(
                        "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "eosmpie",
                    description: Some(
                        "End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "eocie",
                    description: Some(
                        "End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "eosie",
                    description: Some(
                        "End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "awdie",
                    description: Some(
                        "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "eocalie",
                    description: Some(
                        "End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).",
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
                    name: "ldordyie",
                    description: Some(
                        "LDO ready interrupt enable This bit is set and cleared by software. It is used to enable/disable the LDORDY interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensure that no conversion is ongoing).",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "ADC interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy",
                    description: Some(
                        "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.",
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
                    name: "eosmp",
                    description: Some(
                        "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by writing 1 to it.",
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
                    name: "eoc",
                    description: Some(
                        "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.",
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
                    name: "eos",
                    description: Some(
                        "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.",
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
                    name: "ovr",
                    description: Some(
                        "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.",
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
                    name: "awd",
                    description: Some(
                        "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in TR1 and ADC_HR1 registers. It is cleared by software by writing 1 to it.",
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
                    name: "eocal",
                    description: Some(
                        "End of calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.",
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
                    name: "ldordy",
                    description: Some(
                        "LDO ready This bit is set by hardware. It indicates that the ADC internal LDO output is ready. It is cleared by software by writing 1 to it.",
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
            ],
        },
        FieldSet {
            name: "Pwr",
            extends: None,
            description: Some(
                "ADC Power register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "autoff",
                    description: Some(
                        "Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing).",
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
                    name: "dpd",
                    description: Some(
                        "Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO.",
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
            name: "Smpr",
            extends: None,
            description: Some(
                "ADC sampling time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smpsel",
                    description: Some(
                        "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 14,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Smpsel",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Align",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RIGHT",
                    description: Some(
                        "Right alignment.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEFT",
                    description: Some(
                        "Left alignment.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Awd1sgl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL_CHANNELS",
                    description: Some(
                        "Analog watchdog 1 enabled on all channels.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SINGLE_CHANNEL",
                    description: Some(
                        "Analog watchdog 1 enabled on a single channel.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Chselrmod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLE_INPUT",
                    description: Some(
                        "Each bit of the CHSELR register enables an input.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SEQUENCE_EIGHT_CHANNELS",
                    description: Some(
                        "CHSELR register is able to sequence up to 8 channels.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cont",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Single conversion mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONTINUOUS",
                    description: Some(
                        "Continuous conversion mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmacfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONE_SHOT",
                    description: Some(
                        "DMA One Shot mode selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CIRCULAR",
                    description: Some(
                        "DMA Circular mode selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Exten",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Trigger detection disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTH_EDGES",
                    description: Some(
                        "Trigger detection on both the rising and falling edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Extsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TRG0",
                    description: Some(
                        "adc_trg0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRG1",
                    description: Some(
                        "adc_trg1.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TRG2",
                    description: Some(
                        "adc_trg2.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TRG3",
                    description: Some(
                        "adc_trg3.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TRG4",
                    description: Some(
                        "adc_trg4.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TRG5",
                    description: Some(
                        "adc_trg5.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TRG6",
                    description: Some(
                        "adc_trg6.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "TRG7",
                    description: Some(
                        "adc_trg7.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "OversamplingRatio",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "OVERSAMPLE2X",
                    description: Some(
                        "Oversample 2 times",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERSAMPLE4X",
                    description: Some(
                        "Oversample 4 times",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "OVERSAMPLE8X",
                    description: Some(
                        "Oversample 8 times",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "OVERSAMPLE16X",
                    description: Some(
                        "Oversample 16 times",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "OVERSAMPLE32X",
                    description: Some(
                        "Oversample 32 times",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "OVERSAMPLE64X",
                    description: Some(
                        "Oversample 64 times",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "OVERSAMPLE128X",
                    description: Some(
                        "Oversample 128 times",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "OVERSAMPLE256X",
                    description: Some(
                        "Oversample 256 times",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ovrmod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PRESERVE",
                    description: Some(
                        "DR register is preserved with the old data when an overrun is detected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERWRITE",
                    description: Some(
                        "DR register is overwritten with the last conversion result when an overrun is detected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ovss",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "SHIFT0",
                    description: Some(
                        "No shift.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SHIFT1",
                    description: Some(
                        "Shift 1-bit.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SHIFT2",
                    description: Some(
                        "Shift 2-bits.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SHIFT3",
                    description: Some(
                        "Shift 3-bits.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SHIFT4",
                    description: Some(
                        "Shift 4-bits.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SHIFT5",
                    description: Some(
                        "Shift 5-bits.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SHIFT6",
                    description: Some(
                        "Shift 6-bits.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "SHIFT7",
                    description: Some(
                        "Shift 7-bits.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "SHIFT8",
                    description: Some(
                        "Shift 8-bits.",
                    ),
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Presc",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "input ADC clock not divided.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "input ADC clock divided by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "input ADC clock divided by 4.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "input ADC clock divided by 6.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "input ADC clock divided by 8.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "input ADC clock divided by 10.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "input ADC clock divided by 12.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "input ADC clock divided by 16.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "input ADC clock divided by 32.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "input ADC clock divided by 64.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "input ADC clock divided by 128.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "input ADC clock divided by 256.",
                    ),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "Res",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12-bit resolution",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit resolution",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some(
                        "6-bit resolution",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES1_5",
                    description: Some(
                        "1.5 ADC cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES3_5",
                    description: Some(
                        "3.5 ADC cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES7_5",
                    description: Some(
                        "7.5 ADC cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES12_5",
                    description: Some(
                        "12.5 ADC cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES19_5",
                    description: Some(
                        "19.5 ADC cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES39_5",
                    description: Some(
                        "39.5 ADC cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES79_5",
                    description: Some(
                        "79.5 ADC cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES814_5",
                    description: Some(
                        "814.5 ADC cycles",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Scandir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UPWARDS",
                    description: Some(
                        "Upward scan (from CHSEL0 to CHSEL11).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BACKWARDS",
                    description: Some(
                        "Backward scan (from CHSEL11 to CHSEL0).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smpsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SMP1",
                    description: Some(
                        "Sampling time of CHANNELx use the setting of SMP1[2:0] register.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SMP2",
                    description: Some(
                        "Sampling time of CHANNELx use the setting of SMP2[2:0] register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tovs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL_AFTER_TRIGGER",
                    description: Some(
                        "All oversampled conversions for a channel are done consecutively after a trigger.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EACH_AFTER_TRIGGER",
                    description: Some(
                        "Each oversampled conversion for a channel needs a trigger.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
