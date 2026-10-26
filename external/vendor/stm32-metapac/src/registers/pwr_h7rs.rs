
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
                    name: "sr1",
                    description: Some(
                        "PWR control status register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr1",
                    description: Some(
                        "PWR control status register 1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr2",
                    description: Some(
                        "PWR control register 2.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr3",
                    description: Some(
                        "PWR CPU control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr4",
                    description: Some(
                        "PWR control status register 4.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupcr",
                    description: Some(
                        "PWR wakeup clear register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "wkupfr",
                    description: Some(
                        "PWR wakeup flag register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkupfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupepr",
                    description: Some(
                        "PWR wakeup enable and polarity register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "ucpdr",
                    description: Some(
                        "PWR USB Type-C and Power Delivery register.",
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
                    name: "apcr",
                    description: Some(
                        "PWR apply pull configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "pucrn",
                    description: Some(
                        "PWR port N pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrn",
                    description: Some(
                        "PWR port N pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucro",
                    description: Some(
                        "PWR port O pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucro",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcro",
                    description: Some(
                        "PWR port O pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcro",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrp",
                    description: Some(
                        "PWR port P pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdr1",
                    description: Some(
                        "PWR debug register 1.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdr1",
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
                "PWR apply pull configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "apc",
                    description: Some(
                        "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx, PDCRx registers are applied in Standby mode even after wakeup until APC bit is reset to 0. When this bit is cleared, the I/O pull-up or pull-down configurations defined in PO5_PUPD, PN7_PUPD bits and PUCRx and PDCRx registers are not applied in Standby mode and IO becomes Hi-Z.",
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
                    name: "pn7_pupd",
                    description: Some(
                        "Port N bit 7 pull-up/down configuration When this bit is set, a weak pull-up or pull-down resistor is applied on PN7 following inverse logic applied on PN6. If the PUN6 bit in PWR_PUCRN register is set and APC bit is set the week pull-down is applied on PN7. If the PDN6 bit in PWR_PDCRN register is set and APC bit is set the week pull-up is applied on PN7.",
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
                    name: "po5_pupd",
                    description: Some(
                        "Port O bit 5 pull-up/down configuration When this bit is set, a weak pull-up or pull down resistor is applied on PO5 following inverse logic applied on PO4. If the PUO4 bit in PWR_PUCRO register is set and APC bit is set the week pull-down is applied on PO5. If the PDO4 bit in PWR_PDCRO register is set and APC bit is set the week pull-up is applied on PO5..",
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
                    name: "i3cpb6_pu",
                    description: Some(
                        "Port PB6 I3C pull-up bit When I3C is used on PB6, when set, this bit activates the pull-up on I3C1_SCL (PB6) in standby mode.",
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
                    name: "i3cpb7_pu",
                    description: Some(
                        "Port PB7 I3C pull-up bit When I3C is used on PB7, when set, this bit activates the pull-up on I3C1_SDA (PB7) in standby mode.",
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
                    name: "i3cpb8_pu",
                    description: Some(
                        "Port PB8 I3C pull-up bit When I3C is used on PB8, when set, this bit activates the pull-up on I3C1_SCL (PB8) in standby mode.",
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
                    name: "i3cpb9_pu",
                    description: Some(
                        "Port PB9 I3C pull-up bit When I3C is used on PB9, when set, this bit activates the pull-up on I3C1_SDA (PB9) in standby mode.",
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
            name: "Cr1",
            extends: None,
            description: Some(
                "PWR control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svos",
                    description: Some(
                        "System Stop mode voltage scaling selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Svos",
                    ),
                },
                Field {
                    name: "pvde",
                    description: Some(
                        "Programmable voltage detector enable.",
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
                    name: "pls",
                    description: Some(
                        "Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pls",
                    ),
                },
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in the PWR_CSR1 register, are protected against parasitic write access. This bit must be set to enable write access to these registers.",
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
                    name: "flps",
                    description: Some(
                        "Flash low-power mode in Stop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when device is in Stop mode. consumption).",
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
                    name: "rlpsn",
                    description: Some(
                        "RAM low power mode disable in STOP. When set the RAMs will not enter to low power mode when the system enters to STOP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rlpsn",
                    ),
                },
                Field {
                    name: "booste",
                    description: Some(
                        "analog switch VBoost control This bit enables the booster to guarantee the analog switch AC performance when the VDD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The VDD supply voltage can be monitored through the PVD and the PLS bits.",
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
                    name: "avdready",
                    description: Some(
                        "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected VDDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_CSR1 register) after setting the AVDEN bit and selecting the supply level to be monitored (ALS bits).",
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
                    name: "avden",
                    description: Some(
                        "Peripheral voltage monitor on VDDA enable.",
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
                    name: "als",
                    description: Some(
                        "Analog voltage detector level selection These bits select the voltage threshold detected by the AVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
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
            name: "Csr1",
            extends: None,
            description: Some(
                "PWR control status register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bren",
                    description: Some(
                        "Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes.",
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
                        "V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1).",
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
                    name: "brrdy",
                    description: Some(
                        "Backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.",
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
                        "V<sub>BAT</sub> level monitoring versus low threshold.",
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
                        "V<sub>BAT</sub> level monitoring versus high threshold.",
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
                        "Temperature level monitoring versus low threshold.",
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
                        "Temperature level monitoring versus high threshold.",
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
            name: "Csr2",
            extends: None,
            description: Some(
                "PWR control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bypass",
                    description: Some(
                        "Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.",
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
                        "Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.",
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
                    name: "sden",
                    description: Some(
                        "SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.",
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
                    name: "sdexthp",
                    description: Some(
                        "SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.",
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
                    name: "sdlevel",
                    description: Some(
                        "SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdlevel",
                    ),
                },
                Field {
                    name: "vbe",
                    description: Some(
                        "VBAT charging enable.",
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
                        "VBAT charging resistor selection.",
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
                Field {
                    name: "xspicap1",
                    description: Some(
                        "XSPI port 1 capacitor control bits see the product datasheet for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Xspicap",
                    ),
                },
                Field {
                    name: "xspicap2",
                    description: Some(
                        "XSPI port 2 capacitor control bits see the product datasheet for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Xspicap",
                    ),
                },
                Field {
                    name: "en_xspim1",
                    description: Some(
                        "EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit.",
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
                    name: "en_xspim2",
                    description: Some(
                        "EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used.",
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
                    name: "sdextrdy",
                    description: Some(
                        "SMPS step-down converter external supply ready This bit is set by hardware to indicate that the external supply from the SMPS step-down converter is ready.",
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
                    name: "usb33den",
                    description: Some(
                        "VDD33_USB voltage level detector enable.",
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
                    name: "usbregen",
                    description: Some(
                        "USB regulator enable.",
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
                    name: "usb33rdy",
                    description: Some(
                        "USB supply ready.",
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
                    name: "usbhsregen",
                    description: Some(
                        "USB HS regulator enable.",
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
            name: "Csr3",
            extends: None,
            description: Some(
                "PWR CPU control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdds",
                    description: Some(
                        "Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode.",
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
                        "Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.",
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
                    name: "stopf",
                    description: Some(
                        "STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU CSSF bit.",
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
                    name: "sbf",
                    description: Some(
                        "System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU CSSF bit.",
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
            name: "Csr4",
            extends: None,
            description: Some(
                "PWR control status register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vos",
                    description: Some(
                        "Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details.",
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
                        "VOS Ready bit.",
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
            name: "Pdcrn",
            extends: None,
            description: Some(
                "PWR port N pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdn0",
                    description: Some(
                        "Port N pull-down bit 0 When set activates the pull-down on PN0 when the APC bit is set in PWR_APCR.",
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
                    name: "pdn1",
                    description: Some(
                        "Port N pull-down bit 1 When set activates the pull-down on PN1 when the APC bit is set in PWR_APCR.",
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
                    name: "pdn2n5",
                    description: Some(
                        "Port N PN2 to PN5 pull-down activation When set, four pull-down resistors are activated on PN2 to PN5 when the APC bit is set in PWR_APCR.",
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
                    name: "pdn6",
                    description: Some(
                        "Port N pull-down bit 6 When set activates the pull-down on PN6 when the APC bit is set in PWR_APCR.",
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
                    name: "pdn8n11",
                    description: Some(
                        "Port N - PN8 to PN11 pull-down activation When set, four pull-down resistors are activated on PN8 to PN11 when the APC bit is set in PWR_APCR.",
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
                    name: "pdn12",
                    description: Some(
                        "Port N pull-down bit 12 When set activates the pull-down on PN12 when the APC bit is set in PWR_APCR.",
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
            name: "Pdcro",
            extends: None,
            description: Some(
                "PWR port O pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdo0",
                    description: Some(
                        "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.",
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
                    name: "pdo1",
                    description: Some(
                        "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.",
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
                    name: "pdo2",
                    description: Some(
                        "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.",
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
                    name: "pdo3",
                    description: Some(
                        "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.",
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
                    name: "pdo4",
                    description: Some(
                        "Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.",
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
            name: "Pdcrp",
            extends: None,
            description: Some(
                "PWR port P pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdp0p3",
                    description: Some(
                        "Port P0-P3 pull-down activation When set, four pull-down resistors are activated on P0 to P3 when the APC bit is set in PWR_APCR.",
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
                    name: "pdp4p7",
                    description: Some(
                        "Port P4-P7 pull-down activation When set, four pull-down resitors are activated on P4 to P7 when the APC bit is set in PWR_APCR.",
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
                    name: "pdp8p11",
                    description: Some(
                        "Port P8-P11 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.",
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
                    name: "pdp12p15",
                    description: Some(
                        "Port P12-P15 pull-down activation When set, four pull-down resistors are activated on P8 to P11 when the APC bit is set in PWR_APCR.",
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
            name: "Pdr1",
            extends: None,
            description: Some(
                "PWR debug register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "unlocked",
                    description: Some(
                        "Debug Register Unlocked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Unlocked",
                    ),
                },
                Field {
                    name: "sdfpwmen",
                    description: Some(
                        "Step down converter force PWM mode.",
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
                    name: "sync_adc",
                    description: Some(
                        "(Non-User bit).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SyncAdc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pucrn",
            extends: None,
            description: Some(
                "PWR port N pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pun1",
                    description: Some(
                        "Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set.",
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
                    name: "pun6",
                    description: Some(
                        "Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set.",
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
                    name: "pun12",
                    description: Some(
                        "Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set.",
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
            name: "Pucro",
            extends: None,
            description: Some(
                "PWR port O pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "puo0",
                    description: Some(
                        "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.",
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
                    name: "puo1",
                    description: Some(
                        "(n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.",
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
                    name: "puo4",
                    description: Some(
                        "Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set.",
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
            name: "Sr1",
            extends: None,
            description: Some(
                "PWR control status register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "actvos",
                    description: Some(
                        "VOS currently applied for V<sub>CORE</sub> voltage scaling selection. These bit reflect the last VOS value applied to the PMU.",
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
                    name: "actvosrdy",
                    description: Some(
                        "Voltage levels ready bit for currently used ACTVOS and SDHILEVEL This bit is set to 1 by hardware when the voltage regulator and the SMPS step-down converter are both disabled and Bypass mode is selected in PWR control register 2 (PWR_CSR2).",
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
                    name: "pvdo",
                    description: Some(
                        "Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. PLS[2:0] bits. bits. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.",
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
                    name: "avdo",
                    description: Some(
                        "Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Avdo",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ucpdr",
            extends: None,
            description: Some(
                "PWR USB Type-C and Power Delivery register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucpd_dbdis",
                    description: Some(
                        "UCPD dead battery disable.",
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
                        "UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.",
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
            name: "Wkupcr",
            extends: None,
            description: Some(
                "PWR wakeup clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupc",
                    description: Some(
                        "Clear Wakeup pin flag for WKUP1 These bits are always read as 0.",
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
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wkupepr",
            extends: None,
            description: Some(
                "PWR wakeup enable and polarity register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupen",
                    description: Some(
                        "Enable Wakeup Pin WKUPn, (n = 4, 3, 2, 1) Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn bit) when WKUPn pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn selects falling edge.",
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
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "wkupp",
                    description: Some(
                        "Wakeup pin polarity bit for WKUPn, (n = 4, 3, 2, 1) These bits define the polarity used for event detection on WKUPn external wakeup pin.",
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
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wkupp",
                    ),
                },
                Field {
                    name: "wkuppupd",
                    description: Some(
                        "Wakeup pin pull configuration",
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
                                len: 4,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wkuppupd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wkupfr",
            extends: None,
            description: Some(
                "PWR wakeup flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wkupf",
                    description: Some(
                        "Wakeup pin WKUP flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPC1 bit in the PWR wakeup clear register (PWR_WKUPCR).",
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
                                len: 4,
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
                    name: "LEVEL1",
                    description: Some(
                        "AVD level 1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "AVD level 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "AVD level 3.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL4",
                    description: Some(
                        "AVD level 4.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Avdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ABOVE_OR_EQUAL",
                    description: Some(
                        "VDDA is equal or higher than the AVD threshold selected with the ALS[1:0] bits.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BELOW",
                    description: Some(
                        "VDDA is lower than the AVD threshold selected with the ALS[1:0] bits.",
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
                    name: "STOP",
                    description: Some(
                        "Stop mode when device enters Deepsleep.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STANDBY",
                    description: Some(
                        "Standby mode when device enters Deepsleep.",
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
                    name: "LEVEL1",
                    description: Some(
                        "PVD level 1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "PVD level 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "PVD level 3.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL4",
                    description: Some(
                        "PVD level 4.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LEVEL5",
                    description: Some(
                        "PVD level 5.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LEVEL6",
                    description: Some(
                        "PVD level 6.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LEVEL7",
                    description: Some(
                        "PVD level 7.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "External voltage level on PVD_IN pin, compared to internal VREFINT level.",
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
                        "VDD or PVD_IN voltage is equal or higher than the PVD threshold selected through the.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BELOW",
                    description: Some(
                        "VDD or PVD_IN voltage is lower than the PVD threshold selected through the PLS[2:0].",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rlpsn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW_POWER",
                    description: Some(
                        "RAM enters to low power mode when system enters to STOP.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "RAM remains in normal mode when system enters to STOP.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdlevel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "V1_8",
                    description: None,
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
                    name: "LOW",
                    description: Some(
                        "SVOS Low.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "SVOS High (default).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SyncAdc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FREE_RUNNING",
                    description: Some(
                        "SD_Converter clock free running.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNCHRONIZED",
                    description: Some(
                        "SD_Converter clock synchronised to ADC.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Unlocked",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOCKED",
                    description: Some(
                        "accessed locked: key was not written and after each register write access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "UNLOCKED",
                    description: Some(
                        "after key 0xCAFECAFE was written in this register.",
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
                    name: "OHM5K",
                    description: Some(
                        "Charge VBAT through a 5 k resistor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OHM1_5K",
                    description: Some(
                        "Charge VBAT through a 1.5 k resistor.",
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
                    name: "LOW",
                    description: Some(
                        "VOS Low level (default).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "VOS High level.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkupp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Detection on high level (rising edge).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Detection on low level (falling edge).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuppupd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_PULL",
                    description: Some(
                        "No pull-up.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PULL_UP",
                    description: Some(
                        "Pull-up.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PULL_DOWN",
                    description: Some(
                        "Pull-down.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Xspicap",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "XSPI Capacitor OFF (default) note: to confirm with analog design.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONE_THIRD",
                    description: Some(
                        "XSPI Capacitor set to 1/3.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TWO_THIRDS",
                    description: Some(
                        "XSPI Capacitor set to 2/3.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "XSPI Capacitor set to full capacitance.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
