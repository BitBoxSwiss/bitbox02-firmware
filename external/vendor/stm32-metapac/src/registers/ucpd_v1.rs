
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ucpd",
            extends: None,
            description: Some(
                "USB Power Delivery interface",
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
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cfgr3",
                    description: Some(
                        "configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "imr",
                    description: Some(
                        "interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Imr",
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
                    byte_offset: 0x14,
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
                    name: "icr",
                    description: Some(
                        "interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "tx_ordsetr",
                    description: Some(
                        "Tx ordered set type register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TxOrdsetr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_payszr",
                    description: Some(
                        "Tx payload size register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TxPayszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txdr",
                    description: Some(
                        "Tx data register",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                BlockItem {
                    name: "rx_ordsetr",
                    description: None,
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxOrdsetr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_payszr",
                    description: None,
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxPayszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxdr",
                    description: None,
                    array: None,
                    byte_offset: 0x30,
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
                    name: "rx_ordextr1",
                    description: Some(
                        "Rx ordered set extension register 1",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxOrdextr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_ordextr2",
                    description: Some(
                        "Rx ordered set extension register 2",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxOrdextr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipver",
                    description: Some(
                        "UCPD IP ID register",
                    ),
                    array: None,
                    byte_offset: 0x3f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipver",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipid",
                    description: Some(
                        "UCPD IP ID register",
                    ),
                    array: None,
                    byte_offset: 0x3f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mid",
                    description: Some(
                        "UCPD IP ID register",
                    ),
                    array: None,
                    byte_offset: 0x3fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mid",
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
                    name: "hbitclkdiv",
                    description: Some(
                        "Division ratio for producing half-bit clock\r The bitfield determines the division ratio (the bitfield value plus one) of a clk divider producing half-bit clock (hbit_clk).",
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
                Field {
                    name: "ifrgap",
                    description: Some(
                        "Division ratio for producing inter-frame gap timer clock\r The bitfield determines the division ratio (the bitfield value minus one) of a clk divider producing inter-frame gap timer clock (tInterFrameGap).\r The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "transwin",
                    description: Some(
                        "Transition window duration\r The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval.\r Set a value that produces an interval of 12 to 20 us, taking into account the clk frequency and the HBITCLKDIV[5:0] bitfield setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psc_usbpdclk",
                    description: Some(
                        "Pre-scaler division ratio for generating clk\r The bitfield determines the division ratio of a kernel clock pre-scaler producing peripheral clock (clk).\r It is recommended to use the pre-scaler so as to set the clk frequency in the range from 6 to 9 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "PscUsbpdclk",
                    ),
                },
                Field {
                    name: "rxordseten",
                    description: Some(
                        "Receiver ordered set enable\r The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function:\r 0bxxxxxxxx1: SOP detect enabled\r 0bxxxxxxx1x: SOP' detect enabled\r 0bxxxxxx1xx: SOP'' detect enabled\r 0bxxxxx1xxx: Hard Reset detect enabled\r 0bxxxx1xxxx: Cable Detect reset enabled\r 0bxxx1xxxxx: SOP'_Debug enabled\r 0bxx1xxxxxx: SOP''_Debug enabled\r 0bx1xxxxxxx: SOP extension#1 enabled\r 0b1xxxxxxxx: SOP extension#2 enabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txdmaen",
                    description: Some(
                        "Transmission DMA mode enable\r When set, the bit enables DMA mode for transmission.",
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
                    name: "rxdmaen",
                    description: Some(
                        "Reception DMA mode enable\r When set, the bit enables DMA mode for reception.",
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
                    name: "ucpden",
                    description: Some(
                        "peripheral enable\r General enable of the peripheral.\r Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfiltdis",
                    description: Some(
                        "BMC decoder Rx pre-filter enable\r The sampling clock is that of the receiver (that is, after pre-scaler).",
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
                    name: "rxfilt2n3",
                    description: Some(
                        "BMC decoder Rx pre-filter sampling method\r Number of consistent consecutive samples before confirming a new value.",
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
                    name: "forceclk",
                    description: Some(
                        "Force ClkReq clock request",
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
                    name: "wupen",
                    description: Some(
                        "Wakeup from Stop mode enable\r Setting the bit enables the ASYNC_INT signal.",
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
            name: "Cfgr3",
            extends: None,
            description: Some(
                "configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trim_cc1_rd",
                    description: Some(
                        "SW trim value for Rd resistor on the CC1 line",
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
                    name: "trim_cc1_rp",
                    description: Some(
                        "SW trim value for Rp current sources on the CC1 line",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trim_cc2_rd",
                    description: Some(
                        "SW trim value for Rd resistor on the CC2 line",
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
                    name: "trim_cc2_rp",
                    description: Some(
                        "SW trim value for Rp current sources on the CC2 line",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txmode",
                    description: Some(
                        "Type of Tx packet\r Writing the bitfield triggers the action as follows, depending on the value:\r Others: invalid\r From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the \"tBISTContMode\" delay), disable the peripheral (UCPDEN = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Txmode",
                    ),
                },
                Field {
                    name: "txsend",
                    description: Some(
                        "Command to send a Tx packet\r The bit is cleared by hardware as soon as the packet transmission begins or is discarded.",
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
                    name: "txhrst",
                    description: Some(
                        "Command to send a Tx Hard Reset\r The bit is cleared by hardware as soon as the message transmission begins or is discarded.",
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
                    name: "rxmode",
                    description: Some(
                        "Receiver mode\r Determines the mode of the receiver.\r When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.",
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
                    name: "phyrxen",
                    description: Some(
                        "USB Power Delivery receiver enable\r Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.",
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
                    name: "phyccsel",
                    description: Some(
                        "CC1/CC2 line selector for USB Power Delivery signaling\r The selection depends on the cable orientation as discovered at attach.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Phyccsel",
                    ),
                },
                Field {
                    name: "anasubmode",
                    description: Some(
                        "Analog PHY sub-mode\r Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "anamode",
                    description: Some(
                        "Analog PHY operating mode\r The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE[1:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Anamode",
                    ),
                },
                Field {
                    name: "ccenable",
                    description: Some(
                        "CC line enable\r This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE[1:0] setting.\r A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ccenable",
                    ),
                },
                Field {
                    name: "cc1vconnen",
                    description: Some(
                        "VCONN switch enable for CC1",
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
                    name: "cc2vconnen",
                    description: Some(
                        "VCONN switch enable for CC2",
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
                    name: "dbatten",
                    description: Some(
                        "Dead battery function enable\r The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register.\r Dead battery function only operates if the external circuit is appropriately configured.",
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
                    name: "frsrxen",
                    description: Some(
                        "FRS event detection enable\r Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable\r Clear the bit when the device is attached to an FRS-incapable source/sink.",
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
                    name: "frstx",
                    description: Some(
                        "FRS Tx signaling enable.\r Setting the bit enables FRS Tx signaling.\r The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.",
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
                    name: "rdch",
                    description: Some(
                        "Rdch condition drive\r The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to \"USB Type-C ECN for Source VCONN Discharge\". The CCENABLE[1:0] bitfield must be set accordingly, too.",
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
                    name: "cc1tcdis",
                    description: Some(
                        "CC1 Type-C detector disable\r The bit disables the Type-C detector on the CC1 line.\r When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE[1:0].",
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
                    name: "cc2tcdis",
                    description: Some(
                        "CC2 Type-C detector disable\r The bit disables the Type-C detector on the CC2 line.\r When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE[1:0].",
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
            name: "Icr",
            extends: None,
            description: Some(
                "interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txmsgdisccf",
                    description: Some(
                        "Tx message discard flag (TXMSGDISC) clear\r Setting the bit clears the TXMSGDISC flag in the SR register.",
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
                    name: "txmsgsentcf",
                    description: Some(
                        "Tx message send flag (TXMSGSENT) clear\r Setting the bit clears the TXMSGSENT flag in the SR register.",
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
                    name: "txmsgabtcf",
                    description: Some(
                        "Tx message abort flag (TXMSGABT) clear\r Setting the bit clears the TXMSGABT flag in the SR register.",
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
                    name: "hrstdisccf",
                    description: Some(
                        "Hard reset discard flag (HRSTDISC) clear\r Setting the bit clears the HRSTDISC flag in the SR register.",
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
                    name: "hrstsentcf",
                    description: Some(
                        "Hard reset send flag (HRSTSENT) clear\r Setting the bit clears the HRSTSENT flag in the SR register.",
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
                    name: "txundcf",
                    description: Some(
                        "Tx underflow flag (TXUND) clear\r Setting the bit clears the TXUND flag in the SR register.",
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
                    name: "rxorddetcf",
                    description: Some(
                        "Rx ordered set detect flag (RXORDDET) clear\r Setting the bit clears the RXORDDET flag in the SR register.",
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
                    name: "rxhrstdetcf",
                    description: Some(
                        "Rx Hard Reset detect flag (RXHRSTDET) clear\r Setting the bit clears the RXHRSTDET flag in the SR register.",
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
                    name: "rxovrcf",
                    description: Some(
                        "Rx overflow flag (RXOVR) clear\r Setting the bit clears the RXOVR flag in the SR register.",
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
                    name: "rxmsgendcf",
                    description: Some(
                        "Rx message received flag (RXMSGEND) clear\r Setting the bit clears the RXMSGEND flag in the SR register.",
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
                    name: "typecevt1cf",
                    description: Some(
                        "Type-C CC1 event flag (TYPECEVT1) clear\r Setting the bit clears the TYPECEVT1 flag in the SR register",
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
                    name: "typecevt2cf",
                    description: Some(
                        "Type-C CC2 line event flag (TYPECEVT2) clear\r Setting the bit clears the TYPECEVT2 flag in the SR register",
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
                    name: "frsevtcf",
                    description: Some(
                        "FRS event flag (FRSEVT) clear\r Setting the bit clears the FRSEVT flag in the SR register.",
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
            ],
        },
        FieldSet {
            name: "Imr",
            extends: None,
            description: Some(
                "interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txisie",
                    description: Some(
                        "TXIS interrupt enable",
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
                    name: "txmsgdiscie",
                    description: Some(
                        "TXMSGDISC interrupt enable",
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
                    name: "txmsgsentie",
                    description: Some(
                        "TXMSGSENT interrupt enable",
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
                    name: "txmsgabtie",
                    description: Some(
                        "TXMSGABT interrupt enable",
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
                    name: "hrstdiscie",
                    description: Some(
                        "HRSTDISC interrupt enable",
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
                    name: "hrstsentie",
                    description: Some(
                        "HRSTSENT interrupt enable",
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
                    name: "txundie",
                    description: Some(
                        "TXUND interrupt enable",
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
                    name: "rxneie",
                    description: Some(
                        "RXNE interrupt enable",
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
                    name: "rxorddetie",
                    description: Some(
                        "RXORDDET interrupt enable",
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
                    name: "rxhrstdetie",
                    description: Some(
                        "RXHRSTDET interrupt enable",
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
                    name: "rxovrie",
                    description: Some(
                        "RXOVR interrupt enable",
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
                    name: "rxmsgendie",
                    description: Some(
                        "RXMSGEND interrupt enable",
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
                    name: "typecevt1ie",
                    description: Some(
                        "TYPECEVT1 interrupt enable",
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
                    name: "typecevt2ie",
                    description: Some(
                        "TYPECEVT2 interrupt enable",
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
                    name: "frsevtie",
                    description: Some(
                        "FRSEVT interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Ipid",
            extends: None,
            description: Some(
                "UCPD IP ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipid",
                    description: Some(
                        "IPID",
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
            name: "Ipver",
            extends: None,
            description: Some(
                "UCPD IP ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipver",
                    description: Some(
                        "IPVER",
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
            name: "Mid",
            extends: None,
            description: Some(
                "UCPD IP ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipid",
                    description: Some(
                        "IPID",
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
            name: "RxOrdextr1",
            extends: None,
            description: Some(
                "Rx ordered set extension register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxsopx1",
                    description: Some(
                        "Ordered set 1 received\r The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RxOrdextr2",
            extends: None,
            description: Some(
                "Rx ordered set extension register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxsopx2",
                    description: Some(
                        "Ordered set 2 received\r The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RxOrdsetr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxordset",
                    description: Some(
                        "Rx ordered set code detected",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Rxordset",
                    ),
                },
                Field {
                    name: "rxsop3of4",
                    description: Some(
                        "The bit indicates the number of correct K‑codes. For debug purposes only.",
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
                    name: "rxsopkinvalid",
                    description: Some(
                        "The bitfield is for debug purposes only.\r Others: Invalid",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Rxsopkinvalid",
                    ),
                },
            ],
        },
        FieldSet {
            name: "RxPayszr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxpaysz",
                    description: Some(
                        "Rx payload size received\r This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled).\r The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).",
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
            ],
        },
        FieldSet {
            name: "Rxdr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxdata",
                    description: Some(
                        "Data byte received",
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
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txis",
                    description: Some(
                        "Transmit interrupt status\r The flag indicates that the TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the TXDR register.",
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
                    name: "txmsgdisc",
                    description: Some(
                        "Message transmission discarded\r The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit.\r Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle.",
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
                    name: "txmsgsent",
                    description: Some(
                        "Message transmission completed\r The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit.\r In the event of a message transmission interrupted by a Hard Reset, the flag is not raised.",
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
                    name: "txmsgabt",
                    description: Some(
                        "Transmit message abort\r The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit.",
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
                    name: "hrstdisc",
                    description: Some(
                        "Hard Reset discarded\r The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit.",
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
                    name: "hrstsent",
                    description: Some(
                        "Hard Reset message sent\r The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit.",
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
                    name: "txund",
                    description: Some(
                        "Tx data underrun detection\r The flag indicates that the Tx data register (TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit.",
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
                    name: "rxne",
                    description: Some(
                        "Receive data register not empty detection\r The flag indicates that the RXDR register is not empty. It is automatically cleared upon reading RXDR.",
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
                    name: "rxorddet",
                    description: Some(
                        "Rx ordered set (4 K-codes) detection\r The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET[2:0] bitfield of the RX_ORDSET register. It is cleared by setting the RXORDDETCF bit.",
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
                    name: "rxhrstdet",
                    description: Some(
                        "Rx Hard Reset receipt detection\r The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit.",
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
                    name: "rxovr",
                    description: Some(
                        "Rx data overflow detection\r The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit.\r The buffer overflow can occur if the received data are not read fast enough.",
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
                    name: "rxmsgend",
                    description: Some(
                        "Rx message received\r The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit.\r The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message.",
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
                    name: "rxerr",
                    description: Some(
                        "Receive message error\r The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set.",
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
                    name: "typecevt1",
                    description: Some(
                        "Type-C voltage level event on CC1 line\r The flag indicates a change of the TYPEC_VSTATE_CC1[1:0] bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.",
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
                    name: "typecevt2",
                    description: Some(
                        "Type-C voltage level event on CC2 line\r The flag indicates a change of the TYPEC_VSTATE_CC2[1:0] bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.",
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
                    name: "typec_vstate_cc1",
                    description: Some(
                        "The status bitfield indicates the voltage level on the CC1 line in its steady state.\r The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "TypecVstateCc",
                    ),
                },
                Field {
                    name: "typec_vstate_cc2",
                    description: Some(
                        "CC2 line voltage level\r The status bitfield indicates the voltage level on the CC2 line in its steady state.\r The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "TypecVstateCc",
                    ),
                },
                Field {
                    name: "frsevt",
                    description: Some(
                        "FRS detection event\r The flag is cleared by setting the FRSEVTCF bit.",
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
            ],
        },
        FieldSet {
            name: "TxOrdsetr",
            extends: None,
            description: Some(
                "Tx ordered set type register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txordset",
                    description: Some(
                        "Ordered set to transmit\r The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TxPayszr",
            extends: None,
            description: Some(
                "Tx payload size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txpaysz",
                    description: Some(
                        "Payload size yet to transmit\r The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.",
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
            ],
        },
        FieldSet {
            name: "Txdr",
            extends: None,
            description: Some(
                "Tx data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txdata",
                    description: Some(
                        "Data byte to transmit",
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
            name: "Anamode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SOURCE",
                    description: Some(
                        "Source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SINK",
                    description: Some(
                        "Sink",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ccenable",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Disable both PHYs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CC1",
                    description: Some(
                        "Enable CC1 PHY",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CC2",
                    description: Some(
                        "Enable CC2 PHY",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTH",
                    description: Some(
                        "Enable CC1 and CC2 PHY",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Phyccsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CC1",
                    description: Some(
                        "Use CC1 IO for Power Delivery communication",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CC2",
                    description: Some(
                        "Use CC2 IO for Power Delivery communication",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PscUsbpdclk",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "1 (bypass)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "16",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Rxordset",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SOP",
                    description: Some(
                        "SOP code detected in receiver",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOP_PRIME",
                    description: Some(
                        "SOP' code detected in receiver",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SOP_DOUBLE_PRIME",
                    description: Some(
                        "SOP'' code detected in receiver",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SOP_PRIME_DEBUG",
                    description: Some(
                        "SOP'_Debug detected in receiver",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SOP_DOUBLE_PRIME_DEBUG",
                    description: Some(
                        "SOP''_Debug detected in receiver",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CABLE_RESET",
                    description: Some(
                        "Cable Reset detected in receiver",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "EXT1",
                    description: Some(
                        "SOP extension#1 detected in receiver",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXT2",
                    description: Some(
                        "SOP extension#2 detected in receiver",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rxsopkinvalid",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "No K‑code corrupted",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FIRST",
                    description: Some(
                        "First K‑code corrupted",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SECOND",
                    description: Some(
                        "Second K‑code corrupted",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "THIRD",
                    description: Some(
                        "Third K‑code corrupted",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FOURTH",
                    description: Some(
                        "Fourth K‑code corrupted",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Txmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PACKET",
                    description: Some(
                        "Transmission of Tx packet previously defined in other registers",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CABLE_RESET",
                    description: Some(
                        "Cable Reset sequence",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIST",
                    description: Some(
                        "BIST test sequence (BIST Carrier Mode 2)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "TypecVstateCc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOWEST",
                    description: Some(
                        "Lowest",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGHEST",
                    description: Some(
                        "Highest",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
