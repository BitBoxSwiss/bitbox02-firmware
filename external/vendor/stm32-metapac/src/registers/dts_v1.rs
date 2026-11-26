
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dts",
            extends: None,
            description: Some(
                "Digital temperature sensor.",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "Temperature sensor configuration register 1.",
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
                    name: "t0valr1",
                    description: Some(
                        "Temperature sensor T0 value register 1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "T0valr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rampvalr",
                    description: Some(
                        "Temperature sensor ramp value register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rampvalr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itr1",
                    description: Some(
                        "Temperature sensor interrupt threshold register 1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Temperature sensor data register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "sr",
                    description: Some(
                        "Temperature sensor status register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "itenr",
                    description: Some(
                        "Temperature sensor interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icifr",
                    description: Some(
                        "Temperature sensor clear interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "Temperature sensor option register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Or",
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
                "Temperature sensor configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.",
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
                    name: "start",
                    description: Some(
                        "Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.",
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
                    name: "intrig_sel",
                    description: Some(
                        "Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input.",
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
                    name: "smp_time",
                    description: Some(
                        "Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE.",
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
                    name: "refclk_sel",
                    description: Some(
                        "Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).",
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
                    name: "q_meas_opt",
                    description: Some(
                        "Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).",
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
                    name: "hsref_clk_div",
                    description: Some(
                        "High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "Temperature sensor data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfreq",
                    description: Some(
                        "Value of the counter output value for temperature sensor 1.",
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
            ],
        },
        FieldSet {
            name: "Icifr",
            extends: None,
            description: Some(
                "Temperature sensor clear interrupt flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "citef",
                    description: Some(
                        "Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register.",
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
                    name: "citlf",
                    description: Some(
                        "Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register.",
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
                    name: "cithf",
                    description: Some(
                        "Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register.",
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
                    name: "caitef",
                    description: Some(
                        "Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register.",
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
                    name: "caitlf",
                    description: Some(
                        "Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register.",
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
                    name: "caithf",
                    description: Some(
                        "Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register.",
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
            name: "Itenr",
            extends: None,
            description: Some(
                "Temperature sensor interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iteen",
                    description: Some(
                        "Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.",
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
                    name: "itlen",
                    description: Some(
                        "Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.",
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
                    name: "ithen",
                    description: Some(
                        "Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.",
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
                    name: "aiteen",
                    description: Some(
                        "Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).",
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
                    name: "aitlen",
                    description: Some(
                        "Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1).",
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
                    name: "aithen",
                    description: Some(
                        "Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’).",
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
            name: "Itr1",
            extends: None,
            description: Some(
                "Temperature sensor interrupt threshold register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "litthd",
                    description: Some(
                        "Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal.",
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
                    name: "hitthd",
                    description: Some(
                        "High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal.",
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
            name: "Or",
            extends: None,
            description: Some(
                "Temperature sensor option register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "op",
                    description: Some(
                        "general purpose option bits.",
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
            name: "Rampvalr",
            extends: None,
            description: Some(
                "Temperature sensor ramp value register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ramp_coeff",
                    description: Some(
                        "Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C.",
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
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Temperature sensor status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itef",
                    description: Some(
                        "Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set.",
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
                    name: "itlf",
                    description: Some(
                        "Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set.",
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
                    name: "ithf",
                    description: Some(
                        "Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set.",
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
                    name: "aitef",
                    description: Some(
                        "Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set.",
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
                    name: "aitlf",
                    description: Some(
                        "Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set.",
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
                    name: "aithf",
                    description: Some(
                        "Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set.",
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
                    name: "rdy",
                    description: Some(
                        "Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing.",
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
            name: "T0valr1",
            extends: None,
            description: Some(
                "Temperature sensor T0 value register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmt0",
                    description: Some(
                        "Engineering value of the frequency measured at T0 for.  temperature sensor 1 This value is expressed in 0.1 kHz.",
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
                    name: "t0",
                    description: Some(
                        "Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
