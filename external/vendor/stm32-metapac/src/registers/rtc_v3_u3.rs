
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "RTC register block.",
            ),
            items: &[
                BlockItem {
                    name: "tr",
                    description: Some(
                        "RTC time register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "RTC date register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ssr",
                    description: Some(
                        "RTC subsecond register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icsr",
                    description: Some(
                        "RTC initialization control and status register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prer",
                    description: Some(
                        "RTC prescaler register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wutr",
                    description: Some(
                        "RTC wake-up timer register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wutr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RTC control register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "privcr",
                    description: Some(
                        "RTC privilege mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "RTC secure configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "wpr",
                    description: Some(
                        "RTC write protection register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calr",
                    description: Some(
                        "RTC calibration register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shiftr",
                    description: Some(
                        "RTC shift control register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Shiftr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tstr",
                    description: Some(
                        "RTC timestamp time register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsdr",
                    description: Some(
                        "RTC timestamp date register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsssr",
                    description: Some(
                        "RTC timestamp subsecond register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrmr",
                    description: Some(
                        "RTC alarm register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrmssr",
                    description: Some(
                        "RTC alarm subsecond register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrmssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "RTC status register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                    name: "misr",
                    description: Some(
                        "RTC nonsecure masked interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smisr",
                    description: Some(
                        "RTC secure masked interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Smisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scr",
                    description: Some(
                        "RTC status clear register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tamptscr",
                    description: Some(
                        "RTC timestamp on tamper control register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tamptscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsidr",
                    description: Some(
                        "RTC timestamp status register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrbinr",
                    description: Some(
                        "RTC alarm binary mode register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrbinr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alrbinr",
            extends: None,
            description: Some(
                "RTC alarm binary mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous counter alarm value in Binary mode.",
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
            name: "Alrmr",
            extends: None,
            description: Some(
                "RTC alarm register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format.",
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
                    name: "st",
                    description: Some(
                        "Second tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk1",
                    description: Some(
                        "Alarm seconds mask.",
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
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk2",
                    description: Some(
                        "Alarm A minutes mask.",
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
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format.",
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
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ampm",
                    ),
                },
                Field {
                    name: "msk3",
                    description: Some(
                        "Alarm hours mask.",
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
                    name: "du",
                    description: Some(
                        "Date units or day in BCD format.",
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
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdsel",
                    description: Some(
                        "Week day selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrWdsel",
                    ),
                },
                Field {
                    name: "msk4",
                    description: Some(
                        "Alarm date mask.",
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
            name: "Alrmssr",
            extends: None,
            description: Some(
                "RTC alarm A subsecond register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Subseconds value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "maskss",
                    description: Some(
                        "Mask the most-significant bits starting at this bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssclr",
                    description: Some(
                        "Clear synchronous counter on alarm (Binary mode only).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmssrSsclr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Calr",
            extends: None,
            description: Some(
                "RTC calibration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calm",
                    description: Some(
                        "Calibration minus.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpcal",
                    description: Some(
                        "RTC low-power mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpcal",
                    ),
                },
                Field {
                    name: "calw16",
                    description: Some(
                        "Use a 16-second calibration cycle period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calw16",
                    ),
                },
                Field {
                    name: "calw8",
                    description: Some(
                        "Use an 8-second calibration cycle period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calw8",
                    ),
                },
                Field {
                    name: "calp",
                    description: Some(
                        "Increase frequency of RTC by 488.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "RTC control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wucksel",
                    description: Some(
                        "ck_wut wake-up clock selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Wucksel",
                    ),
                },
                Field {
                    name: "tsedge",
                    description: Some(
                        "Timestamp event active edge.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsedge",
                    ),
                },
                Field {
                    name: "refckon",
                    description: Some(
                        "REFIN reference clock detection enable (50 or 60Hz).",
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
                    name: "bypshad",
                    description: Some(
                        "Bypass the shadow registers.",
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
                    name: "fmt",
                    description: Some(
                        "Hour format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmt",
                    ),
                },
                Field {
                    name: "ssruie",
                    description: Some(
                        "SSR underflow interrupt enable.",
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
                    name: "alre",
                    description: Some(
                        "Alarm enable.",
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
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "wute",
                    description: Some(
                        "Wake-up timer enable.",
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
                    name: "tse",
                    description: Some(
                        "timestamp enable.",
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
                    name: "alrie",
                    description: Some(
                        "Alarm interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
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
                    name: "wutie",
                    description: Some(
                        "Wake-up timer interrupt enable.",
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
                    name: "tsie",
                    description: Some(
                        "Timestamp interrupt enable.",
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
                    name: "add1h",
                    description: Some(
                        "Add 1 hour (summer time change).",
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
                    name: "sub1h",
                    description: Some(
                        "Subtract 1 hour (winter time change).",
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
                    name: "bkp",
                    description: Some(
                        "Backup.",
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
                    name: "cosel",
                    description: Some(
                        "Calibration output selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cosel",
                    ),
                },
                Field {
                    name: "pol",
                    description: Some(
                        "Output polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pol",
                    ),
                },
                Field {
                    name: "osel",
                    description: Some(
                        "Output selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Osel",
                    ),
                },
                Field {
                    name: "coe",
                    description: Some(
                        "Calibration output enable.",
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
                    name: "itse",
                    description: Some(
                        "timestamp on internal event enable.",
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
                    name: "tampts",
                    description: Some(
                        "Activate timestamp on tamper detection event.",
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
                    name: "tampoe",
                    description: Some(
                        "Tamper detection output enable on TAMPALRM.",
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
                    name: "alrfclr",
                    description: Some(
                        "Alarm A flag automatic clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
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
                    name: "tampalrm_pu",
                    description: Some(
                        "TAMPALRM pull-up enable.",
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
                    name: "tampalrm_type",
                    description: Some(
                        "TAMPALRM output type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TampalrmType",
                    ),
                },
                Field {
                    name: "out2en",
                    description: Some(
                        "OUT2 output enable.",
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
            name: "Dr",
            extends: None,
            description: Some(
                "RTC date register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format.",
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
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format.",
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
                    name: "wdu",
                    description: Some(
                        "Week day units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "yu",
                    description: Some(
                        "Year units in BCD format.",
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
                    name: "yt",
                    description: Some(
                        "Year tens in BCD format.",
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
            ],
        },
        FieldSet {
            name: "Icsr",
            extends: None,
            description: Some(
                "RTC initialization control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wutwf",
                    description: Some(
                        "Wake-up timer write flag.",
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
                    name: "shpf",
                    description: Some(
                        "Shift operation pending.",
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
                    name: "inits",
                    description: Some(
                        "Initialization status flag.",
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
                    name: "rsf",
                    description: Some(
                        "Registers synchronization flag.",
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
                    name: "initf",
                    description: Some(
                        "Initialization flag.",
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
                    name: "init",
                    description: Some(
                        "Initialization mode.",
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
                    name: "bin",
                    description: Some(
                        "Binary mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Bin",
                    ),
                },
                Field {
                    name: "bcdu",
                    description: Some(
                        "BCD update (BIN = 10 or 11).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Bcdu",
                    ),
                },
                Field {
                    name: "recalpf",
                    description: Some(
                        "Recalibration pending Flag.",
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
            name: "Misr",
            extends: None,
            description: Some(
                "RTC nonsecure masked interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrmf",
                    description: Some(
                        "Alarm A masked flag.",
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
                    name: "wutmf",
                    description: Some(
                        "Wake-up timer nonsecure masked flag.",
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
                    name: "tsmf",
                    description: Some(
                        "Timestamp nonsecure masked flag.",
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
                    name: "tsovmf",
                    description: Some(
                        "Timestamp overflow nonsecure masked flag.",
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
                    name: "itsmf",
                    description: Some(
                        "Internal timestamp nonsecure masked flag.",
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
                    name: "ssrumf",
                    description: Some(
                        "SSR underflow nonsecure masked flag.",
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
            name: "Prer",
            extends: None,
            description: Some(
                "RTC prescaler register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prediv_s",
                    description: Some(
                        "Synchronous prescaler factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prediv_a",
                    description: Some(
                        "Asynchronous prescaler factor.",
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
            name: "Privcr",
            extends: None,
            description: Some(
                "RTC privilege mode control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrpriv",
                    description: Some(
                        "Alarm and SSR underflow privilege protection.",
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
                    name: "wutpriv",
                    description: Some(
                        "Wake-up timer privilege protection.",
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
                    name: "tspriv",
                    description: Some(
                        "Timestamp privilege protection.",
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
                    name: "calpriv",
                    description: Some(
                        "Shift register, Delight saving, calibration and reference clock privilege protection.",
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
                    name: "initpriv",
                    description: Some(
                        "Initialization privilege protection.",
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
                    name: "priv_",
                    description: Some(
                        "RTC privilege protection.",
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
            name: "Scr",
            extends: None,
            description: Some(
                "RTC status clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calrf",
                    description: Some(
                        "Clear alarm A flag.",
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
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "cwutf",
                    description: Some(
                        "Clear wake-up timer flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "ctsf",
                    description: Some(
                        "Clear timestamp flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "ctsovf",
                    description: Some(
                        "Clear timestamp overflow flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "citsf",
                    description: Some(
                        "Clear internal timestamp flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "cssruf",
                    description: Some(
                        "Clear SSR underflow flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "RTC secure configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrasec",
                    description: Some(
                        "Alarm A and SSR underflow protection.",
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
                    name: "alrbsec",
                    description: Some(
                        "Alarm B protection.",
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
                    name: "wutsec",
                    description: Some(
                        "Wake-up timer protection.",
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
                    name: "tssec",
                    description: Some(
                        "Timestamp protection.",
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
                    name: "calsec",
                    description: Some(
                        "Shift register, daylight saving, calibration and reference clock protection.",
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
                    name: "initsec",
                    description: Some(
                        "Initialization protection.",
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
                    name: "sec",
                    description: Some(
                        "RTC global protection.",
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
            name: "Shiftr",
            extends: None,
            description: Some(
                "RTC shift control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subfs",
                    description: Some(
                        "Subtract a fraction of a second.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add1s",
                    description: Some(
                        "Add one second.",
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
            name: "Smisr",
            extends: None,
            description: Some(
                "RTC secure masked interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrmf",
                    description: Some(
                        "Alarm A interrupt secure masked flag.",
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
                    name: "wutmf",
                    description: Some(
                        "Wake-up timer interrupt secure masked flag.",
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
                    name: "tsmf",
                    description: Some(
                        "Timestamp interrupt secure masked flag.",
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
                    name: "tsovmf",
                    description: Some(
                        "Timestamp overflow interrupt secure masked flag.",
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
                    name: "itsmf",
                    description: Some(
                        "Internal timestamp interrupt secure masked flag.",
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
                    name: "ssrumf",
                    description: Some(
                        "SSR underflow secure masked flag.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "RTC status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrf",
                    description: Some(
                        "Alarm flag.",
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
                    name: "wutf",
                    description: Some(
                        "Wake-up timer flag.",
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
                    name: "tsf",
                    description: Some(
                        "Timestamp flag.",
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
                    name: "tsovf",
                    description: Some(
                        "Timestamp overflow flag.",
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
                    name: "itsf",
                    description: Some(
                        "Internal timestamp flag.",
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
                    name: "ssruf",
                    description: Some(
                        "SSR underflow flag.",
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
            name: "Ssr",
            extends: None,
            description: Some(
                "RTC subsecond register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous binary counter.",
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
            name: "Tamptscr",
            extends: None,
            description: Some(
                "RTC timestamp on tamper control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampts",
                    description: Some(
                        "Timestamp on external tamper TAMP1 event.",
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
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampts",
                    description: Some(
                        "Timestamp on internal tamper event.",
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
            name: "Tr",
            extends: None,
            description: Some(
                "RTC time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format.",
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
                    name: "st",
                    description: Some(
                        "Second tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format.",
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
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ampm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Tsdr",
            extends: None,
            description: Some(
                "RTC timestamp date register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format.",
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
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format.",
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
                    name: "wdu",
                    description: Some(
                        "Week day units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tsidr",
            extends: None,
            description: Some(
                "RTC timestamp status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsid",
                    description: Some(
                        "Timestamp flag source identification.",
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
            name: "Tsssr",
            extends: None,
            description: Some(
                "RTC timestamp subsecond register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Subsecond value/synchronous binary counter values.",
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
            name: "Tstr",
            extends: None,
            description: Some(
                "RTC timestamp time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format.",
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
                    name: "st",
                    description: Some(
                        "Second tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format.",
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
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ampm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wpr",
            extends: None,
            description: Some(
                "RTC write protection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Write protection key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Key",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wutr",
            extends: None,
            description: Some(
                "RTC wake-up timer register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wut",
                    description: Some(
                        "Wake-up auto-reload value bits.",
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
                Field {
                    name: "wutoclr",
                    description: Some(
                        "Wake-up auto-reload output clear value.",
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
    ],
    enums: &[
        Enum {
            name: "AlrmrWdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATE_UNITS",
                    description: Some(
                        "DU[3:0] represents the date units.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WEEK_DAY",
                    description: Some(
                        "DU[3:0] represents the week day.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmssrSsclr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FREE_RUNNING",
                    description: Some(
                        "The synchronous binary counter (SS[31:0] in SSR) is free-running.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALRMBINR",
                    description: Some(
                        "The synchronous binary counter (SS[31:0] in SSR) is running from 0xFFFFFFFF to ALRBBINR.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ampm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AM",
                    description: Some(
                        "AM or 24-hour format.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PM",
                    description: Some(
                        "PM.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bcdu",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BIT7",
                    description: Some(
                        "1s calendar increment is generated each time SS[7:0] = 0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT8",
                    description: Some(
                        "1s calendar increment is generated each time SS[8:0] = 0.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIT9",
                    description: Some(
                        "1s calendar increment is generated each time SS[9:0] = 0.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "1s calendar increment is generated each time SS[10:0] = 0.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BIT11",
                    description: Some(
                        "1s calendar increment is generated each time SS[11:0] = 0.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BIT12",
                    description: Some(
                        "1s calendar increment is generated each time SS[12:0] = 0.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BIT13",
                    description: Some(
                        "1s calendar increment is generated each time SS[13:0] = 0.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BIT14",
                    description: Some(
                        "1s calendar increment is generated each time SS[14:0] = 0.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Bin",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BCD",
                    description: Some(
                        "Free running BCD calendar mode (Binary mode disabled).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BINARY",
                    description: Some(
                        "Free running Binary mode (BCD mode disabled).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIN_BCD",
                    description: Some(
                        "Free running BCD calendar and Binary modes.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIN_BCD2",
                    description: Some(
                        "Free running BCD calendar and Binary modes.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Calp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_CHANGE",
                    description: Some(
                        "No RTCCLK pulses are added.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREASE_FREQ",
                    description: Some(
                        "One RTCCLK pulse is effectively inserted every 2less thansup>11less than/sup> pulses (frequency increased by 488.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calrf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear interrupt flag by writing 1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calw16",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SIXTEEN_SECONDS",
                    description: Some(
                        "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calw8",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EIGHT_SECONDS",
                    description: Some(
                        "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CAL_FREQ_512HZ",
                    description: Some(
                        "Calibration output is 512Hz.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CAL_FREQ_1HZ",
                    description: Some(
                        "Calibration output is 1Hz.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fmt",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TWENTY_FOUR_HOUR",
                    description: Some(
                        "24 hour/day format.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AM_PM",
                    description: Some(
                        "AM/PM hour format.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Key",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "ACTIVATE",
                    description: Some(
                        "Activate write protection (any value that is not the keys)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEACTIVATE2",
                    description: Some(
                        "Key 2",
                    ),
                    value: 83,
                },
                EnumVariant {
                    name: "DEACTIVATE1",
                    description: Some(
                        "Key 1",
                    ),
                    value: 202,
                },
            ],
        },
        Enum {
            name: "Lpcal",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RTCCLK",
                    description: Some(
                        "Calibration window is 2less thansup>20less than/sup> RTCCLK, which is a high-consumption mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CK_APRE",
                    description: Some(
                        "Calibration window is 2less thansup>20less than/sup> ck_apre, which is the required configuration for ultra-low consumption mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Osel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Output disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALARM_A",
                    description: Some(
                        "Alarm A output enabled.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALARM_B",
                    description: Some(
                        "Alarm B output enabled.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WAKE_UP",
                    description: Some(
                        "Wake-up output enabled.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TampalrmType",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PUSH_PULL",
                    description: Some(
                        "TAMPALRM is push-pull output.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPEN_DRAIN",
                    description: Some(
                        "TAMPALRM is open-drain output.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsedge",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "TS input rising edge generates a timestamp event.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "TS input falling edge generates a timestamp event.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wucksel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "RTC/16 clock is selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "RTC/8 clock is selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "RTC/4 clock is selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "RTC/2 clock is selected.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
