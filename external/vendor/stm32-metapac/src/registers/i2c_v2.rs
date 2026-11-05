
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "I2c",
            extends: None,
            description: Some(
                "Inter-integrated circuit",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Control register 1",
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
                        "Control register 2",
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
                    name: "oar1",
                    description: Some(
                        "Own address register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oar2",
                    description: Some(
                        "Own address register 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timingr",
                    description: Some(
                        "Timing register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timingr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timeoutr",
                    description: Some(
                        "Timeout register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timeoutr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Interrupt and Status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "icr",
                    description: Some(
                        "Interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pecr",
                    description: Some(
                        "PEC register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxdr",
                    description: Some(
                        "Receive data register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txdr",
                    description: Some(
                        "Transmit data register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pe",
                    description: Some(
                        "Peripheral enable",
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
                    name: "txie",
                    description: Some(
                        "TX Interrupt enable",
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
                    name: "rxie",
                    description: Some(
                        "RX Interrupt enable",
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
                    name: "addrie",
                    description: Some(
                        "Address match interrupt enable (slave only)",
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
                    name: "nackie",
                    description: Some(
                        "Not acknowledge received interrupt enable",
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
                    name: "stopie",
                    description: Some(
                        "STOP detection Interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Transfer Complete interrupt enable",
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
                    name: "errie",
                    description: Some(
                        "Error interrupts enable",
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
                    name: "dnf",
                    description: Some(
                        "Digital noise filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Dnf",
                    ),
                },
                Field {
                    name: "anfoff",
                    description: Some(
                        "Analog noise filter OFF",
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
                    name: "txdmaen",
                    description: Some(
                        "DMA transmission requests enable",
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
                    name: "rxdmaen",
                    description: Some(
                        "DMA reception requests enable",
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
                    name: "sbc",
                    description: Some(
                        "Slave byte control",
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
                    name: "nostretch",
                    description: Some(
                        "Clock stretching disable",
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
                    name: "gcen",
                    description: Some(
                        "General call enable",
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
                    name: "smbhen",
                    description: Some(
                        "SMBus Host address enable",
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
                    name: "smbden",
                    description: Some(
                        "SMBus Device Default address enable",
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
                    name: "alerten",
                    description: Some(
                        "SMBUS alert enable",
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
                    name: "pecen",
                    description: Some(
                        "PEC enable",
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
            name: "Cr2",
            extends: None,
            description: Some(
                "Control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sadd",
                    description: Some(
                        "Slave address bit (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir",
                    description: Some(
                        "Transfer direction (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "add10",
                    description: Some(
                        "10-bit addressing mode (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Addmode",
                    ),
                },
                Field {
                    name: "head10r",
                    description: Some(
                        "10-bit address header only read direction (master receiver mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Headr",
                    ),
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start generation",
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
                    name: "stop",
                    description: Some(
                        "Stop generation (master mode)",
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
                    name: "nack",
                    description: Some(
                        "NACK generation (slave mode)",
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
                    name: "nbytes",
                    description: Some(
                        "Number of bytes",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reload",
                    description: Some(
                        "NBYTES reload mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Reload",
                    ),
                },
                Field {
                    name: "autoend",
                    description: Some(
                        "Automatic end mode (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Autoend",
                    ),
                },
                Field {
                    name: "pecbyte",
                    description: Some(
                        "Packet error checking byte",
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
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some(
                "Interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrcf",
                    description: Some(
                        "Address Matched flag clear",
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
                    name: "nackcf",
                    description: Some(
                        "Not Acknowledge flag clear",
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
                    name: "stopcf",
                    description: Some(
                        "Stop detection flag clear",
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
                    name: "berrcf",
                    description: Some(
                        "Bus error flag clear",
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
                    name: "arlocf",
                    description: Some(
                        "Arbitration lost flag clear",
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
                    name: "ovrcf",
                    description: Some(
                        "Overrun/Underrun flag clear",
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
                    name: "peccf",
                    description: Some(
                        "PEC Error flag clear",
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
                    name: "timoutcf",
                    description: Some(
                        "Timeout detection flag clear",
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
                    name: "alertcf",
                    description: Some(
                        "Alert flag clear",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "Interrupt and Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txe",
                    description: Some(
                        "Transmit data register empty (transmitters)",
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
                    name: "txis",
                    description: Some(
                        "Transmit interrupt status (transmitters)",
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
                    name: "rxne",
                    description: Some(
                        "Receive data register not empty (receivers)",
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
                    name: "addr",
                    description: Some(
                        "Address matched (slave mode)",
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
                    name: "nackf",
                    description: Some(
                        "Not acknowledge received flag",
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
                    name: "stopf",
                    description: Some(
                        "Stop detection flag",
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
                    name: "tc",
                    description: Some(
                        "Transfer Complete (master mode)",
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
                    name: "tcr",
                    description: Some(
                        "Transfer Complete Reload",
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
                    name: "berr",
                    description: Some(
                        "Bus error",
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
                    name: "arlo",
                    description: Some(
                        "Arbitration lost",
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
                    name: "ovr",
                    description: Some(
                        "Overrun/Underrun (slave mode)",
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
                    name: "pecerr",
                    description: Some(
                        "PEC Error in reception",
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
                    name: "timeout",
                    description: Some(
                        "Timeout or t_low detection flag",
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
                    name: "alert",
                    description: Some(
                        "SMBus alert",
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
                    name: "busy",
                    description: Some(
                        "Bus busy",
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
                    name: "dir",
                    description: Some(
                        "Transfer direction (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "addcode",
                    description: Some(
                        "Address match code (Slave mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Oar1",
            extends: None,
            description: Some(
                "Own address register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oa1",
                    description: Some(
                        "Interface address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oa1mode",
                    description: Some(
                        "Own Address 1 10-bit mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Addmode",
                    ),
                },
                Field {
                    name: "oa1en",
                    description: Some(
                        "Own Address 1 enable",
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
            name: "Oar2",
            extends: None,
            description: Some(
                "Own address register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oa2",
                    description: Some(
                        "Interface address",
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
                Field {
                    name: "oa2msk",
                    description: Some(
                        "Own Address 2 masks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Oamsk",
                    ),
                },
                Field {
                    name: "oa2en",
                    description: Some(
                        "Own Address 2 enable",
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
            name: "Pecr",
            extends: None,
            description: Some(
                "PEC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pec",
                    description: Some(
                        "Packet error checking register",
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
            name: "Rxdr",
            extends: None,
            description: Some(
                "Receive data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxdata",
                    description: Some(
                        "8-bit receive data",
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
            name: "Timeoutr",
            extends: None,
            description: Some(
                "Timeout register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timeouta",
                    description: Some(
                        "Bus timeout A",
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
                    name: "tidle",
                    description: Some(
                        "Idle clock timeout detection",
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
                    name: "timouten",
                    description: Some(
                        "Clock timeout enable",
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
                    name: "timeoutb",
                    description: Some(
                        "Bus timeout B",
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
                Field {
                    name: "texten",
                    description: Some(
                        "Extended clock timeout enable",
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
            name: "Timingr",
            extends: None,
            description: Some(
                "Timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scll",
                    description: Some(
                        "SCL low period (master mode)",
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
                Field {
                    name: "sclh",
                    description: Some(
                        "SCL high period (master mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdadel",
                    description: Some(
                        "Data hold time",
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
                    name: "scldel",
                    description: Some(
                        "Data setup time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "presc",
                    description: Some(
                        "Timing prescaler",
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
            name: "Txdr",
            extends: None,
            description: Some(
                "Transmit data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txdata",
                    description: Some(
                        "8-bit transmit data",
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
    ],
    enums: &[
        Enum {
            name: "Addmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BIT7",
                    description: Some(
                        "7-bit addressing mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "10-bit addressing mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Autoend",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AUTOMATIC",
                    description: Some(
                        "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "WRITE",
                    description: Some(
                        "Write transfer, slave enters receiver mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "READ",
                    description: Some(
                        "Read transfer, slave enters transmitter mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dnf",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NO_FILTER",
                    description: Some(
                        "Digital filter disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FILTER1",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 1 tI2CCLK",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FILTER2",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 2 tI2CCLK",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FILTER3",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 3 tI2CCLK",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FILTER4",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 4 tI2CCLK",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FILTER5",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 5 tI2CCLK",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FILTER6",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 6 tI2CCLK",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FILTER7",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 7 tI2CCLK",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FILTER8",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 8 tI2CCLK",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FILTER9",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 9 tI2CCLK",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FILTER10",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 10 tI2CCLK",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FILTER11",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 11 tI2CCLK",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FILTER12",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 12 tI2CCLK",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FILTER13",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 13 tI2CCLK",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FILTER14",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 14 tI2CCLK",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FILTER15",
                    description: Some(
                        "Digital filter enabled and filtering capability up to 15 tI2CCLK",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Headr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "COMPLETE",
                    description: Some(
                        "The master sends the complete 10 bit slave address read sequence",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PARTIAL",
                    description: Some(
                        "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Oamsk",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_MASK",
                    description: Some(
                        "No mask",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASK1",
                    description: Some(
                        "OA2[1] is masked and don’t care. Only OA2[7:2] are compared",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MASK2",
                    description: Some(
                        "OA2[2:1] are masked and don’t care. Only OA2[7:3] are compared",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MASK3",
                    description: Some(
                        "OA2[3:1] are masked and don’t care. Only OA2[7:4] are compared",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MASK4",
                    description: Some(
                        "OA2[4:1] are masked and don’t care. Only OA2[7:5] are compared",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MASK5",
                    description: Some(
                        "OA2[5:1] are masked and don’t care. Only OA2[7:6] are compared",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "MASK6",
                    description: Some(
                        "OA2[6:1] are masked and don’t care. Only OA2[7] is compared.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "MASK7",
                    description: Some(
                        "OA2[7:1] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Reload",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "COMPLETED",
                    description: Some(
                        "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOT_COMPLETED",
                    description: Some(
                        "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
