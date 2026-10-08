
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Eth",
            extends: None,
            description: Some(
                "Ethernet Peripheral",
            ),
            items: &[
                BlockItem {
                    name: "ethernet_mac",
                    description: Some(
                        "Ethernet: media access control (MAC)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetMac",
                        },
                    ),
                },
                BlockItem {
                    name: "ethernet_mtl",
                    description: Some(
                        "Ethernet: MTL mode register (MTL)",
                    ),
                    array: None,
                    byte_offset: 0xc00,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetMtl",
                        },
                    ),
                },
                BlockItem {
                    name: "ethernet_dma",
                    description: Some(
                        "Ethernet: DMA mode register (DMA)",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetDma",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "EthernetDma",
            extends: None,
            description: Some(
                "Ethernet: DMA mode register (DMA)",
            ),
            items: &[
                BlockItem {
                    name: "dmamr",
                    description: Some(
                        "DMA mode register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasbmr",
                    description: Some(
                        "System bus mode register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmasbmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaisr",
                    description: Some(
                        "Interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmadsr",
                    description: Some(
                        "Debug status register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmadsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaccr",
                    description: Some(
                        "Channel control register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactx_cr",
                    description: Some(
                        "Channel transmit control register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmactxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacrx_cr",
                    description: Some(
                        "Channel receive control register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacrxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactx_dlar",
                    description: Some(
                        "Channel Tx descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmactxDlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacrx_dlar",
                    description: Some(
                        "Channel Rx descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacrxDlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactx_dtpr",
                    description: Some(
                        "Channel Tx descriptor tail pointer register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmactxDtpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacrx_dtpr",
                    description: Some(
                        "Channel Rx descriptor tail pointer register",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacrxDtpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactx_rlr",
                    description: Some(
                        "Channel Tx descriptor ring length register",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmactxRlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacrx_rlr",
                    description: Some(
                        "Channel Rx descriptor ring length register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacrxRlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacier",
                    description: Some(
                        "Channel interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacrx_iwtr",
                    description: Some(
                        "Channel Rx interrupt watchdog timer register",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacrxIwtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaccatx_dr",
                    description: Some(
                        "Channel current application transmit descriptor register",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaccatxDr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaccarx_dr",
                    description: Some(
                        "Channel current application receive descriptor register",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaccarxDr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaccatx_br",
                    description: Some(
                        "Channel current application transmit buffer register",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaccatxBr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaccarx_br",
                    description: Some(
                        "Channel current application receive buffer register",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaccarxBr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacsr",
                    description: Some(
                        "Channel status register",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacmfcr",
                    description: Some(
                        "Channel missed frame count register",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacmfcr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "EthernetMac",
            extends: None,
            description: Some(
                "Ethernet: media access control (MAC)",
            ),
            items: &[
                BlockItem {
                    name: "maccr",
                    description: Some(
                        "Operating mode configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macecr",
                    description: Some(
                        "Extended operating mode configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macpfr",
                    description: Some(
                        "Packet filtering control register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macwtr",
                    description: Some(
                        "Watchdog timeout register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macwtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machtr",
                    description: Some(
                        "Hash Table 0/1 register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvtr",
                    description: Some(
                        "VLAN tag register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvhtr",
                    description: Some(
                        "VLAN Hash table register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvhtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvir",
                    description: Some(
                        "VLAN inclusion register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macivir",
                    description: Some(
                        "Inner VLAN inclusion register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macivir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macqtx_fcr",
                    description: Some(
                        "Tx Queue flow control register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacqtxFcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrx_fcr",
                    description: Some(
                        "Rx flow control register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacrxFcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macisr",
                    description: Some(
                        "Interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macier",
                    description: Some(
                        "Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrx_tx_sr",
                    description: Some(
                        "Rx Tx status register",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MacrxTxSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macpcsr",
                    description: Some(
                        "PMT control status register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrwkpfr",
                    description: Some(
                        "Remove wakeup packet filter register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macrwkpfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maclcsr",
                    description: Some(
                        "LPI control status register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maclcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macltcr",
                    description: Some(
                        "LPI timers control register",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macletr",
                    description: Some(
                        "LPI entry timer register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macletr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac1ustcr",
                    description: Some(
                        "1-microsecond-tick counter register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mac1ustcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvr",
                    description: Some(
                        "Version register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macdr",
                    description: Some(
                        "Debug register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machwf1r",
                    description: Some(
                        "HW feature 1 register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Machwf1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machwf2r",
                    description: Some(
                        "HW feature 2 register",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Machwf2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmdioar",
                    description: Some(
                        "MDIO address register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmdioar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmdiodr",
                    description: Some(
                        "MDIO data register",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmdiodr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0hr",
                    description: Some(
                        "Address 0 high register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0lr",
                    description: Some(
                        "Address 0 low register",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macahr",
                    description: Some(
                        "Address 1/2/3 high register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macahr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macalr",
                    description: Some(
                        "Address 1/2/3 low register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macalr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_control",
                    description: Some(
                        "MMC control register",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rx_interrupt",
                    description: Some(
                        "MMC Rx interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxInterrupt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_tx_interrupt",
                    description: Some(
                        "MMC Tx interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxInterrupt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rx_interrupt_mask",
                    description: Some(
                        "MMC Rx interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxInterruptMask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_tx_interrupt_mask",
                    description: Some(
                        "MMC Tx interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxInterruptMask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_single_collision_good_packets",
                    description: Some(
                        "Tx single collision good packets register",
                    ),
                    array: None,
                    byte_offset: 0x74c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TxSingleCollisionGoodPackets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_multiple_collision_good_packets",
                    description: Some(
                        "Tx multiple collision good packets register",
                    ),
                    array: None,
                    byte_offset: 0x750,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TxMultipleCollisionGoodPackets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_packet_count_good",
                    description: Some(
                        "Tx packet count good register",
                    ),
                    array: None,
                    byte_offset: 0x768,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TxPacketCountGood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_crc_error_packets",
                    description: Some(
                        "Rx CRC error packets register",
                    ),
                    array: None,
                    byte_offset: 0x794,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxCrcErrorPackets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_alignment_error_packets",
                    description: Some(
                        "Rx alignment error packets register",
                    ),
                    array: None,
                    byte_offset: 0x798,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxAlignmentErrorPackets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_unicast_packets_good",
                    description: Some(
                        "Rx unicast packets good register",
                    ),
                    array: None,
                    byte_offset: 0x7c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxUnicastPacketsGood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_lpi_usec_cntr",
                    description: Some(
                        "Tx LPI microsecond timer register",
                    ),
                    array: None,
                    byte_offset: 0x7ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TxLpiUsecCntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_lpi_tran_cntr",
                    description: Some(
                        "Tx LPI transition counter register",
                    ),
                    array: None,
                    byte_offset: 0x7f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TxLpiTranCntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_lpi_usec_cntr",
                    description: Some(
                        "Rx LPI microsecond counter register",
                    ),
                    array: None,
                    byte_offset: 0x7f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxLpiUsecCntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_lpi_tran_cntr",
                    description: Some(
                        "Rx LPI transition counter register",
                    ),
                    array: None,
                    byte_offset: 0x7f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RxLpiTranCntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3l4c0r",
                    description: Some(
                        "L3 and L4 control 0 register",
                    ),
                    array: None,
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3l4c0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl4a0r",
                    description: Some(
                        "Layer4 address filter 0 register",
                    ),
                    array: None,
                    byte_offset: 0x904,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl4a0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a00r",
                    description: Some(
                        "MACL3A00R",
                    ),
                    array: None,
                    byte_offset: 0x910,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a00r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a10r",
                    description: Some(
                        "Layer3 address 1 filter 0 register",
                    ),
                    array: None,
                    byte_offset: 0x914,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a10r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a20",
                    description: Some(
                        "Layer3 Address 2 filter 0 register",
                    ),
                    array: None,
                    byte_offset: 0x918,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a30",
                    description: Some(
                        "Layer3 Address 3 filter 0 register",
                    ),
                    array: None,
                    byte_offset: 0x91c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3l4c1r",
                    description: Some(
                        "L3 and L4 control 1 register",
                    ),
                    array: None,
                    byte_offset: 0x930,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3l4c1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl4a1r",
                    description: Some(
                        "Layer 4 address filter 1 register",
                    ),
                    array: None,
                    byte_offset: 0x934,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl4a1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a01r",
                    description: Some(
                        "Layer3 address 0 filter 1 Register",
                    ),
                    array: None,
                    byte_offset: 0x940,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a01r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a11r",
                    description: Some(
                        "Layer3 address 1 filter 1 register",
                    ),
                    array: None,
                    byte_offset: 0x944,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a11r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a21r",
                    description: Some(
                        "Layer3 address 2 filter 1 Register",
                    ),
                    array: None,
                    byte_offset: 0x948,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a21r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macl3a31r",
                    description: Some(
                        "Layer3 address 3 filter 1 register",
                    ),
                    array: None,
                    byte_offset: 0x94c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macl3a31r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macarpar",
                    description: Some(
                        "ARP address register",
                    ),
                    array: None,
                    byte_offset: 0xae0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macarpar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactscr",
                    description: Some(
                        "Timestamp control Register",
                    ),
                    array: None,
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macssir",
                    description: Some(
                        "Sub-second increment register",
                    ),
                    array: None,
                    byte_offset: 0xb04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macssir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macstsr",
                    description: Some(
                        "System time seconds register",
                    ),
                    array: None,
                    byte_offset: 0xb08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macstnr",
                    description: Some(
                        "System time nanoseconds register",
                    ),
                    array: None,
                    byte_offset: 0xb0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macstnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macstsur",
                    description: Some(
                        "System time seconds update register",
                    ),
                    array: None,
                    byte_offset: 0xb10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macstsur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macstnur",
                    description: Some(
                        "System time nanoseconds update register",
                    ),
                    array: None,
                    byte_offset: 0xb14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macstnur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactsar",
                    description: Some(
                        "Timestamp addend register",
                    ),
                    array: None,
                    byte_offset: 0xb18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactsar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactssr",
                    description: Some(
                        "Timestamp status register",
                    ),
                    array: None,
                    byte_offset: 0xb20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactx_tssnr",
                    description: Some(
                        "Tx timestamp status nanoseconds register",
                    ),
                    array: None,
                    byte_offset: 0xb30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MactxTssnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactx_tsssr",
                    description: Some(
                        "Tx timestamp status seconds register",
                    ),
                    array: None,
                    byte_offset: 0xb34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MactxTsssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macacr",
                    description: Some(
                        "Auxiliary control register",
                    ),
                    array: None,
                    byte_offset: 0xb40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macatsnr",
                    description: Some(
                        "Auxiliary timestamp nanoseconds register",
                    ),
                    array: None,
                    byte_offset: 0xb48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macatsnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macatssr",
                    description: Some(
                        "Auxiliary timestamp seconds register",
                    ),
                    array: None,
                    byte_offset: 0xb4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macatssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactsiacr",
                    description: Some(
                        "Timestamp Ingress asymmetric correction register",
                    ),
                    array: None,
                    byte_offset: 0xb50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactsiacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactseacr",
                    description: Some(
                        "Timestamp Egress asymmetric correction register",
                    ),
                    array: None,
                    byte_offset: 0xb54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactseacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactsicnr",
                    description: Some(
                        "Timestamp Ingress correction nanosecond register",
                    ),
                    array: None,
                    byte_offset: 0xb58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactsicnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mactsecnr",
                    description: Some(
                        "Timestamp Egress correction nanosecond register",
                    ),
                    array: None,
                    byte_offset: 0xb5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mactsecnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macppscr",
                    description: Some(
                        "PPS control register",
                    ),
                    array: None,
                    byte_offset: 0xb70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macppscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macppsttsr",
                    description: Some(
                        "PPS target time seconds register",
                    ),
                    array: None,
                    byte_offset: 0xb80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macppsttsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macppsttnr",
                    description: Some(
                        "PPS target time nanoseconds register",
                    ),
                    array: None,
                    byte_offset: 0xb84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macppsttnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macppsir",
                    description: Some(
                        "PPS interval register",
                    ),
                    array: None,
                    byte_offset: 0xb88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macppsir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macppswr",
                    description: Some(
                        "PPS width register",
                    ),
                    array: None,
                    byte_offset: 0xb8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macppswr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macpocr",
                    description: Some(
                        "PTP Offload control register",
                    ),
                    array: None,
                    byte_offset: 0xbc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macspi0r",
                    description: Some(
                        "PTP Source Port Identity 0 Register",
                    ),
                    array: None,
                    byte_offset: 0xbc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macspi0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macspi1r",
                    description: Some(
                        "PTP Source port identity 1 register",
                    ),
                    array: None,
                    byte_offset: 0xbc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macspi1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macspi2r",
                    description: Some(
                        "PTP Source port identity 2 register",
                    ),
                    array: None,
                    byte_offset: 0xbcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macspi2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maclmir",
                    description: Some(
                        "Log message interval register",
                    ),
                    array: None,
                    byte_offset: 0xbd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maclmir",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "EthernetMtl",
            extends: None,
            description: Some(
                "Ethernet: MTL mode register (MTL)",
            ),
            items: &[
                BlockItem {
                    name: "mtlomr",
                    description: Some(
                        "Operating mode Register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mtlomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtlisr",
                    description: Some(
                        "Interrupt status Register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mtlisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtltx_qomr",
                    description: Some(
                        "Tx queue operating mode Register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MtltxQomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtltx_qur",
                    description: Some(
                        "Tx queue underflow register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MtltxQur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtltx_qdr",
                    description: Some(
                        "Tx queue debug Register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MtltxQdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtlqicsr",
                    description: Some(
                        "Queue interrupt control status Register",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mtlqicsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtlrx_qomr",
                    description: Some(
                        "Rx queue operating mode register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MtlrxQomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtlrx_qmpocr",
                    description: Some(
                        "Rx queue missed packet and overflow counter register",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MtlrxQmpocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mtlrx_qdr",
                    description: Some(
                        "Rx queue debug register",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MtlrxQdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "DmaccarxBr",
            extends: None,
            description: Some(
                "Channel current application receive buffer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "currbufaptr",
                    description: Some(
                        "Application Receive Buffer Address Pointer",
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
            name: "DmaccarxDr",
            extends: None,
            description: Some(
                "Channel current application receive descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "currdesaptr",
                    description: Some(
                        "Application Receive Descriptor Address Pointer",
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
            name: "DmaccatxBr",
            extends: None,
            description: Some(
                "Channel current application transmit buffer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "curtbufaptr",
                    description: Some(
                        "Application Transmit Buffer Address Pointer",
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
            name: "DmaccatxDr",
            extends: None,
            description: Some(
                "Channel current application transmit descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "curtdesaptr",
                    description: Some(
                        "Application Transmit Descriptor Address Pointer",
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
            name: "Dmaccr",
            extends: None,
            description: Some(
                "Channel control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mss",
                    description: Some(
                        "Maximum Segment Size",
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
                Field {
                    name: "pblx8",
                    description: Some(
                        "8xPBL mode",
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
                    name: "dsl",
                    description: Some(
                        "Descriptor Skip Length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmacier",
            extends: None,
            description: Some(
                "Channel interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmit Interrupt Enable",
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
                    name: "txse",
                    description: Some(
                        "Transmit Stopped Enable",
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
                    name: "tbue",
                    description: Some(
                        "Transmit Buffer Unavailable Enable",
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
                    name: "rie",
                    description: Some(
                        "Receive Interrupt Enable",
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
                    name: "rbue",
                    description: Some(
                        "Receive Buffer Unavailable Enable",
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
                    name: "rse",
                    description: Some(
                        "Receive Stopped Enable",
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
                    name: "rwte",
                    description: Some(
                        "Receive Watchdog Timeout Enable",
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
                    name: "etie",
                    description: Some(
                        "Early Transmit Interrupt Enable",
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
                    name: "erie",
                    description: Some(
                        "Early Receive Interrupt Enable",
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
                    name: "fbee",
                    description: Some(
                        "Fatal Bus Error Enable",
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
                    name: "cdee",
                    description: Some(
                        "Context Descriptor Error Enable",
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
                    name: "aie",
                    description: Some(
                        "Abnormal Interrupt Summary Enable",
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
                    name: "nie",
                    description: Some(
                        "Normal Interrupt Summary Enable",
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
            ],
        },
        FieldSet {
            name: "Dmacmfcr",
            extends: None,
            description: Some(
                "Channel missed frame count register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfc",
                    description: Some(
                        "Dropped Packet Counters",
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
                    name: "mfco",
                    description: Some(
                        "Overflow status of the MFC Counter",
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
            ],
        },
        FieldSet {
            name: "DmacrxCr",
            extends: None,
            description: Some(
                "Channel receive control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Start or Stop Receive Command",
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
                    name: "rbsz",
                    description: Some(
                        "Receive Buffer size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxpbl",
                    description: Some(
                        "RXPBL",
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
                    name: "rpf",
                    description: Some(
                        "DMA Rx Channel Packet Flush",
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
            name: "DmacrxDlar",
            extends: None,
            description: Some(
                "Channel Rx descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdesla",
                    description: Some(
                        "Start of Receive List",
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
            name: "DmacrxDtpr",
            extends: None,
            description: Some(
                "Channel Rx descriptor tail pointer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdt",
                    description: Some(
                        "Receive Descriptor Tail Pointer",
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
            name: "DmacrxIwtr",
            extends: None,
            description: Some(
                "Channel Rx interrupt watchdog timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwt",
                    description: Some(
                        "Receive Interrupt Watchdog Timer Count",
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
            name: "DmacrxRlr",
            extends: None,
            description: Some(
                "Channel Rx descriptor ring length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdrl",
                    description: Some(
                        "Receive Descriptor Ring Length",
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
            name: "Dmacsr",
            extends: None,
            description: Some(
                "Channel status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ti",
                    description: Some(
                        "Transmit Interrupt",
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
                    name: "tps",
                    description: Some(
                        "Transmit Process Stopped",
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
                    name: "tbu",
                    description: Some(
                        "Transmit Buffer Unavailable",
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
                    name: "ri",
                    description: Some(
                        "Receive Interrupt",
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
                    name: "rbu",
                    description: Some(
                        "Receive Buffer Unavailable",
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
                    name: "rps",
                    description: Some(
                        "Receive Process Stopped",
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
                    name: "rwt",
                    description: Some(
                        "Receive Watchdog Timeout",
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
                    name: "et",
                    description: Some(
                        "Early Transmit Interrupt",
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
                    name: "er",
                    description: Some(
                        "Early Receive Interrupt",
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
                    name: "fbe",
                    description: Some(
                        "Fatal Bus Error",
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
                    name: "cde",
                    description: Some(
                        "Context Descriptor Error",
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
                    name: "ais",
                    description: Some(
                        "Abnormal Interrupt Summary",
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
                    name: "nis",
                    description: Some(
                        "Normal Interrupt Summary",
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
                    name: "teb",
                    description: Some(
                        "Tx DMA Error Bits",
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
                    name: "reb",
                    description: Some(
                        "Rx DMA Error Bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmactxCr",
            extends: None,
            description: Some(
                "Channel transmit control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st",
                    description: Some(
                        "Start or Stop Transmission Command",
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
                    name: "osf",
                    description: Some(
                        "Operate on Second Packet",
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
                    name: "tse",
                    description: Some(
                        "TCP Segmentation Enabled",
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
                    name: "txpbl",
                    description: Some(
                        "Transmit Programmable Burst Length",
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
            ],
        },
        FieldSet {
            name: "DmactxDlar",
            extends: None,
            description: Some(
                "Channel Tx descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdesla",
                    description: Some(
                        "Start of Transmit List",
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
            name: "DmactxDtpr",
            extends: None,
            description: Some(
                "Channel Tx descriptor tail pointer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdt",
                    description: Some(
                        "Transmit Descriptor Tail Pointer",
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
            name: "DmactxRlr",
            extends: None,
            description: Some(
                "Channel Tx descriptor ring length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdrl",
                    description: Some(
                        "Transmit Descriptor Ring Length",
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
            name: "Dmadsr",
            extends: None,
            description: Some(
                "Debug status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "axwhsts",
                    description: Some(
                        "AHB Master Write Channel",
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
                    name: "rps0",
                    description: Some(
                        "DMA Channel Receive Process State",
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
                    name: "tps0",
                    description: Some(
                        "DMA Channel Transmit Process State",
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
            ],
        },
        FieldSet {
            name: "Dmaisr",
            extends: None,
            description: Some(
                "Interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dc0is",
                    description: Some(
                        "DMA Channel Interrupt Status",
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
                    name: "mtlis",
                    description: Some(
                        "MTL Interrupt Status",
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
                    name: "macis",
                    description: Some(
                        "MAC Interrupt Status",
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
            ],
        },
        FieldSet {
            name: "Dmamr",
            extends: None,
            description: Some(
                "DMA mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swr",
                    description: Some(
                        "Software Reset",
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
                    name: "da",
                    description: Some(
                        "DMA Tx or Rx Arbitration Scheme",
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
                    name: "txpr",
                    description: Some(
                        "Transmit priority",
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
                    name: "pr",
                    description: Some(
                        "Priority ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "intm",
                    description: Some(
                        "Interrupt Mode",
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
            name: "Dmasbmr",
            extends: None,
            description: Some(
                "System bus mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fb",
                    description: Some(
                        "Fixed Burst Length",
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
                    name: "aal",
                    description: Some(
                        "Address-Aligned Beats",
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
                    name: "mb",
                    description: Some(
                        "Mixed Burst",
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
                    name: "rb",
                    description: Some(
                        "Rebuild INCRx Burst",
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
            ],
        },
        FieldSet {
            name: "Mac1ustcr",
            extends: None,
            description: Some(
                "1-microsecond-tick counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tic_1us_cntr",
                    description: Some(
                        "1 µs tick Counter",
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
            name: "Maca0hr",
            extends: None,
            description: Some(
                "Address 0 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrhi",
                    description: Some(
                        "MAC Address0[47:32]",
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
                    name: "ae",
                    description: Some(
                        "Address Enable",
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
            name: "Maca0lr",
            extends: None,
            description: Some(
                "Address 0 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrlo",
                    description: Some(
                        "MAC Address 0 [31:0]",
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
            name: "Macacr",
            extends: None,
            description: Some(
                "Auxiliary control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "atsfc",
                    description: Some(
                        "Auxiliary Snapshot FIFO Clear",
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
                    name: "atsen",
                    description: Some(
                        "Auxiliary Snapshot 0-3 Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
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
            name: "Macahr",
            extends: None,
            description: Some(
                "Address 1/2/3 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrhi",
                    description: Some(
                        "MAC Address 1/2/3 [47:32]",
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
                    name: "mbc",
                    description: Some(
                        "Mask Byte Control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source Address",
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
                    name: "ae",
                    description: Some(
                        "Address Enable",
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
            name: "Macalr",
            extends: None,
            description: Some(
                "Address 1/2/3 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrlo",
                    description: Some(
                        "MAC Address 1/2/3 [31:0]",
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
            name: "Macarpar",
            extends: None,
            description: Some(
                "ARP address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arppa",
                    description: Some(
                        "ARP Protocol Address",
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
            name: "Macatsnr",
            extends: None,
            description: Some(
                "Auxiliary timestamp nanoseconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auxtslo",
                    description: Some(
                        "Auxiliary Timestamp",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macatssr",
            extends: None,
            description: Some(
                "Auxiliary timestamp seconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auxtshi",
                    description: Some(
                        "Auxiliary Timestamp",
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
            name: "Maccr",
            extends: None,
            description: Some(
                "Operating mode configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Receiver Enable",
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
                    name: "te",
                    description: Some(
                        "Transmitter Enable",
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
                    name: "prelen",
                    description: Some(
                        "Preamble Length for Transmit Packets",
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
                    name: "dc",
                    description: Some(
                        "Deferral Check",
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
                    name: "bl",
                    description: Some(
                        "Back-Off Limit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dr",
                    description: Some(
                        "Disable Retry",
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
                    name: "dcrs",
                    description: Some(
                        "Disable Carrier Sense During Transmission",
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
                    name: "do_",
                    description: Some(
                        "Disable Receive Own",
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
                    name: "ecrsfd",
                    description: Some(
                        "Enable Carrier Sense Before Transmission in Full-Duplex Mode",
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
                    name: "lm",
                    description: Some(
                        "Loopback Mode",
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
                    name: "dm",
                    description: Some(
                        "Duplex Mode",
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
                    name: "fes",
                    description: Some(
                        "MAC Speed",
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
                    name: "je",
                    description: Some(
                        "Jumbo Packet Enable",
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
                    name: "jd",
                    description: Some(
                        "Jabber Disable",
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
                    name: "wd",
                    description: Some(
                        "Watchdog Disable",
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
                    name: "acs",
                    description: Some(
                        "Automatic Pad or CRC Stripping",
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
                    name: "cst",
                    description: Some(
                        "CRC stripping for Type packets",
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
                    name: "s2kp",
                    description: Some(
                        "IEEE 802.3as Support for 2K Packets",
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
                    name: "gpslce",
                    description: Some(
                        "Giant Packet Size Limit Control Enable",
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
                    name: "ipg",
                    description: Some(
                        "Inter-Packet Gap",
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
                    name: "ipc",
                    description: Some(
                        "Checksum Offload",
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
                    name: "sarc",
                    description: Some(
                        "Source Address Insertion or Replacement Control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arpen",
                    description: Some(
                        "ARP Offload Enable",
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
            name: "Macdr",
            extends: None,
            description: Some(
                "Debug register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rpests",
                    description: Some(
                        "MAC MII Receive Protocol Engine Status",
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
                    name: "rfcfcsts",
                    description: Some(
                        "MAC Receive Packet Controller FIFO Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpests",
                    description: Some(
                        "MAC MII Transmit Protocol Engine Status",
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
                    name: "tfcsts",
                    description: Some(
                        "MAC Transmit Packet Controller Status",
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
            name: "Macecr",
            extends: None,
            description: Some(
                "Extended operating mode configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpsl",
                    description: Some(
                        "Giant Packet Size Limit",
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
                Field {
                    name: "dcrcc",
                    description: Some(
                        "Disable CRC Checking for Received Packets",
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
                    name: "spen",
                    description: Some(
                        "Slow Protocol Detection Enable",
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
                    name: "usp",
                    description: Some(
                        "Unicast Slow Protocol Packet Detect",
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
                    name: "eipgen",
                    description: Some(
                        "Extended Inter-Packet Gap Enable",
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
                    name: "eipg",
                    description: Some(
                        "Extended Inter-Packet Gap",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machtr",
            extends: None,
            description: Some(
                "Hash Table 0/1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ht",
                    description: Some(
                        "MAC Hash Table 32 Bits",
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
            name: "Machwf1r",
            extends: None,
            description: Some(
                "HW feature 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfifosize",
                    description: Some(
                        "MTL Receive FIFO Size",
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
                    name: "txfifosize",
                    description: Some(
                        "MTL Transmit FIFO Size",
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
                    name: "osten",
                    description: Some(
                        "One-Step Timestamping Enable",
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
                    name: "ptoen",
                    description: Some(
                        "PTP Offload Enable",
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
                    name: "advthword",
                    description: Some(
                        "IEEE 1588 High Word Register Enable",
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
                    name: "addr64",
                    description: Some(
                        "Address width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcben",
                    description: Some(
                        "DCB Feature Enable",
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
                    name: "sphen",
                    description: Some(
                        "Split Header Feature Enable",
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
                    name: "tsoen",
                    description: Some(
                        "TCP Segmentation Offload Enable",
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
                    name: "dbgmema",
                    description: Some(
                        "DMA Debug Registers Enable",
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
                    name: "avsel",
                    description: Some(
                        "AV Feature Enable",
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
                    name: "hashtblsz",
                    description: Some(
                        "Hash Table Size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3l4fnum",
                    description: Some(
                        "Total number of L3 or L4 Filters",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machwf2r",
            extends: None,
            description: Some(
                "HW feature 2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxqcnt",
                    description: Some(
                        "Number of MTL Receive Queues",
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
                    name: "txqcnt",
                    description: Some(
                        "Number of MTL Transmit Queues",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxchcnt",
                    description: Some(
                        "Number of DMA Receive Channels",
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
                    name: "txchcnt",
                    description: Some(
                        "Number of DMA Transmit Channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppsoutnum",
                    description: Some(
                        "Number of PPS Outputs",
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
                    name: "auxsnapnum",
                    description: Some(
                        "Number of Auxiliary Snapshot Inputs",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macier",
            extends: None,
            description: Some(
                "Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phyie",
                    description: Some(
                        "PHY Interrupt Enable",
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
                    name: "pmtie",
                    description: Some(
                        "PMT Interrupt Enable",
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
                    name: "lpiie",
                    description: Some(
                        "LPI Interrupt Enable",
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
                    name: "tsie",
                    description: Some(
                        "Timestamp Interrupt Enable",
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
                    name: "txstsie",
                    description: Some(
                        "Transmit Status Interrupt Enable",
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
                    name: "rxstsie",
                    description: Some(
                        "Receive Status Interrupt Enable",
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
            name: "Macisr",
            extends: None,
            description: Some(
                "Interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phyis",
                    description: Some(
                        "PHY Interrupt",
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
                    name: "pmtis",
                    description: Some(
                        "PMT Interrupt Status",
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
                    name: "lpiis",
                    description: Some(
                        "LPI Interrupt Status",
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
                    name: "mmcis",
                    description: Some(
                        "MMC Interrupt Status",
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
                    name: "mmcrxis",
                    description: Some(
                        "MMC Receive Interrupt Status",
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
                    name: "mmctxis",
                    description: Some(
                        "MMC Transmit Interrupt Status",
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
                    name: "tsis",
                    description: Some(
                        "Timestamp Interrupt Status",
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
                    name: "txstsis",
                    description: Some(
                        "Transmit Status Interrupt",
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
                    name: "rxstsis",
                    description: Some(
                        "Receive Status Interrupt",
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
            name: "Macivir",
            extends: None,
            description: Some(
                "Inner VLAN inclusion register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlt",
                    description: Some(
                        "VLAN Tag for Transmit Packets",
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
                    name: "vlc",
                    description: Some(
                        "VLAN Tag Control in Transmit Packets",
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
                Field {
                    name: "vlp",
                    description: Some(
                        "VLAN Priority Control",
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
                    name: "csvl",
                    description: Some(
                        "C-VLAN or S-VLAN",
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
                    name: "vlti",
                    description: Some(
                        "VLAN Tag Input",
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
            name: "Macl3a00r",
            extends: None,
            description: Some(
                "MACL3A00R",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a00",
                    description: Some(
                        "Layer 3 Address 0 Field",
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
            name: "Macl3a01r",
            extends: None,
            description: Some(
                "Layer3 address 0 filter 1 Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a01",
                    description: Some(
                        "Layer 3 Address 0 Field",
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
            name: "Macl3a10r",
            extends: None,
            description: Some(
                "Layer3 address 1 filter 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a10",
                    description: Some(
                        "Layer 3 Address 1 Field",
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
            name: "Macl3a11r",
            extends: None,
            description: Some(
                "Layer3 address 1 filter 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a11",
                    description: Some(
                        "Layer 3 Address 1 Field",
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
            name: "Macl3a20",
            extends: None,
            description: Some(
                "Layer3 Address 2 filter 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a20",
                    description: Some(
                        "Layer 3 Address 2 Field",
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
            name: "Macl3a21r",
            extends: None,
            description: Some(
                "Layer3 address 2 filter 1 Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a21",
                    description: Some(
                        "Layer 3 Address 2 Field",
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
            name: "Macl3a30",
            extends: None,
            description: Some(
                "Layer3 Address 3 filter 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a30",
                    description: Some(
                        "Layer 3 Address 3 Field",
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
            name: "Macl3a31r",
            extends: None,
            description: Some(
                "Layer3 address 3 filter 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3a31",
                    description: Some(
                        "Layer 3 Address 3 Field",
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
            name: "Macl3l4c0r",
            extends: None,
            description: Some(
                "L3 and L4 control 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3pen0",
                    description: Some(
                        "Layer 3 Protocol Enable",
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
                    name: "l3sam0",
                    description: Some(
                        "Layer 3 IP SA Match Enable",
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
                    name: "l3saim0",
                    description: Some(
                        "Layer 3 IP SA Inverse Match Enable",
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
                    name: "l3dam0",
                    description: Some(
                        "Layer 3 IP DA Match Enable",
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
                    name: "l3daim0",
                    description: Some(
                        "Layer 3 IP DA Inverse Match Enable",
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
                    name: "l3hsbm0",
                    description: Some(
                        "Layer 3 IP SA Higher Bits Match",
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
                    name: "l3hdbm0",
                    description: Some(
                        "Layer 3 IP DA Higher Bits Match",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4pen0",
                    description: Some(
                        "Layer 4 Protocol Enable",
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
                    name: "l4spm0",
                    description: Some(
                        "Layer 4 Source Port Match Enable",
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
                    name: "l4spim0",
                    description: Some(
                        "Layer 4 Source Port Inverse Match Enable",
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
                    name: "l4dpm0",
                    description: Some(
                        "Layer 4 Destination Port Match Enable",
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
                    name: "l4dpim0",
                    description: Some(
                        "Layer 4 Destination Port Inverse Match Enable",
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
            name: "Macl3l4c1r",
            extends: None,
            description: Some(
                "L3 and L4 control 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3pen1",
                    description: Some(
                        "Layer 3 Protocol Enable",
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
                    name: "l3sam1",
                    description: Some(
                        "Layer 3 IP SA Match Enable",
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
                    name: "l3saim1",
                    description: Some(
                        "Layer 3 IP SA Inverse Match Enable",
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
                    name: "l3dam1",
                    description: Some(
                        "Layer 3 IP DA Match Enable",
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
                    name: "l3daim1",
                    description: Some(
                        "Layer 3 IP DA Inverse Match Enable",
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
                    name: "l3hsbm1",
                    description: Some(
                        "Layer 3 IP SA Higher Bits Match",
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
                    name: "l3hdbm1",
                    description: Some(
                        "Layer 3 IP DA Higher Bits Match",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4pen1",
                    description: Some(
                        "Layer 4 Protocol Enable",
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
                    name: "l4spm1",
                    description: Some(
                        "Layer 4 Source Port Match Enable",
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
                    name: "l4spim1",
                    description: Some(
                        "Layer 4 Source Port Inverse Match Enable",
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
                    name: "l4dpm1",
                    description: Some(
                        "Layer 4 Destination Port Match Enable",
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
                    name: "l4dpim1",
                    description: Some(
                        "Layer 4 Destination Port Inverse Match Enable",
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
            name: "Macl4a0r",
            extends: None,
            description: Some(
                "Layer4 address filter 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l4sp0",
                    description: Some(
                        "Layer 4 Source Port Number Field",
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
                    name: "l4dp0",
                    description: Some(
                        "Layer 4 Destination Port Number Field",
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
            name: "Macl4a1r",
            extends: None,
            description: Some(
                "Layer 4 address filter 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l4sp1",
                    description: Some(
                        "Layer 4 Source Port Number Field",
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
                    name: "l4dp1",
                    description: Some(
                        "Layer 4 Destination Port Number Field",
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
            name: "Maclcsr",
            extends: None,
            description: Some(
                "LPI control status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlpien",
                    description: Some(
                        "Transmit LPI Entry",
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
                    name: "tlpiex",
                    description: Some(
                        "Transmit LPI Exit",
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
                    name: "rlpien",
                    description: Some(
                        "Receive LPI Entry",
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
                    name: "rlpiex",
                    description: Some(
                        "Receive LPI Exit",
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
                    name: "tlpist",
                    description: Some(
                        "Transmit LPI State",
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
                    name: "rlpist",
                    description: Some(
                        "Receive LPI State",
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
                    name: "lpien",
                    description: Some(
                        "LPI Enable",
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
                    name: "pls",
                    description: Some(
                        "PHY Link Status",
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
                    name: "plsen",
                    description: Some(
                        "PHY Link Status Enable",
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
                    name: "lpitxa",
                    description: Some(
                        "LPI Tx Automate",
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
                    name: "lpite",
                    description: Some(
                        "LPI Timer Enable",
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
            name: "Macletr",
            extends: None,
            description: Some(
                "LPI entry timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpiet",
                    description: Some(
                        "LPI Entry Timer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maclmir",
            extends: None,
            description: Some(
                "Log message interval register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi",
                    description: Some(
                        "Log Sync Interval",
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
                    name: "drsyncr",
                    description: Some(
                        "Delay_Req to SYNC Ratio",
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
                    name: "lmpdri",
                    description: Some(
                        "Log Min Pdelay_Req Interval",
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
            name: "Macltcr",
            extends: None,
            description: Some(
                "LPI timers control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "twt",
                    description: Some(
                        "LPI TW Timer",
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
                    name: "lst",
                    description: Some(
                        "LPI LS Timer",
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
            name: "Macmdioar",
            extends: None,
            description: Some(
                "MDIO address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mb",
                    description: Some(
                        "MII Busy",
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
                    name: "c45e",
                    description: Some(
                        "Clause 45 PHY Enable",
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
                    name: "goc",
                    description: Some(
                        "MII Operation Command",
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
                    name: "skap",
                    description: Some(
                        "Skip Address Packet",
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
                    name: "cr",
                    description: Some(
                        "CSR Clock Range",
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
                    name: "ntc",
                    description: Some(
                        "Number of Training Clocks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rda",
                    description: Some(
                        "Register/Device Address",
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
                    name: "pa",
                    description: Some(
                        "Physical Layer Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btb",
                    description: Some(
                        "Back to Back transactions",
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
                    name: "pse",
                    description: Some(
                        "Preamble Suppression Enable",
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
            name: "Macmdiodr",
            extends: None,
            description: Some(
                "MDIO data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md",
                    description: Some(
                        "MII Data",
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
                    name: "ra",
                    description: Some(
                        "Register Address",
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
            name: "Macpcsr",
            extends: None,
            description: Some(
                "PMT control status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrdwn",
                    description: Some(
                        "Power Down",
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
                    name: "mgkpkten",
                    description: Some(
                        "Magic Packet Enable",
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
                    name: "rwkpkten",
                    description: Some(
                        "Remote wakeup Packet Enable",
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
                    name: "mgkprcvd",
                    description: Some(
                        "Magic Packet Received",
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
                    name: "rwkprcvd",
                    description: Some(
                        "Remote wakeup Packet Received",
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
                    name: "glblucast",
                    description: Some(
                        "Global Unicast",
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
                    name: "rwkpfe",
                    description: Some(
                        "Remote wakeup Packet Forwarding Enable",
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
                    name: "rwkptr",
                    description: Some(
                        "Remote wakeup FIFO Pointer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwkfiltrst",
                    description: Some(
                        "Remote wakeup Packet Filter Register Pointer Reset",
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
            name: "Macpfr",
            extends: None,
            description: Some(
                "Packet filtering control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pr",
                    description: Some(
                        "Promiscuous Mode",
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
                    name: "huc",
                    description: Some(
                        "Hash Unicast",
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
                    name: "hmc",
                    description: Some(
                        "Hash Multicast",
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
                    name: "daif",
                    description: Some(
                        "DA Inverse Filtering",
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
                    name: "pm",
                    description: Some(
                        "Pass All Multicast",
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
                    name: "dbf",
                    description: Some(
                        "Disable Broadcast Packets",
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
                    name: "pcf",
                    description: Some(
                        "Pass Control Packets",
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
                    name: "saif",
                    description: Some(
                        "SA Inverse Filtering",
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
                    name: "saf",
                    description: Some(
                        "Source Address Filter Enable",
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
                    name: "hpf",
                    description: Some(
                        "Hash or Perfect Filter",
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
                    name: "vtfe",
                    description: Some(
                        "VLAN Tag Filter Enable",
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
                    name: "ipfe",
                    description: Some(
                        "Layer 3 and Layer 4 Filter Enable",
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
                    name: "dntu",
                    description: Some(
                        "Drop Non-TCP/UDP over IP Packets",
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
                    name: "ra",
                    description: Some(
                        "Receive All",
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
            name: "Macpocr",
            extends: None,
            description: Some(
                "PTP Offload control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptoen",
                    description: Some(
                        "PTP Offload Enable",
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
                    name: "asyncen",
                    description: Some(
                        "Automatic PTP SYNC message Enable",
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
                    name: "apdreqen",
                    description: Some(
                        "Automatic PTP Pdelay_Req message Enable",
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
                    name: "asynctrig",
                    description: Some(
                        "Automatic PTP SYNC message Trigger",
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
                    name: "apdreqtrig",
                    description: Some(
                        "Automatic PTP Pdelay_Req message Trigger",
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
                    name: "drrdis",
                    description: Some(
                        "Disable PTO Delay Request/Response response generation",
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
                    name: "dn",
                    description: Some(
                        "Domain Number",
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
            name: "Macppscr",
            extends: None,
            description: Some(
                "PPS control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsctrl",
                    description: Some(
                        "Flexible PPS Output (ptp_pps_o[0]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared",
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
                    name: "ppsen0",
                    description: Some(
                        "Flexible PPS Output Mode Enable",
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
                    name: "trgtmodsel0",
                    description: Some(
                        "Target Time Register Mode for PPS Output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macppsir",
            extends: None,
            description: Some(
                "PPS interval register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsint0",
                    description: Some(
                        "PPS Output Signal Interval",
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
            name: "Macppsttnr",
            extends: None,
            description: Some(
                "PPS target time nanoseconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ttsl0",
                    description: Some(
                        "Target Time Low for PPS Register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trgtbusy0",
                    description: Some(
                        "PPS Target Time Register Busy",
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
            name: "Macppsttsr",
            extends: None,
            description: Some(
                "PPS target time seconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tstrh0",
                    description: Some(
                        "PPS Target Time Seconds Register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macppswr",
            extends: None,
            description: Some(
                "PPS width register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppswidth0",
                    description: Some(
                        "PPS Output Signal Width",
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
            name: "MacqtxFcr",
            extends: None,
            description: Some(
                "Tx Queue flow control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fcb_bpa",
                    description: Some(
                        "Flow Control Busy or Backpressure Activate",
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
                    name: "tfe",
                    description: Some(
                        "Transmit Flow Control Enable",
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
                    name: "plt",
                    description: Some(
                        "Pause Low Threshold",
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
                    name: "dzpq",
                    description: Some(
                        "Disable Zero-Quanta Pause",
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
                    name: "pt",
                    description: Some(
                        "Pause Time",
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
            name: "Macrwkpfr",
            extends: None,
            description: Some(
                "Remove wakeup packet filter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "macrwkpfr",
                    description: Some(
                        "Remote wakeup packet filter",
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
            name: "MacrxFcr",
            extends: None,
            description: Some(
                "Rx flow control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfe",
                    description: Some(
                        "Receive Flow Control Enable",
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
                    name: "up",
                    description: Some(
                        "Unicast Pause Packet Detect",
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
            name: "MacrxTxSr",
            extends: None,
            description: Some(
                "Rx Tx status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tjt",
                    description: Some(
                        "Transmit Jabber Timeout",
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
                    name: "ncarr",
                    description: Some(
                        "No Carrier",
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
                    name: "lcarr",
                    description: Some(
                        "Loss of Carrier",
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
                    name: "exdef",
                    description: Some(
                        "Excessive Deferral",
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
                    name: "lcol",
                    description: Some(
                        "Late Collision",
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
                    name: "excol",
                    description: Some(
                        "Excessive Collisions",
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
                    name: "rwt",
                    description: Some(
                        "Receive Watchdog Timeout",
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
            name: "Macspi0r",
            extends: None,
            description: Some(
                "PTP Source Port Identity 0 Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi0",
                    description: Some(
                        "Source Port Identity 0",
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
            name: "Macspi1r",
            extends: None,
            description: Some(
                "PTP Source port identity 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1",
                    description: Some(
                        "Source Port Identity 1",
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
            name: "Macspi2r",
            extends: None,
            description: Some(
                "PTP Source port identity 2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi2",
                    description: Some(
                        "Source Port Identity 2",
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
            name: "Macssir",
            extends: None,
            description: Some(
                "Sub-second increment register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsinc",
                    description: Some(
                        "Sub-nanosecond Increment Value",
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
                    name: "ssinc",
                    description: Some(
                        "Sub-second Increment Value",
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
            name: "Macstnr",
            extends: None,
            description: Some(
                "System time nanoseconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsss",
                    description: Some(
                        "Timestamp Sub-seconds",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macstnur",
            extends: None,
            description: Some(
                "System time nanoseconds update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsss",
                    description: Some(
                        "Timestamp Sub-seconds",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addsub",
                    description: Some(
                        "Add or Subtract Time",
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
            name: "Macstsr",
            extends: None,
            description: Some(
                "System time seconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tss",
                    description: Some(
                        "Timestamp Second",
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
            name: "Macstsur",
            extends: None,
            description: Some(
                "System time seconds update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tss",
                    description: Some(
                        "Timestamp Seconds",
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
            name: "Mactsar",
            extends: None,
            description: Some(
                "Timestamp addend register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsar",
                    description: Some(
                        "Timestamp Addend Register",
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
            name: "Mactscr",
            extends: None,
            description: Some(
                "Timestamp control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsena",
                    description: Some(
                        "Enable Timestamp",
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
                    name: "tscfupdt",
                    description: Some(
                        "Fine or Coarse Timestamp Update",
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
                    name: "tsinit",
                    description: Some(
                        "Initialize Timestamp",
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
                    name: "tsupdt",
                    description: Some(
                        "Update Timestamp",
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
                    name: "tsaddreg",
                    description: Some(
                        "Update Addend Register",
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
                    name: "tsenall",
                    description: Some(
                        "Enable Timestamp for All Packets",
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
                    name: "tsctrlssr",
                    description: Some(
                        "Timestamp Digital or Binary Rollover Control",
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
                    name: "tsver2ena",
                    description: Some(
                        "Enable PTP Packet Processing for Version 2 Format",
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
                    name: "tsipena",
                    description: Some(
                        "Enable Processing of PTP over Ethernet Packets",
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
                    name: "tsipv6ena",
                    description: Some(
                        "Enable Processing of PTP Packets Sent over IPv6-UDP",
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
                    name: "tsipv4ena",
                    description: Some(
                        "Enable Processing of PTP Packets Sent over IPv4-UDP",
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
                    name: "tsevntena",
                    description: Some(
                        "Enable Timestamp Snapshot for Event Messages",
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
                    name: "tsmstrena",
                    description: Some(
                        "Enable Snapshot for Messages Relevant to Master",
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
                    name: "snaptypsel",
                    description: Some(
                        "Select PTP packets for Taking Snapshots",
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
                Field {
                    name: "tsenmacaddr",
                    description: Some(
                        "Enable MAC Address for PTP Packet Filtering",
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
                    name: "csc",
                    description: Some(
                        "Enable checksum correction during OST for PTP over UDP/IPv4 packets",
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
                    name: "txtsstsm",
                    description: Some(
                        "Transmit Timestamp Status Mode",
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
            name: "Mactseacr",
            extends: None,
            description: Some(
                "Timestamp Egress asymmetric correction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "osteac",
                    description: Some(
                        "One-Step Timestamp Egress Asymmetry Correction",
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
            name: "Mactsecnr",
            extends: None,
            description: Some(
                "Timestamp Egress correction nanosecond register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsec",
                    description: Some(
                        "Timestamp Egress Correction",
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
            name: "Mactsiacr",
            extends: None,
            description: Some(
                "Timestamp Ingress asymmetric correction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ostiac",
                    description: Some(
                        "One-Step Timestamp Ingress Asymmetry Correction",
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
            name: "Mactsicnr",
            extends: None,
            description: Some(
                "Timestamp Ingress correction nanosecond register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsic",
                    description: Some(
                        "Timestamp Ingress Correction",
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
            name: "Mactssr",
            extends: None,
            description: Some(
                "Timestamp status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tssovf",
                    description: Some(
                        "Timestamp Seconds Overflow",
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
                    name: "tstargt0",
                    description: Some(
                        "Timestamp Target Time Reached",
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
                    name: "auxtstrig",
                    description: Some(
                        "Auxiliary Timestamp Trigger Snapshot",
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
                    name: "tstrgterr0",
                    description: Some(
                        "Timestamp Target Time Error",
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
                    name: "txtssis",
                    description: Some(
                        "Tx Timestamp Status Interrupt Status",
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
                    name: "atsstn",
                    description: Some(
                        "Auxiliary Timestamp Snapshot Trigger Identifier",
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
                    name: "atsstm",
                    description: Some(
                        "Auxiliary Timestamp Snapshot Trigger Missed",
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
                    name: "atsns",
                    description: Some(
                        "Number of Auxiliary Timestamp Snapshots",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MactxTssnr",
            extends: None,
            description: Some(
                "Tx timestamp status nanoseconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txtsslo",
                    description: Some(
                        "Transmit Timestamp Status Low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txtssmis",
                    description: Some(
                        "Transmit Timestamp Status Missed",
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
            name: "MactxTsssr",
            extends: None,
            description: Some(
                "Tx timestamp status seconds register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txtsshi",
                    description: Some(
                        "Transmit Timestamp Status High",
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
            name: "Macvhtr",
            extends: None,
            description: Some(
                "VLAN Hash table register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlht",
                    description: Some(
                        "VLAN Hash Table",
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
            name: "Macvir",
            extends: None,
            description: Some(
                "VLAN inclusion register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlt",
                    description: Some(
                        "VLAN Tag for Transmit Packets",
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
                    name: "vlc",
                    description: Some(
                        "VLAN Tag Control in Transmit Packets",
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
                Field {
                    name: "vlp",
                    description: Some(
                        "VLAN Priority Control",
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
                    name: "csvl",
                    description: Some(
                        "C-VLAN or S-VLAN",
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
                    name: "vlti",
                    description: Some(
                        "VLAN Tag Input",
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
            name: "Macvr",
            extends: None,
            description: Some(
                "Version register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snpsver",
                    description: Some(
                        "IP version",
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
                    name: "userver",
                    description: Some(
                        "ST-defined version",
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
            name: "Macvtr",
            extends: None,
            description: Some(
                "VLAN tag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vl",
                    description: Some(
                        "VLAN Tag Identifier for Receive Packets",
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
                    name: "etv",
                    description: Some(
                        "Enable 12-Bit VLAN Tag Comparison",
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
                    name: "vtim",
                    description: Some(
                        "VLAN Tag Inverse Match Enable",
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
                    name: "esvl",
                    description: Some(
                        "Enable S-VLAN",
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
                    name: "ersvlm",
                    description: Some(
                        "Enable Receive S-VLAN Match",
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
                    name: "dovltc",
                    description: Some(
                        "Disable VLAN Type Check",
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
                    name: "evls",
                    description: Some(
                        "Enable VLAN Tag Stripping on Receive",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evlrxs",
                    description: Some(
                        "Enable VLAN Tag in Rx status",
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
                    name: "vthm",
                    description: Some(
                        "VLAN Tag Hash Table Match Enable",
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
                    name: "edvlp",
                    description: Some(
                        "Enable Double VLAN Processing",
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
                    name: "erivlt",
                    description: Some(
                        "Enable Inner VLAN Tag",
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
                    name: "eivls",
                    description: Some(
                        "Enable Inner VLAN Tag Stripping on Receive",
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
                    name: "eivlrxs",
                    description: Some(
                        "Enable Inner VLAN Tag in Rx Status",
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
            name: "Macwtr",
            extends: None,
            description: Some(
                "Watchdog timeout register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wto",
                    description: Some(
                        "Watchdog Timeout",
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
                    name: "pwe",
                    description: Some(
                        "Programmable Watchdog Enable",
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
            name: "MmcControl",
            extends: None,
            description: Some(
                "MMC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntrst",
                    description: Some(
                        "Counters Reset",
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
                    name: "cntstopro",
                    description: Some(
                        "Counter Stop Rollover",
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
                    name: "rstonrd",
                    description: Some(
                        "Reset on Read",
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
                    name: "cntfreez",
                    description: Some(
                        "MMC Counter Freeze",
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
                    name: "cntprst",
                    description: Some(
                        "Counters Preset",
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
                    name: "cntprstlvl",
                    description: Some(
                        "Full-Half Preset",
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
                    name: "ucdbc",
                    description: Some(
                        "Update MMC Counters for Dropped Broadcast Packets",
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
            name: "MmcRxInterrupt",
            extends: None,
            description: Some(
                "MMC Rx interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrcerpis",
                    description: Some(
                        "MMC Receive CRC Error Packet Counter Interrupt Status",
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
                    name: "rxalgnerpis",
                    description: Some(
                        "MMC Receive Alignment Error Packet Counter Interrupt Status",
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
                    name: "rxucgpis",
                    description: Some(
                        "MMC Receive Unicast Good Packet Counter Interrupt Status",
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
                    name: "rxlpiuscis",
                    description: Some(
                        "MMC Receive LPI microsecond counter interrupt status",
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
                    name: "rxlpitrcis",
                    description: Some(
                        "MMC Receive LPI transition counter interrupt status",
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
            name: "MmcRxInterruptMask",
            extends: None,
            description: Some(
                "MMC Rx interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrcerpim",
                    description: Some(
                        "MMC Receive CRC Error Packet Counter Interrupt Mask",
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
                    name: "rxalgnerpim",
                    description: Some(
                        "MMC Receive Alignment Error Packet Counter Interrupt Mask",
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
                    name: "rxucgpim",
                    description: Some(
                        "MMC Receive Unicast Good Packet Counter Interrupt Mask",
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
                    name: "rxlpiuscim",
                    description: Some(
                        "MMC Receive LPI microsecond counter interrupt Mask",
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
                    name: "rxlpitrcim",
                    description: Some(
                        "MMC Receive LPI transition counter interrupt Mask",
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
            name: "MmcTxInterrupt",
            extends: None,
            description: Some(
                "MMC Tx interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txscolgpis",
                    description: Some(
                        "MMC Transmit Single Collision Good Packet Counter Interrupt Status",
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
                    name: "txmcolgpis",
                    description: Some(
                        "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status",
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
                    name: "txgpktis",
                    description: Some(
                        "MMC Transmit Good Packet Counter Interrupt Status",
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
                    name: "txlpiuscis",
                    description: Some(
                        "MMC Transmit LPI microsecond counter interrupt status",
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
                    name: "txlpitrcis",
                    description: Some(
                        "MMC Transmit LPI transition counter interrupt status",
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
            name: "MmcTxInterruptMask",
            extends: None,
            description: Some(
                "MMC Tx interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txscolgpim",
                    description: Some(
                        "MMC Transmit Single Collision Good Packet Counter Interrupt Mask",
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
                    name: "txmcolgpim",
                    description: Some(
                        "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask",
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
                    name: "txgpktim",
                    description: Some(
                        "MMC Transmit Good Packet Counter Interrupt Mask",
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
                    name: "txlpiuscim",
                    description: Some(
                        "MMC Transmit LPI microsecond counter interrupt Mask",
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
                    name: "txlpitrcim",
                    description: Some(
                        "MMC Transmit LPI transition counter interrupt Mask",
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
            name: "Mtlisr",
            extends: None,
            description: Some(
                "Interrupt status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "q0is",
                    description: Some(
                        "Queue interrupt status",
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
            name: "Mtlomr",
            extends: None,
            description: Some(
                "Operating mode Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtxsts",
                    description: Some(
                        "Drop Transmit Status",
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
                    name: "cntprst",
                    description: Some(
                        "Counters Preset",
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
                    name: "cntclr",
                    description: Some(
                        "Counters Reset",
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
            name: "Mtlqicsr",
            extends: None,
            description: Some(
                "Queue interrupt control status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txunfis",
                    description: Some(
                        "Transmit Queue Underflow Interrupt Status",
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
                    name: "txuie",
                    description: Some(
                        "Transmit Queue Underflow Interrupt Enable",
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
                    name: "rxovfis",
                    description: Some(
                        "Receive Queue Overflow Interrupt Status",
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
                    name: "rxoie",
                    description: Some(
                        "Receive Queue Overflow Interrupt Enable",
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
            name: "MtlrxQdr",
            extends: None,
            description: Some(
                "Rx queue debug register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwcsts",
                    description: Some(
                        "MTL Rx Queue Write Controller Active Status",
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
                    name: "rrcsts",
                    description: Some(
                        "MTL Rx Queue Read Controller State",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxqsts",
                    description: Some(
                        "MTL Rx Queue Fill-Level Status",
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
                    name: "prxq",
                    description: Some(
                        "Number of Packets in Receive Queue",
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
            name: "MtlrxQmpocr",
            extends: None,
            description: Some(
                "Rx queue missed packet and overflow counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovfpktcnt",
                    description: Some(
                        "Overflow Packet Counter",
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
                    name: "ovfcntovf",
                    description: Some(
                        "Overflow Counter Overflow Bit",
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
                    name: "mispktcnt",
                    description: Some(
                        "Missed Packet Counter",
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
                    name: "miscntovf",
                    description: Some(
                        "Missed Packet Counter Overflow Bit",
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
            name: "MtlrxQomr",
            extends: None,
            description: Some(
                "Rx queue operating mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtc",
                    description: Some(
                        "Receive Queue Threshold Control",
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
                    name: "fup",
                    description: Some(
                        "Forward Undersized Good Packets",
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
                    name: "fep",
                    description: Some(
                        "Forward Error Packets",
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
                    name: "rsf",
                    description: Some(
                        "Receive Queue Store and Forward",
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
                    name: "dis_tcp_ef",
                    description: Some(
                        "Disable Dropping of TCP/IP Checksum Error Packets",
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
                    name: "ehfc",
                    description: Some(
                        "Enable Hardware Flow Control",
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
                    name: "rfa",
                    description: Some(
                        "Threshold for Activating Flow Control (in half-duplex and full-duplex modes)",
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
                    name: "rfd",
                    description: Some(
                        "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rqs",
                    description: Some(
                        "Receive Queue Size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MtltxQdr",
            extends: None,
            description: Some(
                "Tx queue debug Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txqpaused",
                    description: Some(
                        "Transmit Queue in Pause",
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
                    name: "trcsts",
                    description: Some(
                        "MTL Tx Queue Read Controller Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twcsts",
                    description: Some(
                        "MTL Tx Queue Write Controller Status",
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
                    name: "txqsts",
                    description: Some(
                        "MTL Tx Queue Not Empty Status",
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
                    name: "txstsfsts",
                    description: Some(
                        "MTL Tx Status FIFO Full Status",
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
                    name: "ptxq",
                    description: Some(
                        "Number of Packets in the Transmit Queue",
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
                    name: "stxstsf",
                    description: Some(
                        "Number of Status Words in Tx Status FIFO of Queue",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MtltxQomr",
            extends: None,
            description: Some(
                "Tx queue operating mode Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftq",
                    description: Some(
                        "Flush Transmit Queue",
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
                    name: "tsf",
                    description: Some(
                        "Transmit Store and Forward",
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
                    name: "txqen",
                    description: Some(
                        "Transmit Queue Enable",
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
                    name: "ttc",
                    description: Some(
                        "Transmit Threshold Control",
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
                    name: "tqs",
                    description: Some(
                        "Transmit Queue Size",
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
            name: "MtltxQur",
            extends: None,
            description: Some(
                "Tx queue underflow register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uffrmcnt",
                    description: Some(
                        "Underflow Packet Counter",
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
                    name: "ufcntovf",
                    description: Some(
                        "Overflow Bit for Underflow Packet Counter",
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
            name: "RxAlignmentErrorPackets",
            extends: None,
            description: Some(
                "Rx alignment error packets register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxalgnerr",
                    description: Some(
                        "Rx Alignment Error Packets",
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
            name: "RxCrcErrorPackets",
            extends: None,
            description: Some(
                "Rx CRC error packets register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrcerr",
                    description: Some(
                        "Rx CRC Error Packets",
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
            name: "RxLpiTranCntr",
            extends: None,
            description: Some(
                "Rx LPI transition counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxlpitrc",
                    description: Some(
                        "Rx LPI Transition counter",
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
            name: "RxLpiUsecCntr",
            extends: None,
            description: Some(
                "Rx LPI microsecond counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxlpiusc",
                    description: Some(
                        "Rx LPI Microseconds Counter",
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
            name: "RxUnicastPacketsGood",
            extends: None,
            description: Some(
                "Rx unicast packets good register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxucastg",
                    description: Some(
                        "Rx Unicast Packets Good",
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
            name: "TxLpiTranCntr",
            extends: None,
            description: Some(
                "Tx LPI transition counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txlpitrc",
                    description: Some(
                        "Tx LPI Transition counter",
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
            name: "TxLpiUsecCntr",
            extends: None,
            description: Some(
                "Tx LPI microsecond timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txlpiusc",
                    description: Some(
                        "Tx LPI Microseconds Counter",
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
            name: "TxMultipleCollisionGoodPackets",
            extends: None,
            description: Some(
                "Tx multiple collision good packets register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txmultcolg",
                    description: Some(
                        "Tx Multiple Collision Good Packets",
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
            name: "TxPacketCountGood",
            extends: None,
            description: Some(
                "Tx packet count good register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txpktg",
                    description: Some(
                        "Tx Packet Count Good",
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
            name: "TxSingleCollisionGoodPackets",
            extends: None,
            description: Some(
                "Tx single collision good packets register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txsnglcolg",
                    description: Some(
                        "Tx Single Collision Good Packets",
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
    ],
    enums: &[],
};
