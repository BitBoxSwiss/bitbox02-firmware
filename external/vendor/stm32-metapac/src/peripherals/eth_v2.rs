#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet Peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet: media access control (MAC)"]
    #[inline(always)]
    pub const fn ethernet_mac(self) -> EthernetMac {
        unsafe { EthernetMac::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet: MTL mode register (MTL)"]
    #[inline(always)]
    pub const fn ethernet_mtl(self) -> EthernetMtl {
        unsafe { EthernetMtl::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[doc = "Ethernet: DMA mode register (DMA)"]
    #[inline(always)]
    pub const fn ethernet_dma(self) -> EthernetDma {
        unsafe { EthernetDma::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
}
#[doc = "Ethernet: DMA mode register (DMA)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetDma {
    ptr: *mut u8,
}
unsafe impl Send for EthernetDma {}
unsafe impl Sync for EthernetDma {}
impl EthernetDma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA mode register"]
    #[inline(always)]
    pub const fn dmamr(self) -> crate::common::Reg<regs::Dmamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "System bus mode register"]
    #[inline(always)]
    pub const fn dmasbmr(self) -> crate::common::Reg<regs::Dmasbmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt status register"]
    #[inline(always)]
    pub const fn dmaisr(self) -> crate::common::Reg<regs::Dmaisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Debug status register"]
    #[inline(always)]
    pub const fn dmadsr(self) -> crate::common::Reg<regs::Dmadsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Channel control register"]
    #[inline(always)]
    pub const fn dmaccr(self) -> crate::common::Reg<regs::Dmaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Channel transmit control register"]
    #[inline(always)]
    pub const fn dmactx_cr(self) -> crate::common::Reg<regs::DmactxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Channel receive control register"]
    #[inline(always)]
    pub const fn dmacrx_cr(self) -> crate::common::Reg<regs::DmacrxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Channel Tx descriptor list address register"]
    #[inline(always)]
    pub const fn dmactx_dlar(self) -> crate::common::Reg<regs::DmactxDlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Channel Rx descriptor list address register"]
    #[inline(always)]
    pub const fn dmacrx_dlar(self) -> crate::common::Reg<regs::DmacrxDlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Channel Tx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmactx_dtpr(self) -> crate::common::Reg<regs::DmactxDtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Channel Rx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmacrx_dtpr(self) -> crate::common::Reg<regs::DmacrxDtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Channel Tx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmactx_rlr(self) -> crate::common::Reg<regs::DmactxRlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Channel Rx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmacrx_rlr(self) -> crate::common::Reg<regs::DmacrxRlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Channel interrupt enable register"]
    #[inline(always)]
    pub const fn dmacier(self) -> crate::common::Reg<regs::Dmacier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Channel Rx interrupt watchdog timer register"]
    #[inline(always)]
    pub const fn dmacrx_iwtr(self) -> crate::common::Reg<regs::DmacrxIwtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Channel current application transmit descriptor register"]
    #[inline(always)]
    pub const fn dmaccatx_dr(self) -> crate::common::Reg<regs::DmaccatxDr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "Channel current application receive descriptor register"]
    #[inline(always)]
    pub const fn dmaccarx_dr(self) -> crate::common::Reg<regs::DmaccarxDr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Channel current application transmit buffer register"]
    #[inline(always)]
    pub const fn dmaccatx_br(self) -> crate::common::Reg<regs::DmaccatxBr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "Channel current application receive buffer register"]
    #[inline(always)]
    pub const fn dmaccarx_br(self) -> crate::common::Reg<regs::DmaccarxBr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "Channel status register"]
    #[inline(always)]
    pub const fn dmacsr(self) -> crate::common::Reg<regs::Dmacsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "Channel missed frame count register"]
    #[inline(always)]
    pub const fn dmacmfcr(self) -> crate::common::Reg<regs::Dmacmfcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
}
#[doc = "Ethernet: media access control (MAC)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMac {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMac {}
unsafe impl Sync for EthernetMac {}
impl EthernetMac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operating mode configuration register"]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Extended operating mode configuration register"]
    #[inline(always)]
    pub const fn macecr(self) -> crate::common::Reg<regs::Macecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Packet filtering control register"]
    #[inline(always)]
    pub const fn macpfr(self) -> crate::common::Reg<regs::Macpfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Watchdog timeout register"]
    #[inline(always)]
    pub const fn macwtr(self) -> crate::common::Reg<regs::Macwtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Hash Table 0/1 register"]
    #[inline(always)]
    pub const fn machtr(self, n: usize) -> crate::common::Reg<regs::Machtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "VLAN tag register"]
    #[inline(always)]
    pub const fn macvtr(self) -> crate::common::Reg<regs::Macvtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "VLAN Hash table register"]
    #[inline(always)]
    pub const fn macvhtr(self) -> crate::common::Reg<regs::Macvhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "VLAN inclusion register"]
    #[inline(always)]
    pub const fn macvir(self) -> crate::common::Reg<regs::Macvir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Inner VLAN inclusion register"]
    #[inline(always)]
    pub const fn macivir(self) -> crate::common::Reg<regs::Macivir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Tx Queue flow control register"]
    #[inline(always)]
    pub const fn macqtx_fcr(self) -> crate::common::Reg<regs::MacqtxFcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Rx flow control register"]
    #[inline(always)]
    pub const fn macrx_fcr(self) -> crate::common::Reg<regs::MacrxFcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Interrupt status register"]
    #[inline(always)]
    pub const fn macisr(self) -> crate::common::Reg<regs::Macisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Interrupt enable register"]
    #[inline(always)]
    pub const fn macier(self) -> crate::common::Reg<regs::Macier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Rx Tx status register"]
    #[inline(always)]
    pub const fn macrx_tx_sr(self) -> crate::common::Reg<regs::MacrxTxSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "PMT control status register"]
    #[inline(always)]
    pub const fn macpcsr(self) -> crate::common::Reg<regs::Macpcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Remove wakeup packet filter register"]
    #[inline(always)]
    pub const fn macrwkpfr(self) -> crate::common::Reg<regs::Macrwkpfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "LPI control status register"]
    #[inline(always)]
    pub const fn maclcsr(self) -> crate::common::Reg<regs::Maclcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "LPI timers control register"]
    #[inline(always)]
    pub const fn macltcr(self) -> crate::common::Reg<regs::Macltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "LPI entry timer register"]
    #[inline(always)]
    pub const fn macletr(self) -> crate::common::Reg<regs::Macletr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "1-microsecond-tick counter register"]
    #[inline(always)]
    pub const fn mac1ustcr(self) -> crate::common::Reg<regs::Mac1ustcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Version register"]
    #[inline(always)]
    pub const fn macvr(self) -> crate::common::Reg<regs::Macvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Debug register"]
    #[inline(always)]
    pub const fn macdr(self) -> crate::common::Reg<regs::Macdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "HW feature 1 register"]
    #[inline(always)]
    pub const fn machwf1r(self) -> crate::common::Reg<regs::Machwf1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "HW feature 2 register"]
    #[inline(always)]
    pub const fn machwf2r(self) -> crate::common::Reg<regs::Machwf2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "MDIO address register"]
    #[inline(always)]
    pub const fn macmdioar(self) -> crate::common::Reg<regs::Macmdioar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "MDIO data register"]
    #[inline(always)]
    pub const fn macmdiodr(self) -> crate::common::Reg<regs::Macmdiodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Address 1/2/3 high register"]
    #[inline(always)]
    pub const fn macahr(self, n: usize) -> crate::common::Reg<regs::Macahr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize + n * 8usize) as _) }
    }
    #[doc = "Address 1/2/3 low register"]
    #[inline(always)]
    pub const fn macalr(self, n: usize) -> crate::common::Reg<regs::Macalr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize + n * 8usize) as _) }
    }
    #[doc = "MMC control register"]
    #[inline(always)]
    pub const fn mmc_control(self) -> crate::common::Reg<regs::MmcControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "MMC Rx interrupt register"]
    #[inline(always)]
    pub const fn mmc_rx_interrupt(self) -> crate::common::Reg<regs::MmcRxInterrupt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "MMC Tx interrupt register"]
    #[inline(always)]
    pub const fn mmc_tx_interrupt(self) -> crate::common::Reg<regs::MmcTxInterrupt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[doc = "MMC Rx interrupt mask register"]
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(self) -> crate::common::Reg<regs::MmcRxInterruptMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[doc = "MMC Tx interrupt mask register"]
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(self) -> crate::common::Reg<regs::MmcTxInterruptMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[doc = "Tx single collision good packets register"]
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(
        self,
    ) -> crate::common::Reg<regs::TxSingleCollisionGoodPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x074cusize) as _) }
    }
    #[doc = "Tx multiple collision good packets register"]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(
        self,
    ) -> crate::common::Reg<regs::TxMultipleCollisionGoodPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0750usize) as _) }
    }
    #[doc = "Tx packet count good register"]
    #[inline(always)]
    pub const fn tx_packet_count_good(self) -> crate::common::Reg<regs::TxPacketCountGood, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0768usize) as _) }
    }
    #[doc = "Rx CRC error packets register"]
    #[inline(always)]
    pub const fn rx_crc_error_packets(self) -> crate::common::Reg<regs::RxCrcErrorPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0794usize) as _) }
    }
    #[doc = "Rx alignment error packets register"]
    #[inline(always)]
    pub const fn rx_alignment_error_packets(
        self,
    ) -> crate::common::Reg<regs::RxAlignmentErrorPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0798usize) as _) }
    }
    #[doc = "Rx unicast packets good register"]
    #[inline(always)]
    pub const fn rx_unicast_packets_good(self) -> crate::common::Reg<regs::RxUnicastPacketsGood, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c4usize) as _) }
    }
    #[doc = "Tx LPI microsecond timer register"]
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(self) -> crate::common::Reg<regs::TxLpiUsecCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07ecusize) as _) }
    }
    #[doc = "Tx LPI transition counter register"]
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(self) -> crate::common::Reg<regs::TxLpiTranCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f0usize) as _) }
    }
    #[doc = "Rx LPI microsecond counter register"]
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(self) -> crate::common::Reg<regs::RxLpiUsecCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f4usize) as _) }
    }
    #[doc = "Rx LPI transition counter register"]
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(self) -> crate::common::Reg<regs::RxLpiTranCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
    #[doc = "L3 and L4 control 0 register"]
    #[inline(always)]
    pub const fn macl3l4c0r(self) -> crate::common::Reg<regs::Macl3l4c0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Layer4 address filter 0 register"]
    #[inline(always)]
    pub const fn macl4a0r(self) -> crate::common::Reg<regs::Macl4a0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[doc = "MACL3A00R"]
    #[inline(always)]
    pub const fn macl3a00r(self) -> crate::common::Reg<regs::Macl3a00r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0910usize) as _) }
    }
    #[doc = "Layer3 address 1 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a10r(self) -> crate::common::Reg<regs::Macl3a10r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0914usize) as _) }
    }
    #[doc = "Layer3 Address 2 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a20(self) -> crate::common::Reg<regs::Macl3a20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0918usize) as _) }
    }
    #[doc = "Layer3 Address 3 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a30(self) -> crate::common::Reg<regs::Macl3a30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x091cusize) as _) }
    }
    #[doc = "L3 and L4 control 1 register"]
    #[inline(always)]
    pub const fn macl3l4c1r(self) -> crate::common::Reg<regs::Macl3l4c1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0930usize) as _) }
    }
    #[doc = "Layer 4 address filter 1 register"]
    #[inline(always)]
    pub const fn macl4a1r(self) -> crate::common::Reg<regs::Macl4a1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0934usize) as _) }
    }
    #[doc = "Layer3 address 0 filter 1 Register"]
    #[inline(always)]
    pub const fn macl3a01r(self) -> crate::common::Reg<regs::Macl3a01r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0940usize) as _) }
    }
    #[doc = "Layer3 address 1 filter 1 register"]
    #[inline(always)]
    pub const fn macl3a11r(self) -> crate::common::Reg<regs::Macl3a11r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0944usize) as _) }
    }
    #[doc = "Layer3 address 2 filter 1 Register"]
    #[inline(always)]
    pub const fn macl3a21r(self) -> crate::common::Reg<regs::Macl3a21r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0948usize) as _) }
    }
    #[doc = "Layer3 address 3 filter 1 register"]
    #[inline(always)]
    pub const fn macl3a31r(self) -> crate::common::Reg<regs::Macl3a31r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x094cusize) as _) }
    }
    #[doc = "ARP address register"]
    #[inline(always)]
    pub const fn macarpar(self) -> crate::common::Reg<regs::Macarpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ae0usize) as _) }
    }
    #[doc = "Timestamp control Register"]
    #[inline(always)]
    pub const fn mactscr(self) -> crate::common::Reg<regs::Mactscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize) as _) }
    }
    #[doc = "Sub-second increment register"]
    #[inline(always)]
    pub const fn macssir(self) -> crate::common::Reg<regs::Macssir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b04usize) as _) }
    }
    #[doc = "System time seconds register"]
    #[inline(always)]
    pub const fn macstsr(self) -> crate::common::Reg<regs::Macstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b08usize) as _) }
    }
    #[doc = "System time nanoseconds register"]
    #[inline(always)]
    pub const fn macstnr(self) -> crate::common::Reg<regs::Macstnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b0cusize) as _) }
    }
    #[doc = "System time seconds update register"]
    #[inline(always)]
    pub const fn macstsur(self) -> crate::common::Reg<regs::Macstsur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b10usize) as _) }
    }
    #[doc = "System time nanoseconds update register"]
    #[inline(always)]
    pub const fn macstnur(self) -> crate::common::Reg<regs::Macstnur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b14usize) as _) }
    }
    #[doc = "Timestamp addend register"]
    #[inline(always)]
    pub const fn mactsar(self) -> crate::common::Reg<regs::Mactsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b18usize) as _) }
    }
    #[doc = "Timestamp status register"]
    #[inline(always)]
    pub const fn mactssr(self) -> crate::common::Reg<regs::Mactssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b20usize) as _) }
    }
    #[doc = "Tx timestamp status nanoseconds register"]
    #[inline(always)]
    pub const fn mactx_tssnr(self) -> crate::common::Reg<regs::MactxTssnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b30usize) as _) }
    }
    #[doc = "Tx timestamp status seconds register"]
    #[inline(always)]
    pub const fn mactx_tsssr(self) -> crate::common::Reg<regs::MactxTsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b34usize) as _) }
    }
    #[doc = "Auxiliary control register"]
    #[inline(always)]
    pub const fn macacr(self) -> crate::common::Reg<regs::Macacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b40usize) as _) }
    }
    #[doc = "Auxiliary timestamp nanoseconds register"]
    #[inline(always)]
    pub const fn macatsnr(self) -> crate::common::Reg<regs::Macatsnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b48usize) as _) }
    }
    #[doc = "Auxiliary timestamp seconds register"]
    #[inline(always)]
    pub const fn macatssr(self) -> crate::common::Reg<regs::Macatssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b4cusize) as _) }
    }
    #[doc = "Timestamp Ingress asymmetric correction register"]
    #[inline(always)]
    pub const fn mactsiacr(self) -> crate::common::Reg<regs::Mactsiacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b50usize) as _) }
    }
    #[doc = "Timestamp Egress asymmetric correction register"]
    #[inline(always)]
    pub const fn mactseacr(self) -> crate::common::Reg<regs::Mactseacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b54usize) as _) }
    }
    #[doc = "Timestamp Ingress correction nanosecond register"]
    #[inline(always)]
    pub const fn mactsicnr(self) -> crate::common::Reg<regs::Mactsicnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b58usize) as _) }
    }
    #[doc = "Timestamp Egress correction nanosecond register"]
    #[inline(always)]
    pub const fn mactsecnr(self) -> crate::common::Reg<regs::Mactsecnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b5cusize) as _) }
    }
    #[doc = "PPS control register"]
    #[inline(always)]
    pub const fn macppscr(self) -> crate::common::Reg<regs::Macppscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b70usize) as _) }
    }
    #[doc = "PPS target time seconds register"]
    #[inline(always)]
    pub const fn macppsttsr(self) -> crate::common::Reg<regs::Macppsttsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b80usize) as _) }
    }
    #[doc = "PPS target time nanoseconds register"]
    #[inline(always)]
    pub const fn macppsttnr(self) -> crate::common::Reg<regs::Macppsttnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b84usize) as _) }
    }
    #[doc = "PPS interval register"]
    #[inline(always)]
    pub const fn macppsir(self) -> crate::common::Reg<regs::Macppsir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b88usize) as _) }
    }
    #[doc = "PPS width register"]
    #[inline(always)]
    pub const fn macppswr(self) -> crate::common::Reg<regs::Macppswr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b8cusize) as _) }
    }
    #[doc = "PTP Offload control register"]
    #[inline(always)]
    pub const fn macpocr(self) -> crate::common::Reg<regs::Macpocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bc0usize) as _) }
    }
    #[doc = "PTP Source Port Identity 0 Register"]
    #[inline(always)]
    pub const fn macspi0r(self) -> crate::common::Reg<regs::Macspi0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bc4usize) as _) }
    }
    #[doc = "PTP Source port identity 1 register"]
    #[inline(always)]
    pub const fn macspi1r(self) -> crate::common::Reg<regs::Macspi1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bc8usize) as _) }
    }
    #[doc = "PTP Source port identity 2 register"]
    #[inline(always)]
    pub const fn macspi2r(self) -> crate::common::Reg<regs::Macspi2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bccusize) as _) }
    }
    #[doc = "Log message interval register"]
    #[inline(always)]
    pub const fn maclmir(self) -> crate::common::Reg<regs::Maclmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bd0usize) as _) }
    }
}
#[doc = "Ethernet: MTL mode register (MTL)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMtl {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMtl {}
unsafe impl Sync for EthernetMtl {}
impl EthernetMtl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operating mode Register"]
    #[inline(always)]
    pub const fn mtlomr(self) -> crate::common::Reg<regs::Mtlomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt status Register"]
    #[inline(always)]
    pub const fn mtlisr(self) -> crate::common::Reg<regs::Mtlisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Tx queue operating mode Register"]
    #[inline(always)]
    pub const fn mtltx_qomr(self) -> crate::common::Reg<regs::MtltxQomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Tx queue underflow register"]
    #[inline(always)]
    pub const fn mtltx_qur(self) -> crate::common::Reg<regs::MtltxQur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Tx queue debug Register"]
    #[inline(always)]
    pub const fn mtltx_qdr(self) -> crate::common::Reg<regs::MtltxQdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Queue interrupt control status Register"]
    #[inline(always)]
    pub const fn mtlqicsr(self) -> crate::common::Reg<regs::Mtlqicsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Rx queue operating mode register"]
    #[inline(always)]
    pub const fn mtlrx_qomr(self) -> crate::common::Reg<regs::MtlrxQomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Rx queue missed packet and overflow counter register"]
    #[inline(always)]
    pub const fn mtlrx_qmpocr(self) -> crate::common::Reg<regs::MtlrxQmpocr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Rx queue debug register"]
    #[inline(always)]
    pub const fn mtlrx_qdr(self) -> crate::common::Reg<regs::MtlrxQdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel current application receive buffer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccarxBr(pub u32);
    impl DmaccarxBr {
        #[doc = "Application Receive Buffer Address Pointer"]
        #[inline(always)]
        pub const fn currbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Receive Buffer Address Pointer"]
        #[inline(always)]
        pub fn set_currbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccarxBr {
        #[inline(always)]
        fn default() -> DmaccarxBr {
            DmaccarxBr(0)
        }
    }
    impl core::fmt::Debug for DmaccarxBr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccarxBr")
                .field("currbufaptr", &self.currbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccarxBr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccarxBr {{ currbufaptr: {=u32:?} }}", self.currbufaptr())
        }
    }
    #[doc = "Channel current application receive descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccarxDr(pub u32);
    impl DmaccarxDr {
        #[doc = "Application Receive Descriptor Address Pointer"]
        #[inline(always)]
        pub const fn currdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Receive Descriptor Address Pointer"]
        #[inline(always)]
        pub fn set_currdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccarxDr {
        #[inline(always)]
        fn default() -> DmaccarxDr {
            DmaccarxDr(0)
        }
    }
    impl core::fmt::Debug for DmaccarxDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccarxDr")
                .field("currdesaptr", &self.currdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccarxDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccarxDr {{ currdesaptr: {=u32:?} }}", self.currdesaptr())
        }
    }
    #[doc = "Channel current application transmit buffer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccatxBr(pub u32);
    impl DmaccatxBr {
        #[doc = "Application Transmit Buffer Address Pointer"]
        #[inline(always)]
        pub const fn curtbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Transmit Buffer Address Pointer"]
        #[inline(always)]
        pub fn set_curtbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccatxBr {
        #[inline(always)]
        fn default() -> DmaccatxBr {
            DmaccatxBr(0)
        }
    }
    impl core::fmt::Debug for DmaccatxBr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccatxBr")
                .field("curtbufaptr", &self.curtbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccatxBr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccatxBr {{ curtbufaptr: {=u32:?} }}", self.curtbufaptr())
        }
    }
    #[doc = "Channel current application transmit descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccatxDr(pub u32);
    impl DmaccatxDr {
        #[doc = "Application Transmit Descriptor Address Pointer"]
        #[inline(always)]
        pub const fn curtdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Transmit Descriptor Address Pointer"]
        #[inline(always)]
        pub fn set_curtdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccatxDr {
        #[inline(always)]
        fn default() -> DmaccatxDr {
            DmaccatxDr(0)
        }
    }
    impl core::fmt::Debug for DmaccatxDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccatxDr")
                .field("curtdesaptr", &self.curtdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccatxDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccatxDr {{ curtdesaptr: {=u32:?} }}", self.curtdesaptr())
        }
    }
    #[doc = "Channel control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaccr(pub u32);
    impl Dmaccr {
        #[doc = "Maximum Segment Size"]
        #[inline(always)]
        pub const fn mss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Maximum Segment Size"]
        #[inline(always)]
        pub fn set_mss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "8xPBL mode"]
        #[inline(always)]
        pub const fn pblx8(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "8xPBL mode"]
        #[inline(always)]
        pub fn set_pblx8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Descriptor Skip Length"]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Descriptor Skip Length"]
        #[inline(always)]
        pub fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
    }
    impl Default for Dmaccr {
        #[inline(always)]
        fn default() -> Dmaccr {
            Dmaccr(0)
        }
    }
    impl core::fmt::Debug for Dmaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaccr")
                .field("mss", &self.mss())
                .field("pblx8", &self.pblx8())
                .field("dsl", &self.dsl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaccr {{ mss: {=u16:?}, pblx8: {=bool:?}, dsl: {=u8:?} }}",
                self.mss(),
                self.pblx8(),
                self.dsl()
            )
        }
    }
    #[doc = "Channel interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacier(pub u32);
    impl Dmacier {
        #[doc = "Transmit Interrupt Enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt Enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Stopped Enable"]
        #[inline(always)]
        pub const fn txse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Stopped Enable"]
        #[inline(always)]
        pub fn set_txse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable Enable"]
        #[inline(always)]
        pub const fn tbue(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable Enable"]
        #[inline(always)]
        pub fn set_tbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive Interrupt Enable"]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt Enable"]
        #[inline(always)]
        pub fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable Enable"]
        #[inline(always)]
        pub const fn rbue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable Enable"]
        #[inline(always)]
        pub fn set_rbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Stopped Enable"]
        #[inline(always)]
        pub const fn rse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Stopped Enable"]
        #[inline(always)]
        pub fn set_rse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout Enable"]
        #[inline(always)]
        pub const fn rwte(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout Enable"]
        #[inline(always)]
        pub fn set_rwte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt Enable"]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt Enable"]
        #[inline(always)]
        pub fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Early Receive Interrupt Enable"]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt Enable"]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Fatal Bus Error Enable"]
        #[inline(always)]
        pub const fn fbee(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error Enable"]
        #[inline(always)]
        pub fn set_fbee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Context Descriptor Error Enable"]
        #[inline(always)]
        pub const fn cdee(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Context Descriptor Error Enable"]
        #[inline(always)]
        pub fn set_cdee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Abnormal Interrupt Summary Enable"]
        #[inline(always)]
        pub const fn aie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary Enable"]
        #[inline(always)]
        pub fn set_aie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Normal Interrupt Summary Enable"]
        #[inline(always)]
        pub const fn nie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary Enable"]
        #[inline(always)]
        pub fn set_nie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dmacier {
        #[inline(always)]
        fn default() -> Dmacier {
            Dmacier(0)
        }
    }
    impl core::fmt::Debug for Dmacier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacier")
                .field("tie", &self.tie())
                .field("txse", &self.txse())
                .field("tbue", &self.tbue())
                .field("rie", &self.rie())
                .field("rbue", &self.rbue())
                .field("rse", &self.rse())
                .field("rwte", &self.rwte())
                .field("etie", &self.etie())
                .field("erie", &self.erie())
                .field("fbee", &self.fbee())
                .field("cdee", &self.cdee())
                .field("aie", &self.aie())
                .field("nie", &self.nie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmacier {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}" , self . tie () , self . txse () , self . tbue () , self . rie () , self . rbue () , self . rse () , self . rwte () , self . etie () , self . erie () , self . fbee () , self . cdee () , self . aie () , self . nie ())
        }
    }
    #[doc = "Channel missed frame count register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacmfcr(pub u32);
    impl Dmacmfcr {
        #[doc = "Dropped Packet Counters"]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Dropped Packet Counters"]
        #[inline(always)]
        pub fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow status of the MFC Counter"]
        #[inline(always)]
        pub const fn mfco(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow status of the MFC Counter"]
        #[inline(always)]
        pub fn set_mfco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dmacmfcr {
        #[inline(always)]
        fn default() -> Dmacmfcr {
            Dmacmfcr(0)
        }
    }
    impl core::fmt::Debug for Dmacmfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacmfcr")
                .field("mfc", &self.mfc())
                .field("mfco", &self.mfco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacmfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacmfcr {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
                self.mfc(),
                self.mfco()
            )
        }
    }
    #[doc = "Channel receive control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacrxCr(pub u32);
    impl DmacrxCr {
        #[doc = "Start or Stop Receive Command"]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Receive Command"]
        #[inline(always)]
        pub fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receive Buffer size"]
        #[inline(always)]
        pub const fn rbsz(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "Receive Buffer size"]
        #[inline(always)]
        pub fn set_rbsz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u32) & 0x3fff) << 1usize);
        }
        #[doc = "RXPBL"]
        #[inline(always)]
        pub const fn rxpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "RXPBL"]
        #[inline(always)]
        pub fn set_rxpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "DMA Rx Channel Packet Flush"]
        #[inline(always)]
        pub const fn rpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Rx Channel Packet Flush"]
        #[inline(always)]
        pub fn set_rpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DmacrxCr {
        #[inline(always)]
        fn default() -> DmacrxCr {
            DmacrxCr(0)
        }
    }
    impl core::fmt::Debug for DmacrxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacrxCr")
                .field("sr", &self.sr())
                .field("rbsz", &self.rbsz())
                .field("rxpbl", &self.rxpbl())
                .field("rpf", &self.rpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacrxCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmacrxCr {{ sr: {=bool:?}, rbsz: {=u16:?}, rxpbl: {=u8:?}, rpf: {=bool:?} }}",
                self.sr(),
                self.rbsz(),
                self.rxpbl(),
                self.rpf()
            )
        }
    }
    #[doc = "Channel Rx descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacrxDlar(pub u32);
    impl DmacrxDlar {
        #[doc = "Start of Receive List"]
        #[inline(always)]
        pub const fn rdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Receive List"]
        #[inline(always)]
        pub fn set_rdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacrxDlar {
        #[inline(always)]
        fn default() -> DmacrxDlar {
            DmacrxDlar(0)
        }
    }
    impl core::fmt::Debug for DmacrxDlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacrxDlar").field("rdesla", &self.rdesla()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacrxDlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacrxDlar {{ rdesla: {=u32:?} }}", self.rdesla())
        }
    }
    #[doc = "Channel Rx descriptor tail pointer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacrxDtpr(pub u32);
    impl DmacrxDtpr {
        #[doc = "Receive Descriptor Tail Pointer"]
        #[inline(always)]
        pub const fn rdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive Descriptor Tail Pointer"]
        #[inline(always)]
        pub fn set_rdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacrxDtpr {
        #[inline(always)]
        fn default() -> DmacrxDtpr {
            DmacrxDtpr(0)
        }
    }
    impl core::fmt::Debug for DmacrxDtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacrxDtpr").field("rdt", &self.rdt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacrxDtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacrxDtpr {{ rdt: {=u32:?} }}", self.rdt())
        }
    }
    #[doc = "Channel Rx interrupt watchdog timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacrxIwtr(pub u32);
    impl DmacrxIwtr {
        #[doc = "Receive Interrupt Watchdog Timer Count"]
        #[inline(always)]
        pub const fn rwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive Interrupt Watchdog Timer Count"]
        #[inline(always)]
        pub fn set_rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DmacrxIwtr {
        #[inline(always)]
        fn default() -> DmacrxIwtr {
            DmacrxIwtr(0)
        }
    }
    impl core::fmt::Debug for DmacrxIwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacrxIwtr").field("rwt", &self.rwt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacrxIwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacrxIwtr {{ rwt: {=u8:?} }}", self.rwt())
        }
    }
    #[doc = "Channel Rx descriptor ring length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacrxRlr(pub u32);
    impl DmacrxRlr {
        #[doc = "Receive Descriptor Ring Length"]
        #[inline(always)]
        pub const fn rdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Receive Descriptor Ring Length"]
        #[inline(always)]
        pub fn set_rdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DmacrxRlr {
        #[inline(always)]
        fn default() -> DmacrxRlr {
            DmacrxRlr(0)
        }
    }
    impl core::fmt::Debug for DmacrxRlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacrxRlr").field("rdrl", &self.rdrl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacrxRlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacrxRlr {{ rdrl: {=u16:?} }}", self.rdrl())
        }
    }
    #[doc = "Channel status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacsr(pub u32);
    impl Dmacsr {
        #[doc = "Transmit Interrupt"]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt"]
        #[inline(always)]
        pub fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Process Stopped"]
        #[inline(always)]
        pub const fn tps(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Process Stopped"]
        #[inline(always)]
        pub fn set_tps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable"]
        #[inline(always)]
        pub const fn tbu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable"]
        #[inline(always)]
        pub fn set_tbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive Interrupt"]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt"]
        #[inline(always)]
        pub fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable"]
        #[inline(always)]
        pub const fn rbu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable"]
        #[inline(always)]
        pub fn set_rbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Process Stopped"]
        #[inline(always)]
        pub const fn rps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Process Stopped"]
        #[inline(always)]
        pub fn set_rps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout"]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout"]
        #[inline(always)]
        pub fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt"]
        #[inline(always)]
        pub const fn et(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt"]
        #[inline(always)]
        pub fn set_et(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Early Receive Interrupt"]
        #[inline(always)]
        pub const fn er(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt"]
        #[inline(always)]
        pub fn set_er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Fatal Bus Error"]
        #[inline(always)]
        pub const fn fbe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error"]
        #[inline(always)]
        pub fn set_fbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Context Descriptor Error"]
        #[inline(always)]
        pub const fn cde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Context Descriptor Error"]
        #[inline(always)]
        pub fn set_cde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Abnormal Interrupt Summary"]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary"]
        #[inline(always)]
        pub fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Normal Interrupt Summary"]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary"]
        #[inline(always)]
        pub fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Tx DMA Error Bits"]
        #[inline(always)]
        pub const fn teb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Tx DMA Error Bits"]
        #[inline(always)]
        pub fn set_teb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Rx DMA Error Bits"]
        #[inline(always)]
        pub const fn reb(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[doc = "Rx DMA Error Bits"]
        #[inline(always)]
        pub fn set_reb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
    }
    impl Default for Dmacsr {
        #[inline(always)]
        fn default() -> Dmacsr {
            Dmacsr(0)
        }
    }
    impl core::fmt::Debug for Dmacsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacsr")
                .field("ti", &self.ti())
                .field("tps", &self.tps())
                .field("tbu", &self.tbu())
                .field("ri", &self.ri())
                .field("rbu", &self.rbu())
                .field("rps", &self.rps())
                .field("rwt", &self.rwt())
                .field("et", &self.et())
                .field("er", &self.er())
                .field("fbe", &self.fbe())
                .field("cde", &self.cde())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("teb", &self.teb())
                .field("reb", &self.reb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmacsr {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, et: {=bool:?}, er: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}" , self . ti () , self . tps () , self . tbu () , self . ri () , self . rbu () , self . rps () , self . rwt () , self . et () , self . er () , self . fbe () , self . cde () , self . ais () , self . nis () , self . teb () , self . reb ())
        }
    }
    #[doc = "Channel transmit control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmactxCr(pub u32);
    impl DmactxCr {
        #[doc = "Start or Stop Transmission Command"]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Transmission Command"]
        #[inline(always)]
        pub fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operate on Second Packet"]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Operate on Second Packet"]
        #[inline(always)]
        pub fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TCP Segmentation Enabled"]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TCP Segmentation Enabled"]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit Programmable Burst Length"]
        #[inline(always)]
        pub const fn txpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit Programmable Burst Length"]
        #[inline(always)]
        pub fn set_txpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for DmactxCr {
        #[inline(always)]
        fn default() -> DmactxCr {
            DmactxCr(0)
        }
    }
    impl core::fmt::Debug for DmactxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmactxCr")
                .field("st", &self.st())
                .field("osf", &self.osf())
                .field("tse", &self.tse())
                .field("txpbl", &self.txpbl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmactxCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmactxCr {{ st: {=bool:?}, osf: {=bool:?}, tse: {=bool:?}, txpbl: {=u8:?} }}",
                self.st(),
                self.osf(),
                self.tse(),
                self.txpbl()
            )
        }
    }
    #[doc = "Channel Tx descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmactxDlar(pub u32);
    impl DmactxDlar {
        #[doc = "Start of Transmit List"]
        #[inline(always)]
        pub const fn tdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Transmit List"]
        #[inline(always)]
        pub fn set_tdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmactxDlar {
        #[inline(always)]
        fn default() -> DmactxDlar {
            DmactxDlar(0)
        }
    }
    impl core::fmt::Debug for DmactxDlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmactxDlar").field("tdesla", &self.tdesla()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmactxDlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmactxDlar {{ tdesla: {=u32:?} }}", self.tdesla())
        }
    }
    #[doc = "Channel Tx descriptor tail pointer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmactxDtpr(pub u32);
    impl DmactxDtpr {
        #[doc = "Transmit Descriptor Tail Pointer"]
        #[inline(always)]
        pub const fn tdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit Descriptor Tail Pointer"]
        #[inline(always)]
        pub fn set_tdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmactxDtpr {
        #[inline(always)]
        fn default() -> DmactxDtpr {
            DmactxDtpr(0)
        }
    }
    impl core::fmt::Debug for DmactxDtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmactxDtpr").field("tdt", &self.tdt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmactxDtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmactxDtpr {{ tdt: {=u32:?} }}", self.tdt())
        }
    }
    #[doc = "Channel Tx descriptor ring length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmactxRlr(pub u32);
    impl DmactxRlr {
        #[doc = "Transmit Descriptor Ring Length"]
        #[inline(always)]
        pub const fn tdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Transmit Descriptor Ring Length"]
        #[inline(always)]
        pub fn set_tdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DmactxRlr {
        #[inline(always)]
        fn default() -> DmactxRlr {
            DmactxRlr(0)
        }
    }
    impl core::fmt::Debug for DmactxRlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmactxRlr").field("tdrl", &self.tdrl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmactxRlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmactxRlr {{ tdrl: {=u16:?} }}", self.tdrl())
        }
    }
    #[doc = "Debug status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmadsr(pub u32);
    impl Dmadsr {
        #[doc = "AHB Master Write Channel"]
        #[inline(always)]
        pub const fn axwhsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AHB Master Write Channel"]
        #[inline(always)]
        pub fn set_axwhsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Channel Receive Process State"]
        #[inline(always)]
        pub const fn rps0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "DMA Channel Receive Process State"]
        #[inline(always)]
        pub fn set_rps0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DMA Channel Transmit Process State"]
        #[inline(always)]
        pub const fn tps0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "DMA Channel Transmit Process State"]
        #[inline(always)]
        pub fn set_tps0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Dmadsr {
        #[inline(always)]
        fn default() -> Dmadsr {
            Dmadsr(0)
        }
    }
    impl core::fmt::Debug for Dmadsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmadsr")
                .field("axwhsts", &self.axwhsts())
                .field("rps0", &self.rps0())
                .field("tps0", &self.tps0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmadsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmadsr {{ axwhsts: {=bool:?}, rps0: {=u8:?}, tps0: {=u8:?} }}",
                self.axwhsts(),
                self.rps0(),
                self.tps0()
            )
        }
    }
    #[doc = "Interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaisr(pub u32);
    impl Dmaisr {
        #[doc = "DMA Channel Interrupt Status"]
        #[inline(always)]
        pub const fn dc0is(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Interrupt Status"]
        #[inline(always)]
        pub fn set_dc0is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MTL Interrupt Status"]
        #[inline(always)]
        pub const fn mtlis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Interrupt Status"]
        #[inline(always)]
        pub fn set_mtlis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC Interrupt Status"]
        #[inline(always)]
        pub const fn macis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Interrupt Status"]
        #[inline(always)]
        pub fn set_macis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Dmaisr {
        #[inline(always)]
        fn default() -> Dmaisr {
            Dmaisr(0)
        }
    }
    impl core::fmt::Debug for Dmaisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaisr")
                .field("dc0is", &self.dc0is())
                .field("mtlis", &self.mtlis())
                .field("macis", &self.macis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaisr {{ dc0is: {=bool:?}, mtlis: {=bool:?}, macis: {=bool:?} }}",
                self.dc0is(),
                self.mtlis(),
                self.macis()
            )
        }
    }
    #[doc = "DMA mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamr(pub u32);
    impl Dmamr {
        #[doc = "Software Reset"]
        #[inline(always)]
        pub const fn swr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset"]
        #[inline(always)]
        pub fn set_swr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Tx or Rx Arbitration Scheme"]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Tx or Rx Arbitration Scheme"]
        #[inline(always)]
        pub fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit priority"]
        #[inline(always)]
        pub const fn txpr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit priority"]
        #[inline(always)]
        pub fn set_txpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Priority ratio"]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Priority ratio"]
        #[inline(always)]
        pub fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Interrupt Mode"]
        #[inline(always)]
        pub const fn intm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Interrupt Mode"]
        #[inline(always)]
        pub fn set_intm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Dmamr {
        #[inline(always)]
        fn default() -> Dmamr {
            Dmamr(0)
        }
    }
    impl core::fmt::Debug for Dmamr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmamr")
                .field("swr", &self.swr())
                .field("da", &self.da())
                .field("txpr", &self.txpr())
                .field("pr", &self.pr())
                .field("intm", &self.intm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmamr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmamr {{ swr: {=bool:?}, da: {=bool:?}, txpr: {=bool:?}, pr: {=u8:?}, intm: {=u8:?} }}",
                self.swr(),
                self.da(),
                self.txpr(),
                self.pr(),
                self.intm()
            )
        }
    }
    #[doc = "System bus mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasbmr(pub u32);
    impl Dmasbmr {
        #[doc = "Fixed Burst Length"]
        #[inline(always)]
        pub const fn fb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed Burst Length"]
        #[inline(always)]
        pub fn set_fb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address-Aligned Beats"]
        #[inline(always)]
        pub const fn aal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Address-Aligned Beats"]
        #[inline(always)]
        pub fn set_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Mixed Burst"]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Mixed Burst"]
        #[inline(always)]
        pub fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rebuild INCRx Burst"]
        #[inline(always)]
        pub const fn rb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rebuild INCRx Burst"]
        #[inline(always)]
        pub fn set_rb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dmasbmr {
        #[inline(always)]
        fn default() -> Dmasbmr {
            Dmasbmr(0)
        }
    }
    impl core::fmt::Debug for Dmasbmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmasbmr")
                .field("fb", &self.fb())
                .field("aal", &self.aal())
                .field("mb", &self.mb())
                .field("rb", &self.rb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmasbmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmasbmr {{ fb: {=bool:?}, aal: {=bool:?}, mb: {=bool:?}, rb: {=bool:?} }}",
                self.fb(),
                self.aal(),
                self.mb(),
                self.rb()
            )
        }
    }
    #[doc = "1-microsecond-tick counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mac1ustcr(pub u32);
    impl Mac1ustcr {
        #[doc = "1 µs tick Counter"]
        #[inline(always)]
        pub const fn tic_1us_cntr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "1 µs tick Counter"]
        #[inline(always)]
        pub fn set_tic_1us_cntr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Mac1ustcr {
        #[inline(always)]
        fn default() -> Mac1ustcr {
            Mac1ustcr(0)
        }
    }
    impl core::fmt::Debug for Mac1ustcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mac1ustcr")
                .field("tic_1us_cntr", &self.tic_1us_cntr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mac1ustcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mac1ustcr {{ tic_1us_cntr: {=u16:?} }}", self.tic_1us_cntr())
        }
    }
    #[doc = "Address 0 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "MAC Address0\\[47:32\\]"]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC Address0\\[47:32\\]"]
        #[inline(always)]
        pub fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Address Enable"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable"]
        #[inline(always)]
        pub fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca0hr {
        #[inline(always)]
        fn default() -> Maca0hr {
            Maca0hr(0)
        }
    }
    impl core::fmt::Debug for Maca0hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0hr")
                .field("addrhi", &self.addrhi())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maca0hr {{ addrhi: {=u16:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.ae()
            )
        }
    }
    #[doc = "Address 0 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "MAC Address 0 \\[31:0\\]"]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address 0 \\[31:0\\]"]
        #[inline(always)]
        pub fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca0lr {
        #[inline(always)]
        fn default() -> Maca0lr {
            Maca0lr(0)
        }
    }
    impl core::fmt::Debug for Maca0lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0lr").field("addrlo", &self.addrlo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maca0lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "Auxiliary control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macacr(pub u32);
    impl Macacr {
        #[doc = "Auxiliary Snapshot FIFO Clear"]
        #[inline(always)]
        pub const fn atsfc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot FIFO Clear"]
        #[inline(always)]
        pub fn set_atsfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Auxiliary Snapshot 0-3 Enable"]
        #[inline(always)]
        pub const fn atsen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot 0-3 Enable"]
        #[inline(always)]
        pub fn set_atsen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Macacr {
        #[inline(always)]
        fn default() -> Macacr {
            Macacr(0)
        }
    }
    impl core::fmt::Debug for Macacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macacr")
                .field("atsfc", &self.atsfc())
                .field("atsen[0]", &self.atsen(0usize))
                .field("atsen[1]", &self.atsen(1usize))
                .field("atsen[2]", &self.atsen(2usize))
                .field("atsen[3]", &self.atsen(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macacr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macacr {{ atsfc: {=bool:?}, atsen[0]: {=bool:?}, atsen[1]: {=bool:?}, atsen[2]: {=bool:?}, atsen[3]: {=bool:?} }}" , self . atsfc () , self . atsen (0usize) , self . atsen (1usize) , self . atsen (2usize) , self . atsen (3usize))
        }
    }
    #[doc = "Address 1/2/3 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macahr(pub u32);
    impl Macahr {
        #[doc = "MAC Address 1/2/3 \\[47:32\\]"]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC Address 1/2/3 \\[47:32\\]"]
        #[inline(always)]
        pub fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask Byte Control"]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask Byte Control"]
        #[inline(always)]
        pub fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source Address"]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address Enable"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable"]
        #[inline(always)]
        pub fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macahr {
        #[inline(always)]
        fn default() -> Macahr {
            Macahr(0)
        }
    }
    impl core::fmt::Debug for Macahr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macahr")
                .field("addrhi", &self.addrhi())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macahr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macahr {{ addrhi: {=u16:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[doc = "Address 1/2/3 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macalr(pub u32);
    impl Macalr {
        #[doc = "MAC Address 1/2/3 \\[31:0\\]"]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address 1/2/3 \\[31:0\\]"]
        #[inline(always)]
        pub fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macalr {
        #[inline(always)]
        fn default() -> Macalr {
            Macalr(0)
        }
    }
    impl core::fmt::Debug for Macalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macalr").field("addrlo", &self.addrlo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macalr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "ARP address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macarpar(pub u32);
    impl Macarpar {
        #[doc = "ARP Protocol Address"]
        #[inline(always)]
        pub const fn arppa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ARP Protocol Address"]
        #[inline(always)]
        pub fn set_arppa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macarpar {
        #[inline(always)]
        fn default() -> Macarpar {
            Macarpar(0)
        }
    }
    impl core::fmt::Debug for Macarpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macarpar").field("arppa", &self.arppa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macarpar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macarpar {{ arppa: {=u32:?} }}", self.arppa())
        }
    }
    #[doc = "Auxiliary timestamp nanoseconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macatsnr(pub u32);
    impl Macatsnr {
        #[doc = "Auxiliary Timestamp"]
        #[inline(always)]
        pub const fn auxtslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Auxiliary Timestamp"]
        #[inline(always)]
        pub fn set_auxtslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Macatsnr {
        #[inline(always)]
        fn default() -> Macatsnr {
            Macatsnr(0)
        }
    }
    impl core::fmt::Debug for Macatsnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macatsnr").field("auxtslo", &self.auxtslo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macatsnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macatsnr {{ auxtslo: {=u32:?} }}", self.auxtslo())
        }
    }
    #[doc = "Auxiliary timestamp seconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macatssr(pub u32);
    impl Macatssr {
        #[doc = "Auxiliary Timestamp"]
        #[inline(always)]
        pub const fn auxtshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Auxiliary Timestamp"]
        #[inline(always)]
        pub fn set_auxtshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macatssr {
        #[inline(always)]
        fn default() -> Macatssr {
            Macatssr(0)
        }
    }
    impl core::fmt::Debug for Macatssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macatssr").field("auxtshi", &self.auxtshi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macatssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macatssr {{ auxtshi: {=u32:?} }}", self.auxtshi())
        }
    }
    #[doc = "Operating mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Receiver Enable"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Enable"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmitter Enable"]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Enable"]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Preamble Length for Transmit Packets"]
        #[inline(always)]
        pub const fn prelen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Preamble Length for Transmit Packets"]
        #[inline(always)]
        pub fn set_prelen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Deferral Check"]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Deferral Check"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Back-Off Limit"]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Back-Off Limit"]
        #[inline(always)]
        pub fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Disable Retry"]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Retry"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Disable Carrier Sense During Transmission"]
        #[inline(always)]
        pub const fn dcrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Carrier Sense During Transmission"]
        #[inline(always)]
        pub fn set_dcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Disable Receive Own"]
        #[inline(always)]
        pub const fn do_(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Receive Own"]
        #[inline(always)]
        pub fn set_do_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
        #[inline(always)]
        pub const fn ecrsfd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
        #[inline(always)]
        pub fn set_ecrsfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback Mode"]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Mode"]
        #[inline(always)]
        pub fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Duplex Mode"]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex Mode"]
        #[inline(always)]
        pub fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MAC Speed"]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Speed"]
        #[inline(always)]
        pub fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Jumbo Packet Enable"]
        #[inline(always)]
        pub const fn je(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Jumbo Packet Enable"]
        #[inline(always)]
        pub fn set_je(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Jabber Disable"]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber Disable"]
        #[inline(always)]
        pub fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Watchdog Disable"]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Disable"]
        #[inline(always)]
        pub fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Automatic Pad or CRC Stripping"]
        #[inline(always)]
        pub const fn acs(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Pad or CRC Stripping"]
        #[inline(always)]
        pub fn set_acs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CRC stripping for Type packets"]
        #[inline(always)]
        pub const fn cst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CRC stripping for Type packets"]
        #[inline(always)]
        pub fn set_cst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "IEEE 802.3as Support for 2K Packets"]
        #[inline(always)]
        pub const fn s2kp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 802.3as Support for 2K Packets"]
        #[inline(always)]
        pub fn set_s2kp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Giant Packet Size Limit Control Enable"]
        #[inline(always)]
        pub const fn gpslce(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Giant Packet Size Limit Control Enable"]
        #[inline(always)]
        pub fn set_gpslce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Inter-Packet Gap"]
        #[inline(always)]
        pub const fn ipg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Inter-Packet Gap"]
        #[inline(always)]
        pub fn set_ipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Checksum Offload"]
        #[inline(always)]
        pub const fn ipc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Checksum Offload"]
        #[inline(always)]
        pub fn set_ipc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Source Address Insertion or Replacement Control"]
        #[inline(always)]
        pub const fn sarc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Source Address Insertion or Replacement Control"]
        #[inline(always)]
        pub fn set_sarc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "ARP Offload Enable"]
        #[inline(always)]
        pub const fn arpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ARP Offload Enable"]
        #[inline(always)]
        pub fn set_arpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maccr {
        #[inline(always)]
        fn default() -> Maccr {
            Maccr(0)
        }
    }
    impl core::fmt::Debug for Maccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccr")
                .field("re", &self.re())
                .field("te", &self.te())
                .field("prelen", &self.prelen())
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("dr", &self.dr())
                .field("dcrs", &self.dcrs())
                .field("do_", &self.do_())
                .field("ecrsfd", &self.ecrsfd())
                .field("lm", &self.lm())
                .field("dm", &self.dm())
                .field("fes", &self.fes())
                .field("je", &self.je())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .field("acs", &self.acs())
                .field("cst", &self.cst())
                .field("s2kp", &self.s2kp())
                .field("gpslce", &self.gpslce())
                .field("ipg", &self.ipg())
                .field("ipc", &self.ipc())
                .field("sarc", &self.sarc())
                .field("arpen", &self.arpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maccr {{ re: {=bool:?}, te: {=bool:?}, prelen: {=u8:?}, dc: {=bool:?}, bl: {=u8:?}, dr: {=bool:?}, dcrs: {=bool:?}, do_: {=bool:?}, ecrsfd: {=bool:?}, lm: {=bool:?}, dm: {=bool:?}, fes: {=bool:?}, je: {=bool:?}, jd: {=bool:?}, wd: {=bool:?}, acs: {=bool:?}, cst: {=bool:?}, s2kp: {=bool:?}, gpslce: {=bool:?}, ipg: {=u8:?}, ipc: {=bool:?}, sarc: {=u8:?}, arpen: {=bool:?} }}" , self . re () , self . te () , self . prelen () , self . dc () , self . bl () , self . dr () , self . dcrs () , self . do_ () , self . ecrsfd () , self . lm () , self . dm () , self . fes () , self . je () , self . jd () , self . wd () , self . acs () , self . cst () , self . s2kp () , self . gpslce () , self . ipg () , self . ipc () , self . sarc () , self . arpen ())
        }
    }
    #[doc = "Debug register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macdr(pub u32);
    impl Macdr {
        #[doc = "MAC MII Receive Protocol Engine Status"]
        #[inline(always)]
        pub const fn rpests(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MAC MII Receive Protocol Engine Status"]
        #[inline(always)]
        pub fn set_rpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MAC Receive Packet Controller FIFO Status"]
        #[inline(always)]
        pub const fn rfcfcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MAC Receive Packet Controller FIFO Status"]
        #[inline(always)]
        pub fn set_rfcfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MAC MII Transmit Protocol Engine Status"]
        #[inline(always)]
        pub const fn tpests(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MAC MII Transmit Protocol Engine Status"]
        #[inline(always)]
        pub fn set_tpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC Transmit Packet Controller Status"]
        #[inline(always)]
        pub const fn tfcsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "MAC Transmit Packet Controller Status"]
        #[inline(always)]
        pub fn set_tfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
    }
    impl Default for Macdr {
        #[inline(always)]
        fn default() -> Macdr {
            Macdr(0)
        }
    }
    impl core::fmt::Debug for Macdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macdr")
                .field("rpests", &self.rpests())
                .field("rfcfcsts", &self.rfcfcsts())
                .field("tpests", &self.tpests())
                .field("tfcsts", &self.tfcsts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macdr {{ rpests: {=bool:?}, rfcfcsts: {=u8:?}, tpests: {=bool:?}, tfcsts: {=u8:?} }}",
                self.rpests(),
                self.rfcfcsts(),
                self.tpests(),
                self.tfcsts()
            )
        }
    }
    #[doc = "Extended operating mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macecr(pub u32);
    impl Macecr {
        #[doc = "Giant Packet Size Limit"]
        #[inline(always)]
        pub const fn gpsl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Giant Packet Size Limit"]
        #[inline(always)]
        pub fn set_gpsl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Disable CRC Checking for Received Packets"]
        #[inline(always)]
        pub const fn dcrcc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Disable CRC Checking for Received Packets"]
        #[inline(always)]
        pub fn set_dcrcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Slow Protocol Detection Enable"]
        #[inline(always)]
        pub const fn spen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Slow Protocol Detection Enable"]
        #[inline(always)]
        pub fn set_spen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Unicast Slow Protocol Packet Detect"]
        #[inline(always)]
        pub const fn usp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Slow Protocol Packet Detect"]
        #[inline(always)]
        pub fn set_usp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Extended Inter-Packet Gap Enable"]
        #[inline(always)]
        pub const fn eipgen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Extended Inter-Packet Gap Enable"]
        #[inline(always)]
        pub fn set_eipgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Extended Inter-Packet Gap"]
        #[inline(always)]
        pub const fn eipg(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Extended Inter-Packet Gap"]
        #[inline(always)]
        pub fn set_eipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Macecr {
        #[inline(always)]
        fn default() -> Macecr {
            Macecr(0)
        }
    }
    impl core::fmt::Debug for Macecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macecr")
                .field("gpsl", &self.gpsl())
                .field("dcrcc", &self.dcrcc())
                .field("spen", &self.spen())
                .field("usp", &self.usp())
                .field("eipgen", &self.eipgen())
                .field("eipg", &self.eipg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macecr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macecr {{ gpsl: {=u16:?}, dcrcc: {=bool:?}, spen: {=bool:?}, usp: {=bool:?}, eipgen: {=bool:?}, eipg: {=u8:?} }}" , self . gpsl () , self . dcrcc () , self . spen () , self . usp () , self . eipgen () , self . eipg ())
        }
    }
    #[doc = "Hash Table 0/1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtr(pub u32);
    impl Machtr {
        #[doc = "MAC Hash Table 32 Bits"]
        #[inline(always)]
        pub const fn ht(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Hash Table 32 Bits"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtr {
        #[inline(always)]
        fn default() -> Machtr {
            Machtr(0)
        }
    }
    impl core::fmt::Debug for Machtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machtr").field("ht", &self.ht()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Machtr {{ ht: {=u32:?} }}", self.ht())
        }
    }
    #[doc = "HW feature 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf1r(pub u32);
    impl Machwf1r {
        #[doc = "MTL Receive FIFO Size"]
        #[inline(always)]
        pub const fn rxfifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MTL Receive FIFO Size"]
        #[inline(always)]
        pub fn set_rxfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "MTL Transmit FIFO Size"]
        #[inline(always)]
        pub const fn txfifosize(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "MTL Transmit FIFO Size"]
        #[inline(always)]
        pub fn set_txfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "One-Step Timestamping Enable"]
        #[inline(always)]
        pub const fn osten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "One-Step Timestamping Enable"]
        #[inline(always)]
        pub fn set_osten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PTP Offload Enable"]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PTP Offload Enable"]
        #[inline(always)]
        pub fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "IEEE 1588 High Word Register Enable"]
        #[inline(always)]
        pub const fn advthword(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 1588 High Word Register Enable"]
        #[inline(always)]
        pub fn set_advthword(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Address width"]
        #[inline(always)]
        pub const fn addr64(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Address width"]
        #[inline(always)]
        pub fn set_addr64(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "DCB Feature Enable"]
        #[inline(always)]
        pub const fn dcben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCB Feature Enable"]
        #[inline(always)]
        pub fn set_dcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Split Header Feature Enable"]
        #[inline(always)]
        pub const fn sphen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Split Header Feature Enable"]
        #[inline(always)]
        pub fn set_sphen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TCP Segmentation Offload Enable"]
        #[inline(always)]
        pub const fn tsoen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TCP Segmentation Offload Enable"]
        #[inline(always)]
        pub fn set_tsoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DMA Debug Registers Enable"]
        #[inline(always)]
        pub const fn dbgmema(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Debug Registers Enable"]
        #[inline(always)]
        pub fn set_dbgmema(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "AV Feature Enable"]
        #[inline(always)]
        pub const fn avsel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "AV Feature Enable"]
        #[inline(always)]
        pub fn set_avsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Hash Table Size"]
        #[inline(always)]
        pub const fn hashtblsz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Hash Table Size"]
        #[inline(always)]
        pub fn set_hashtblsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Total number of L3 or L4 Filters"]
        #[inline(always)]
        pub const fn l3l4fnum(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Total number of L3 or L4 Filters"]
        #[inline(always)]
        pub fn set_l3l4fnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
    }
    impl Default for Machwf1r {
        #[inline(always)]
        fn default() -> Machwf1r {
            Machwf1r(0)
        }
    }
    impl core::fmt::Debug for Machwf1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf1r")
                .field("rxfifosize", &self.rxfifosize())
                .field("txfifosize", &self.txfifosize())
                .field("osten", &self.osten())
                .field("ptoen", &self.ptoen())
                .field("advthword", &self.advthword())
                .field("addr64", &self.addr64())
                .field("dcben", &self.dcben())
                .field("sphen", &self.sphen())
                .field("tsoen", &self.tsoen())
                .field("dbgmema", &self.dbgmema())
                .field("avsel", &self.avsel())
                .field("hashtblsz", &self.hashtblsz())
                .field("l3l4fnum", &self.l3l4fnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf1r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Machwf1r {{ rxfifosize: {=u8:?}, txfifosize: {=u8:?}, osten: {=bool:?}, ptoen: {=bool:?}, advthword: {=bool:?}, addr64: {=u8:?}, dcben: {=bool:?}, sphen: {=bool:?}, tsoen: {=bool:?}, dbgmema: {=bool:?}, avsel: {=bool:?}, hashtblsz: {=u8:?}, l3l4fnum: {=u8:?} }}" , self . rxfifosize () , self . txfifosize () , self . osten () , self . ptoen () , self . advthword () , self . addr64 () , self . dcben () , self . sphen () , self . tsoen () , self . dbgmema () , self . avsel () , self . hashtblsz () , self . l3l4fnum ())
        }
    }
    #[doc = "HW feature 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf2r(pub u32);
    impl Machwf2r {
        #[doc = "Number of MTL Receive Queues"]
        #[inline(always)]
        pub const fn rxqcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of MTL Receive Queues"]
        #[inline(always)]
        pub fn set_rxqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Number of MTL Transmit Queues"]
        #[inline(always)]
        pub const fn txqcnt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of MTL Transmit Queues"]
        #[inline(always)]
        pub fn set_txqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "Number of DMA Receive Channels"]
        #[inline(always)]
        pub const fn rxchcnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of DMA Receive Channels"]
        #[inline(always)]
        pub fn set_rxchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Number of DMA Transmit Channels"]
        #[inline(always)]
        pub const fn txchcnt(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of DMA Transmit Channels"]
        #[inline(always)]
        pub fn set_txchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Number of PPS Outputs"]
        #[inline(always)]
        pub const fn ppsoutnum(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Number of PPS Outputs"]
        #[inline(always)]
        pub fn set_ppsoutnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Number of Auxiliary Snapshot Inputs"]
        #[inline(always)]
        pub const fn auxsnapnum(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Auxiliary Snapshot Inputs"]
        #[inline(always)]
        pub fn set_auxsnapnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Machwf2r {
        #[inline(always)]
        fn default() -> Machwf2r {
            Machwf2r(0)
        }
    }
    impl core::fmt::Debug for Machwf2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf2r")
                .field("rxqcnt", &self.rxqcnt())
                .field("txqcnt", &self.txqcnt())
                .field("rxchcnt", &self.rxchcnt())
                .field("txchcnt", &self.txchcnt())
                .field("ppsoutnum", &self.ppsoutnum())
                .field("auxsnapnum", &self.auxsnapnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf2r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Machwf2r {{ rxqcnt: {=u8:?}, txqcnt: {=u8:?}, rxchcnt: {=u8:?}, txchcnt: {=u8:?}, ppsoutnum: {=u8:?}, auxsnapnum: {=u8:?} }}" , self . rxqcnt () , self . txqcnt () , self . rxchcnt () , self . txchcnt () , self . ppsoutnum () , self . auxsnapnum ())
        }
    }
    #[doc = "Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macier(pub u32);
    impl Macier {
        #[doc = "PHY Interrupt Enable"]
        #[inline(always)]
        pub const fn phyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Interrupt Enable"]
        #[inline(always)]
        pub fn set_phyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PMT Interrupt Enable"]
        #[inline(always)]
        pub const fn pmtie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Enable"]
        #[inline(always)]
        pub fn set_pmtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPI Interrupt Enable"]
        #[inline(always)]
        pub const fn lpiie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Enable"]
        #[inline(always)]
        pub fn set_lpiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timestamp Interrupt Enable"]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Enable"]
        #[inline(always)]
        pub fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit Status Interrupt Enable"]
        #[inline(always)]
        pub const fn txstsie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Status Interrupt Enable"]
        #[inline(always)]
        pub fn set_txstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Receive Status Interrupt Enable"]
        #[inline(always)]
        pub const fn rxstsie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Status Interrupt Enable"]
        #[inline(always)]
        pub fn set_rxstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Macier {
        #[inline(always)]
        fn default() -> Macier {
            Macier(0)
        }
    }
    impl core::fmt::Debug for Macier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macier")
                .field("phyie", &self.phyie())
                .field("pmtie", &self.pmtie())
                .field("lpiie", &self.lpiie())
                .field("tsie", &self.tsie())
                .field("txstsie", &self.txstsie())
                .field("rxstsie", &self.rxstsie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macier {{ phyie: {=bool:?}, pmtie: {=bool:?}, lpiie: {=bool:?}, tsie: {=bool:?}, txstsie: {=bool:?}, rxstsie: {=bool:?} }}" , self . phyie () , self . pmtie () , self . lpiie () , self . tsie () , self . txstsie () , self . rxstsie ())
        }
    }
    #[doc = "Interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macisr(pub u32);
    impl Macisr {
        #[doc = "PHY Interrupt"]
        #[inline(always)]
        pub const fn phyis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Interrupt"]
        #[inline(always)]
        pub fn set_phyis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PMT Interrupt Status"]
        #[inline(always)]
        pub const fn pmtis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Status"]
        #[inline(always)]
        pub fn set_pmtis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPI Interrupt Status"]
        #[inline(always)]
        pub const fn lpiis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Status"]
        #[inline(always)]
        pub fn set_lpiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Interrupt Status"]
        #[inline(always)]
        pub const fn mmcis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Interrupt Status"]
        #[inline(always)]
        pub fn set_mmcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive Interrupt Status"]
        #[inline(always)]
        pub const fn mmcrxis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Interrupt Status"]
        #[inline(always)]
        pub fn set_mmcrxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Transmit Interrupt Status"]
        #[inline(always)]
        pub const fn mmctxis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Interrupt Status"]
        #[inline(always)]
        pub fn set_mmctxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timestamp Interrupt Status"]
        #[inline(always)]
        pub const fn tsis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Status"]
        #[inline(always)]
        pub fn set_tsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit Status Interrupt"]
        #[inline(always)]
        pub const fn txstsis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Status Interrupt"]
        #[inline(always)]
        pub fn set_txstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Receive Status Interrupt"]
        #[inline(always)]
        pub const fn rxstsis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Status Interrupt"]
        #[inline(always)]
        pub fn set_rxstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Macisr {
        #[inline(always)]
        fn default() -> Macisr {
            Macisr(0)
        }
    }
    impl core::fmt::Debug for Macisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macisr")
                .field("phyis", &self.phyis())
                .field("pmtis", &self.pmtis())
                .field("lpiis", &self.lpiis())
                .field("mmcis", &self.mmcis())
                .field("mmcrxis", &self.mmcrxis())
                .field("mmctxis", &self.mmctxis())
                .field("tsis", &self.tsis())
                .field("txstsis", &self.txstsis())
                .field("rxstsis", &self.rxstsis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macisr {{ phyis: {=bool:?}, pmtis: {=bool:?}, lpiis: {=bool:?}, mmcis: {=bool:?}, mmcrxis: {=bool:?}, mmctxis: {=bool:?}, tsis: {=bool:?}, txstsis: {=bool:?}, rxstsis: {=bool:?} }}" , self . phyis () , self . pmtis () , self . lpiis () , self . mmcis () , self . mmcrxis () , self . mmctxis () , self . tsis () , self . txstsis () , self . rxstsis ())
        }
    }
    #[doc = "Inner VLAN inclusion register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macivir(pub u32);
    impl Macivir {
        #[doc = "VLAN Tag for Transmit Packets"]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag for Transmit Packets"]
        #[inline(always)]
        pub fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Control in Transmit Packets"]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "VLAN Tag Control in Transmit Packets"]
        #[inline(always)]
        pub fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VLAN Priority Control"]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Priority Control"]
        #[inline(always)]
        pub fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "C-VLAN or S-VLAN"]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "C-VLAN or S-VLAN"]
        #[inline(always)]
        pub fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "VLAN Tag Input"]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Input"]
        #[inline(always)]
        pub fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Macivir {
        #[inline(always)]
        fn default() -> Macivir {
            Macivir(0)
        }
    }
    impl core::fmt::Debug for Macivir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macivir")
                .field("vlt", &self.vlt())
                .field("vlc", &self.vlc())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macivir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macivir {{ vlt: {=u16:?}, vlc: {=u8:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?} }}",
                self.vlt(),
                self.vlc(),
                self.vlp(),
                self.csvl(),
                self.vlti()
            )
        }
    }
    #[doc = "MACL3A00R"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a00r(pub u32);
    impl Macl3a00r {
        #[doc = "Layer 3 Address 0 Field"]
        #[inline(always)]
        pub const fn l3a00(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 0 Field"]
        #[inline(always)]
        pub fn set_l3a00(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a00r {
        #[inline(always)]
        fn default() -> Macl3a00r {
            Macl3a00r(0)
        }
    }
    impl core::fmt::Debug for Macl3a00r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a00r").field("l3a00", &self.l3a00()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a00r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a00r {{ l3a00: {=u32:?} }}", self.l3a00())
        }
    }
    #[doc = "Layer3 address 0 filter 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a01r(pub u32);
    impl Macl3a01r {
        #[doc = "Layer 3 Address 0 Field"]
        #[inline(always)]
        pub const fn l3a01(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 0 Field"]
        #[inline(always)]
        pub fn set_l3a01(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a01r {
        #[inline(always)]
        fn default() -> Macl3a01r {
            Macl3a01r(0)
        }
    }
    impl core::fmt::Debug for Macl3a01r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a01r").field("l3a01", &self.l3a01()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a01r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a01r {{ l3a01: {=u32:?} }}", self.l3a01())
        }
    }
    #[doc = "Layer3 address 1 filter 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a10r(pub u32);
    impl Macl3a10r {
        #[doc = "Layer 3 Address 1 Field"]
        #[inline(always)]
        pub const fn l3a10(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 1 Field"]
        #[inline(always)]
        pub fn set_l3a10(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a10r {
        #[inline(always)]
        fn default() -> Macl3a10r {
            Macl3a10r(0)
        }
    }
    impl core::fmt::Debug for Macl3a10r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a10r").field("l3a10", &self.l3a10()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a10r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a10r {{ l3a10: {=u32:?} }}", self.l3a10())
        }
    }
    #[doc = "Layer3 address 1 filter 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a11r(pub u32);
    impl Macl3a11r {
        #[doc = "Layer 3 Address 1 Field"]
        #[inline(always)]
        pub const fn l3a11(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 1 Field"]
        #[inline(always)]
        pub fn set_l3a11(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a11r {
        #[inline(always)]
        fn default() -> Macl3a11r {
            Macl3a11r(0)
        }
    }
    impl core::fmt::Debug for Macl3a11r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a11r").field("l3a11", &self.l3a11()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a11r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a11r {{ l3a11: {=u32:?} }}", self.l3a11())
        }
    }
    #[doc = "Layer3 Address 2 filter 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a20(pub u32);
    impl Macl3a20 {
        #[doc = "Layer 3 Address 2 Field"]
        #[inline(always)]
        pub const fn l3a20(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 2 Field"]
        #[inline(always)]
        pub fn set_l3a20(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a20 {
        #[inline(always)]
        fn default() -> Macl3a20 {
            Macl3a20(0)
        }
    }
    impl core::fmt::Debug for Macl3a20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a20").field("l3a20", &self.l3a20()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a20 {{ l3a20: {=u32:?} }}", self.l3a20())
        }
    }
    #[doc = "Layer3 address 2 filter 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a21r(pub u32);
    impl Macl3a21r {
        #[doc = "Layer 3 Address 2 Field"]
        #[inline(always)]
        pub const fn l3a21(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 2 Field"]
        #[inline(always)]
        pub fn set_l3a21(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a21r {
        #[inline(always)]
        fn default() -> Macl3a21r {
            Macl3a21r(0)
        }
    }
    impl core::fmt::Debug for Macl3a21r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a21r").field("l3a21", &self.l3a21()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a21r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a21r {{ l3a21: {=u32:?} }}", self.l3a21())
        }
    }
    #[doc = "Layer3 Address 3 filter 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a30(pub u32);
    impl Macl3a30 {
        #[doc = "Layer 3 Address 3 Field"]
        #[inline(always)]
        pub const fn l3a30(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 3 Field"]
        #[inline(always)]
        pub fn set_l3a30(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a30 {
        #[inline(always)]
        fn default() -> Macl3a30 {
            Macl3a30(0)
        }
    }
    impl core::fmt::Debug for Macl3a30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a30").field("l3a30", &self.l3a30()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a30 {{ l3a30: {=u32:?} }}", self.l3a30())
        }
    }
    #[doc = "Layer3 address 3 filter 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a31r(pub u32);
    impl Macl3a31r {
        #[doc = "Layer 3 Address 3 Field"]
        #[inline(always)]
        pub const fn l3a31(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 3 Field"]
        #[inline(always)]
        pub fn set_l3a31(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a31r {
        #[inline(always)]
        fn default() -> Macl3a31r {
            Macl3a31r(0)
        }
    }
    impl core::fmt::Debug for Macl3a31r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a31r").field("l3a31", &self.l3a31()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a31r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a31r {{ l3a31: {=u32:?} }}", self.l3a31())
        }
    }
    #[doc = "L3 and L4 control 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3l4c0r(pub u32);
    impl Macl3l4c0r {
        #[doc = "Layer 3 Protocol Enable"]
        #[inline(always)]
        pub const fn l3pen0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 Protocol Enable"]
        #[inline(always)]
        pub fn set_l3pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Layer 3 IP SA Match Enable"]
        #[inline(always)]
        pub const fn l3sam0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Match Enable"]
        #[inline(always)]
        pub fn set_l3sam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable"]
        #[inline(always)]
        pub const fn l3saim0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l3saim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Layer 3 IP DA Match Enable"]
        #[inline(always)]
        pub const fn l3dam0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Match Enable"]
        #[inline(always)]
        pub fn set_l3dam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable"]
        #[inline(always)]
        pub const fn l3daim0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l3daim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Layer 3 IP SA Higher Bits Match"]
        #[inline(always)]
        pub const fn l3hsbm0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP SA Higher Bits Match"]
        #[inline(always)]
        pub fn set_l3hsbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Layer 3 IP DA Higher Bits Match"]
        #[inline(always)]
        pub const fn l3hdbm0(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP DA Higher Bits Match"]
        #[inline(always)]
        pub fn set_l3hdbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Layer 4 Protocol Enable"]
        #[inline(always)]
        pub const fn l4pen0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Protocol Enable"]
        #[inline(always)]
        pub fn set_l4pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 4 Source Port Match Enable"]
        #[inline(always)]
        pub const fn l4spm0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Match Enable"]
        #[inline(always)]
        pub fn set_l4spm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable"]
        #[inline(always)]
        pub const fn l4spim0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l4spim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Layer 4 Destination Port Match Enable"]
        #[inline(always)]
        pub const fn l4dpm0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Match Enable"]
        #[inline(always)]
        pub fn set_l4dpm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable"]
        #[inline(always)]
        pub const fn l4dpim0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l4dpim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Macl3l4c0r {
        #[inline(always)]
        fn default() -> Macl3l4c0r {
            Macl3l4c0r(0)
        }
    }
    impl core::fmt::Debug for Macl3l4c0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3l4c0r")
                .field("l3pen0", &self.l3pen0())
                .field("l3sam0", &self.l3sam0())
                .field("l3saim0", &self.l3saim0())
                .field("l3dam0", &self.l3dam0())
                .field("l3daim0", &self.l3daim0())
                .field("l3hsbm0", &self.l3hsbm0())
                .field("l3hdbm0", &self.l3hdbm0())
                .field("l4pen0", &self.l4pen0())
                .field("l4spm0", &self.l4spm0())
                .field("l4spim0", &self.l4spim0())
                .field("l4dpm0", &self.l4dpm0())
                .field("l4dpim0", &self.l4dpim0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3l4c0r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macl3l4c0r {{ l3pen0: {=bool:?}, l3sam0: {=bool:?}, l3saim0: {=bool:?}, l3dam0: {=bool:?}, l3daim0: {=bool:?}, l3hsbm0: {=u8:?}, l3hdbm0: {=u8:?}, l4pen0: {=bool:?}, l4spm0: {=bool:?}, l4spim0: {=bool:?}, l4dpm0: {=bool:?}, l4dpim0: {=bool:?} }}" , self . l3pen0 () , self . l3sam0 () , self . l3saim0 () , self . l3dam0 () , self . l3daim0 () , self . l3hsbm0 () , self . l3hdbm0 () , self . l4pen0 () , self . l4spm0 () , self . l4spim0 () , self . l4dpm0 () , self . l4dpim0 ())
        }
    }
    #[doc = "L3 and L4 control 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3l4c1r(pub u32);
    impl Macl3l4c1r {
        #[doc = "Layer 3 Protocol Enable"]
        #[inline(always)]
        pub const fn l3pen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 Protocol Enable"]
        #[inline(always)]
        pub fn set_l3pen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Layer 3 IP SA Match Enable"]
        #[inline(always)]
        pub const fn l3sam1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Match Enable"]
        #[inline(always)]
        pub fn set_l3sam1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable"]
        #[inline(always)]
        pub const fn l3saim1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l3saim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Layer 3 IP DA Match Enable"]
        #[inline(always)]
        pub const fn l3dam1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Match Enable"]
        #[inline(always)]
        pub fn set_l3dam1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable"]
        #[inline(always)]
        pub const fn l3daim1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l3daim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Layer 3 IP SA Higher Bits Match"]
        #[inline(always)]
        pub const fn l3hsbm1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP SA Higher Bits Match"]
        #[inline(always)]
        pub fn set_l3hsbm1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Layer 3 IP DA Higher Bits Match"]
        #[inline(always)]
        pub const fn l3hdbm1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP DA Higher Bits Match"]
        #[inline(always)]
        pub fn set_l3hdbm1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Layer 4 Protocol Enable"]
        #[inline(always)]
        pub const fn l4pen1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Protocol Enable"]
        #[inline(always)]
        pub fn set_l4pen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 4 Source Port Match Enable"]
        #[inline(always)]
        pub const fn l4spm1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Match Enable"]
        #[inline(always)]
        pub fn set_l4spm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable"]
        #[inline(always)]
        pub const fn l4spim1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l4spim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Layer 4 Destination Port Match Enable"]
        #[inline(always)]
        pub const fn l4dpm1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Match Enable"]
        #[inline(always)]
        pub fn set_l4dpm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable"]
        #[inline(always)]
        pub const fn l4dpim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable"]
        #[inline(always)]
        pub fn set_l4dpim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Macl3l4c1r {
        #[inline(always)]
        fn default() -> Macl3l4c1r {
            Macl3l4c1r(0)
        }
    }
    impl core::fmt::Debug for Macl3l4c1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3l4c1r")
                .field("l3pen1", &self.l3pen1())
                .field("l3sam1", &self.l3sam1())
                .field("l3saim1", &self.l3saim1())
                .field("l3dam1", &self.l3dam1())
                .field("l3daim1", &self.l3daim1())
                .field("l3hsbm1", &self.l3hsbm1())
                .field("l3hdbm1", &self.l3hdbm1())
                .field("l4pen1", &self.l4pen1())
                .field("l4spm1", &self.l4spm1())
                .field("l4spim1", &self.l4spim1())
                .field("l4dpm1", &self.l4dpm1())
                .field("l4dpim1", &self.l4dpim1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3l4c1r {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macl3l4c1r {{ l3pen1: {=bool:?}, l3sam1: {=bool:?}, l3saim1: {=bool:?}, l3dam1: {=bool:?}, l3daim1: {=bool:?}, l3hsbm1: {=u8:?}, l3hdbm1: {=u8:?}, l4pen1: {=bool:?}, l4spm1: {=bool:?}, l4spim1: {=bool:?}, l4dpm1: {=bool:?}, l4dpim1: {=bool:?} }}" , self . l3pen1 () , self . l3sam1 () , self . l3saim1 () , self . l3dam1 () , self . l3daim1 () , self . l3hsbm1 () , self . l3hdbm1 () , self . l4pen1 () , self . l4spm1 () , self . l4spim1 () , self . l4dpm1 () , self . l4dpim1 ())
        }
    }
    #[doc = "Layer4 address filter 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl4a0r(pub u32);
    impl Macl4a0r {
        #[doc = "Layer 4 Source Port Number Field"]
        #[inline(always)]
        pub const fn l4sp0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Source Port Number Field"]
        #[inline(always)]
        pub fn set_l4sp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Layer 4 Destination Port Number Field"]
        #[inline(always)]
        pub const fn l4dp0(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Destination Port Number Field"]
        #[inline(always)]
        pub fn set_l4dp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macl4a0r {
        #[inline(always)]
        fn default() -> Macl4a0r {
            Macl4a0r(0)
        }
    }
    impl core::fmt::Debug for Macl4a0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl4a0r")
                .field("l4sp0", &self.l4sp0())
                .field("l4dp0", &self.l4dp0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl4a0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl4a0r {{ l4sp0: {=u16:?}, l4dp0: {=u16:?} }}",
                self.l4sp0(),
                self.l4dp0()
            )
        }
    }
    #[doc = "Layer 4 address filter 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl4a1r(pub u32);
    impl Macl4a1r {
        #[doc = "Layer 4 Source Port Number Field"]
        #[inline(always)]
        pub const fn l4sp1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Source Port Number Field"]
        #[inline(always)]
        pub fn set_l4sp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Layer 4 Destination Port Number Field"]
        #[inline(always)]
        pub const fn l4dp1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Destination Port Number Field"]
        #[inline(always)]
        pub fn set_l4dp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macl4a1r {
        #[inline(always)]
        fn default() -> Macl4a1r {
            Macl4a1r(0)
        }
    }
    impl core::fmt::Debug for Macl4a1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl4a1r")
                .field("l4sp1", &self.l4sp1())
                .field("l4dp1", &self.l4dp1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl4a1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl4a1r {{ l4sp1: {=u16:?}, l4dp1: {=u16:?} }}",
                self.l4sp1(),
                self.l4dp1()
            )
        }
    }
    #[doc = "LPI control status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maclcsr(pub u32);
    impl Maclcsr {
        #[doc = "Transmit LPI Entry"]
        #[inline(always)]
        pub const fn tlpien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Entry"]
        #[inline(always)]
        pub fn set_tlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit LPI Exit"]
        #[inline(always)]
        pub const fn tlpiex(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Exit"]
        #[inline(always)]
        pub fn set_tlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive LPI Entry"]
        #[inline(always)]
        pub const fn rlpien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Entry"]
        #[inline(always)]
        pub fn set_rlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive LPI Exit"]
        #[inline(always)]
        pub const fn rlpiex(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Exit"]
        #[inline(always)]
        pub fn set_rlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit LPI State"]
        #[inline(always)]
        pub const fn tlpist(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI State"]
        #[inline(always)]
        pub fn set_tlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive LPI State"]
        #[inline(always)]
        pub const fn rlpist(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI State"]
        #[inline(always)]
        pub fn set_rlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPI Enable"]
        #[inline(always)]
        pub const fn lpien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Enable"]
        #[inline(always)]
        pub fn set_lpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY Link Status"]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status"]
        #[inline(always)]
        pub fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY Link Status Enable"]
        #[inline(always)]
        pub const fn plsen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status Enable"]
        #[inline(always)]
        pub fn set_plsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPI Tx Automate"]
        #[inline(always)]
        pub const fn lpitxa(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Tx Automate"]
        #[inline(always)]
        pub fn set_lpitxa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "LPI Timer Enable"]
        #[inline(always)]
        pub const fn lpite(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Timer Enable"]
        #[inline(always)]
        pub fn set_lpite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Maclcsr {
        #[inline(always)]
        fn default() -> Maclcsr {
            Maclcsr(0)
        }
    }
    impl core::fmt::Debug for Maclcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maclcsr")
                .field("tlpien", &self.tlpien())
                .field("tlpiex", &self.tlpiex())
                .field("rlpien", &self.rlpien())
                .field("rlpiex", &self.rlpiex())
                .field("tlpist", &self.tlpist())
                .field("rlpist", &self.rlpist())
                .field("lpien", &self.lpien())
                .field("pls", &self.pls())
                .field("plsen", &self.plsen())
                .field("lpitxa", &self.lpitxa())
                .field("lpite", &self.lpite())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maclcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maclcsr {{ tlpien: {=bool:?}, tlpiex: {=bool:?}, rlpien: {=bool:?}, rlpiex: {=bool:?}, tlpist: {=bool:?}, rlpist: {=bool:?}, lpien: {=bool:?}, pls: {=bool:?}, plsen: {=bool:?}, lpitxa: {=bool:?}, lpite: {=bool:?} }}" , self . tlpien () , self . tlpiex () , self . rlpien () , self . rlpiex () , self . tlpist () , self . rlpist () , self . lpien () , self . pls () , self . plsen () , self . lpitxa () , self . lpite ())
        }
    }
    #[doc = "LPI entry timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macletr(pub u32);
    impl Macletr {
        #[doc = "LPI Entry Timer"]
        #[inline(always)]
        pub const fn lpiet(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "LPI Entry Timer"]
        #[inline(always)]
        pub fn set_lpiet(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 3usize)) | (((val as u32) & 0x0001_ffff) << 3usize);
        }
    }
    impl Default for Macletr {
        #[inline(always)]
        fn default() -> Macletr {
            Macletr(0)
        }
    }
    impl core::fmt::Debug for Macletr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macletr").field("lpiet", &self.lpiet()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macletr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macletr {{ lpiet: {=u32:?} }}", self.lpiet())
        }
    }
    #[doc = "Log message interval register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maclmir(pub u32);
    impl Maclmir {
        #[doc = "Log Sync Interval"]
        #[inline(always)]
        pub const fn lsi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Log Sync Interval"]
        #[inline(always)]
        pub fn set_lsi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Delay_Req to SYNC Ratio"]
        #[inline(always)]
        pub const fn drsyncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Delay_Req to SYNC Ratio"]
        #[inline(always)]
        pub fn set_drsyncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Log Min Pdelay_Req Interval"]
        #[inline(always)]
        pub const fn lmpdri(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Log Min Pdelay_Req Interval"]
        #[inline(always)]
        pub fn set_lmpdri(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Maclmir {
        #[inline(always)]
        fn default() -> Maclmir {
            Maclmir(0)
        }
    }
    impl core::fmt::Debug for Maclmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maclmir")
                .field("lsi", &self.lsi())
                .field("drsyncr", &self.drsyncr())
                .field("lmpdri", &self.lmpdri())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maclmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maclmir {{ lsi: {=u8:?}, drsyncr: {=u8:?}, lmpdri: {=u8:?} }}",
                self.lsi(),
                self.drsyncr(),
                self.lmpdri()
            )
        }
    }
    #[doc = "LPI timers control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macltcr(pub u32);
    impl Macltcr {
        #[doc = "LPI TW Timer"]
        #[inline(always)]
        pub const fn twt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "LPI TW Timer"]
        #[inline(always)]
        pub fn set_twt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "LPI LS Timer"]
        #[inline(always)]
        pub const fn lst(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "LPI LS Timer"]
        #[inline(always)]
        pub fn set_lst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Macltcr {
        #[inline(always)]
        fn default() -> Macltcr {
            Macltcr(0)
        }
    }
    impl core::fmt::Debug for Macltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macltcr")
                .field("twt", &self.twt())
                .field("lst", &self.lst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macltcr {{ twt: {=u16:?}, lst: {=u16:?} }}", self.twt(), self.lst())
        }
    }
    #[doc = "MDIO address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmdioar(pub u32);
    impl Macmdioar {
        #[doc = "MII Busy"]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MII Busy"]
        #[inline(always)]
        pub fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clause 45 PHY Enable"]
        #[inline(always)]
        pub const fn c45e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clause 45 PHY Enable"]
        #[inline(always)]
        pub fn set_c45e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MII Operation Command"]
        #[inline(always)]
        pub const fn goc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "MII Operation Command"]
        #[inline(always)]
        pub fn set_goc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Skip Address Packet"]
        #[inline(always)]
        pub const fn skap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Skip Address Packet"]
        #[inline(always)]
        pub fn set_skap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CSR Clock Range"]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "CSR Clock Range"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Number of Training Clocks"]
        #[inline(always)]
        pub const fn ntc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Training Clocks"]
        #[inline(always)]
        pub fn set_ntc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Register/Device Address"]
        #[inline(always)]
        pub const fn rda(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Register/Device Address"]
        #[inline(always)]
        pub fn set_rda(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Physical Layer Address"]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "Physical Layer Address"]
        #[inline(always)]
        pub fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Back to Back transactions"]
        #[inline(always)]
        pub const fn btb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Back to Back transactions"]
        #[inline(always)]
        pub fn set_btb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Preamble Suppression Enable"]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preamble Suppression Enable"]
        #[inline(always)]
        pub fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Macmdioar {
        #[inline(always)]
        fn default() -> Macmdioar {
            Macmdioar(0)
        }
    }
    impl core::fmt::Debug for Macmdioar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmdioar")
                .field("mb", &self.mb())
                .field("c45e", &self.c45e())
                .field("goc", &self.goc())
                .field("skap", &self.skap())
                .field("cr", &self.cr())
                .field("ntc", &self.ntc())
                .field("rda", &self.rda())
                .field("pa", &self.pa())
                .field("btb", &self.btb())
                .field("pse", &self.pse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmdioar {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macmdioar {{ mb: {=bool:?}, c45e: {=bool:?}, goc: {=u8:?}, skap: {=bool:?}, cr: {=u8:?}, ntc: {=u8:?}, rda: {=u8:?}, pa: {=u8:?}, btb: {=bool:?}, pse: {=bool:?} }}" , self . mb () , self . c45e () , self . goc () , self . skap () , self . cr () , self . ntc () , self . rda () , self . pa () , self . btb () , self . pse ())
        }
    }
    #[doc = "MDIO data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmdiodr(pub u32);
    impl Macmdiodr {
        #[doc = "MII Data"]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MII Data"]
        #[inline(always)]
        pub fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Register Address"]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Register Address"]
        #[inline(always)]
        pub fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macmdiodr {
        #[inline(always)]
        fn default() -> Macmdiodr {
            Macmdiodr(0)
        }
    }
    impl core::fmt::Debug for Macmdiodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmdiodr")
                .field("md", &self.md())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmdiodr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macmdiodr {{ md: {=u16:?}, ra: {=u16:?} }}", self.md(), self.ra())
        }
    }
    #[doc = "PMT control status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpcsr(pub u32);
    impl Macpcsr {
        #[doc = "Power Down"]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power Down"]
        #[inline(always)]
        pub fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic Packet Enable"]
        #[inline(always)]
        pub const fn mgkpkten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Enable"]
        #[inline(always)]
        pub fn set_mgkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remote wakeup Packet Enable"]
        #[inline(always)]
        pub const fn rwkpkten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wakeup Packet Enable"]
        #[inline(always)]
        pub fn set_rwkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Magic Packet Received"]
        #[inline(always)]
        pub const fn mgkprcvd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Received"]
        #[inline(always)]
        pub fn set_mgkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Remote wakeup Packet Received"]
        #[inline(always)]
        pub const fn rwkprcvd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wakeup Packet Received"]
        #[inline(always)]
        pub fn set_rwkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Unicast"]
        #[inline(always)]
        pub const fn glblucast(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Unicast"]
        #[inline(always)]
        pub fn set_glblucast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Remote wakeup Packet Forwarding Enable"]
        #[inline(always)]
        pub const fn rwkpfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wakeup Packet Forwarding Enable"]
        #[inline(always)]
        pub fn set_rwkpfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Remote wakeup FIFO Pointer"]
        #[inline(always)]
        pub const fn rwkptr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Remote wakeup FIFO Pointer"]
        #[inline(always)]
        pub fn set_rwkptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Remote wakeup Packet Filter Register Pointer Reset"]
        #[inline(always)]
        pub const fn rwkfiltrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wakeup Packet Filter Register Pointer Reset"]
        #[inline(always)]
        pub fn set_rwkfiltrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpcsr {
        #[inline(always)]
        fn default() -> Macpcsr {
            Macpcsr(0)
        }
    }
    impl core::fmt::Debug for Macpcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpcsr")
                .field("pwrdwn", &self.pwrdwn())
                .field("mgkpkten", &self.mgkpkten())
                .field("rwkpkten", &self.rwkpkten())
                .field("mgkprcvd", &self.mgkprcvd())
                .field("rwkprcvd", &self.rwkprcvd())
                .field("glblucast", &self.glblucast())
                .field("rwkpfe", &self.rwkpfe())
                .field("rwkptr", &self.rwkptr())
                .field("rwkfiltrst", &self.rwkfiltrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macpcsr {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, rwkpkten: {=bool:?}, mgkprcvd: {=bool:?}, rwkprcvd: {=bool:?}, glblucast: {=bool:?}, rwkpfe: {=bool:?}, rwkptr: {=u8:?}, rwkfiltrst: {=bool:?} }}" , self . pwrdwn () , self . mgkpkten () , self . rwkpkten () , self . mgkprcvd () , self . rwkprcvd () , self . glblucast () , self . rwkpfe () , self . rwkptr () , self . rwkfiltrst ())
        }
    }
    #[doc = "Packet filtering control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpfr(pub u32);
    impl Macpfr {
        #[doc = "Promiscuous Mode"]
        #[inline(always)]
        pub const fn pr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous Mode"]
        #[inline(always)]
        pub fn set_pr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash Unicast"]
        #[inline(always)]
        pub const fn huc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Unicast"]
        #[inline(always)]
        pub fn set_huc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash Multicast"]
        #[inline(always)]
        pub const fn hmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Multicast"]
        #[inline(always)]
        pub fn set_hmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DA Inverse Filtering"]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DA Inverse Filtering"]
        #[inline(always)]
        pub fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass All Multicast"]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass All Multicast"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable Broadcast Packets"]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Broadcast Packets"]
        #[inline(always)]
        pub fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass Control Packets"]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass Control Packets"]
        #[inline(always)]
        pub fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "SA Inverse Filtering"]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SA Inverse Filtering"]
        #[inline(always)]
        pub fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source Address Filter Enable"]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address Filter Enable"]
        #[inline(always)]
        pub fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or Perfect Filter"]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or Perfect Filter"]
        #[inline(always)]
        pub fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "VLAN Tag Filter Enable"]
        #[inline(always)]
        pub const fn vtfe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Filter Enable"]
        #[inline(always)]
        pub fn set_vtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable"]
        #[inline(always)]
        pub const fn ipfe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable"]
        #[inline(always)]
        pub fn set_ipfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Drop Non-TCP/UDP over IP Packets"]
        #[inline(always)]
        pub const fn dntu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Non-TCP/UDP over IP Packets"]
        #[inline(always)]
        pub fn set_dntu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Receive All"]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive All"]
        #[inline(always)]
        pub fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpfr {
        #[inline(always)]
        fn default() -> Macpfr {
            Macpfr(0)
        }
    }
    impl core::fmt::Debug for Macpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpfr")
                .field("pr", &self.pr())
                .field("huc", &self.huc())
                .field("hmc", &self.hmc())
                .field("daif", &self.daif())
                .field("pm", &self.pm())
                .field("dbf", &self.dbf())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("vtfe", &self.vtfe())
                .field("ipfe", &self.ipfe())
                .field("dntu", &self.dntu())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macpfr {{ pr: {=bool:?}, huc: {=bool:?}, hmc: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {=bool:?}, pcf: {=u8:?}, saif: {=bool:?}, saf: {=bool:?}, hpf: {=bool:?}, vtfe: {=bool:?}, ipfe: {=bool:?}, dntu: {=bool:?}, ra: {=bool:?} }}" , self . pr () , self . huc () , self . hmc () , self . daif () , self . pm () , self . dbf () , self . pcf () , self . saif () , self . saf () , self . hpf () , self . vtfe () , self . ipfe () , self . dntu () , self . ra ())
        }
    }
    #[doc = "PTP Offload control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpocr(pub u32);
    impl Macpocr {
        #[doc = "PTP Offload Enable"]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PTP Offload Enable"]
        #[inline(always)]
        pub fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic PTP SYNC message Enable"]
        #[inline(always)]
        pub const fn asyncen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP SYNC message Enable"]
        #[inline(always)]
        pub fn set_asyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Automatic PTP Pdelay_Req message Enable"]
        #[inline(always)]
        pub const fn apdreqen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP Pdelay_Req message Enable"]
        #[inline(always)]
        pub fn set_apdreqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Automatic PTP SYNC message Trigger"]
        #[inline(always)]
        pub const fn asynctrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP SYNC message Trigger"]
        #[inline(always)]
        pub fn set_asynctrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Automatic PTP Pdelay_Req message Trigger"]
        #[inline(always)]
        pub const fn apdreqtrig(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP Pdelay_Req message Trigger"]
        #[inline(always)]
        pub fn set_apdreqtrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable PTO Delay Request/Response response generation"]
        #[inline(always)]
        pub const fn drrdis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable PTO Delay Request/Response response generation"]
        #[inline(always)]
        pub fn set_drrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Domain Number"]
        #[inline(always)]
        pub const fn dn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Domain Number"]
        #[inline(always)]
        pub fn set_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Macpocr {
        #[inline(always)]
        fn default() -> Macpocr {
            Macpocr(0)
        }
    }
    impl core::fmt::Debug for Macpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpocr")
                .field("ptoen", &self.ptoen())
                .field("asyncen", &self.asyncen())
                .field("apdreqen", &self.apdreqen())
                .field("asynctrig", &self.asynctrig())
                .field("apdreqtrig", &self.apdreqtrig())
                .field("drrdis", &self.drrdis())
                .field("dn", &self.dn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macpocr {{ ptoen: {=bool:?}, asyncen: {=bool:?}, apdreqen: {=bool:?}, asynctrig: {=bool:?}, apdreqtrig: {=bool:?}, drrdis: {=bool:?}, dn: {=u8:?} }}" , self . ptoen () , self . asyncen () , self . apdreqen () , self . asynctrig () , self . apdreqtrig () , self . drrdis () , self . dn ())
        }
    }
    #[doc = "PPS control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppscr(pub u32);
    impl Macppscr {
        #[doc = "Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
        #[inline(always)]
        pub const fn ppsctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
        #[inline(always)]
        pub fn set_ppsctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flexible PPS Output Mode Enable"]
        #[inline(always)]
        pub const fn ppsen0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible PPS Output Mode Enable"]
        #[inline(always)]
        pub fn set_ppsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Target Time Register Mode for PPS Output"]
        #[inline(always)]
        pub const fn trgtmodsel0(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS Output"]
        #[inline(always)]
        pub fn set_trgtmodsel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Macppscr {
        #[inline(always)]
        fn default() -> Macppscr {
            Macppscr(0)
        }
    }
    impl core::fmt::Debug for Macppscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppscr")
                .field("ppsctrl", &self.ppsctrl())
                .field("ppsen0", &self.ppsen0())
                .field("trgtmodsel0", &self.trgtmodsel0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macppscr {{ ppsctrl: {=u8:?}, ppsen0: {=bool:?}, trgtmodsel0: {=u8:?} }}",
                self.ppsctrl(),
                self.ppsen0(),
                self.trgtmodsel0()
            )
        }
    }
    #[doc = "PPS interval register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsir(pub u32);
    impl Macppsir {
        #[doc = "PPS Output Signal Interval"]
        #[inline(always)]
        pub const fn ppsint0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS Output Signal Interval"]
        #[inline(always)]
        pub fn set_ppsint0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macppsir {
        #[inline(always)]
        fn default() -> Macppsir {
            Macppsir(0)
        }
    }
    impl core::fmt::Debug for Macppsir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsir").field("ppsint0", &self.ppsint0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppsir {{ ppsint0: {=u32:?} }}", self.ppsint0())
        }
    }
    #[doc = "PPS target time nanoseconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsttnr(pub u32);
    impl Macppsttnr {
        #[doc = "Target Time Low for PPS Register"]
        #[inline(always)]
        pub const fn ttsl0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Target Time Low for PPS Register"]
        #[inline(always)]
        pub fn set_ttsl0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "PPS Target Time Register Busy"]
        #[inline(always)]
        pub const fn trgtbusy0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PPS Target Time Register Busy"]
        #[inline(always)]
        pub fn set_trgtbusy0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macppsttnr {
        #[inline(always)]
        fn default() -> Macppsttnr {
            Macppsttnr(0)
        }
    }
    impl core::fmt::Debug for Macppsttnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsttnr")
                .field("ttsl0", &self.ttsl0())
                .field("trgtbusy0", &self.trgtbusy0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsttnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macppsttnr {{ ttsl0: {=u32:?}, trgtbusy0: {=bool:?} }}",
                self.ttsl0(),
                self.trgtbusy0()
            )
        }
    }
    #[doc = "PPS target time seconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsttsr(pub u32);
    impl Macppsttsr {
        #[doc = "PPS Target Time Seconds Register"]
        #[inline(always)]
        pub const fn tstrh0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "PPS Target Time Seconds Register"]
        #[inline(always)]
        pub fn set_tstrh0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Macppsttsr {
        #[inline(always)]
        fn default() -> Macppsttsr {
            Macppsttsr(0)
        }
    }
    impl core::fmt::Debug for Macppsttsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsttsr").field("tstrh0", &self.tstrh0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsttsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppsttsr {{ tstrh0: {=u32:?} }}", self.tstrh0())
        }
    }
    #[doc = "PPS width register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppswr(pub u32);
    impl Macppswr {
        #[doc = "PPS Output Signal Width"]
        #[inline(always)]
        pub const fn ppswidth0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS Output Signal Width"]
        #[inline(always)]
        pub fn set_ppswidth0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macppswr {
        #[inline(always)]
        fn default() -> Macppswr {
            Macppswr(0)
        }
    }
    impl core::fmt::Debug for Macppswr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppswr")
                .field("ppswidth0", &self.ppswidth0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppswr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppswr {{ ppswidth0: {=u32:?} }}", self.ppswidth0())
        }
    }
    #[doc = "Tx Queue flow control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacqtxFcr(pub u32);
    impl MacqtxFcr {
        #[doc = "Flow Control Busy or Backpressure Activate"]
        #[inline(always)]
        pub const fn fcb_bpa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow Control Busy or Backpressure Activate"]
        #[inline(always)]
        pub fn set_fcb_bpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Flow Control Enable"]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Flow Control Enable"]
        #[inline(always)]
        pub fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Pause Low Threshold"]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Pause Low Threshold"]
        #[inline(always)]
        pub fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Disable Zero-Quanta Pause"]
        #[inline(always)]
        pub const fn dzpq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Zero-Quanta Pause"]
        #[inline(always)]
        pub fn set_dzpq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pause Time"]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause Time"]
        #[inline(always)]
        pub fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MacqtxFcr {
        #[inline(always)]
        fn default() -> MacqtxFcr {
            MacqtxFcr(0)
        }
    }
    impl core::fmt::Debug for MacqtxFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacqtxFcr")
                .field("fcb_bpa", &self.fcb_bpa())
                .field("tfe", &self.tfe())
                .field("plt", &self.plt())
                .field("dzpq", &self.dzpq())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacqtxFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacqtxFcr {{ fcb_bpa: {=bool:?}, tfe: {=bool:?}, plt: {=u8:?}, dzpq: {=bool:?}, pt: {=u16:?} }}",
                self.fcb_bpa(),
                self.tfe(),
                self.plt(),
                self.dzpq(),
                self.pt()
            )
        }
    }
    #[doc = "Remove wakeup packet filter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrwkpfr(pub u32);
    impl Macrwkpfr {
        #[doc = "Remote wakeup packet filter"]
        #[inline(always)]
        pub const fn macrwkpfr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Remote wakeup packet filter"]
        #[inline(always)]
        pub fn set_macrwkpfr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macrwkpfr {
        #[inline(always)]
        fn default() -> Macrwkpfr {
            Macrwkpfr(0)
        }
    }
    impl core::fmt::Debug for Macrwkpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrwkpfr")
                .field("macrwkpfr", &self.macrwkpfr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrwkpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macrwkpfr {{ macrwkpfr: {=u32:?} }}", self.macrwkpfr())
        }
    }
    #[doc = "Rx flow control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacrxFcr(pub u32);
    impl MacrxFcr {
        #[doc = "Receive Flow Control Enable"]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Flow Control Enable"]
        #[inline(always)]
        pub fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Unicast Pause Packet Detect"]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Pause Packet Detect"]
        #[inline(always)]
        pub fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MacrxFcr {
        #[inline(always)]
        fn default() -> MacrxFcr {
            MacrxFcr(0)
        }
    }
    impl core::fmt::Debug for MacrxFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacrxFcr")
                .field("rfe", &self.rfe())
                .field("up", &self.up())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacrxFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacrxFcr {{ rfe: {=bool:?}, up: {=bool:?} }}", self.rfe(), self.up())
        }
    }
    #[doc = "Rx Tx status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacrxTxSr(pub u32);
    impl MacrxTxSr {
        #[doc = "Transmit Jabber Timeout"]
        #[inline(always)]
        pub const fn tjt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Jabber Timeout"]
        #[inline(always)]
        pub fn set_tjt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No Carrier"]
        #[inline(always)]
        pub const fn ncarr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No Carrier"]
        #[inline(always)]
        pub fn set_ncarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loss of Carrier"]
        #[inline(always)]
        pub const fn lcarr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loss of Carrier"]
        #[inline(always)]
        pub fn set_lcarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Excessive Deferral"]
        #[inline(always)]
        pub const fn exdef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Excessive Deferral"]
        #[inline(always)]
        pub fn set_exdef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Late Collision"]
        #[inline(always)]
        pub const fn lcol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Late Collision"]
        #[inline(always)]
        pub fn set_lcol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Excessive Collisions"]
        #[inline(always)]
        pub const fn excol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Excessive Collisions"]
        #[inline(always)]
        pub fn set_excol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive Watchdog Timeout"]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout"]
        #[inline(always)]
        pub fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MacrxTxSr {
        #[inline(always)]
        fn default() -> MacrxTxSr {
            MacrxTxSr(0)
        }
    }
    impl core::fmt::Debug for MacrxTxSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacrxTxSr")
                .field("tjt", &self.tjt())
                .field("ncarr", &self.ncarr())
                .field("lcarr", &self.lcarr())
                .field("exdef", &self.exdef())
                .field("lcol", &self.lcol())
                .field("excol", &self.excol())
                .field("rwt", &self.rwt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacrxTxSr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MacrxTxSr {{ tjt: {=bool:?}, ncarr: {=bool:?}, lcarr: {=bool:?}, exdef: {=bool:?}, lcol: {=bool:?}, excol: {=bool:?}, rwt: {=bool:?} }}" , self . tjt () , self . ncarr () , self . lcarr () , self . exdef () , self . lcol () , self . excol () , self . rwt ())
        }
    }
    #[doc = "PTP Source Port Identity 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi0r(pub u32);
    impl Macspi0r {
        #[doc = "Source Port Identity 0"]
        #[inline(always)]
        pub const fn spi0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Port Identity 0"]
        #[inline(always)]
        pub fn set_spi0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macspi0r {
        #[inline(always)]
        fn default() -> Macspi0r {
            Macspi0r(0)
        }
    }
    impl core::fmt::Debug for Macspi0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi0r").field("spi0", &self.spi0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi0r {{ spi0: {=u32:?} }}", self.spi0())
        }
    }
    #[doc = "PTP Source port identity 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi1r(pub u32);
    impl Macspi1r {
        #[doc = "Source Port Identity 1"]
        #[inline(always)]
        pub const fn spi1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Port Identity 1"]
        #[inline(always)]
        pub fn set_spi1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macspi1r {
        #[inline(always)]
        fn default() -> Macspi1r {
            Macspi1r(0)
        }
    }
    impl core::fmt::Debug for Macspi1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi1r").field("spi1", &self.spi1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi1r {{ spi1: {=u32:?} }}", self.spi1())
        }
    }
    #[doc = "PTP Source port identity 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi2r(pub u32);
    impl Macspi2r {
        #[doc = "Source Port Identity 2"]
        #[inline(always)]
        pub const fn spi2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Source Port Identity 2"]
        #[inline(always)]
        pub fn set_spi2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macspi2r {
        #[inline(always)]
        fn default() -> Macspi2r {
            Macspi2r(0)
        }
    }
    impl core::fmt::Debug for Macspi2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi2r").field("spi2", &self.spi2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi2r {{ spi2: {=u16:?} }}", self.spi2())
        }
    }
    #[doc = "Sub-second increment register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macssir(pub u32);
    impl Macssir {
        #[doc = "Sub-nanosecond Increment Value"]
        #[inline(always)]
        pub const fn snsinc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Sub-nanosecond Increment Value"]
        #[inline(always)]
        pub fn set_snsinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Sub-second Increment Value"]
        #[inline(always)]
        pub const fn ssinc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Sub-second Increment Value"]
        #[inline(always)]
        pub fn set_ssinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Macssir {
        #[inline(always)]
        fn default() -> Macssir {
            Macssir(0)
        }
    }
    impl core::fmt::Debug for Macssir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macssir")
                .field("snsinc", &self.snsinc())
                .field("ssinc", &self.ssinc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macssir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macssir {{ snsinc: {=u8:?}, ssinc: {=u8:?} }}",
                self.snsinc(),
                self.ssinc()
            )
        }
    }
    #[doc = "System time nanoseconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstnr(pub u32);
    impl Macstnr {
        #[doc = "Timestamp Sub-seconds"]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp Sub-seconds"]
        #[inline(always)]
        pub fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Macstnr {
        #[inline(always)]
        fn default() -> Macstnr {
            Macstnr(0)
        }
    }
    impl core::fmt::Debug for Macstnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstnr").field("tsss", &self.tsss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstnr {{ tsss: {=u32:?} }}", self.tsss())
        }
    }
    #[doc = "System time nanoseconds update register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstnur(pub u32);
    impl Macstnur {
        #[doc = "Timestamp Sub-seconds"]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp Sub-seconds"]
        #[inline(always)]
        pub fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Add or Subtract Time"]
        #[inline(always)]
        pub const fn addsub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add or Subtract Time"]
        #[inline(always)]
        pub fn set_addsub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macstnur {
        #[inline(always)]
        fn default() -> Macstnur {
            Macstnur(0)
        }
    }
    impl core::fmt::Debug for Macstnur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstnur")
                .field("tsss", &self.tsss())
                .field("addsub", &self.addsub())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstnur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macstnur {{ tsss: {=u32:?}, addsub: {=bool:?} }}",
                self.tsss(),
                self.addsub()
            )
        }
    }
    #[doc = "System time seconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstsr(pub u32);
    impl Macstsr {
        #[doc = "Timestamp Second"]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Second"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macstsr {
        #[inline(always)]
        fn default() -> Macstsr {
            Macstsr(0)
        }
    }
    impl core::fmt::Debug for Macstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstsr").field("tss", &self.tss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstsr {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "System time seconds update register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstsur(pub u32);
    impl Macstsur {
        #[doc = "Timestamp Seconds"]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Seconds"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macstsur {
        #[inline(always)]
        fn default() -> Macstsur {
            Macstsur(0)
        }
    }
    impl core::fmt::Debug for Macstsur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstsur").field("tss", &self.tss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstsur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstsur {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "Timestamp addend register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsar(pub u32);
    impl Mactsar {
        #[doc = "Timestamp Addend Register"]
        #[inline(always)]
        pub const fn tsar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Addend Register"]
        #[inline(always)]
        pub fn set_tsar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsar {
        #[inline(always)]
        fn default() -> Mactsar {
            Mactsar(0)
        }
    }
    impl core::fmt::Debug for Mactsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsar").field("tsar", &self.tsar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsar {{ tsar: {=u32:?} }}", self.tsar())
        }
    }
    #[doc = "Timestamp control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactscr(pub u32);
    impl Mactscr {
        #[doc = "Enable Timestamp"]
        #[inline(always)]
        pub const fn tsena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp"]
        #[inline(always)]
        pub fn set_tsena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fine or Coarse Timestamp Update"]
        #[inline(always)]
        pub const fn tscfupdt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fine or Coarse Timestamp Update"]
        #[inline(always)]
        pub fn set_tscfupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Initialize Timestamp"]
        #[inline(always)]
        pub const fn tsinit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Initialize Timestamp"]
        #[inline(always)]
        pub fn set_tsinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Update Timestamp"]
        #[inline(always)]
        pub const fn tsupdt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Update Timestamp"]
        #[inline(always)]
        pub fn set_tsupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Update Addend Register"]
        #[inline(always)]
        pub const fn tsaddreg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Update Addend Register"]
        #[inline(always)]
        pub fn set_tsaddreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Timestamp for All Packets"]
        #[inline(always)]
        pub const fn tsenall(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp for All Packets"]
        #[inline(always)]
        pub fn set_tsenall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timestamp Digital or Binary Rollover Control"]
        #[inline(always)]
        pub const fn tsctrlssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Digital or Binary Rollover Control"]
        #[inline(always)]
        pub fn set_tsctrlssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable PTP Packet Processing for Version 2 Format"]
        #[inline(always)]
        pub const fn tsver2ena(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PTP Packet Processing for Version 2 Format"]
        #[inline(always)]
        pub fn set_tsver2ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Processing of PTP over Ethernet Packets"]
        #[inline(always)]
        pub const fn tsipena(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP over Ethernet Packets"]
        #[inline(always)]
        pub fn set_tsipena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP"]
        #[inline(always)]
        pub const fn tsipv6ena(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP"]
        #[inline(always)]
        pub fn set_tsipv6ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP"]
        #[inline(always)]
        pub const fn tsipv4ena(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP"]
        #[inline(always)]
        pub fn set_tsipv4ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages"]
        #[inline(always)]
        pub const fn tsevntena(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages"]
        #[inline(always)]
        pub fn set_tsevntena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master"]
        #[inline(always)]
        pub const fn tsmstrena(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master"]
        #[inline(always)]
        pub fn set_tsmstrena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Select PTP packets for Taking Snapshots"]
        #[inline(always)]
        pub const fn snaptypsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Select PTP packets for Taking Snapshots"]
        #[inline(always)]
        pub fn set_snaptypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Enable MAC Address for PTP Packet Filtering"]
        #[inline(always)]
        pub const fn tsenmacaddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable MAC Address for PTP Packet Filtering"]
        #[inline(always)]
        pub fn set_tsenmacaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable checksum correction during OST for PTP over UDP/IPv4 packets"]
        #[inline(always)]
        pub const fn csc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable checksum correction during OST for PTP over UDP/IPv4 packets"]
        #[inline(always)]
        pub fn set_csc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Transmit Timestamp Status Mode"]
        #[inline(always)]
        pub const fn txtsstsm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Timestamp Status Mode"]
        #[inline(always)]
        pub fn set_txtsstsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Mactscr {
        #[inline(always)]
        fn default() -> Mactscr {
            Mactscr(0)
        }
    }
    impl core::fmt::Debug for Mactscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactscr")
                .field("tsena", &self.tsena())
                .field("tscfupdt", &self.tscfupdt())
                .field("tsinit", &self.tsinit())
                .field("tsupdt", &self.tsupdt())
                .field("tsaddreg", &self.tsaddreg())
                .field("tsenall", &self.tsenall())
                .field("tsctrlssr", &self.tsctrlssr())
                .field("tsver2ena", &self.tsver2ena())
                .field("tsipena", &self.tsipena())
                .field("tsipv6ena", &self.tsipv6ena())
                .field("tsipv4ena", &self.tsipv4ena())
                .field("tsevntena", &self.tsevntena())
                .field("tsmstrena", &self.tsmstrena())
                .field("snaptypsel", &self.snaptypsel())
                .field("tsenmacaddr", &self.tsenmacaddr())
                .field("csc", &self.csc())
                .field("txtsstsm", &self.txtsstsm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mactscr {{ tsena: {=bool:?}, tscfupdt: {=bool:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tsaddreg: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {=bool:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, csc: {=bool:?}, txtsstsm: {=bool:?} }}" , self . tsena () , self . tscfupdt () , self . tsinit () , self . tsupdt () , self . tsaddreg () , self . tsenall () , self . tsctrlssr () , self . tsver2ena () , self . tsipena () , self . tsipv6ena () , self . tsipv4ena () , self . tsevntena () , self . tsmstrena () , self . snaptypsel () , self . tsenmacaddr () , self . csc () , self . txtsstsm ())
        }
    }
    #[doc = "Timestamp Egress asymmetric correction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactseacr(pub u32);
    impl Mactseacr {
        #[doc = "One-Step Timestamp Egress Asymmetry Correction"]
        #[inline(always)]
        pub const fn osteac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "One-Step Timestamp Egress Asymmetry Correction"]
        #[inline(always)]
        pub fn set_osteac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactseacr {
        #[inline(always)]
        fn default() -> Mactseacr {
            Mactseacr(0)
        }
    }
    impl core::fmt::Debug for Mactseacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactseacr").field("osteac", &self.osteac()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactseacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactseacr {{ osteac: {=u32:?} }}", self.osteac())
        }
    }
    #[doc = "Timestamp Egress correction nanosecond register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsecnr(pub u32);
    impl Mactsecnr {
        #[doc = "Timestamp Egress Correction"]
        #[inline(always)]
        pub const fn tsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Egress Correction"]
        #[inline(always)]
        pub fn set_tsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsecnr {
        #[inline(always)]
        fn default() -> Mactsecnr {
            Mactsecnr(0)
        }
    }
    impl core::fmt::Debug for Mactsecnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsecnr").field("tsec", &self.tsec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsecnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsecnr {{ tsec: {=u32:?} }}", self.tsec())
        }
    }
    #[doc = "Timestamp Ingress asymmetric correction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsiacr(pub u32);
    impl Mactsiacr {
        #[doc = "One-Step Timestamp Ingress Asymmetry Correction"]
        #[inline(always)]
        pub const fn ostiac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "One-Step Timestamp Ingress Asymmetry Correction"]
        #[inline(always)]
        pub fn set_ostiac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsiacr {
        #[inline(always)]
        fn default() -> Mactsiacr {
            Mactsiacr(0)
        }
    }
    impl core::fmt::Debug for Mactsiacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsiacr").field("ostiac", &self.ostiac()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsiacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsiacr {{ ostiac: {=u32:?} }}", self.ostiac())
        }
    }
    #[doc = "Timestamp Ingress correction nanosecond register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsicnr(pub u32);
    impl Mactsicnr {
        #[doc = "Timestamp Ingress Correction"]
        #[inline(always)]
        pub const fn tsic(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Ingress Correction"]
        #[inline(always)]
        pub fn set_tsic(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsicnr {
        #[inline(always)]
        fn default() -> Mactsicnr {
            Mactsicnr(0)
        }
    }
    impl core::fmt::Debug for Mactsicnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsicnr").field("tsic", &self.tsic()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsicnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsicnr {{ tsic: {=u32:?} }}", self.tsic())
        }
    }
    #[doc = "Timestamp status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactssr(pub u32);
    impl Mactssr {
        #[doc = "Timestamp Seconds Overflow"]
        #[inline(always)]
        pub const fn tssovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Seconds Overflow"]
        #[inline(always)]
        pub fn set_tssovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timestamp Target Time Reached"]
        #[inline(always)]
        pub const fn tstargt0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Reached"]
        #[inline(always)]
        pub fn set_tstargt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Auxiliary Timestamp Trigger Snapshot"]
        #[inline(always)]
        pub const fn auxtstrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Timestamp Trigger Snapshot"]
        #[inline(always)]
        pub fn set_auxtstrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp Target Time Error"]
        #[inline(always)]
        pub const fn tstrgterr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Error"]
        #[inline(always)]
        pub fn set_tstrgterr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tx Timestamp Status Interrupt Status"]
        #[inline(always)]
        pub const fn txtssis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Timestamp Status Interrupt Status"]
        #[inline(always)]
        pub fn set_txtssis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier"]
        #[inline(always)]
        pub const fn atsstn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier"]
        #[inline(always)]
        pub fn set_atsstn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed"]
        #[inline(always)]
        pub const fn atsstm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed"]
        #[inline(always)]
        pub fn set_atsstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots"]
        #[inline(always)]
        pub const fn atsns(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots"]
        #[inline(always)]
        pub fn set_atsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Mactssr {
        #[inline(always)]
        fn default() -> Mactssr {
            Mactssr(0)
        }
    }
    impl core::fmt::Debug for Mactssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactssr")
                .field("tssovf", &self.tssovf())
                .field("tstargt0", &self.tstargt0())
                .field("auxtstrig", &self.auxtstrig())
                .field("tstrgterr0", &self.tstrgterr0())
                .field("txtssis", &self.txtssis())
                .field("atsstn", &self.atsstn())
                .field("atsstm", &self.atsstm())
                .field("atsns", &self.atsns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactssr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mactssr {{ tssovf: {=bool:?}, tstargt0: {=bool:?}, auxtstrig: {=bool:?}, tstrgterr0: {=bool:?}, txtssis: {=bool:?}, atsstn: {=u8:?}, atsstm: {=bool:?}, atsns: {=u8:?} }}" , self . tssovf () , self . tstargt0 () , self . auxtstrig () , self . tstrgterr0 () , self . txtssis () , self . atsstn () , self . atsstm () , self . atsns ())
        }
    }
    #[doc = "Tx timestamp status nanoseconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MactxTssnr(pub u32);
    impl MactxTssnr {
        #[doc = "Transmit Timestamp Status Low"]
        #[inline(always)]
        pub const fn txtsslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Transmit Timestamp Status Low"]
        #[inline(always)]
        pub fn set_txtsslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Transmit Timestamp Status Missed"]
        #[inline(always)]
        pub const fn txtssmis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Timestamp Status Missed"]
        #[inline(always)]
        pub fn set_txtssmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MactxTssnr {
        #[inline(always)]
        fn default() -> MactxTssnr {
            MactxTssnr(0)
        }
    }
    impl core::fmt::Debug for MactxTssnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MactxTssnr")
                .field("txtsslo", &self.txtsslo())
                .field("txtssmis", &self.txtssmis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MactxTssnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MactxTssnr {{ txtsslo: {=u32:?}, txtssmis: {=bool:?} }}",
                self.txtsslo(),
                self.txtssmis()
            )
        }
    }
    #[doc = "Tx timestamp status seconds register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MactxTsssr(pub u32);
    impl MactxTsssr {
        #[doc = "Transmit Timestamp Status High"]
        #[inline(always)]
        pub const fn txtsshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit Timestamp Status High"]
        #[inline(always)]
        pub fn set_txtsshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MactxTsssr {
        #[inline(always)]
        fn default() -> MactxTsssr {
            MactxTsssr(0)
        }
    }
    impl core::fmt::Debug for MactxTsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MactxTsssr").field("txtsshi", &self.txtsshi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MactxTsssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MactxTsssr {{ txtsshi: {=u32:?} }}", self.txtsshi())
        }
    }
    #[doc = "VLAN Hash table register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvhtr(pub u32);
    impl Macvhtr {
        #[doc = "VLAN Hash Table"]
        #[inline(always)]
        pub const fn vlht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Hash Table"]
        #[inline(always)]
        pub fn set_vlht(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macvhtr {
        #[inline(always)]
        fn default() -> Macvhtr {
            Macvhtr(0)
        }
    }
    impl core::fmt::Debug for Macvhtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvhtr").field("vlht", &self.vlht()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvhtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macvhtr {{ vlht: {=u16:?} }}", self.vlht())
        }
    }
    #[doc = "VLAN inclusion register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvir(pub u32);
    impl Macvir {
        #[doc = "VLAN Tag for Transmit Packets"]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag for Transmit Packets"]
        #[inline(always)]
        pub fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Control in Transmit Packets"]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "VLAN Tag Control in Transmit Packets"]
        #[inline(always)]
        pub fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VLAN Priority Control"]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Priority Control"]
        #[inline(always)]
        pub fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "C-VLAN or S-VLAN"]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "C-VLAN or S-VLAN"]
        #[inline(always)]
        pub fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "VLAN Tag Input"]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Input"]
        #[inline(always)]
        pub fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Macvir {
        #[inline(always)]
        fn default() -> Macvir {
            Macvir(0)
        }
    }
    impl core::fmt::Debug for Macvir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvir")
                .field("vlt", &self.vlt())
                .field("vlc", &self.vlc())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvir {{ vlt: {=u16:?}, vlc: {=u8:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?} }}",
                self.vlt(),
                self.vlc(),
                self.vlp(),
                self.csvl(),
                self.vlti()
            )
        }
    }
    #[doc = "Version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvr(pub u32);
    impl Macvr {
        #[doc = "IP version"]
        #[inline(always)]
        pub const fn snpsver(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "IP version"]
        #[inline(always)]
        pub fn set_snpsver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "ST-defined version"]
        #[inline(always)]
        pub const fn userver(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "ST-defined version"]
        #[inline(always)]
        pub fn set_userver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Macvr {
        #[inline(always)]
        fn default() -> Macvr {
            Macvr(0)
        }
    }
    impl core::fmt::Debug for Macvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvr")
                .field("snpsver", &self.snpsver())
                .field("userver", &self.userver())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvr {{ snpsver: {=u8:?}, userver: {=u8:?} }}",
                self.snpsver(),
                self.userver()
            )
        }
    }
    #[doc = "VLAN tag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvtr(pub u32);
    impl Macvtr {
        #[doc = "VLAN Tag Identifier for Receive Packets"]
        #[inline(always)]
        pub const fn vl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag Identifier for Receive Packets"]
        #[inline(always)]
        pub fn set_vl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison"]
        #[inline(always)]
        pub const fn etv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison"]
        #[inline(always)]
        pub fn set_etv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VLAN Tag Inverse Match Enable"]
        #[inline(always)]
        pub const fn vtim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Inverse Match Enable"]
        #[inline(always)]
        pub fn set_vtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable S-VLAN"]
        #[inline(always)]
        pub const fn esvl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-VLAN"]
        #[inline(always)]
        pub fn set_esvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Receive S-VLAN Match"]
        #[inline(always)]
        pub const fn ersvlm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Receive S-VLAN Match"]
        #[inline(always)]
        pub fn set_ersvlm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Disable VLAN Type Check"]
        #[inline(always)]
        pub const fn dovltc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Disable VLAN Type Check"]
        #[inline(always)]
        pub fn set_dovltc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable VLAN Tag Stripping on Receive"]
        #[inline(always)]
        pub const fn evls(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Enable VLAN Tag Stripping on Receive"]
        #[inline(always)]
        pub fn set_evls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "Enable VLAN Tag in Rx status"]
        #[inline(always)]
        pub const fn evlrxs(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable VLAN Tag in Rx status"]
        #[inline(always)]
        pub fn set_evlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VLAN Tag Hash Table Match Enable"]
        #[inline(always)]
        pub const fn vthm(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Hash Table Match Enable"]
        #[inline(always)]
        pub fn set_vthm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Enable Double VLAN Processing"]
        #[inline(always)]
        pub const fn edvlp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Double VLAN Processing"]
        #[inline(always)]
        pub fn set_edvlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Enable Inner VLAN Tag"]
        #[inline(always)]
        pub const fn erivlt(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Inner VLAN Tag"]
        #[inline(always)]
        pub fn set_erivlt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Enable Inner VLAN Tag Stripping on Receive"]
        #[inline(always)]
        pub const fn eivls(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Enable Inner VLAN Tag Stripping on Receive"]
        #[inline(always)]
        pub fn set_eivls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Enable Inner VLAN Tag in Rx Status"]
        #[inline(always)]
        pub const fn eivlrxs(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Inner VLAN Tag in Rx Status"]
        #[inline(always)]
        pub fn set_eivlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macvtr {
        #[inline(always)]
        fn default() -> Macvtr {
            Macvtr(0)
        }
    }
    impl core::fmt::Debug for Macvtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvtr")
                .field("vl", &self.vl())
                .field("etv", &self.etv())
                .field("vtim", &self.vtim())
                .field("esvl", &self.esvl())
                .field("ersvlm", &self.ersvlm())
                .field("dovltc", &self.dovltc())
                .field("evls", &self.evls())
                .field("evlrxs", &self.evlrxs())
                .field("vthm", &self.vthm())
                .field("edvlp", &self.edvlp())
                .field("erivlt", &self.erivlt())
                .field("eivls", &self.eivls())
                .field("eivlrxs", &self.eivlrxs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvtr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macvtr {{ vl: {=u16:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, ersvlm: {=bool:?}, dovltc: {=bool:?}, evls: {=u8:?}, evlrxs: {=bool:?}, vthm: {=bool:?}, edvlp: {=bool:?}, erivlt: {=bool:?}, eivls: {=u8:?}, eivlrxs: {=bool:?} }}" , self . vl () , self . etv () , self . vtim () , self . esvl () , self . ersvlm () , self . dovltc () , self . evls () , self . evlrxs () , self . vthm () , self . edvlp () , self . erivlt () , self . eivls () , self . eivlrxs ())
        }
    }
    #[doc = "Watchdog timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macwtr(pub u32);
    impl Macwtr {
        #[doc = "Watchdog Timeout"]
        #[inline(always)]
        pub const fn wto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Watchdog Timeout"]
        #[inline(always)]
        pub fn set_wto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Programmable Watchdog Enable"]
        #[inline(always)]
        pub const fn pwe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable Watchdog Enable"]
        #[inline(always)]
        pub fn set_pwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Macwtr {
        #[inline(always)]
        fn default() -> Macwtr {
            Macwtr(0)
        }
    }
    impl core::fmt::Debug for Macwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macwtr")
                .field("wto", &self.wto())
                .field("pwe", &self.pwe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macwtr {{ wto: {=u8:?}, pwe: {=bool:?} }}", self.wto(), self.pwe())
        }
    }
    #[doc = "MMC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcControl(pub u32);
    impl MmcControl {
        #[doc = "Counters Reset"]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Reset"]
        #[inline(always)]
        pub fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter Stop Rollover"]
        #[inline(always)]
        pub const fn cntstopro(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter Stop Rollover"]
        #[inline(always)]
        pub fn set_cntstopro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset on Read"]
        #[inline(always)]
        pub const fn rstonrd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on Read"]
        #[inline(always)]
        pub fn set_rstonrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Counter Freeze"]
        #[inline(always)]
        pub const fn cntfreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Counter Freeze"]
        #[inline(always)]
        pub fn set_cntfreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Counters Preset"]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Preset"]
        #[inline(always)]
        pub fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Full-Half Preset"]
        #[inline(always)]
        pub const fn cntprstlvl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Full-Half Preset"]
        #[inline(always)]
        pub fn set_cntprstlvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Packets"]
        #[inline(always)]
        pub const fn ucdbc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Packets"]
        #[inline(always)]
        pub fn set_ucdbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MmcControl {
        #[inline(always)]
        fn default() -> MmcControl {
            MmcControl(0)
        }
    }
    impl core::fmt::Debug for MmcControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcControl")
                .field("cntrst", &self.cntrst())
                .field("cntstopro", &self.cntstopro())
                .field("rstonrd", &self.rstonrd())
                .field("cntfreez", &self.cntfreez())
                .field("cntprst", &self.cntprst())
                .field("cntprstlvl", &self.cntprstlvl())
                .field("ucdbc", &self.ucdbc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcControl {{ cntrst: {=bool:?}, cntstopro: {=bool:?}, rstonrd: {=bool:?}, cntfreez: {=bool:?}, cntprst: {=bool:?}, cntprstlvl: {=bool:?}, ucdbc: {=bool:?} }}" , self . cntrst () , self . cntstopro () , self . rstonrd () , self . cntfreez () , self . cntprst () , self . cntprstlvl () , self . ucdbc ())
        }
    }
    #[doc = "MMC Rx interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxInterrupt(pub u32);
    impl MmcRxInterrupt {
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn rxcrcerpis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_rxcrcerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn rxalgnerpis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_rxalgnerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn rxucgpis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_rxucgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt status"]
        #[inline(always)]
        pub const fn rxlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt status"]
        #[inline(always)]
        pub fn set_rxlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive LPI transition counter interrupt status"]
        #[inline(always)]
        pub const fn rxlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI transition counter interrupt status"]
        #[inline(always)]
        pub fn set_rxlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcRxInterrupt {
        #[inline(always)]
        fn default() -> MmcRxInterrupt {
            MmcRxInterrupt(0)
        }
    }
    impl core::fmt::Debug for MmcRxInterrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxInterrupt")
                .field("rxcrcerpis", &self.rxcrcerpis())
                .field("rxalgnerpis", &self.rxalgnerpis())
                .field("rxucgpis", &self.rxucgpis())
                .field("rxlpiuscis", &self.rxlpiuscis())
                .field("rxlpitrcis", &self.rxlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcRxInterrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcRxInterrupt {{ rxcrcerpis: {=bool:?}, rxalgnerpis: {=bool:?}, rxucgpis: {=bool:?}, rxlpiuscis: {=bool:?}, rxlpitrcis: {=bool:?} }}" , self . rxcrcerpis () , self . rxalgnerpis () , self . rxucgpis () , self . rxlpiuscis () , self . rxlpitrcis ())
        }
    }
    #[doc = "MMC Rx interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxInterruptMask(pub u32);
    impl MmcRxInterruptMask {
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn rxcrcerpim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_rxcrcerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn rxalgnerpim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_rxalgnerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn rxucgpim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_rxucgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt Mask"]
        #[inline(always)]
        pub const fn rxlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt Mask"]
        #[inline(always)]
        pub fn set_rxlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive LPI transition counter interrupt Mask"]
        #[inline(always)]
        pub const fn rxlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI transition counter interrupt Mask"]
        #[inline(always)]
        pub fn set_rxlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcRxInterruptMask {
        #[inline(always)]
        fn default() -> MmcRxInterruptMask {
            MmcRxInterruptMask(0)
        }
    }
    impl core::fmt::Debug for MmcRxInterruptMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxInterruptMask")
                .field("rxcrcerpim", &self.rxcrcerpim())
                .field("rxalgnerpim", &self.rxalgnerpim())
                .field("rxucgpim", &self.rxucgpim())
                .field("rxlpiuscim", &self.rxlpiuscim())
                .field("rxlpitrcim", &self.rxlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcRxInterruptMask {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcRxInterruptMask {{ rxcrcerpim: {=bool:?}, rxalgnerpim: {=bool:?}, rxucgpim: {=bool:?}, rxlpiuscim: {=bool:?}, rxlpitrcim: {=bool:?} }}" , self . rxcrcerpim () , self . rxalgnerpim () , self . rxucgpim () , self . rxlpiuscim () , self . rxlpitrcim ())
        }
    }
    #[doc = "MMC Tx interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxInterrupt(pub u32);
    impl MmcTxInterrupt {
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn txscolgpis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_txscolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn txmcolgpis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_txmcolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub const fn txgpktis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Status"]
        #[inline(always)]
        pub fn set_txgpktis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt status"]
        #[inline(always)]
        pub const fn txlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt status"]
        #[inline(always)]
        pub fn set_txlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Transmit LPI transition counter interrupt status"]
        #[inline(always)]
        pub const fn txlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI transition counter interrupt status"]
        #[inline(always)]
        pub fn set_txlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcTxInterrupt {
        #[inline(always)]
        fn default() -> MmcTxInterrupt {
            MmcTxInterrupt(0)
        }
    }
    impl core::fmt::Debug for MmcTxInterrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxInterrupt")
                .field("txscolgpis", &self.txscolgpis())
                .field("txmcolgpis", &self.txmcolgpis())
                .field("txgpktis", &self.txgpktis())
                .field("txlpiuscis", &self.txlpiuscis())
                .field("txlpitrcis", &self.txlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcTxInterrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcTxInterrupt {{ txscolgpis: {=bool:?}, txmcolgpis: {=bool:?}, txgpktis: {=bool:?}, txlpiuscis: {=bool:?}, txlpitrcis: {=bool:?} }}" , self . txscolgpis () , self . txmcolgpis () , self . txgpktis () , self . txlpiuscis () , self . txlpitrcis ())
        }
    }
    #[doc = "MMC Tx interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxInterruptMask(pub u32);
    impl MmcTxInterruptMask {
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn txscolgpim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_txscolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn txmcolgpim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_txmcolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub const fn txgpktim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Mask"]
        #[inline(always)]
        pub fn set_txgpktim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt Mask"]
        #[inline(always)]
        pub const fn txlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt Mask"]
        #[inline(always)]
        pub fn set_txlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Transmit LPI transition counter interrupt Mask"]
        #[inline(always)]
        pub const fn txlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI transition counter interrupt Mask"]
        #[inline(always)]
        pub fn set_txlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcTxInterruptMask {
        #[inline(always)]
        fn default() -> MmcTxInterruptMask {
            MmcTxInterruptMask(0)
        }
    }
    impl core::fmt::Debug for MmcTxInterruptMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxInterruptMask")
                .field("txscolgpim", &self.txscolgpim())
                .field("txmcolgpim", &self.txmcolgpim())
                .field("txgpktim", &self.txgpktim())
                .field("txlpiuscim", &self.txlpiuscim())
                .field("txlpitrcim", &self.txlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcTxInterruptMask {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcTxInterruptMask {{ txscolgpim: {=bool:?}, txmcolgpim: {=bool:?}, txgpktim: {=bool:?}, txlpiuscim: {=bool:?}, txlpitrcim: {=bool:?} }}" , self . txscolgpim () , self . txmcolgpim () , self . txgpktim () , self . txlpiuscim () , self . txlpitrcim ())
        }
    }
    #[doc = "Interrupt status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlisr(pub u32);
    impl Mtlisr {
        #[doc = "Queue interrupt status"]
        #[inline(always)]
        pub const fn q0is(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Queue interrupt status"]
        #[inline(always)]
        pub fn set_q0is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Mtlisr {
        #[inline(always)]
        fn default() -> Mtlisr {
            Mtlisr(0)
        }
    }
    impl core::fmt::Debug for Mtlisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlisr").field("q0is", &self.q0is()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mtlisr {{ q0is: {=bool:?} }}", self.q0is())
        }
    }
    #[doc = "Operating mode Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlomr(pub u32);
    impl Mtlomr {
        #[doc = "Drop Transmit Status"]
        #[inline(always)]
        pub const fn dtxsts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Transmit Status"]
        #[inline(always)]
        pub fn set_dtxsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counters Preset"]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Preset"]
        #[inline(always)]
        pub fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Counters Reset"]
        #[inline(always)]
        pub const fn cntclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Reset"]
        #[inline(always)]
        pub fn set_cntclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mtlomr {
        #[inline(always)]
        fn default() -> Mtlomr {
            Mtlomr(0)
        }
    }
    impl core::fmt::Debug for Mtlomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlomr")
                .field("dtxsts", &self.dtxsts())
                .field("cntprst", &self.cntprst())
                .field("cntclr", &self.cntclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlomr {{ dtxsts: {=bool:?}, cntprst: {=bool:?}, cntclr: {=bool:?} }}",
                self.dtxsts(),
                self.cntprst(),
                self.cntclr()
            )
        }
    }
    #[doc = "Queue interrupt control status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlqicsr(pub u32);
    impl Mtlqicsr {
        #[doc = "Transmit Queue Underflow Interrupt Status"]
        #[inline(always)]
        pub const fn txunfis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue Underflow Interrupt Status"]
        #[inline(always)]
        pub fn set_txunfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Queue Underflow Interrupt Enable"]
        #[inline(always)]
        pub const fn txuie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue Underflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_txuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Queue Overflow Interrupt Status"]
        #[inline(always)]
        pub const fn rxovfis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Overflow Interrupt Status"]
        #[inline(always)]
        pub fn set_rxovfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive Queue Overflow Interrupt Enable"]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Mtlqicsr {
        #[inline(always)]
        fn default() -> Mtlqicsr {
            Mtlqicsr(0)
        }
    }
    impl core::fmt::Debug for Mtlqicsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlqicsr")
                .field("txunfis", &self.txunfis())
                .field("txuie", &self.txuie())
                .field("rxovfis", &self.rxovfis())
                .field("rxoie", &self.rxoie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlqicsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlqicsr {{ txunfis: {=bool:?}, txuie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
                self.txunfis(),
                self.txuie(),
                self.rxovfis(),
                self.rxoie()
            )
        }
    }
    #[doc = "Rx queue debug register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlrxQdr(pub u32);
    impl MtlrxQdr {
        #[doc = "MTL Rx Queue Write Controller Active Status"]
        #[inline(always)]
        pub const fn rwcsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Rx Queue Write Controller Active Status"]
        #[inline(always)]
        pub fn set_rwcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MTL Rx Queue Read Controller State"]
        #[inline(always)]
        pub const fn rrcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Rx Queue Read Controller State"]
        #[inline(always)]
        pub fn set_rrcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MTL Rx Queue Fill-Level Status"]
        #[inline(always)]
        pub const fn rxqsts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Rx Queue Fill-Level Status"]
        #[inline(always)]
        pub fn set_rxqsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of Packets in Receive Queue"]
        #[inline(always)]
        pub const fn prxq(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Number of Packets in Receive Queue"]
        #[inline(always)]
        pub fn set_prxq(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for MtlrxQdr {
        #[inline(always)]
        fn default() -> MtlrxQdr {
            MtlrxQdr(0)
        }
    }
    impl core::fmt::Debug for MtlrxQdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlrxQdr")
                .field("rwcsts", &self.rwcsts())
                .field("rrcsts", &self.rrcsts())
                .field("rxqsts", &self.rxqsts())
                .field("prxq", &self.prxq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlrxQdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlrxQdr {{ rwcsts: {=bool:?}, rrcsts: {=u8:?}, rxqsts: {=u8:?}, prxq: {=u16:?} }}",
                self.rwcsts(),
                self.rrcsts(),
                self.rxqsts(),
                self.prxq()
            )
        }
    }
    #[doc = "Rx queue missed packet and overflow counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlrxQmpocr(pub u32);
    impl MtlrxQmpocr {
        #[doc = "Overflow Packet Counter"]
        #[inline(always)]
        pub const fn ovfpktcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Overflow Packet Counter"]
        #[inline(always)]
        pub fn set_ovfpktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow Counter Overflow Bit"]
        #[inline(always)]
        pub const fn ovfcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Counter Overflow Bit"]
        #[inline(always)]
        pub fn set_ovfcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Missed Packet Counter"]
        #[inline(always)]
        pub const fn mispktcnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Missed Packet Counter"]
        #[inline(always)]
        pub fn set_mispktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
        #[doc = "Missed Packet Counter Overflow Bit"]
        #[inline(always)]
        pub const fn miscntovf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Missed Packet Counter Overflow Bit"]
        #[inline(always)]
        pub fn set_miscntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MtlrxQmpocr {
        #[inline(always)]
        fn default() -> MtlrxQmpocr {
            MtlrxQmpocr(0)
        }
    }
    impl core::fmt::Debug for MtlrxQmpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlrxQmpocr")
                .field("ovfpktcnt", &self.ovfpktcnt())
                .field("ovfcntovf", &self.ovfcntovf())
                .field("mispktcnt", &self.mispktcnt())
                .field("miscntovf", &self.miscntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlrxQmpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MtlrxQmpocr {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}" , self . ovfpktcnt () , self . ovfcntovf () , self . mispktcnt () , self . miscntovf ())
        }
    }
    #[doc = "Rx queue operating mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlrxQomr(pub u32);
    impl MtlrxQomr {
        #[doc = "Receive Queue Threshold Control"]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Queue Threshold Control"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Forward Undersized Good Packets"]
        #[inline(always)]
        pub const fn fup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Undersized Good Packets"]
        #[inline(always)]
        pub fn set_fup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Forward Error Packets"]
        #[inline(always)]
        pub const fn fep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Error Packets"]
        #[inline(always)]
        pub fn set_fep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receive Queue Store and Forward"]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Store and Forward"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Packets"]
        #[inline(always)]
        pub const fn dis_tcp_ef(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Packets"]
        #[inline(always)]
        pub fn set_dis_tcp_ef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Hardware Flow Control"]
        #[inline(always)]
        pub const fn ehfc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Hardware Flow Control"]
        #[inline(always)]
        pub fn set_ehfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex modes)"]
        #[inline(always)]
        pub const fn rfa(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex modes)"]
        #[inline(always)]
        pub fn set_rfa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)"]
        #[inline(always)]
        pub const fn rfd(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)"]
        #[inline(always)]
        pub fn set_rfd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Receive Queue Size"]
        #[inline(always)]
        pub const fn rqs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Receive Queue Size"]
        #[inline(always)]
        pub fn set_rqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for MtlrxQomr {
        #[inline(always)]
        fn default() -> MtlrxQomr {
            MtlrxQomr(0)
        }
    }
    impl core::fmt::Debug for MtlrxQomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlrxQomr")
                .field("rtc", &self.rtc())
                .field("fup", &self.fup())
                .field("fep", &self.fep())
                .field("rsf", &self.rsf())
                .field("dis_tcp_ef", &self.dis_tcp_ef())
                .field("ehfc", &self.ehfc())
                .field("rfa", &self.rfa())
                .field("rfd", &self.rfd())
                .field("rqs", &self.rqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlrxQomr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MtlrxQomr {{ rtc: {=u8:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, dis_tcp_ef: {=bool:?}, ehfc: {=bool:?}, rfa: {=u8:?}, rfd: {=u8:?}, rqs: {=u8:?} }}" , self . rtc () , self . fup () , self . fep () , self . rsf () , self . dis_tcp_ef () , self . ehfc () , self . rfa () , self . rfd () , self . rqs ())
        }
    }
    #[doc = "Tx queue debug Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtltxQdr(pub u32);
    impl MtltxQdr {
        #[doc = "Transmit Queue in Pause"]
        #[inline(always)]
        pub const fn txqpaused(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue in Pause"]
        #[inline(always)]
        pub fn set_txqpaused(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MTL Tx Queue Read Controller Status"]
        #[inline(always)]
        pub const fn trcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Tx Queue Read Controller Status"]
        #[inline(always)]
        pub fn set_trcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MTL Tx Queue Write Controller Status"]
        #[inline(always)]
        pub const fn twcsts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Queue Write Controller Status"]
        #[inline(always)]
        pub fn set_twcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MTL Tx Queue Not Empty Status"]
        #[inline(always)]
        pub const fn txqsts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Queue Not Empty Status"]
        #[inline(always)]
        pub fn set_txqsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MTL Tx Status FIFO Full Status"]
        #[inline(always)]
        pub const fn txstsfsts(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Status FIFO Full Status"]
        #[inline(always)]
        pub fn set_txstsfsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Number of Packets in the Transmit Queue"]
        #[inline(always)]
        pub const fn ptxq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Packets in the Transmit Queue"]
        #[inline(always)]
        pub fn set_ptxq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Number of Status Words in Tx Status FIFO of Queue"]
        #[inline(always)]
        pub const fn stxstsf(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Status Words in Tx Status FIFO of Queue"]
        #[inline(always)]
        pub fn set_stxstsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for MtltxQdr {
        #[inline(always)]
        fn default() -> MtltxQdr {
            MtltxQdr(0)
        }
    }
    impl core::fmt::Debug for MtltxQdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtltxQdr")
                .field("txqpaused", &self.txqpaused())
                .field("trcsts", &self.trcsts())
                .field("twcsts", &self.twcsts())
                .field("txqsts", &self.txqsts())
                .field("txstsfsts", &self.txstsfsts())
                .field("ptxq", &self.ptxq())
                .field("stxstsf", &self.stxstsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtltxQdr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MtltxQdr {{ txqpaused: {=bool:?}, trcsts: {=u8:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}" , self . txqpaused () , self . trcsts () , self . twcsts () , self . txqsts () , self . txstsfsts () , self . ptxq () , self . stxstsf ())
        }
    }
    #[doc = "Tx queue operating mode Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtltxQomr(pub u32);
    impl MtltxQomr {
        #[doc = "Flush Transmit Queue"]
        #[inline(always)]
        pub const fn ftq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flush Transmit Queue"]
        #[inline(always)]
        pub fn set_ftq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Store and Forward"]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Store and Forward"]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Queue Enable"]
        #[inline(always)]
        pub const fn txqen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Transmit Queue Enable"]
        #[inline(always)]
        pub fn set_txqen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Transmit Threshold Control"]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Threshold Control"]
        #[inline(always)]
        pub fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Transmit Queue Size"]
        #[inline(always)]
        pub const fn tqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Queue Size"]
        #[inline(always)]
        pub fn set_tqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for MtltxQomr {
        #[inline(always)]
        fn default() -> MtltxQomr {
            MtltxQomr(0)
        }
    }
    impl core::fmt::Debug for MtltxQomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtltxQomr")
                .field("ftq", &self.ftq())
                .field("tsf", &self.tsf())
                .field("txqen", &self.txqen())
                .field("ttc", &self.ttc())
                .field("tqs", &self.tqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtltxQomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtltxQomr {{ ftq: {=bool:?}, tsf: {=bool:?}, txqen: {=u8:?}, ttc: {=u8:?}, tqs: {=u8:?} }}",
                self.ftq(),
                self.tsf(),
                self.txqen(),
                self.ttc(),
                self.tqs()
            )
        }
    }
    #[doc = "Tx queue underflow register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtltxQur(pub u32);
    impl MtltxQur {
        #[doc = "Underflow Packet Counter"]
        #[inline(always)]
        pub const fn uffrmcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Underflow Packet Counter"]
        #[inline(always)]
        pub fn set_uffrmcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow Bit for Underflow Packet Counter"]
        #[inline(always)]
        pub const fn ufcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Bit for Underflow Packet Counter"]
        #[inline(always)]
        pub fn set_ufcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for MtltxQur {
        #[inline(always)]
        fn default() -> MtltxQur {
            MtltxQur(0)
        }
    }
    impl core::fmt::Debug for MtltxQur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtltxQur")
                .field("uffrmcnt", &self.uffrmcnt())
                .field("ufcntovf", &self.ufcntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtltxQur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtltxQur {{ uffrmcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
                self.uffrmcnt(),
                self.ufcntovf()
            )
        }
    }
    #[doc = "Rx alignment error packets register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAlignmentErrorPackets(pub u32);
    impl RxAlignmentErrorPackets {
        #[doc = "Rx Alignment Error Packets"]
        #[inline(always)]
        pub const fn rxalgnerr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Alignment Error Packets"]
        #[inline(always)]
        pub fn set_rxalgnerr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxAlignmentErrorPackets {
        #[inline(always)]
        fn default() -> RxAlignmentErrorPackets {
            RxAlignmentErrorPackets(0)
        }
    }
    impl core::fmt::Debug for RxAlignmentErrorPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxAlignmentErrorPackets")
                .field("rxalgnerr", &self.rxalgnerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxAlignmentErrorPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxAlignmentErrorPackets {{ rxalgnerr: {=u32:?} }}", self.rxalgnerr())
        }
    }
    #[doc = "Rx CRC error packets register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxCrcErrorPackets(pub u32);
    impl RxCrcErrorPackets {
        #[doc = "Rx CRC Error Packets"]
        #[inline(always)]
        pub const fn rxcrcerr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx CRC Error Packets"]
        #[inline(always)]
        pub fn set_rxcrcerr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxCrcErrorPackets {
        #[inline(always)]
        fn default() -> RxCrcErrorPackets {
            RxCrcErrorPackets(0)
        }
    }
    impl core::fmt::Debug for RxCrcErrorPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxCrcErrorPackets")
                .field("rxcrcerr", &self.rxcrcerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxCrcErrorPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxCrcErrorPackets {{ rxcrcerr: {=u32:?} }}", self.rxcrcerr())
        }
    }
    #[doc = "Rx LPI transition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLpiTranCntr(pub u32);
    impl RxLpiTranCntr {
        #[doc = "Rx LPI Transition counter"]
        #[inline(always)]
        pub const fn rxlpitrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx LPI Transition counter"]
        #[inline(always)]
        pub fn set_rxlpitrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxLpiTranCntr {
        #[inline(always)]
        fn default() -> RxLpiTranCntr {
            RxLpiTranCntr(0)
        }
    }
    impl core::fmt::Debug for RxLpiTranCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLpiTranCntr")
                .field("rxlpitrc", &self.rxlpitrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxLpiTranCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxLpiTranCntr {{ rxlpitrc: {=u32:?} }}", self.rxlpitrc())
        }
    }
    #[doc = "Rx LPI microsecond counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLpiUsecCntr(pub u32);
    impl RxLpiUsecCntr {
        #[doc = "Rx LPI Microseconds Counter"]
        #[inline(always)]
        pub const fn rxlpiusc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx LPI Microseconds Counter"]
        #[inline(always)]
        pub fn set_rxlpiusc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxLpiUsecCntr {
        #[inline(always)]
        fn default() -> RxLpiUsecCntr {
            RxLpiUsecCntr(0)
        }
    }
    impl core::fmt::Debug for RxLpiUsecCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLpiUsecCntr")
                .field("rxlpiusc", &self.rxlpiusc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxLpiUsecCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxLpiUsecCntr {{ rxlpiusc: {=u32:?} }}", self.rxlpiusc())
        }
    }
    #[doc = "Rx unicast packets good register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxUnicastPacketsGood(pub u32);
    impl RxUnicastPacketsGood {
        #[doc = "Rx Unicast Packets Good"]
        #[inline(always)]
        pub const fn rxucastg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Unicast Packets Good"]
        #[inline(always)]
        pub fn set_rxucastg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxUnicastPacketsGood {
        #[inline(always)]
        fn default() -> RxUnicastPacketsGood {
            RxUnicastPacketsGood(0)
        }
    }
    impl core::fmt::Debug for RxUnicastPacketsGood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxUnicastPacketsGood")
                .field("rxucastg", &self.rxucastg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxUnicastPacketsGood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxUnicastPacketsGood {{ rxucastg: {=u32:?} }}", self.rxucastg())
        }
    }
    #[doc = "Tx LPI transition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxLpiTranCntr(pub u32);
    impl TxLpiTranCntr {
        #[doc = "Tx LPI Transition counter"]
        #[inline(always)]
        pub const fn txlpitrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx LPI Transition counter"]
        #[inline(always)]
        pub fn set_txlpitrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxLpiTranCntr {
        #[inline(always)]
        fn default() -> TxLpiTranCntr {
            TxLpiTranCntr(0)
        }
    }
    impl core::fmt::Debug for TxLpiTranCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxLpiTranCntr")
                .field("txlpitrc", &self.txlpitrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxLpiTranCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxLpiTranCntr {{ txlpitrc: {=u32:?} }}", self.txlpitrc())
        }
    }
    #[doc = "Tx LPI microsecond timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxLpiUsecCntr(pub u32);
    impl TxLpiUsecCntr {
        #[doc = "Tx LPI Microseconds Counter"]
        #[inline(always)]
        pub const fn txlpiusc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx LPI Microseconds Counter"]
        #[inline(always)]
        pub fn set_txlpiusc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxLpiUsecCntr {
        #[inline(always)]
        fn default() -> TxLpiUsecCntr {
            TxLpiUsecCntr(0)
        }
    }
    impl core::fmt::Debug for TxLpiUsecCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxLpiUsecCntr")
                .field("txlpiusc", &self.txlpiusc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxLpiUsecCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxLpiUsecCntr {{ txlpiusc: {=u32:?} }}", self.txlpiusc())
        }
    }
    #[doc = "Tx multiple collision good packets register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxMultipleCollisionGoodPackets(pub u32);
    impl TxMultipleCollisionGoodPackets {
        #[doc = "Tx Multiple Collision Good Packets"]
        #[inline(always)]
        pub const fn txmultcolg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Multiple Collision Good Packets"]
        #[inline(always)]
        pub fn set_txmultcolg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxMultipleCollisionGoodPackets {
        #[inline(always)]
        fn default() -> TxMultipleCollisionGoodPackets {
            TxMultipleCollisionGoodPackets(0)
        }
    }
    impl core::fmt::Debug for TxMultipleCollisionGoodPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxMultipleCollisionGoodPackets")
                .field("txmultcolg", &self.txmultcolg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxMultipleCollisionGoodPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxMultipleCollisionGoodPackets {{ txmultcolg: {=u32:?} }}",
                self.txmultcolg()
            )
        }
    }
    #[doc = "Tx packet count good register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxPacketCountGood(pub u32);
    impl TxPacketCountGood {
        #[doc = "Tx Packet Count Good"]
        #[inline(always)]
        pub const fn txpktg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Packet Count Good"]
        #[inline(always)]
        pub fn set_txpktg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxPacketCountGood {
        #[inline(always)]
        fn default() -> TxPacketCountGood {
            TxPacketCountGood(0)
        }
    }
    impl core::fmt::Debug for TxPacketCountGood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxPacketCountGood")
                .field("txpktg", &self.txpktg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxPacketCountGood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxPacketCountGood {{ txpktg: {=u32:?} }}", self.txpktg())
        }
    }
    #[doc = "Tx single collision good packets register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxSingleCollisionGoodPackets(pub u32);
    impl TxSingleCollisionGoodPackets {
        #[doc = "Tx Single Collision Good Packets"]
        #[inline(always)]
        pub const fn txsnglcolg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Single Collision Good Packets"]
        #[inline(always)]
        pub fn set_txsnglcolg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxSingleCollisionGoodPackets {
        #[inline(always)]
        fn default() -> TxSingleCollisionGoodPackets {
            TxSingleCollisionGoodPackets(0)
        }
    }
    impl core::fmt::Debug for TxSingleCollisionGoodPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxSingleCollisionGoodPackets")
                .field("txsnglcolg", &self.txsnglcolg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxSingleCollisionGoodPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxSingleCollisionGoodPackets {{ txsnglcolg: {=u32:?} }}",
                self.txsnglcolg()
            )
        }
    }
}
