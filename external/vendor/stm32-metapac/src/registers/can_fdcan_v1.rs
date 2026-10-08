
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fdcan",
            extends: None,
            description: Some(
                "Controller area network with flexible data rate (FD)",
            ),
            items: &[
                BlockItem {
                    name: "crel",
                    description: Some(
                        "FDCAN core release register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "endn",
                    description: Some(
                        "FDCAN endian register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Endn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbtp",
                    description: Some(
                        "FDCAN data bit timing and prescaler register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test",
                    description: Some(
                        "FDCAN test register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Test",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwd",
                    description: Some(
                        "FDCAN RAM watchdog register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "FDCAN CC control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nbtp",
                    description: Some(
                        "FDCAN nominal bit timing and prescaler register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscc",
                    description: Some(
                        "FDCAN timestamp counter configuration register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscv",
                    description: Some(
                        "FDCAN timestamp counter value register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocc",
                    description: Some(
                        "FDCAN timeout counter configuration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocv",
                    description: Some(
                        "FDCAN timeout counter value register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "FDCAN error counter register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psr",
                    description: Some(
                        "FDCAN protocol status register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdcr",
                    description: Some(
                        "FDCAN transmitter delay compensation register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ir",
                    description: Some(
                        "FDCAN interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ie",
                    description: Some(
                        "FDCAN interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ils",
                    description: Some(
                        "FDCAN interrupt line select register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ils",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ile",
                    description: Some(
                        "FDCAN interrupt line enable register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ile",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxgfc",
                    description: Some(
                        "FDCAN global filter configuration register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxgfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xidam",
                    description: Some(
                        "FDCAN extended ID and mask register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xidam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpms",
                    description: Some(
                        "FDCAN high-priority message status register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpms",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxfs",
                    description: Some(
                        "FDCAN Rx FIFO X status register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxfa",
                    description: Some(
                        "CAN Rx FIFO X acknowledge register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxfa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbc",
                    description: Some(
                        "FDCAN Tx buffer configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txfqs",
                    description: Some(
                        "FDCAN Tx FIFO/queue status register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txfqs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbrp",
                    description: Some(
                        "FDCAN Tx buffer request pending register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbar",
                    description: Some(
                        "FDCAN Tx buffer add request register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcr",
                    description: Some(
                        "FDCAN Tx buffer cancellation request register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbto",
                    description: Some(
                        "FDCAN Tx buffer transmission occurred register",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcf",
                    description: Some(
                        "FDCAN Tx buffer cancellation finished register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbtie",
                    description: Some(
                        "FDCAN Tx buffer transmission interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbtie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcie",
                    description: Some(
                        "FDCAN Tx buffer cancellation finished interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefs",
                    description: Some(
                        "FDCAN Tx event FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefa",
                    description: Some(
                        "FDCAN Tx event FIFO acknowledge register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckdiv",
                    description: Some(
                        "FDCAN CFG clock divider register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckdiv",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some(
                "FDCAN CC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "init",
                    description: Some(
                        "Initialization",
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
                    name: "cce",
                    description: Some(
                        "Configuration change enable",
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
                    name: "asm",
                    description: Some(
                        "ASM restricted operation mode. The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time",
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
                    name: "csa",
                    description: Some(
                        "Clock stop acknowledge",
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
                    name: "csr",
                    description: Some(
                        "Clock stop request",
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
                    name: "mon",
                    description: Some(
                        "Bus monitoring mode. Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time",
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
                    name: "dar",
                    description: Some(
                        "Disable automatic retransmission",
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
                    name: "test",
                    description: Some(
                        "Test mode enable",
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
                    name: "fdoe",
                    description: Some(
                        "FD operation enable",
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
                    name: "brse",
                    description: Some(
                        "FDCAN bit rate switching",
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
                    name: "pxhd",
                    description: Some(
                        "Protocol exception handling disable",
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
                    name: "efbi",
                    description: Some(
                        "Edge filtering during bus integration",
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
                    name: "txp",
                    description: Some(
                        "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame",
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
                    name: "niso",
                    description: Some(
                        "Non ISO operation. If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0",
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
            name: "Ckdiv",
            extends: None,
            description: Some(
                "FDCAN CFG clock divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdiv",
                    description: Some(
                        "input clock divider. The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pdiv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Crel",
            extends: None,
            description: Some(
                "FDCAN core release register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "day",
                    description: Some(
                        "DAY",
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
                    name: "mon",
                    description: Some(
                        "MON",
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
                    name: "year",
                    description: Some(
                        "YEAR",
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
                    name: "substep",
                    description: Some(
                        "SUBSTEP",
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
                    name: "step",
                    description: Some(
                        "STEP",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rel",
                    description: Some(
                        "REL",
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
            name: "Dbtp",
            extends: None,
            description: Some(
                "FDCAN data bit timing and prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsjw",
                    description: Some(
                        "Synchronization jump width. Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq.",
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
                    name: "dtseg2",
                    description: Some(
                        "Data time segment after sample point. Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq",
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
                    name: "dtseg1",
                    description: Some(
                        "Data time segment before sample point. Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbrp",
                    description: Some(
                        "Data bit rate prescaler. The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1",
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
                    name: "tdc",
                    description: Some(
                        "Transceiver delay compensation",
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
            name: "Ecr",
            extends: None,
            description: Some(
                "FDCAN error counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tec",
                    description: Some(
                        "Transmit error counter. Actual state of the transmit error counter, values between 0 and 255.  When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented",
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
                    name: "rec",
                    description: Some(
                        "Receive error counter. Actual state of the receive error counter, values between 0 and 127",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rp",
                    description: Some(
                        "Receive error passive",
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
                    name: "cel",
                    description: Some(
                        "CAN error logging. The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR[ELO].  Access type is RX: reset on read.",
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
            ],
        },
        FieldSet {
            name: "Endn",
            extends: None,
            description: Some(
                "FDCAN endian register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etv",
                    description: Some(
                        "Endianness test value. The endianness test value is 0x8765 4321",
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
            name: "Hpms",
            extends: None,
            description: Some(
                "FDCAN high-priority message status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidx",
                    description: Some(
                        "Buffer index. Index of Rx FIFO element to which the message was stored. Only valid when MSI[1] = 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msi",
                    description: Some(
                        "Message storage indicator",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Msi",
                    ),
                },
                Field {
                    name: "fidx",
                    description: Some(
                        "Filter index. Index of matching filter element. Range is 0 to RXGFC[LSS] - 1 or RXGFC[LSE] - 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flst",
                    description: Some(
                        "Filter list. Indicates the filter list of the matching filter element",
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
            name: "Ie",
            extends: None,
            description: Some(
                "FDCAN interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfne",
                    description: Some(
                        "Rx FIFO X new message interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rffe",
                    description: Some(
                        "Rx FIFO X full interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rfle",
                    description: Some(
                        "Rx FIFO X message lost interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hpme",
                    description: Some(
                        "High-priority message interrupt enable",
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
                    name: "tce",
                    description: Some(
                        "Transmission completed interrupt enable",
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
                    name: "tcfe",
                    description: Some(
                        "Transmission cancellation finished interrupt enable",
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
                    name: "tfee",
                    description: Some(
                        "Tx FIFO empty interrupt enable",
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
                    name: "tefne",
                    description: Some(
                        "Tx event FIFO new entry interrupt enable",
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
                    name: "teffe",
                    description: Some(
                        "Tx event FIFO full interrupt enable",
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
                    name: "tefle",
                    description: Some(
                        "Tx event FIFO element lost interrupt enable",
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
                    name: "tswe",
                    description: Some(
                        "Timestamp wraparound interrupt enable",
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
                    name: "mrafe",
                    description: Some(
                        "Message RAM access failure interrupt enable",
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
                    name: "tooe",
                    description: Some(
                        "Timeout occurred interrupt enable",
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
                    name: "eloe",
                    description: Some(
                        "Error logging overflow interrupt enable",
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
                    name: "epe",
                    description: Some(
                        "Error passive interrupt enable",
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
                    name: "ewe",
                    description: Some(
                        "Warning status interrupt enable",
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
                    name: "boe",
                    description: Some(
                        "Bus_Off status enable",
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
                    name: "wdie",
                    description: Some(
                        "Watchdog interrupt enable",
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
                    name: "peae",
                    description: Some(
                        "Protocol error in arbitration phase enable",
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
                    name: "pede",
                    description: Some(
                        "Protocol error in data phase enable",
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
                    name: "arae",
                    description: Some(
                        "Access to reserved address enable",
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
            name: "Ile",
            extends: None,
            description: Some(
                "FDCAN interrupt line enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eint0",
                    description: Some(
                        "Enable interrupt line 0",
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
                    name: "eint1",
                    description: Some(
                        "Enable interrupt line 1",
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
            name: "Ils",
            extends: None,
            description: Some(
                "FDCAN interrupt line select register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfifo",
                    description: Some(
                        "RX FIFO bit grouping the following interruption. RFLL: Rx FIFO X message lost interrupt line  RFFL: Rx FIFO X full interrupt line  RFNL: Rx FIFO X new message interrupt line.",
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
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "smsg",
                    description: Some(
                        "Status message bit grouping the following interruption. TCFL: Transmission cancellation finished interrupt line  TCL: Transmission completed interrupt line  HPML: High-priority message interrupt line.",
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
                    name: "tferr",
                    description: Some(
                        "Tx FIFO ERROR grouping the following interruption. TEFLL: Tx event FIFO element lost interrupt line  TEFFL: Tx event FIFO full interrupt line  TEFNL: Tx event FIFO new entry interrupt line  TFEL: Tx FIFO empty interrupt line.",
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
                    name: "misc",
                    description: Some(
                        "Interrupt regrouping the following interruption. TOOL: Timeout occurred interrupt line  MRAFL: Message RAM access failure interrupt line  TSWL: Timestamp wraparound interrupt line.",
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
                    name: "berr",
                    description: Some(
                        "Bit and line error grouping the following interruption. EPL Error passive interrupt line  ELOL: Error logging overflow interrupt line.",
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
                    name: "perr",
                    description: Some(
                        "Protocol error grouping the following interruption. ARAL: Access to reserved address line  PEDL: Protocol error in data phase line  PEAL: Protocol error in arbitration phase line  WDIL: Watchdog interrupt line  BOL: Bus_Off status  EWL: Warning status interrupt line.",
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
            name: "Ir",
            extends: None,
            description: Some(
                "FDCAN interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfn",
                    description: Some(
                        "Rx FIFO X new message",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rff",
                    description: Some(
                        "Rx FIFO X full",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rfl",
                    description: Some(
                        "Rx FIFO X message lost",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hpm",
                    description: Some(
                        "High-priority message",
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
                    name: "tc",
                    description: Some(
                        "Transmission completed",
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
                    name: "tcf",
                    description: Some(
                        "Transmission cancellation finished",
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
                    name: "tfe",
                    description: Some(
                        "Tx FIFO empty",
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
                    name: "tefn",
                    description: Some(
                        "Tx event FIFO New Entry",
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
                    name: "teff",
                    description: Some(
                        "Tx event FIFO full",
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
                    name: "tefl",
                    description: Some(
                        "Tx event FIFO element lost",
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
                    name: "tsw",
                    description: Some(
                        "Timestamp wraparound",
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
                    name: "mraf",
                    description: Some(
                        "Message RAM access failure. The flag is set when the Rx handler:  has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message.  was unable to write a message to the message RAM. In this case message storage is aborted.  In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location.  The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.",
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
                    name: "too",
                    description: Some(
                        "Timeout occurred",
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
                    name: "elo",
                    description: Some(
                        "Error logging overflow",
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
                    name: "ep",
                    description: Some(
                        "Error passive",
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
                    name: "ew",
                    description: Some(
                        "Warning status",
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
                    name: "bo",
                    description: Some(
                        "Bus_Off status",
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
                    name: "wdi",
                    description: Some(
                        "Watchdog interrupt",
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
                    name: "pea",
                    description: Some(
                        "Protocol error in arbitration phase (nominal bit time is used)",
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
                    name: "ped",
                    description: Some(
                        "Protocol error in data phase (data bit time is used)",
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
                    name: "ara",
                    description: Some(
                        "Access to reserved address",
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
            name: "Nbtp",
            extends: None,
            description: Some(
                "FDCAN nominal bit timing and prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ntseg2",
                    description: Some(
                        "Nominal time segment after sample point. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used",
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
                Field {
                    name: "ntseg1",
                    description: Some(
                        "Nominal time segment before sample point. Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "nbrp",
                    description: Some(
                        "Bit rate prescaler. Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsjw",
                    description: Some(
                        "Nominal (re)synchronization jump width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one.  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Psr",
            extends: None,
            description: Some(
                "FDCAN protocol status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lec",
                    description: Some(
                        "Last error code. The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error.  Access type is RS: set on read.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lec",
                    ),
                },
                Field {
                    name: "act",
                    description: Some(
                        "Activity. Monitors the module’s CAN communication state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Act",
                    ),
                },
                Field {
                    name: "ep",
                    description: Some(
                        "Error passive",
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
                    name: "ew",
                    description: Some(
                        "Warning Sstatus",
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
                    name: "bo",
                    description: Some(
                        "Bus_Off status",
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
                    name: "dlec",
                    description: Some(
                        "Data last error code. Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error.  Access type is RS: set on read.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "resi",
                    description: Some(
                        "ESI flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering.  Access type is RX: reset on read.",
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
                    name: "rbrs",
                    description: Some(
                        "BRS flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering.  Access type is RX: reset on read.",
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
                    name: "redl",
                    description: Some(
                        "Received FDCAN message. This bit is set independent of acceptance filtering.  Access type is RX: reset on read.",
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
                    name: "pxe",
                    description: Some(
                        "Protocol exception event",
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
                    name: "tdcv",
                    description: Some(
                        "Transmitter delay compensation value. Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwd",
            extends: None,
            description: Some(
                "FDCAN RAM watchdog register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdc",
                    description: Some(
                        "Watchdog configuration. Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled.  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of FDCAN_CCCR register are set to 1",
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
                    name: "wdv",
                    description: Some(
                        "Watchdog value. Actual message RAM watchdog counter value",
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
            ],
        },
        FieldSet {
            name: "Rxfa",
            extends: None,
            description: Some(
                "CAN Rx FIFO X acknowledge register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fai",
                    description: Some(
                        "Rx FIFO X acknowledge index. After the Host has read a message or a sequence of messages from Rx FIFO X it has to write the buffer index of the last element read from Rx FIFO X to FAI. This sets the Rx FIFO X get index RXFS[FGI] to FAI + 1 and update the FIFO X fill level RXFS[FFL]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxfs",
            extends: None,
            description: Some(
                "FDCAN Rx FIFO X status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ffl",
                    description: Some(
                        "Rx FIFO X fill level. Number of elements stored in Rx FIFO X, range 0 to 3",
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
                    name: "fgi",
                    description: Some(
                        "Rx FIFO X get index. Rx FIFO X read index pointer, range 0 to 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpi",
                    description: Some(
                        "Rx FIFO X put index. Rx FIFO X write index pointer, range 0 to 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ff",
                    description: Some(
                        "Rx FIFO X full",
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
                    name: "rfl",
                    description: Some(
                        "Rx FIFO X message lost. This bit is a copy of interrupt flag IR[RFL]. When IR[RFL] is reset, this bit is also reset",
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
            name: "Rxgfc",
            extends: None,
            description: Some(
                "FDCAN global filter configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rrfe",
                    description: Some(
                        "Reject remote frames extended. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "rrfs",
                    description: Some(
                        "Reject remote frames standard. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "anfe",
                    description: Some(
                        "Accept non-matching frames extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Anfe",
                    ),
                },
                Field {
                    name: "anfs",
                    description: Some(
                        "Accept Non-matching frames standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Anfs",
                    ),
                },
                Field {
                    name: "f1om",
                    description: Some(
                        "FIFO 1 operation mode (overwrite or blocking). This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "f0om",
                    description: Some(
                        "FIFO 0 operation mode (overwrite or blocking). This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "lss",
                    description: Some(
                        "List size standard. >28: Values greater than 28 are interpreted as 28.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1.",
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
                    name: "lse",
                    description: Some(
                        "List size extended. >8: Values greater than 8 are interpreted as 8.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tdcr",
            extends: None,
            description: Some(
                "FDCAN transmitter delay compensation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdcf",
                    description: Some(
                        "Transmitter delay compensation filter window length. Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                Field {
                    name: "tdco",
                    description: Some(
                        "Transmitter delay compensation offset. Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Test",
            extends: None,
            description: Some(
                "FDCAN test register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lbck",
                    description: Some(
                        "Loop back mode",
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
                    name: "tx",
                    description: Some(
                        "Control of transmit pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tx",
                    ),
                },
                Field {
                    name: "rx",
                    description: Some(
                        "Receive pin. Monitors the actual value of pin FDCANx_RX",
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
            name: "Tocc",
            extends: None,
            description: Some(
                "FDCAN timeout counter configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etoc",
                    description: Some(
                        "Timeout counter enable. This is a protected write (P) bit, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
                    name: "tos",
                    description: Some(
                        "Timeout select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC[TOP] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC[TOP]. Down-counting is started when the first FIFO element is stored.  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tos",
                    ),
                },
                Field {
                    name: "top",
                    description: Some(
                        "Timeout period. Start value of the timeout counter (down-counter). Configures the timeout period",
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
            name: "Tocv",
            extends: None,
            description: Some(
                "FDCAN timeout counter value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toc",
                    description: Some(
                        "Timeout counter. The timeout counter is decremented in multiples of CAN bit times [1 … 16] depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS",
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
            name: "Tscc",
            extends: None,
            description: Some(
                "FDCAN timestamp counter configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tss",
                    description: Some(
                        "Timestamp select. These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tss",
                    ),
                },
                Field {
                    name: "tcp",
                    description: Some(
                        "Timestamp counter prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times  [1 … 16].  The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.  In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10).  These are protected write (P) bits, write access is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
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
            ],
        },
        FieldSet {
            name: "Tscv",
            extends: None,
            description: Some(
                "FDCAN timestamp counter value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsc",
                    description: Some(
                        "Timestamp counter. The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC[TSS] = 01, the timestamp counter is incremented in multiples of CAN bit times [1 … 16] depending on the configuration of TSCC[TCP]. A wrap around sets interrupt flag IR[TSW]. Write access resets the counter to 0.  When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact",
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
            name: "Txbar",
            extends: None,
            description: Some(
                "FDCAN Tx buffer add request register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ar",
                    description: Some(
                        "Add request. Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbc",
            extends: None,
            description: Some(
                "FDCAN Tx buffer configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tfqm",
                    description: Some(
                        "Tx FIFO/queue mode. This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tfqm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Txbcf",
            extends: None,
            description: Some(
                "FDCAN Tx buffer cancellation finished register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cf",
                    description: Some(
                        "Cancellation finished. Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbcie",
            extends: None,
            description: Some(
                "FDCAN Tx buffer cancellation finished interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfie",
                    description: Some(
                        "Cancellation finished interrupt enable.. Each Tx buffer has its own CFIE bit",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbcr",
            extends: None,
            description: Some(
                "FDCAN Tx buffer cancellation request register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "Cancellation request. Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact.  This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbrp",
            extends: None,
            description: Some(
                "FDCAN Tx buffer request pending register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trp",
                    description: Some(
                        "Transmission request pending. Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR.  After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID).  A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset.  After a cancellation has been requested, a finished cancellation is signaled via TXBCF  after successful transmission together with the corresponding TXBTO bit  when the transmission has not yet been started at the point of cancellation  when the transmission has been aborted due to lost arbitration  when an error occurred during frame transmission  In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbtie",
            extends: None,
            description: Some(
                "FDCAN Tx buffer transmission interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmission interrupt enable. Each Tx buffer has its own TIE bit",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbto",
            extends: None,
            description: Some(
                "FDCAN Tx buffer transmission occurred register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to",
                    description: Some(
                        "Transmission occurred.. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txefa",
            extends: None,
            description: Some(
                "FDCAN Tx event FIFO acknowledge register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efai",
                    description: Some(
                        "Event FIFO acknowledge index. After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS[EFGI] to EFAI + 1 and updates the FIFO 0 fill level TXEFS[EFFL]",
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
            ],
        },
        FieldSet {
            name: "Txefs",
            extends: None,
            description: Some(
                "FDCAN Tx event FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "effl",
                    description: Some(
                        "Event FIFO fill level. Number of elements stored in Tx event FIFO, range 0 to 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efgi",
                    description: Some(
                        "Event FIFO get index. Tx event FIFO read index pointer, range 0 to 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efpi",
                    description: Some(
                        "Event FIFO put index. Tx event FIFO write index pointer, range 0 to 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eff",
                    description: Some(
                        "Event FIFO full",
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
                    name: "tefl",
                    description: Some(
                        "Tx event FIFO element lost. This bit is a copy of interrupt flag IR[TEFL]. When IR[TEFL] is reset, this bit is also reset.  0 No Tx event FIFO element lost  1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0",
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
            name: "Txfqs",
            extends: None,
            description: Some(
                "FDCAN Tx FIFO/queue status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tffl",
                    description: Some(
                        "Tx FIFO free level. Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC[TFQM] = 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfgi",
                    description: Some(
                        "Tx FIFO get index. Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqpi",
                    description: Some(
                        "Tx FIFO/queue put index. Tx FIFO/queue write index pointer, range 0 to 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqf",
                    description: Some(
                        "Tx FIFO/queue full",
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
            name: "Xidam",
            extends: None,
            description: Some(
                "FDCAN extended ID and mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eidm",
                    description: Some(
                        "Extended ID mask. For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active.  These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 [CCE] and bit 0 [INIT] of CCCR register are set to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Act",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYNC",
                    description: Some(
                        "Synchronizing: node is synchronizing on CAN communication.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLE",
                    description: Some(
                        "Idle: node is neither receiver nor transmitter.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RX",
                    description: Some(
                        "Receiver: node is operating as receiver.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TX",
                    description: Some(
                        "Transmitter: node is operating as transmitter.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Anfe",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACCEPT_FIFO_0",
                    description: Some(
                        "Accept in Rx FIFO 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACCEPT_FIFO_1",
                    description: Some(
                        "Accept in Rx FIFO 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "REJECT",
                    description: Some(
                        "Reject",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Anfs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACCEPT_FIFO_0",
                    description: Some(
                        "Accept in Rx FIFO 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACCEPT_FIFO_1",
                    description: Some(
                        "Accept in Rx FIFO 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "REJECT",
                    description: Some(
                        "Reject",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lec",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_ERROR",
                    description: Some(
                        "No Error: No error occurred since LEC has been reset by successful reception or transmission.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STUFF",
                    description: Some(
                        "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FORM",
                    description: Some(
                        "Form Error: A fixed format part of a received frame has the wrong format.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "AckError: The message transmitted by the FDCAN was not acknowledged by another node.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BIT_1",
                    description: Some(
                        "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BIT_0",
                    description: Some(
                        "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed).",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CRC",
                    description: Some(
                        "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "NO_CHANGE",
                    description: Some(
                        "NoChange: Any read access to the Protocol status register re-initializes the LEC to ‘7’. When the LEC shows the value ‘7’, no CAN bus event was detected since the last CPU read access to the Protocol status register.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Msi",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_FIFO",
                    description: Some(
                        "No FIFO selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERRUN",
                    description: Some(
                        "FIFO overrun",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FIFO_0",
                    description: Some(
                        "Message stored in FIFO 0",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FIFO_1",
                    description: Some(
                        "Message stored in FIFO 1",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pdiv",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV_1",
                    description: Some(
                        "Divide by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV_2",
                    description: Some(
                        "Divide by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV_4",
                    description: Some(
                        "Divide by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV_6",
                    description: Some(
                        "Divide by 6",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV_8",
                    description: Some(
                        "Divide by 8",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV_10",
                    description: Some(
                        "Divide by 10",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV_12",
                    description: Some(
                        "Divide by 12",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV_14",
                    description: Some(
                        "Divide by 14",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV_16",
                    description: Some(
                        "Divide by 16",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV_18",
                    description: Some(
                        "Divide by 18",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV_20",
                    description: Some(
                        "Divide by 20",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV_22",
                    description: Some(
                        "Divide by 22",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV_24",
                    description: Some(
                        "Divide by 24",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV_26",
                    description: Some(
                        "Divide by 26",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV_28",
                    description: Some(
                        "Divide by 28",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV_30",
                    description: Some(
                        "Divide by 30",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Tfqm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIFO",
                    description: Some(
                        "Tx FIFO operation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUEUE",
                    description: Some(
                        "Tx queue operation",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CONTINUOUS",
                    description: Some(
                        "Continuous operation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TX_EVENT_FIFO",
                    description: Some(
                        "Timeout controlled by Tx event FIFO",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RX_FIFO_0",
                    description: Some(
                        "Timeout controlled by Rx FIFO 0",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RX_FIFO_1",
                    description: Some(
                        "Timeout controlled by Rx FIFO 1",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Tss",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ZERO",
                    description: Some(
                        "Timestamp counter value always 0x0000",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREMENT",
                    description: Some(
                        "Timestamp counter value incremented according to TCP",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "External timestamp counter from TIM3 value (tim3_cnt[0:15])",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Tx",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SAMPLE_POINT",
                    description: Some(
                        "Sample point can be monitored at pin FDCANx_TX",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DOMINANT",
                    description: Some(
                        "Dominant (0) level at pin FDCANx_TX",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RECESSIVE",
                    description: Some(
                        "Recessive (1) at pin FDCANx_TX",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
