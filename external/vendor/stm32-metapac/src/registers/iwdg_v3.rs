
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Iwdg",
            extends: None,
            description: Some(
                "Independent watchdog",
            ),
            items: &[
                BlockItem {
                    name: "kr",
                    description: Some(
                        "Key register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Kr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pr",
                    description: Some(
                        "Prescaler register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rlr",
                    description: Some(
                        "Reload register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rlr",
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
                    byte_offset: 0xc,
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
                    name: "winr",
                    description: Some(
                        "Window register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Winr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ewcr",
                    description: Some(
                        "IWDG early wakeup interrupt register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ewcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ewcr",
            extends: None,
            description: Some(
                "IWDG early wakeup interrupt register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewit",
                    description: Some(
                        "Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT[11:0] - 1. EWIT[11:0] must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset.",
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
                    name: "ewic",
                    description: Some(
                        "Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.",
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
                    name: "ewie",
                    description: Some(
                        "Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.",
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
            name: "Kr",
            extends: None,
            description: Some(
                "Key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Key value (write only, read 0000h)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: Some(
                        "Key",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pr",
            extends: None,
            description: Some(
                "Prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pr",
                    description: Some(
                        "Prescaler divider",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rlr",
            extends: None,
            description: Some(
                "Reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rl",
                    description: Some(
                        "Watchdog counter reload value",
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
                    name: "pvu",
                    description: Some(
                        "Watchdog prescaler value update",
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
                    name: "rvu",
                    description: Some(
                        "Watchdog counter reload value update",
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
                    name: "wvu",
                    description: Some(
                        "Watchdog counter window value update",
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
                    name: "ewu",
                    description: Some(
                        "Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT[11:0]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT[11:0] and EWIE fields can be updated only when EWU bit is reset.",
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
                    name: "ewif",
                    description: Some(
                        "Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’.",
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
            ],
        },
        FieldSet {
            name: "Winr",
            extends: None,
            description: Some(
                "Window register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "win",
                    description: Some(
                        "Watchdog counter window value",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Key",
            description: None,
            bit_size: 16,
            variants: &[
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "Enable access to PR, RLR and WINR registers (0x5555)",
                    ),
                    value: 21845,
                },
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the watchdog value (0xAAAA)",
                    ),
                    value: 43690,
                },
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Start the watchdog (0xCCCC)",
                    ),
                    value: 52428,
                },
            ],
        },
        Enum {
            name: "Pr",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIVIDE_BY4",
                    description: Some(
                        "Divider /4",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIVIDE_BY8",
                    description: Some(
                        "Divider /8",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIVIDE_BY16",
                    description: Some(
                        "Divider /16",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIVIDE_BY32",
                    description: Some(
                        "Divider /32",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIVIDE_BY64",
                    description: Some(
                        "Divider /64",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIVIDE_BY128",
                    description: Some(
                        "Divider /128",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIVIDE_BY256",
                    description: Some(
                        "Divider /256",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIVIDE_BY512",
                    description: Some(
                        "Divider /512",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIVIDE_BY1024",
                    description: Some(
                        "Divider /1024",
                    ),
                    value: 8,
                },
            ],
        },
    ],
};
