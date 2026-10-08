
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Xspi",
            extends: None,
            description: Some(
                "XSPI register block",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "XSPI control register",
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
                    name: "dcr1",
                    description: Some(
                        "XSPI device configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr2",
                    description: Some(
                        "XSPI device configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr3",
                    description: Some(
                        "XSPI device configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr4",
                    description: Some(
                        "XSPI device configuration register 4",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "XSPI status register",
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
                    name: "fcr",
                    description: Some(
                        "XSPI flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlr",
                    description: Some(
                        "XSPI data length register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ar",
                    description: Some(
                        "XSPIaddress register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "XSPI data register",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                    name: "psmkr",
                    description: Some(
                        "XSPI polling status mask register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psmkr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psmar",
                    description: Some(
                        "XSPI polling status match register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pir",
                    description: Some(
                        "XSPI polling interval register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "XSPI communication configuration register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tcr",
                    description: Some(
                        "XSPI timing configuration register",
                    ),
                    array: None,
                    byte_offset: 0x108,
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
                    name: "ir",
                    description: Some(
                        "XSPI instruction register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abr",
                    description: Some(
                        "XSPI alternate bytes register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Abr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lptr",
                    description: Some(
                        "XSPI low-power timeout register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lptr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpccr",
                    description: Some(
                        "XSPI wrap communication configuration register",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wptcr",
                    description: Some(
                        "XSPI wrap timing configuration register",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wptcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpir",
                    description: Some(
                        "XSPI wrap instruction register",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpabr",
                    description: Some(
                        "XSPI wrap alternate byte register",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpabr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wccr",
                    description: Some(
                        "XSPI write communication configuration register",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wtcr",
                    description: Some(
                        "XSPI write timing configuration register",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wtcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wir",
                    description: Some(
                        "XSPI write instruction register",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wabr",
                    description: Some(
                        "XSPI write alternate byte register",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wabr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hlcr",
                    description: Some(
                        "XSPI HyperBus latency configuration register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hlcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfcr",
                    description: Some(
                        "XSPI full-cycle calibration configuration",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calmr",
                    description: Some(
                        "XSPI DLL master calibration configuration",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calsor",
                    description: Some(
                        "XSPI DLL slave output calibration configuration",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calsor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calsir",
                    description: Some(
                        "XSPI DLL slave input calibration configuration",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calsir",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Abr",
            extends: None,
            description: Some(
                "XSPI alternate bytes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "Alternate bytes\r Optional data to be sent to the external SPI device right after the address.",
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
            name: "Ar",
            extends: None,
            description: Some(
                "XSPIaddress register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "Address\r Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR[0] is forced to 0. \r Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (memory-mapped mode).\r Some memory specifications consider that each address corresponds to a 16-bit value. XSPI considers that each address corresponds to an 8-bit value. So the software needs to multiple the address by two when accessing the memory registers.",
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
            name: "Calfcr",
            extends: None,
            description: Some(
                "XSPI full-cycle calibration configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "Fine calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                    name: "coarse",
                    description: Some(
                        "Coarse calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calmax",
                    description: Some(
                        "Max value\r This bit gets set when the memory-clock period is outside the range of DLL master, in which case XSPI_CALFCR and XSPI_CALSR are updated with the values for the maximum delay.",
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
            name: "Calmr",
            extends: None,
            description: Some(
                "XSPI DLL master calibration configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "Fine calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                    name: "coarse",
                    description: Some(
                        "Coarse calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calsir",
            extends: None,
            description: Some(
                "XSPI DLL slave input calibration configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "Fine calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                    name: "coarse",
                    description: Some(
                        "Coarse calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calsor",
            extends: None,
            description: Some(
                "XSPI DLL slave output calibration configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "Fine calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                    name: "coarse",
                    description: Some(
                        "Coarse calibration\r The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "XSPI communication configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode\r This field defines the instruction phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CcrImode",
                    ),
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate\r This bit sets the DTR mode for the instruction phase.",
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
                    name: "isize",
                    description: Some(
                        "Instruction size\r This bit defines instruction size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "CcrIsize",
                    ),
                },
                Field {
                    name: "admode",
                    description: Some(
                        "Address mode\r This field defines the address phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CcrAdmode",
                    ),
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate\r This bit sets the DTR mode for the address phase.",
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
                    name: "adsize",
                    description: Some(
                        "Address size\r This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "CcrAdsize",
                    ),
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode\r This field defines the alternate byte phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CcrAbmode",
                    ),
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double transfer rate\r This bit sets the DTR mode for the alternate bytes phase.\r Note: This field can be written only when BUSY = 0.",
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
                    name: "absize",
                    description: Some(
                        "Alternate bytes size\r This bit defines alternate bytes size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "CcrAbsize",
                    ),
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode\r This field defines the data phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CcrDmode",
                    ),
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "Data double transfer rate\r This bit sets the DTR mode for the data phase.",
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
                    name: "dqse",
                    description: Some(
                        "DQS enable\r This bit enables the data strobe management.",
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
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "XSPI control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable\r This bit enables the XSPI.\r The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation.\r Note: In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.",
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
                    name: "abort",
                    description: Some(
                        "Abort request\r This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer.\r Note: This bit is always read as 0.",
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
                    name: "dmaen",
                    description: Some(
                        "DMA enable\r In indirect mode, the DMA can be used to input or output data via XSPI_DR. DMA transfers are initiated when FTF is set. \r Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.",
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
                    name: "tcen",
                    description: Some(
                        "Timeout counter enable\r This bit is valid only when the memory-mapped mode (FMODE[1:0] = 11) is selected. This bit enables the timeout counter.\r Note: This bit can be modified only when BUSY = 0.",
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
                    name: "dmm",
                    description: Some(
                        "Dual-memory configuration\r This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity\r Note: This bit can be modified only when BUSY = 0.",
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
                    name: "fthres",
                    description: Some(
                        "FIFO threshold level\r This field defines, in indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in XSPI_SR, to be set.\r ...\r Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES[5:0] value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Fthres",
                    ),
                },
                Field {
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable\r This bit enables the transfer error interrupt.",
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
                    name: "tcie",
                    description: Some(
                        "Transfer complete interrupt enable\r This bit enables the transfer complete interrupt.",
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
                    name: "ftie",
                    description: Some(
                        "FIFO threshold interrupt enable\r This bit enables the FIFO threshold interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smie",
                    description: Some(
                        "Status match interrupt enable\r This bit enables the status match interrupt.",
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
                    name: "toie",
                    description: Some(
                        "Timeout interrupt enable\r This bit enables the timeout interrupt.",
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
                    name: "apms",
                    description: Some(
                        "Automatic status-polling mode stop\r This bit determines if the automatic status-polling is stopped after a match.\r Note: This bit can be modified only when BUSY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Apms",
                    ),
                },
                Field {
                    name: "pmm",
                    description: Some(
                        "Polling match mode\r This bit indicates which method must be used to determine a match during the automatic status-polling mode.\r Note: This bit can be modified only when BUSY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pmm",
                    ),
                },
                Field {
                    name: "cssel",
                    description: Some(
                        "chip select selection\r This bit indicates if the XSPI must activate NCS1 or NCS2.\r Note: This bit can be modified only when BUSY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cssel",
                    ),
                },
                Field {
                    name: "fmode",
                    description: Some(
                        "Functional mode\r This field defines the XSPI functional mode of operation.\r If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE[1:0] value. If FMODE[1:0] and FTHRES[4:0] are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state.\r Note: This bitfield can be modified only when BUSY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fmode",
                    ),
                },
                Field {
                    name: "msel",
                    description: Some(
                        "Flash select",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Msel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dcr1",
            extends: None,
            description: Some(
                "XSPI device configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckmode",
                    description: Some(
                        "clock mode 0/mode 3\r This bit indicates the level taken by the CLK between commands (when NCS = 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckmode",
                    ),
                },
                Field {
                    name: "frck",
                    description: Some(
                        "Free running clock\r This bit configures the free running clock.",
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
                    name: "csht",
                    description: Some(
                        "Chip-select high time\r CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device.\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Csht",
                    ),
                },
                Field {
                    name: "devsize",
                    description: Some(
                        "Device size\r This field defines the size of the external device using the following formula:\r Number of bytes in device = 2<sup>[DEVSIZE+1]</sup>.\r DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes.\r In regular-command protocol, if DMM = 1, DEVSIZE[4:0] indicates the total capacity of the two devices together.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtyp",
                    description: Some(
                        "Memory type\r This bit indicates the type of memory to be supported.\r Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP[2:0] for memories different from Micron.\r Others: Reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dcr2",
            extends: None,
            description: Some(
                "XSPI device configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prescaler",
                    description: Some(
                        "Clock prescaler\r This field defines the scaler factor for generating the CLK based on the kernel clock (value + 1). \r ...\r For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low one cycle longer than it stays high.\r Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case XSPI_CALOSR or XSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.",
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
                    name: "wrapsize",
                    description: Some(
                        "Wrap size\r This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in XSPI_WPIR.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Wrapsize",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dcr3",
            extends: None,
            description: Some(
                "XSPI device configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maxtran",
                    description: Some(
                        "Maximum transfer\r This field enables the communication regulation feature.\r The NCS is released every MAXTRAN+1 clock cycles when the other XSPI request the access to the bus.\r Others: maximum communication is set to MAXTRAN + 1 bytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Maxtran",
                    ),
                },
                Field {
                    name: "csbound",
                    description: Some(
                        "NCS boundary\r This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended.\r The NCS is released on each boundary of 2<sup>CSBOUND</sup> bytes.\r Others: NCS boundary set to 2<sup>CSBOUND</sup> bytes",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Csbound",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dcr4",
            extends: None,
            description: Some(
                "XSPI device configuration register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refresh",
                    description: Some(
                        "Refresh rate\r This field enables the refresh rate feature.\r The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. \r Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single-, dual- or quad-SPI mode, because the byte transmission must be completed.\r Others: maximum communication length is set to REFRESH + 1 clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Refresh",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dlr",
            extends: None,
            description: Some(
                "XSPI data length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dl",
                    description: Some(
                        "Data length",
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
            name: "Dr",
            extends: None,
            description: Some(
                "XSPI data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Data\r Data to be sent/received to/from the external SPI device\r In indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written.\r In indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY = 1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first.\r In automatic status-polling mode, this register contains the last data read from the external device (without masking).\r Word, half-word, and byte accesses to this register are supported. In indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes.\r Similarly, in indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2 bytes, and a word read 4 bytes. Accesses in indirect mode must be aligned to the bottom of this register: A byte read must read DATA[7:0] and a half-word read must read DATA[15:0].",
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
            name: "Fcr",
            extends: None,
            description: Some(
                "XSPI flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctef",
                    description: Some(
                        "Clear transfer error flag\r Writing 1 clears the TEF flag in the XSPI_SR register.",
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
                    name: "ctcf",
                    description: Some(
                        "Clear transfer complete flag\r Writing 1 clears the TCF flag in the XSPI_SR register.",
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
                    name: "csmf",
                    description: Some(
                        "Clear status match flag\r Writing 1 clears the SMF flag in the XSPI_SR register.",
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
                    name: "ctof",
                    description: Some(
                        "Clear timeout flag\r Writing 1 clears the TOF flag in the XSPI_SR register.",
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
            name: "Hlcr",
            extends: None,
            description: Some(
                "XSPI HyperBus latency configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lm",
                    description: Some(
                        "Latency mode\r This bit selects the Latency mode.\r Note: This bit must be set when using the dual-octal HyperBus configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lm",
                    ),
                },
                Field {
                    name: "wzl",
                    description: Some(
                        "Write zero latency\r This bit enables zero latency on write operations.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wzl",
                    ),
                },
                Field {
                    name: "tacc",
                    description: Some(
                        "Access time\r Device access time expressed in number of communication clock cycles",
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
                    name: "trwr",
                    description: Some(
                        "Read write recovery time\r Device read write recovery time expressed in number of communication clock cycles",
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
            ],
        },
        FieldSet {
            name: "Ir",
            extends: None,
            description: Some(
                "XSPI instruction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "Instruction\r Instruction to be sent to the external SPI device",
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
            name: "Lptr",
            extends: None,
            description: Some(
                "XSPI low-power timeout register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timeout",
                    description: Some(
                        "Timeout period\r After each access in memory-mapped mode, the XSPI prefetches the subsequent bytes and hold them in the FIFO.\r This field indicates how many CLK cycles the XSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state.",
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
            name: "Pir",
            extends: None,
            description: Some(
                "XSPI polling interval register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interval",
                    description: Some(
                        "Polling interval\r Number of CLK cycle between a read during the automatic status-polling phases",
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
            name: "Psmar",
            extends: None,
            description: Some(
                "XSPI polling status match register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "Status match\r Value to be compared with the masked status register to get a match",
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
            name: "Psmkr",
            extends: None,
            description: Some(
                "XSPI polling status mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask",
                    description: Some(
                        "Status mask\r Mask to be applied to the status bytes received in automatic status-polling mode\r For bit n:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Mask",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "XSPI status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tef",
                    description: Some(
                        "Transfer error flag\r This bit is set in indirect mode when an invalid address is being accessed in indirect mode.\r It is cleared by writing 1 to CTEF.",
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
                        "Transfer complete flag\r This bit is set in indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.",
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
                    name: "ftf",
                    description: Some(
                        "FIFO threshold flag\r In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete.\r It is cleared automatically as soon as the threshold condition is no longer true.\r In automatic status-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read.",
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
                    name: "smf",
                    description: Some(
                        "Status match flag\r This bit is set in automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (XSPI_PSMAR). \r It is cleared by writing 1 to CSMF.",
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
                    name: "tof",
                    description: Some(
                        "Timeout flag\r This bit is set when timeout occurs. It is cleared by writing 1 to CTOF.",
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
                    name: "busy",
                    description: Some(
                        "Busy\r This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty.",
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
                    name: "flevel",
                    description: Some(
                        "FIFO level\r This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 64 when it is full.\r In automatic-status polling mode, FLEVEL is zero.",
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
            ],
        },
        FieldSet {
            name: "Tcr",
            extends: None,
            description: Some(
                "XSPI timing configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles\r This field defines the duration of the dummy phase.\r In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).",
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
                    name: "dhqc",
                    description: Some(
                        "Delay hold quarter cycle",
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
                    name: "sshift",
                    description: Some(
                        "Sample shift\r By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device.\r This bit allows the data to be sampled later in order to consider the external signal delays.\r The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)",
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
            ],
        },
        FieldSet {
            name: "Wabr",
            extends: None,
            description: Some(
                "XSPI write alternate byte register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "Alternate bytes\r Optional data to be sent to the external SPI device right after the address",
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
            name: "Wccr",
            extends: None,
            description: Some(
                "XSPI write communication configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode\r This field defines the instruction phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WccrImode",
                    ),
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate\r This bit sets the DTR mode for the instruction phase.",
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
                    name: "isize",
                    description: Some(
                        "Instruction size\r This bit defines instruction size:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WccrIsize",
                    ),
                },
                Field {
                    name: "admode",
                    description: Some(
                        "Address mode\r This field defines the address phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WccrAdmode",
                    ),
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate\r This bit sets the DTR mode for the address phase.",
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
                    name: "adsize",
                    description: Some(
                        "Address size\r This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WccrAdsize",
                    ),
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode\r This field defines the alternate-byte phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WccrAbmode",
                    ),
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double-transfer rate\r This bit sets the DTR mode for the alternate-bytes phase.",
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
                    name: "absize",
                    description: Some(
                        "Alternate bytes size\r This field defines alternate bytes size:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WccrAbsize",
                    ),
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode\r This field defines the data phase mode of operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WccrDmode",
                    ),
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "data double transfer rate\r This bit sets the DTR mode for the data phase.",
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
                    name: "dqse",
                    description: Some(
                        "DQS enable\r This bit enables the data strobe management.",
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
            ],
        },
        FieldSet {
            name: "Wir",
            extends: None,
            description: Some(
                "XSPI write instruction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "Instruction\r Instruction to be sent to the external SPI device",
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
            name: "Wpabr",
            extends: None,
            description: Some(
                "XSPI wrap alternate byte register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "Alternate bytes\r Optional data to be sent to the external SPI device right after the address",
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
            name: "Wpccr",
            extends: None,
            description: Some(
                "XSPI wrap communication configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode\r This field defines the instruction phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WpccrImode",
                    ),
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate\r This bit sets the DTR mode for the instruction phase.",
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
                    name: "isize",
                    description: Some(
                        "Instruction size\r This field defines instruction size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WpccrIsize",
                    ),
                },
                Field {
                    name: "admode",
                    description: Some(
                        "Address mode\r This field defines the address phase mode of operation.\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WpccrAdmode",
                    ),
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate\r This bit sets the DTR mode for the address phase.",
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
                    name: "adsize",
                    description: Some(
                        "Address size\r This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WpccrAdsize",
                    ),
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode\r This field defines the alternate byte phase mode of operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WpccrAbmode",
                    ),
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double transfer rate\r This bit sets the DTR mode for the alternate bytes phase.",
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
                    name: "absize",
                    description: Some(
                        "Alternate bytes size\r This bit defines alternate bytes size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WpccrAbsize",
                    ),
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode\r This field defines the data phase mode of operation.\r 101; data on 16 lines\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WpccrDmode",
                    ),
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "Data double transfer rate\r This bit sets the DTR mode for the data phase.",
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
                    name: "dqse",
                    description: Some(
                        "DQS enable\r This bit enables the data strobe management.",
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
            ],
        },
        FieldSet {
            name: "Wpir",
            extends: None,
            description: Some(
                "XSPI wrap instruction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "Instruction\r Instruction to be sent to the external SPI device",
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
            name: "Wptcr",
            extends: None,
            description: Some(
                "XSPI wrap timing configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles\r This field defines the duration of the dummy phase.\r In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).",
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
                    name: "dhqc",
                    description: Some(
                        "Delay hold quarter cycle\r Add a quarter cycle delay on the outputs in DTR communication to match hold requirement.",
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
                    name: "sshift",
                    description: Some(
                        "Sample shift\r By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device.\r This bit allows the data to be sampled later in order to consider the external signal delays.\r The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR = 1).",
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
            ],
        },
        FieldSet {
            name: "Wtcr",
            extends: None,
            description: Some(
                "XSPI write timing configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles\r This field defines the duration of the dummy phase.\r In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Apms",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Automatic status-polling mode is stopped only by abort or by disabling the XSPI.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Automatic status-polling mode stops as soon as there is a match.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CcrAbmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "alternate bytes on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "alternate bytes on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "alternate bytes on four lines",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcrAbsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit alternate bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit alternate bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit alternate bytes",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcrAdmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "address on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "address on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "address on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "address on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "CcrAdsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit address",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit address",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit address",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcrDmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no data",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "data on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "data on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "data on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "data on eight lines",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "data on 16 lines",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "CcrImode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "instruction on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "instruction on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "instruction on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "instruction on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "CcrIsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit instruction",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit instruction",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit instruction",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ckmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "CLK must stay low while NCS is high (chip-select released), referred to as clock mode 0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "CLK must stay high while NCS is high (chip-select released), referred to as clock mode 3.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Csbound",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "NCS boundary disabled",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Csht",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "NCS stays high for at least 1 cycle between external device commands.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "NCS stays high for at least 2 cycles between external device commands.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X3F",
                    description: Some(
                        "NCS stays high for at least 64 cycles between external device commands.",
                    ),
                    value: 63,
                },
            ],
        },
        Enum {
            name: "Cssel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "NCS1 active",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "NCS2 active",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "indirect-write mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "indirect-read mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "automatic status-polling mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "memory-mapped mode",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Fthres",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "FTF is set if there are one or more free bytes available to be written to in the FIFO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "FTF is set if there are two or more free bytes available to be written to in the FIFO",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X3F",
                    description: Some(
                        "FTF is set if there are 64 free bytes available to be written to in the FIFO",
                    ),
                    value: 63,
                },
            ],
        },
        Enum {
            name: "Lm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Variable initial latency",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Fixed latency",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mask",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "bit n of the data received in automatic status-polling mode is masked and its value is not considered in the matching logic.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "bit n of the data received in automatic status-polling mode is unmasked and its value is considered in the matching logic.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Maxtran",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "maximum communication disabled",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Msel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "data exchanged over IO[3:0]",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "data exchanged over IO[7:4]",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "data exchanged over IO[11:8]",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "data exchanged over IO[15:12]",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mtyp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Standard mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes with dedicated address mapping.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "HyperBus memory mode, the protocol follows the HyperBus<sup> </sup>specification. 8-data-bit DTR mode must be selected.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or indirect read/write modes must be used.",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Pmm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Refresh",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "refresh disabled",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "WccrAbmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "alternate bytes on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "alternate bytes on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "alternate bytes on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "alternate bytes on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WccrAbsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit alternate bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit alternate bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit alternate bytes",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WccrAdmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "address on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "address on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "address on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "address on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WccrAdsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit address",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit address",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit address",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WccrDmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no data",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "data on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "data on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "data on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "data on eight lines",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "Data on 16 lines",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "DATA reserved",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "WccrImode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "instruction on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "instruction on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "instruction on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "instruction on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WccrIsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit instruction",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit instruction",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit instruction",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WpccrAbmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "alternate bytes on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "alternate bytes on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "alternate bytes on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "alternate bytes on eight lines",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "alternate bytes on 16 lines",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "DATA reserved",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "WpccrAbsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit alternate bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit alternate bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit alternate bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit alternate bytes",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WpccrAdmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "address on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "address on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "address on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "address on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WpccrAdsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit address",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit address",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit address",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WpccrDmode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no data",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "data on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "data on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "data on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "data on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WpccrImode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "instruction on a single line",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "instruction on two lines",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "instruction on four lines",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "instruction on eight lines",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "WpccrIsize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "8-bit instruction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "16-bit instruction",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "24-bit instruction",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "32-bit instruction",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Wrapsize",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "wrapped reads are not supported by the memory.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "external memory supports wrap size of 16 bytes.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "external memory supports wrap size of 32 bytes.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "external memory supports wrap size of 64 bytes.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "external memory supports wrap size of 128 bytes.",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Wzl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "latency on write accesses",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "no latency on write accesses",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
