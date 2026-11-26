
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fmc",
            extends: None,
            description: Some(
                "Flexible memory controller.",
            ),
            items: &[
                BlockItem {
                    name: "nor_psram",
                    description: None,
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "NorPsram",
                        },
                    ),
                },
                BlockItem {
                    name: "nand",
                    description: None,
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Nand",
                        },
                    ),
                },
                BlockItem {
                    name: "sdram",
                    description: None,
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Sdram",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Nand",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "pcr",
                    description: Some(
                        "NAND Flash control registers.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "FIFO status and interrupt register.",
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
                    name: "pmem",
                    description: Some(
                        "Common memory space timing register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "patt",
                    description: Some(
                        "Attribute memory space timing register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Patt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr",
                    description: Some(
                        "ECC result registers.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "NorPsram",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "bcr1",
                    description: Some(
                        "SRAM/NOR-Flash chip-select control register for bank 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select timing register for bank 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Btr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bcr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select control register for bank 2.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcscntr",
                    description: Some(
                        "PSRAM chip select counter register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcscntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bwtr",
                    description: Some(
                        "SRAM/NOR-Flash write timing registers 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bwtr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Sdram",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "sdcr",
                    description: Some(
                        "SDRAM control registers 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdtr",
                    description: Some(
                        "SDRAM timing registers 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdcmr",
                    description: Some(
                        "SDRAM Command Mode register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdcmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrtr",
                    description: Some(
                        "SDRAM refresh timer register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdsr",
                    description: Some(
                        "SDRAM status register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select control register for bank 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbken",
                    description: Some(
                        "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus.",
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
                    name: "muxen",
                    description: Some(
                        "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:.",
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
                    name: "mtyp",
                    description: Some(
                        "Memory type Defines the type of external memory attached to the corresponding memory bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width Defines the external memory device width, valid for all type of memories.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "faccen",
                    description: Some(
                        "Flash access enable Enables NOR Flash memory access operations.",
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
                    name: "bursten",
                    description: Some(
                        "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode.",
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
                    name: "waitpol",
                    description: Some(
                        "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitpol",
                    ),
                },
                Field {
                    name: "waitcfg",
                    description: Some(
                        "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitcfg",
                    ),
                },
                Field {
                    name: "wren",
                    description: Some(
                        "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC.",
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
                    name: "waiten",
                    description: Some(
                        "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode.",
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
                    name: "extmod",
                    description: Some(
                        "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).",
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
                    name: "asyncwait",
                    description: Some(
                        "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.",
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
                    name: "cpsize",
                    description: Some(
                        "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cpsize",
                    ),
                },
                Field {
                    name: "cburstrw",
                    description: Some(
                        "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cburstrw",
                    ),
                },
                Field {
                    name: "cclken",
                    description: Some(
                        "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.).",
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
                    name: "wfdis",
                    description: Some(
                        "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register.",
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
                    name: "nblset",
                    description: Some(
                        "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmcen",
                    description: Some(
                        "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register.",
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
            name: "Bcr1",
            extends: Some(
                "BCR",
            ),
            description: Some(
                "SRAM/NOR-Flash chip-select control register for bank 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbken",
                    description: Some(
                        "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus.",
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
                    name: "muxen",
                    description: Some(
                        "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:.",
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
                    name: "mtyp",
                    description: Some(
                        "Memory type Defines the type of external memory attached to the corresponding memory bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width Defines the external memory device width, valid for all type of memories.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "faccen",
                    description: Some(
                        "Flash access enable Enables NOR Flash memory access operations.",
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
                    name: "bursten",
                    description: Some(
                        "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode.",
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
                    name: "waitpol",
                    description: Some(
                        "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitpol",
                    ),
                },
                Field {
                    name: "waitcfg",
                    description: Some(
                        "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitcfg",
                    ),
                },
                Field {
                    name: "wren",
                    description: Some(
                        "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC.",
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
                    name: "waiten",
                    description: Some(
                        "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode.",
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
                    name: "extmod",
                    description: Some(
                        "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).",
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
                    name: "asyncwait",
                    description: Some(
                        "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.",
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
                    name: "cpsize",
                    description: Some(
                        "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cpsize",
                    ),
                },
                Field {
                    name: "cburstrw",
                    description: Some(
                        "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cburstrw",
                    ),
                },
                Field {
                    name: "cclken",
                    description: Some(
                        "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.).",
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
                    name: "wfdis",
                    description: Some(
                        "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register.",
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
                    name: "nblset",
                    description: Some(
                        "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmcen",
                    description: Some(
                        "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register.",
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
            name: "Btr",
            extends: Some(
                "BWTR",
            ),
            description: Some(
                "SRAM/NOR-Flash chip-select timing register for bank 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ...",
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
                    name: "clkdiv",
                    description: Some(
                        "Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datlat",
                    description: Some(
                        "(see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "accmod",
                    description: Some(
                        "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
                Field {
                    name: "datahld",
                    description: Some(
                        "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bwtr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash write timing registers 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ...",
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
                    name: "accmod",
                    description: Some(
                        "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
                Field {
                    name: "datahld",
                    description: Some(
                        "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Patt",
            extends: None,
            description: Some(
                "Attribute memory space timing register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "attset",
                    description: Some(
                        "Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:.",
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
                    name: "attwait",
                    description: Some(
                        "Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atthold",
                    description: Some(
                        "Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atthiz",
                    description: Some(
                        "Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcr",
            extends: None,
            description: Some(
                "NAND Flash control registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwaiten",
                    description: Some(
                        "Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:.",
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
                    name: "pbken",
                    description: Some(
                        "NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus.",
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
                    name: "ptyp",
                    description: Some(
                        "Memory type Defines the type of device attached to the corresponding memory bank:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ptyp",
                    ),
                },
                Field {
                    name: "pwid",
                    description: Some(
                        "Data bus width Defines the external memory device width.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pwid",
                    ),
                },
                Field {
                    name: "eccen",
                    description: Some(
                        "ECC computation logic enable bit.",
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
                    name: "tclr",
                    description: Some(
                        "CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tar",
                    description: Some(
                        "ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccps",
                    description: Some(
                        "ECC page size Defines the page size for the extended ECC:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Eccps",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pcscntr",
            extends: None,
            description: Some(
                "PSRAM chip select counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cscount",
                    description: Some(
                        "Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0.",
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
                    name: "cntben",
                    description: Some(
                        "Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1.",
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
            name: "Pmem",
            extends: None,
            description: Some(
                "Common memory space timing register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "memset",
                    description: Some(
                        "Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:.",
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
                    name: "memwait",
                    description: Some(
                        "Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "memhold",
                    description: Some(
                        "Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "memhiz",
                    description: Some(
                        "Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdcmr",
            extends: None,
            description: Some(
                "SDRAM Command Mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "ctb",
                    description: Some(
                        "Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "nrfs",
                    description: Some(
                        "Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ....",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrd",
                    description: Some(
                        "Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdcr",
            extends: None,
            description: Some(
                "SDRAM control registers 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nc",
                    description: Some(
                        "Number of column address bits These bits define the number of bits of a column address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Nc",
                    ),
                },
                Field {
                    name: "nr",
                    description: Some(
                        "Number of row address bits These bits define the number of bits of a row address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Nr",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width. These bits define the memory device width.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of internal banks This bit sets the number of internal banks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nb",
                    ),
                },
                Field {
                    name: "cas",
                    description: Some(
                        "CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cas",
                    ),
                },
                Field {
                    name: "wp",
                    description: Some(
                        "Write protection This bit enables write mode access to the SDRAM bank.",
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
                    name: "sdclk",
                    description: Some(
                        "SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sdclk",
                    ),
                },
                Field {
                    name: "rburst",
                    description: Some(
                        "Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care.",
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
                    name: "rpipe",
                    description: Some(
                        "Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rpipe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sdrtr",
            extends: None,
            description: Some(
                "SDRAM refresh timer register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cre",
                    description: Some(
                        "Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.",
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
                    name: "count",
                    description: Some(
                        "Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reie",
                    description: Some(
                        "RES Interrupt Enable.",
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
            name: "Sdsr",
            extends: None,
            description: Some(
                "SDRAM status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Refresh error flag An interrupt is generated if REIE = 1 and RE = 1.",
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
                    name: "modes",
                    description: Some(
                        "Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Modes",
                    ),
                },
                Field {
                    name: "busy",
                    description: Some(
                        "Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request.",
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
            name: "Sdtr",
            extends: None,
            description: Some(
                "SDRAM timing registers 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmrd",
                    description: Some(
                        "Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ....",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txsr",
                    description: Some(
                        "Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tras",
                    description: Some(
                        "Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ....",
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
                    name: "trc",
                    description: Some(
                        "Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care.",
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
                    name: "twr",
                    description: Some(
                        "Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t<sub>WR</sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank.",
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
                    name: "trp",
                    description: Some(
                        "Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trcd",
                    description: Some(
                        "Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ....",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "FIFO status and interrupt register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irs",
                    description: Some(
                        "Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set.",
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
                    name: "ils",
                    description: Some(
                        "Interrupt high-level status The flag is set by hardware and reset by software.",
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
                    name: "ifs",
                    description: Some(
                        "Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set.",
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
                    name: "iren",
                    description: Some(
                        "Interrupt rising edge detection enable bit.",
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
                    name: "ilen",
                    description: Some(
                        "Interrupt high-level detection enable bit.",
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
                    name: "ifen",
                    description: Some(
                        "Interrupt falling edge detection enable bit.",
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
                    name: "fempt",
                    description: Some(
                        "FIFO empty Read-only bit that provides the status of the FIFO.",
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
    ],
    enums: &[
        Enum {
            name: "Accmod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "A",
                    description: Some(
                        "Access mode A",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B",
                    description: Some(
                        "Access mode B",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "C",
                    description: Some(
                        "Access mode C",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "D",
                    description: Some(
                        "Access mode D",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cas",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CLOCKS1",
                    description: Some(
                        "1 cycle",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CLOCKS2",
                    description: Some(
                        "2 cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CLOCKS3",
                    description: Some(
                        "3 cycles",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cburstrw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ASYNCHRONOUS",
                    description: Some(
                        "Write operations are always performed in Asynchronous mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNCHRONOUS",
                    description: Some(
                        "Write operations are performed in Synchronous mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cpsize",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_BURST_SPLIT",
                    description: Some(
                        "No burst split when crossing page boundary",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES128",
                    description: Some(
                        "128 bytes CRAM page size",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTES256",
                    description: Some(
                        "256 bytes CRAM page size",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BYTES512",
                    description: Some(
                        "512 bytes CRAM page size",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BYTES1024",
                    description: Some(
                        "1024 bytes CRAM page size",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Eccps",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BYTES256",
                    description: Some(
                        "ECC page size 256 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES512",
                    description: Some(
                        "ECC page size 512 bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTES1024",
                    description: Some(
                        "ECC page size 1024 bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BYTES2048",
                    description: Some(
                        "ECC page size 2048 bytes",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BYTES4096",
                    description: Some(
                        "ECC page size 4096 bytes",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BYTES8192",
                    description: Some(
                        "ECC page size 8192 bytes",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal Mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CLOCK_CONFIGURATION_ENABLE",
                    description: Some(
                        "Clock Configuration Enable",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PALL",
                    description: Some(
                        "PALL (All Bank Precharge) command",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUTO_REFRESH_COMMAND",
                    description: Some(
                        "Auto-refresh command",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LOAD_MODE_REGISTER",
                    description: Some(
                        "Load Mode Resgier",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SELF_REFRESH_COMMAND",
                    description: Some(
                        "Self-refresh command",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "POWER_DOWN_COMMAND",
                    description: Some(
                        "Power-down command",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Modes",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal Mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SELF_REFRESH",
                    description: Some(
                        "Self-refresh mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "POWER_DOWN",
                    description: Some(
                        "Power-down mode",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mtyp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "SRAM memory type",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PSRAM",
                    description: Some(
                        "PSRAM (CRAM) memory type",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FLASH",
                    description: Some(
                        "NOR Flash/OneNAND Flash",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mwid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "Memory data bus width 8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "Memory data bus width 16 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "Memory data bus width 32 bits",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Nb",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NB2",
                    description: Some(
                        "Two internal Banks",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NB4",
                    description: Some(
                        "Four internal Banks",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Nc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS9",
                    description: Some(
                        "9 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10 bits",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS11",
                    description: Some(
                        "11 bits",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Nr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS11",
                    description: Some(
                        "11 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS13",
                    description: Some(
                        "13 bits",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ptyp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NAND",
                    description: Some(
                        "NAND flash",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pwid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "External memory device width 8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "External memory device width 16 bits",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rpipe",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_DELAY",
                    description: Some(
                        "No clock cycle delay",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CLOCKS1",
                    description: Some(
                        "One clock cycle delay",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CLOCKS2",
                    description: Some(
                        "Two clock cycles delay",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sdclk",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "SDCLK clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SDCLK period = 2 x HCLK period",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "SDCLK period = 3 x HCLK period",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Waitcfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BEFORE_WAIT_STATE",
                    description: Some(
                        "NWAIT signal is active one data cycle before wait state",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DURING_WAIT_STATE",
                    description: Some(
                        "NWAIT signal is active during wait state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Waitpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "NWAIT active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "NWAIT active high",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
