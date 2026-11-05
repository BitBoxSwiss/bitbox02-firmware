
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pssi",
            extends: None,
            description: Some(
                "Parallel synchronous slave interface.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "PSSI control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "sr",
                    description: Some(
                        "PSSI status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "ris",
                    description: Some(
                        "PSSI raw interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ris",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "PSSI interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "mis",
                    description: Some(
                        "PSSI masked interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mis",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "PSSI interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    name: "dr",
                    description: Some(
                        "PSSI data register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "PSSI control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckpol",
                    description: Some(
                        "Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckpol",
                    ),
                },
                Field {
                    name: "depol",
                    description: Some(
                        "Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Depol",
                    ),
                },
                Field {
                    name: "rdypol",
                    description: Some(
                        "Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rdypol",
                    ),
                },
                Field {
                    name: "edm",
                    description: Some(
                        "Extended data mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Edm",
                    ),
                },
                Field {
                    name: "enable",
                    description: Some(
                        "PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.",
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
                    name: "derdycfg",
                    description: Some(
                        "Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Derdycfg",
                    ),
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "DMA enable bit.",
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
                    name: "outen",
                    description: Some(
                        "Data direction selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Outen",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "PSSI data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "byte",
                    description: Some(
                        "Data byte 0.",
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
            name: "Icr",
            extends: None,
            description: Some(
                "PSSI interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovr_isc",
                    description: Some(
                        "Data buffer overrun/underrun interrupt status clear Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "PSSI interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovr_ie",
                    description: Some(
                        "Data buffer overrun/underrun interrupt enable.",
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
            name: "Mis",
            extends: None,
            description: Some(
                "PSSI masked interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovr_mis",
                    description: Some(
                        "Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1.",
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
            name: "Ris",
            extends: None,
            description: Some(
                "PSSI raw interrupt status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovr_ris",
                    description: Some(
                        "Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "PSSI status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtt4b",
                    description: Some(
                        "FIFO is ready to transfer four bytes.",
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
                    name: "rtt1b",
                    description: Some(
                        "FIFO is ready to transfer one byte.",
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
    ],
    enums: &[
        Enum {
            name: "Ckpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "Falling edge active for inputs or rising edge active for outputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "Rising edge active for inputs or falling edge active for outputs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Depol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "PSSI_DE active low (0 indicates that data is valid).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "PSSI_DE active high (1 indicates that data is valid).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Derdycfg",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "PSSI_DE and PSSI_RDY both disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RDY",
                    description: Some(
                        "Only PSSI_RDY enabled.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DE",
                    description: Some(
                        "Only PSSI_DE enabled.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RDY_DE_ALT",
                    description: Some(
                        "Both PSSI_RDY and PSSI_DE alternate functions enabled.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RDY_DE",
                    description: Some(
                        "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RDY_REMAPPED",
                    description: Some(
                        "Only PSSI_RDY function enabled, but mapped to PSSI_DE pin.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DE_REMAPPED",
                    description: Some(
                        "Only PSSI_DE function enabled, but mapped to PSSI_RDY pin.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RDY_DE_BIDI",
                    description: Some(
                        "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Edm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BIT_WIDTH8",
                    description: Some(
                        "Interface captures 8-bit data on every parallel data clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT_WIDTH16",
                    description: Some(
                        "The interface captures 16-bit data on every parallel data clock.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Outen",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RECEIVE_MODE",
                    description: Some(
                        "Data is input synchronously with PSSI_PDCK.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRANSMIT_MODE",
                    description: Some(
                        "Data is output synchronously with PSSI_PDCK.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rdypol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "PSSI_RDY active low (0 indicates that the receiver is ready to receive).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "PSSI_RDY active high (1 indicates that the receiver is ready to receive).",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
