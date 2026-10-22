
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Exti",
            extends: None,
            description: Some(
                "Extended interrupt and event controller",
            ),
            items: &[
                BlockItem {
                    name: "rtsr",
                    description: Some(
                        "rising trigger selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ftsr",
                    description: Some(
                        "falling trigger selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swier",
                    description: Some(
                        "software interrupt event register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpr",
                    description: Some(
                        "rising edge pending register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpr",
                    description: Some(
                        "falling edge pending register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "privilege configuration register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Priv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "external interrupt selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "imr",
                    description: Some(
                        "CPU wakeup with interrupt mask register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "emr",
                    description: Some(
                        "CPU wakeup with event mask register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lines",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Exti",
            extends: None,
            description: Some(
                "EXTI external interrupt selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI12 GPIO port selection\r These bits are written by software to select the source input for EXTI12 external interrupt.\r When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access.\r When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lines",
            extends: None,
            description: Some(
                "EXTI lines register, 1 bit per line",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "line",
                    description: Some(
                        "EXTI line",
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
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Priv",
            extends: None,
            description: Some(
                "privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv_",
                    description: Some(
                        "Security enable on event input x\r When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access.\r When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded.",
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
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
