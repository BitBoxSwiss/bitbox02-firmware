
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "SYSCFG register block",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "SYSCFG configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                        "SYSCFG configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "scsr",
                    description: Some(
                        "SYSCFG SRAM2 control and status register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "skr",
                    description: Some(
                        "SYSCFG SRAM2 key register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Skr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsccr",
                    description: Some(
                        "SYSCFG TSC comparator register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline0",
                    description: Some(
                        "SYSCFG interrupt line 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline1",
                    description: Some(
                        "SYSCFG interrupt line 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline2",
                    description: Some(
                        "SYSCFG interrupt line 2 status register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline3",
                    description: Some(
                        "SYSCFG interrupt line 3 status register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline4",
                    description: Some(
                        "SYSCFG interrupt line 4 status register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline5",
                    description: Some(
                        "SYSCFG interrupt line 5 status register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline6",
                    description: Some(
                        "SYSCFG interrupt line 6 status register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline7",
                    description: Some(
                        "SYSCFG interrupt line 7 status register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline8",
                    description: Some(
                        "SYSCFG interrupt line 8 status register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline9",
                    description: Some(
                        "SYSCFG interrupt line 9 status register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline10",
                    description: Some(
                        "SYSCFG interrupt line 10 status register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline11",
                    description: Some(
                        "SYSCFG interrupt line 11 status register",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline12",
                    description: Some(
                        "SYSCFG interrupt line 12 status register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline13",
                    description: Some(
                        "SYSCFG interrupt line 13 status register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline14",
                    description: Some(
                        "SYSCFG interrupt line 14 status register",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline15",
                    description: Some(
                        "SYSCFG interrupt line 15 status register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline16",
                    description: Some(
                        "SYSCFG interrupt line 16 status register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline17",
                    description: Some(
                        "SYSCFG interrupt line 17 status register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline18",
                    description: Some(
                        "SYSCFG interrupt line 18 status register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline19",
                    description: Some(
                        "SYSCFG interrupt line 19 status register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline20",
                    description: Some(
                        "SYSCFG interrupt line 20 status register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline21",
                    description: Some(
                        "SYSCFG interrupt line 21 status register",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline22",
                    description: Some(
                        "SYSCFG interrupt line 22 status register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline23",
                    description: Some(
                        "SYSCFG interrupt line 23 status register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline24",
                    description: Some(
                        "SYSCFG interrupt line 24 status register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline25",
                    description: Some(
                        "SYSCFG interrupt line 25 status register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline26",
                    description: Some(
                        "SYSCFG interrupt line 26 status register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline27",
                    description: Some(
                        "SYSCFG interrupt line 27 status register",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline28",
                    description: Some(
                        "SYSCFG interrupt line 28 status register",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline29",
                    description: Some(
                        "SYSCFG interrupt line 29 status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline30",
                    description: Some(
                        "SYSCFG interrupt line 30 status register",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline31",
                    description: Some(
                        "SYSCFG interrupt line 31 status register",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline31",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "SYSCFG configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some(
                        "Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "MemMode",
                    ),
                },
                Field {
                    name: "pa11_rmp",
                    description: Some(
                        "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.\n0: No remap (PA11)\n1: Remap (PA9)",
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
                    name: "pa12_rmp",
                    description: Some(
                        "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.\n0: No remap (PA12)\n1: Remap (PA10)",
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
                    name: "ir_pol",
                    description: Some(
                        "IR output polarity selection",
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
                    name: "ir_mod",
                    description: Some(
                        "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "IrMod",
                    ),
                },
                Field {
                    name: "boosten",
                    description: Some(
                        "I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V).",
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
                    name: "i2c_pb6_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c_pb7_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c_pb8_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c_pb9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c_pa9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c_pa10_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
                    name: "i2c3_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n0: Disable\n1: Enable",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "SYSCFG configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccl",
                    description: Some(
                        "Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.",
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
                    name: "spl",
                    description: Some(
                        "SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.",
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
                    name: "pvdl",
                    description: Some(
                        "PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS[2:0] in the PWR_CR register.",
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
                    name: "eccl",
                    description: Some(
                        "ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.",
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
                    name: "bkpl",
                    description: Some(
                        "Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.",
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
                    name: "bkpf",
                    description: Some(
                        "Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.",
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
                    name: "spf",
                    description: Some(
                        "SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.",
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
            ],
        },
        FieldSet {
            name: "Itline0",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wwdg",
                    description: Some(
                        "Window watchdog interrupt pending flag",
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
            name: "Itline1",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 1 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvdout",
                    description: Some(
                        "PVD supply monitoring interrupt request pending (EXTI line 16).",
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
                    name: "pvmout1",
                    description: Some(
                        "V<sub>DDUSB</sub> supply monitoring interrupt request pending (EXTI line 19)",
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
                    name: "pvmout3",
                    description: Some(
                        "ADC supply monitoring interrupt request pending (EXTI line 20)",
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
                    name: "pvmout4",
                    description: Some(
                        "DAC supply monitoring interrupt request pending (EXTI line 21)",
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
            name: "Itline10",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 10 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch2",
                    description: Some(
                        "DMA1 channel 2 interrupt request pending",
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
                    name: "dma1_ch3",
                    description: Some(
                        "DMA1 channel 3 interrupt request pending",
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
            name: "Itline11",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 11 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmamux",
                    description: Some(
                        "DMAMUX interrupt request pending",
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
                    name: "dma1_ch4",
                    description: Some(
                        "DMA1 channel 4 interrupt request pending",
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
                    name: "dma1_ch5",
                    description: Some(
                        "DMA1 channel 5 interrupt request pending",
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
                    name: "dma1_ch6",
                    description: Some(
                        "DMA1 channel 6 interrupt request pending",
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
                    name: "dma1_ch7",
                    description: Some(
                        "DMA1 channel 7 interrupt request pending",
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
                    name: "dma2_ch1",
                    description: Some(
                        "DMA2 channel 1 interrupt request pending",
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
                    name: "dma2_ch2",
                    description: Some(
                        "DMA2 channel 2 interrupt request pending",
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
                    name: "dma2_ch3",
                    description: Some(
                        "DMA2 channel 3 interrupt request pending",
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
                    name: "dma2_ch4",
                    description: Some(
                        "DMA2 channel 4 interrupt request pending",
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
                    name: "dma2_ch5",
                    description: Some(
                        "DMA2 channel 5 interrupt request pending",
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
            name: "Itline12",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 12 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc",
                    description: Some(
                        "ADC interrupt request pending",
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
                    name: "comp1",
                    description: Some(
                        "Comparator 1 interrupt request pending (EXTI line 17)",
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
                    name: "comp2",
                    description: Some(
                        "Comparator 2 interrupt request pending (EXTI line 18)",
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
            name: "Itline13",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 13 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_ccu",
                    description: Some(
                        "Timer 1 commutation interrupt request pending",
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
                    name: "tim1_trg",
                    description: Some(
                        "Timer 1 trigger interrupt request pending",
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
                    name: "tim1_upd",
                    description: Some(
                        "Timer 1 update interrupt request pending",
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
                    name: "tim1_brk",
                    description: Some(
                        "Timer 1 break interrupt request pending",
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
            name: "Itline14",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 14 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_cc1",
                    description: Some(
                        "Timer 1 capture compare 1 interrupt request pending",
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
                    name: "tim1_cc2",
                    description: Some(
                        "Timer 1 capture compare 2 interrupt request pending",
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
                    name: "tim1_cc3",
                    description: Some(
                        "Timer 1 capture compare 3 interrupt request pending",
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
                    name: "tim1_cc4",
                    description: Some(
                        "Timer 1 capture compare 4 interrupt request pending",
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
            name: "Itline15",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 15 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2",
                    description: Some(
                        "Timer 2 interrupt request pending",
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
            name: "Itline16",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 16 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim3",
                    description: Some(
                        "Timer 3 interrupt request pending",
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
            name: "Itline17",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 17 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim6",
                    description: Some(
                        "Timer 6 interrupt request pending",
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
                    name: "dac",
                    description: Some(
                        "DAC underrun interrupt request pending",
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
                    name: "lptim1",
                    description: Some(
                        "Low-power timer 1 interrupt request pending (EXTI line 29)",
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
            name: "Itline18",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 18 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim7",
                    description: Some(
                        "Timer 7 interrupt request pending",
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
                    name: "lptim2",
                    description: Some(
                        "Low-power timer 2 interrupt request pending (EXTI line 30)",
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
            name: "Itline19",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 19 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim15",
                    description: Some(
                        "Timer 15 interrupt request pending",
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
                    name: "lptim3",
                    description: Some(
                        "Low-power timer 3 interrupt request pending",
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
            name: "Itline2",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 2 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tamp",
                    description: Some(
                        "Tamper interrupt request pending (EXTI line 21)",
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
                    name: "rtc",
                    description: Some(
                        "RTC interrupt request pending (EXTI line 19)",
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
            name: "Itline20",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 20 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim16",
                    description: Some(
                        "Timer 16 interrupt request pending",
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
            name: "Itline21",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 21 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsc_mce",
                    description: Some(
                        "TSC max count error interrupt request pending",
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
                    name: "tsc_eoa",
                    description: Some(
                        "TSC end of acquisition interrupt request pending",
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
            name: "Itline22",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 22 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lcd",
                    description: Some(
                        "LCD interrupt request pending",
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
            name: "Itline23",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 23 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c1",
                    description: Some(
                        "I2C1 interrupt request pending (EXTI line 33)",
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
            name: "Itline24",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 24 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c2",
                    description: Some(
                        "I2C2 interrupt request pending",
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
                    name: "i2c4",
                    description: Some(
                        "I2C4 interrupt request pending",
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
                    name: "i2c3",
                    description: Some(
                        "I2C3 interrupt request pending (EXTI line 23)",
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
            name: "Itline25",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 25 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1",
                    description: Some(
                        "SPI1 interrupt request pending",
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
            name: "Itline26",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 26 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi2",
                    description: Some(
                        "SPI2 interrupt request pending",
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
                    name: "spi3",
                    description: Some(
                        "SPI3 interrupt request pending",
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
            name: "Itline27",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 27 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1",
                    description: Some(
                        "USART1 interrupt request pending, combined with EXTI line 25",
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
            name: "Itline28",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 28 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart2",
                    description: Some(
                        "USART2 interrupt request pending (EXTI line 35)",
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
                    name: "lpuart2",
                    description: Some(
                        "LPUART2 interrupt request pending (EXTI line 31)",
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
            name: "Itline29",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 29 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart3",
                    description: Some(
                        "USART3 interrupt request pending",
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
                    name: "lpuart1",
                    description: Some(
                        "LPUART1 interrupt request pending (EXTI line 30)",
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
            name: "Itline3",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 3 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flash_itf",
                    description: Some(
                        "Flash interface interrupt request pending",
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
                    name: "flash_ecc",
                    description: Some(
                        "Flash interface ECC interrupt request pending",
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
            name: "Itline30",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 30 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart4",
                    description: Some(
                        "USART4 interrupt request pending",
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
                    name: "lpuart3",
                    description: Some(
                        "LPUART3 interrupt request pending (EXTI line 32)",
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
            name: "Itline31",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 31 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rng",
                    description: Some(
                        "RNG interrupt request pending",
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
                    name: "aes",
                    description: Some(
                        "AES interrupt request pending",
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
            name: "Itline4",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 4 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcc",
                    description: Some(
                        "Reset and clock control interrupt request pending",
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
                    name: "crs",
                    description: Some(
                        "CRS interrupt request pending",
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
            name: "Itline5",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 5 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti0",
                    description: Some(
                        "EXTI line 0 interrupt request pending",
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
                    name: "exti1",
                    description: Some(
                        "EXTI line 1 interrupt request pending",
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
            name: "Itline6",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 6 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti2",
                    description: Some(
                        "EXTI line 2 interrupt request pending",
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
                    name: "exti3",
                    description: Some(
                        "EXTI line 3 interrupt request pending",
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
            name: "Itline7",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 7 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti4",
                    description: Some(
                        "EXTI line 4 interrupt request pending",
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
                    name: "exti5",
                    description: Some(
                        "EXTI line 5 interrupt request pending",
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
                    name: "exti6",
                    description: Some(
                        "EXTI line 6 interrupt request pending",
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
                    name: "exti7",
                    description: Some(
                        "EXTI line 7 interrupt request pending",
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
                    name: "exti8",
                    description: Some(
                        "EXTI line 8 interrupt request pending",
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
                    name: "exti9",
                    description: Some(
                        "EXTI line 9 interrupt request pending",
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
                    name: "exti10",
                    description: Some(
                        "EXTI line 10 interrupt request pending",
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
                    name: "exti11",
                    description: Some(
                        "EXTI line 11 interrupt request pending",
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
                    name: "exti12",
                    description: Some(
                        "EXTI line 12 interrupt request pending",
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
                    name: "exti13",
                    description: Some(
                        "EXTI line 13 interrupt request pending",
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
                    name: "exti14",
                    description: Some(
                        "EXTI line 14 interrupt request pending",
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
                    name: "exti15",
                    description: Some(
                        "EXTI line 15 interrupt request pending",
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
            name: "Itline8",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 8 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb",
                    description: Some(
                        "USB interrupt request pending",
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
            name: "Itline9",
            extends: None,
            description: Some(
                "SYSCFG interrupt line 9 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch1",
                    description: Some(
                        "DMA1 channel 1 interrupt request pending",
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
            name: "Scsr",
            extends: None,
            description: Some(
                "SYSCFG SRAM2 control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram2er",
                    description: Some(
                        "SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register.",
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
                    name: "sram2bsy",
                    description: Some(
                        "SRAM2 busy by erase operation",
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
            name: "Skr",
            extends: None,
            description: Some(
                "SYSCFG SRAM2 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY[7:0] Write 0x53 into KEY[7:0] Writing a wrong key reactivates the write protection.",
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
            name: "Tsccr",
            extends: None,
            description: Some(
                "SYSCFG TSC comparator register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "g2_io1",
                    description: Some(
                        "Comparator mode for group 2 on I/O 1",
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
                    name: "g2_io3",
                    description: Some(
                        "Comparator mode for group 2 on I/O 3",
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
                    name: "g4_io3",
                    description: Some(
                        "Comparator mode for group 4 on I/O 3",
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
                    name: "g6_io1",
                    description: Some(
                        "Comparator mode for group 6 on I/O 1",
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
                    name: "g7_io1",
                    description: Some(
                        "Comparator mode for group 7 on I/O 1",
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
                    name: "tsc_ioctrl",
                    description: Some(
                        "I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.",
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
    ],
    enums: &[
        Enum {
            name: "IrMod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TIM16",
                    description: Some(
                        "TIM16",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USART1",
                    description: Some(
                        "USART1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USART2",
                    description: Some(
                        "USART2",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "MemMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSTEM_FLASH",
                    description: Some(
                        "System flash memory mapped at 0x000010000",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "Embedded SRAM mapped at 0x000010000",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
