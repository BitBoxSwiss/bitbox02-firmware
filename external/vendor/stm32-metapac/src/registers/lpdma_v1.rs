
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Channel",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "lbar",
                    description: Some(
                        "LPDMA channel 15 linked-list base address register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChLbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fcr",
                    description: Some(
                        "LPDMA channel 15 flag clear register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChFcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "LPDMA channel 15 status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "LPDMA channel 15 control register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tr1",
                    description: Some(
                        "LPDMA channel 15 transfer register 1",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChTr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tr2",
                    description: Some(
                        "LPDMA channel 15 transfer register 2",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChTr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br1",
                    description: Some(
                        "LPDMA channel 15 alternate block register 1",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChBr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar",
                    description: Some(
                        "LPDMA channel 15 source address register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar",
                    description: Some(
                        "LPDMA channel 15 destination address register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "tr3",
                    description: Some(
                        "LPDMA channel 15 transfer register 3",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChTr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br2",
                    description: Some(
                        "LPDMA channel 15 block register 2",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChBr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llr",
                    description: Some(
                        "LPDMA channel 15 alternate linked-list address register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChLlr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Lpdma",
            extends: None,
            description: Some(
                "LPDMA",
            ),
            items: &[
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "LPDMA secure configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "LPDMA privileged configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcfglockr",
                    description: Some(
                        "LPDMA configuration lock register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcfglockr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misr",
                    description: Some(
                        "LPDMA non-secure masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smisr",
                    description: Some(
                        "LPDMA secure masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch",
                    description: None,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Channel",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChBr1",
            extends: None,
            description: Some(
                "LPDMA channel 15 alternate block register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bndt",
                    description: Some(
                        "block number of data bytes to transfer from the source. Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT[15:0] is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT[15:0] = 0): - if CH[x].LLR.UB1 = 1, this field is updated by the LLI in the memory. - if CH[x].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all CH[x].LLR.Uxx = 0 and if CH[x].LLR.LA[15:0] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if CH[x].LLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT[2:0] versus CH[x].TR1.SDW_LOG2[1:0]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (CH[x].TR1.PAM[1]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT[2:0] versus CH[x].TR1.DDW[1:0]). Else a user setting error is reported and no transfer is issued.",
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
                    name: "brc",
                    description: Some(
                        "Block repeat counter. This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC[10:0] = BNDT[15:0] = 0): If CH[x].LLR.UB1 = 1, all CH[x].BR1 fields are updated by the next LLI in the memory. If CH[x].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all CH[x].LLR.Uxx = 0 and if CH[x].LLR.LA[15:0] ≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if CH[x].LLR = 0, this field is kept as zero following the last LLI and data transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdec",
                    description: Some(
                        "source address decrement",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dec",
                    ),
                },
                Field {
                    name: "ddec",
                    description: Some(
                        "destination address decrement",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dec",
                    ),
                },
                Field {
                    name: "brsdec",
                    description: Some(
                        "Block repeat source address decrement. Note: On top of this increment/decrement (depending on BRSDEC), CH[x].SAR is in the same time also updated by the increment/decrement (depending on SDEC) of the CH[x].TR3.SAO value, as it is done after any programmed burst transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dec",
                    ),
                },
                Field {
                    name: "brddec",
                    description: Some(
                        "Block repeat destination address decrement. Note: On top of this increment/decrement (depending on BRDDEC), CH[x].DAR is in the same time also updated by the increment/decrement (depending on DDEC) of the CH[x].TR3.DAO value, as it is usually done at the end of each programmed burst transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dec",
                    ),
                },
            ],
        },
        FieldSet {
            name: "ChBr2",
            extends: None,
            description: Some(
                "LPDMA channel 12 block register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brsao",
                    description: Some(
                        "Block repeated source address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH[x].BR1.BRSDEC) the current source address (CH[x].SAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO[2:0] versus CH[x].TR1.SDW_LOG2[1:0]). Else a user setting error is reported and no transfer is issued.",
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
                    name: "brdao",
                    description: Some(
                        "Block repeated destination address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH[x].BR1.BRDDEC) the current destination address (CH[x].DAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO[2:0] versus CH[x].TR1.DDW[1:0]). Else a user setting error is reported and no transfer is issued.",
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
            name: "ChCr",
            extends: None,
            description: Some(
                "LPDMA channel 11 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "enable. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.",
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
                    name: "reset",
                    description: Some(
                        "reset. This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (CH[x].SR.SUSPF = 1 and CH[x].SR.IDLEF = CH[x].CR.EN = 1). - channel in disabled state (CH[x].SR.IDLEF = 1 and CH[x].CR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (CH[x].BR1, CH[x].SAR and CH[x].DAR) before enabling again the channel (see the programming sequence in ).",
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
                    name: "susp",
                    description: Some(
                        "suspend. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going LPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .",
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
                    name: "tcie",
                    description: Some(
                        "transfer complete interrupt enable",
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
                    name: "htie",
                    description: Some(
                        "half transfer complete interrupt enable",
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
                    name: "dteie",
                    description: Some(
                        "data transfer error interrupt enable",
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
                    name: "uleie",
                    description: Some(
                        "update link transfer error interrupt enable",
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
                    name: "useie",
                    description: Some(
                        "user setting error interrupt enable",
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
                    name: "suspie",
                    description: Some(
                        "completed suspension interrupt enable",
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
                    name: "toie",
                    description: Some(
                        "trigger overrun interrupt enable",
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
                    name: "lsm",
                    description: Some(
                        "Link step mode. First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until CH[x].BR1.BNDT[15:0] = 0 and CH[x].BR1.BRC[10:0] = 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by CH[x].LLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsm",
                    ),
                },
                Field {
                    name: "prio",
                    description: Some(
                        "priority level of the channel x LPDMA transfer versus others. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Prio",
                    ),
                },
            ],
        },
        FieldSet {
            name: "ChFcr",
            extends: None,
            description: Some(
                "LPDMA channel 7 flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcf",
                    description: Some(
                        "transfer complete flag clear",
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
                    name: "htf",
                    description: Some(
                        "half transfer flag clear",
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
                    name: "dtef",
                    description: Some(
                        "data transfer error flag clear",
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
                    name: "ulef",
                    description: Some(
                        "update link transfer error flag clear",
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
                    name: "usef",
                    description: Some(
                        "user setting error flag clear",
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
                    name: "suspf",
                    description: Some(
                        "completed suspension flag clear",
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
                    name: "tof",
                    description: Some(
                        "trigger overrun flag clear",
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
            name: "ChLbar",
            extends: None,
            description: Some(
                "LPDMA channel 14 linked-list base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lba",
                    description: Some(
                        "linked-list base address of LPDMA channel x",
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
            name: "ChLlr",
            extends: None,
            description: Some(
                "LPDMA channel 15 alternate linked-list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "la",
                    description: Some(
                        "pointer (16-bit low-significant address) to the next linked-list data structure. If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA[15:20] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list LPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list LPDMA internal register file (CH[x].CTR1, CH[x].TR2, CH[x].BR1, CH[x].SAR, CH[x].DAR and CH[x].LLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ull",
                    description: Some(
                        "Update CH[x].LLR register from memory. This bit is used to control the update of CH[x].LLR from the memory during the link transfer.",
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
                    name: "ub2",
                    description: Some(
                        "Update CH[x].BR2 from memory. This bit controls the update of CH[x].BR2 from the memory during the link transfer.",
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
                    name: "ut3",
                    description: Some(
                        "Update CH[x].TR3 from memory. This bit controls the update of CH[x].TR3 from the memory during the link transfer.",
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
                    name: "uda",
                    description: Some(
                        "Update CH[x].DAR register from memory. This bit is used to control the update of CH[x].DAR from the memory during the link transfer.",
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
                Field {
                    name: "usa",
                    description: Some(
                        "update CH[x].SAR from memory. This bit controls the update of CH[x].SAR from the memory during the link transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ub1",
                    description: Some(
                        "Update CH[x].BR1 from memory. This bit controls the update of CH[x].BR1 from the memory during the link transfer. If UB1 = 0 and if CH[x].LLR ≠ 0, the linked-list is not completed. CH[x].BR1.BNDT[15:0] is then restored to the programmed value after data transfer is completed and before the link transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ut2",
                    description: Some(
                        "Update CH[x].TR2 from memory. This bit controls the update of CH[x].TR2 from the memory during the link transfer.",
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
                    name: "ut1",
                    description: Some(
                        "Update CH[x].TR1 from memory. This bit controls the update of CH[x].TR1 from the memory during the link transfer.",
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
            name: "ChSr",
            extends: None,
            description: Some(
                "LPDMA channel 15 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idlef",
                    description: Some(
                        "idle flag. This idle flag is de-asserted by hardware when the channel is enabled (CH[x].CR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state).",
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
                    name: "tcf",
                    description: Some(
                        "transfer complete flag. A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (CH[x].TR2.TCEM[1:0]).",
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
                    name: "htf",
                    description: Some(
                        "half transfer flag. An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode (CH[x].TR2.TCEM[1:0]). An half block transfer occurs when half of the bytes of the source block size (rounded up integer of CH[x].BR1.BNDT[15:0]/2) has been transferred to the destination. An half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (CH[x].BR1.BRC[10:0]+1)/2)) has been transferred to the destination.",
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
                    name: "dtef",
                    description: Some(
                        "data transfer error flag",
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
                    name: "ulef",
                    description: Some(
                        "update link transfer error flag",
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
                    name: "usef",
                    description: Some(
                        "user setting error flag",
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
                    name: "suspf",
                    description: Some(
                        "completed suspension flag",
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
                    name: "tof",
                    description: Some(
                        "trigger overrun flag",
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
            name: "ChTr1",
            extends: None,
            description: Some(
                "LPDMA channel 8 transfer register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdw",
                    description: Some(
                        "binary logarithm of the source data width of a burst in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (CH[x].BR1.BNDT[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address CH[x].SAR[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error is reported and none transfer is issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dw",
                    ),
                },
                Field {
                    name: "sinc",
                    description: Some(
                        "source incrementing burst. The source address, pointed by CH[x].SAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.",
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
                    name: "pam",
                    description: Some(
                        "padding/alignment mode. If DDW[1:0] = SDW_LOG2[1:0]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer. - Case 2: If destination data width < source data width. 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination. Note:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pam",
                    ),
                },
                Field {
                    name: "ssec",
                    description: Some(
                        "security attribute of the LPDMA transfer from the source. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx =1 . A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.",
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
                Field {
                    name: "ddw",
                    description: Some(
                        "binary logarithm of the destination data width of a burst, in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address CH[x].DAR[2:0] and address offset CH[x].TR3.DAO[2:0], versus DDW[1:0]). Otherwise a user setting error is reported and no transfer is issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dw",
                    ),
                },
                Field {
                    name: "dinc",
                    description: Some(
                        "destination incrementing burst. The destination address, pointed by CH[x].DAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.",
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
                    name: "dsec",
                    description: Some(
                        "security attribute of the LPDMA transfer to the destination. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx = 1. A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.",
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
            name: "ChTr2",
            extends: None,
            description: Some(
                "LPDMA channel 10 transfer register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reqsel",
                    description: Some(
                        "LPDMA hardware request selection. These bits are ignored if channel x is activated (CH[x].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL[6:0] value) to different active LPDMA channels (CH[x].CR.EN = 1 and CH[x].TR2.SWREQ = 0 for these channels). LPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.",
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
                    name: "swreq",
                    description: Some(
                        "software request. This bit is internally taken into account when CH[x].CR.EN is asserted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Swreq",
                    ),
                },
                Field {
                    name: "dreq",
                    description: Some(
                        "destination hardware request. This bit is ignored if channel x is activated (CH[x].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dreq",
                    ),
                },
                Field {
                    name: "breq",
                    description: Some(
                        "Block hardware request. If the channel x is activated (CH[x].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Breq",
                    ),
                },
                Field {
                    name: "trigm",
                    description: Some(
                        "trigger mode. These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (CH[x].CR.EN asserted) with TRIGPOL[1:0] = 00 or 11, these TRIGM[1:0] bits are ignored. Else, a LPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The LPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL[1:0] = 01 or respectively TRIGPOL[1:0] = 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL[5:0] is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the CH[x].TR2 with a new value for any of TRIGSEL[5:0] or TRIGPOL[1:0], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (CH[x].SR.TOF =1 ), and an interrupt is generated if enabled (CH[x].CR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM[1:0] = 11 and (SWREQ =1  or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM[1:0] = 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Trigm",
                    ),
                },
                Field {
                    name: "trigsel",
                    description: Some(
                        "trigger event input selection. These bits select the trigger event input of the LPDMA transfer (as per ), with an active trigger event if TRIGPOL[1:0] ≠ 00.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigpol",
                    description: Some(
                        "trigger event polarity. These bits define the polarity of the selected trigger event input defined by TRIGSEL[5:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Trigpol",
                    ),
                },
                Field {
                    name: "tcem",
                    description: Some(
                        "transfer complete event mode. These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH[x].BR1.BNDT[15:0] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH[x].BR1.BNDT[15:0] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with CH[x].BR1.BNDT[15:0] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tcem",
                    ),
                },
            ],
        },
        FieldSet {
            name: "ChTr3",
            extends: None,
            description: Some(
                "LPDMA channel 14 transfer register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sao",
                    description: Some(
                        "source address offset increment. The source address, pointed by CH[x].SAR, is incremented or decremented (depending on CH[x].BR1.SDEC) by this offset SAO[12:0] for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH[x].TR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO[2:0] versus CH[x].TR1.SDW_LOG2[1:0]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional CH[x].TR3.SAO[12:0] is not applied.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dao",
                    description: Some(
                        "destination address offset increment. The destination address, pointed by CH[x].DAR, is incremented or decremented (depending on CH[x].BR1.DDEC) by this offset DAO[12:0] for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH[x].TR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO[2:0] versus CH[x].TR1.DDW[1:0]). Else, a user setting error is reported and no transfer is issued.",
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
            name: "Misr",
            extends: None,
            description: Some(
                "LPDMA secure masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mis",
                    description: Some(
                        "MIS0",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "LPDMA privileged configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv_",
                    description: Some(
                        "PRIV0",
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
            name: "Rcfglockr",
            extends: None,
            description: Some(
                "LPDMA configuration lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "LOCK0",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "LPDMA secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec",
                    description: Some(
                        "SEC0",
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
    ],
    enums: &[
        Enum {
            name: "Breq",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BURST",
                    description: Some(
                        "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BLOCK",
                    description: Some(
                        "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level (see ).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dec",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ADD",
                    description: Some(
                        "The address is incremented by the programmed offset.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SUBTRACT",
                    description: Some(
                        "The address is decremented by the programmed offset.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dreq",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SOURCE_PERIPHERAL",
                    description: Some(
                        "selected hardware request driven by a source peripheral (request signal taken into account by the LPDMA transfer scheduler over the source/read port)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DESTINATION_PERIPHERAL",
                    description: Some(
                        "selected hardware request driven by a destination peripheral (request signal taken into account by the LPDMA transfer scheduler over the destination/write port)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BYTE",
                    description: Some(
                        "byte",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF_WORD",
                    description: Some(
                        "half-word (2 bytes)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WORD",
                    description: Some(
                        "word (4 bytes)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lsm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RUN_TO_COMPLETION",
                    description: Some(
                        "channel executed for the full linked-list and completed at the end of the last LLI (CH[x].LLR = 0). The 16 low-significant bits of the link address are null (LA[15:0] = 0) and all the update bits are null (UT1 =UB1 = UT2 = USA = UDA = ULL = 0 and UT3 = UB2 = 0 if present). Then CH[x].BR1.BNDT[15:0] = 0 and CH[x].BR1.BRC[10:0] = 0 if present.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LINK_STEP",
                    description: Some(
                        "channel executed once for the current LLI",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pam",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ZERO_EXTEND_OR_LEFT_TRUNCATE",
                    description: Some(
                        "If destination is wider: source data is transferred as right aligned, padded with 0s up to the destination data width\nIf source is wider: source data is transferred as right aligned, left-truncated down to the destination data width",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SIGN_EXTEND_OR_RIGHT_TRUNCATE",
                    description: Some(
                        "If destination is wider: source data is transferred as right aligned, sign extended up to the destination data width\nIf source is wider: source data is transferred as left-aligned, right-truncated down to the destination data width",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PACK",
                    description: Some(
                        "source data is FIFO queued and packed/unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Prio",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW_WITH_LOWH_WEIGHT",
                    description: Some(
                        "low priority, low weight",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_WITH_MID_WEIGHT",
                    description: Some(
                        "low priority, mid weight",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LOW_WITH_HIGH_WEIGHT",
                    description: Some(
                        "low priority, high weight",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "high priority",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Swreq",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HARDWARE",
                    description: Some(
                        "no software request. The selected hardware request REQSEL[6:0] is taken into account.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "software request for a memory-to-memory transfer. The default selected hardware request as per REQSEL[6:0] is ignored.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tcem",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EACH_BLOCK",
                    description: Some(
                        "at block level (when CH[x].BR1.BNDT[15:0] = 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EACH2DBLOCK",
                    description: Some(
                        "channel x = 0 to 11, same as 00; channel x=12 to 15, at 2D/repeated block level (when CH[x].BR1.BRC[10:0] =  0 and CH[x].BR1.BNDT[15:0] =  0), the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "EACH_LINKED_LIST_ITEM",
                    description: Some(
                        "at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block transfer or a 2D/repeated block transfer for channel x = 12 to 15), if any data transfer.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LAST_LINKED_LIST_ITEM",
                    description: Some(
                        "at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI updates the link address CH[x].LLR.LA[15:2] to zero and clears all the CH[x].LLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2 if present). If the channel transfer is continuous/infinite, no event is generated.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Trigm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BLOCK",
                    description: Some(
                        "at block level: the first burst read of each block transfer is conditioned by one hit trigger (channel x = 12 to 15, for each block if a 2D/repeated block is configured with CH[x].BR1.BRC[10:0] ≠ 0).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_2DBLOCK",
                    description: Some(
                        "channel x = 0 to 11, same as 00; channel x=12 to 15, at 2D/repeated block level, the",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LINKED_LIST_ITEM",
                    description: Some(
                        "at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if any) is not conditioned.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BURST",
                    description: Some(
                        "at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected peripheral, is conditioned by one hit trigger.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Trigpol",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "no trigger (masked trigger event)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "trigger on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "trigger on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NONE_ALT",
                    description: Some(
                        "same as 00",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
