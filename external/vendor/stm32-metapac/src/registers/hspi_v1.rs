
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hspi",
            extends: None,
            description: Some(
                "HSPI.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "HSPI control register.",
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
                        "HSPI device configuration register 1.",
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
                        "HSPI device configuration register 2.",
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
                        "HSPI device configuration register 3.",
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
                        "HSPI device configuration register 4.",
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
                    description: None,
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
                    description: None,
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
                        "HSPI data length register.",
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
                    description: None,
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
                    description: None,
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
                        "HSPI polling status mask register.",
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
                        "HSPI polling status match register.",
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
                        "HSPI polling interval register.",
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
                        "HSPI communication configuration register.",
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
                        "HSPI timing configuration register.",
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
                        "HSPI instruction register.",
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
                        "HSPI alternate bytes register.",
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
                        "HSPI low-power timeout register.",
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
                        "HSPI wrap communication configuration register.",
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
                        "HSPI wrap timing configuration register.",
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
                        "HSPI wrap instruction register.",
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
                        "HSPI wrap alternate bytes register.",
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
                        "HSPI write communication configuration register.",
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
                        "HSPI write timing configuration register.",
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
                        "HSPI write instruction register.",
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
                        "HSPI write alternate bytes register.",
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
                        "HSPI HyperBus latency configuration register.",
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
                        "HSPI full-cycle calibration configuration.",
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
                        "HSPI DLL master calibration configuration.",
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
                        "HSPI DLL slave output calibration configuration.",
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
                        "HSPI DLL slave input calibration configuration.",
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
                "HSPI alternate bytes register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "[31: 0]: Alternate bytes Optional data to be send to the external SPI device right after the address.",
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
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR[0] is forced to 1. Writes to this field are ignored when BUSY├é┬á=├é┬á1 or when FMODE = 11 (Memory-mapped mode).",
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
                "HSPI full-cycle calibration configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "[6: 0]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                        "[4: 0]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                        "Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case CALFCR and CALSR are updated with the values for the maximum delay.",
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
                "HSPI DLL master calibration configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "[6: 0]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                        "[4: 0]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                "HSPI DLL slave input calibration configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "[6: 0]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                        "[4: 0]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                "HSPI DLL slave output calibration configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fine",
                    description: Some(
                        "[6: 0]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                        "[4: 0]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet).",
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
                "HSPI communication configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate This bit sets the DTR mode for the instruction phase.",
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
                        "Instruction size This bit defines instruction size.",
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
                    name: "admode",
                    description: Some(
                        "Address mode This field defines the address phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate This bit sets the DTR mode for the address phase.",
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
                        "Address size This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode This field defines the alternate byte phase mode of operation. 100-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase. This field can be written only when BUSY├é┬á=├é┬á0.",
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
                        "Alternate bytes size This bit defines alternate bytes size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode This field defines the data phase mode of operation. 110-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "Data double transfer rate This bit sets the DTR mode for the data phase.",
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
                        "DQS enable This bit enables the data strobe management.",
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
                    name: "sioo",
                    description: Some(
                        "Send instruction only once mode This bit has no effect when IMODE├é┬á=├é┬á00 (see ).",
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
            name: "Cr",
            extends: None,
            description: Some(
                "HSPI control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active.",
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
                        "Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0.",
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
                        "DMA enable In Indirect mode, the DMA can be used to input or output data via DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation.",
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
                        "Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE[1:0]├é┬á=├é┬á11) is selected. This bit enables the timeout counter.",
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
                        "Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity.",
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
                    name: "fsel",
                    description: Some(
                        "Memory select This bit is the mirror of bit 30. Refer to the description of MSEL[1:0] above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0.",
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
                    name: "fthres",
                    description: Some(
                        "FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in SR, to be set. ... Note: If DMAEN├é┬á=├é┬á1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES[5:0] value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable This bit enables the transfer error interrupt.",
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
                        "Transfer complete interrupt enable This bit enables the transfer complete interrupt.",
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
                        "FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.",
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
                        "Status match interrupt enable This bit enables the status match interrupt.",
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
                        "Timeout interrupt enable This bit enables the timeout interrupt.",
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
                        "Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match.",
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
                    name: "pmm",
                    description: Some(
                        "Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode.",
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
                    name: "fmode",
                    description: Some(
                        "Functional mode This field defines the HSPI functional mode of operation. If DMAEN├é┬á=├é┬á1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE[1:0] value. If FMODE[1:0] and FTHRES[4:0] are wrongly updated while DMAEN├é┬á=├é┬á1, the DMA request signal automatically goes to inactive state.",
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
                Field {
                    name: "msel",
                    description: Some(
                        "Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in single├ó\u{80}\u{91}memory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO[7:0] 1x: data exchanged over IO[15:8] These bits are ignored when in dual-octal configuration (data on 8 bits and DMM├é┬á=├é┬á1) or 16├ó\u{80}\u{91}bit configuration (data exchanged over IO[15:0]).",
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
            name: "Dcr1",
            extends: None,
            description: Some(
                "HSPI device configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckmode",
                    description: Some(
                        "Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCS├é┬á=├é┬á1).",
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
                    name: "frck",
                    description: Some(
                        "Free running clock This bit configures the free running clock.",
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
                    name: "dlybyp",
                    description: Some(
                        "Delay block bypass.",
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
                    name: "csht",
                    description: Some(
                        "Chip-select high time CSHT├é┬á+├é┬á1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT[5:3] are reserved and the number of cycles is limited to eight (refer to implementation).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "devsize",
                    description: Some(
                        "Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2[DEVSIZE+1]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4├é┬áGbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256├é┬áMbytes. In Regular-command mode, if DMM├é┬á=├é┬á1, DEVSIZE[4:0] indicates the total capacity of the two devices together.",
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
                        "Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP[2:0] for memories different from Micron. Others: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcr2",
            extends: None,
            description: Some(
                "HSPI device configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prescaler",
                    description: Some(
                        "Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (value├é┬á+├é┬á1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50├é┬á%. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case CALOSR or CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.",
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
                        "Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the WPIR register. 110-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcr3",
            extends: None,
            description: Some(
                "HSPI device configuration register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maxtran",
                    description: Some(
                        "Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes.",
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
                    name: "csbound",
                    description: Some(
                        "CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes.",
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
            name: "Dcr4",
            extends: None,
            description: Some(
                "HSPI device configuration register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refresh",
                    description: Some(
                        "Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles.",
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
            name: "Dlr",
            extends: None,
            description: Some(
                "HSPI data length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dl",
                    description: Some(
                        "[31: 0]: Data length Number of data to be retrieved (value+1) in Indirect and Status-polling modes. A value not greater than three (indicating 4 bytes) must be used for status polling-mode. All 1's in Indirect mode means undefined length, where HSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE├é┬á=├é┬á0x1F. DL[0] is stuck at 1 in Dual-memory mode (DMM├é┬á=├é┬á1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect when in Memory-mapped mode.",
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
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "[31: 0]: Data Data to be sent/received to/from the external SPI device In Indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In Indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY├é┬á=├é┬á1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In Automatic-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In Indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in Indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2├é┬ábytes, and a word read 4├é┬ábytes. Accesses in Indirect mode must be aligned to the bottom of this register: A byte read must read DATA[7:0] and a half-word read must read DATA[15:0].",
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
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctef",
                    description: Some(
                        "Clear transfer error flag Writing 1 clears the TEF flag in the SR register.",
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
                        "Clear transfer complete flag Writing 1 clears the TCF flag in the SR register.",
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
                        "Clear status match flag Writing 1 clears the SMF flag in the SR register.",
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
                        "Clear timeout flag Writing 1 clears the TOF flag in the SR register.",
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
                "HSPI HyperBus latency configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lm",
                    description: Some(
                        "Latency mode This bit selects the Latency mode.",
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
                    name: "wzl",
                    description: Some(
                        "Write zero latency This bit enables zero latency on write operations.",
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
                    name: "tacc",
                    description: Some(
                        "[7: 0]: Access time Device access time expressed in number of communication clock cycles.",
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
                        "Read write recovery time Device read write recovery time expressed in number of communication clock cycles.",
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
                "HSPI instruction register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "Instruction Instruction to be sent to the external SPI device.",
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
                "HSPI low-power timeout register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timeout",
                    description: Some(
                        "[15: 0]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.",
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
                "HSPI polling interval register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interval",
                    description: Some(
                        "[15: 0]: Polling interval Number of CLK cycle between a read during the automatic-polling phases.",
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
                "HSPI polling status match register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "[31: 0]: Status match Value to be compared with the masked status register to get a match.",
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
                "HSPI polling status mask register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask",
                    description: Some(
                        "Status mask Mask to be applied to the status bytes received in Polling mode For bit n:.",
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
            name: "Sr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "tef",
                    description: Some(
                        "Transfer error flag This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF.",
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
                        "Transfer complete flag This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.",
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
                        "FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read.",
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
                        "Status match flag This bit is set in Automatic-polling mode when the unmasked received data matches the corresponding bits in the match register (PSMAR). It is cleared by writing 1 to CSMF.",
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
                        "Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF.",
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
                        "Busy This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty.",
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
                        "FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL├é┬á=├é┬á0 when the FIFO is empty, and 64 when it is full. In Automatic-status polling mode, FLEVEL is zero.",
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
                "HSPI timing configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).",
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
                        "Delay hold quarter cycle.",
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
                        "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT├é┬á=├é┬á0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1.).",
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
                "HSPI write alternate bytes register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "[31: 0]: Alternate bytes Optional data to be sent to the external SPI device right after the address.",
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
                "HSPI write communication configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate This bit sets the DTR mode for the instruction phase.",
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
                        "Instruction size This bit defines instruction size:.",
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
                    name: "admode",
                    description: Some(
                        "Address mode This field defines the address phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate This bit sets the DTR mode for the address phase.",
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
                        "Address size This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase.",
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
                        "Alternate bytes size This field defines alternate bytes size:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode This field defines the data phase mode of operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "data double transfer rate This bit sets the DTR mode for the data phase.",
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
                        "DQS enable This bit enables the data strobe management.",
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
                "HSPI write instruction register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "Instruction Instruction to be sent to the external SPI device.",
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
                "HSPI wrap alternate bytes register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alternate",
                    description: Some(
                        "[31: 0]: Alternate bytes Optional data to be sent to the external SPI device right after the address.",
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
                "HSPI wrap communication configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imode",
                    description: Some(
                        "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idtr",
                    description: Some(
                        "Instruction double transfer rate This bit sets the DTR mode for the instruction phase.",
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
                        "Instruction size This field defines instruction size.",
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
                    name: "admode",
                    description: Some(
                        "Address mode This field defines the address phase mode of operation. 101-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addtr",
                    description: Some(
                        "Address double transfer rate This bit sets the DTR mode for the address phase.",
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
                        "Address size This field defines address size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abmode",
                    description: Some(
                        "Alternate-byte mode This field defines the alternate byte phase mode of operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abdtr",
                    description: Some(
                        "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase.",
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
                        "Alternate bytes size This bit defines alternate bytes size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmode",
                    description: Some(
                        "Data mode This field defines the data phase mode of operation. 101; Data on 16 lines 110-111: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddtr",
                    description: Some(
                        "Data double transfer rate This bit sets the DTR mode for the data phase.",
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
                        "DQS enable This bit enables the data strobe management.",
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
                "HSPI wrap instruction register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instruction",
                    description: Some(
                        "[31: 0]: Instruction Instruction to be sent to the external SPI device.",
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
                "HSPI wrap timing configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.",
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
                        "Delay hold quarter cycle Add a quarter cycle delay on the outputs in DTR communication to match hold requirement.",
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
                        "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1).",
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
                "HSPI write timing configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcyc",
                    description: Some(
                        "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated.",
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
    enums: &[],
};
