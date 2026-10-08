
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "PWR Address block.",
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
                    name: "vosr",
                    description: Some(
                        "PWR voltage scaling register.",
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
                        "PWR supply voltage monitoring control register.",
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
                        "PWR wakeup control register 1.",
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
                        "PWR wakeup control register 2.",
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
                        "PWR wakeup control register 3.",
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
                    name: "bdcr",
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
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbpcr",
                    description: Some(
                        "PWR disable Backup domain register.",
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
                    name: "seccfgr",
                    description: Some(
                        "PWR security configuration register.",
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
                        "PWR privilege control register.",
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
                        "PWR status register.",
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
                    description: Some(
                        "PWR supply voltage monitoring status register.",
                    ),
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
                    name: "wuscr",
                    description: Some(
                        "PWR wakeup status clear register.",
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
                        "PWR apply pull configuration register.",
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
                    name: "pucra",
                    description: Some(
                        "PWR port A pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucra",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcra",
                    description: Some(
                        "PWR port A pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcra",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucrb",
                    description: Some(
                        "PWR port B pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrb",
                    description: Some(
                        "PWR port B pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucrc",
                    description: Some(
                        "PWR port C pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrc",
                    description: Some(
                        "PWR port C pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucrd",
                    description: Some(
                        "PWR port D pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrd",
                    description: Some(
                        "PWR port D pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucre",
                    description: Some(
                        "PWR port E pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucre",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcre",
                    description: Some(
                        "PWR port E pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcre",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucrg",
                    description: Some(
                        "PWR port G pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrg",
                    description: Some(
                        "PWR port G pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucrh",
                    description: Some(
                        "PWR port H pull-up control register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcrh",
                    description: Some(
                        "PWR port H pull-down control register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdcrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i3cpucr1",
                    description: Some(
                        "PWR I3C pull-up control register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I3cpucr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i3cpucr2",
                    description: Some(
                        "PWR I3C pull-up control register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I3cpucr2",
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
                        "When this bit is set, the I/O pull-up and pull-down configurations defined in PUCRx and PDCRx are applied. When this bit is cleared, PUCRx and PDCRx are not applied to the I/Os.",
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
            name: "Bdcr",
            extends: None,
            description: Some(
                "PWR Backup domain control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbe",
                    description: Some(
                        "None 0: VBAT battery charging disabled 1: VBAT battery charging enabled.",
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
                    name: "vbrs",
                    description: Some(
                        "None 0: Charge VBAT through a 5 k ohm resistor 1: Charge VBAT through a 1.5 k ohm resistor.",
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
            name: "Cr1",
            extends: None,
            description: Some(
                "PWR control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 000: Stop 0 mode 001: Stop 1 mode 010: Stop 2 mode 011: Stop 3 mode 100-101: Standby mode 110-111: Shutdown mode.",
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
                        "This bit is used to keep the SRAM2 page 1 content in Standby mode. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). 0: SRAM2 page1 content not retained in Standby mode 1: SRAM2 page1 content retained in Standby mode Note: This bit has no effect in Shutdown mode.",
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
                    name: "rrsb2",
                    description: Some(
                        "This bit is used to keep the SRAM2 page 2 content in Standby mode. The SRAM2 page 2 corresponds to the 24 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0x7FFF). 0: SRAM2 page2 content not retained in Standby mode 1: SRAM2 page2 content retained in Standby mode Note: This bit has no effect in Shutdown mode.",
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
                    name: "rrsb3",
                    description: Some(
                        "This bit is used to keep the SRAM2 page 3 content in Standby mode. The SRAM2 page 3 corresponds to the last 32 Kbytes of the SRAM2 (from SRAM2 base address + 0x8000 to SRAM2 base address + 0xFFFF). 0: SRAM2 page3 content not retained in Standby mode 1: SRAM2 page3 content retained in Standby mode Note: This bit has no effect in Shutdown mode.",
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
                        "This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit has effect only when the BOR level 0 is selected and when the device is in Standby mode. This bit must be set to reach the lowest power consumption in Standby mode. 0: BOR level 0 operating in continuous (normal) mode in Standby mode 1: BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode.",
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
                        "This bit is used to reduce the consumption by powering off the SRAM1. 0: SRAM1 powered on 1: SRAM1 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM.",
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
                        "This bit is used to reduce the consumption by powering off the SRAM2. 0: SRAM2 powered on 1: SRAM2 powered off Note: When this bit is cleared to 0, wait for more than 1.6us before accessing the SRAM.",
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
                    name: "sram1pds1",
                    description: Some(
                        "None 0: SRAM1 page 1 content retained in Stop modes 1: SRAM1 page 1 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                        "None 0: SRAM1 page 2 content retained in Stop modes 1: SRAM1 page 2 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                        "None 0: SRAM1 page 3 content retained in Stop modes 1: SRAM1 page 3 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                    name: "sram1pds4",
                    description: Some(
                        "None 0: SRAM1 page 4 content retained in Stop modes 1: SRAM1 page 4 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sram1pds5",
                    description: Some(
                        "None 0: SRAM1 page 5 content retained in Stop modes 1: SRAM1 page 5 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                    name: "sram1pds6",
                    description: Some(
                        "None 0: SRAM1 page 6 content retained in Stop modes 1: SRAM1 page 6 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                    name: "sram1pds7",
                    description: Some(
                        "None 0: SRAM1 page 7 content retained in Stop modes 1: SRAM1 page 7 content lost in Stop modes Note: Page 1 to 2 size is 16 kBytes. Page 3 to 7 size is 32 kBytes.",
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
                    name: "sram2pds1",
                    description: Some(
                        "None 0: SRAM2 page 1 content retained in Stop modes 1: SRAM2 page 1 content lost in Stop modes.",
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
                    name: "sram2pds2",
                    description: Some(
                        "None 0: SRAM2 page 2 content retained in Stop modes 1: SRAM2 page 2 content lost in Stop modes.",
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
                    name: "sram2pds3",
                    description: Some(
                        "None 0: SRAM2 page 3 content retained in Stop modes 1: SRAM2 page 3 content lost in Stop modes.",
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
                    name: "icrampds",
                    description: Some(
                        "None 0: ICACHE SRAM content retained in Stop modes 1: ICACHE SRAM content lost in Stop modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
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
                        "None 0: FDCAN and USB peripherals SRAM content retained in Stop modes 1: FDCAN and USB peripherals SRAM content lost in Stop modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
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
                        "None 0: PKA SRAM content retained in Stop modes 1: PKA SRAM content lost in Stop modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pds",
                    ),
                },
                Field {
                    name: "sramfwu",
                    description: Some(
                        "This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAMs wakeup time increases the wakeup time when exiting Stop 0 and 1 modes, and also increases the GPDMA1 access time to SRAMs during Stop modes. 0: SRAMs enters low-power mode in Stop 0 and Stop 1 modes (source biasing for lower-power consumption). 1: SRAMs remains in normal mode in Stop 0 and Stop 1 modes (higher consumption but no SRAM wakeup time). Note: in case one or several SRAMs are configured to be in power-down in Stop mode, setting SRAMFWU bit has no effect.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                        "This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption. 0: Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption). 1: Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Flashfwu",
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
                    name: "regsel",
                    description: Some(
                        "None 0: LDO selected 1: SMPS selected Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.",
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
                        "None 0: LDO/SMPS fast startup disabled (limited inrush current) 1: LDO/SMPS fast startup enabled.",
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
                "PWR disable Backup domain register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers. 0: Write access to Backup domain disabled 1: Write access to Backup domain enabled.",
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
            name: "I3cpucr1",
            extends: None,
            description: Some(
                "PWR I3C pull-up control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa1_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PA1.",
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
                    name: "pa6_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PA6.",
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
                    name: "pa7_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PA7.",
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
                    name: "pb2_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB2.",
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
                    name: "pb6_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB6.",
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
                    name: "pb8_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB8.",
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
                    name: "pb9_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB9.",
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
                    name: "pb10_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB10.",
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
                    name: "pb12_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB12.",
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
                    name: "pb13_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB13.",
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
                    name: "pb14_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PB14.",
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
            ],
        },
        FieldSet {
            name: "I3cpucr2",
            extends: None,
            description: Some(
                "PWR I3C pull-up control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pc0_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PC0.",
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
                    name: "pc1_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PC1.",
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
                    name: "pd12_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PD12.",
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
                    name: "pd13_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PD13.",
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
                    name: "pg7_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PG7.",
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
                    name: "pg8_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PG8.",
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
                    name: "pg13_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PG13.",
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
                    name: "pg14_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PG14.",
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
                    name: "ph3_i3cpu",
                    description: Some(
                        "When set, the bit activates the I3C pull-up on PH3.",
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
            ],
        },
        FieldSet {
            name: "Pdcra",
            extends: None,
            description: Some(
                "PWR port A pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PA0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PA1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PA2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PA3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd4",
                    description: Some(
                        "When set, each bit activates the pull-down on PA4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PA5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PA6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PA7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PA8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PA9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PA10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PA11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PA12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PA14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            ],
        },
        FieldSet {
            name: "Pdcrb",
            extends: None,
            description: Some(
                "PWR port B pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PB0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PB1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PB2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PB3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PB5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PB6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PB7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PB8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PB9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PB10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PB11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PB12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd13",
                    description: Some(
                        "When set, each bit activates the pull-down on PB13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PB14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd15",
                    description: Some(
                        "When set, each bit activates the pull-down on PB15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pdcrc",
            extends: None,
            description: Some(
                "PWR port C pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PC0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PC1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PC2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PC3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd4",
                    description: Some(
                        "When set, each bit activates the pull-down on PC4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PC5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PC6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PC7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PC8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PC9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PC10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PC11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PC12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd13",
                    description: Some(
                        "When set, each bit activates the pull-down on PC13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PC14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd15",
                    description: Some(
                        "When set, each bit activates the pull-down on PC15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pdcrd",
            extends: None,
            description: Some(
                "PWR port D pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PD0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PD1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PD2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PD3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd4",
                    description: Some(
                        "When set, each bit activates the pull-down on PD4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PD5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PD6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PD7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PD8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PD9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PD10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PD11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PD12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd13",
                    description: Some(
                        "When set, each bit activates the pull-down on PD13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PD14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd15",
                    description: Some(
                        "When set, each bit activates the pull-down on PD15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pdcre",
            extends: None,
            description: Some(
                "PWR port E pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PE0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PE1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PE2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PE3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd4",
                    description: Some(
                        "When set, each bit activates the pull-down on PE4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PE5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PE6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PE7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PE8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PE9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PE10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PE11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PE12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd13",
                    description: Some(
                        "When set, each bit activates the pull-down on PE13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PE14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd15",
                    description: Some(
                        "When set, each bit activates the pull-down on PE15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pdcrg",
            extends: None,
            description: Some(
                "PWR port G pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd2",
                    description: Some(
                        "When set, each bit activates the pull-down on PG2 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PG3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd4",
                    description: Some(
                        "When set, each bit activates the pull-down on PG4 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd5",
                    description: Some(
                        "When set, each bit activates the pull-down on PG5 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd6",
                    description: Some(
                        "When set, each bit activates the pull-down on PG6 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd7",
                    description: Some(
                        "When set, each bit activates the pull-down on PG7 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd8",
                    description: Some(
                        "When set, each bit activates the pull-down on PG8 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd9",
                    description: Some(
                        "When set, each bit activates the pull-down on PG9 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd10",
                    description: Some(
                        "When set, each bit activates the pull-down on PG10 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd11",
                    description: Some(
                        "When set, each bit activates the pull-down on PG11 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd12",
                    description: Some(
                        "When set, each bit activates the pull-down on PG12 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd13",
                    description: Some(
                        "When set, each bit activates the pull-down on PG13 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd14",
                    description: Some(
                        "When set, each bit activates the pull-down on PG14 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd15",
                    description: Some(
                        "When set, each bit activates the pull-down on PG15 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pdcrh",
            extends: None,
            description: Some(
                "PWR port H pull-down control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0",
                    description: Some(
                        "When set, each bit activates the pull-down on PH0 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd1",
                    description: Some(
                        "When set, each bit activates the pull-down on PH1 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pd3",
                    description: Some(
                        "When set, each bit activates the pull-down on PH3 when the APC bit is set in APCR. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "PWR privilege control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "This bit is set and reset by software. It can be written only by a secure privileged access. 0: Read and write to PWR secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR secure functions can be done by privileged access only.",
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
                        "This bit is set and reset by software. It can be written only by privileged access, secure or non-secure. 0: Read and write to PWR non-secure functions can be done by privileged or unprivileged access. 1: Read and write to PWR non-secure functions can be done by privileged access only.",
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
            name: "Pucra",
            extends: None,
            description: Some(
                "PWR port A pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PA0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PA1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PA2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PA3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PA4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PA5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PA6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PA7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PA8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PA9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PA10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PA11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PA12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PA13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PA15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucrb",
            extends: None,
            description: Some(
                "PWR port B pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PB0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PB1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PB2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PB3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PB4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PB5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PB6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PB7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PB8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PB9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PB10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PB11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PB12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PB13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu14",
                    description: Some(
                        "When set, each bit activates the pull-up on PB14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PB15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucrc",
            extends: None,
            description: Some(
                "PWR port C pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PC0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PC1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PC2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PC3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PC4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PC5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PC6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PC7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PC8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PC9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PC10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PC11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PC12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PC13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu14",
                    description: Some(
                        "When set, each bit activates the pull-up on PC14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PC15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucrd",
            extends: None,
            description: Some(
                "PWR port D pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PD0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PD1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PD2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PD3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PD4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PD5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PD6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PD7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PD8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PD9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PD10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PD11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PD12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PD13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu14",
                    description: Some(
                        "When set, each bit activates the pull-up on PD14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PD15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucre",
            extends: None,
            description: Some(
                "PWR port E pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PE0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PE1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PE2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PE3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PE4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PE5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PE6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PE7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PE8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PE9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PE10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PE11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PE12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PE13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu14",
                    description: Some(
                        "When set, each bit activates the pull-up on PE14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PE15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucrg",
            extends: None,
            description: Some(
                "PWR port G pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu2",
                    description: Some(
                        "When set, each bit activates the pull-up on PG2 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD2 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PG3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu4",
                    description: Some(
                        "When set, each bit activates the pull-up on PG4 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD4 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu5",
                    description: Some(
                        "When set, each bit activates the pull-up on PG5 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD5 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu6",
                    description: Some(
                        "When set, each bit activates the pull-up on PG6 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD6 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu7",
                    description: Some(
                        "When set, each bit activates the pull-up on PG7 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD7 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu8",
                    description: Some(
                        "When set, each bit activates the pull-up on PG8 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD8 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu9",
                    description: Some(
                        "When set, each bit activates the pull-up on PG9 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD9 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu10",
                    description: Some(
                        "When set, each bit activates the pull-up on PG10 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD10 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu11",
                    description: Some(
                        "When set, each bit activates the pull-up on PG11 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD11 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu12",
                    description: Some(
                        "When set, each bit activates the pull-up on PG12 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD12 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu13",
                    description: Some(
                        "When set, each bit activates the pull-up on PG13 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD13 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu14",
                    description: Some(
                        "When set, each bit activates the pull-up on PG14 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD14 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu15",
                    description: Some(
                        "When set, each bit activates the pull-up on PG15 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD15 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
            name: "Pucrh",
            extends: None,
            description: Some(
                "PWR port H pull-up control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pu0",
                    description: Some(
                        "When set, each bit activates the pull-up on PH0 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD0 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu1",
                    description: Some(
                        "When set, each bit activates the pull-up on PH1 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD1 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "pu3",
                    description: Some(
                        "When set, each bit activates the pull-up on PH3 when the APC bit is set in APCR. The pull-up is not activated if the corresponding PD3 bit is also set. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.",
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
                    name: "wup1sec",
                    description: Some(
                        "None 0: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP1 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup2sec",
                    description: Some(
                        "None 0: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP2 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup3sec",
                    description: Some(
                        "None 0: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP3 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup4sec",
                    description: Some(
                        "None 0: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP4 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup5sec",
                    description: Some(
                        "None 0: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP5 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup6sec",
                    description: Some(
                        "None 0: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP6 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup7sec",
                    description: Some(
                        "None 0: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP7 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup8sec",
                    description: Some(
                        "None 0: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP8 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup9sec",
                    description: Some(
                        "None 0: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP9 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "wup10sec",
                    description: Some(
                        "None 0: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written with secure or non-secure access. 1: Bits related to the WKUP10 line in WUCR1, WUCR2, WUCR3 and WUSCR can be read and written only with secure access.",
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
                    name: "lpmsec",
                    description: Some(
                        "None 0: CR1, CR2 and CSSF in the SR can be read and written with secure or non-secure access. 1: CR1, CR2, and CSSF in the SR can be read and written only with secure access.",
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
                        "None 0: SVMCR and CR3 can be read and written with secure or non-secure access. 1: SVMCR and CR3 can be read and written only with secure access.",
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
                        "None 0: BDCR and DBPR can be read and written with secure or non-secure access. 1: BDCR and DBPR can be read and written only with secure access.",
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
                        "None 0: APCR can be read and written with secure or non-secure access. 1: APCR can be read and written only with secure access.",
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
                "PWR status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cssf",
                    description: Some(
                        "This bit is protected against non-secure access when LPMSEC=1 in SECCFGR. This bit is protected against unprivileged access when LPMSEC=1 and SPRIV=1 in PRIVCFGR, or when LPMSEC=0 and NSPRIV=1. Writing 1 to this bit clears the STOPF and SBF flags.",
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
                        "This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit. 0: The device did not enter any Stop mode. 1: The device entered a Stop mode.",
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
                        "This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset. 0: The device did not enter Standby mode. 1: The device entered Standby mode.",
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
                "PWR supply voltage monitoring control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "None 0: PVD disabled 1: PVD enabled.",
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
                        "These bits select the voltage threshold detected by the PVD: 000: VPVD0 around 2.0V 001: VPVD1 around 2.2V 010: VPVD2 around 2.4V 011: VPVD3 around 2.5V 100: VPVD4 around 2.6V 101: VPVD5 around 2.8V 110: VPVD6 around 2.9V 111: External input analog voltage PVD_IN (compared internally to VREFINT).",
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
                        "None 0: VDDUSB voltage monitor disabled 1: VDDUSB voltage monitor enabled.",
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
                        "None 0: VDDIO2 voltage monitor disabled 1: VDDIO2 voltage monitor enabled.",
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
                        "None 0: VDDA voltage monitor 1 disabled 1: VDDA voltage monitor 1 enabled.",
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
                        "None 0: VDDA voltage monitor 2 disabled 1: VDDA voltage monitor 2 enabled.",
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
                        "This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not. 0: VDDUSB not present: logical and electrical isolation is applied to ignore this supply. 1: VDDUSB valid.",
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
                        "This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG[15:2]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not. 0: VDDIO2 not present: logical and electrical isolation is applied to ignore this supply. 1: VDDIO2 valid.",
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
                        "This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not. 0: VDDA not present: logical and electrical isolation is applied to ignore this supply. 1: VDDA valid.",
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
            description: Some(
                "PWR supply voltage monitoring status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "regs",
                    description: Some(
                        "None 0: LDO selected 1: SMPS selected.",
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
                        "None 0: VDD is equal or above the PVD threshold selected by PVDLS[2:0]. 1: VDD is below the PVD threshold selected by PVDLS[2:0].",
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
                    name: "vddusbrdy",
                    description: Some(
                        "None 0: VDDUSB is below the threshold of the VDDUSB voltage monitor. 1: VDDUSB is equal or above the threshold of the VDDUSB voltage monitor.",
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
                        "None 0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor. 1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor.",
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
                        "None 0: VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6V).",
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
                        "None 0: VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8V). 1: VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8V).",
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
            name: "Vosr",
            extends: None,
            description: Some(
                "PWR voltage scaling register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "r1en",
                    description: Some(
                        "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 1 disabled 1: Voltage scaling range 1 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN).",
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
                    name: "r2en",
                    description: Some(
                        "This field is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. 0: Voltage scaling range 2 disabled 1: Voltage scaling range 2 enabled Note: R1EN and R2EN must be at opposite value. Any attempt to write R1EN and R2EN to same value is ignored. Modifying R1EN and R2EN is possible only when current range is ready (R1RDY=R1EN and R2RDY=R2EN).",
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
                    name: "boosten",
                    description: Some(
                        "This bit is protected against non-secure access when SYSCLKSEC=1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC=1 in RCC_SECCFGR and SPRIV=1 in PRIVCFGR, or when SYSCLKSEC=0 and NSPRIV=1. This bit must be set in Range 1, and before increasing the system clock frequency above 24 MHz in Range 2. The booster clock must be configured before setting this bit, and must not be disabled as long as the booster is enabled. 0: Booster disabled 1: Booster enabled.",
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
                    name: "r1rdy",
                    description: Some(
                        "None 0: Range 1 not ready: voltage level less than VOS range 1 level 1: Range 1 ready: voltage level greater or equal VOS range 1 level Note: R1RDY and R2RDY cannot be set at the same time.",
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
                    name: "r2rdy",
                    description: Some(
                        "None 0: Range 2 not ready: voltage level less than VOS range 2 level 1: Range 2 ready: voltage level greater or equal VOS range 2 level Note: R1RDY and R2RDY cannot be set at the same time.",
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
                    name: "boostrdy",
                    description: Some(
                        "This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 24 MHz only after this bit is set. Disabling the booster clock when the booster is ready is forbidden. 0: Power booster not ready 1: Power booster ready.",
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
            name: "Wucr1",
            extends: None,
            description: Some(
                "PWR wakeup control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupen1",
                    description: Some(
                        "None 0: Wakeup line WKUP1 disabled 1: Wakeup line WKUP1 enabled.",
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
                    name: "wupen2",
                    description: Some(
                        "None 0: Wakeup line WKUP2 disabled 1: Wakeup line WKUP2 enabled.",
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
                    name: "wupen3",
                    description: Some(
                        "None 0: Wakeup line WKUP3 disabled 1: Wakeup line WKUP3 enabled.",
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
                    name: "wupen4",
                    description: Some(
                        "None 0: Wakeup line WKUP4 disabled 1: Wakeup line WKUP4 enabled.",
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
                    name: "wupen5",
                    description: Some(
                        "None 0: Wakeup line WKUP5 disabled 1: Wakeup line WKUP5 enabled.",
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
                    name: "wupen6",
                    description: Some(
                        "None 0: Wakeup line WKUP6 disabled 1: Wakeup line WKUP6 enabled.",
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
                    name: "wupen7",
                    description: Some(
                        "None 0: Wakeup line WKUP7 disabled 1: Wakeup line WKUP7 enabled.",
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
                    name: "wupen8",
                    description: Some(
                        "None 0: Wakeup line WKUP8 disabled 1: Wakeup line WKUP8 enabled.",
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
                    name: "wupen9",
                    description: Some(
                        "None 0: Wakeup line WKUP9 disabled 1: Wakeup line WKUP9 enabled.",
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
                    name: "wupen10",
                    description: Some(
                        "None 0: Wakeup line WKUP10 disabled 1: Wakeup line WKUP10 enabled.",
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
            name: "Wucr2",
            extends: None,
            description: Some(
                "PWR wakeup control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupp",
                    description: Some(
                        "This bit must be configured when WUPEN1 = 0. It has no effect when WUSEL1 = 11. 0: Detection on high level (rising edge) 1: Detection on low level (falling edge).",
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
                "PWR wakeup control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wusel1",
                    description: Some(
                        "This field must be configured when WUPEN1 = 0. 00: WKUP1_0 01: WKUP1_1 10: WKUP1_2 11: WKUP1_3.",
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
                        "This field must be configured when WUPEN2 = 0. 00: WKUP2_0 01: WKUP2_1 10: WKUP2_2 11: WKUP2_3.",
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
                        "This field must be configured when WUPEN3 = 0. 00: WKUP3_0 01: WKUP3_1 10: WKUP3_2 11: WKUP3_3.",
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
                        "This field must be configured when WUPEN4 = 0. 00: WKUP4_0 01: WKUP4_1 10: WKUP4_2 11: WKUP4_3.",
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
                        "This field must be configured when WUPEN5 = 0. 00: WKUP5_0 01: WKUP5_1 10: WKUP5_2 11: WKUP5_3.",
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
                        "This field must be configured when WUPEN6 = 0. 00: WKUP6_0 01: WKUP6_1 10: WKUP6_2 11: WKUP6_3.",
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
                        "This field must be configured when WUPEN7 = 0. 00: WKUP7_0 01: WKUP7_1 10: WKUP7_2 11: WKUP7_3.",
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
                        "This field must be configured when WUPEN8 = 0. 00: WKUP8_0 01: WKUP8_1 10: WKUP8_2 11: WKUP8_3.",
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
                "PWR wakeup status clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf1",
                    description: Some(
                        "Writing 1 to this bit clears the WUF1 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF2 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF3 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF4 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF5 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF6 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF7 flag in WUSR.",
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
                        "Writing 1 to this bit clears the WUF8 flag in WUSR.",
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
                    name: "cwuf9",
                    description: Some(
                        "Writing 1 to this bit clears the WUF9 flag in WUSR.",
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
                    name: "cwuf10",
                    description: Some(
                        "Writing 1 to this bit clears the WUF10 flag in WUSR.",
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
            name: "Wusr",
            extends: None,
            description: Some(
                "PWR wakeup status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf1",
                    description: Some(
                        "This bit is set when a wakeup event is detected on WKUP1 line. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN1=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP2 line. This bit is cleared by writing 1 in the CWUF2 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN2=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP3 line. This bit is cleared by writing 1 in the CWUF3 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN3=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP4 line. This bit is cleared by writing 1 in the CWUF4 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN4=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP5 line. This bit is cleared by writing 1 in the CWUF5 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN5=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP6 line. This bit is cleared by writing 1 in the CWUF6 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN6=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP7 line. This bit is cleared by writing 1 in the CWUF7 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN7=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                        "This bit is set when a wakeup event is detected on WKUP8 line. This bit is cleared by writing 1 in the CWUF8 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN8=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                    name: "wuf9",
                    description: Some(
                        "This bit is set when a wakeup event is detected on WKUP9 line. This bit is cleared by writing 1 in the CWUF9 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN9=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
                    name: "wuf10",
                    description: Some(
                        "This bit is set when a wakeup event is detected on WKUP10 line. This bit is cleared by writing 1 in the CWUF10 bit of WUSCR when WUSEL different 11, or by hardware when WUPEN10=0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.",
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
    ],
    enums: &[
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
                        "Stop 0 mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOP1",
                    description: Some(
                        "Stop 1 mode.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "STOP2",
                    description: Some(
                        "Stop 2 mode.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STOP3",
                    description: Some(
                        "Stop 3 mode.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "STOP4",
                    description: Some(
                        "Standby mode.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "STOP5",
                    description: Some(
                        "Standby mode.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "STOP6",
                    description: Some(
                        "Shutdown mode.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "STOP7",
                    description: Some(
                        "Shutdown mode.",
                    ),
                    value: 7,
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
                        "Contents retained in Stop modes.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOST",
                    description: Some(
                        "Content lost in Stop modes.",
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
                        "VPVD0 around 2.0V.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V22",
                    description: Some(
                        "VPVD1 around 2.2V.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "V24",
                    description: Some(
                        "VPVD2 around 2.4V.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "V25",
                    description: Some(
                        "VPVD3 around 2.5V.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "V26",
                    description: Some(
                        "VPVD4 around 2.6V.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V28",
                    description: Some(
                        "VPVD5 around 2.8V.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V29",
                    description: Some(
                        "VPVD6 around 2.9V.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PVD_IN",
                    description: Some(
                        "External input analog voltage PVD_IN (compared internally to VREFINT).",
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
                        "LDO selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SMPS",
                    description: Some(
                        "SMPS selected.",
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
                    name: "LOW_POWER",
                    description: Some(
                        "SRAMs enters low-power mode in Stop 0 and Stop 1 modes (source biasing for lower-power consumption).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "SRAMs remains in normal mode in Stop 0 and Stop 1 modes (higher consumption but no SRAM wakeup time).",
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
                        "SRAM1 powered on.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POWERED_OFF",
                    description: Some(
                        "SRAM1 powered off.",
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
                    name: "CHARGE_5K",
                    description: Some(
                        "Charge VBAT through a 5 k ohm resistor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CHARGE_1_5K",
                    description: Some(
                        "Charge VBAT through a 1.5 k ohm resistor.",
                    ),
                    value: 1,
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
            name: "Wusel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "WKUPX_0",
                    description: Some(
                        "WKUPx_0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WKUPX_1",
                    description: Some(
                        "WKUPx_1.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WKUPX_2",
                    description: Some(
                        "WKUPx_2.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WKUPX_3",
                    description: Some(
                        "WKUPx_3.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
