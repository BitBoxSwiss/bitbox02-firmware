
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpio",
            extends: None,
            description: Some(
                "General-purpose I/Os",
            ),
            items: &[
                BlockItem {
                    name: "moder",
                    description: Some(
                        "GPIO port mode register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Moder",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otyper",
                    description: Some(
                        "GPIO port output type register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otyper",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ospeedr",
                    description: Some(
                        "GPIO port output speed register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ospeedr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pupdr",
                    description: Some(
                        "GPIO port pull-up/pull-down register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pupdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idr",
                    description: Some(
                        "GPIO port input data register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odr",
                    description: Some(
                        "GPIO port output data register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bsrr",
                    description: Some(
                        "GPIO port bit set/reset register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Bsrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lckr",
                    description: Some(
                        "GPIO port configuration lock register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afr",
                    description: Some(
                        "GPIO alternate function registers. The register described in the datasheet as AFRL is index 0 in this array, and AFRH is index 1. Note that when operating on AFRH, you need to subtract 8 from any operations on the field array it contains -- the alternate function for pin 9 is at index 1, for instance.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascr",
                    description: Some(
                        "GPIO port analog switch control register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Afr",
            extends: None,
            description: Some(
                "GPIO alternate function register. This contains an array of 8 fields, which correspond to pins 0-7 of the port (for AFRL) or pins 8-15 of the port (for AFRH).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afr",
                    description: Some(
                        "Alternate function selection for one of the pins controlled by this register (0-7).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascr",
            extends: None,
            description: Some(
                "GPIO port analog switch control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "asc",
                    description: Some(
                        "Port x analog switch control I/O pin y (y= 0..15)",
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
            ],
        },
        FieldSet {
            name: "Bsrr",
            extends: None,
            description: Some(
                "GPIO port bit set/reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bs",
                    description: Some(
                        "Port x set bit y (y= 0..15)",
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
                    name: "br",
                    description: Some(
                        "Port x set bit y (y= 0..15)",
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
            name: "Idr",
            extends: None,
            description: Some(
                "GPIO port input data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idr",
                    description: Some(
                        "Port input data (y = 0..15)",
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
                    enumm: Some(
                        "Idr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Lckr",
            extends: None,
            description: Some(
                "GPIO port configuration lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lck",
                    description: Some(
                        "Port configuration locked",
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
                    name: "lckk",
                    description: Some(
                        "Port configuration lock key active",
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
            name: "Moder",
            extends: None,
            description: Some(
                "GPIO port mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "moder",
                    description: Some(
                        "Port x configuration bits (y = 0..15)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Moder",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Odr",
            extends: None,
            description: Some(
                "GPIO port output data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "odr",
                    description: Some(
                        "Port output data (y = 0..15)",
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
                    enumm: Some(
                        "Odr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ospeedr",
            extends: None,
            description: Some(
                "GPIO port output speed register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ospeedr",
                    description: Some(
                        "Port x configuration bits (y = 0..15)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ospeedr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Otyper",
            extends: None,
            description: Some(
                "GPIO port output type register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ot",
                    description: Some(
                        "Port x configuration bits (y = 0..15)",
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
                    enumm: Some(
                        "Ot",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pupdr",
            extends: None,
            description: Some(
                "GPIO port pull-up/pull-down register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pupdr",
                    description: Some(
                        "Port x configuration bits (y = 0..15)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pupdr",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Idr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Input is logic low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Input is logic high",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Moder",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some(
                        "Input mode (reset state)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "General purpose output mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALTERNATE",
                    description: Some(
                        "Alternate function mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "Analog mode",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Odr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Set output to logic low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Set output to logic high",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ospeedr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW_SPEED",
                    description: Some(
                        "Low speed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM_SPEED",
                    description: Some(
                        "Medium speed",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH_SPEED",
                    description: Some(
                        "High speed",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VERY_HIGH_SPEED",
                    description: Some(
                        "Very high speed",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ot",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PUSH_PULL",
                    description: Some(
                        "Output push-pull (reset state)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPEN_DRAIN",
                    description: Some(
                        "Output open-drain",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pupdr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FLOATING",
                    description: Some(
                        "No pull-up, pull-down",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PULL_UP",
                    description: Some(
                        "Pull-up",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PULL_DOWN",
                    description: Some(
                        "Pull-down",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
