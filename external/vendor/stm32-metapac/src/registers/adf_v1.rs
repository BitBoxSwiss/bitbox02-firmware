
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adf",
            extends: None,
            description: Some(
                "ADF.",
            ),
            items: &[
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "ADF Global Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckgcr",
                    description: Some(
                        "ADF clock generator control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckgcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sitfcr",
                    description: Some(
                        "ADF serial interface control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sitfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bsmxcr",
                    description: Some(
                        "ADF bitstream matrix control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bsmxcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltcr",
                    description: Some(
                        "ADF digital filter control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltcicr",
                    description: Some(
                        "ADF digital filer configuration register 0.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltcicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltrsfr",
                    description: Some(
                        "ADF reshape filter configuration register 0.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltrsfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlycr",
                    description: Some(
                        "ADF delay control register 0.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlycr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltier",
                    description: Some(
                        "ADF DFLT0 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltisr",
                    description: Some(
                        "ADF DFLT0 interrupt status register 0.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sadcr",
                    description: Some(
                        "ADF SAD control register.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sadcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sadcfgr",
                    description: Some(
                        "ADF SAD configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sadcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sadsdlvr",
                    description: Some(
                        "ADF SAD sound level register.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sadsdlvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sadanlvr",
                    description: Some(
                        "ADF SAD ambient noise level register.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sadanlvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfltdr",
                    description: Some(
                        "ADF digital filter data register 0.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfltdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bsmxcr",
            extends: None,
            description: Some(
                "ADF bitstream matrix control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bssel",
                    description: Some(
                        "Bitstream selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Bssel",
                    ),
                },
                Field {
                    name: "bsmxactive",
                    description: Some(
                        "BSMX active flag. This bit is set and cleared by hardware. It is used by the application to check if the BSMX is effectively enabled (active) or not. BSSEL[4:0] can only be updated when BSMXACTIVE is set to 0. This BSMXACTIVE flag cannot go to 0 if DFLT0 is enabled.",
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
            name: "Ckgcr",
            extends: None,
            description: Some(
                "ADF clock generator control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckgden",
                    description: Some(
                        "Clock generator dividers enable.",
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
                    name: "cck0en",
                    description: Some(
                        "CCK0 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccken",
                    ),
                },
                Field {
                    name: "cck1en",
                    description: Some(
                        "CCK1 clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccken",
                    ),
                },
                Field {
                    name: "ckgmod",
                    description: Some(
                        "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckgmod",
                    ),
                },
                Field {
                    name: "cck0dir",
                    description: Some(
                        "CCK0 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK0 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cckdir",
                    ),
                },
                Field {
                    name: "cck1dir",
                    description: Some(
                        "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cckdir",
                    ),
                },
                Field {
                    name: "trgsens",
                    description: Some(
                        "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Trgsens",
                    ),
                },
                Field {
                    name: "trgsrc",
                    description: Some(
                        "Digital filter trigger signal selection. This bit is set and cleared by software. It is used to select the trigger signal for the digital filter. This bit is not significant if the CKGMOD = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Trgsrc",
                    ),
                },
                Field {
                    name: "cckdiv",
                    description: Some(
                        "Divider to control the CCK clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Cckdiv",
                    ),
                },
                Field {
                    name: "procdiv",
                    description: Some(
                        "Divider to control the serial interface clock.",
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
                Field {
                    name: "ckgactive",
                    description: Some(
                        "Clock generator active flag.",
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
            name: "Dfltcicr",
            extends: None,
            description: Some(
                "ADF digital filer configuration register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datsrc",
                    description: Some(
                        "Source data for the digital filter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Datsrc",
                    ),
                },
                Field {
                    name: "cicmod",
                    description: Some(
                        "Select the CIC order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cicmod",
                    ),
                },
                Field {
                    name: "mcicd",
                    description: Some(
                        "CIC decimation ratio selection. This bitfield is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scale",
                    description: Some(
                        "Scaling factor selection. This bitfield is set and cleared by software. It is used to select the gain to be applied at CIC output. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this bitfield informs the application on the current gain value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dfltcr",
            extends: None,
            description: Some(
                "ADF digital filter control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dflten",
                    description: Some(
                        "DFLT enable. This bit is set and reset by software. It is used to enable the digital filter.",
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
                    name: "dmaen",
                    description: Some(
                        "DMA requests enable. This bit is set and reset by software. It is used to control the generation of DMA request to transfer the processed samples into the memory.",
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
                    name: "fth",
                    description: Some(
                        "RXFIFO threshold selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rxfifo",
                    ),
                },
                Field {
                    name: "acqmod",
                    description: Some(
                        "DFLT trigger mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Acqmod",
                    ),
                },
                Field {
                    name: "trgsrc",
                    description: Some(
                        "DFLT trigger signal selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbdis",
                    description: Some(
                        "Number of samples to be discarded.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfltrun",
                    description: Some(
                        "DFLT run status flag.",
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
                    name: "dfltactive",
                    description: Some(
                        "DFLT active flag.",
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
            name: "Dfltdr",
            extends: None,
            description: Some(
                "ADF digital filter data register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "DR. Data processed by DFT",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dfltier",
            extends: None,
            description: Some(
                "ADF DFLT interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fthie",
                    description: Some(
                        "RXFIFO threshold interrupt enable.",
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
                    name: "dovrie",
                    description: Some(
                        "Data overflow interrupt enable.",
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
                    name: "satie",
                    description: Some(
                        "Saturation detection interrupt enable.",
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
                    name: "ckabie",
                    description: Some(
                        "Clock absence detection interrupt enable.",
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
                    name: "rfovrie",
                    description: Some(
                        "Reshape filter overrun interrupt enable.",
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
                    name: "sddetie",
                    description: Some(
                        "Sound activity detection interrupt enable.",
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
                    name: "sdlvlie",
                    description: Some(
                        "SAD sound-level value ready enable.",
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
            ],
        },
        FieldSet {
            name: "Dfltisr",
            extends: None,
            description: Some(
                "ADF DFLT interrupt status register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fthf",
                    description: Some(
                        "RXFIFO threshold flag.",
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
                    name: "dovrf",
                    description: Some(
                        "Data overflow flag.",
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
                    name: "rxnef",
                    description: Some(
                        "RXFIFO not empty flag.",
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
                    name: "satf",
                    description: Some(
                        "Saturation detection flag.",
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
                    name: "ckabf",
                    description: Some(
                        "Clock absence detection flag.",
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
                    name: "rfovrf",
                    description: Some(
                        "Reshape filter overrun detection flag.",
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
                    name: "sddetf",
                    description: Some(
                        "Sound activity detection flag.",
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
                    name: "sdlvlf",
                    description: Some(
                        "Sound level value ready flag.",
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
            ],
        },
        FieldSet {
            name: "Dfltrsfr",
            extends: None,
            description: Some(
                "ADF reshape filter configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsfltbyp",
                    description: Some(
                        "Reshaper filter bypass.",
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
                    name: "rsfltd",
                    description: Some(
                        "Reshaper filter decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rsfltd",
                    ),
                },
                Field {
                    name: "hpfbyp",
                    description: Some(
                        "High-pass filter bypass. This bit is set and cleared by software. It is used to bypass the high-pass filter.",
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
                    name: "hpfc",
                    description: Some(
                        "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hpfc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dlycr",
            extends: None,
            description: Some(
                "ADF delay control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "skpdly",
                    description: Some(
                        "Delay to apply to a bitstream. This bitfield is set and cleared by software. It defines the number of input samples that are skipped. Skipping is applied immediately after writing to this bitfield, if SKPBF = 0 and DFLTEN = 1. If SKPBF = 1, the value written into the register is ignored by the delay state machine.",
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
                    name: "skpbf",
                    description: Some(
                        "Skip busy flag.",
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
            name: "Gcr",
            extends: None,
            description: Some(
                "ADF Global Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgo",
                    description: Some(
                        "Trigger output control Set by software and reset by.",
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
            ],
        },
        FieldSet {
            name: "Sadanlvr",
            extends: None,
            description: Some(
                "ADF SAD ambient noise level register. This bitfield is set by hardware. It contains the latest ambient noise level computed by the SAD. To refresh this bitfield, the SDLVLF flag must be cleared.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "anlvl",
                    description: Some(
                        "ANLVL.",
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
            ],
        },
        FieldSet {
            name: "Sadcfgr",
            extends: None,
            description: Some(
                "ADF SAD configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snthr",
                    description: Some(
                        "SNTHR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Snthr",
                    ),
                },
                Field {
                    name: "anslp",
                    description: Some(
                        "ANSLP.",
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
                    name: "lfrnb",
                    description: Some(
                        "LFRNB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lfrnb",
                    ),
                },
                Field {
                    name: "hgovr",
                    description: Some(
                        "Hangover time window.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hgovr",
                    ),
                },
                Field {
                    name: "anmin",
                    description: Some(
                        "ANMIN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sadcr",
            extends: None,
            description: Some(
                "ADF Sound activity detector (SAD) control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saden",
                    description: Some(
                        "Sound activity detector enable.",
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
                    name: "datcap",
                    description: Some(
                        "Data capture mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Datcap",
                    ),
                },
                Field {
                    name: "detcfg",
                    description: Some(
                        "Sound trigger event configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Detcfg",
                    ),
                },
                Field {
                    name: "sadst",
                    description: Some(
                        "SAD state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sadst",
                    ),
                },
                Field {
                    name: "hysten",
                    description: Some(
                        "Hysteresis enable.",
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
                    name: "frsize",
                    description: Some(
                        "Frame size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Frsize",
                    ),
                },
                Field {
                    name: "sadmod",
                    description: Some(
                        "Sound activity detector working mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sadmod",
                    ),
                },
                Field {
                    name: "sadactive",
                    description: Some(
                        "SAD Active flag.",
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
            name: "Sadsdlvr",
            extends: None,
            description: Some(
                "ADF SAD sound level register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdlvl",
                    description: Some(
                        "Short term sound level. This bitfield is set by hardware. It contains the latest sound level computed by the SAD. To refresh this value, SDLVLF must be cleared.",
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
            ],
        },
        FieldSet {
            name: "Sitfcr",
            extends: None,
            description: Some(
                "ADF serial interface control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sitfen",
                    description: None,
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
                    name: "scksrc",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Scksrc",
                    ),
                },
                Field {
                    name: "sitfmod",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sitfmod",
                    ),
                },
                Field {
                    name: "sth",
                    description: Some(
                        "Manchester symbol threshold/SPI threshold. This bitfield is set and cleared by software. It is used for Manchester mode to define the expected symbol threshold levels (seer to Manchester mode for details on computation). In addition this bitfield is used to define the timeout value for the clock absence detection in Normal SPI mode. STH[4:0] values lower than four are invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sitfactive",
                    description: Some(
                        "SITFACTIVE.",
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
    ],
    enums: &[
        Enum {
            name: "Acqmod",
            description: Some(
                "DFLT trigger mode. This bitfield is set and cleared by software. It is used to select the trigger mode of the DFLT0.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ASYNCHRONOUS_CONTINUOUS",
                    description: Some(
                        "Asynchronous continuous acquisition mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ASYNCHRONOUS_SINGLE_SHOT",
                    description: Some(
                        "Asynchronous single-shot acquisition mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SYNCRONOUS_CONTINUOUS",
                    description: Some(
                        "Synchronous continuous acquisition mode.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SYNCRONOUS_SINGLE_SHOT",
                    description: Some(
                        "Synchronous single-shot acquisition mode.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "WINDOW_CONTINUOUS",
                    description: Some(
                        "Window continuous acquisition mode.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Bssel",
            description: Some(
                "Bitstream selection. This bitfield is set and cleared by software. It is used to select the bitstream to be used by the DFLT0.",
            ),
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "BS0_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BS0_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BS1_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BS1_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BS2_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BS2_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BS3_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BS3_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "BS4_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "BS4_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "BS5_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "BS5_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "BS6_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "BS6_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "BS7_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "BS7_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "BS8_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "BS8_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "BS9_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "BS9_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "BS10_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "BS10_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "BS11_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "BS11_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "BS12_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "BS12_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "BS13_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "BS13_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "BS14_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "BS14_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "BS15_R",
                    description: Some(
                        "bsx_r provided to DFLTy (and SCDy).",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "BS15_F",
                    description: Some(
                        "bsx_f provided to DFLTy (and SCDy).",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Cckdir",
            description: Some(
                "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some(
                        "CCK is an input.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "CCK is an output.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cckdiv",
            description: Some(
                "Divider to control the CCK clock. This bit is set and reset by software. It is used to control the frequency of the bitstream clock on the CCK pin.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 3.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 4.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 5.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 6.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 7.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 8.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 9.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 10.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 11.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 12.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 13.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 14.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 15.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "The ADF_CCK clock is adf_proc_ck divided by 16.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Ccken",
            description: Some(
                "CCK clock enable. This bit is set and reset by software. It is used to control the generation of the bitstream clock on the CCK pin.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_GENERATED",
                    description: Some(
                        "Bitstream clock not generated.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GENERATED",
                    description: Some(
                        "Bitstream clock generated on the CCK pin.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cicmod",
            description: Some(
                "Select the CIC order. This bitfield is set and cleared by software. It is used to select the MCIC order.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SINC4",
                    description: Some(
                        "MCIC configured in single Sinc4 filter.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SINC5",
                    description: Some(
                        "MCIC configured in single Sinc5 filter.",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ckgmod",
            description: Some(
                "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1).",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IMMEDIATE",
                    description: Some(
                        "The kernel clock is provided to the dividers as soon as CKGDEN is set to 1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRIGGER",
                    description: Some(
                        "The kernel clock is provided to the dividers when CKGDEN is set to 1 and the trigger condition met.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Datcap",
            description: Some(
                "Data capture mode. This bitfield is set and cleared by software. It is used to define in which conditions, the samples provided by DLFT0 are stored into the memory.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Samples from DFLT0 not transfered into the memory.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_DETECTED",
                    description: Some(
                        "Samples from DFLT0 transfered into the memory when SAD is in DETECT state.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Datsrc",
            description: Some(
                "Source data for the digital filter.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BSMX",
                    description: Some(
                        "Stream coming from the BSMX selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ADCITF1",
                    description: Some(
                        "Stream coming from the ADCITF1 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ADCITF2",
                    description: Some(
                        "Stream coming from the ADCITF2 selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Detcfg",
            description: Some(
                "Sound trigger event configuration. This bit is set and cleared by software. It is used to define if the sddet_evt event is generated only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT state.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MONITOR",
                    description: Some(
                        "sddet_evt generated when SAD enters the MONITOR state.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DETECT",
                    description: Some(
                        "sddet_evt generated when SAD enters or exits the DETECT state.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Frsize",
            description: Some(
                "Frame size. This bitfield is set and cleared by software. it is used to define the size of one frame and also to define how many samples are taken into account to compute the short-term signal level.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SAMPLES8",
                    description: Some(
                        "8 sample.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SAMPLES16",
                    description: Some(
                        "16 samples.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SAMPLES32",
                    description: Some(
                        "32 samples.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SAMPLES64",
                    description: Some(
                        "64 samples.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SAMPLES128",
                    description: Some(
                        "128 samples.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SAMPLES256",
                    description: Some(
                        "256 samples.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SAMPLES512",
                    description: Some(
                        "512 samples.",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Hgovr",
            description: Some(
                "Hangover time window. This bitfield is set and cleared by software. It is used to select the hangover time window.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "FRAMES_4",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FRAMES_8",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FRAMES_16",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FRAMES_32",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FRAMES_64",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FRAMES_128",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FRAMES_256",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FRAMES_512",
                    description: Some(
                        "SAD back to MONITOR state if sound is below threshold for 4 frames.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Hpfc",
            description: Some(
                "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Cut-off frequency = 0.000625 x FPCM.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some(
                        "Cut-off frequency = 0.00125 x FPCM.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Cut-off frequency = 0.00250 x FPCM",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MAXIMUM",
                    description: Some(
                        "Cut-off frequency = 0.00950 x FPCM",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lfrnb",
            description: Some(
                "LFRNB. This bitfield is set and cleared by software. It is used to define the number of learning frames to perform the first estimate of the noise level.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "FRAMES_2",
                    description: Some(
                        "2 samples.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FRAMES_4",
                    description: Some(
                        "4 samples.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FRAMES_8",
                    description: Some(
                        "8 samples.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FRAMES_16",
                    description: Some(
                        "16 samples.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FRAMES_32",
                    description: Some(
                        "32 samples.",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Rsfltd",
            description: Some(
                "Reshaper filter decimation ratio. This bitfield is set and cleared by software. It is used to select the decimation ratio of the reshaper filter.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DECIMATION4",
                    description: Some(
                        "Decimation ratio is 4 (default value).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DECIMATION1",
                    description: Some(
                        "Decimation ratio is 1.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rxfifo",
            description: Some(
                "RXFIFO threshold selection. This bitfield is set and cleared by software. It is used to select the RXFIFO threshold.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOT_EMPTY",
                    description: Some(
                        "RXFIFO threshold event generated when the RXFIFO is not empty",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF_FULL",
                    description: Some(
                        "RXFIFO threshold event generated when the RXFIFO is half-full",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sadmod",
            description: Some(
                "SAD working mode. This bitfield is set and cleared by software. It is used to define the way the SAD works",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "THRESHOLD_ESTIMATED_AMBIENT_NOISE",
                    description: Some(
                        "Threshold value computed according to the estimated ambient noise. The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a voice activity detector.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "THRESHOLD_MINIMUM_NOISELEVEL",
                    description: Some(
                        "Threshold value equal to ANMIN[12:0], multiplied by the gain selected by SNTHR[3:0] The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this mode, the SAD works like a sound detector.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "THRESHOLD_MINIMUM_NOISELEVELX4",
                    description: Some(
                        "Threshold value given by 4 x ANMIN[12:0]. The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected by SNTHR[3:0] is bigger than the defined threshold. In this mode, the SAD is working like an ambient noise estimator. Hysteresis function cannot be used in this mode.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sadst",
            description: Some(
                "SAD state. This bitfield is set and cleared by hardware. It indicates the SAD state and is meaningful only when SADEN = 1.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LEARN",
                    description: Some(
                        "SAD in LEARN state.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MONITOR",
                    description: Some(
                        "SAD in MONITOR state.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DETECT",
                    description: Some(
                        "SAD in DETECT state.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Scksrc",
            description: Some(
                "Serial clock source. This bitfield is set and cleared by software. It is used to select the clock source of the serial interface.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CCK0",
                    description: Some(
                        "Serial clock source is CCK0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CCK1",
                    description: Some(
                        "Serial clock source is CCK1.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CKI0",
                    description: Some(
                        "Serial clock source is CCI0.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CKI1",
                    description: Some(
                        "Serial clock source is CCI1.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sitfmod",
            description: Some(
                "Serial interface mode. This bitfield is set and cleared by software. It is used to select the serial interface mode.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MASTER_SPI",
                    description: Some(
                        "LF_MASTER SPI mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL_SPI",
                    description: Some(
                        "Normal SPI mode.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MANCHESTER_FALLING",
                    description: Some(
                        "Manchester mode rising edge = logic 0, falling edge = logic 1.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MANCHESTER_RISING",
                    description: Some(
                        "Manchester mode rising edge = logic 1, falling edge = logic 0.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Snthr",
            description: Some(
                "SNTHR. This bitfield is set and cleared by software. It is used to select the gain to be applied at CIC output. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this bitfield informs the application on the current gain value.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOISE_PLUS_3_5",
                    description: Some(
                        "Threshold is 3.5 dB higher than ANLVL",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOISE_PLUS_6_0",
                    description: Some(
                        "Threshold is 6.0 dB higher than ANLVL",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NOISE_PLUS_9_5",
                    description: Some(
                        "Threshold is 9.5 dB higher than ANLVL",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NOISE_PLUS_12",
                    description: Some(
                        "Threshold is 12 dB higher than ANLVL",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "NOISE_PLUS_15_6",
                    description: Some(
                        "Threshold is 15.6 dB higher than ANLVL",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "NOISE_PLUS_18",
                    description: Some(
                        "Threshold is 18 dB higher than ANLVL",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "NOISE_PLUS_21_6",
                    description: Some(
                        "Threshold is 21.6 dB higher than ANLVL",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "NOISE_PLUS_24_1",
                    description: Some(
                        "Threshold is 24.1 dB higher than ANLVL",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "NOISE_PLUS_27_6",
                    description: Some(
                        "Threshold is 27.6 dB higher than ANLVL",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "NOISE_PLUS_30_1",
                    description: Some(
                        "Threshold is 30.1 dB higher than ANLVL",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Trgsens",
            description: Some(
                "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "A rising edge event triggers the activation of CKGEN dividers.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "A falling edge even triggers the activation of CKGEN dividers.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Trgsrc",
            description: Some(
                "Digital filter trigger signal selection.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "TRGO",
                    description: Some(
                        "TRGO Selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRG1",
                    description: Some(
                        "adf_trg1 selected.",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
