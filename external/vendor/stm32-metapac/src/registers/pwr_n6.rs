
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control.",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "PWR control register 1.",
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
                        "PWR control register 2.",
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
                        "PWR control register 3.",
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
                    name: "cr4",
                    description: Some(
                        "PWR control register 4.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "voscr",
                    description: Some(
                        "PWR voltage scaling control register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Voscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr1",
                    description: Some(
                        "PWR backup domain control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr2",
                    description: Some(
                        "PWR backup domain control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbpcr",
                    description: Some(
                        "PWR disable backup protection control register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpucr",
                    description: Some(
                        "PWR CPU control register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpucr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "svmcr1",
                    description: Some(
                        "PWR supply voltage monitoring control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "svmcr2",
                    description: Some(
                        "PWR supply voltage monitoring control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "svmcr3",
                    description: Some(
                        "PWR supply voltage monitoring control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupcr",
                    description: Some(
                        "PWR wake-up clear register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkupcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupsr",
                    description: Some(
                        "PWR wake-up status register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkupsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupepr",
                    description: Some(
                        "PWR wake-up enable and polarity register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkupepr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "PWR security configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "PWR privilege configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bdcr1",
            extends: None,
            description: Some(
                "PWR backup domain control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "monen",
                    description: Some(
                        "V less than sub>BAT less than /sub> and temperature monitoring enable.",
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
                    name: "vbatl",
                    description: Some(
                        "V less than sub>BAT less than /sub> level monitoring versus low threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbatl",
                    ),
                },
                Field {
                    name: "vbath",
                    description: Some(
                        "V less than sub>BAT less than /sub> level monitoring versus high threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbath",
                    ),
                },
                Field {
                    name: "templ",
                    description: Some(
                        "Temperature level monitoring versus low threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Templ",
                    ),
                },
                Field {
                    name: "temph",
                    description: Some(
                        "Temperature level monitoring versus high threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Temph",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bdcr2",
            extends: None,
            description: Some(
                "PWR backup domain control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkprbsen",
                    description: Some(
                        "Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and V less than sub>BAT less than /sub> modes).",
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
            name: "Cpucr",
            extends: None,
            description: Some(
                "PWR CPU control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdds",
                    description: Some(
                        "Power-down deepsleep selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pdds",
                    ),
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "Clear Standby and Stop flags (always read as 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cssf",
                    ),
                },
                Field {
                    name: "stopf",
                    description: Some(
                        "Stop flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopf",
                    ),
                },
                Field {
                    name: "sbf",
                    description: Some(
                        "Standby flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sbf",
                    ),
                },
                Field {
                    name: "svos",
                    description: Some(
                        "System Stop mode voltage scaling selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Svos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "PWR control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sden",
                    description: Some(
                        "SMPS step-down converter enable.",
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
                    name: "mode_pdn",
                    description: Some(
                        "Enables the pull down on output voltage during power-down mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ModePdn",
                    ),
                },
                Field {
                    name: "lpds08v",
                    description: Some(
                        "SMPS low-power mode enable (SVOS high only).",
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
                    name: "popl",
                    description: Some(
                        "pwr_on pulse low configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Popl",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "PWR control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvden",
                    description: Some(
                        "Programmable voltage detector enable.",
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
                    name: "pvdo",
                    description: Some(
                        "Programmable voltage detect output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pvdo",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "PWR control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcoremonen",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> monitoring enable.",
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
                    name: "vcorells",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> voltage detector low-level selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vcorells",
                    ),
                },
                Field {
                    name: "vcorel",
                    description: Some(
                        "Monitored V less than sub>DDCORE less than /sub> level above low threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vcorel",
                    ),
                },
                Field {
                    name: "vcoreh",
                    description: Some(
                        "Monitored V less than sub>DDCORE less than /sub> level above high threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vcoreh",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr4",
            extends: None,
            description: Some(
                "PWR control register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcmrbsen",
                    description: Some(
                        "I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode).",
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
                    name: "tcmflxrbsen",
                    description: Some(
                        "I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode).",
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
            name: "Dbpcr",
            extends: None,
            description: Some(
                "PWR disable backup protection control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable backup domain write protection.",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "PWR privilege configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv0",
                    description: Some(
                        "System supply configuration privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv0",
                    ),
                },
                Field {
                    name: "priv1",
                    description: Some(
                        "Programmable voltage detector privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv1",
                    ),
                },
                Field {
                    name: "priv2",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> monitor privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv2",
                    ),
                },
                Field {
                    name: "priv3",
                    description: Some(
                        "I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv3",
                    ),
                },
                Field {
                    name: "priv4",
                    description: Some(
                        "Voltage scaling selection privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv4",
                    ),
                },
                Field {
                    name: "priv5",
                    description: Some(
                        "Backup domain privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv5",
                    ),
                },
                Field {
                    name: "priv6",
                    description: Some(
                        "CPU power control privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv6",
                    ),
                },
                Field {
                    name: "priv7",
                    description: Some(
                        "Peripheral voltage monitor privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv7",
                    ),
                },
                Field {
                    name: "wkuppriv1",
                    description: Some(
                        "WKUP1 pin privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkuppriv1",
                    ),
                },
                Field {
                    name: "wkuppriv2",
                    description: Some(
                        "WKUP2 pin privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkuppriv2",
                    ),
                },
                Field {
                    name: "wkuppriv3",
                    description: Some(
                        "WKUP3 pin privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkuppriv3",
                    ),
                },
                Field {
                    name: "wkuppriv4",
                    description: Some(
                        "WKUP4 pin privileged protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkuppriv4",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "PWR security configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec0",
                    description: Some(
                        "System supply configuration secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec0",
                    ),
                },
                Field {
                    name: "sec1",
                    description: Some(
                        "Programmable voltage detector secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec1",
                    ),
                },
                Field {
                    name: "sec2",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> monitor secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec2",
                    ),
                },
                Field {
                    name: "sec3",
                    description: Some(
                        "I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec3",
                    ),
                },
                Field {
                    name: "sec4",
                    description: Some(
                        "Voltage scaling selection secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec4",
                    ),
                },
                Field {
                    name: "sec5",
                    description: Some(
                        "Backup domain secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec5",
                    ),
                },
                Field {
                    name: "sec6",
                    description: Some(
                        "CPU power control secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec6",
                    ),
                },
                Field {
                    name: "sec7",
                    description: Some(
                        "Peripheral voltage monitor secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec7",
                    ),
                },
                Field {
                    name: "wkupsec1",
                    description: Some(
                        "WKUP1 pin secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupsec1",
                    ),
                },
                Field {
                    name: "wkupsec2",
                    description: Some(
                        "WKUP2 pin secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupsec2",
                    ),
                },
                Field {
                    name: "wkupsec3",
                    description: Some(
                        "WKUP3 pin secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupsec3",
                    ),
                },
                Field {
                    name: "wkupsec4",
                    description: Some(
                        "WKUP4 pin secure protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupsec4",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Svmcr1",
            extends: None,
            description: Some(
                "PWR supply voltage monitoring control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vddio4vmen",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub>independent I/O voltage monitor enable.",
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
                    name: "vddio4sv",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub>independent I/O supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4sv",
                    ),
                },
                Field {
                    name: "vddio4rdy",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub>ready.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4rdy",
                    ),
                },
                Field {
                    name: "vddio4vrsel",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> I/O voltage range selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4vrsel",
                    ),
                },
                Field {
                    name: "vddio4vrstby",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> I/O voltage range Standby mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4vrstby",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Svmcr2",
            extends: None,
            description: Some(
                "PWR supply voltage monitoring control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vddio5vmen",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub>independent voltage monitor enable.",
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
                    name: "vddio5sv",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub>independent supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio5sv",
                    ),
                },
                Field {
                    name: "vddio5rdy",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub>ready.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio5rdy",
                    ),
                },
                Field {
                    name: "vddio5vrsel",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> I/O voltage range selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio5vrsel",
                    ),
                },
                Field {
                    name: "vddio5vrstby",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> I/O voltage range Standby mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio5vrstby",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Svmcr3",
            extends: None,
            description: Some(
                "PWR supply voltage monitoring control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vddio2vmen",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub>independent voltage monitor enable.",
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
                    name: "vddio3vmen",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub>independent voltage monitor enable.",
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
                    name: "usb33vmen",
                    description: Some(
                        "V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable.",
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
                    name: "avmen",
                    description: Some(
                        "V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable.",
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
                    name: "vddio2sv",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub>independent supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2sv",
                    ),
                },
                Field {
                    name: "vddio3sv",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub>independent supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3sv",
                    ),
                },
                Field {
                    name: "usb33sv",
                    description: Some(
                        "V less than sub>DD33USB less than /sub>independent supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usb33sv",
                    ),
                },
                Field {
                    name: "asv",
                    description: Some(
                        "V less than sub>DDA18ADC less than /sub>independent supply valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Asv",
                    ),
                },
                Field {
                    name: "vddio2rdy",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub>ready.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2rdy",
                    ),
                },
                Field {
                    name: "vddio3rdy",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub>ready.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3rdy",
                    ),
                },
                Field {
                    name: "usb33rdy",
                    description: Some(
                        "V less than sub>DD33USB less than /sub>ready.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usb33rdy",
                    ),
                },
                Field {
                    name: "ardy",
                    description: Some(
                        "V less than sub>DDA18ADC less than /sub>ready.",
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
                    name: "vddiovrsel",
                    description: Some(
                        "V less than sub>DD less than /sub> I/O voltage range selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddiovrsel",
                    ),
                },
                Field {
                    name: "vddio2vrsel",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub> I/O voltage range selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2vrsel",
                    ),
                },
                Field {
                    name: "vddio3vrsel",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub> I/O voltage range selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3vrsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Voscr",
            extends: None,
            description: Some(
                "PWR voltage scaling control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vos",
                    description: Some(
                        "Voltage scaling selection according to performance.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
                Field {
                    name: "vosrdy",
                    description: Some(
                        "VOS ready bit for V less than sub>CORE less than /sub> voltage scaling output selection.",
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
                    name: "actvos",
                    description: Some(
                        "VOS currently applied for V less than sub>CORE less than /sub> voltage scaling selection.",
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
                    name: "actvosrdy",
                    description: Some(
                        "Voltage level ready bit for currently used ACTVOS.",
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
            name: "Wkupcr",
            extends: None,
            description: Some(
                "PWR wake-up clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupc1",
                    description: Some(
                        "Clear wake-up flag for WKUP1 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupc1",
                    ),
                },
                Field {
                    name: "wkupc2",
                    description: Some(
                        "Clear wake-up flag for WKUP2 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupc2",
                    ),
                },
                Field {
                    name: "wkupc3",
                    description: Some(
                        "Clear wake-up flag for WKUP3 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupc3",
                    ),
                },
                Field {
                    name: "wkupc4",
                    description: Some(
                        "Clear wake-up flag for WKUP4 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupc4",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wkupepr",
            extends: None,
            description: Some(
                "PWR wake-up enable and polarity register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupen1",
                    description: Some(
                        "Enable WKUP1 pin.",
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
                    name: "wkupen2",
                    description: Some(
                        "Enable WKUP2 pin.",
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
                    name: "wkupen3",
                    description: Some(
                        "Enable WKUP3 pin.",
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
                    name: "wkupen4",
                    description: Some(
                        "Enable WKUP4 pin.",
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
                    name: "wkupp1",
                    description: Some(
                        "Wake-up polarity bit for WKUP1 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupp1",
                    ),
                },
                Field {
                    name: "wkupp2",
                    description: Some(
                        "Wake-up polarity bit for WKUP2 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupp2",
                    ),
                },
                Field {
                    name: "wkupp3",
                    description: Some(
                        "Wake-up polarity bit for WKUP3 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupp3",
                    ),
                },
                Field {
                    name: "wkupp4",
                    description: Some(
                        "Wake-up polarity bit for WKUP4 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupp4",
                    ),
                },
                Field {
                    name: "wkuppupd1",
                    description: Some(
                        "Wake-up pull configuration for WKUP1 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wkuppupd1",
                    ),
                },
                Field {
                    name: "wkuppupd2",
                    description: Some(
                        "Wake-up pull configuration for WKUP2 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wkuppupd2",
                    ),
                },
                Field {
                    name: "wkuppupd3",
                    description: Some(
                        "Wake-up pull configuration for WKUP3 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wkuppupd3",
                    ),
                },
                Field {
                    name: "wkuppupd4",
                    description: Some(
                        "Wake-up pull configuration for WKUP4 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wkuppupd4",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wkupsr",
            extends: None,
            description: Some(
                "PWR wake-up status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupf1",
                    description: Some(
                        "Wake-up flag for WKUP1 pin before enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupf1",
                    ),
                },
                Field {
                    name: "wkupf2",
                    description: Some(
                        "Wake-up flag for WKUP2 pin before enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupf2",
                    ),
                },
                Field {
                    name: "wkupf3",
                    description: Some(
                        "Wake-up flag for WKUP3 pin before enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupf3",
                    ),
                },
                Field {
                    name: "wkupf4",
                    description: Some(
                        "Wake-up flag for WKUP4 pin before enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkupf4",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Asv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDA18ADC less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDA18ADC less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cssf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "When written, clear the CPU flags (STOPF, SBF).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ModePdn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Pull-down disabled. The output is in high impedance during the shutdown (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-down enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pdds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Stop mode when device enters deepsleep.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Standby mode when device enters deepsleep.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Popl",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No guaranteed minimum low time.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "~ 1 ms guaranteed minimum low time (1 x 32 LSI cycles).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "~ 2 ms guaranteed minimum low time (2 x 32 LSI cycles).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X1F",
                    description: Some(
                        "~ 31 ms guaranteed minimum low time (31 x 32 LSI cycles).",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Priv0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR1 can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR1 can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR2 can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR2 can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR3 can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR3 can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR4 can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR4 can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VOSCR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VOSCR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv5",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "BDCR1, BDCR2, and DBPCR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BDCR1, BDCR2, and DBPCR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv6",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CPUCR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CPUCR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv7",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SVMCR1, SVMCR2, and SVMCR3 can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SVMCR1, SVMCR2, and SVMCR3 can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pvdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Voltage level on PVD_IN is equal or higher than the internal VREFINT.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Voltage level on PVD_IN is lower than the internal VREFINT.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sbf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "System has not been in Standby mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "System has been in Standby mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR1 can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR1 can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR2 can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR2 can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR3 can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR3 can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CR4 can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CR4 can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VOSCR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VOSCR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec5",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "BDCR1, BDCR2, and DBPCR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BDCR1, BDCR2, and DBPCR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec6",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CPUCR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CPUCR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sec7",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SVMCR1, SVMCR2, and SVMCR3 can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SVMCR1, SVMCR2, and SVMCR3 can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stopf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "System has not been in Stop mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "System has been in Stop mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Svos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SVOS low.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SVOS high (default).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Temph",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Temperature below high threshold level.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Temperature equal or above high threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Templ",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Temperature above low threshold level.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Temperature equal or below low threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usb33rdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DD33USB less than /sub> is below the threshold of the USB33 voltage monitor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DD33USB less than /sub> is equal or above the threshold of the USB33 voltage monitor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usb33sv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DD33USB less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DD33USB less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbath",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>BAT less than /sub> level below high threshold level.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>BAT less than /sub> level equal or above high threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbatl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>BAT less than /sub> level above low threshold level.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>BAT less than /sub> level equal or below low threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vcoreh",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> level below high threshold level, or monitor disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> level equal or above high threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vcorel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> level above low threshold level, or monitor disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> level equal or below low threshold level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vcorells",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> low-voltage threshold 1 selected (VOS low).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDCORE less than /sub> low-voltage threshold 2 selected (VOS high).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2rdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub> is below the threshold of the VDDIO2 voltage monitor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub> is equal or above the threshold of the VDDIO2 voltage monitor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2sv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO2 less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2vrsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "3v3 voltage range selected. If V less than sub>DDIO2 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "1v8 voltage range selected. HSLV_VDDIO2 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO2 less than /sub> is in 3v3 range damages the device.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3rdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub> is below the threshold of the VDDIO3 voltage monitor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub> is equal or above the threshold of the VDDIO3 voltage monitor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3sv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO3 less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3vrsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "3v3 voltage range selected. If V less than sub>DDIO3 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "1v8 voltage range selected. HSLV_VDDIO3 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO3 less than /sub> is in 3v3 range damages the device.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4rdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> is below the threshold of the V less than sub>DDIO4 less than /sub> voltage monitor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> is equal or above the threshold of the V less than sub>DDIO4 less than /sub> voltage monitor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4sv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO4 less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4vrsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "3v3 voltage range selected. If V less than sub>DDIO4 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "1v8 voltage range selected. HSLV_VDDIO4 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO4 less than /sub> is in 3v3 range damages the device.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4vrstby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIO4VRSEL not retained in Standby mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIO4VRSEL retained in Standby mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio5rdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> is below the threshold of the V less than sub>DDIO5 less than /sub> voltage monitor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> is equal or above the threshold of the V less than sub>DDIO5 less than /sub> voltage monitor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio5sv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V less than sub>DDIO5 less than /sub> is valid.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio5vrsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "3v3 voltage range selected. If V less than sub>DDIO5 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "1v8 voltage range selected. HSLV_VDDIO5 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO5 less than /sub> is in 3v3 range damages the device.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio5vrstby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIO5VRSEL not retained in Standby mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIO5VRSEL retained in Standby mod.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddiovrsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "3v3 voltage range selected. If V less than sub>DD less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "1v8 voltage range selected. HSLV_VDD option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DD less than /sub> is in 3v3 range damages the device.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VOS low level (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VOS high level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupc1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Writing 1 clears WKUPF1 in WKUPSR.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupc2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Writing 1 clears WKUPF2 in WKUPSR.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupc3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Writing 1 clears WKUPF3 in WKUPSR.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupc4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Writing 1 clears WKUPF4 in WKUPSR.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupf1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No wake-up event occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "A wake-up event was received from WKUP1 pin.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupf2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No wake-up event occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "A wake-up event was received from WKUP2 pin.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupf3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No wake-up event occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "A wake-up event was received from WKUP3 pin.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupf4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No wake-up event occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "A wake-up event was received from WKUP4 pin.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupp1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Detection on high level (rising edge).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Detection on low level (falling edge).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupp2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Detection on high level (rising edge).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Detection on low level (falling edge).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupp3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Detection on high level (rising edge).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Detection on low level (falling edge).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupp4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Detection on high level (rising edge).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Detection on low level (falling edge).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppriv1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppriv2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppriv3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppriv4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppupd1",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No pulls.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-up.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Pull-down.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Wkuppupd2",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No pulls.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-up.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Pull-down.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Wkuppupd3",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No pulls.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-up.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Pull-down.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Wkuppupd4",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No pulls.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-up.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Pull-down.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Wkupsec1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupsec2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupsec3",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupsec4",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
