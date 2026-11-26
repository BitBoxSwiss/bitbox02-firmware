
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dma2d",
            extends: None,
            description: Some(
                "DMA2D",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "DMA2D control register",
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
                    name: "isr",
                    description: Some(
                        "DMA2D interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                        "DMA2D interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgmar",
                    description: Some(
                        "DMA2D foreground memory address register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgor",
                    description: Some(
                        "DMA2D foreground offset register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgmar",
                    description: Some(
                        "DMA2D background memory address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgor",
                    description: Some(
                        "DMA2D background offset register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgpfccr",
                    description: Some(
                        "DMA2D foreground PFC (pixel format converter) control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgpfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgcolr",
                    description: Some(
                        "DMA2D foreground color register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgcolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgpfccr",
                    description: Some(
                        "DMA2D background PFC (pixel format converter) control register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgpfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgcolr",
                    description: Some(
                        "DMA2D background color register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgcolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgcmar",
                    description: Some(
                        "DMA2D foreground CLUT memory address register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgcmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgcmar",
                    description: Some(
                        "DMA2D background CLUT memory address register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgcmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opfccr",
                    description: Some(
                        "DMA2D output PFC (pixel format converter) control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ocolr",
                    description: Some(
                        "DMA2D output color register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ocolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "omar",
                    description: Some(
                        "DMA2D output memory address register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Omar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oor",
                    description: Some(
                        "DMA2D output offset register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nlr",
                    description: Some(
                        "DMA2D number of line register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lwr",
                    description: Some(
                        "DMA2D line watermark register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lwr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "amtcr",
                    description: Some(
                        "DMA2D AXI master timer configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Amtcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Amtcr",
            extends: None,
            description: Some(
                "DMA2D AXI master timer configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable. Enables the dead time functionality.",
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
                    name: "dt",
                    description: Some(
                        "Dead time. Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.",
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
            ],
        },
        FieldSet {
            name: "Bgcmar",
            extends: None,
            description: Some(
                "DMA2D background CLUT memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address. Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned.",
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
            name: "Bgcolr",
            extends: None,
            description: Some(
                "DMA2D background color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue value. These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
                    name: "green",
                    description: Some(
                        "Green value. These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
                    name: "red",
                    description: Some(
                        "Red value. These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Bgmar",
            extends: None,
            description: Some(
                "DMA2D background memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address. Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned.",
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
            name: "Bgor",
            extends: None,
            description: Some(
                "DMA2D background offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset used for the background image (expressed in pixels (default) or bytes as per LOM in CR). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.",
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
            name: "Bgpfccr",
            extends: None,
            description: Some(
                "DMA2D background PFC (pixel format converter) control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode. These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "BgpfccrCm",
                    ),
                },
                Field {
                    name: "ccm",
                    description: Some(
                        "CLUT (color lookup table) color mode. These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrCcm",
                    ),
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start. This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrStart",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "CLUT size. These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS[7:0] + 1.",
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
                    name: "am",
                    description: Some(
                        "Alpha mode. These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "BgpfccrAm",
                    ),
                },
                Field {
                    name: "ai",
                    description: Some(
                        "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrAi",
                    ),
                },
                Field {
                    name: "rbs",
                    description: Some(
                        "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrRbs",
                    ),
                },
                Field {
                    name: "alpha",
                    description: Some(
                        "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM[1: 0]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "DMA2D control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Start. This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CrStart",
                    ),
                },
                Field {
                    name: "susp",
                    description: Some(
                        "Suspend. This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.",
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
                    name: "abort",
                    description: Some(
                        "Abort. This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Abort",
                    ),
                },
                Field {
                    name: "lom",
                    description: Some(
                        "Line offset mode. This bit configures how the line offset is expressed (in pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lom",
                    ),
                },
                Field {
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable. This bit is set and cleared by software.",
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
                    name: "tcie",
                    description: Some(
                        "Transfer complete interrupt enable. This bit is set and cleared by software.",
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
                    name: "twie",
                    description: Some(
                        "Transfer watermark interrupt enable. This bit is set and cleared by software.",
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
                    name: "caeie",
                    description: Some(
                        "CLUT access error interrupt enable. This bit is set and cleared by software.",
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
                    name: "ctcie",
                    description: Some(
                        "CLUT transfer complete interrupt enable. This bit is set and cleared by software.",
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
                    name: "ceie",
                    description: Some(
                        "Configuration error interrupt enable. This bit is set and cleared by software.",
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
                    name: "mode",
                    description: Some(
                        "DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fgcmar",
            extends: None,
            description: Some(
                "DMA2D foreground CLUT memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory Address. Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.",
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
            name: "Fgcolr",
            extends: None,
            description: Some(
                "DMA2D foreground color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue Value. These bits define the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.",
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
                    name: "green",
                    description: Some(
                        "Green Value. These bits define the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.",
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
                    name: "red",
                    description: Some(
                        "Red Value. These bits define the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Fgmar",
            extends: None,
            description: Some(
                "DMA2D foreground memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address. Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned.",
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
            name: "Fgor",
            extends: None,
            description: Some(
                "DMA2D foreground offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset. Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.",
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
            name: "Fgpfccr",
            extends: None,
            description: Some(
                "DMA2D foreground PFC (pixel format converter) control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode. These bits define the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "FgpfccrCm",
                    ),
                },
                Field {
                    name: "ccm",
                    description: Some(
                        "CLUT color mode. This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrCcm",
                    ),
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start. This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrStart",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "CLUT size. These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS[7:0] + 1.",
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
                    name: "am",
                    description: Some(
                        "Alpha mode. These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "FgpfccrAm",
                    ),
                },
                Field {
                    name: "css",
                    description: Some(
                        "Chroma Sub-Sampling. These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ai",
                    description: Some(
                        "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrAi",
                    ),
                },
                Field {
                    name: "rbs",
                    description: Some(
                        "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrRbs",
                    ),
                },
                Field {
                    name: "alpha",
                    description: Some(
                        "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM[1:0] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.",
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
            name: "Ifcr",
            extends: None,
            description: Some(
                "DMA2D interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cteif",
                    description: Some(
                        "Clear transfer error interrupt flag. Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cteif",
                    ),
                },
                Field {
                    name: "ctcif",
                    description: Some(
                        "Clear transfer complete interrupt flag. Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ctcif",
                    ),
                },
                Field {
                    name: "ctwif",
                    description: Some(
                        "Clear transfer watermark interrupt flag. Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ctwif",
                    ),
                },
                Field {
                    name: "caecif",
                    description: Some(
                        "Clear CLUT access error interrupt flag. Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Caecif",
                    ),
                },
                Field {
                    name: "cctcif",
                    description: Some(
                        "Clear CLUT transfer complete interrupt flag. Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cctcif",
                    ),
                },
                Field {
                    name: "cceif",
                    description: Some(
                        "Clear configuration error interrupt flag. Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cceif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "DMA2D Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teif",
                    description: Some(
                        "Transfer error interrupt flag. This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading).",
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
                    name: "tcif",
                    description: Some(
                        "Transfer complete interrupt flag. This bit is set when a DMA2D transfer operation is complete (data transfer only).",
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
                    name: "twif",
                    description: Some(
                        "Transfer watermark interrupt flag. This bit is set when the last pixel of the watermarked line has been transferred.",
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
                    name: "caeif",
                    description: Some(
                        "CLUT access error interrupt flag. This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D.",
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
                    name: "ctcif",
                    description: Some(
                        "CLUT transfer complete interrupt flag. This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete.",
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
                    name: "ceif",
                    description: Some(
                        "Configuration error interrupt flag. This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed.",
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
            name: "Lwr",
            extends: None,
            description: Some(
                "DMA2D line watermark register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lw",
                    description: Some(
                        "Line watermark. These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Nlr",
            extends: None,
            description: Some(
                "DMA2D number of line register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nl",
                    description: Some(
                        "Number of lines. Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
                    name: "pl",
                    description: Some(
                        "Pixel per lines. Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ocolr",
            extends: None,
            description: Some(
                "DMA2D output color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "color",
                    description: Some(
                        "Color. Color in the format specified by color mode in OPFCCR (16, 24 or 32 bits). These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Omar",
            extends: None,
            description: Some(
                "DMA2D output memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory Address. Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.",
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
            name: "Oor",
            extends: None,
            description: Some(
                "DMA2D output offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset. Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.",
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
            name: "Opfccr",
            extends: None,
            description: Some(
                "DMA2D output PFC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode. These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "OpfccrCm",
                    ),
                },
                Field {
                    name: "sb",
                    description: Some(
                        "Swap Bytes",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sb",
                    ),
                },
                Field {
                    name: "ai",
                    description: Some(
                        "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OpfccrAi",
                    ),
                },
                Field {
                    name: "rbs",
                    description: Some(
                        "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OpfccrRbs",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Abort",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ABORT_REQUEST",
                    description: Some(
                        "Transfer abort requested",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrAi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR_ALPHA",
                    description: Some(
                        "Regular alpha",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED_ALPHA",
                    description: Some(
                        "Inverted alpha",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrAm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_MODIFY",
                    description: Some(
                        "No modification of alpha channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REPLACE",
                    description: Some(
                        "Replace with value in ALPHA[7:0]",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MULTIPLY",
                    description: Some(
                        "Multiply with value in ALPHA[7:0]",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "BgpfccrCcm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "CLUT color format ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "CLUT color format RGB888",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrCm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "Color mode ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "Color mode RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "Color mode RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "Color mode ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "Color mode ARGB4444",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "L8",
                    description: Some(
                        "Color mode L8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "AL44",
                    description: Some(
                        "Color mode AL44",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "AL88",
                    description: Some(
                        "Color mode AL88",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "L4",
                    description: Some(
                        "Color mode L4",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "A8",
                    description: Some(
                        "Color mode A8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "A4",
                    description: Some(
                        "Color mode A4",
                    ),
                    value: 10,
                },
            ],
        },
        Enum {
            name: "BgpfccrRbs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR",
                    description: Some(
                        "No red blue swap (RGB or ARGB)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SWAP",
                    description: Some(
                        "Red blue swap (BGR or ABGR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Start the automatic loading of the CLUT",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Caecif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CAEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cceif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cctcif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CTCIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Launch the DMA2D",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ctcif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TCIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cteif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ctwif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TWIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrAi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR_ALPHA",
                    description: Some(
                        "Regular alpha",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED_ALPHA",
                    description: Some(
                        "Inverted alpha",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrAm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_MODIFY",
                    description: Some(
                        "No modification of alpha channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REPLACE",
                    description: Some(
                        "Replace with value in ALPHA[7:0]",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MULTIPLY",
                    description: Some(
                        "Multiply with value in ALPHA[7:0]",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "FgpfccrCcm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "CLUT color format ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "CLUT color format RGB888",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrCm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "Color mode ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "Color mode RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "Color mode RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "Color mode ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "Color mode ARGB4444",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "L8",
                    description: Some(
                        "Color mode L8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "AL44",
                    description: Some(
                        "Color mode AL44",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "AL88",
                    description: Some(
                        "Color mode AL88",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "L4",
                    description: Some(
                        "Color mode L4",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "A8",
                    description: Some(
                        "Color mode A8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "A4",
                    description: Some(
                        "Color mode A4",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "YCB_CR",
                    description: Some(
                        "Color mode YCbCr",
                    ),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "FgpfccrRbs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR",
                    description: Some(
                        "No red blue swap (RGB or ARGB)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SWAP",
                    description: Some(
                        "red blue swap (BGR or ABGR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Start the automatic loading of the CLUT",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lom",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PIXELS",
                    description: Some(
                        "Line offsets expressed in pixels",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES",
                    description: Some(
                        "Line offsets expressed in bytes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MEMORY_TO_MEMORY",
                    description: Some(
                        "Memory-to-memory (FG fetch only)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFC",
                    description: Some(
                        "Memory-to-memory with PFC (pixel format converter) (FG fetch only with FG PFC active)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFCBLENDING",
                    description: Some(
                        "Memory-to-memory with blending (FG and BG fetch with PFC and blending)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "REGISTER_TO_MEMORY",
                    description: Some(
                        "Register-to-memory (no FG nor BG, only output stage active)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFCBLENDING_FIXED_COLOR_FG",
                    description: Some(
                        "Memory-to-memory with blending and fixed color FG (BG fetch only with FG and BG PFC active)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFCBLENDING_FIXED_COLOR_BG",
                    description: Some(
                        "Memory-to-memory with blending and fixed color BG (FG fetch only with FG and BG PFC active)",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "OpfccrAi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR_ALPHA",
                    description: Some(
                        "Regular alpha",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED_ALPHA",
                    description: Some(
                        "Inverted alpha",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OpfccrCm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "ARGB4444",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "OpfccrRbs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR",
                    description: Some(
                        "No red blue swap (RGB or ARGB)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SWAP",
                    description: Some(
                        "Red blue swap (BGR or ABGR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sb",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "REGULAR",
                    description: Some(
                        "Regular byte order",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SWAP_BYTES",
                    description: Some(
                        "Bytes are swapped two by two",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
