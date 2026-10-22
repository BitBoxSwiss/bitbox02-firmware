
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Spi",
            extends: None,
            description: Some(
                "Serial peripheral interface",
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
                    name: "cfg1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "sr",
                    description: Some(
                        "Status Register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some(
                        "Interrupt/Status Flags Clear Register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txdr16",
                    description: Some(
                        "Transmit Data Register - half-word sized",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "txdr32",
                    description: Some(
                        "Transmit Data Register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "txdr8",
                    description: Some(
                        "Transmit Data Register - byte sized",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rxdr16",
                    description: Some(
                        "Receive Data Register - half-word sized",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rxdr32",
                    description: Some(
                        "Receive Data Register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rxdr8",
                    description: Some(
                        "Receive Data Register - byte sized",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "crcpoly",
                    description: Some(
                        "Polynomial Register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcpoly",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txcrc",
                    description: Some(
                        "Transmitter CRC Register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txcrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxcrc",
                    description: Some(
                        "Receiver CRC Register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxcrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "udrdr",
                    description: Some(
                        "Underrun Data Register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Udrdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i2scfgr",
                    description: Some(
                        "I2S Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I2scfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsize",
                    description: Some(
                        "Number of bits in at single SPI data frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fthlv",
                    description: Some(
                        "threshold level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Fthlv",
                    ),
                },
                Field {
                    name: "udrcfg",
                    description: Some(
                        "Behavior of slave transmitter at underrun condition",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Udrcfg",
                    ),
                },
                Field {
                    name: "rxdmaen",
                    description: Some(
                        "Rx DMA stream enable",
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
                    name: "txdmaen",
                    description: Some(
                        "Tx DMA stream enable",
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
                    name: "crcsize",
                    description: Some(
                        "Length of CRC frame to be transacted and compared",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "Hardware CRC computation enable",
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
                    name: "mbr",
                    description: Some(
                        "Master baud rate",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mbr",
                    ),
                },
                Field {
                    name: "bpass",
                    description: Some(
                        "bypass of the prescaler at master baud rate clock generator",
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
            name: "Cfg2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mssi",
                    description: Some(
                        "Master SS Idleness",
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
                    name: "midi",
                    description: Some(
                        "Master Inter-Data Idleness",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdiom",
                    description: Some(
                        "RDY signal input/output management\n Note: When DSIZE at the CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rdiom",
                    ),
                },
                Field {
                    name: "rdiop",
                    description: Some(
                        "RDY signal input/output polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rdiop",
                    ),
                },
                Field {
                    name: "ioswp",
                    description: Some(
                        "Swap functionality of MISO and MOSI pins",
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
                    name: "comm",
                    description: Some(
                        "SPI Communication Mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Comm",
                    ),
                },
                Field {
                    name: "sp",
                    description: Some(
                        "Serial Protocol",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sp",
                    ),
                },
                Field {
                    name: "master",
                    description: Some(
                        "SPI Master",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Master",
                    ),
                },
                Field {
                    name: "lsbfirst",
                    description: Some(
                        "Data frame format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsbfirst",
                    ),
                },
                Field {
                    name: "cpha",
                    description: Some(
                        "Clock phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpha",
                    ),
                },
                Field {
                    name: "cpol",
                    description: Some(
                        "Clock polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpol",
                    ),
                },
                Field {
                    name: "ssm",
                    description: Some(
                        "Software management of SS signal input",
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
                    name: "ssiop",
                    description: Some(
                        "SS input/output polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssiop",
                    ),
                },
                Field {
                    name: "ssoe",
                    description: Some(
                        "SS output enable",
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
                    name: "ssom",
                    description: Some(
                        "SS output management in master mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssom",
                    ),
                },
                Field {
                    name: "afcntr",
                    description: Some(
                        "Alternate function always control GPIOs",
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
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spe",
                    description: Some(
                        "Serial Peripheral Enable",
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
                    name: "masrx",
                    description: Some(
                        "Master automatic SUSP in Receive mode",
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
                    name: "cstart",
                    description: Some(
                        "Master transfer start",
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
                    name: "csusp",
                    description: Some(
                        "Master SUSPend request",
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
                    name: "hddir",
                    description: Some(
                        "Rx/Tx direction at Half-duplex mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hddir",
                    ),
                },
                Field {
                    name: "ssi",
                    description: Some(
                        "Internal SS signal input level",
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
                    name: "crc33_17",
                    description: Some(
                        "Full size (33-bit or 17-bit) CRC polynomial is used",
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
                    name: "rcrcini",
                    description: Some(
                        "CRC calculation initialization pattern control for receiver",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rcrcini",
                    ),
                },
                Field {
                    name: "tcrcini",
                    description: Some(
                        "CRC calculation initialization pattern control for transmitter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tcrcini",
                    ),
                },
                Field {
                    name: "iolock",
                    description: Some(
                        "Locking the AF configuration of associated IOs",
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
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsize",
                    description: Some(
                        "Number of data at current transfer",
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
            name: "Crcpoly",
            extends: None,
            description: Some(
                "Polynomial Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crcpoly",
                    description: Some(
                        "CRC polynomial register",
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
            name: "I2scfgr",
            extends: None,
            description: Some(
                "I2S Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2smod",
                    description: Some(
                        "I2S mode selection",
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
                    name: "i2scfg",
                    description: Some(
                        "I2S configuration mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "I2scfg",
                    ),
                },
                Field {
                    name: "i2sstd",
                    description: Some(
                        "I2S standard selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2sstd",
                    ),
                },
                Field {
                    name: "pcmsync",
                    description: Some(
                        "PCM frame synchronization",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pcmsync",
                    ),
                },
                Field {
                    name: "datlen",
                    description: Some(
                        "Data length to be transferred",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Datlen",
                    ),
                },
                Field {
                    name: "chlen",
                    description: Some(
                        "Channel length (number of bits per audio channel)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Chlen",
                    ),
                },
                Field {
                    name: "ckpol",
                    description: Some(
                        "Serial audio clock polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckpol",
                    ),
                },
                Field {
                    name: "fixch",
                    description: Some(
                        "Fixed channel length in slave",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fixch",
                    ),
                },
                Field {
                    name: "wsinv",
                    description: Some(
                        "Word select inversion",
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
                    name: "datfmt",
                    description: Some(
                        "Data format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Datfmt",
                    ),
                },
                Field {
                    name: "i2sdiv",
                    description: Some(
                        "I2S linear prescaler",
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
                    name: "odd",
                    description: Some(
                        "Odd factor for the prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Odd",
                    ),
                },
                Field {
                    name: "mckoe",
                    description: Some(
                        "Master clock output enable",
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
            name: "Ier",
            extends: None,
            description: Some(
                "Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxpie",
                    description: Some(
                        "RXP Interrupt Enable",
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
                    name: "txpie",
                    description: Some(
                        "TXP interrupt enable",
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
                    name: "dxpie",
                    description: Some(
                        "DXP interrupt enabled",
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
                    name: "eotie",
                    description: Some(
                        "EOT, SUSP and TXC interrupt enable",
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
                    name: "txtfie",
                    description: Some(
                        "TXTFIE interrupt enable",
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
                    name: "udrie",
                    description: Some(
                        "UDR interrupt enable",
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
                    name: "ovrie",
                    description: Some(
                        "OVR interrupt enable",
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
                    name: "crceie",
                    description: Some(
                        "CRC Interrupt enable",
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
                    name: "tifreie",
                    description: Some(
                        "TIFRE interrupt enable",
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
                    name: "modfie",
                    description: Some(
                        "Mode Fault interrupt enable",
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
            name: "Ifcr",
            extends: None,
            description: Some(
                "Interrupt/Status Flags Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eotc",
                    description: Some(
                        "End Of Transfer flag clear",
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
                    name: "txtfc",
                    description: Some(
                        "Transmission Transfer Filled flag clear",
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
                    name: "udrc",
                    description: Some(
                        "Underrun flag clear",
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
                    name: "ovrc",
                    description: Some(
                        "Overrun flag clear",
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
                    name: "crcec",
                    description: Some(
                        "CRC Error flag clear",
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
                    name: "tifrec",
                    description: Some(
                        "TI frame format error flag clear",
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
                    name: "modfc",
                    description: Some(
                        "Mode Fault flag clear",
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
                    name: "suspc",
                    description: Some(
                        "SUSPend flag clear",
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
            name: "Rxcrc",
            extends: None,
            description: Some(
                "Receiver CRC Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrc",
                    description: Some(
                        "CRC register for receiver",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxp",
                    description: Some(
                        "Rx-Packet available",
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
                    name: "txp",
                    description: Some(
                        "Tx-Packet space available",
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
                    name: "dxp",
                    description: Some(
                        "Duplex Packet",
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
                    name: "eot",
                    description: Some(
                        "End Of Transfer",
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
                    name: "txtf",
                    description: Some(
                        "Transmission Transfer Filled",
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
                    name: "udr",
                    description: Some(
                        "Underrun at slave transmission mode",
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
                    name: "ovr",
                    description: Some(
                        "Overrun",
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
                    name: "crce",
                    description: Some(
                        "CRC Error",
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
                    name: "tifre",
                    description: Some(
                        "TI frame format error",
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
                    name: "modf",
                    description: Some(
                        "Mode Fault",
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
                    name: "susp",
                    description: Some(
                        "SUSPend",
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
                    name: "txc",
                    description: Some(
                        "TxFIFO transmission complete",
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
                    name: "rxplvl",
                    description: Some(
                        "RxFIFO Packing LeVeL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rxplvl",
                    ),
                },
                Field {
                    name: "rxwne",
                    description: Some(
                        "RxFIFO Word Not Empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rxwne",
                    ),
                },
                Field {
                    name: "ctsize",
                    description: Some(
                        "Number of data frames remaining in current TSIZE session",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txcrc",
            extends: None,
            description: Some(
                "Transmitter CRC Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txcrc",
                    description: Some(
                        "CRC register for transmitter",
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
            name: "Udrdr",
            extends: None,
            description: Some(
                "Underrun Data Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "udrdr",
                    description: Some(
                        "Data at slave underrun condition",
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
    ],
    enums: &[
        Enum {
            name: "Chlen",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "16 bits per channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "32 bits per channel",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ckpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IDLE_LOW",
                    description: Some(
                        "CK idle Level is Low. Signals are sampled on rising and changed on falling clock edges",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLE_HIGH",
                    description: Some(
                        "CK idle level is High. Signals are sampled on falling and changed on rising clock edges",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Comm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FULL_DUPLEX",
                    description: Some(
                        "Full duplex",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRANSMITTER",
                    description: Some(
                        "Simplex transmitter only",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RECEIVER",
                    description: Some(
                        "Simplex receiver only",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HALF_DUPLEX",
                    description: Some(
                        "Half duplex",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cpha",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIRST_EDGE",
                    description: Some(
                        "The first clock transition is the first data capture edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SECOND_EDGE",
                    description: Some(
                        "The second clock transition is the first data capture edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IDLE_LOW",
                    description: Some(
                        "SCK to 0 when idle",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLE_HIGH",
                    description: Some(
                        "SCK to 1 when idle",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Datfmt",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RIGHT_ALIGNED",
                    description: Some(
                        "The data inside RXDR and TXDR are right aligned",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEFT_ALIGNED",
                    description: Some(
                        "The data inside RXDR and TXDR are left aligned",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Datlen",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "16-bit data length",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS24",
                    description: Some(
                        "24-bit data length",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "32-bit data length",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Fixch",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_FIXED",
                    description: Some(
                        "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FIXED",
                    description: Some(
                        "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fthlv",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "ONE_FRAME",
                    description: Some(
                        "1 frame",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TWO_FRAMES",
                    description: Some(
                        "2 frames",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "THREE_FRAMES",
                    description: Some(
                        "3 frames",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FOUR_FRAMES",
                    description: Some(
                        "4 frames",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FIVE_FRAMES",
                    description: Some(
                        "5 frames",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SIX_FRAMES",
                    description: Some(
                        "6 frames",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SEVEN_FRAMES",
                    description: Some(
                        "7 frames",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EIGHT_FRAMES",
                    description: Some(
                        "8 frames",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "NINE_FRAMES",
                    description: Some(
                        "9 frames",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "TEN_FRAMES",
                    description: Some(
                        "10 frames",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "ELEVEN_FRAMES",
                    description: Some(
                        "11 frames",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "TWELVE_FRAMES",
                    description: Some(
                        "12 frames",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "THIRTEEN_FRAMES",
                    description: Some(
                        "13 frames",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FOURTEEN_FRAMES",
                    description: Some(
                        "14 frames",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FIFTEEN_FRAMES",
                    description: Some(
                        "15 frames",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "SIXTEEN_FRAMES",
                    description: Some(
                        "16 frames",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Hddir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RECEIVER",
                    description: Some(
                        "Receiver in half duplex mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRANSMITTER",
                    description: Some(
                        "Transmitter in half duplex mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2scfg",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SLAVE_TX",
                    description: Some(
                        "Slave, transmit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SLAVE_RX",
                    description: Some(
                        "Slave, receive",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MASTER_TX",
                    description: Some(
                        "Master, transmit",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MASTER_RX",
                    description: Some(
                        "Master, receive",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SLAVE_FULL_DUPLEX",
                    description: Some(
                        "Slave, full duplex",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MASTER_FULL_DUPLEX",
                    description: Some(
                        "Master, full duplex",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "I2sstd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PHILIPS",
                    description: Some(
                        "I2S Philips standard",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSB",
                    description: Some(
                        "MSB/left justified standard",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSB",
                    description: Some(
                        "LSB/right justified standard",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PCM",
                    description: Some(
                        "PCM standard",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lsbfirst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSBFIRST",
                    description: Some(
                        "Data is transmitted/received with the MSB first",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSBFIRST",
                    description: Some(
                        "Data is transmitted/received with the LSB first",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Master",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SLAVE",
                    description: Some(
                        "Slave configuration",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASTER",
                    description: Some(
                        "Master configuration",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mbr",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "f_spi_ker_ck / 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "f_spi_ker_ck / 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "f_spi_ker_ck / 8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "f_spi_ker_ck / 16",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "f_spi_ker_ck / 32",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "f_spi_ker_ck / 64",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "f_spi_ker_ck / 128",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "f_spi_ker_ck / 256",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Odd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EVEN",
                    description: Some(
                        "Real divider value is I2SDIV*2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ODD",
                    description: Some(
                        "Real divider value is I2SDIV*2 + 1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcmsync",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SHORT",
                    description: Some(
                        "Short PCM frame synchronization",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LONG",
                    description: Some(
                        "Long PCM frame synchronization",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rcrcini",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL_ZEROS",
                    description: Some(
                        "All zeros RX CRC initialization pattern",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALL_ONES",
                    description: Some(
                        "All ones RX CRC initialization pattern",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rdiom",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PERMANENTLY_ACTIVE",
                    description: Some(
                        "RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FROM_INPUT",
                    description: Some(
                        "RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rdiop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "READY_HIGH",
                    description: Some(
                        "high level of the signal means the slave is ready for communication",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "READY_LOW",
                    description: Some(
                        "low level of the signal means the slave is ready for communication",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rxplvl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ZERO_FRAMES",
                    description: Some(
                        "Zero frames beyond packing ratio available",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONE_FRAME",
                    description: Some(
                        "One frame beyond packing ratio available",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TWO_FRAMES",
                    description: Some(
                        "Two frame beyond packing ratio available",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "THREE_FRAMES",
                    description: Some(
                        "Three frame beyond packing ratio available",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rxwne",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LESS_THAN32",
                    description: Some(
                        "Less than 32-bit data frame received",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AT_LEAST32",
                    description: Some(
                        "At least 32-bit data frame received",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MOTOROLA",
                    description: Some(
                        "Motorola SPI protocol",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TI",
                    description: Some(
                        "TI SPI protocol",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ssiop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "Low level is active for SS signal",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "High level is active for SS signal",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ssom",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ASSERTED",
                    description: Some(
                        "SS is asserted until data transfer complete",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOT_ASSERTED",
                    description: Some(
                        "Data frames interleaved with SS not asserted during MIDI",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tcrcini",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL_ZEROS",
                    description: Some(
                        "All zeros TX CRC initialization pattern",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALL_ONES",
                    description: Some(
                        "All ones TX CRC initialization pattern",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Udrcfg",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CONSTANT",
                    description: Some(
                        "Slave sends a constant underrun pattern",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REPEAT_RECEIVED",
                    description: Some(
                        "Slave repeats last received data frame from master",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "REPEAT_TRANSMITTED",
                    description: Some(
                        "Slave repeats last transmitted data frame",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
