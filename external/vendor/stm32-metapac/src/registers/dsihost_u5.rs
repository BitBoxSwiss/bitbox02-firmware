
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dsihost",
            extends: None,
            description: Some(
                "DSI Host.",
            ),
            items: &[
                BlockItem {
                    name: "vr",
                    description: Some(
                        "DSI Host version register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "DSI Host control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "ccr",
                    description: Some(
                        "DSI Host clock control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "lvcidr",
                    description: Some(
                        "DSI Host LTDC VCID register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcolcr",
                    description: Some(
                        "DSI Host LTDC color coding register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcolcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpcr",
                    description: Some(
                        "DSI Host LTDC polarity configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmcr",
                    description: Some(
                        "DSI Host low-power mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcr",
                    description: Some(
                        "DSI Host protocol configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    name: "gvcidr",
                    description: Some(
                        "DSI Host generic VCID register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "DSI Host mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmcr",
                    description: Some(
                        "DSI Host video mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vpcr",
                    description: Some(
                        "DSI Host video packet configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vccr",
                    description: Some(
                        "DSI Host video chunks configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vnpcr",
                    description: Some(
                        "DSI Host video null packet configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vnpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhsacr",
                    description: Some(
                        "DSI Host video HSA configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhsacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhbpcr",
                    description: Some(
                        "DSI Host video HBP configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vlcr",
                    description: Some(
                        "DSI Host video line configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vlcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvsacr",
                    description: Some(
                        "DSI Host video VSA configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvsacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvbpcr",
                    description: Some(
                        "DSI Host video VBP configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvfpcr",
                    description: Some(
                        "DSI Host video VFP configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvfpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvacr",
                    description: Some(
                        "DSI Host video VA configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lccr",
                    description: Some(
                        "DSI Host LTDC command configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmcr",
                    description: Some(
                        "DSI Host command mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ghcr",
                    description: Some(
                        "DSI Host generic header configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ghcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpdr",
                    description: Some(
                        "DSI Host generic payload data register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpsr",
                    description: Some(
                        "DSI Host generic packet status register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr0",
                    description: Some(
                        "DSI Host timeout counter configuration register 0.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr1",
                    description: Some(
                        "DSI Host timeout counter configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr2",
                    description: Some(
                        "DSI Host timeout counter configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr3",
                    description: Some(
                        "DSI Host timeout counter configuration register 3.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr4",
                    description: Some(
                        "DSI Host timeout counter configuration register 4.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr5",
                    description: Some(
                        "DSI Host timeout counter configuration register 5.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clcr",
                    description: Some(
                        "DSI Host clock lane configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cltcr",
                    description: Some(
                        "DSI Host clock lane timer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dltcr",
                    description: Some(
                        "DSI Host data lane timer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pctlr",
                    description: Some(
                        "DSI Host PHY control register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pconfr",
                    description: Some(
                        "DSI Host PHY configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pconfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucr",
                    description: Some(
                        "DSI Host PHY ULPS control register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pttcr",
                    description: Some(
                        "DSI Host PHY TX triggers configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pttcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psr",
                    description: Some(
                        "DSI Host PHY status register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr0",
                    description: Some(
                        "DSI Host interrupt and status register 0.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr1",
                    description: Some(
                        "DSI Host interrupt and status register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier0",
                    description: Some(
                        "DSI Host interrupt enable register 0.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier1",
                    description: Some(
                        "DSI Host interrupt enable register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fir0",
                    description: Some(
                        "DSI Host force interrupt register 0.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fir0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fir1",
                    description: Some(
                        "DSI Host force interrupt register 1.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fir1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dltrcr",
                    description: Some(
                        "DSI Host data lane timer read configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dltrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vscr",
                    description: Some(
                        "DSI Host video shadow control register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcvcidr",
                    description: Some(
                        "DSI Host LTDC current VCID register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcccr",
                    description: Some(
                        "DSI Host LTDC current color coding register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmccr",
                    description: Some(
                        "DSI Host low-power mode current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmccr",
                    description: Some(
                        "DSI Host video mode current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vpccr",
                    description: Some(
                        "DSI Host video packet current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vcccr",
                    description: Some(
                        "DSI Host video chunks current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vcccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vnpccr",
                    description: Some(
                        "DSI Host video null packet current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vnpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhsaccr",
                    description: Some(
                        "DSI Host video HSA current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhsaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhbpccr",
                    description: Some(
                        "DSI Host video HBP current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhbpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vlccr",
                    description: Some(
                        "DSI Host video line current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vlccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvsaccr",
                    description: Some(
                        "DSI Host video VSA current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvsaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvbpccr",
                    description: Some(
                        "DSI Host video VBP current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvbpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvfpccr",
                    description: Some(
                        "DSI Host video VFP current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvfpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvaccr",
                    description: Some(
                        "DSI Host video VA current configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fbsr",
                    description: Some(
                        "DSI Host FIFO and buffer status register.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fbsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wcfgr",
                    description: Some(
                        "DSI Wrapper configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wcr",
                    description: Some(
                        "DSI Wrapper control register.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wier",
                    description: Some(
                        "DSI Wrapper interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wisr",
                    description: Some(
                        "DSI Wrapper interrupt and status register.",
                    ),
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wifcr",
                    description: Some(
                        "DSI Wrapper interrupt flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr0",
                    description: Some(
                        "DSI Wrapper PHY configuration register 0.",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpcr",
                    description: Some(
                        "DSI Wrapper regulator and PLL control register.",
                    ),
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bcfgr",
                    description: Some(
                        "DSI bias configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpcbcr",
                    description: Some(
                        "DSI D-PHY clock band control register.",
                    ),
                    array: None,
                    byte_offset: 0xc04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpcbcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpcsrcr",
                    description: Some(
                        "DSI D-PHY clock skew rate control register.",
                    ),
                    array: None,
                    byte_offset: 0xc34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpcsrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpdl0bcr",
                    description: Some(
                        "DSI D-PHY data lane 0 band control register.",
                    ),
                    array: None,
                    byte_offset: 0xc70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpdl0bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpdl0srcr",
                    description: Some(
                        "DSI D-PHY data lane 0 skew rate control register.",
                    ),
                    array: None,
                    byte_offset: 0xca0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpdl0srcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpdl1bcr",
                    description: Some(
                        "DSI D-PHY data lane 1 band control register.",
                    ),
                    array: None,
                    byte_offset: 0xd08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpdl1bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpdl1srcr",
                    description: Some(
                        "DSI D-PHY data lane 1 skew rate control register.",
                    ),
                    array: None,
                    byte_offset: 0xd38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dpdl1srcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcfgr",
            extends: None,
            description: Some(
                "DSI bias configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrup",
                    description: Some(
                        "Power-up This bit powers-up the reference bias for the MIPI D-PHY.",
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
            name: "Ccr",
            extends: None,
            description: Some(
                "DSI Host clock control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txeckdiv",
                    description: Some(
                        "TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation.",
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
                    name: "tockdiv",
                    description: Some(
                        "Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error.",
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
            name: "Clcr",
            extends: None,
            description: Some(
                "DSI Host clock lane configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpcc",
                    description: Some(
                        "D-PHY clock control This bit controls the D-PHY clock state:.",
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
                    name: "acr",
                    description: Some(
                        "Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.",
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
            ],
        },
        FieldSet {
            name: "Cltcr",
            extends: None,
            description: Some(
                "DSI Host clock lane timer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lp2hs_time",
                    description: Some(
                        "Low-power to high-speed time This field configures the maximum time that the D-PHY clock lane takes to go from lowâ\u{80}\u{91}power to high-speed transmission measured in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs2lp_time",
                    description: Some(
                        "High-speed to low-power time This field configures the maximum time that the D-PHY clock lane takes to go from highâ\u{80}\u{91}speed to low-power transmission measured in lane byte clock cycles.",
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
            name: "Cmcr",
            extends: None,
            description: Some(
                "DSI Host command mode configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teare",
                    description: Some(
                        "Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:.",
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
                    name: "are",
                    description: Some(
                        "Acknowledge request enable This bit enables the acknowledge request after each packet transmission:.",
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
                    name: "gsw0tx",
                    description: Some(
                        "Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:.",
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
                    name: "gsw1tx",
                    description: Some(
                        "Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:.",
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
                    name: "gsw2tx",
                    description: Some(
                        "Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:.",
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
                    name: "gsr0tx",
                    description: Some(
                        "Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:.",
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
                    name: "gsr1tx",
                    description: Some(
                        "Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:.",
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
                    name: "gsr2tx",
                    description: Some(
                        "Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:.",
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
                    name: "glwtx",
                    description: Some(
                        "Generic long write transmission This bit configures the generic long write packet command transmission type :.",
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
                    name: "dsw0tx",
                    description: Some(
                        "DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:.",
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
                    name: "dsw1tx",
                    description: Some(
                        "DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:.",
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
                    name: "dsr0tx",
                    description: Some(
                        "DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:.",
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
                    name: "dlwtx",
                    description: Some(
                        "DCS long write transmission This bit configures the DCS long write packet command transmission type:.",
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
                    name: "mrdps",
                    description: Some(
                        "Maximum read packet size This bit configures the maximum read packet size command transmission type:.",
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
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "DSI Host control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable This bit configures the DSI Host in either power-up mode or to reset.",
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
            name: "Dltcr",
            extends: None,
            description: Some(
                "DSI Host data lane timer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lp2hs_time",
                    description: Some(
                        "Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs2lp_time",
                    description: Some(
                        "High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles.",
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
            name: "Dltrcr",
            extends: None,
            description: Some(
                "DSI Host data lane timer read configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mrd_time",
                    description: Some(
                        "Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.",
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
            name: "Dpcbcr",
            extends: None,
            description: Some(
                "DSI D-PHY clock band control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bc",
                    description: Some(
                        "Band control This field selects the frequency band used by the D-PHY. Others: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dpcsrcr",
            extends: None,
            description: Some(
                "DSI D-PHY clock skew rate control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src",
                    description: Some(
                        "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved.",
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
            ],
        },
        FieldSet {
            name: "Dpdl0bcr",
            extends: None,
            description: Some(
                "DSI D-PHY data lane 0 band control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bc",
                    description: Some(
                        "Band control This field selects the frequency band used by the D-PHY. Others: Reserved.",
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
        FieldSet {
            name: "Dpdl0srcr",
            extends: None,
            description: Some(
                "DSI D-PHY data lane 0 skew rate control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src",
                    description: Some(
                        "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved.",
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
            ],
        },
        FieldSet {
            name: "Dpdl1bcr",
            extends: None,
            description: Some(
                "DSI D-PHY data lane 1 band control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bc",
                    description: Some(
                        "Band control This field selects the frequency band used by the D-PHY. Others: Reserved.",
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
        FieldSet {
            name: "Dpdl1srcr",
            extends: None,
            description: Some(
                "DSI D-PHY data lane 1 skew rate control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src",
                    description: Some(
                        "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved.",
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
            ],
        },
        FieldSet {
            name: "Fbsr",
            extends: None,
            description: Some(
                "DSI Host FIFO and buffer status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcwfe",
                    description: Some(
                        "Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:.",
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
                    name: "vcwff",
                    description: Some(
                        "Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:.",
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
                    name: "vpwfe",
                    description: Some(
                        "Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:.",
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
                    name: "vpwff",
                    description: Some(
                        "Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:.",
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
                    name: "acwfe",
                    description: Some(
                        "Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:.",
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
                    name: "acwff",
                    description: Some(
                        "Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:.",
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
                    name: "apwfe",
                    description: Some(
                        "Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:.",
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
                    name: "apwff",
                    description: Some(
                        "Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:.",
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
                    name: "vpbe",
                    description: Some(
                        "Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:.",
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
                    name: "vpbf",
                    description: Some(
                        "Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:.",
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
                    name: "acbe",
                    description: Some(
                        "Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:.",
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
                    name: "acbf",
                    description: Some(
                        "Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:.",
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
                    name: "apbe",
                    description: Some(
                        "Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:.",
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
                    name: "apbf",
                    description: Some(
                        "Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:.",
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
            ],
        },
        FieldSet {
            name: "Fir0",
            extends: None,
            description: Some(
                "DSI Host force interrupt register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fae0",
                    description: Some(
                        "Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0.",
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
                    name: "fae1",
                    description: Some(
                        "Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1.",
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
                    name: "fae2",
                    description: Some(
                        "Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2.",
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
                    name: "fae3",
                    description: Some(
                        "Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3.",
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
                    name: "fae4",
                    description: Some(
                        "Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4.",
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
                    name: "fae5",
                    description: Some(
                        "Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5.",
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
                    name: "fae6",
                    description: Some(
                        "Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6.",
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
                    name: "fae7",
                    description: Some(
                        "Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7.",
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
                    name: "fae8",
                    description: Some(
                        "Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8.",
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
                    name: "fae9",
                    description: Some(
                        "Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9.",
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
                    name: "fae10",
                    description: Some(
                        "Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10.",
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
                    name: "fae11",
                    description: Some(
                        "Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11.",
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
                    name: "fae12",
                    description: Some(
                        "Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12.",
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
                    name: "fae13",
                    description: Some(
                        "Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13.",
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
                    name: "fae14",
                    description: Some(
                        "Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14.",
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
                    name: "fae15",
                    description: Some(
                        "Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15.",
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
                    name: "fpe0",
                    description: Some(
                        "Force PHY error 0 Writing one to this bit forces a PHY error 0.",
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
                    name: "fpe1",
                    description: Some(
                        "Force PHY error 1 Writing one to this bit forces a PHY error 1.",
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
                    name: "fpe2",
                    description: Some(
                        "Force PHY error 2 Writing one to this bit forces a PHY error 2.",
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
                    name: "fpe3",
                    description: Some(
                        "Force PHY error 3 Writing one to this bit forces a PHY error 3.",
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
                    name: "fpe4",
                    description: Some(
                        "Force PHY error 4 Writing one to this bit forces a PHY error 4.",
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
            ],
        },
        FieldSet {
            name: "Fir1",
            extends: None,
            description: Some(
                "DSI Host force interrupt register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftohstx",
                    description: Some(
                        "Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission.",
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
                    name: "ftolprx",
                    description: Some(
                        "Force timeout low-power reception Writing one to this bit forces a timeout low-power reception.",
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
                    name: "feccse",
                    description: Some(
                        "Force ECC single-bit error Writing one to this bit forces a ECC single-bit error.",
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
                    name: "feccme",
                    description: Some(
                        "Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error.",
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
                    name: "fcrce",
                    description: Some(
                        "Force CRC error Writing one to this bit forces a CRC error.",
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
                    name: "fpse",
                    description: Some(
                        "Force packet size error Writing one to this bit forces a packet size error.",
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
                    name: "feotpe",
                    description: Some(
                        "Force EoTp error Writing one to this bit forces a EoTp error.",
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
                    name: "flpwre",
                    description: Some(
                        "Force LTDC payload write error Writing one to this bit forces a LTDC payload write error.",
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
                    name: "fgcwre",
                    description: Some(
                        "Force generic command write error Writing one to this bit forces a generic command write error.",
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
                    name: "fgpwre",
                    description: Some(
                        "Force generic payload write error Writing one to this bit forces a generic payload write error.",
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
                    name: "fgptxe",
                    description: Some(
                        "Force generic payload transmit error Writing one to this bit forces a generic payload transmit error.",
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
                    name: "fgprde",
                    description: Some(
                        "Force generic payload read error Writing one to this bit forces a generic payload read error.",
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
                    name: "fgprxe",
                    description: Some(
                        "Force generic payload receive error Writing one to this bit forces a generic payload receive error.",
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
                    name: "fpbue",
                    description: Some(
                        "Force payload buffer underflow error Writing one to this bit forces a payload undrflow error.",
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
            ],
        },
        FieldSet {
            name: "Ghcr",
            extends: None,
            description: Some(
                "DSI Host generic header configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dt",
                    description: Some(
                        "Type This field configures the packet data type of the header packet.",
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
                    name: "vcid",
                    description: Some(
                        "Channel This field configures the virtual channel ID of the header packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wclsb",
                    description: Some(
                        "WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets.",
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
                    name: "wcmsb",
                    description: Some(
                        "WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets.",
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
            name: "Gpdr",
            extends: None,
            description: Some(
                "DSI Host generic payload data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data1",
                    description: Some(
                        "Payload byte 1 This field indicates the byte 1 of the packet payload.",
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
                    name: "data2",
                    description: Some(
                        "Payload byte 2 This field indicates the byte 2 of the packet payload.",
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
                    name: "data3",
                    description: Some(
                        "Payload byte 3 This field indicates the byte 3 of the packet payload.",
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
                    name: "data4",
                    description: Some(
                        "Payload byte 4 This field indicates the byte 4 of the packet payload.",
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
            name: "Gpsr",
            extends: None,
            description: Some(
                "DSI Host generic packet status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdfe",
                    description: Some(
                        "Command FIFO empty This bit indicates the empty status of the generic command FIFO:.",
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
                    name: "cmdff",
                    description: Some(
                        "Command FIFO full This bit indicates the full status of the generic command FIFO:.",
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
                    name: "pwrfe",
                    description: Some(
                        "Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:.",
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
                    name: "pwrff",
                    description: Some(
                        "Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:.",
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
                    name: "prdfe",
                    description: Some(
                        "Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:.",
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
                    name: "prdff",
                    description: Some(
                        "Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:.",
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
                    name: "rcb",
                    description: Some(
                        "Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:.",
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
                    name: "cmdbe",
                    description: Some(
                        "Command buffer empty This bit indicates the empty status of the generic payload internal buffer:.",
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
                    name: "cmdbf",
                    description: Some(
                        "Command buffer full This bit indicates the full status of the generic command internal buffer:.",
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
                    name: "pbe",
                    description: Some(
                        "Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:.",
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
                    name: "pbf",
                    description: Some(
                        "Payload buffer full This bit indicates the full status of the generic payload internal buffer:.",
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
            ],
        },
        FieldSet {
            name: "Gvcidr",
            extends: None,
            description: Some(
                "DSI Host generic VCID register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcidrx",
                    description: Some(
                        "Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification.",
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
                    name: "vcidtx",
                    description: Some(
                        "Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted.",
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
        FieldSet {
            name: "Ier0",
            extends: None,
            description: Some(
                "DSI Host interrupt enable register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ae0ie",
                    description: Some(
                        "Acknowledge error 0 interrupt enable This bit enables the interrupt generation on acknowledge error 0.",
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
                    name: "ae1ie",
                    description: Some(
                        "Acknowledge error 1 interrupt enable This bit enables the interrupt generation on acknowledge error 1.",
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
                    name: "ae2ie",
                    description: Some(
                        "Acknowledge error 2 interrupt enable This bit enables the interrupt generation on acknowledge error 2.",
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
                    name: "ae3ie",
                    description: Some(
                        "Acknowledge error 3 interrupt enable This bit enables the interrupt generation on acknowledge error 3.",
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
                    name: "ae4ie",
                    description: Some(
                        "Acknowledge error 4 interrupt enable This bit enables the interrupt generation on acknowledge error 4.",
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
                    name: "ae5ie",
                    description: Some(
                        "Acknowledge error 5 interrupt enable This bit enables the interrupt generation on acknowledge error 5.",
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
                    name: "ae6ie",
                    description: Some(
                        "Acknowledge error 6 interrupt enable This bit enables the interrupt generation on acknowledge error 6.",
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
                    name: "ae7ie",
                    description: Some(
                        "Acknowledge error 7 interrupt enable This bit enables the interrupt generation on acknowledge error 7.",
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
                    name: "ae8ie",
                    description: Some(
                        "Acknowledge error 8 interrupt enable This bit enables the interrupt generation on acknowledge error 8.",
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
                    name: "ae9ie",
                    description: Some(
                        "Acknowledge error 9 interrupt enable This bit enables the interrupt generation on acknowledge error 9.",
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
                    name: "ae10ie",
                    description: Some(
                        "Acknowledge error 10 interrupt enable This bit enables the interrupt generation on acknowledge error 10.",
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
                    name: "ae11ie",
                    description: Some(
                        "Acknowledge error 11 interrupt enable This bit enables the interrupt generation on acknowledge error 11.",
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
                    name: "ae12ie",
                    description: Some(
                        "Acknowledge error 12 interrupt enable This bit enables the interrupt generation on acknowledge error 12.",
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
                    name: "ae13ie",
                    description: Some(
                        "Acknowledge error 13 interrupt enable This bit enables the interrupt generation on acknowledge error 13.",
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
                    name: "ae14ie",
                    description: Some(
                        "Acknowledge error 14 interrupt enable This bit enables the interrupt generation on acknowledge error 14.",
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
                    name: "ae15ie",
                    description: Some(
                        "Acknowledge error 15 interrupt enable This bit enables the interrupt generation on acknowledge error 15.",
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
                    name: "pe0ie",
                    description: Some(
                        "PHY error 0 interrupt enable This bit enables the interrupt generation on PHY error 0.",
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
                    name: "pe1ie",
                    description: Some(
                        "PHY error 1 interrupt enable This bit enables the interrupt generation on PHY error 1.",
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
                    name: "pe2ie",
                    description: Some(
                        "PHY error 2 interrupt enable This bit enables the interrupt generation on PHY error 2.",
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
                    name: "pe3ie",
                    description: Some(
                        "PHY error 3 interrupt enable This bit enables the interrupt generation on PHY error 4.",
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
                    name: "pe4ie",
                    description: Some(
                        "PHY error 4 interrupt enable This bit enables the interrupt generation on PHY error 4.",
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
            ],
        },
        FieldSet {
            name: "Ier1",
            extends: None,
            description: Some(
                "DSI Host interrupt enable register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tohstxie",
                    description: Some(
                        "Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission.",
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
                    name: "tolprxie",
                    description: Some(
                        "Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.",
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
                    name: "eccseie",
                    description: Some(
                        "ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.",
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
                    name: "eccmeie",
                    description: Some(
                        "ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.",
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
                    name: "crceie",
                    description: Some(
                        "CRC error interrupt enable This bit enables the interrupt generation on CRC error.",
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
                    name: "pseie",
                    description: Some(
                        "Packet size error interrupt enable This bit enables the interrupt generation on packet size error.",
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
                    name: "eotpeie",
                    description: Some(
                        "EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.",
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
                    name: "lpwreie",
                    description: Some(
                        "LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.",
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
                    name: "gcwreie",
                    description: Some(
                        "Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.",
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
                    name: "gpwreie",
                    description: Some(
                        "Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.",
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
                    name: "gptxeie",
                    description: Some(
                        "Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.",
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
                    name: "gprdeie",
                    description: Some(
                        "Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.",
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
                    name: "gprxeie",
                    description: Some(
                        "Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.",
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
                    name: "pbueie",
                    description: Some(
                        "Payload buffer underflow error interrupt enable This bit enables the interrupt generation on payload buffer underflow error.",
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
            ],
        },
        FieldSet {
            name: "Isr0",
            extends: None,
            description: Some(
                "DSI Host interrupt and status register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ae0",
                    description: Some(
                        "Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report.",
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
                    name: "ae1",
                    description: Some(
                        "Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report.",
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
                    name: "ae2",
                    description: Some(
                        "Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report.",
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
                    name: "ae3",
                    description: Some(
                        "Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report.",
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
                    name: "ae4",
                    description: Some(
                        "Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report.",
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
                    name: "ae5",
                    description: Some(
                        "Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report.",
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
                    name: "ae6",
                    description: Some(
                        "Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report.",
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
                    name: "ae7",
                    description: Some(
                        "Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report.",
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
                    name: "ae8",
                    description: Some(
                        "Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report.",
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
                    name: "ae9",
                    description: Some(
                        "Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report.",
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
                    name: "ae10",
                    description: Some(
                        "Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report.",
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
                    name: "ae11",
                    description: Some(
                        "Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report.",
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
                    name: "ae12",
                    description: Some(
                        "Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report.",
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
                    name: "ae13",
                    description: Some(
                        "Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report.",
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
                    name: "ae14",
                    description: Some(
                        "Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report.",
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
                    name: "ae15",
                    description: Some(
                        "Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report.",
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
                    name: "pe0",
                    description: Some(
                        "PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0.",
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
                    name: "pe1",
                    description: Some(
                        "PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0.",
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
                    name: "pe2",
                    description: Some(
                        "PHY error 2 This bit indicates the ErrControl error from lane 0.",
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
                    name: "pe3",
                    description: Some(
                        "PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0.",
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
                    name: "pe4",
                    description: Some(
                        "PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0.",
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
            ],
        },
        FieldSet {
            name: "Isr1",
            extends: None,
            description: Some(
                "DSI Host interrupt and status register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tohstx",
                    description: Some(
                        "Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected.",
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
                    name: "tolprx",
                    description: Some(
                        "Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected.",
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
                    name: "eccse",
                    description: Some(
                        "ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet.",
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
                    name: "eccme",
                    description: Some(
                        "ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet.",
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
                    name: "crce",
                    description: Some(
                        "CRC error This bit indicates that the CRC error is detected in the received packet payload.",
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
                    name: "pse",
                    description: Some(
                        "Packet size error This bit indicates that the packet size error is detected during the packet reception.",
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
                    name: "eotpe",
                    description: Some(
                        "EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission.",
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
                    name: "lpwre",
                    description: Some(
                        "LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted.",
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
                    name: "gcwre",
                    description: Some(
                        "Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written.",
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
                    name: "gpwre",
                    description: Some(
                        "Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written.",
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
                    name: "gptxe",
                    description: Some(
                        "Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent.",
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
                    name: "gprde",
                    description: Some(
                        "Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted.",
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
                    name: "gprxe",
                    description: Some(
                        "Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted.",
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
                    name: "pbue",
                    description: Some(
                        "Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode.",
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
            ],
        },
        FieldSet {
            name: "Lcccr",
            extends: None,
            description: Some(
                "DSI Host LTDC current color coding register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colc",
                    description: Some(
                        "Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit.",
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
                    name: "lpe",
                    description: Some(
                        "Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations.",
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
            name: "Lccr",
            extends: None,
            description: Some(
                "DSI Host LTDC command configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdsize",
                    description: Some(
                        "Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.",
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
            name: "Lcolcr",
            extends: None,
            description: Some(
                "DSI Host LTDC color coding register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colc",
                    description: Some(
                        "Color coding This field configures the DPI color coding. Others: Reserved.",
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
                    name: "lpe",
                    description: Some(
                        "Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration.",
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
            name: "Lcvcidr",
            extends: None,
            description: Some(
                "DSI Host LTDC current VCID register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcid",
                    description: Some(
                        "Virtual channel ID This field returns the virtual channel ID for the LTDC interface.",
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
            ],
        },
        FieldSet {
            name: "Lpcr",
            extends: None,
            description: Some(
                "DSI Host LTDC polarity configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dep",
                    description: Some(
                        "Data enable polarity This bit configures the polarity of data enable pin.",
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
                    name: "vsp",
                    description: Some(
                        "VSYNC polarity This bit configures the polarity of VSYNC pin.",
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
                    name: "hsp",
                    description: Some(
                        "HSYNC polarity This bit configures the polarity of HSYNC pin.",
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
            ],
        },
        FieldSet {
            name: "Lpmccr",
            extends: None,
            description: Some(
                "DSI Host low-power mode current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlpsize",
                    description: Some(
                        "VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode.",
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
                    name: "lpsize",
                    description: Some(
                        "Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode.",
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
            name: "Lpmcr",
            extends: None,
            description: Some(
                "DSI Host low-power mode configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlpsize",
                    description: Some(
                        "VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions.",
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
                    name: "lpsize",
                    description: Some(
                        "Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.",
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
            name: "Lvcidr",
            extends: None,
            description: Some(
                "DSI Host LTDC VCID register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcid",
                    description: Some(
                        "Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic.",
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
            ],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some(
                "DSI Host mode configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdm",
                    description: Some(
                        "Command mode This bit configures the DSI Host in either video or command mode.",
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
            name: "Pconfr",
            extends: None,
            description: Some(
                "DSI Host PHY configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nl",
                    description: Some(
                        "Number of lanes This field configures the number of active data lanes: Others: Reserved.",
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
                    name: "sw_time",
                    description: Some(
                        "Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state.",
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
            name: "Pcr",
            extends: None,
            description: Some(
                "DSI Host protocol configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ettxe",
                    description: Some(
                        "EoTp transmission enable This bit enables the EoTP transmission.",
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
                    name: "etrxe",
                    description: Some(
                        "EoTp reception enable This bit enables the EoTp reception.",
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
                    name: "btae",
                    description: Some(
                        "Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.",
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
                    name: "eccrxe",
                    description: Some(
                        "ECC reception enable This bit enables the ECC reception, error correction and reporting.",
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
                    name: "crcrxe",
                    description: Some(
                        "CRC reception enable This bit enables the CRC reception and error reporting.",
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
                    name: "ettxlpe",
                    description: Some(
                        "EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power.",
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
            name: "Pctlr",
            extends: None,
            description: Some(
                "DSI Host PHY control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den",
                    description: Some(
                        "Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state.",
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
                    name: "cke",
                    description: Some(
                        "Clock enable This bit enables the D-PHY clock lane module:.",
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
            ],
        },
        FieldSet {
            name: "Psr",
            extends: None,
            description: Some(
                "DSI Host PHY status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "PHY direction This bit indicates the status of phydirection D-PHY signal.",
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
                    name: "pssc",
                    description: Some(
                        "PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal.",
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
                    name: "uanc",
                    description: Some(
                        "ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal.",
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
                    name: "pss0",
                    description: Some(
                        "PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal.",
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
                    name: "uan0",
                    description: Some(
                        "ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal.",
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
                    name: "rue0",
                    description: Some(
                        "RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal.",
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
                    name: "pss1",
                    description: Some(
                        "PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal.",
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
                    name: "uan1",
                    description: Some(
                        "ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal.",
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
            name: "Pttcr",
            extends: None,
            description: Some(
                "DSI Host PHY TX triggers configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_trig",
                    description: Some(
                        "Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time.",
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
            ],
        },
        FieldSet {
            name: "Pucr",
            extends: None,
            description: Some(
                "DSI Host PHY ULPS control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "urcl",
                    description: Some(
                        "ULPS request on clock lane ULPS mode request on clock lane.",
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
                    name: "uecl",
                    description: Some(
                        "ULPS exit on clock lane ULPS mode exit on clock lane.",
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
                    name: "urdl",
                    description: Some(
                        "ULPS request on data lane ULPS mode request on all active data lanes.",
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
                    name: "uedl",
                    description: Some(
                        "ULPS exit on data lane ULPS mode exit on all active data lanes.",
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
            ],
        },
        FieldSet {
            name: "Tccr0",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lprx_tocnt",
                    description: Some(
                        "Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles).",
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
                    name: "hstx_tocnt",
                    description: Some(
                        "High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one FRAME data transmission *Â\u{a0}(1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one LINE data transmission *Â\u{a0}(1Â\u{a0}+Â\u{a0}10%).",
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
            name: "Tccr1",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsrd_tocnt",
                    description: Some(
                        "High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.",
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
            name: "Tccr2",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lprd_tocnt",
                    description: Some(
                        "Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.",
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
            name: "Tccr3",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hswr_tocnt",
                    description: Some(
                        "High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.",
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
                    name: "pm",
                    description: Some(
                        "Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready.",
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
            ],
        },
        FieldSet {
            name: "Tccr4",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpwr_tocnt",
                    description: Some(
                        "Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.",
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
            name: "Tccr5",
            extends: None,
            description: Some(
                "DSI Host timeout counter configuration register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bta_tocnt",
                    description: Some(
                        "Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the Dâ\u{80}\u{91}PHY enters the Stop state and causes no interrupts.",
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
            name: "Vcccr",
            extends: None,
            description: Some(
                "DSI Host video chunks current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numc",
                    description: Some(
                        "Number of chunks This field returns the number of chunks being transmitted during a line period.",
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
            ],
        },
        FieldSet {
            name: "Vccr",
            extends: None,
            description: Some(
                "DSI Host video chunks configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numc",
                    description: Some(
                        "Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.",
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
            ],
        },
        FieldSet {
            name: "Vhbpccr",
            extends: None,
            description: Some(
                "DSI Host video HBP current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hbp",
                    description: Some(
                        "Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhbpcr",
            extends: None,
            description: Some(
                "DSI Host video HBP configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hbp",
                    description: Some(
                        "Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhsaccr",
            extends: None,
            description: Some(
                "DSI Host video HSA current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsa",
                    description: Some(
                        "Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhsacr",
            extends: None,
            description: Some(
                "DSI Host video HSA configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsa",
                    description: Some(
                        "Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vlccr",
            extends: None,
            description: Some(
                "DSI Host video line current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hline",
                    description: Some(
                        "Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.",
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
            name: "Vlcr",
            extends: None,
            description: Some(
                "DSI Host video line configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hline",
                    description: Some(
                        "Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.",
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
            name: "Vmccr",
            extends: None,
            description: Some(
                "DSI Host video mode current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vmt",
                    description: Some(
                        "Video mode type This field returns the current video mode transmission type: 1x: Burst mode.",
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
                    name: "lpvsae",
                    description: Some(
                        "Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows.",
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
                    name: "lpvbpe",
                    description: Some(
                        "Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows.",
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
                    name: "lpvfpe",
                    description: Some(
                        "Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows.",
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
                    name: "lpvae",
                    description: Some(
                        "Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows.",
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
                    name: "lphbpe",
                    description: Some(
                        "Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows.",
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
                    name: "lphfe",
                    description: Some(
                        "Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows.",
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
                    name: "fbtaae",
                    description: Some(
                        "Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame.",
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
                    name: "lpce",
                    description: Some(
                        "Low-power command enable This bit returns the current command transmission state in low-power mode.",
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
            ],
        },
        FieldSet {
            name: "Vmcr",
            extends: None,
            description: Some(
                "DSI Host video mode configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vmt",
                    description: Some(
                        "Video mode type This field configures the video mode transmission type : 1x: Burst mode.",
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
                    name: "lpvsae",
                    description: Some(
                        "Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.",
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
                    name: "lpvbpe",
                    description: Some(
                        "Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.",
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
                    name: "lpvfpe",
                    description: Some(
                        "Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.",
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
                    name: "lpvae",
                    description: Some(
                        "Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.",
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
                    name: "lphbpe",
                    description: Some(
                        "Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.",
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
                    name: "lphfpe",
                    description: Some(
                        "Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.",
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
                    name: "fbtaae",
                    description: Some(
                        "Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.",
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
                    name: "lpce",
                    description: Some(
                        "Low-power command enable This bit enables the command transmission only in low-power mode.",
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
                    name: "pge",
                    description: Some(
                        "Pattern generator enable This bit enables the video mode pattern generator.",
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
                    name: "pgm",
                    description: Some(
                        "Pattern generator mode This bit configures the pattern generator mode.",
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
                    name: "pgo",
                    description: Some(
                        "Pattern generator orientation This bit configures the color bar orientation.",
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
            ],
        },
        FieldSet {
            name: "Vnpccr",
            extends: None,
            description: Some(
                "DSI Host video null packet current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "npsize",
                    description: Some(
                        "Null packet size This field returns the number of bytes inside a null packet.",
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
            ],
        },
        FieldSet {
            name: "Vnpcr",
            extends: None,
            description: Some(
                "DSI Host video null packet configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "npsize",
                    description: Some(
                        "Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.",
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
            ],
        },
        FieldSet {
            name: "Vpccr",
            extends: None,
            description: Some(
                "DSI Host video packet current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpsize",
                    description: Some(
                        "Video packet size This field returns the number of pixels in a single video packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vpcr",
            extends: None,
            description: Some(
                "DSI Host video packet configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpsize",
                    description: Some(
                        "Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vr",
            extends: None,
            description: Some(
                "DSI Host version register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "Version of the DSI Host This read-only register contains the version of the DSI Host.",
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
            name: "Vscr",
            extends: None,
            description: Some(
                "DSI Host video shadow control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.",
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
                    name: "ur",
                    description: Some(
                        "Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.",
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
            name: "Vvaccr",
            extends: None,
            description: Some(
                "DSI Host video VA current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "va",
                    description: Some(
                        "Vertical active duration This field returns the current vertical active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvacr",
            extends: None,
            description: Some(
                "DSI Host video VA configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "va",
                    description: Some(
                        "Vertical active duration This fields configures the vertical active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvbpccr",
            extends: None,
            description: Some(
                "DSI Host video VBP current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp",
                    description: Some(
                        "Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvbpcr",
            extends: None,
            description: Some(
                "DSI Host video VBP configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp",
                    description: Some(
                        "Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvfpccr",
            extends: None,
            description: Some(
                "DSI Host video VFP current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp",
                    description: Some(
                        "Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvfpcr",
            extends: None,
            description: Some(
                "DSI Host video VFP configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp",
                    description: Some(
                        "Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvsaccr",
            extends: None,
            description: Some(
                "DSI Host video VSA current configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa",
                    description: Some(
                        "Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvsacr",
            extends: None,
            description: Some(
                "DSI Host video VSA configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa",
                    description: Some(
                        "Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wcfgr",
            extends: None,
            description: Some(
                "DSI Wrapper configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsim",
                    description: Some(
                        "DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (CR.EN = 0).",
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
                    name: "colmux",
                    description: Some(
                        "Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ\u{a0}=Â\u{a0}0).",
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
                    name: "tesrc",
                    description: Some(
                        "TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0).",
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
                    name: "tepol",
                    description: Some(
                        "TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0).",
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
                    name: "ar",
                    description: Some(
                        "Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (CR.EN = 0).",
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
                    name: "vspol",
                    description: Some(
                        "VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ\u{a0}=Â\u{a0}0).",
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
            ],
        },
        FieldSet {
            name: "Wcr",
            extends: None,
            description: Some(
                "DSI Wrapper control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colm",
                    description: Some(
                        "Color mode This bit controls the display color mode in video mode.",
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
                    name: "shtdn",
                    description: Some(
                        "Shutdown This bit controls the display shutdown in video mode.",
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
                    name: "ltdcen",
                    description: Some(
                        "LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.",
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
                    name: "dsien",
                    description: Some(
                        "DSI enable This bit enables the DSI Wrapper.",
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
            ],
        },
        FieldSet {
            name: "Wier",
            extends: None,
            description: Some(
                "DSI Wrapper interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teie",
                    description: Some(
                        "Tearing effect interrupt enable This bit enables the tearing effect interrupt.",
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
                    name: "erie",
                    description: Some(
                        "End of refresh interrupt enable This bit enables the end of refresh interrupt.",
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
                    name: "plllie",
                    description: Some(
                        "PLL lock interrupt enable This bit enables the PLL lock interrupt.",
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
                    name: "plluie",
                    description: Some(
                        "PLL unlock interrupt enable This bit enables the PLL unlock interrupt.",
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
            ],
        },
        FieldSet {
            name: "Wifcr",
            extends: None,
            description: Some(
                "DSI Wrapper interrupt flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cteif",
                    description: Some(
                        "Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the WSR register.",
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
                    name: "cerif",
                    description: Some(
                        "Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the WSR register.",
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
                    name: "cplllif",
                    description: Some(
                        "Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the WSR register.",
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
                    name: "cplluif",
                    description: Some(
                        "Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the WSR register.",
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
            ],
        },
        FieldSet {
            name: "Wisr",
            extends: None,
            description: Some(
                "DSI Wrapper interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teif",
                    description: Some(
                        "Tearing effect interrupt flag This bit is set when a tearing effect event occurs.",
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
                    name: "erif",
                    description: Some(
                        "End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.",
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
                    name: "pllls",
                    description: Some(
                        "PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.",
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
                    name: "plllif",
                    description: Some(
                        "PLL lock interrupt flag This bit is set when the PLL becomes locked.",
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
                    name: "plluif",
                    description: Some(
                        "PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.",
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
            ],
        },
        FieldSet {
            name: "Wpcr0",
            extends: None,
            description: Some(
                "DSI Wrapper PHY configuration register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swcl",
                    description: Some(
                        "Swap clock lane pins This bit swaps the pins on clock lane.",
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
                    name: "swdl0",
                    description: Some(
                        "Swap data lane 0 pins This bit swaps the pins on data lane 0.",
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
                    name: "swdl1",
                    description: Some(
                        "Swap data lane 1 pins This bit swaps the pins on clock lane.",
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
                    name: "ftxsmcl",
                    description: Some(
                        "Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.",
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
                    name: "ftxsmdl",
                    description: Some(
                        "Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.",
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
            name: "Wrpcr",
            extends: None,
            description: Some(
                "DSI Wrapper regulator and PLL control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllen",
                    description: Some(
                        "PLL enable This bit enables the D-PHY PLL.",
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
                    name: "ndiv",
                    description: Some(
                        "PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idf",
                    description: Some(
                        "PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odf",
                    description: Some(
                        "PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
