
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "Real-time clock",
            ),
            items: &[
                BlockItem {
                    name: "tr",
                    description: Some(
                        "Time register",
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
                        "Date register",
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
                        "Sub second register",
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
                        "Initialization control and status register",
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
                        "Prescaler register",
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
                        "Wakeup timer register",
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
                        "Control register",
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
                    name: "privcfgr",
                    description: Some(
                        "Privilege mode control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "wpr",
                    description: Some(
                        "Write protection register",
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
                        "Calibration register",
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
                        "Shift control register",
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
                        "Timestamp time register",
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
                        "Timestamp date register",
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
                        "Timestamp sub second register",
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
                        "Alarm register",
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
                        "Alarm sub second register",
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
                        "Status register",
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
                        "Masked interrupt status register",
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
                    name: "scr",
                    description: Some(
                        "Status clear register",
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
                    name: "alrbinr",
                    description: Some(
                        "Alarm binary mode register",
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
                "RTC alarm binary mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous counter alarm value in Binary mode. This value is compared with the contents of the synchronous counter to determine if Alarm is to be activated. Only bits 0 up MASKSS-1 are compared. SS[14:0] is the mirror of SS[14:0] in the ALRMSSR, and so can also be read or written through ALRMSSR.",
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
                "Alarm register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
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
                        "Second tens in BCD format",
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
                        "Alarm seconds mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
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
                        "Minute tens in BCD format",
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
                        "Alarm minutes mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
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
                        "Hour tens in BCD format",
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
                        "AM/PM notation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrPm",
                    ),
                },
                Field {
                    name: "msk3",
                    description: Some(
                        "Alarm hours mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "du",
                    description: Some(
                        "Date units or day in BCD format",
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
                        "Date tens in BCD format",
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
                        "Week day selection",
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
                        "Alarm date mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Alrmssr",
            extends: None,
            description: Some(
                "Alarm sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Subseconds value. This value is compared with the contents of the synchronous prescaler counter to determine if alarm is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS[14:0] in the ALRBINR, and so can also be read or written through ALRBINR.",
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
                        "Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation.",
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
                        "Clear synchronous counter on alarm (Binary mode only)",
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
                "Calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calm",
                    description: Some(
                        "Calibration minus. The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP.",
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
                        "Calibration low-power mode",
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
                        "Use a 16-second calibration cycle period. When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM[0] is stuck at 0 when CALW16 = 1.",
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
                        "Use an 8-second calibration cycle period. When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM[1:0] are stuck at 00 when CALW8 = 1.",
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
                        "Increase frequency of RTC by 488.5 ppm",
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
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wucksel",
                    description: Some(
                        "Wakeup clock selection. 10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. 11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. Furthermore, 2<sup>16</sup> is added to the WUT counter value.",
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
                        "Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.",
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
                        "REFIN reference clock detection enable (50 or 60 Hz) Note: BIN must be 0x00 and PREDIV_S must be 0x00FF.",
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
                        "Bypass the shadow registers. Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.",
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
                        "Hour format",
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
                        "SSR underflow interrupt enable",
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
                        "Alarm enable",
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
                        "Wakeup timer enable",
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
                        "Timestamp enable",
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
                        "Alarm interrupt enable",
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
                        "Wakeup timer interrupt enable",
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
                        "Timestamp interrupt enable",
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
                        "Add 1 hour (summer time change). When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.",
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
                        "Subtract 1 hour (winter time change). When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.",
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
                        "Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.",
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
                        "Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 45.3.17: Calibration clock output.",
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
                        "Output polarity. This bit is used to configure the polarity of TAMPALRM output.",
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
                        "Output selection. These bits are used to select the flag to be routed to TAMPALRM output.",
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
                        "Calibration output enable. This bit enables the CALIB output.",
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
                        "Timestamp on internal event enable",
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
                        "Activate timestamp on tamper detection event. TAMPTS is valid even if TSE = 0 in the CR register. Timestamp flag is set up to 3 ck_apre cycles after the tamper flags.",
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
                        "Tamper detection output enable on TAMPALRM",
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
                        "Alarm flag automatic clear.",
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
                    enumm: Some(
                        "Alrfclr",
                    ),
                },
                Field {
                    name: "tampalrm_pu",
                    description: Some(
                        "TAMPALRM pull-up enable",
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
                        "TAMPALRM output type",
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
                        "OUT2 output enable With this bit set, the RTC outputs can be remapped on OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different from 00 or TAMPOE = 1: TAMPALRM is output on OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different from 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on OUT2 If (OSEL different from 00 or TAMPOE = 1) and COE = 1: CALIB is output on OUT2 and TAMPALRM is output on OUT1.",
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
                "Date register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format",
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
                        "Date tens in BCD format",
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
                        "Month units in BCD format",
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
                        "Month tens in BCD format",
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
                        "Week day units",
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
                        "Year units in BCD format",
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
                        "Year tens in BCD format",
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
                "Initialization control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wutwf",
                    description: Some(
                        "Wakeup timer write flag. This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in CR. It is cleared by hardware in initialization mode.",
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
                        "Shift operation pending. This flag is set by hardware as soon as a shift operation is initiated by a write to the SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.",
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
                        "Initialization status flag. This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).",
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
                        "Registers synchronization flag. This bit is set by hardware each time the calendar registers are copied into the shadow registers (SSR, TR and DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.",
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
                        "Initialization flag. When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.",
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
                        "Initialization mode",
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
                        "Binary mode",
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
                        "BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits.",
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
                        "Recalibration pending Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Recalpf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Misr",
            extends: None,
            description: Some(
                "Masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrmf",
                    description: Some(
                        "Alarm masked flag",
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
                        "Alrmf",
                    ),
                },
                Field {
                    name: "wutmf",
                    description: Some(
                        "Wakeup timer masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wutmf",
                    ),
                },
                Field {
                    name: "tsmf",
                    description: Some(
                        "Timestamp masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsmf",
                    ),
                },
                Field {
                    name: "tsovmf",
                    description: Some(
                        "Timestamp overflow masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovmf",
                    ),
                },
                Field {
                    name: "itsmf",
                    description: Some(
                        "Internal timestamp masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Itsmf",
                    ),
                },
                Field {
                    name: "ssrumf",
                    description: Some(
                        "SSR underflow masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssrumf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Prer",
            extends: None,
            description: Some(
                "Prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prediv_s",
                    description: Some(
                        "Synchronous prescaler factor. This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1).",
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
                        "Asynchronous prescaler factor. This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1).",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "Privilege mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrpriv",
                    description: Some(
                        "Alarm and SSR underflow privilege protection",
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
                        "Priv",
                    ),
                },
                Field {
                    name: "wutpriv",
                    description: Some(
                        "Wakeup timer privilege protection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
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
                    enumm: Some(
                        "Priv",
                    ),
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
                    enumm: Some(
                        "Priv",
                    ),
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
                    enumm: Some(
                        "Priv",
                    ),
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
                    enumm: Some(
                        "Priv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some(
                "Status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calrf",
                    description: Some(
                        "Clear alarm x flag",
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
                        "Clear wakeup timer flag",
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
                        "Clear timestamp flag",
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
                        "Clear timestamp overflow flag",
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
                        "Clear internal timestamp flag",
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
                        "Clear SSR underflow flag",
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
            name: "Shiftr",
            extends: None,
            description: Some(
                "Shift control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subfs",
                    description: Some(
                        "Subtract a fraction of a second. These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in ICSR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). In mixed BCD-binary mode (BIN=10 or 11), the SUBFS[14:BCDU+8] must be written with 0. Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be sure that the shadow registers have been updated with the shifted time.",
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
                        "Add one second. This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrf",
                    description: Some(
                        "Alarm flag",
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
                        "Alrf",
                    ),
                },
                Field {
                    name: "wutf",
                    description: Some(
                        "Wakeup timer flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wutf",
                    ),
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "Timestamp flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsf",
                    ),
                },
                Field {
                    name: "tsovf",
                    description: Some(
                        "Timestamp overflow flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovf",
                    ),
                },
                Field {
                    name: "itsf",
                    description: Some(
                        "Internal timestamp flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Itsf",
                    ),
                },
                Field {
                    name: "ssruf",
                    description: Some(
                        "SSR underflow flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssruf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ssr",
            extends: None,
            description: Some(
                "Sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous binary counter. SS[31:16]: Synchronous binary counter MSB values When Binary or Mixed mode is selected (BIN = 01 or 10 or 11): SS[31:16] are the 16 MSB of the SS[31:0] free-running down-counter. When BCD mode is selected (BIN=00): SS[31:16] are forced by hardware to 0x0000. SS[15:0]: Subsecond value/synchronous binary counter LSB values When Binary mode is selected (BIN = 01 or 10 or 11): SS[15:0] are the 16 LSB of the SS[31:0] free-running down-counter. When BCD mode is selected (BIN=00): SS[15:0] is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by TR/DR.",
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
            name: "Tr",
            extends: None,
            description: Some(
                "Time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
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
                        "Second tens in BCD format",
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
                        "Minute units in BCD format",
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
                        "Minute tens in BCD format",
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
                        "Hour units in BCD format",
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
                        "Hour tens in BCD format",
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
                        "AM/PM notation",
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
                "Timestamp date register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format",
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
                        "Date tens in BCD format",
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
                        "Month units in BCD format",
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
                        "Month tens in BCD format",
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
                        "Week day units",
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
            name: "Tsssr",
            extends: None,
            description: Some(
                "Timestamp sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Subsecond value. Synchronous binary counter values SS[31:0] is the value of the synchronous prescaler counter when the timestamp event occurred.",
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
                "Timestamp time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
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
                        "Second tens in BCD format",
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
                        "Minute units in BCD format",
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
                        "Minute tens in BCD format",
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
                        "Hour units in BCD format",
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
                        "Hour tens in BCD format",
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
                        "AM/PM notation",
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
            ],
        },
        FieldSet {
            name: "Wpr",
            extends: None,
            description: Some(
                "Write protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Write protection key. This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection.",
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
                "Wakeup timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wut",
                    description: Some(
                        "Wakeup auto-reload value bits. When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT[15:0] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL[2:0] bits of the CR register. When WUCKSEL[2] = 1, the wakeup timer becomes 17-bits and WUCKSEL[1] effectively becomes WUT[16] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT[15:0] to 0x0000 with WUCKSEL[2:0] = 011 (RTCCLK/2) is forbidden.",
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
                        "Wakeup auto-reload output clear value. When WUTOCLR[15:0] is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR[15:0]. When WUTOCLR[15:0] = 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software.",
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
            name: "Alrf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MATCH",
                    description: Some(
                        "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Alrfclr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MANUAL",
                    description: Some(
                        "Alarm x event generates a trigger event and ALRxF must be cleared by software to allow next alarm event.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AUTOMATIC",
                    description: Some(
                        "Alarm x event generates a trigger event. ALRxF is automatically cleared by hardware after 1 ck_apre cycle.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Alrmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MATCH",
                    description: Some(
                        "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrMsk",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TO_MATCH",
                    description: Some(
                        "Alarm set if the date/day match",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOT_MATCH",
                    description: Some(
                        "Date/day dont care in Alarm comparison",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrPm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AM",
                    description: Some(
                        "AM or 24-hour format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PM",
                    description: Some(
                        "PM",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrWdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATE_UNITS",
                    description: Some(
                        "DU[3:0] represents the date units",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WEEK_DAY",
                    description: Some(
                        "DU[3:0] represents the week day. DT[1:0] is dont care.",
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
                        "The synchronous binary counter (SS[31:0] in RTC_SSR) is free-running",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALRMBINR",
                    description: Some(
                        "The synchronous binary counter (SS[31:0] in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRBINR → SS[31:0] value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRBINR → SS[31:0]",
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
                        "AM or 24-hour format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PM",
                    description: Some(
                        "PM",
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
                        "1s calendar increment is generated each time SS[7:0] = 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT8",
                    description: Some(
                        "1s calendar increment is generated each time SS[8:0] = 0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIT9",
                    description: Some(
                        "1s calendar increment is generated each time SS[9:0] = 0",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "1s calendar increment is generated each time SS[10:0] = 0",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BIT11",
                    description: Some(
                        "1s calendar increment is generated each time SS[11:0] = 0",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BIT12",
                    description: Some(
                        "1s calendar increment is generated each time SS[12:0] = 0",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BIT13",
                    description: Some(
                        "1s calendar increment is generated each time SS[13:0] = 0",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BIT14",
                    description: Some(
                        "1s calendar increment is generated each time SS[14:0] = 0",
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
                        "Free running BCD calendar mode (Binary mode disabled)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BINARY",
                    description: Some(
                        "Free running Binary mode (BCD mode disabled)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIN_BCD",
                    description: Some(
                        "Free running BCD calendar and Binary modes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIN_BCD2",
                    description: Some(
                        "Free running BCD calendar and Binary modes",
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
                        "No RTCCLK pulses are added",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREASE_FREQ",
                    description: Some(
                        "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)",
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
                        "Calibration output is 512 Hz (with default prescaler setting)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CAL_FREQ_1HZ",
                    description: Some(
                        "Calibration output is 1 Hz (with default prescaler setting)",
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
                        "24 hour/day format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AM_PM",
                    description: Some(
                        "AM/PM hour format",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Itsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a timestamp on the internal event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Itsmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a timestamp on the internal event occurs",
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
                        "Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CK_APRE",
                    description: Some(
                        "Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode",
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
                        "Output disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALARM_A",
                    description: Some(
                        "Alarm A output enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALARM_B",
                    description: Some(
                        "Alarm B output enabled",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WAKEUP",
                    description: Some(
                        "Wakeup output enabled",
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
            name: "Priv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANYTIME",
                    description: Some(
                        "Alarm and SSR underflow configuration and interrupt clear can be written when the APB access is privileged or non-privileged.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRIVILEGED",
                    description: Some(
                        "Alarm and SSR underflow configuration and interrupt clear can be written only when the APB access is privileged.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Recalpf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PENDING",
                    description: Some(
                        "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ssruf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNDERFLOW",
                    description: Some(
                        "This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ssrumf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNDERFLOW",
                    description: Some(
                        "This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1",
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
                        "TAMPALRM is push-pull output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPEN_DRAIN",
                    description: Some(
                        "TAMPALRM is open-drain output",
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
                        "RTC_TS input rising edge generates a time-stamp event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "RTC_TS input falling edge generates a time-stamp event",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsovf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OVERFLOW",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs while TSF is already set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsovmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OVERFLOW",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs while TSF is already set",
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
                        "RTC/16 clock is selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "RTC/8 clock is selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "RTC/4 clock is selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "RTC/2 clock is selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CLOCK_SPARE",
                    description: Some(
                        "ck_spre (usually 1 Hz) clock is selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CLOCK_SPARE_WITH_OFFSET",
                    description: Some(
                        "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Wutf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ZERO",
                    description: Some(
                        "This flag is set by hardware when the wakeup auto-reload counter reaches 0",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wutmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ZERO",
                    description: Some(
                        "This flag is set by hardware when the wakeup auto-reload counter reaches 0",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
