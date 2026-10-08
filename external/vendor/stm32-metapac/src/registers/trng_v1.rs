
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Trng",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "CR register.",
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
                        "SR register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "val",
                    description: Some(
                        "VAL register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Val",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oscs_cr",
                    description: Some(
                        "OSCS_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OscsCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "postp_cr",
                    description: Some(
                        "POSTP_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PostpCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "postp_sr",
                    description: Some(
                        "POSTP_SR register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PostpSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "defkey0",
                    description: Some(
                        "DEFKEY0 register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Defkey0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "defkey1",
                    description: Some(
                        "DEFKEY1 register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Defkey1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "defkey2",
                    description: Some(
                        "DEFKEY2 register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Defkey2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "defkey3",
                    description: Some(
                        "DEFKEY3 register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Defkey3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_cr",
                    description: Some(
                        "HEALTH_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc1_cr",
                    description: Some(
                        "HEALTH_OSC1_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc1Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc2_cr",
                    description: Some(
                        "HEALTH_OSC2_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc2Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc3_cr",
                    description: Some(
                        "HEALTH_OSC3_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc3Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc1_sr",
                    description: Some(
                        "HEALTH_OSC1_SR register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc1Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc2_sr",
                    description: Some(
                        "HEALTH_OSC2_SR register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc2Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "health_osc3_sr",
                    description: Some(
                        "HEALTH_OSC3_SR register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HealthOsc3Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_cr",
                    description: Some(
                        "IRQ_CR register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sr",
                    description: Some(
                        "IRQ_SR register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqSr",
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
                "CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "disable",
                    description: Some(
                        "Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written.",
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
                    name: "clr_revclk_flag",
                    description: Some(
                        "Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClrRevclkFlag",
                    ),
                },
                Field {
                    name: "rst_health_flags",
                    description: Some(
                        "Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RstHealthFlags",
                    ),
                },
                Field {
                    name: "clkdiv",
                    description: Some(
                        "Sampling Clock Enable Divider. CLKDIV[15:0] control the sampling clock enable divider, dividing by a factor equal to CLKDIV[15:0] + 1, values being in the range of 1 to 65536.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Defkey0",
            extends: None,
            description: Some(
                "DEFKEY0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "defkey0",
                    description: Some(
                        "Bits 31 to 0 of AES 128-bit Default Key.",
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
            name: "Defkey1",
            extends: None,
            description: Some(
                "DEFKEY1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "defkey1",
                    description: Some(
                        "Bits 63 to 31 of AES 128-bit Default Key.",
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
            name: "Defkey2",
            extends: None,
            description: Some(
                "DEFKEY2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "defkey2",
                    description: Some(
                        "Bits 95 to 64 of AES 128-bit Default Key.",
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
            name: "Defkey3",
            extends: None,
            description: Some(
                "DEFKEY3 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "defkey3",
                    description: Some(
                        "Bits 127 to 96 of AES 128-bit Default Key.",
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
            name: "HealthCr",
            extends: None,
            description: Some(
                "HEALTH_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "repet_cutoff",
                    description: Some(
                        "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
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
                    name: "adap_cutoff",
                    description: Some(
                        "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iter_adap",
                    description: Some(
                        "Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration.",
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
            ],
        },
        FieldSet {
            name: "HealthOsc1Cr",
            extends: None,
            description: Some(
                "HEALTH_OSC1_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "repet_cutoff_osc1",
                    description: Some(
                        "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
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
                    name: "adap_cutoff_osc1",
                    description: Some(
                        "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HealthOsc1Sr",
            extends: None,
            description: Some(
                "HEALTH_OSC1_SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to1_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of first triple-oscillator cell.",
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
                    name: "to1_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of first triple-oscillator cell.",
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
                    name: "to2_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to2_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to3_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of third triple-oscillator cell.",
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
                    name: "to3_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of third triple-oscillator cell.",
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
            name: "HealthOsc2Cr",
            extends: None,
            description: Some(
                "HEALTH_OSC2_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "repet_cutoff_osc2",
                    description: Some(
                        "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
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
                    name: "adap_cutoff_osc2",
                    description: Some(
                        "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HealthOsc2Sr",
            extends: None,
            description: Some(
                "HEALTH_OSC2_SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to1_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of first triple-oscillator cell.",
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
                    name: "to1_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of first triple-oscillator cell.",
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
                    name: "to2_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to2_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to3_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of third triple-oscillator cell.",
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
                    name: "to3_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of third triple-oscillator cell.",
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
            name: "HealthOsc3Cr",
            extends: None,
            description: Some(
                "HEALTH_OSC3_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "repet_cutoff_osc3",
                    description: Some(
                        "Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
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
                    name: "adap_cutoff_osc3",
                    description: Some(
                        "Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HealthOsc3Sr",
            extends: None,
            description: Some(
                "HEALTH_OSC3_SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to1_repet_error",
                    description: Some(
                        "Repetition error flag of third oscillator of first triple-oscillator cell.",
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
                    name: "to1_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of first triple-oscillator cell.",
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
                    name: "to2_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to2_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of second triple-oscillator cell.",
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
                    name: "to3_repet_error",
                    description: Some(
                        "Repetition error flag of first oscillator of third triple-oscillator cell.",
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
                    name: "to3_adapt_error",
                    description: Some(
                        "Adaptive error flag of first oscillator of third triple-oscillator cell.",
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
            name: "IrqCr",
            extends: None,
            description: Some(
                "IRQ_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_ff_full_irq",
                    description: Some(
                        "Enable the interrupt when the output fifo is full of new random.",
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
                    name: "en_error_irq",
                    description: Some(
                        "Enable the interrupt when an error is reported by the health tests.",
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
            ],
        },
        FieldSet {
            name: "IrqSr",
            extends: None,
            description: Some(
                "IRQ_SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ff_full_irq",
                    description: Some(
                        "Set to 1 when the output fifo is full of new random. Flag is cleared by writing a 1.",
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
                    name: "error_irq",
                    description: Some(
                        "Set to 1 when an error is reported by the health tests. Flag is cleared by writing a 1.",
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
            ],
        },
        FieldSet {
            name: "OscsCr",
            extends: None,
            description: Some(
                "OSCS_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrd1",
                    description: Some(
                        "Power down of individual oscillators in triple-oscillator block number 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrd2",
                    description: Some(
                        "Power down of individual oscillators in triple-oscillator block number 2.",
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
                    name: "pwrd3",
                    description: Some(
                        "Power down of individual oscillators in triple-oscillator block number 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sync_oscs",
                    description: Some(
                        "When set, selection of resynchronized output of oscillators.",
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
            name: "PostpCr",
            extends: None,
            description: Some(
                "POSTP_CR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aes_reset",
                    description: Some(
                        "Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AesReset",
                    ),
                },
                Field {
                    name: "nb_loop_aes",
                    description: Some(
                        "NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed.",
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
                    name: "nb_rnd_reinit",
                    description: Some(
                        "Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized.",
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
            name: "PostpSr",
            extends: None,
            description: Some(
                "POSTP_SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aes_init",
                    description: Some(
                        "AES Post processing has been fully initialized (key and state) and is ready for generating 128-bit random words.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AesInit",
                    ),
                },
                Field {
                    name: "aes_key_ld",
                    description: Some(
                        "AES random key has been generated and loaded in AES key register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AesKeyLd",
                    ),
                },
                Field {
                    name: "aes_busy",
                    description: Some(
                        "AES core is busy, generating a random value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AesBusy",
                    ),
                },
                Field {
                    name: "aes_health_done",
                    description: Some(
                        "AES-CMAC health test is completed.",
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
                    name: "aes_k12_error",
                    description: Some(
                        "Health test error on AES-CMAC sub-keys generation.",
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
                    name: "aes_dout_error",
                    description: Some(
                        "Health test error on AES-CMAC output generation.",
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
                "SR register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "disabled",
                    description: Some(
                        "TRNG is disabled.",
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
                    name: "all_oscs_down",
                    description: Some(
                        "All oscillators of the random source noise have been powered down. This can cause the rising of OEC3 flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AllOscsDown",
                    ),
                },
                Field {
                    name: "reveal_clk_err",
                    description: Some(
                        "The internal clock for the RNG core is not revealed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RevealClkErr",
                    ),
                },
                Field {
                    name: "entropy_err",
                    description: Some(
                        "The error refers to a fault in the bit sequence detected by the Entropy Monitor. Failed test is given by REPET_ERROR, and ADAPT_ERROR, OSCS_REPET_ERROR and OSCS_ADAPT_ERROR status flags.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "EntropyErr",
                    ),
                },
                Field {
                    name: "val_ready",
                    description: Some(
                        "TRNG Value ready At least one 32-bit random value is available in the data FIFO. Note that application must ensure that a random is available in internal FIFO before starting a read otherwise a bus error will be generated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ValReady",
                    ),
                },
                Field {
                    name: "fifo_full",
                    description: Some(
                        "Indicates whether random data FIFO is full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FifoFull",
                    ),
                },
                Field {
                    name: "src_health_done",
                    description: Some(
                        "First run of noise source health test is completed.",
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
                    name: "repet_error",
                    description: Some(
                        "Noise source Repetition health test error.",
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
                    name: "adapt_error",
                    description: Some(
                        "Noise source Adaptive 1024 health test error.",
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
                    name: "oscs_health_done",
                    description: Some(
                        "First run of source health tests of individual oscillators composing the noise source are completed.Reserved.",
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
                    name: "oscs_repet_error",
                    description: Some(
                        "Logical OR of repetition health test errors of individual oscillators composing the noise source.",
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
                    name: "oscs_adapt_error",
                    description: Some(
                        "Logical OR of adaptive health test errors of individual oscillators composing the noise source.",
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
            name: "Val",
            extends: None,
            description: Some(
                "VAL register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rnd_val",
                    description: Some(
                        "RND_VAL is a 32-bit Random Value. This is the output of the internal four-word FIFO. Note that application must ensure that a random is available in FIFO by ready VAL_READY flag before starting a read otherwise a null value will be returned.",
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
    ],
    enums: &[
        Enum {
            name: "AesBusy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "AES core is idle.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "AES core is busy.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AesInit",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "AES core is not initialized (no key or state set).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "AES core is fully initialized.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AesKeyLd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "AES core is waiting for 128 random bits from the entropy sources for generating its key.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "AES key register has been loaded with a random key.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AesReset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No effect.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Reset AES core.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AllOscsDown",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "At least one oscillator is ON.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "All oscillators are down.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ClrRevclkFlag",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no reset.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "reset revclk flag.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EntropyErr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No fault detected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Embedded heath monitor detects an error in bit stream quality.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FifoFull",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "FIFO is not full.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "The internal data FIFO is full and four 32-bit random values can be read.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RevealClkErr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Internal clock for RNG clock is present.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Internal RNG clock is not present.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RstHealthFlags",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no reset.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "reset health flag.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ValReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No value is ready in FIFO.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "A 32-bit value is available in the internal FIFO.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
