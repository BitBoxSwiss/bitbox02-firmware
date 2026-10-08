
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
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
                    name: "cfgr",
                    description: Some(
                        "ADC configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
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
                        "ADC sample time register 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
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
                    name: "pcsel",
                    description: Some(
                        "ADC channel preselection register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr1",
                    description: Some(
                        "ADC regular sequence register 1.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr2",
                    description: Some(
                        "ADC regular sequence register 2.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr3",
                    description: Some(
                        "ADC regular sequence register 3.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr4",
                    description: Some(
                        "ADC regular sequence register 4.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "ADC regular data register.",
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
                    name: "jsqr",
                    description: Some(
                        "ADC injected sequence register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jsqr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ofcfgr",
                    description: Some(
                        "ADC offset configuration register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ofr",
                    description: Some(
                        "ADC offset register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcomp",
                    description: Some(
                        "ADC gain compensation register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcomp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr",
                    description: Some(
                        "ADC injected channel data register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd2cr",
                    description: Some(
                        "ADC Analog Watchdog 2 Configuration Register.",
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
                        "ADC Analog Watchdog 3 Configuration Register.",
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
                    name: "ltr1",
                    description: Some(
                        "ADC analog watchdog 1 lower threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr1",
                    description: Some(
                        "ADC analog watchdog 1 higher threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr2",
                    description: Some(
                        "ADC analog watchdog 2 lower threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr2",
                    description: Some(
                        "ADC analog watchdog 2 higher threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr3",
                    description: Some(
                        "ADC analog watchdog 3 lower threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr3",
                    description: Some(
                        "ADC analog watchdog 3 higher threshold register.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact",
                    description: Some(
                        "ADC calibration factors.",
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
                    name: "or",
                    description: Some(
                        "ADC option register.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Or",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Awd2cr",
            extends: None,
            description: Some(
                "ADC Analog Watchdog 2 Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog 2 channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd3cr",
            extends: None,
            description: Some(
                "ADC Analog Watchdog 3 Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog 3 channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact",
            extends: None,
            description: Some(
                "ADC calibration factors.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calfact",
                    description: Some(
                        "Calibration factors.",
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
            name: "Cfgr",
            extends: None,
            description: Some(
                "ADC configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmngt",
                    description: Some(
                        "Data management configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dmngt",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "Data resolution.",
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
                    name: "extsel",
                    description: Some(
                        "External trigger selection for regular group.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exten",
                    description: Some(
                        "External trigger enable and polarity selection for regular channels.",
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
                        "Overrun mode.",
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
                    name: "cont",
                    description: Some(
                        "Single / continuous conversion mode for regular conversions.",
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
                    name: "autdly",
                    description: Some(
                        "Delayed conversion mode.",
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
                        "Discontinuous mode for regular channels.",
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
                    name: "discnum",
                    description: Some(
                        "Discontinuous mode channel count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdiscen",
                    description: Some(
                        "Discontinuous mode on injected channels.",
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
                    name: "jqm",
                    description: Some(
                        "JSQR queue mode.",
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
                    name: "awd1sgl",
                    description: Some(
                        "Enable the watchdog 1 on a single channel or on all channels.",
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
                    name: "awd1en",
                    description: Some(
                        "Analog watchdog 1 enable on regular channels.",
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
                    name: "jawd1en",
                    description: Some(
                        "Analog watchdog 1 enable on injected channels.",
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
                Field {
                    name: "jauto",
                    description: Some(
                        "Automatic injected group conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ch",
                    description: Some(
                        "Analog watchdog 1 channel selection.",
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
                Field {
                    name: "jqdis",
                    description: Some(
                        "Injected queue disable.",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "ADC configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rovse",
                    description: Some(
                        "Regular oversampling enable.",
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
                    name: "jovse",
                    description: Some(
                        "Injected oversampling enable.",
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
                    name: "ovss",
                    description: Some(
                        "Oversampling shift.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trovs",
                    description: Some(
                        "Triggered regular oversampling.",
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
                    name: "rovsm",
                    description: Some(
                        "Regular oversampling mode.",
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
                    name: "bulb",
                    description: Some(
                        "Bulb sampling mode.",
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
                    name: "swtrig",
                    description: Some(
                        "Software trigger bit for sampling time control trigger mode.",
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
                    name: "smptrig",
                    description: Some(
                        "Sampling time control trigger mode.",
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
                    name: "ovsr",
                    description: Some(
                        "Oversampling ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lshift",
                    description: Some(
                        "Left shift factor.",
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
                        "ADC enable control.",
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
                        "ADC disable command.",
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
                        "ADC start of regular conversion.",
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
                    name: "jadstart",
                    description: Some(
                        "ADC start of injected conversion.",
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
                    name: "adstp",
                    description: Some(
                        "ADC stop of regular conversion command.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adstp",
                    ),
                },
                Field {
                    name: "jadstp",
                    description: Some(
                        "ADC stop of injected conversion command.",
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
                    name: "advregen",
                    description: Some(
                        "ADC internal voltage regulator enable.",
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
                    name: "deeppwd",
                    description: Some(
                        "Deep-power-down enable.",
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
                    name: "adcaldif",
                    description: Some(
                        "Differential mode for calibration.",
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
                    name: "adcal",
                    description: Some(
                        "ADC calibration.",
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
                "ADC regular data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "Regular data converted.",
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
            name: "Gcomp",
            extends: None,
            description: Some(
                "ADC gain compensation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gcompcoeff",
                    description: Some(
                        "Gain compensation coefficient.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gcomp",
                    description: Some(
                        "Gain compensation mode.",
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
            name: "Htr1",
            extends: None,
            description: Some(
                "ADC analog watchdog 1 higher threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr",
                    description: Some(
                        "Analog watchdog 1 higher threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdfilt",
                    description: Some(
                        "Analog watchdog filtering parameter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr2",
            extends: None,
            description: Some(
                "ADC analog watchdog 2 higher threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr",
                    description: Some(
                        "Analog watchdog 2 higher threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr3",
            extends: None,
            description: Some(
                "ADC analog watchdog 3 higher threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr",
                    description: Some(
                        "Analog watchdog 3 higher threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
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
                        "ADC ready interrupt enable.",
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
                        "End of sampling flag interrupt enable for regular conversions.",
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
                        "End of regular conversion interrupt enable.",
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
                        "End of regular sequence of conversions interrupt enable.",
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
                        "Overrun interrupt enable.",
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
                    name: "jeocie",
                    description: Some(
                        "End of injected conversion interrupt enable.",
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
                    name: "jeosie",
                    description: Some(
                        "End of injected sequence of conversions interrupt enable.",
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
                    name: "awd1ie",
                    description: Some(
                        "Analog watchdog 1 interrupt enable.",
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
                    name: "awd2ie",
                    description: Some(
                        "Analog watchdog 2 interrupt enable.",
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
                    name: "awd3ie",
                    description: Some(
                        "Analog watchdog 3 interrupt enable.",
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
                    name: "jqovfie",
                    description: Some(
                        "Injected context queue overflow interrupt enable.",
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
                    name: "ldordyie",
                    description: Some(
                        "ADC internal voltage regulator interrupt enable.",
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
                        "ADC ready.",
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
                        "End of sampling flag.",
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
                        "End of conversion flag.",
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
                        "End of regular sequence flag.",
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
                        "ADC overrun.",
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
                    name: "jeoc",
                    description: Some(
                        "Injected channel end of conversion flag.",
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
                    name: "jeos",
                    description: Some(
                        "Injected channel end of sequence flag.",
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
                    name: "awd1",
                    description: Some(
                        "Analog watchdog 1 flag.",
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
                    name: "awd2",
                    description: Some(
                        "Analog watchdog 2 flag.",
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
                    name: "awd3",
                    description: Some(
                        "Analog watchdog 3 flag.",
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
                    name: "jqovf",
                    description: Some(
                        "Injected context queue overflow.",
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
                    name: "ldordy",
                    description: Some(
                        "ADC internal voltage regulator output ready flag.",
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
            name: "Jdr",
            extends: None,
            description: Some(
                "ADC injected channel 1 data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data.",
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
            name: "Jsqr",
            extends: None,
            description: Some(
                "ADC injected sequence register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jl",
                    description: Some(
                        "Injected channel sequence length.",
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
                    name: "jextsel",
                    description: Some(
                        "External trigger selection for injected group.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jexten",
                    description: Some(
                        "External trigger enable and polarity selection for injected channels.",
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
                    name: "jsq",
                    description: Some(
                        "1st conversion in the injected sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr1",
            extends: None,
            description: Some(
                "ADC analog watchdog 1 lower threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr",
                    description: Some(
                        "Analog watchdog 1 lower threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr2",
            extends: None,
            description: Some(
                "ADC analog watchdog 2 lower threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr",
                    description: Some(
                        "Analog watchdog 2 lower threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr3",
            extends: None,
            description: Some(
                "ADC analog watchdog 3 lower threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr",
                    description: Some(
                        "Analog watchdog 3 lower threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofcfgr",
            extends: None,
            description: Some(
                "ADC offset 1 configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "posoff",
                    description: Some(
                        "Positive offset enable.",
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
                Field {
                    name: "usat",
                    description: Some(
                        "Unsigned saturation enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssat",
                    description: Some(
                        "Signed saturation enable.",
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
                    name: "offset_ch",
                    description: Some(
                        "Channel selection for the data offset y.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofr",
            extends: None,
            description: Some(
                "ADC offset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "Data offset y for the channel programmed in OFFSETy_CH[4:0] bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Or",
            extends: None,
            description: Some(
                "ADC option register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcoren",
                    description: Some(
                        "Set and cleared by software to enable the vcore channel.",
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
            name: "Pcsel",
            extends: None,
            description: Some(
                "ADC channel preselection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcsel",
                    description: Some(
                        "Channel i (Vless thansub>INPless than/sub>[i]) preselection.",
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
                                len: 19,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr",
            extends: None,
            description: Some(
                "ADC sample time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "Channel x sampling time selection (x=9 to 0).",
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
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sqr1",
            extends: None,
            description: Some(
                "ADC regular sequence register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l",
                    description: Some(
                        "Regular channel sequence length.",
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
                    name: "sq",
                    description: Some(
                        "1st conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr2",
            extends: None,
            description: Some(
                "ADC regular sequence register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "5th conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr3",
            extends: None,
            description: Some(
                "ADC regular sequence register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "10th conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr4",
            extends: None,
            description: Some(
                "ADC regular sequence register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "15th conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 6,
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
            name: "Adstp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP",
                    description: Some(
                        "Stop conversion of channel",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmngt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DR",
                    description: Some(
                        "Regular conversion data stored in DR only.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DMA_ONE_SHOT",
                    description: Some(
                        "DMA one-shot mode selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MDF",
                    description: Some(
                        "MDF mode detected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DMA_CIRCULAR",
                    description: Some(
                        "DMA circular mode selected.",
                    ),
                    value: 3,
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
            name: "Pcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_PRESELECTED",
                    description: Some(
                        "Input channel i (Vless thansub>INPless than/sub>[i]) is not preselected for conversion, the result of the ADC conversion for this channel is wrong.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRESELECTED",
                    description: Some(
                        "Input channel i (Vless thansub>INPless than/sub>[i]) is preselected for conversion.",
                    ),
                    value: 1,
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
                        "12-bit.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some(
                        "6-bit.",
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
                        "1.5 ADC clock cycles.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES2_5",
                    description: Some(
                        "2.5 ADC clock cycles.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES6_5",
                    description: Some(
                        "6.5 ADC clock cycles.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES11_5",
                    description: Some(
                        "11.5 ADC clock cycles.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES23_5",
                    description: Some(
                        "23.5 ADC clock cycles.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES46_5",
                    description: Some(
                        "46.5 ADC clock cycles.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES246_5",
                    description: Some(
                        "246.5 ADC clock cycles.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES1499_5",
                    description: Some(
                        "1499.5 ADC clock cycles.",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
