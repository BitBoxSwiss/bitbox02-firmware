
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ch",
            extends: None,
            description: Some(
                "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers.",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "MDMA channel x interrupt/status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some(
                        "MDMA channel x interrupt flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esr",
                    description: Some(
                        "MDMA Channel x error status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Esr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "This register is used to control the concerned channel.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "tcr",
                    description: Some(
                        "This register is used to configure the concerned channel.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bndtr",
                    description: Some(
                        "MDMA Channel x block number of data register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bndtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar",
                    description: Some(
                        "MDMA channel x source address register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dar",
                    description: Some(
                        "MDMA channel x destination address register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "brur",
                    description: Some(
                        "MDMA channel x Block Repeat address Update register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Brur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lar",
                    description: Some(
                        "MDMA channel x Link Address register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tbr",
                    description: Some(
                        "MDMA channel x Trigger and Bus selection Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mar",
                    description: Some(
                        "MDMA channel x Mask address register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdr",
                    description: Some(
                        "MDMA channel x Mask Data register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Mdma",
            extends: None,
            description: Some(
                "MDMA.",
            ),
            items: &[
                BlockItem {
                    name: "gisr0",
                    description: Some(
                        "MDMA Global Interrupt/Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gisr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ch",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bndtr",
            extends: None,
            description: Some(
                "MDMA Channel x block number of data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bndt",
                    description: Some(
                        "block number of data to transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brsum",
                    description: Some(
                        "Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Updatemode",
                    ),
                },
                Field {
                    name: "brdum",
                    description: Some(
                        "Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Updatemode",
                    ),
                },
                Field {
                    name: "brc",
                    description: Some(
                        "Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Brur",
            extends: None,
            description: Some(
                "MDMA channel x Block Repeat address Update register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "suv",
                    description: Some(
                        "source adresse update value.",
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
                    name: "duv",
                    description: Some(
                        "destination address update.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "This register is used to control the concerned channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "channel enable.",
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
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable This bit is set and cleared by software.",
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
                    name: "ctcie",
                    description: Some(
                        "Channel Transfer Complete interrupt enable This bit is set and cleared by software.",
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
                    name: "brtie",
                    description: Some(
                        "Block Repeat transfer interrupt enable This bit is set and cleared by software.",
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
                    name: "btie",
                    description: Some(
                        "Block Transfer interrupt enable This bit is set and cleared by software.",
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
                    name: "tcie",
                    description: Some(
                        "buffer Transfer Complete interrupt enable This bit is set and cleared by software.",
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
                    name: "pl",
                    description: Some(
                        "Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pl",
                    ),
                },
                Field {
                    name: "bex",
                    description: Some(
                        "byte Endianness exchange.",
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
                    name: "hex",
                    description: Some(
                        "Half word Endianes exchange.",
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
                    name: "wex",
                    description: Some(
                        "Word Endianness exchange.",
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
                    name: "swrq",
                    description: Some(
                        "SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access).",
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
            name: "Dar",
            extends: None,
            description: Some(
                "MDMA channel x destination address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dar",
                    description: Some(
                        "Destination adr base.",
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
            name: "Esr",
            extends: None,
            description: Some(
                "MDMA Channel x error status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tea",
                    description: Some(
                        "Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error.",
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
                    name: "ted",
                    description: Some(
                        "Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error.",
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
                    name: "teld",
                    description: Some(
                        "Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.",
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
                    name: "temd",
                    description: Some(
                        "Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.",
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
                    name: "ase",
                    description: Some(
                        "Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.",
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
                    name: "bse",
                    description: Some(
                        "Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.",
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
            ],
        },
        FieldSet {
            name: "Gisr0",
            extends: None,
            description: Some(
                "MDMA Global Interrupt/Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gif",
                    description: Some(
                        "Channel x global interrupt flag (x=0..15) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx).",
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
            name: "Ifcr",
            extends: None,
            description: Some(
                "MDMA channel x interrupt flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cteif",
                    description: Some(
                        "Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register.",
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
                    name: "cctcif",
                    description: Some(
                        "Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register.",
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
                    name: "cbrtif",
                    description: Some(
                        "Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register.",
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
                    name: "cbtif",
                    description: Some(
                        "Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register.",
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
                    name: "cltcif",
                    description: Some(
                        "CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register.",
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
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "MDMA channel x interrupt/status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teif",
                    description: Some(
                        "Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.",
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
                    name: "ctcif",
                    description: Some(
                        "Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.",
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
                    name: "brtif",
                    description: Some(
                        "Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.",
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
                    name: "btif",
                    description: Some(
                        "Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.",
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
                    name: "tcif",
                    description: Some(
                        "channel x buffer transfer complete.",
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
                    name: "crqa",
                    description: Some(
                        "channel x request active flag.",
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
            name: "Lar",
            extends: None,
            description: Some(
                "MDMA channel x Link Address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lar",
                    description: Some(
                        "Link address register.",
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
            name: "Mar",
            extends: None,
            description: Some(
                "MDMA channel x Mask address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mar",
                    description: Some(
                        "Mask address.",
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
            name: "Mdr",
            extends: None,
            description: Some(
                "MDMA channel x Mask Data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdr",
                    description: Some(
                        "Mask data.",
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
            name: "Sar",
            extends: None,
            description: Some(
                "MDMA channel x source address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sar",
                    description: Some(
                        "source adr base.",
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
            name: "Tbr",
            extends: None,
            description: Some(
                "MDMA channel x Trigger and Bus selection Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsel",
                    description: Some(
                        "Trigger selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbus",
                    description: Some(
                        "Source BUS select This bit is protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bus",
                    ),
                },
                Field {
                    name: "dbus",
                    description: Some(
                        "Destination BUS slect This bit is protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bus",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Tcr",
            extends: None,
            description: Some(
                "This register is used to configure the concerned channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR[31:0] + 0x00).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Incmode",
                    ),
                },
                Field {
                    name: "dinc",
                    description: Some(
                        "Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Incmode",
                    ),
                },
                Field {
                    name: "ssize",
                    description: Some(
                        "Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wordsize",
                    ),
                },
                Field {
                    name: "dsize",
                    description: Some(
                        "Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wordsize",
                    ),
                },
                Field {
                    name: "sincos",
                    description: Some(
                        "source increment offset size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wordsize",
                    ),
                },
                Field {
                    name: "dincos",
                    description: Some(
                        "Destination increment offset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wordsize",
                    ),
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "source burst transfer configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Burst",
                    ),
                },
                Field {
                    name: "dburst",
                    description: Some(
                        "Destination burst transfer configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Burst",
                    ),
                },
                Field {
                    name: "tlen",
                    description: Some(
                        "buffer transfer lengh.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pke",
                    description: Some(
                        "PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM[0] value. This bit is protected and can be written only if EN is 0.",
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
                    name: "pam",
                    description: Some(
                        "Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pam",
                    ),
                },
                Field {
                    name: "trgm",
                    description: Some(
                        "Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Trgm",
                    ),
                },
                Field {
                    name: "swrm",
                    description: Some(
                        "SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.",
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
                    name: "bwm",
                    description: Some(
                        "Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.",
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
            name: "Burst",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Single transfer",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCR4",
                    description: Some(
                        "Incremental burst of 4 beats",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "INCR8",
                    description: Some(
                        "Incremental burst of 8 beats",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "INCR16",
                    description: Some(
                        "Incremental burst of 16 beats",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "INCR32",
                    description: Some(
                        "Incremental burst of 32 beats",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "INCR64",
                    description: Some(
                        "Incremental burst of 64 beats",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "INCR128",
                    description: Some(
                        "Incremental burst of 128 beats",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "INCR256",
                    description: Some(
                        "Incremental burst of 256 beats",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Bus",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SYSTEM",
                    description: Some(
                        "System/AXI bus",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AHB",
                    description: Some(
                        "AHB bus/TCM",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Incmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FIXED",
                    description: Some(
                        "Address pointer is fixed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RESERVED",
                    description: Some(
                        "Reserved",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "INCREMENT",
                    description: Some(
                        "Address pointer is incremented after each data transfer",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DECREMENT",
                    description: Some(
                        "Address pointer is decremented after each data transfer",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pam",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RIGHT_PADDED",
                    description: Some(
                        "Right aligned, padded with 0s (default). If source data is larger than destination size, only LSB part of the source is written to the destination address. The reminder part is discarded",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RIGHT_SIGN_EXTENDED",
                    description: Some(
                        "Right aligned, sign extended",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEFT_PADDED",
                    description: Some(
                        "Left aligned (padded with 0s). if source data is larger than destination size, only MSB part of the source is written to the destination address. The reminder part is discarded",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some(
                        "Medium",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VERY_HIGH",
                    description: Some(
                        "Very high",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Trgm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BUFFER",
                    description: Some(
                        "Each MDMA request (software or hardware) triggers a buffer transfer",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BLOCK",
                    description: Some(
                        "Each MDMA request (software or hardware) triggers a block transfer",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "REPEATED",
                    description: Some(
                        "Each MDMA request (software or hardware) triggers a repeated block transfer",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WHOLE_DATA",
                    description: Some(
                        "Each MDMA request (software or hardware) triggers the transfer of the whole data for the respective channel (for example linked list) until the channel reach the end and it is disabled.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Updatemode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ADD",
                    description: Some(
                        "Add",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SUBTRACT",
                    description: Some(
                        "Subtract",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wordsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BYTE",
                    description: Some(
                        "Byte (8-bit)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF_WORD",
                    description: Some(
                        "HalfWord (16-bit)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WORD",
                    description: Some(
                        "Word (32-bit)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DOUBLE_WORD",
                    description: Some(
                        "DoubleWord (64-bit)",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
