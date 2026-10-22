
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
                    name: "pmcr",
                    description: Some(
                        "PWR power mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pmsr",
                    description: Some(
                        "PWR status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmsr",
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
                    byte_offset: 0x10,
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
                    name: "vossr",
                    description: Some(
                        "PWR voltage scaling status register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vossr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "PWR Backup domain control register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbpcr",
                    description: Some(
                        "PWR Backup domain control register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "bdsr",
                    description: Some(
                        "PWR Backup domain status register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "ucpdr",
                    description: Some(
                        "PWR USB Type-C power delivery register.",
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
                    name: "sccr",
                    description: Some(
                        "PWR supply configuration control register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmcr",
                    description: Some(
                        "PWR voltage monitor control register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usbscr",
                    description: Some(
                        "PWR USB supply control register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Usbscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmsr",
                    description: Some(
                        "PWR voltage monitor status register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wuscr",
                    description: Some(
                        "PWR wakeup status clear register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                    name: "wusr",
                    description: Some(
                        "PWR wakeup status register.",
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
                    name: "wucr",
                    description: Some(
                        "PWR wakeup configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioretr",
                    description: Some(
                        "PWR I/O retention register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioretr",
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
                    byte_offset: 0x100,
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
                    byte_offset: 0x104,
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
            name: "Bdcr",
            extends: None,
            description: Some(
                "PWR Backup domain control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bren",
                    description: Some(
                        "Backup RAM retention in Standby and V_BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V_BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in. Run and Stop modes. However its content is lost in Standby and V_BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V_BAT modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Retention",
                    ),
                },
                Field {
                    name: "monen",
                    description: Some(
                        "Backup domain voltage and temperature monitoring enable.",
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
                    name: "vbe",
                    description: Some(
                        "V_BAT charging enable Note: Reset only by POR,.",
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
                    name: "vbrs",
                    description: Some(
                        "V_BAT charging resistor selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
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
                "PWR Backup domain status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brrdy",
                    description: Some(
                        "backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.",
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
                    name: "vbatl",
                    description: Some(
                        "V_BAT level monitoring versus low threshold.",
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
                    name: "vbath",
                    description: Some(
                        "V_BAT level monitoring versus high threshold.",
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
                    name: "templ",
                    description: Some(
                        "temperature level monitoring versus low threshold.",
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
                    name: "temph",
                    description: Some(
                        "temperature level monitoring versus high threshold.",
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
            name: "Dbpcr",
            extends: None,
            description: Some(
                "PWR Backup domain control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write. access. This bit must be set to enable write access to these registers.",
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
            name: "Ioretr",
            extends: None,
            description: Some(
                "PWR I/O retention register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioreten",
                    description: Some(
                        "IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.",
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
                    name: "jtagioreten",
                    description: Some(
                        "IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode.",
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
            name: "Pmcr",
            extends: None,
            description: Some(
                "PWR power mode control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "low-power mode selection This bit defines the Deepsleep mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpms",
                    ),
                },
                Field {
                    name: "svos",
                    description: Some(
                        "system Stop mode voltage scaling selection These bits control the V_CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Svos",
                    ),
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.",
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
                    name: "flps",
                    description: Some(
                        "Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PowerModeInStopMode",
                    ),
                },
                Field {
                    name: "booste",
                    description: Some(
                        "analog switch V_BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V_DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V_DD supply voltage can be monitored through the PVD and the PLS bits.",
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
                    name: "avd_ready",
                    description: Some(
                        "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V_DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored. (ALS bits).",
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
                    name: "ethernetso",
                    description: Some(
                        "ETHERNET RAM shut-off in Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShutOff",
                    ),
                },
                Field {
                    name: "sram3so",
                    description: Some(
                        "AHB SRAM3 shut-off in Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShutOff",
                    ),
                },
                Field {
                    name: "sram2_16so",
                    description: Some(
                        "AHB SRAM2 16-Kbyte shut-off in Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShutOff",
                    ),
                },
                Field {
                    name: "sram2_48so",
                    description: Some(
                        "AHB SRAM2 48-Kbyte shut-off in Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShutOff",
                    ),
                },
                Field {
                    name: "sram1so",
                    description: Some(
                        "AHB SRAM1 shut-off in Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShutOff",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pmsr",
            extends: None,
            description: Some(
                "PWR status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stopf",
                    description: Some(
                        "Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit.",
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
                    name: "sbf",
                    description: Some(
                        "System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit.",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "PWR privilege configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "PWR secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access.",
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
                        "PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.",
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
            name: "Sccr",
            extends: None,
            description: Some(
                "PWR supply configuration control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bypass",
                    description: Some(
                        "power management unit bypass.",
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
                    name: "ldoen",
                    description: Some(
                        "LDO enable The value is set by hardware when the package uses the LDO regulator.",
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
                    name: "smpsen",
                    description: Some(
                        "SMPS enable The value is set by hardware when the package uses the SMPS regulator.",
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
                    name: "wupsec",
                    description: Some(
                        "WUPx secure protection.",
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
                    name: "retsec",
                    description: Some(
                        "retention secure protection.",
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
                    name: "lpmsec",
                    description: Some(
                        "low-power modes secure protection.",
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
                    name: "scmsec",
                    description: Some(
                        "supply configuration and monitoring secure protection.",
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
                        "backup domain secure protection.",
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
                    name: "vusbsec",
                    description: Some(
                        "voltage USB secure protection.",
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
            name: "Ucpdr",
            extends: None,
            description: Some(
                "PWR USB Type-C power delivery register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucpd_dbdis",
                    description: Some(
                        "USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).",
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
                        "USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.",
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
            name: "Usbscr",
            extends: None,
            description: Some(
                "PWR USB supply control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb33den",
                    description: Some(
                        "V_DDUSB voltage level detector enable.",
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
                    name: "usb33sv",
                    description: Some(
                        "independent USB supply valid This bit is used to validate the V_DDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V_DDUSB is not always present in the application, the V_DDUSB voltage monitor can be used to determine whether this supply is ready or not.",
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
            ],
        },
        FieldSet {
            name: "Vmcr",
            extends: None,
            description: Some(
                "PWR voltage monitor control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "PVD enable.",
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
                    name: "pls",
                    description: Some(
                        "programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pls",
                    ),
                },
                Field {
                    name: "avden",
                    description: Some(
                        "peripheral voltage monitor on V_DDA enable.",
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
                    name: "als",
                    description: Some(
                        "analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Als",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vmsr",
            extends: None,
            description: Some(
                "PWR voltage monitor status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avdo",
                    description: Some(
                        "analog voltage detector output on V_DDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.",
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
                    name: "vddio2rdy",
                    description: Some(
                        "voltage detector output on V_DDIO2 This bit is set and cleared by hardware.",
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
                    name: "pvdo",
                    description: Some(
                        "programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.",
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
                    name: "usb33rdy",
                    description: Some(
                        "V_DDUSB ready.",
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
                        "voltage scaling selection according to performance These bits control the V_CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vossr",
            extends: None,
            description: Some(
                "PWR voltage scaling status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vosrdy",
                    description: Some(
                        "Ready bit for V_CORE voltage scaling output selection.",
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
                    name: "actvosrdy",
                    description: Some(
                        "Voltage level ready for currently used VOS.",
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
                    name: "actvos",
                    description: Some(
                        "voltage output scaling currently applied to V_CORE This field provides the last VOS value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wucr",
            extends: None,
            description: Some(
                "PWR wakeup configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupen",
                    description: Some(
                        "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.",
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
                    name: "wupp",
                    description: Some(
                        "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wupp",
                    ),
                },
                Field {
                    name: "wuppupd",
                    description: Some(
                        "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wuppupd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wuscr",
            extends: None,
            description: Some(
                "PWR wakeup status clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some(
                        "clear wakeup pin flag for WUFx These bits are always read as 0.",
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
            name: "Wusr",
            extends: None,
            description: Some(
                "PWR wakeup status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some(
                        "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.",
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
    ],
    enums: &[
        Enum {
            name: "Als",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LEVEL0",
                    description: Some(
                        "AVD level0 (VAVD0 ~ 1.7 V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "AVD level1 (VAVD1 ~ 2.1 V)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "AVD level2 (VAVD2 ~ 2.5 V)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "AVD level3 (VAVD3 ~ 2.8 V)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpms",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP",
                    description: Some(
                        "Keeps Stop mode when entering DeepSleep.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STANDBY",
                    description: Some(
                        "Allows Standby mode when entering DeepSleep.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "LEVEL0",
                    description: Some(
                        "PVD level0 (VPVD0 ~ 1.95 V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "PVD level1 (VPVD1 ~ 2.10 V)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "PVD level2 (VPVD2 ~ 2.25 V)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "PVD level3 (VPVD3 ~ 2.40 V)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LEVEL4",
                    description: Some(
                        "PVD level4 (VPVD4 ~ 2.55 V)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LEVEL5",
                    description: Some(
                        "PVD level5 (VPVD5 ~ 2.70 V)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LEVEL6",
                    description: Some(
                        "PVD level6 (VPVD6 ~ 2.85 V)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PVDIN_PIN",
                    description: Some(
                        "PVD_IN pin",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "PowerModeInStopMode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Remains in normal mode when the system enters Stop mode (quick restart time).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_POWER",
                    description: Some(
                        "Enters low-power mode when the system enters Stop mode (low-power consumption).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Retention",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOST",
                    description: Some(
                        "Content is lost.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRESERVED",
                    description: Some(
                        "Content is preserved.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ShutOff",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "KEPT",
                    description: Some(
                        "Content is kept.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOST",
                    description: Some(
                        "Content is lost.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Svos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SCALE5",
                    description: Some(
                        "SVOS5 scale 5",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SCALE4",
                    description: Some(
                        "SVOS4 scale 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SCALE3",
                    description: Some(
                        "SVOS3 scale 3 (default)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Vbrs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "R5K_OHM",
                    description: Some(
                        "Charge VBAT through a 5 kΩ resistor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "R1_5K_OHM",
                    description: Some(
                        "Charge VBAT through a 1.5 kΩ resistor.",
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
                    name: "SCALE3",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SCALE2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "SCALE1",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "SCALE0",
                    description: None,
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
                        "detection on high level (rising edge)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "detection on low level (falling edge)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wuppupd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_PULL_UP",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "PULL_UP",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "PULL_DOWN",
                    description: None,
                    value: 2,
                },
            ],
        },
    ],
};
