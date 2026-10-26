
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
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
                        "configuration register 1",
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
                    name: "vrefbuf_csr",
                    description: Some(
                        "VREFBUF control and status register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VrefbufCsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vrefbuf_ccr",
                    description: Some(
                        "VREFBUF calibration control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VrefbufCcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline0",
                    description: Some(
                        "interrupt line 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 2 status register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 3 status register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 4 status register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 5 status register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 6 status register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 7 status register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 8 status register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 9 status register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 10 status register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 11 status register",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 12 status register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 13 status register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 14 status register",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 15 status register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 16 status register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 17 status register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 18 status register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 19 status register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 20 status register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 21 status register",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 22 status register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 23 status register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 24 status register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 25 status register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 26 status register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 27 status register",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 28 status register",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 29 status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 30 status register",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "interrupt line 31 status register",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some(
                        "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details.",
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
                        "PA11 pin remapping\r This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.",
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
                        "PA12 pin remapping\r This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.",
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
                        "IR Modulation Envelope signal selection.",
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
                    name: "boosten",
                    description: Some(
                        "I/O analog switch voltage booster enable",
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
                    name: "ucpd1_strobe",
                    description: Some(
                        "Strobe signal bit for UCPD1",
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
                    name: "ucpd2_strobe",
                    description: Some(
                        "Strobe signal bit for UCPD2",
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
                    name: "i2c_pbx_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits",
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
                    name: "i2c1_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C1",
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
                    name: "i2c2_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C2",
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
                    name: "i2c_pax_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex-M0+ LOCKUP bit enable bit",
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
                    name: "sram_parity_lock",
                    description: Some(
                        "SRAM parity lock bit",
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
                    name: "pvd_lock",
                    description: Some(
                        "PVD lock enable bit",
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
                    name: "ecc_lock",
                    description: Some(
                        "ECC error lock bit",
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
                    name: "sram_pef",
                    description: Some(
                        "SRAM parity error flag",
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
                    name: "pa1_cden",
                    description: Some(
                        "PA1_CDEN",
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
                    name: "pa3_cden",
                    description: Some(
                        "PA3_CDEN",
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
                    name: "pa5_cden",
                    description: Some(
                        "PA5_CDEN",
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
                    name: "pa6_cden",
                    description: Some(
                        "PA6_CDEN",
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
                    name: "pa13_cden",
                    description: Some(
                        "PA13_CDEN",
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
                    name: "pb0_cden",
                    description: Some(
                        "PB0_CDEN",
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
                    name: "pb1_cden",
                    description: Some(
                        "PB1_CDEN",
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
                    name: "pb2_cden",
                    description: Some(
                        "PB2_CDEN",
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
            name: "Itline0",
            extends: None,
            description: Some(
                "interrupt line 0 status register",
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
                "interrupt line 1 status register",
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
            ],
        },
        FieldSet {
            name: "Itline10",
            extends: None,
            description: Some(
                "interrupt line 10 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch2",
                    description: Some(
                        "DMA1_CH1",
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
                        "DMA1_CH3",
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
                "interrupt line 11 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmamux",
                    description: Some(
                        "DMAMUX",
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
                        "DMA1_CH4",
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
                        "DMA1_CH5",
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
                        "DMA1_CH6",
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
                        "DMA1_CH7",
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
            name: "Itline12",
            extends: None,
            description: Some(
                "interrupt line 12 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc",
                    description: Some(
                        "ADC",
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
                        "COMP1",
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
                        "COMP2",
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
                "interrupt line 13 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_ccu",
                    description: Some(
                        "TIM1_CCU",
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
                        "TIM1_TRG",
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
                        "TIM1_UPD",
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
                        "TIM1_BRK",
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
                "interrupt line 14 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_cc",
                    description: Some(
                        "TIM1_CC",
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
            name: "Itline15",
            extends: None,
            description: Some(
                "interrupt line 15 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2",
                    description: Some(
                        "TIM2",
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
                "interrupt line 16 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim3",
                    description: Some(
                        "TIM3",
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
                "interrupt line 17 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim6",
                    description: Some(
                        "TIM6",
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
                        "DAC",
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
                        "LPTIM1",
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
                "interrupt line 18 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim7",
                    description: Some(
                        "TIM7",
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
                        "LPTIM2",
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
                "interrupt line 19 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim14",
                    description: Some(
                        "TIM14",
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
            name: "Itline2",
            extends: None,
            description: Some(
                "interrupt line 2 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tamp",
                    description: Some(
                        "TAMP",
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
                        "RTC",
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
                "interrupt line 20 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim15",
                    description: Some(
                        "TIM15",
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
                "interrupt line 21 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim16",
                    description: Some(
                        "TIM16",
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
            name: "Itline22",
            extends: None,
            description: Some(
                "interrupt line 22 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim17",
                    description: Some(
                        "TIM17",
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
                "interrupt line 23 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c1",
                    description: Some(
                        "I2C1",
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
                "interrupt line 24 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c2",
                    description: Some(
                        "I2C2",
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
            name: "Itline25",
            extends: None,
            description: Some(
                "interrupt line 25 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1",
                    description: Some(
                        "SPI1",
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
                "interrupt line 26 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi2",
                    description: Some(
                        "SPI2",
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
            name: "Itline27",
            extends: None,
            description: Some(
                "interrupt line 27 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1",
                    description: Some(
                        "USART1",
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
                "interrupt line 28 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart2",
                    description: Some(
                        "USART2",
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
            name: "Itline29",
            extends: None,
            description: Some(
                "interrupt line 29 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart3",
                    description: None,
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
                    name: "usart4",
                    description: None,
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
                    name: "usart5",
                    description: None,
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
                    name: "usart6",
                    description: None,
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
            name: "Itline3",
            extends: None,
            description: Some(
                "interrupt line 3 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flash_itf",
                    description: Some(
                        "FLASH_ITF",
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
                        "FLASH_ECC",
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
                "interrupt line 30 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cec",
                    description: Some(
                        "CEC",
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
            name: "Itline31",
            extends: None,
            description: Some(
                "interrupt line 31 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rng",
                    description: Some(
                        "RNG",
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
                        "AES",
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
                "interrupt line 4 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcc",
                    description: Some(
                        "RCC",
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
            name: "Itline5",
            extends: None,
            description: Some(
                "interrupt line 5 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti0",
                    description: Some(
                        "EXTI0",
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
                        "EXTI1",
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
                "interrupt line 6 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti2",
                    description: Some(
                        "EXTI2",
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
                        "EXTI3",
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
                "interrupt line 7 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti4",
                    description: Some(
                        "EXTI4",
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
                        "EXTI5",
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
                        "EXTI6",
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
                        "EXTI7",
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
                        "EXTI8",
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
                        "EXTI9",
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
                        "EXTI10",
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
                        "EXTI11",
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
                        "EXTI12",
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
                        "EXTI13",
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
                        "EXTI14",
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
                        "EXTI15",
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
                "interrupt line 8 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucpd1",
                    description: Some(
                        "UCPD1",
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
                    name: "ucpd2",
                    description: Some(
                        "UCPD2",
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
                    name: "usb",
                    description: Some(
                        "USB",
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
            name: "Itline9",
            extends: None,
            description: Some(
                "interrupt line 9 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch1",
                    description: Some(
                        "DMA1_CH1",
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
            name: "VrefbufCcr",
            extends: None,
            description: Some(
                "VREFBUF calibration control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trim",
                    description: Some(
                        "Trimming code These bits are automatically initialized after reset with the trimming value stored in the Flash memory during the production test. Writing into these bits allows to tune the internal reference buffer voltage.",
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
            name: "VrefbufCsr",
            extends: None,
            description: Some(
                "VREFBUF control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "envr",
                    description: Some(
                        "Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.",
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
                    name: "hiz",
                    description: Some(
                        "High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration.",
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
                    name: "vrs",
                    description: Some(
                        "Voltage reference scale. 0: Voltage reference set to VREF_OUT1 (around 2.048 V). 1: Voltage reference set to VREF_OUT2 (around 2.5 V). ",
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
                    name: "vrr",
                    description: Some(
                        "Voltage reference buffer ready",
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
    ],
    enums: &[
        Enum {
            name: "MemMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAIN_FLASH",
                    description: Some(
                        "Main Flash memory mapped at address 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSTEM_FLASH",
                    description: Some(
                        "System Flash memory mapped at address 0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAIN_FLASH_ALT",
                    description: Some(
                        "Main Flash memory mapped at address 0 (alternate encoding)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "Embedded SRAM mapped at address 0",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
