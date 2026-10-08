
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
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
                        "control register 2",
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
                        "control register 3",
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
                    name: "vosr",
                    description: Some(
                        "voltage scaling register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vosr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "svmcr",
                    description: Some(
                        "supply voltage monitoring control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr1",
                    description: Some(
                        "wakeup control register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr2",
                    description: Some(
                        "wakeup control register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr3",
                    description: Some(
                        "wakeup control register 3",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr1",
                    description: Some(
                        "Backup domain control register 1",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                        "Backup domain control register 2",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                        "disable Backup domain register",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "ucpdr",
                    description: Some(
                        "USB Type-C™ and Power Delivery register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ucpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "security configuration register",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                        "privilege control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x38,
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
                    name: "svmsr",
                    description: None,
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdsr",
                    description: Some(
                        "Backup domain status register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wusr",
                    description: Some(
                        "wakeup status register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wusr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wuscr",
                    description: Some(
                        "wakeup status clear register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wuscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apcr",
                    description: Some(
                        "apply pull configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucr",
                    description: Some(
                        "Power Port pull-up control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcr",
                    description: Some(
                        "Power Port pull-down control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Apcr",
            extends: None,
            description: Some(
                "apply pull configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "apc",
                    description: Some(
                        "Apply pull-up and pull-down configuration\r When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os.",
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
            name: "Bdcr1",
            extends: None,
            description: Some(
                "Backup domain control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bren",
                    description: Some(
                        "Backup RAM retention in Standby and VBAT modes\r When this bit is set, the backup RAM content is kept in Standby and VBAT modes.\r If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS.\r Note: Backup RAM cannot be preserved in Shutdown mode.",
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
                    name: "monen",
                    description: Some(
                        "Backup domain voltage and temperature monitoring enable",
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
            name: "Bdcr2",
            extends: None,
            description: Some(
                "Backup domain control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbe",
                    description: Some(
                        "VBAT charging enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbe",
                    ),
                },
                Field {
                    name: "vbrs",
                    description: Some(
                        "VBAT charging resistor selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbrs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bdsr",
            extends: None,
            description: Some(
                "Backup domain status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbath",
                    description: Some(
                        "Backup domain voltage level monitoring versus high threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
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
                        "Temperature level monitoring versus low threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
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
                        "Temperature level monitoring versus high threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
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
            name: "Cr1",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "Low-power mode selection\r These bits select the low-power mode entered when the CPU enters the Deepsleep mode.\r 10x: Standby mode (Standby mode also entered if LPMS=11X in CR1\r with BREN=1 in BDCR1)\r 11x: Shutdown mode if BREN = 0 in BDCR1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpms",
                    ),
                },
                Field {
                    name: "rrsb1",
                    description: Some(
                        "SRAM2 page 1 retention in Stop 3 and Standby modes\r This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2\r (from SRAM2 base address to SRAM2 base address + 0x1FFF).\r Note: This bit has no effect in Shutdown mode.",
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
                    name: "rrsb2",
                    description: Some(
                        "SRAM2 page 2 retention in Stop 3 and Standby modes\r This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2\r (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF).\r Note: This bit has no effect in Shutdown mode.",
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
                    name: "ulpmen",
                    description: Some(
                        "BOR ultra-low power mode\r This bit is used to reduce the consumption by configuring the BOR in discontinuous mode.\r This bit must be set to reach the lowest power consumption in the low-power modes.",
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
                    name: "sram1pd",
                    description: Some(
                        "SRAM1 power down\r This bit is used to reduce the consumption by powering off the SRAM1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampd",
                    ),
                },
                Field {
                    name: "sram2pd",
                    description: Some(
                        "SRAM2 power down\r This bit is used to reduce the consumption by powering off the SRAM2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampd",
                    ),
                },
                Field {
                    name: "sram3pd",
                    description: Some(
                        "SRAM3 power down\r This bit is used to reduce the consumption by powering off the SRAM3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampd",
                    ),
                },
                Field {
                    name: "sram4pd",
                    description: Some(
                        "SRAM4 power down\r This bit is used to reduce the consumption by powering off the SRAM4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1pds1",
                    description: Some(
                        "SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram1pds2",
                    description: Some(
                        "SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram1pds3",
                    description: Some(
                        "SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram2pds1",
                    description: Some(
                        "SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2)\r Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in CR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram2pds2",
                    description: Some(
                        "SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2)\r Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in CR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram4pds",
                    description: Some(
                        "SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "icrampds",
                    description: Some(
                        "ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "dc1rampds",
                    description: Some(
                        "DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "dma2drampds",
                    description: Some(
                        "DMA2D SRAM power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "prampds",
                    description: Some(
                        "FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop0,1,2,3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "pkarampds",
                    description: Some(
                        "PKA SRAM power-down",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram4fwu",
                    description: Some(
                        "SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes\r This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sramfwu",
                    ),
                },
                Field {
                    name: "flashfwu",
                    description: Some(
                        "Flash memory fast wakeup from Stop 0 and Stop 1 modes\r This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes.\r When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Flashfwu",
                    ),
                },
                Field {
                    name: "sram3pds1",
                    description: Some(
                        "SRAM3 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds2",
                    description: Some(
                        "SRAM3 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds3",
                    description: Some(
                        "SRAM3 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds4",
                    description: Some(
                        "SRAM3 page 4 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds5",
                    description: Some(
                        "SRAM3 page 5 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds6",
                    description: Some(
                        "SRAM3 page 6 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds7",
                    description: Some(
                        "SRAM3 page 7 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram3pds8",
                    description: Some(
                        "SRAM3 page 8 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "srdrun",
                    description: Some(
                        "SmartRun domain in Run mode",
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
            name: "Cr3",
            extends: None,
            description: Some(
                "control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "regsel",
                    description: Some(
                        "Regulator selection\r Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Regsel",
                    ),
                },
                Field {
                    name: "fsten",
                    description: Some(
                        "Fast soft start",
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
            ],
        },
        FieldSet {
            name: "Dbpcr",
            extends: None,
            description: Some(
                "disable Backup domain register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable Backup domain write protection\r In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.",
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
            name: "Pcr",
            extends: None,
            description: Some(
                "Power Port pull control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "p",
                    description: Some(
                        "Port pull bit y (y=0..15)",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "privilege control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "secure functions privilege configuration\r This bit is set and reset by software. It can be written only by a secure privileged access.",
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
                    name: "nspriv",
                    description: Some(
                        "non-secure functions privilege configuration\r This bit is set and reset by software. It can be written only by privileged access, secure or non-secure.",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "security configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wup1sec",
                    description: Some(
                        "WUP1 secure protection",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lpmsec",
                    description: Some(
                        "Low-power modes secure protection",
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
                    name: "vdmsec",
                    description: Some(
                        "Voltage detection and monitoring secure protection",
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
                    name: "vbsec",
                    description: Some(
                        "Backup domain secure protection",
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
                    name: "apcsec",
                    description: Some(
                        "Pull-up/pull-down secure protection",
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
                    name: "cssf",
                    description: Some(
                        "Clear Stop and Standby flags\r This bit is protected against non-secure access when LPMSEC=1 in SECCFGR.\r This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1.\r Writing 1 to this bit clears the STOPF and SBF flags.",
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
                    name: "stopf",
                    description: Some(
                        "Stop flag\r This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.",
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
                    name: "sbf",
                    description: Some(
                        "Standby flag\r This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.",
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
            ],
        },
        FieldSet {
            name: "Svmcr",
            extends: None,
            description: Some(
                "supply voltage monitoring control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "Power voltage detector enable",
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
                    name: "pvdls",
                    description: Some(
                        "Power voltage detector level selection\r These bits select the voltage threshold detected by the power voltage detector:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pvdls",
                    ),
                },
                Field {
                    name: "uvmen",
                    description: Some(
                        "VDDUSB independent USB voltage monitor enable",
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
                    name: "io2vmen",
                    description: Some(
                        "VDDIO2 independent I/Os voltage monitor enable",
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
                    name: "avm1en",
                    description: Some(
                        "VDDA independent analog supply voltage monitor 1 enable (1.6V threshold)",
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
                    name: "avm2en",
                    description: Some(
                        "VDDA independent analog supply voltage monitor 2 enable (1.8V threshold)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usv",
                    description: Some(
                        "VDDUSB independent USB supply valid",
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
                    name: "io2sv",
                    description: Some(
                        "VDDIO2 independent I/Os supply valid\r This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose.\r Setting this bit is mandatory to use PG[15:2]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.",
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
                    name: "asv",
                    description: Some(
                        "VDDA independent analog supply valid",
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
            ],
        },
        FieldSet {
            name: "Svmsr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "regs",
                    description: Some(
                        "Regulator selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Regsel",
                    ),
                },
                Field {
                    name: "pvdo",
                    description: Some(
                        "VDD voltage detector output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pvdo",
                    ),
                },
                Field {
                    name: "actvosrdy",
                    description: Some(
                        "Voltage level ready for currently used VOS",
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
                    name: "actvos",
                    description: Some(
                        "VOS currently applied to VCORE\r This field provides the last VOS value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Actvos",
                    ),
                },
                Field {
                    name: "vddusbrdy",
                    description: Some(
                        "VDDUSB ready",
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
                    name: "vddio2rdy",
                    description: Some(
                        "VDDIO2 ready",
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
                    name: "vdda1rdy",
                    description: Some(
                        "VDDA ready versus 1.6V voltage monitor",
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
                    name: "vdda2rdy",
                    description: Some(
                        "VDDA ready versus 1.8V voltage monitor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ucpdr",
            extends: None,
            description: Some(
                "USB Type-C™ and Power Delivery register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucpd_dbdis",
                    description: Some(
                        "UCPD dead battery disable\r After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).",
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
                    name: "ucpd_stby",
                    description: Some(
                        "UCPD Standby mode\r When set, this bit is used to memorize the UCPD configuration in Standby mode.\r This bit must be written to 1 just before entering Standby mode when using UCPD.\r It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.",
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
            name: "Vosr",
            extends: None,
            description: Some(
                "voltage scaling register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usbboostrdy",
                    description: Some(
                        "OTG_HS EPOD booster ready\r This bit is set to one by hardware when the power booster startup time is reached. The OTG_HS clock can be provided only after this bit is set.",
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
                    name: "boostrdy",
                    description: Some(
                        "EPOD booster ready\r This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.",
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
                    name: "vosrdy",
                    description: Some(
                        "Ready bit for VCORE voltage scaling output selection",
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
                    name: "vos",
                    description: Some(
                        "Voltage scaling range selection\r This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
                Field {
                    name: "boosten",
                    description: Some(
                        "EPOD booster enable",
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
                    name: "usbpwren",
                    description: Some(
                        "OTG_HS power enable",
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
                    name: "usbboosten",
                    description: Some(
                        "OTG_HS EPOD booster enable",
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
                    name: "vdd11usbdis",
                    description: Some(
                        "OTG_HS VDD11USB disable",
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
            ],
        },
        FieldSet {
            name: "Wucr1",
            extends: None,
            description: Some(
                "wakeup control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupen",
                    description: Some(
                        "Wakeup pin WKUP1 enable",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wucr2",
            extends: None,
            description: Some(
                "wakeup control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupp",
                    description: Some(
                        "Wakeup pin WKUP1 polarity.\r This bit must be configured when WUPEN1 = 0.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wupp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wucr3",
            extends: None,
            description: Some(
                "wakeup control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wusel1",
                    description: Some(
                        "Wakeup pin WKUP1 selection\r This field must be configured when WUPEN1 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel2",
                    description: Some(
                        "Wakeup pin WKUP2 selection\r This field must be configured when WUPEN2 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel3",
                    description: Some(
                        "Wakeup pin WKUP3 selection\r This field must be configured when WUPEN3 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel4",
                    description: Some(
                        "Wakeup pin WKUP4 selection\r This field must be configured when WUPEN4 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel5",
                    description: Some(
                        "Wakeup pin WKUP5 selection\r This field must be configured when WUPEN5 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel6",
                    description: Some(
                        "Wakeup pin WKUP6 selection\r This field must be configured when WUPEN6 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel7",
                    description: Some(
                        "Wakeup pin WKUP7 selection\r This field must be configured when WUPEN7 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel8",
                    description: Some(
                        "Wakeup pin WKUP8 selection\r This field must be configured when WUPEN8 = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wuscr",
            extends: None,
            description: Some(
                "wakeup status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf1",
                    description: Some(
                        "Wakeup flag 1\r Writing 1 to this bit clears the WUF1 flag in WUSR.",
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
                    name: "cwuf2",
                    description: Some(
                        "Wakeup flag 2\r Writing 1 to this bit clears the WUF2 flag in WUSR.",
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
                    name: "cwuf3",
                    description: Some(
                        "Wakeup flag 3\r Writing 1 to this bit clears the WUF3 flag in WUSR.",
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
                    name: "cwuf4",
                    description: Some(
                        "Wakeup flag 4\r Writing 1 to this bit clears the WUF4 flag in WUSR.",
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
                    name: "cwuf5",
                    description: Some(
                        "Wakeup flag 5\r Writing 1 to this bit clears the WUF5 flag in WUSR.",
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
                    name: "cwuf6",
                    description: Some(
                        "Wakeup flag 6\r Writing 1 to this bit clears the WUF6 flag in WUSR.",
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
                    name: "cwuf7",
                    description: Some(
                        "Wakeup flag 7\r Writing 1 to this bit clears the WUF7 flag in WUSR.",
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
                    name: "cwuf8",
                    description: Some(
                        "Wakeup flag 8\r Writing 1 to this bit clears the WUF8 flag in WUSR.",
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
            name: "Wusr",
            extends: None,
            description: Some(
                "wakeup status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf1",
                    description: Some(
                        "Wakeup flag 1\r This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1=0.",
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
                    name: "wuf2",
                    description: Some(
                        "Wakeup flag 2\r This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2=0.",
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
                    name: "wuf3",
                    description: Some(
                        "Wakeup flag 3\r This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3=0.",
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
                    name: "wuf4",
                    description: Some(
                        "Wakeup flag 4\r This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4=0.",
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
                    name: "wuf5",
                    description: Some(
                        "Wakeup flag 5\r This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5=0.",
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
                    name: "wuf6",
                    description: Some(
                        "Wakeup flag 6\r This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6=0.\r If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                    name: "wuf7",
                    description: Some(
                        "Wakeup flag 7\r This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7=0.\r If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                    name: "wuf8",
                    description: Some(
                        "Wakeup flag 8\r This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8=0.\r If WUSEL=11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
    ],
    enums: &[
        Enum {
            name: "Actvos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE4",
                    description: Some(
                        "Range 4 (lowest power)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE3",
                    description: Some(
                        "Range 3",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Range 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Range 1 (highest frequency)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Flashfwu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW_POWER",
                    description: Some(
                        "Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "STOP0",
                    description: Some(
                        "Stop 0 mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOP1",
                    description: Some(
                        "Stop 1 mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "STOP2",
                    description: Some(
                        "Stop 2 mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STOP3",
                    description: Some(
                        "Stop 3 mode",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RETAINED",
                    description: Some(
                        "Content retained in Stop modes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOST",
                    description: Some(
                        "Content lost in Stop modes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pvdls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "V20",
                    description: Some(
                        "VPVD0 around 2.0 V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V22",
                    description: Some(
                        "VPVD1 around 2.2 V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "V24",
                    description: Some(
                        "VPVD2 around 2.4 V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "V25",
                    description: Some(
                        "VPVD3 around 2.5 V",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "V26",
                    description: Some(
                        "VPVD4 around 2.6 V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V28",
                    description: Some(
                        "VPVD5 around 2.8 V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V29",
                    description: Some(
                        "VPVD6 around 2.9 V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PVD_IN",
                    description: Some(
                        "External input analog voltage PVD_IN (compared internally to VREFINT)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pvdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ABOVE_OR_EQUAL",
                    description: Some(
                        "VDD is equal or above the PVD threshold selected by PVDLS[2:0].",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BELOW",
                    description: Some(
                        "VDD is below the PVD threshold selected by PVDLS[2:0].",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Regsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LDO",
                    description: Some(
                        "LDO selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SMPS",
                    description: Some(
                        "SMPS selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sramfwu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SRAM4 enters low-power mode in Stop 0, 1 and 2 modes (source biasing for lower-power consumption).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SRAM4 remains in normal mode in Stop 0, 1 and 2 modes (higher consumption but no SRAM4 wakeup time).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Srampd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "POWERED_ON",
                    description: Some(
                        "SRAM1 powered on",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POWERED_OFF",
                    description: Some(
                        "SRAM1 powered off",
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
                        "Temperature < high threshold",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Temperature ≥ high threshold",
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
                        "Temperature > low threshold",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Temperature ≤ low threshold",
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
                        "Backup domain voltage level < high threshold",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Backup domain voltage level ≥ high threshold",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VBAT battery charging disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VBAT battery charging enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbrs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Charge VBAT through a 5 kΩ resistor",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Charge VBAT through a 1.5 kΩ resistor",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE4",
                    description: Some(
                        "Range 4 (lowest power)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE3",
                    description: Some(
                        "Range 3",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Range 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Range 1 (highest frequency). This value cannot be written when VCOREMEN = 1 in TAMP_OR register.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Wupp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Detection on high level (rising edge)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Detection on low level (falling edge)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wusel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "WKUP7_0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "WKUP7_1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "WKUP7_2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "WKUP7_3",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
