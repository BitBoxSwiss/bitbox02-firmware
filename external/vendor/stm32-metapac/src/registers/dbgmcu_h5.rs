
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "Microcontroller debug unit.",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: Some(
                        "DBGMCU identity code register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idcode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "DBGMCU configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "apb1lfzr",
                    description: Some(
                        "DBGMCU APB1L peripheral freeze register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lfzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hfzr",
                    description: Some(
                        "DBGMCU APB1H peripheral freeze register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hfzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2fzr",
                    description: Some(
                        "DBGMCU APB2 peripheral freeze register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3fzr",
                    description: Some(
                        "DBGMCU APB3 peripheral freeze register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1fzr",
                    description: Some(
                        "DBGMCU AHB1 peripheral freeze register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1fzr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "DBGMCU status register.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
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
                    name: "auth_host",
                    description: Some(
                        "DBGMCU debug authentication mailbox host register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "auth_device",
                    description: Some(
                        "DBGMCU debug authentication mailbox device register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "auth_ack",
                    description: Some(
                        "DBGMCU debug authentication mailbox acknowledge register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AuthAck",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr4",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 4.",
                    ),
                    array: None,
                    byte_offset: 0xfd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr0",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 0.",
                    ),
                    array: None,
                    byte_offset: 0xfe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr1",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 1.",
                    ),
                    array: None,
                    byte_offset: 0xfe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr2",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 2.",
                    ),
                    array: None,
                    byte_offset: 0xfe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pidr3",
                    description: Some(
                        "DBGMCU CoreSight peripheral identity register 3.",
                    ),
                    array: None,
                    byte_offset: 0xfec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pidr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr0",
                    description: Some(
                        "DBGMCU CoreSight component identity register 0.",
                    ),
                    array: None,
                    byte_offset: 0xff0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr1",
                    description: Some(
                        "DBGMCU CoreSight component identity register 1.",
                    ),
                    array: None,
                    byte_offset: 0xff4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr2",
                    description: Some(
                        "DBGMCU CoreSight component identity register 2.",
                    ),
                    array: None,
                    byte_offset: 0xff8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cidr3",
                    description: Some(
                        "DBGMCU CoreSight component identity register 3.",
                    ),
                    array: None,
                    byte_offset: 0xffc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cidr3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb1fzr",
            extends: None,
            description: Some(
                "DBGMCU AHB1 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1_stop",
                    description: Some(
                        "GPDMA1 channel 0 stop in debug.",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "gpdma2_stop",
                    description: Some(
                        "GPDMA2 channel 0 stop in debug.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1hfzr",
            extends: None,
            description: Some(
                "DBGMCU APB1H peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lptim2_stop",
                    description: Some(
                        "LPTIM2 stop in debug.",
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
        FieldSet {
            name: "Apb1lfzr",
            extends: None,
            description: Some(
                "DBGMCU APB1L peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2_stop",
                    description: Some(
                        "TIM2 stop in debug.",
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
                    name: "tim3_stop",
                    description: Some(
                        "TIM3 stop in debug.",
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
                    name: "tim4_stop",
                    description: Some(
                        "TIM4 stop in debug.",
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
                    name: "tim5_stop",
                    description: Some(
                        "TIM5 stop in debug.",
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
                    name: "tim6_stop",
                    description: Some(
                        "TIM6 stop in debug.",
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
                    name: "tim7_stop",
                    description: Some(
                        "TIM7 stop in debug.",
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
                    name: "tim12_stop",
                    description: Some(
                        "TIM12 stop in debug.",
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
                    name: "tim13_stop",
                    description: Some(
                        "TIM13 stop in debug.",
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
                    name: "tim14_stop",
                    description: Some(
                        "TIM14 stop in debug.",
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
                    name: "wwdg_stop",
                    description: Some(
                        "WWDG stop in debug.",
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
                    name: "iwdg_stop",
                    description: Some(
                        "IWDG stop in debug.",
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
                    name: "i2c1_stop",
                    description: Some(
                        "I2C1 SMBUS timeout stop in debug.",
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
                    name: "i2c2_stop",
                    description: Some(
                        "I2C2 SMBUS timeout stop in debug.",
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
                    name: "i3c1_stop",
                    description: Some(
                        "I3C1 SCL stall counter stop in debug.",
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
            name: "Apb2fzr",
            extends: None,
            description: Some(
                "DBGMCU APB2 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_stop",
                    description: Some(
                        "TIM1 stop in debug.",
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
                    name: "tim8_stop",
                    description: Some(
                        "TIM8 stop in debug.",
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
                    name: "tim15_stop",
                    description: Some(
                        "TIM15 stop in debug.",
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
                    name: "tim16_stop",
                    description: Some(
                        "TIM16 stop in debug.",
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
                    name: "tim17_stop",
                    description: Some(
                        "TIM17 stop in debug.",
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
            ],
        },
        FieldSet {
            name: "Apb3fzr",
            extends: None,
            description: Some(
                "DBGMCU APB3 peripheral freeze register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c3_stop",
                    description: Some(
                        "I2C3 SMBUS timeout stop in debug.",
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
                    name: "i2c4_stop",
                    description: Some(
                        "I2C4 SMBUS timeout stop in debug.",
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
                    name: "lptim1_stop",
                    description: Some(
                        "LPTIM1 stop in debug.",
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
                    name: "lptim3_stop",
                    description: Some(
                        "LPTIM3 stop in debug.",
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
                    name: "lptim4_stop",
                    description: Some(
                        "LPTIM4 stop in debug.",
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
                    name: "lptim5_stop",
                    description: Some(
                        "LPTIM5 stop in debug.",
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
                    name: "lptim6_stop",
                    description: Some(
                        "LPTIM6 stop in debug.",
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
                    name: "rtc_stop",
                    description: Some(
                        "RTC stop in debug.",
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
            name: "AuthAck",
            extends: None,
            description: Some(
                "DBGMCU debug authentication mailbox acknowledge register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "host_ack",
                    description: Some(
                        "Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message.",
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
                    name: "dev_ack",
                    description: Some(
                        "Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message.",
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
            name: "Cidr0",
            extends: None,
            description: Some(
                "DBGMCU CoreSight component identity register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [7:0].",
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
            name: "Cidr1",
            extends: None,
            description: Some(
                "DBGMCU CoreSight component identity register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [11:8].",
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
                    name: "class",
                    description: Some(
                        "component identification bits [15:12] - component class.",
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
        FieldSet {
            name: "Cidr2",
            extends: None,
            description: Some(
                "DBGMCU CoreSight component identity register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [23:16].",
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
            name: "Cidr3",
            extends: None,
            description: Some(
                "DBGMCU CoreSight component identity register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamble",
                    description: Some(
                        "component identification bits [31:24].",
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
            name: "Cr",
            extends: None,
            description: Some(
                "DBGMCU configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stop",
                    description: Some(
                        "Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.",
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
                    name: "standby",
                    description: Some(
                        "Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed.",
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
                    name: "trace_ioen",
                    description: Some(
                        "trace pin enable.",
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
                    name: "trace_en",
                    description: Some(
                        "trace port and clock enable. This bit enables the trace port clock, TRACECK.",
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
                    name: "trace_mode",
                    description: Some(
                        "trace pin assignment.",
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
                    name: "dcrt",
                    description: Some(
                        "Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials.",
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
            name: "Idcode",
            extends: None,
            description: Some(
                "DBGMCU identity code register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_id",
                    description: Some(
                        "device identification.",
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
                    name: "rev_id",
                    description: Some(
                        "revision.",
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
            name: "Pidr0",
            extends: None,
            description: Some(
                "DBGMCU CoreSight peripheral identity register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "part number bits [7:0].",
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
            name: "Pidr1",
            extends: None,
            description: Some(
                "DBGMCU CoreSight peripheral identity register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "part number bits [11:8].",
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
                    name: "jep106id",
                    description: Some(
                        "JEP106 identity code bits [3:0].",
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
        FieldSet {
            name: "Pidr2",
            extends: None,
            description: Some(
                "DBGMCU CoreSight peripheral identity register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106id",
                    description: Some(
                        "JEP106 identity code bits [6:4].",
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
                    name: "jedec",
                    description: Some(
                        "JEDEC assigned value.",
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
                    name: "revision",
                    description: Some(
                        "component revision number.",
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
        FieldSet {
            name: "Pidr3",
            extends: None,
            description: Some(
                "DBGMCU CoreSight peripheral identity register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmod",
                    description: Some(
                        "customer modified.",
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
                    name: "revand",
                    description: Some(
                        "metal fix version.",
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
        FieldSet {
            name: "Pidr4",
            extends: None,
            description: Some(
                "DBGMCU CoreSight peripheral identity register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106con",
                    description: Some(
                        "JEP106 continuation code.",
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
                    name: "size",
                    description: Some(
                        "register file size.",
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
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "DBGMCU status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap_present",
                    description: Some(
                        "Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present.",
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
                    name: "ap_enabled",
                    description: Some(
                        "Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled.",
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
    enums: &[],
};
