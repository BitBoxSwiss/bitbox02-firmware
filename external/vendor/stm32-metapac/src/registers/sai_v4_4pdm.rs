
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ch",
            extends: None,
            description: Some(
                "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frcr",
                    description: Some(
                        "This register has no meaning in AC97 and SPDIF audio protocol",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Frcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slotr",
                    description: Some(
                        "This register has no meaning in AC97 and SPDIF audio protocol",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slotr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "im",
                    description: Some(
                        "Interrupt mask register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Im",
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
                    byte_offset: 0x14,
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
                    name: "clrfr",
                    description: Some(
                        "Clear flag register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Clrfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Data register",
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
            ],
        },
        Block {
            name: "Sai",
            extends: None,
            description: Some(
                "Serial audio interface",
            ),
            items: &[
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "Global configuration register",
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
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR",
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
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ch",
                        },
                    ),
                },
                BlockItem {
                    name: "pdmcr",
                    description: Some(
                        "PDM control register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdmdly",
                    description: Some(
                        "PDM delay register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdmdly",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Clrfr",
            extends: None,
            description: Some(
                "Clear flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "covrudr",
                    description: Some(
                        "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.",
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
                    name: "cmutedet",
                    description: Some(
                        "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.",
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
                    name: "cwckcfg",
                    description: Some(
                        "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE[1] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.",
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
                    name: "ccnrdy",
                    description: Some(
                        "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.",
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
                    name: "cafsdet",
                    description: Some(
                        "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0.",
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
                    name: "clfsdet",
                    description: Some(
                        "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0.",
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
            name: "Cr1",
            extends: None,
            description: Some(
                "Configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "SAIx audio block mode immediately",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "prtcfg",
                    description: Some(
                        "Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Prtcfg",
                    ),
                },
                Field {
                    name: "ds",
                    description: Some(
                        "Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG[1:0]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP[1:0] bits, DS[1:0] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ds",
                    ),
                },
                Field {
                    name: "lsbfirst",
                    description: Some(
                        "Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsbfirst",
                    ),
                },
                Field {
                    name: "ckstr",
                    description: Some(
                        "Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckstr",
                    ),
                },
                Field {
                    name: "syncen",
                    description: Some(
                        "Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Syncen",
                    ),
                },
                Field {
                    name: "mono",
                    description: Some(
                        "Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mono",
                    ),
                },
                Field {
                    name: "outdriv",
                    description: Some(
                        "Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Outdriv",
                    ),
                },
                Field {
                    name: "saien",
                    description: Some(
                        "Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit.",
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
                    name: "dmaen",
                    description: Some(
                        "DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE[1:0] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.",
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
                    name: "nodiv",
                    description: Some(
                        "No fixed divider between MCLK and FS",
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
                    name: "mckdiv",
                    description: Some(
                        "Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Mckdiv",
                    ),
                },
                Field {
                    name: "osr",
                    description: Some(
                        "Oversampling ratio for master clock",
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
                    name: "mcken",
                    description: Some(
                        "Master clock generation enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fth",
                    description: Some(
                        "FIFO threshold. This bit is set and cleared by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Fth",
                    ),
                },
                Field {
                    name: "fflush",
                    description: Some(
                        "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled.",
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
                    name: "tris",
                    description: Some(
                        "Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details.",
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
                    name: "mute",
                    description: Some(
                        "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.",
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
                    name: "muteval",
                    description: Some(
                        "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Muteval",
                    ),
                },
                Field {
                    name: "mutecnt",
                    description: Some(
                        "Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpl",
                    description: Some(
                        "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpl",
                    ),
                },
                Field {
                    name: "comp",
                    description: Some(
                        "Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE[0]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Comp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "Data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.",
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
            name: "Frcr",
            extends: None,
            description: Some(
                "This register has no meaning in AC97 and SPDIF audio protocol",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frl",
                    description: Some(
                        "Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL[7:0] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT[4:0] of SAI_xSLOTR register (NBSLOT[3:0] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration.",
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
                    name: "fsall",
                    description: Some(
                        "Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL[6:0] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fsdef",
                    description: Some(
                        "Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.",
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
                    name: "fspol",
                    description: Some(
                        "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fspol",
                    ),
                },
                Field {
                    name: "fsoff",
                    description: Some(
                        "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fsoff",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some(
                "Global configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncin",
                    description: Some(
                        "Synchronization inputs",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncout",
                    description: Some(
                        "Synchronization outputs These bits are set and cleared by software.",
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
            ],
        },
        FieldSet {
            name: "Im",
            extends: None,
            description: Some(
                "Interrupt mask register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovrudrie",
                    description: Some(
                        "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set.",
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
                    name: "mutedetie",
                    description: Some(
                        "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode.",
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
                    name: "wckcfgie",
                    description: Some(
                        "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE[1] = 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in TDM mode and is meaningless in other modes.",
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
                    name: "freqie",
                    description: Some(
                        "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interruption in receiver mode,",
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
                    name: "cnrdyie",
                    description: Some(
                        "Codec not ready interrupt enable (AC97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interruption i generated. This bit has a meaning only if the AC97 mode is selected through PRTCFG[1:0] bits and the audio block is operates as a receiver.",
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
                    name: "afsdetie",
                    description: Some(
                        "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master.",
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
                    name: "lfsdetie",
                    description: Some(
                        "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master.",
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
            name: "Pdmcr",
            extends: None,
            description: Some(
                "PDM control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdmen",
                    description: Some(
                        "PDM enable",
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
                    name: "micnbr",
                    description: Some(
                        "Number of microphones",
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
                    name: "cken",
                    description: Some(
                        "Clock enable of bitstream clock number 1",
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
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pdmdly",
            extends: None,
            description: Some(
                "PDM delay register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlyml",
                    description: Some(
                        "Delay line adjust for first microphone of pair 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
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
                Field {
                    name: "dlymr",
                    description: Some(
                        "Delay line adjust for second microphone of pair 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
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
            name: "Slotr",
            extends: None,
            description: Some(
                "This register has no meaning in AC97 and SPDIF audio protocol",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fboff",
                    description: Some(
                        "First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slotsz",
                    description: Some(
                        "Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined. Refer to Section: Output data line management on an inactive slot for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Slotsz",
                    ),
                },
                Field {
                    name: "nbslot",
                    description: Some(
                        "Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC97 or SPDIF mode.",
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
                    name: "sloten",
                    description: Some(
                        "Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC97 or SPDIF mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: Some(
                        "Sloten",
                    ),
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
                    name: "ovrudr",
                    description: Some(
                        "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register.",
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
                    name: "mutedet",
                    description: Some(
                        "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.",
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
                    name: "wckcfg",
                    description: Some(
                        "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE[1] = 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wckcfg",
                    ),
                },
                Field {
                    name: "freq",
                    description: Some(
                        "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register.",
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
                    name: "cnrdy",
                    description: Some(
                        "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cnrdy",
                    ),
                },
                Field {
                    name: "afsdet",
                    description: Some(
                        "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register.",
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
                    name: "lfsdet",
                    description: Some(
                        "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register",
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
                    name: "flvl",
                    description: Some(
                        "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Flvl",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ckstr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "Data strobing edge is falling edge of SCK",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "Data strobing edge is rising edge of SCK",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cnrdy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "READY",
                    description: Some(
                        "External AC’97 Codec is ready",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOT_READY",
                    description: Some(
                        "External AC’97 Codec is not ready",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Comp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_COMPANDING",
                    description: Some(
                        "No companding algorithm",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MU_LAW",
                    description: Some(
                        "μ-Law algorithm",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ALAW",
                    description: Some(
                        "A-Law algorithm",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cpl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONES_COMPLEMENT",
                    description: Some(
                        "1’s complement representation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TWOS_COMPLEMENT",
                    description: Some(
                        "2’s complement representation",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ds",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BIT8",
                    description: Some(
                        "8 bits",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "10 bits",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BIT16",
                    description: Some(
                        "16 bits",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BIT20",
                    description: Some(
                        "20 bits",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BIT24",
                    description: Some(
                        "24 bits",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BIT32",
                    description: Some(
                        "32 bits",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Flvl",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "EMPTY",
                    description: Some(
                        "FIFO empty",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER1",
                    description: Some(
                        "FIFO <= 1⁄4 but not empty",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "QUARTER2",
                    description: Some(
                        "1⁄4 < FIFO <= 1⁄2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "QUARTER3",
                    description: Some(
                        "1⁄2 < FIFO <= 3⁄4",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "QUARTER4",
                    description: Some(
                        "3⁄4 < FIFO but not full",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "FIFO full",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Fsoff",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_FIRST",
                    description: Some(
                        "FS is asserted on the first bit of the slot 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BEFORE_FIRST",
                    description: Some(
                        "FS is asserted one bit before the first bit of the slot 0",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fspol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "FS is active low (falling edge)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "FS is active high (rising edge)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fth",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "EMPTY",
                    description: Some(
                        "FIFO empty",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER1",
                    description: Some(
                        "1⁄4 FIFO",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "QUARTER2",
                    description: Some(
                        "1⁄2 FIFO",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "QUARTER3",
                    description: Some(
                        "3⁄4 FIFO",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "FIFO full",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Lsbfirst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSB_FIRST",
                    description: Some(
                        "Data are transferred with MSB first",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSB_FIRST",
                    description: Some(
                        "Data are transferred with LSB first",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mckdiv",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "DIV18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "DIV19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "DIV20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "DIV21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "DIV22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "DIV23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "DIV24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "DIV25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "DIV26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "DIV27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "DIV28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "DIV29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "DIV30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "DIV31",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "DIV33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "DIV34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "DIV35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "DIV36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "DIV37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "DIV38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "DIV39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "DIV40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "DIV41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "DIV42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "DIV43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "DIV44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "DIV45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "DIV46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "DIV47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "DIV48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "DIV49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "DIV50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "DIV51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "DIV52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "DIV53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "DIV54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "DIV55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "DIV56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "DIV57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "DIV58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "DIV59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "DIV60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "DIV61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "DIV62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "DIV63",
                    description: None,
                    value: 63,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MASTER_TX",
                    description: Some(
                        "Master transmitter",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASTER_RX",
                    description: Some(
                        "Master receiver",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SLAVE_TX",
                    description: Some(
                        "Slave transmitter",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SLAVE_RX",
                    description: Some(
                        "Slave receiver",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mono",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STEREO",
                    description: Some(
                        "Stereo mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MONO",
                    description: Some(
                        "Mono mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Muteval",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SEND_ZERO",
                    description: Some(
                        "Bit value 0 is sent during the mute mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SEND_LAST",
                    description: Some(
                        "Last values are sent during the mute mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Outdriv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_START",
                    description: Some(
                        "Audio block output driven when SAIEN is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IMMEDIATELY",
                    description: Some(
                        "Audio block output driven immediately after the setting of this bit",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Prtcfg",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FREE",
                    description: Some(
                        "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SPDIF",
                    description: Some(
                        "SPDIF protocol",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "AC97",
                    description: Some(
                        "AC’97 protocol",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sloten",
            description: None,
            bit_size: 16,
            variants: &[
                EnumVariant {
                    name: "INACTIVE",
                    description: Some(
                        "Inactive slot",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Active slot",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Slotsz",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DATA_SIZE",
                    description: Some(
                        "The slot size is equivalent to the data size (specified in DS[3:0] in the SAI_xCR1 register)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT16",
                    description: Some(
                        "16-bit",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIT32",
                    description: Some(
                        "32-bit",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Syncen",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ASYNCHRONOUS",
                    description: Some(
                        "audio sub-block in asynchronous mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INTERNAL",
                    description: Some(
                        "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Wckcfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CORRECT",
                    description: Some(
                        "Clock configuration is correct",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WRONG",
                    description: Some(
                        "Clock configuration does not respect the rule concerning the frame length specification",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
