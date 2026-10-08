
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Otg",
            extends: None,
            description: Some(
                "USB OTG core by Synopsys (more docs at <https://www.intel.com/content/www/us/en/programmable/hps/agilex5/index_frames.html>)",
            ),
            items: &[
                BlockItem {
                    name: "gotgctl",
                    description: Some(
                        "Control and status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gotgctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gotgint",
                    description: Some(
                        "Interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gotgint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gahbcfg",
                    description: Some(
                        "AHB configuration register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gahbcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gusbcfg",
                    description: Some(
                        "USB configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gusbcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstctl",
                    description: Some(
                        "Reset register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grstctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gintsts",
                    description: Some(
                        "Core interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gintmsk",
                    description: Some(
                        "Interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxstsr",
                    description: Some(
                        "Receive status debug read register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxstsp",
                    description: Some(
                        "Status read and pop register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxfsiz",
                    description: Some(
                        "Receive FIFO size register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxfsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf0",
                    description: Some(
                        "Endpoint 0 transmit FIFO size register (device mode)",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hnptxfsiz",
                    description: Some(
                        "Non-periodic transmit FIFO size register (host mode)",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hnptxsts",
                    description: Some(
                        "Non-periodic transmit FIFO/queue status register (host mode)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hnptxsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gi2cctl",
                    description: Some(
                        "OTG I2C access register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gi2cctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gccfg_v1",
                    description: Some(
                        "General core configuration register, for core_id 0x0000_1xxx",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GccfgV1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gccfg_v2",
                    description: Some(
                        "General core configuration register, for core_id 0x0000_[23]xxx",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GccfgV2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gccfg_v3",
                    description: Some(
                        "General core configuration register, for core_id 0x0000_5xxx",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GccfgV3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cid",
                    description: Some(
                        "Core ID register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snpsid",
                    description: Some(
                        "Synopsis ID Register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hwcfg1",
                    description: Some(
                        "User HW Config 1 Register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hwcfg2",
                    description: Some(
                        "User HW Config 2 Register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hwcfg3",
                    description: Some(
                        "User HW Config 3 Register",
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
                    name: "hwcfg4",
                    description: Some(
                        "User HW Config 4 Register",
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
                    name: "glpmcfg",
                    description: Some(
                        "OTG core LPM configuration register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Glpmcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpwrdn",
                    description: Some(
                        "Global PowerDn Register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "gdfifocfg",
                    description: Some(
                        "Global DFIFO SW Config Register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "adpctl",
                    description: Some(
                        "ADP (Attach Detection Protocol) Control Register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hptxfsiz",
                    description: Some(
                        "Host periodic transmit FIFO size register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf",
                    description: Some(
                        "Device IN endpoint transmit FIFO size register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcfg",
                    description: Some(
                        "Host configuration register",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hfir",
                    description: Some(
                        "Host frame interval register",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hfir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hfnum",
                    description: Some(
                        "Host frame number/frame time remaining register",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hfnum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hptxsts",
                    description: Some(
                        "Periodic transmit FIFO/queue status register",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hptxsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "haint",
                    description: Some(
                        "Host all channels interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Haint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "haintmsk",
                    description: Some(
                        "Host all channels interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Haintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hflbaddr",
                    description: Some(
                        "Host Frame Scheduling List Register",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hprt",
                    description: Some(
                        "Host port control and status register",
                    ),
                    array: None,
                    byte_offset: 0x440,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hprt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcchar",
                    description: Some(
                        "Host channel characteristics register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hcchar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcsplt",
                    description: Some(
                        "Host channel split control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hcint",
                    description: Some(
                        "Host channel interrupt register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hcint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcintmsk",
                    description: Some(
                        "Host channel mask register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x50c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hcintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hctsiz",
                    description: Some(
                        "Host channel transfer size register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x510,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hctsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcdma",
                    description: Some(
                        "Host channel DMA address register (config for scatter/gather)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x514,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hcdma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hcdmab",
                    description: Some(
                        "Host channel DMA address register (address for current transfer; debug)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x51c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dcfg",
                    description: Some(
                        "Device configuration register",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dctl",
                    description: Some(
                        "Device control register",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dsts",
                    description: Some(
                        "Device status register",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepmsk",
                    description: Some(
                        "Device IN endpoint common interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x810,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepmsk",
                    description: Some(
                        "Device OUT endpoint common interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x814,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daint",
                    description: Some(
                        "Device all endpoints interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x818,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Daint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daintmsk",
                    description: Some(
                        "All endpoints interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x81c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Daintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dvbusdis",
                    description: Some(
                        "Device VBUS discharge time register",
                    ),
                    array: None,
                    byte_offset: 0x828,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dvbusdis",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dvbuspulse",
                    description: Some(
                        "Device VBUS pulsing time register",
                    ),
                    array: None,
                    byte_offset: 0x82c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dvbuspulse",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepempmsk",
                    description: Some(
                        "Device IN endpoint FIFO empty interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x834,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepempmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl",
                    description: Some(
                        "Device IN endpoint control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint",
                    description: Some(
                        "Device IN endpoint interrupt register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x908,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz",
                    description: Some(
                        "Device IN endpoint transfer size register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x910,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts",
                    description: Some(
                        "Device IN endpoint transmit FIFO status register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x918,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl",
                    description: Some(
                        "Device OUT endpoint control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint",
                    description: Some(
                        "Device OUT endpoint interrupt register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xb08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz",
                    description: Some(
                        "Device OUT endpoint transfer size register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xb10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma",
                    description: Some(
                        "Device OUT/IN endpoint DMA address register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xb14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "pcgcctl",
                    description: Some(
                        "Power and clock gating control register",
                    ),
                    array: None,
                    byte_offset: 0xe00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcgcctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo",
                    description: Some(
                        "Device endpoint / host channel FIFO register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4096,
                            },
                        ),
                    ),
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifo",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adpctl",
            extends: None,
            description: Some(
                "ADP (Attach Detection Protocol) Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prb_dschg",
                    description: Some(
                        "Probe Discharge time (times for TADP_DSCHG)",
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
                    name: "prb_delta",
                    description: Some(
                        "Probe Delta (resolution for RTIM)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prb_per",
                    description: Some(
                        "Probe Period (TADP_PRD)",
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
                    name: "rtim",
                    description: Some(
                        "Probe Period (TADP_PRD)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enaprb",
                    description: Some(
                        "Enable Probe",
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
                    name: "enasns",
                    description: Some(
                        "Enable Sense",
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
                    name: "adpres",
                    description: Some(
                        "ADP Reset",
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
                    name: "adpen",
                    description: Some(
                        "ADP Enable",
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
                    name: "adp_prb_int",
                    description: Some(
                        "ADP Probe Interrupt Enable",
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
                    name: "adp_sns_int",
                    description: Some(
                        "ADP Sense Interrupt Enable",
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
                    name: "adp_tmout_int",
                    description: Some(
                        "ADP Timeout Interrupt Enable",
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
                    name: "adp_prb_msk",
                    description: Some(
                        "ADP Probe Interrupt Mask",
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
                Field {
                    name: "adp_tmout_msk",
                    description: Some(
                        "ADP Timeout Interrupt Mask",
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
                    name: "ar",
                    description: Some(
                        "Access Request",
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
            ],
        },
        FieldSet {
            name: "Cid",
            extends: None,
            description: Some(
                "Core ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "product_id",
                    description: Some(
                        "Product ID field",
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
            name: "Daint",
            extends: None,
            description: Some(
                "Device all endpoints interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepint",
                    description: Some(
                        "IN endpoint interrupt bits",
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
                    name: "oepint",
                    description: Some(
                        "OUT endpoint interrupt bits",
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
            name: "Daintmsk",
            extends: None,
            description: Some(
                "All endpoints interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepm",
                    description: Some(
                        "IN EP interrupt mask bits",
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
                    name: "oepm",
                    description: Some(
                        "OUT EP interrupt mask bits",
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
            name: "Dcfg",
            extends: None,
            description: Some(
                "Device configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dspd",
                    description: Some(
                        "Device speed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dspd",
                    ),
                },
                Field {
                    name: "nzlsohsk",
                    description: Some(
                        "Non-zero-length status OUT handshake",
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
                    name: "dad",
                    description: Some(
                        "Device address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pfivl",
                    description: Some(
                        "Periodic frame interval",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pfivl",
                    ),
                },
                Field {
                    name: "xcvrdly",
                    description: Some(
                        "Transceiver delay",
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
            name: "Dctl",
            extends: None,
            description: Some(
                "Device control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwusig",
                    description: Some(
                        "Remote wakeup signaling",
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
                    name: "sdis",
                    description: Some(
                        "Soft disconnect",
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
                    name: "ginsts",
                    description: Some(
                        "Global IN NAK status",
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
                    name: "gonsts",
                    description: Some(
                        "Global OUT NAK status",
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
                    name: "tctl",
                    description: Some(
                        "Test control",
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
                    name: "sginak",
                    description: Some(
                        "Set global IN NAK",
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
                    name: "cginak",
                    description: Some(
                        "Clear global IN NAK",
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
                    name: "sgonak",
                    description: Some(
                        "Set global OUT NAK",
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
                    name: "cgonak",
                    description: Some(
                        "Clear global OUT NAK",
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
                    name: "poprgdne",
                    description: Some(
                        "Power-on programming done",
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
            name: "Diepctl",
            extends: None,
            description: Some(
                "Device endpoint control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "MPSIZ",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbaep",
                    description: Some(
                        "USBAEP",
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
                    name: "eonum_dpid",
                    description: Some(
                        "EONUM/DPID",
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
                    name: "naksts",
                    description: Some(
                        "NAKSTS",
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
                    name: "eptyp",
                    description: Some(
                        "EPTYP",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Eptyp",
                    ),
                },
                Field {
                    name: "snpm",
                    description: Some(
                        "SNPM",
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
                    name: "stall",
                    description: Some(
                        "STALL",
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
                    name: "txfnum",
                    description: Some(
                        "TXFNUM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "CNAK",
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
                    name: "snak",
                    description: Some(
                        "SNAK",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
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
                    name: "soddfrm_sd1pid",
                    description: Some(
                        "SODDFRM/SD1PID",
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
                    name: "epdis",
                    description: Some(
                        "EPDIS",
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
                    name: "epena",
                    description: Some(
                        "EPENA",
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
            name: "Diepempmsk",
            extends: None,
            description: Some(
                "Device IN endpoint FIFO empty interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxfem",
                    description: Some(
                        "IN EP Tx FIFO empty interrupt mask bits",
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
            name: "Diepint",
            extends: None,
            description: Some(
                "Device endpoint interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "XFRC",
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
                    name: "epdisd",
                    description: Some(
                        "EPDISD",
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
                    name: "toc",
                    description: Some(
                        "TOC",
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
                    name: "ittxfe",
                    description: Some(
                        "ITTXFE",
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
                    name: "inepne",
                    description: Some(
                        "INEPNE",
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
                    name: "txfe",
                    description: Some(
                        "TXFE",
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
            name: "Diepmsk",
            extends: None,
            description: Some(
                "Device IN endpoint common interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrcm",
                    description: Some(
                        "Transfer completed interrupt mask",
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
                    name: "epdm",
                    description: Some(
                        "Endpoint disabled interrupt mask",
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
                    name: "tom",
                    description: Some(
                        "Timeout condition mask (Non-isochronous endpoints)",
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
                    name: "ittxfemsk",
                    description: Some(
                        "IN token received when TxFIFO empty mask",
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
                    name: "inepnmm",
                    description: Some(
                        "IN token received with EP mismatch mask",
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
                    name: "inepnem",
                    description: Some(
                        "IN endpoint NAK effective mask",
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
            name: "Dieptsiz",
            extends: None,
            description: Some(
                "Device endpoint transfer size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "Transfer size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doepctl",
            extends: None,
            description: Some(
                "Device endpoint control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "MPSIZ",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbaep",
                    description: Some(
                        "USBAEP",
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
                    name: "eonum_dpid",
                    description: Some(
                        "EONUM/DPID",
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
                    name: "naksts",
                    description: Some(
                        "NAKSTS",
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
                    name: "eptyp",
                    description: Some(
                        "EPTYP",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Eptyp",
                    ),
                },
                Field {
                    name: "snpm",
                    description: Some(
                        "SNPM",
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
                    name: "stall",
                    description: Some(
                        "STALL",
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
                    name: "cnak",
                    description: Some(
                        "CNAK",
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
                    name: "snak",
                    description: Some(
                        "SNAK",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
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
                    name: "soddfrm",
                    description: Some(
                        "SODDFRM",
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
                    name: "epdis",
                    description: Some(
                        "EPDIS",
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
                    name: "epena",
                    description: Some(
                        "EPENA",
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
            name: "Doepint",
            extends: None,
            description: Some(
                "Device endpoint interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "XFRC",
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
                    name: "epdisd",
                    description: Some(
                        "EPDISD",
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
                    name: "stup",
                    description: Some(
                        "STUP",
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
                    name: "otepdis",
                    description: Some(
                        "OTEPDIS",
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
                    name: "b2bstup",
                    description: Some(
                        "B2BSTUP",
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
            name: "Doepmsk",
            extends: None,
            description: Some(
                "Device OUT endpoint common interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrcm",
                    description: Some(
                        "Transfer completed interrupt mask",
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
                    name: "epdm",
                    description: Some(
                        "Endpoint disabled interrupt mask",
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
                    name: "stupm",
                    description: Some(
                        "SETUP phase done mask",
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
                    name: "otepdm",
                    description: Some(
                        "OUT token received when endpoint disabled mask",
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
            name: "Doeptsiz",
            extends: None,
            description: Some(
                "Device OUT endpoint transfer size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "Transfer size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxdpid_stupcnt",
                    description: Some(
                        "Received data PID/SETUP packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dsts",
            extends: None,
            description: Some(
                "Device status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "suspsts",
                    description: Some(
                        "Suspend status",
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
                    name: "enumspd",
                    description: Some(
                        "Enumerated speed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dspd",
                    ),
                },
                Field {
                    name: "eerr",
                    description: Some(
                        "Erratic error",
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
                    name: "fnsof",
                    description: Some(
                        "Frame number of the received SOF",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtxfsts",
            extends: None,
            description: Some(
                "Device IN endpoint transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "IN endpoint TxFIFO space available",
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
            name: "Dvbusdis",
            extends: None,
            description: Some(
                "Device VBUS discharge time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbusdt",
                    description: Some(
                        "Device VBUS discharge time",
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
            name: "Dvbuspulse",
            extends: None,
            description: Some(
                "Device VBUS pulsing time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dvbusp",
                    description: Some(
                        "Device VBUS pulsing time",
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
            name: "Fifo",
            extends: None,
            description: Some(
                "FIFO register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Data",
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
            name: "Fsiz",
            extends: None,
            description: Some(
                "FIFO size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa",
                    description: Some(
                        "RAM start address",
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
                    name: "fd",
                    description: Some(
                        "FIFO depth",
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
            name: "Gahbcfg",
            extends: None,
            description: Some(
                "AHB configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gint",
                    description: Some(
                        "Global interrupt mask",
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
                    name: "hbstlen",
                    description: Some(
                        "Burst length/type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "DMA enable",
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
                    name: "txfelvl",
                    description: Some(
                        "TxFIFO empty level",
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
                    name: "ptxfelvl",
                    description: Some(
                        "Periodic TxFIFO empty level",
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
            name: "GccfgV1",
            extends: None,
            description: Some(
                "General core configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrdwn",
                    description: Some(
                        "Power down",
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
                    name: "vbusasen",
                    description: Some(
                        "Enable the VBUS \"A\" sensing device",
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
                    name: "vbusbsen",
                    description: Some(
                        "Enable the VBUS \"B\" sensing device",
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
                    name: "sofouten",
                    description: Some(
                        "SOF output enable",
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
                    name: "novbussens",
                    description: Some(
                        "VBUS sensing disable",
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
            ],
        },
        FieldSet {
            name: "GccfgV2",
            extends: None,
            description: Some(
                "General core configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcdet",
                    description: Some(
                        "Data contact detection (DCD) status",
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
                    name: "pdet",
                    description: Some(
                        "Primary detection (PD) status",
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
                    name: "sdet",
                    description: Some(
                        "Secondary detection (SD) status",
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
                    name: "ps2det",
                    description: Some(
                        "DM pull-up detection status",
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
                    name: "pwrdwn",
                    description: Some(
                        "Power down",
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
                    name: "bcden",
                    description: Some(
                        "Battery charging detector (BCD) enable",
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
                    name: "dcden",
                    description: Some(
                        "Data contact detection (DCD) mode enable",
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
                    name: "pden",
                    description: Some(
                        "Primary detection (PD) mode enable",
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
                    name: "sden",
                    description: Some(
                        "Secondary detection (SD) mode enable",
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
                    name: "vbden",
                    description: Some(
                        "USB VBUS detection enable",
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
                    name: "phyhsen",
                    description: Some(
                        "Internal high-speed PHY enable.",
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
            name: "GccfgV3",
            extends: None,
            description: Some(
                "OTG general core configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chgdet",
                    description: Some(
                        "Charger detection, result of the current mode (primary or secondary).",
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
                    name: "fsvplus",
                    description: Some(
                        "Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).",
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
                    name: "fsvminus",
                    description: Some(
                        "Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard).",
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
                    name: "sessvld",
                    description: Some(
                        "VBUS session indicator Indicates if VBUS is above VBUS session threshold.",
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
                    name: "hcdpen",
                    description: Some(
                        "Host CDP behavior enable.",
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
                    name: "hcdpdeten",
                    description: Some(
                        "Host CDP port voltage detector enable on DP.",
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
                    name: "hvdmsrcen",
                    description: Some(
                        "Host CDP port Voltage source enable on DM.",
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
                    name: "dcden",
                    description: Some(
                        "Data Contact Detection enable.",
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
                    name: "pden",
                    description: Some(
                        "Primary detection enable.",
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
                    name: "vbden",
                    description: Some(
                        "VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation.",
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
                    name: "sden",
                    description: Some(
                        "Secondary detection enable.",
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
                    name: "vbvaloval",
                    description: Some(
                        "Software override value of the VBUS B-session detection.",
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
                    name: "vbvaloven",
                    description: Some(
                        "Enables a software override of the VBUS B-session detection.",
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
                Field {
                    name: "forcehostpd",
                    description: Some(
                        "Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active.",
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
            ],
        },
        FieldSet {
            name: "Gi2cctl",
            extends: None,
            description: Some(
                "I2C access register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwdata",
                    description: Some(
                        "I2C Read/Write Data",
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
                    name: "regaddr",
                    description: Some(
                        "I2C Register Address",
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
                    name: "addr",
                    description: Some(
                        "I2C Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2cen",
                    description: Some(
                        "I2C Enable",
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
                    name: "ack",
                    description: Some(
                        "I2C ACK",
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
                Field {
                    name: "i2cdevadr",
                    description: Some(
                        "I2C Device Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2cdatse0",
                    description: Some(
                        "I2C DatSe0 USB mode",
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
                    name: "rw",
                    description: Some(
                        "Read/Write Indicator",
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
                    name: "bsydne",
                    description: Some(
                        "I2C Busy/Done",
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
            name: "Gintmsk",
            extends: None,
            description: Some(
                "Interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmism",
                    description: Some(
                        "Mode mismatch interrupt mask",
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
                    name: "otgint",
                    description: Some(
                        "OTG interrupt mask",
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
                    name: "sofm",
                    description: Some(
                        "Start of frame mask",
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
                    name: "rxflvlm",
                    description: Some(
                        "Receive FIFO non-empty mask",
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
                    name: "nptxfem",
                    description: Some(
                        "Non-periodic TxFIFO empty mask",
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
                    name: "ginakeffm",
                    description: Some(
                        "Global non-periodic IN NAK effective mask",
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
                    name: "gonakeffm",
                    description: Some(
                        "Global OUT NAK effective mask",
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
                    name: "esuspm",
                    description: Some(
                        "Early suspend mask",
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
                    name: "usbsuspm",
                    description: Some(
                        "USB suspend mask",
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
                    name: "usbrst",
                    description: Some(
                        "USB reset mask",
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
                    name: "enumdnem",
                    description: Some(
                        "Enumeration done mask",
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
                    name: "isoodrpm",
                    description: Some(
                        "Isochronous OUT packet dropped interrupt mask",
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
                    name: "eopfm",
                    description: Some(
                        "End of periodic frame interrupt mask",
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
                    name: "epmism",
                    description: Some(
                        "Endpoint mismatch interrupt mask",
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
                    name: "iepint",
                    description: Some(
                        "IN endpoints interrupt mask",
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
                    name: "oepint",
                    description: Some(
                        "OUT endpoints interrupt mask",
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
                    name: "iisoixfrm",
                    description: Some(
                        "Incomplete isochronous IN transfer mask",
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
                    name: "ipxfrm_iisooxfrm",
                    description: Some(
                        "Incomplete periodic transfer mask (host mode) / Incomplete isochronous OUT transfer mask (device mode)",
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
                    name: "fsuspm",
                    description: Some(
                        "Data fetch suspended mask",
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
                    name: "rstde",
                    description: Some(
                        "Reset detected interrupt mask",
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
                    name: "prtim",
                    description: Some(
                        "Host port interrupt mask",
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
                Field {
                    name: "hcim",
                    description: Some(
                        "Host channels interrupt mask",
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
                    name: "ptxfem",
                    description: Some(
                        "Periodic TxFIFO empty mask",
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
                    name: "lpmintm",
                    description: Some(
                        "LPM interrupt mask",
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
                    name: "cidschgm",
                    description: Some(
                        "Connector ID status change mask",
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
                    name: "discint",
                    description: Some(
                        "Disconnect detected interrupt mask",
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
                    name: "srqim",
                    description: Some(
                        "Session request/new session detected interrupt mask",
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
                    name: "wuim",
                    description: Some(
                        "Resume/remote wakeup detected interrupt mask",
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
            name: "Gintsts",
            extends: None,
            description: Some(
                "Core interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmod",
                    description: Some(
                        "Current mode of operation",
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
                    name: "mmis",
                    description: Some(
                        "Mode mismatch interrupt",
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
                    name: "otgint",
                    description: Some(
                        "OTG interrupt",
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
                    name: "sof",
                    description: Some(
                        "Start of frame",
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
                    name: "rxflvl",
                    description: Some(
                        "RxFIFO non-empty",
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
                    name: "nptxfe",
                    description: Some(
                        "Non-periodic TxFIFO empty",
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
                    name: "ginakeff",
                    description: Some(
                        "Global IN non-periodic NAK effective",
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
                    name: "goutnakeff",
                    description: Some(
                        "Global OUT NAK effective",
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
                    name: "esusp",
                    description: Some(
                        "Early suspend",
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
                    name: "usbsusp",
                    description: Some(
                        "USB suspend",
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
                    name: "usbrst",
                    description: Some(
                        "USB reset",
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
                    name: "enumdne",
                    description: Some(
                        "Enumeration done",
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
                    name: "isoodrp",
                    description: Some(
                        "Isochronous OUT packet dropped interrupt",
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
                    name: "eopf",
                    description: Some(
                        "End of periodic frame interrupt",
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
                    name: "iepint",
                    description: Some(
                        "IN endpoint interrupt",
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
                    name: "oepint",
                    description: Some(
                        "OUT endpoint interrupt",
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
                    name: "iisoixfr",
                    description: Some(
                        "Incomplete isochronous IN transfer",
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
                    name: "ipxfr_incompisoout",
                    description: Some(
                        "Incomplete periodic transfer (host mode) / Incomplete isochronous OUT transfer (device mode)",
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
                    name: "datafsusp",
                    description: Some(
                        "Data fetch suspended",
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
                    name: "resetdet",
                    description: Some(
                        "Reset detected",
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
                    name: "hprtint",
                    description: Some(
                        "Host port interrupt",
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
                Field {
                    name: "hcint",
                    description: Some(
                        "Host channels interrupt",
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
                    name: "ptxfe",
                    description: Some(
                        "Periodic TxFIFO empty",
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
                    name: "cidschg",
                    description: Some(
                        "Connector ID status change",
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
                    name: "discint",
                    description: Some(
                        "Disconnect detected interrupt",
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
                    name: "srqint",
                    description: Some(
                        "Session request/new session detected interrupt",
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
                    name: "wkupint",
                    description: Some(
                        "Resume/remote wakeup detected interrupt",
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
            name: "Glpmcfg",
            extends: None,
            description: Some(
                "Core LPM configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpmen",
                    description: Some(
                        "LPM support enable",
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
                    name: "lpmack",
                    description: Some(
                        "LPM token acknowledge enable",
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
                    name: "besl",
                    description: Some(
                        "Best effort service latency",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "remwake",
                    description: Some(
                        "bRemoteWake value",
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
                    name: "l1ssen",
                    description: Some(
                        "L1 Shallow Sleep enable",
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
                    name: "beslthrs",
                    description: Some(
                        "BESL threshold",
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
                    name: "l1dsen",
                    description: Some(
                        "L1 deep sleep enable",
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
                    name: "lpmrst",
                    description: Some(
                        "LPM response",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slpsts",
                    description: Some(
                        "Port sleep status",
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
                    name: "l1rsmok",
                    description: Some(
                        "Sleep State Resume OK",
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
                    name: "lpmchidx",
                    description: Some(
                        "LPM Channel Index",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpmrcnt",
                    description: Some(
                        "LPM retry count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sndlpm",
                    description: Some(
                        "Send LPM transaction",
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
                Field {
                    name: "lpmrcntsts",
                    description: Some(
                        "LPM retry count status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enbesl",
                    description: Some(
                        "Enable best effort service latency",
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
            ],
        },
        FieldSet {
            name: "Gotgctl",
            extends: None,
            description: Some(
                "Control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srqscs",
                    description: Some(
                        "Session request success",
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
                    name: "srq",
                    description: Some(
                        "Session request",
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
                    name: "vbvaloen",
                    description: Some(
                        "VBUS valid override enable",
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
                    name: "vbvaloval",
                    description: Some(
                        "VBUS valid override value",
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
                    name: "avaloen",
                    description: Some(
                        "A-peripheral session valid override enable",
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
                    name: "avaloval",
                    description: Some(
                        "A-peripheral session valid override value",
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
                    name: "bvaloen",
                    description: Some(
                        "B-peripheral session valid override enable",
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
                    name: "bvaloval",
                    description: Some(
                        "B-peripheral session valid override value",
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
                    name: "hngscs",
                    description: Some(
                        "Host negotiation success",
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
                    name: "hnprq",
                    description: Some(
                        "HNP request",
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
                    name: "hshnpen",
                    description: Some(
                        "Host set HNP enable",
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
                    name: "dhnpen",
                    description: Some(
                        "Device HNP enabled",
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
                    name: "ehen",
                    description: Some(
                        "Embedded host enable",
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
                    name: "cidsts",
                    description: Some(
                        "Connector ID status",
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
                    name: "dbct",
                    description: Some(
                        "Long/short debounce time",
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
                    name: "asvld",
                    description: Some(
                        "A-session valid",
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
                    name: "bsvld",
                    description: Some(
                        "B-session valid",
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
            name: "Gotgint",
            extends: None,
            description: Some(
                "Interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sedet",
                    description: Some(
                        "Session end detected",
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
                    name: "srsschg",
                    description: Some(
                        "Session request success status change",
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
                    name: "hnsschg",
                    description: Some(
                        "Host negotiation success status change",
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
                    name: "hngdet",
                    description: Some(
                        "Host negotiation detected",
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
                    name: "adtochg",
                    description: Some(
                        "A-device timeout change",
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
                    name: "dbcdne",
                    description: Some(
                        "Debounce done",
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
                    name: "idchng",
                    description: Some(
                        "ID input pin changed",
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
            name: "Grstctl",
            extends: None,
            description: Some(
                "Reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csrst",
                    description: Some(
                        "Core soft reset",
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
                    name: "hsrst",
                    description: Some(
                        "HCLK soft reset",
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
                    name: "fcrst",
                    description: Some(
                        "Host frame counter reset",
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
                    name: "rxfflsh",
                    description: Some(
                        "RxFIFO flush",
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
                    name: "txfflsh",
                    description: Some(
                        "TxFIFO flush",
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
                    name: "txfnum",
                    description: Some(
                        "TxFIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmareq",
                    description: Some(
                        "DMA request signal enabled for USB OTG HS",
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
                    name: "ahbidl",
                    description: Some(
                        "AHB master idle",
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
            name: "Grxfsiz",
            extends: None,
            description: Some(
                "Receive FIFO size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfd",
                    description: Some(
                        "RxFIFO depth",
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
            name: "Grxsts",
            extends: None,
            description: Some(
                "Status read and pop register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epnum",
                    description: Some(
                        "Endpoint number (device mode) / Channel number (host mode)",
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
                    name: "bcnt",
                    description: Some(
                        "Byte count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dpid",
                    ),
                },
                Field {
                    name: "pktstsd",
                    description: Some(
                        "Packet status (device mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pktstsd",
                    ),
                },
                Field {
                    name: "pktstsh",
                    description: Some(
                        "Packet status (host mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pktstsh",
                    ),
                },
                Field {
                    name: "frmnum",
                    description: Some(
                        "Frame number (device mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gusbcfg",
            extends: None,
            description: Some(
                "USB configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tocal",
                    description: Some(
                        "FS timeout calibration",
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
                    name: "physel",
                    description: Some(
                        "Full-speed internal serial transceiver enable",
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
                    name: "srpcap",
                    description: Some(
                        "SRP-capable",
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
                    name: "hnpcap",
                    description: Some(
                        "HNP-capable",
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
                    name: "trdt",
                    description: Some(
                        "USB turnaround time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phylpcs",
                    description: Some(
                        "PHY Low-power clock select",
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
                    name: "ulpifsls",
                    description: Some(
                        "ULPI FS/LS select",
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
                    name: "ulpiar",
                    description: Some(
                        "ULPI Auto-resume",
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
                    name: "ulpicsm",
                    description: Some(
                        "ULPI Clock SuspendM",
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
                    name: "ulpievbusd",
                    description: Some(
                        "ULPI External VBUS Drive",
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
                    name: "ulpievbusi",
                    description: Some(
                        "ULPI external VBUS indicator",
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
                    name: "tsdps",
                    description: Some(
                        "TermSel DLine pulsing selection",
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
                    name: "pcci",
                    description: Some(
                        "Indicator complement",
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
                    name: "ptci",
                    description: Some(
                        "Indicator pass through",
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
                Field {
                    name: "ulpiipd",
                    description: Some(
                        "ULPI interface protect disable",
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
                    name: "fhmod",
                    description: Some(
                        "Force host mode",
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
                    name: "fdmod",
                    description: Some(
                        "Force device mode",
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
                    name: "ctxpkt",
                    description: Some(
                        "Corrupt Tx packet",
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
            name: "Haint",
            extends: None,
            description: Some(
                "Host all channels interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "haint",
                    description: Some(
                        "Channel interrupts",
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
            name: "Haintmsk",
            extends: None,
            description: Some(
                "Host all channels interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "haintm",
                    description: Some(
                        "Channel interrupt mask",
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
            name: "Hcchar",
            extends: None,
            description: Some(
                "Host channel characteristics register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "Maximum packet size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epnum",
                    description: Some(
                        "Endpoint number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdir",
                    description: Some(
                        "Endpoint direction",
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
                    name: "lsdev",
                    description: Some(
                        "Low-speed device",
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
                    name: "eptyp",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Eptyp",
                    ),
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multicount",
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
                    name: "dad",
                    description: Some(
                        "Device address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oddfrm",
                    description: Some(
                        "Odd frame (request iso/interrupt transaction to be performed on odd micro-frame)",
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
                    name: "chdis",
                    description: Some(
                        "Channel disable",
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
                    name: "chena",
                    description: Some(
                        "Channel enable",
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
            name: "Hcdma",
            extends: None,
            description: Some(
                "Host channel DMA config register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qtdaddr",
                    description: Some(
                        "QTD list base address",
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
                Field {
                    name: "cqtd",
                    description: Some(
                        "Current QTD (transfer descriptor) index",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hcfg",
            extends: None,
            description: Some(
                "Host configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fslspcs",
                    description: Some(
                        "FS/LS PHY clock select",
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
                    name: "fslss",
                    description: Some(
                        "FS- and LS-only support",
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
                    name: "descdma",
                    description: Some(
                        "Descriptor DMA-mode enable (qtd)",
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
                    name: "frlistlen",
                    description: Some(
                        "Frame list length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Frlistlen",
                    ),
                },
                Field {
                    name: "perschedena",
                    description: Some(
                        "Period scheduling enable",
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
            ],
        },
        FieldSet {
            name: "Hcint",
            extends: None,
            description: Some(
                "Host channel interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "Transfer completed",
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
                    name: "chh",
                    description: Some(
                        "Channel halted",
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
                    name: "stall",
                    description: Some(
                        "STALL response received interrupt",
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
                    name: "nak",
                    description: Some(
                        "NAK response received interrupt",
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
                    name: "ack",
                    description: Some(
                        "ACK response received/transmitted interrupt",
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
                    name: "txerr",
                    description: Some(
                        "Transaction error",
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
                    name: "bberr",
                    description: Some(
                        "Babble error",
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
                    name: "frmor",
                    description: Some(
                        "Frame overrun",
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
                    name: "dterr",
                    description: Some(
                        "Data toggle error",
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
            name: "Hcintmsk",
            extends: None,
            description: Some(
                "Host channel mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrcm",
                    description: Some(
                        "Transfer completed mask",
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
                    name: "chhm",
                    description: Some(
                        "Channel halted mask",
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
                    name: "stallm",
                    description: Some(
                        "STALL response received interrupt mask",
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
                    name: "nakm",
                    description: Some(
                        "NAK response received interrupt mask",
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
                    name: "ackm",
                    description: Some(
                        "ACK response received/transmitted interrupt mask",
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
                    name: "nyet",
                    description: Some(
                        "Response received interrupt mask",
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
                    name: "txerrm",
                    description: Some(
                        "Transaction error mask",
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
                    name: "bberrm",
                    description: Some(
                        "Babble error mask",
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
                    name: "frmorm",
                    description: Some(
                        "Frame overrun mask",
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
                    name: "dterrm",
                    description: Some(
                        "Data toggle error mask",
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
            name: "Hctsiz",
            extends: None,
            description: Some(
                "Host channel transfer size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "schedinfo",
                    description: Some(
                        "Schedule info for isochronuous & interrupt pipes (xfrsiz[7:0])",
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
                    name: "xfrsiz",
                    description: Some(
                        "Transfer size for non-isochronuous/interrupt pipes",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ntdl",
                    description: Some(
                        "NTD descriptor list length for isochronuous & interrupt pipes (xfrsiz[15:8], note val+1 is actual length)",
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
                    name: "pktcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "doping",
                    description: Some(
                        "Do Ping",
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
            name: "Hfir",
            extends: None,
            description: Some(
                "Host frame interval register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frivl",
                    description: Some(
                        "Frame interval",
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
                    name: "rldctrl",
                    description: Some(
                        "Dynamic Loading Control",
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
            name: "Hfnum",
            extends: None,
            description: Some(
                "Host frame number/frame time remaining register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frnum",
                    description: Some(
                        "Frame number",
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
                    name: "ftrem",
                    description: Some(
                        "Frame time remaining",
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
            name: "Hnptxsts",
            extends: None,
            description: Some(
                "Non-periodic transmit FIFO/queue status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nptxfsav",
                    description: Some(
                        "Non-periodic TxFIFO space available",
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
                    name: "nptqxsav",
                    description: Some(
                        "Non-periodic transmit request queue space available",
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
                    name: "nptxqtop",
                    description: Some(
                        "Top of the non-periodic transmit request queue",
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
            name: "Hprt",
            extends: None,
            description: Some(
                "Host port control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcsts",
                    description: Some(
                        "Port connect status",
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
                    name: "pcdet",
                    description: Some(
                        "Port connect detected",
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
                    name: "pena",
                    description: Some(
                        "Port enable (W1C)",
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
                    name: "penchng",
                    description: Some(
                        "Port enable/disable change",
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
                    name: "poca",
                    description: Some(
                        "Port overcurrent active",
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
                    name: "pocchng",
                    description: Some(
                        "Port overcurrent change",
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
                    name: "pres",
                    description: Some(
                        "Port resume",
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
                    name: "psusp",
                    description: Some(
                        "Port suspend",
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
                    name: "prst",
                    description: Some(
                        "Port reset",
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
                    name: "plsts",
                    description: Some(
                        "Port line status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppwr",
                    description: Some(
                        "Port power",
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
                    name: "ptctl",
                    description: Some(
                        "Port test control",
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
                    name: "pspd",
                    description: Some(
                        "Port speed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hptxsts",
            extends: None,
            description: Some(
                "Periodic transmit FIFO/queue status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptxfsavl",
                    description: Some(
                        "Periodic transmit data FIFO space available",
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
                    name: "ptxqsav",
                    description: Some(
                        "Periodic transmit request queue space available",
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
                    name: "ptxqtop",
                    description: Some(
                        "Top of the periodic transmit request queue",
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
            name: "Pcgcctl",
            extends: None,
            description: Some(
                "Power and clock gating control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stppclk",
                    description: Some(
                        "Stop PHY clock",
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
                    name: "gatehclk",
                    description: Some(
                        "Gate HCLK",
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
                    name: "physusp",
                    description: Some(
                        "PHY Suspended",
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
    ],
    enums: &[
        Enum {
            name: "Dpid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DATA0",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DATA2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DATA1",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "MDATA",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dspd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HIGH_SPEED",
                    description: Some(
                        "High speed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FULL_SPEED_EXTERNAL",
                    description: Some(
                        "Full speed using external ULPI PHY",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FULL_SPEED_INTERNAL",
                    description: Some(
                        "Full speed using internal embedded PHY",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Eptyp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CONTROL",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "ISOCHRONOUS",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "BULK",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "INTERRUPT",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Frlistlen",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LEN8",
                    description: Some(
                        "Length = 8",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEN16",
                    description: Some(
                        "Length = 16",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEN32",
                    description: Some(
                        "Length = 32",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEN64",
                    description: Some(
                        "Length = 64",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pfivl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FRAME_INTERVAL_80",
                    description: Some(
                        "80% of the frame interval",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FRAME_INTERVAL_85",
                    description: Some(
                        "85% of the frame interval",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FRAME_INTERVAL_90",
                    description: Some(
                        "90% of the frame interval",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FRAME_INTERVAL_95",
                    description: Some(
                        "95% of the frame interval",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pktstsd",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "OUT_NAK",
                    description: Some(
                        "Global OUT NAK (triggers an interrupt)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "OUT_DATA_RX",
                    description: Some(
                        "OUT data packet received",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "OUT_DATA_DONE",
                    description: Some(
                        "OUT transfer completed (triggers an interrupt)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SETUP_DATA_DONE",
                    description: Some(
                        "SETUP transaction completed (triggers an interrupt)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SETUP_DATA_RX",
                    description: Some(
                        "SETUP data packet received",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Pktstsh",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "IN_DATA_RX",
                    description: Some(
                        "IN data packet received",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "IN_DATA_DONE",
                    description: Some(
                        "IN transfer completed (triggers an interrupt)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DATA_TOGGLE_ERR",
                    description: Some(
                        "Data toggle error (triggers an interrupt)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CHANNEL_HALTED",
                    description: Some(
                        "Channel halted (triggers an interrupt)",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
