
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usb",
            extends: None,
            description: Some(
                "Universal serial bus full-speed device interface",
            ),
            items: &[
                BlockItem {
                    name: "epr",
                    description: Some(
                        "endpoint register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Epr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "istr",
                    description: Some(
                        "interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Istr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fnr",
                    description: Some(
                        "frame number register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Fnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daddr",
                    description: Some(
                        "device address",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btable",
                    description: Some(
                        "Buffer table address",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Btable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmcsr",
                    description: Some(
                        "LPM control and status register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpmcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bcdr",
                    description: Some(
                        "Battery Charging Detector",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcdr",
            extends: None,
            description: Some(
                "Battery Charging Detector",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bcden",
                    description: Some(
                        "Battery charging detector mode enable",
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
                    name: "dcden",
                    description: Some(
                        "Data contact detection mode enable",
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
                    name: "pden",
                    description: Some(
                        "Primary detection mode enable",
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
                    name: "sden",
                    description: Some(
                        "Secondary detection mode enable",
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
                    name: "dcdet",
                    description: Some(
                        "Data contact detection status",
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
                    name: "pdet",
                    description: Some(
                        "Primary detection status",
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
                    name: "sdet",
                    description: Some(
                        "Secondary detection status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdet",
                    ),
                },
                Field {
                    name: "ps2det",
                    description: Some(
                        "DM pull-up detection status",
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
                    name: "dppu",
                    description: Some(
                        "DP pull-up control",
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
            name: "Btable",
            extends: None,
            description: Some(
                "Buffer table address",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "btable",
                    description: Some(
                        "BTABLE",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cntr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fres",
                    description: Some(
                        "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB",
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
                    name: "pdwn",
                    description: Some(
                        "Enter power down mode",
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
                    name: "lpmode",
                    description: Some(
                        "Enter low-power mode",
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
                    name: "fsusp",
                    description: Some(
                        "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected",
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
                    name: "resume",
                    description: Some(
                        "Resume request",
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
                    name: "l1resume",
                    description: Some(
                        "LPM L1 request request",
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
                    name: "l1reqm",
                    description: Some(
                        "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "esofm",
                    description: Some(
                        "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "sofm",
                    description: Some(
                        "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "resetm",
                    description: Some(
                        "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "suspm",
                    description: Some(
                        "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "wkupm",
                    description: Some(
                        "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "errm",
                    description: Some(
                        "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "pmaovrm",
                    description: Some(
                        "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
                    name: "ctrm",
                    description: Some(
                        "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set",
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
            name: "Daddr",
            extends: None,
            description: Some(
                "device address",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "add",
                    description: Some(
                        "device address",
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
                    name: "ef",
                    description: Some(
                        "USB device enabled",
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
            name: "Epr",
            extends: None,
            description: Some(
                "endpoint register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ea",
                    description: Some(
                        "EA",
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
                    name: "stat_tx",
                    description: Some(
                        "STAT_TX",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Stat",
                    ),
                },
                Field {
                    name: "dtog_tx",
                    description: Some(
                        "DTOG_TX",
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
                    name: "ctr_tx",
                    description: Some(
                        "CTR_TX",
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
                    name: "ep_kind",
                    description: Some(
                        "EP_KIND",
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
                    name: "ep_type",
                    description: Some(
                        "EPTYPE",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpType",
                    ),
                },
                Field {
                    name: "setup",
                    description: Some(
                        "SETUP",
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
                    name: "stat_rx",
                    description: Some(
                        "STAT_RX",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Stat",
                    ),
                },
                Field {
                    name: "dtog_rx",
                    description: Some(
                        "DTOG_RX",
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
                    name: "ctr_rx",
                    description: Some(
                        "CTR_RX",
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
            name: "Fnr",
            extends: None,
            description: Some(
                "frame number register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fn_",
                    description: Some(
                        "FN",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsof",
                    description: Some(
                        "LSOF",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lck",
                    description: Some(
                        "the frame timer remains in this state until an USB reset or USB suspend event occurs",
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
                    name: "rxdm",
                    description: Some(
                        "received data minus upstream port data line",
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
                    name: "rxdp",
                    description: Some(
                        "received data plus upstream port data line",
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
            name: "Istr",
            extends: None,
            description: Some(
                "interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ep_id",
                    description: Some(
                        "EP_ID",
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
                    name: "dir",
                    description: Some(
                        "DIR",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "l1req",
                    description: Some(
                        "LPM command to enter the L1 state is successfully received and acknowledged",
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
                    name: "esof",
                    description: Some(
                        "an SOF packet is expected but not received",
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
                    name: "sof",
                    description: Some(
                        "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus",
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
                    name: "reset",
                    description: Some(
                        "peripheral detects an active USB RESET signal at its inputs",
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
                    name: "susp",
                    description: Some(
                        "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus",
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
                    name: "wkup",
                    description: Some(
                        "activity is detected that wakes up the USB peripheral",
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
                    name: "err",
                    description: Some(
                        "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred",
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
                    name: "pmaovr",
                    description: Some(
                        "microcontroller has not been able to respond in time to an USB memory request",
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
                    name: "ctr",
                    description: Some(
                        "endpoint has successfully completed a transaction",
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
            name: "Lpmcsr",
            extends: None,
            description: Some(
                "LPM control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpmen",
                    description: Some(
                        "enable the LPM support within the USB device",
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
                    name: "lpmack",
                    description: Some(
                        "LPMACK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpmack",
                    ),
                },
                Field {
                    name: "remwake",
                    description: Some(
                        "REMWAKE",
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
                    name: "besl",
                    description: Some(
                        "BESL",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TO",
                    description: Some(
                        "data transmitted by the USB peripheral to the host PC",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FROM",
                    description: Some(
                        "data received by the USB peripheral from the host PC",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EpType",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BULK",
                    description: Some(
                        "Bulk endpoint",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONTROL",
                    description: Some(
                        "Control endpoint",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ISO",
                    description: Some(
                        "Iso endpoint",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "INTERRUPT",
                    description: Some(
                        "Interrupt endpoint",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpmack",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NYET",
                    description: Some(
                        "the valid LPM Token will be NYET",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "the valid LPM Token will be ACK",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdet",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CDP",
                    description: Some(
                        "CDP detected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DCP",
                    description: Some(
                        "DCP detected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stat",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "all requests addressed to this endpoint are ignored",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "the endpoint is stalled and all requests result in a STALL handshake",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NAK",
                    description: Some(
                        "the endpoint is naked and all requests result in a NAK handshake",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VALID",
                    description: Some(
                        "this endpoint is enabled, requests are ACKed",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
