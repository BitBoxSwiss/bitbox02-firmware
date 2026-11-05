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
    #[doc = "Ethernet: Precision Time Protocol (PTP)"]
    #[inline(always)]
    pub const fn ethernet_ptp(self) -> EthernetPtp {
        unsafe { EthernetPtp::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "Ethernet: DMA mode register (DMA)"]
    #[inline(always)]
    pub const fn ethernet_dma(self) -> EthernetDma {
        unsafe { EthernetDma::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
}
#[doc = "Ethernet: DMA controller operation"]
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
    #[doc = "Ethernet DMA bus mode register"]
    #[inline(always)]
    pub const fn dmabmr(self) -> crate::common::Reg<regs::Dmabmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet DMA transmit poll demand register"]
    #[inline(always)]
    pub const fn dmatpdr(self) -> crate::common::Reg<regs::Dmatpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "EHERNET DMA receive poll demand register"]
    #[inline(always)]
    pub const fn dmarpdr(self) -> crate::common::Reg<regs::Dmarpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet DMA receive descriptor list address register"]
    #[inline(always)]
    pub const fn dmardlar(self) -> crate::common::Reg<regs::Dmardlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet DMA transmit descriptor list address register"]
    #[inline(always)]
    pub const fn dmatdlar(self) -> crate::common::Reg<regs::Dmatdlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet DMA status register"]
    #[inline(always)]
    pub const fn dmasr(self) -> crate::common::Reg<regs::Dmasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ethernet DMA operation mode register"]
    #[inline(always)]
    pub const fn dmaomr(self) -> crate::common::Reg<regs::Dmaomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ethernet DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dmaier(self) -> crate::common::Reg<regs::Dmaier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
    #[inline(always)]
    pub const fn dmamfbocr(self) -> crate::common::Reg<regs::Dmamfbocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Ethernet DMA receive status watchdog timer register"]
    #[inline(always)]
    pub const fn dmarswtr(self) -> crate::common::Reg<regs::Dmarswtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit descriptor register"]
    #[inline(always)]
    pub const fn dmachtdr(self) -> crate::common::Reg<regs::Dmachtdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive descriptor register"]
    #[inline(always)]
    pub const fn dmachrdr(self) -> crate::common::Reg<regs::Dmachrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit buffer address register"]
    #[inline(always)]
    pub const fn dmachtbar(self) -> crate::common::Reg<regs::Dmachtbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive buffer address register"]
    #[inline(always)]
    pub const fn dmachrbar(self) -> crate::common::Reg<regs::Dmachrbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
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
    #[doc = "Ethernet MAC configuration register"]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet MAC frame filter register"]
    #[inline(always)]
    pub const fn macffr(self) -> crate::common::Reg<regs::Macffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn machthr(self) -> crate::common::Reg<regs::Machthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn machtlr(self) -> crate::common::Reg<regs::Machtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet MAC MII address register"]
    #[inline(always)]
    pub const fn macmiiar(self) -> crate::common::Reg<regs::Macmiiar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet MAC MII data register"]
    #[inline(always)]
    pub const fn macmiidr(self) -> crate::common::Reg<regs::Macmiidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ethernet MAC flow control register"]
    #[inline(always)]
    pub const fn macfcr(self) -> crate::common::Reg<regs::Macfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn macvlantr(self) -> crate::common::Reg<regs::Macvlantr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ethernet MAC remote wakeup frame filter register"]
    #[inline(always)]
    pub const fn macrwuffr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Ethernet MAC PMT control and status register"]
    #[inline(always)]
    pub const fn macpmtcsr(self) -> crate::common::Reg<regs::Macpmtcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Ethernet MAC debug register"]
    #[inline(always)]
    pub const fn macdbgr(self) -> crate::common::Reg<regs::Macdbgr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Ethernet MAC interrupt status register"]
    #[inline(always)]
    pub const fn macsr(self) -> crate::common::Reg<regs::Macsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Ethernet MAC interrupt mask register"]
    #[inline(always)]
    pub const fn macimr(self) -> crate::common::Reg<regs::Macimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Ethernet MAC address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Ethernet MAC address 1/2/3 high register"]
    #[inline(always)]
    pub const fn macahr(self, n: usize) -> crate::common::Reg<regs::Macahr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize + n * 8usize) as _) }
    }
    #[doc = "Ethernet MAC address 1/2/3 low register"]
    #[inline(always)]
    pub const fn macalr(self, n: usize) -> crate::common::Reg<regs::Macalr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize + n * 8usize) as _) }
    }
    #[doc = "Ethernet MMC control register"]
    #[inline(always)]
    pub const fn mmccr(self) -> crate::common::Reg<regs::Mmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Ethernet MMC receive interrupt register"]
    #[inline(always)]
    pub const fn mmcrir(self) -> crate::common::Reg<regs::Mmcrir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Ethernet MMC transmit interrupt register"]
    #[inline(always)]
    pub const fn mmctir(self) -> crate::common::Reg<regs::Mmctir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Ethernet MMC receive interrupt mask register"]
    #[inline(always)]
    pub const fn mmcrimr(self) -> crate::common::Reg<regs::Mmcrimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Ethernet MMC transmit interrupt mask register"]
    #[inline(always)]
    pub const fn mmctimr(self) -> crate::common::Reg<regs::Mmctimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub const fn mmctgfsccr(self) -> crate::common::Reg<regs::Mmctgfsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
    #[inline(always)]
    pub const fn mmctgfmsccr(self) -> crate::common::Reg<regs::Mmctgfmsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames counter register"]
    #[inline(always)]
    pub const fn mmctgfcr(self) -> crate::common::Reg<regs::Mmctgfcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "Ethernet MMC received frames with CRC error counter register"]
    #[inline(always)]
    pub const fn mmcrfcecr(self) -> crate::common::Reg<regs::Mmcrfcecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "Ethernet MMC received frames with alignment error counter register"]
    #[inline(always)]
    pub const fn mmcrfaecr(self) -> crate::common::Reg<regs::Mmcrfaecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "MMC received good unicast frames counter register"]
    #[inline(always)]
    pub const fn mmcrgufcr(self) -> crate::common::Reg<regs::Mmcrgufcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
}
#[doc = "Ethernet: Precision time protocol"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetPtp {
    ptr: *mut u8,
}
unsafe impl Send for EthernetPtp {}
unsafe impl Sync for EthernetPtp {}
impl EthernetPtp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet PTP time stamp control register"]
    #[inline(always)]
    pub const fn ptptscr(self) -> crate::common::Reg<regs::Ptptscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet PTP subsecond increment register"]
    #[inline(always)]
    pub const fn ptpssir(self) -> crate::common::Reg<regs::Ptpssir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp high register"]
    #[inline(always)]
    pub const fn ptptshr(self) -> crate::common::Reg<regs::Ptptshr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp low register"]
    #[inline(always)]
    pub const fn ptptslr(self) -> crate::common::Reg<regs::Ptptslr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet PTP time stamp high update register"]
    #[inline(always)]
    pub const fn ptptshur(self) -> crate::common::Reg<regs::Ptptshur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp low update register"]
    #[inline(always)]
    pub const fn ptptslur(self) -> crate::common::Reg<regs::Ptptslur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp addend register"]
    #[inline(always)]
    pub const fn ptptsar(self) -> crate::common::Reg<regs::Ptptsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ethernet PTP target time high register"]
    #[inline(always)]
    pub const fn ptptthr(self) -> crate::common::Reg<regs::Ptptthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ethernet PTP target time low register"]
    #[inline(always)]
    pub const fn ptpttlr(self) -> crate::common::Reg<regs::Ptpttlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp status register"]
    #[inline(always)]
    pub const fn ptptssr(self) -> crate::common::Reg<regs::Ptptssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Ethernet PTP PPS control register"]
    #[inline(always)]
    pub const fn ptpppscr(self) -> crate::common::Reg<regs::Ptpppscr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Ethernet DMA bus mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmabmr(pub u32);
    impl Dmabmr {
        #[doc = "Software reset"]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset"]
        #[inline(always)]
        pub fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA arbitration"]
        #[inline(always)]
        pub const fn da(&self) -> super::vals::Da {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Da::from_bits(val as u8)
        }
        #[doc = "DMA arbitration"]
        #[inline(always)]
        pub fn set_da(&mut self, val: super::vals::Da) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Descriptor skip length"]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Descriptor skip length"]
        #[inline(always)]
        pub fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "Enhanced descriptor format enable"]
        #[inline(always)]
        pub const fn edfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced descriptor format enable"]
        #[inline(always)]
        pub fn set_edfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Programmable burst length"]
        #[inline(always)]
        pub const fn pbl(&self) -> super::vals::Pbl {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Pbl::from_bits(val as u8)
        }
        #[doc = "Programmable burst length"]
        #[inline(always)]
        pub fn set_pbl(&mut self, val: super::vals::Pbl) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx-Tx priority ratio"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::PriorityRxOverTx {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::PriorityRxOverTx::from_bits(val as u8)
        }
        #[doc = "Rx-Tx priority ratio"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::PriorityRxOverTx) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Fixed burst"]
        #[inline(always)]
        pub const fn fb(&self) -> super::vals::Fb {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Fb::from_bits(val as u8)
        }
        #[doc = "Fixed burst"]
        #[inline(always)]
        pub fn set_fb(&mut self, val: super::vals::Fb) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Rx DMA PBL"]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 17usize) & 0x3f;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Rx DMA PBL"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val.to_bits() as u32) & 0x3f) << 17usize);
        }
        #[doc = "Use separate PBL"]
        #[inline(always)]
        pub const fn usp(&self) -> super::vals::Usp {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Usp::from_bits(val as u8)
        }
        #[doc = "Use separate PBL"]
        #[inline(always)]
        pub fn set_usp(&mut self, val: super::vals::Usp) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "4xPBL mode"]
        #[inline(always)]
        pub const fn fpm(&self) -> super::vals::Fpm {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Fpm::from_bits(val as u8)
        }
        #[doc = "4xPBL mode"]
        #[inline(always)]
        pub fn set_fpm(&mut self, val: super::vals::Fpm) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Address-aligned beats"]
        #[inline(always)]
        pub const fn aab(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Address-aligned beats"]
        #[inline(always)]
        pub fn set_aab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Mixed burst"]
        #[inline(always)]
        pub const fn mb(&self) -> super::vals::Mb {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Mb::from_bits(val as u8)
        }
        #[doc = "Mixed burst"]
        #[inline(always)]
        pub fn set_mb(&mut self, val: super::vals::Mb) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dmabmr {
        #[inline(always)]
        fn default() -> Dmabmr {
            Dmabmr(0)
        }
    }
    impl core::fmt::Debug for Dmabmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmabmr")
                .field("sr", &self.sr())
                .field("da", &self.da())
                .field("dsl", &self.dsl())
                .field("edfe", &self.edfe())
                .field("pbl", &self.pbl())
                .field("pm", &self.pm())
                .field("fb", &self.fb())
                .field("rdp", &self.rdp())
                .field("usp", &self.usp())
                .field("fpm", &self.fpm())
                .field("aab", &self.aab())
                .field("mb", &self.mb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmabmr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmabmr {{ sr: {=bool:?}, da: {:?}, dsl: {=u8:?}, edfe: {=bool:?}, pbl: {:?}, pm: {:?}, fb: {:?}, rdp: {:?}, usp: {:?}, fpm: {:?}, aab: {=bool:?}, mb: {:?} }}" , self . sr () , self . da () , self . dsl () , self . edfe () , self . pbl () , self . pm () , self . fb () , self . rdp () , self . usp () , self . fpm () , self . aab () , self . mb ())
        }
    }
    #[doc = "Ethernet DMA current host receive buffer address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrbar(pub u32);
    impl Dmachrbar {
        #[doc = "Host receive buffer address pointer"]
        #[inline(always)]
        pub const fn hrbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive buffer address pointer"]
        #[inline(always)]
        pub fn set_hrbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrbar {
        #[inline(always)]
        fn default() -> Dmachrbar {
            Dmachrbar(0)
        }
    }
    impl core::fmt::Debug for Dmachrbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrbar").field("hrbap", &self.hrbap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmachrbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmachrbar {{ hrbap: {=u32:?} }}", self.hrbap())
        }
    }
    #[doc = "Ethernet DMA current host receive descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrdr(pub u32);
    impl Dmachrdr {
        #[doc = "Host receive descriptor address pointer"]
        #[inline(always)]
        pub const fn hrdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive descriptor address pointer"]
        #[inline(always)]
        pub fn set_hrdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrdr {
        #[inline(always)]
        fn default() -> Dmachrdr {
            Dmachrdr(0)
        }
    }
    impl core::fmt::Debug for Dmachrdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrdr").field("hrdap", &self.hrdap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmachrdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmachrdr {{ hrdap: {=u32:?} }}", self.hrdap())
        }
    }
    #[doc = "Ethernet DMA current host transmit buffer address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtbar(pub u32);
    impl Dmachtbar {
        #[doc = "Host transmit buffer address pointer"]
        #[inline(always)]
        pub const fn htbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit buffer address pointer"]
        #[inline(always)]
        pub fn set_htbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtbar {
        #[inline(always)]
        fn default() -> Dmachtbar {
            Dmachtbar(0)
        }
    }
    impl core::fmt::Debug for Dmachtbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtbar").field("htbap", &self.htbap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmachtbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmachtbar {{ htbap: {=u32:?} }}", self.htbap())
        }
    }
    #[doc = "Ethernet DMA current host transmit descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtdr(pub u32);
    impl Dmachtdr {
        #[doc = "Host transmit descriptor address pointer"]
        #[inline(always)]
        pub const fn htdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit descriptor address pointer"]
        #[inline(always)]
        pub fn set_htdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtdr {
        #[inline(always)]
        fn default() -> Dmachtdr {
            Dmachtdr(0)
        }
    }
    impl core::fmt::Debug for Dmachtdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtdr").field("htdap", &self.htdap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmachtdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmachtdr {{ htdap: {=u32:?} }}", self.htdap())
        }
    }
    #[doc = "Ethernet DMA interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaier(pub u32);
    impl Dmaier {
        #[doc = "Transmit interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped interrupt enable"]
        #[inline(always)]
        pub const fn tpsie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped interrupt enable"]
        #[inline(always)]
        pub fn set_tpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable interrupt enable"]
        #[inline(always)]
        pub const fn tbuie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable interrupt enable"]
        #[inline(always)]
        pub fn set_tbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout interrupt enable"]
        #[inline(always)]
        pub const fn tjtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout interrupt enable"]
        #[inline(always)]
        pub fn set_tjtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow interrupt enable"]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow interrupt enable"]
        #[inline(always)]
        pub fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow interrupt enable"]
        #[inline(always)]
        pub const fn tuie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow interrupt enable"]
        #[inline(always)]
        pub fn set_tuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive interrupt enable"]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive interrupt enable"]
        #[inline(always)]
        pub fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable interrupt enable"]
        #[inline(always)]
        pub const fn rbuie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable interrupt enable"]
        #[inline(always)]
        pub fn set_rbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped interrupt enable"]
        #[inline(always)]
        pub const fn rpsie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped interrupt enable"]
        #[inline(always)]
        pub fn set_rpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout interrupt enable"]
        #[inline(always)]
        pub const fn rwtie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout interrupt enable"]
        #[inline(always)]
        pub fn set_rwtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit interrupt enable"]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit interrupt enable"]
        #[inline(always)]
        pub fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal bus error interrupt enable"]
        #[inline(always)]
        pub const fn fbeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal bus error interrupt enable"]
        #[inline(always)]
        pub fn set_fbeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early receive interrupt enable"]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive interrupt enable"]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary enable"]
        #[inline(always)]
        pub const fn aise(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary enable"]
        #[inline(always)]
        pub fn set_aise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary enable"]
        #[inline(always)]
        pub const fn nise(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary enable"]
        #[inline(always)]
        pub fn set_nise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Dmaier {
        #[inline(always)]
        fn default() -> Dmaier {
            Dmaier(0)
        }
    }
    impl core::fmt::Debug for Dmaier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaier")
                .field("tie", &self.tie())
                .field("tpsie", &self.tpsie())
                .field("tbuie", &self.tbuie())
                .field("tjtie", &self.tjtie())
                .field("roie", &self.roie())
                .field("tuie", &self.tuie())
                .field("rie", &self.rie())
                .field("rbuie", &self.rbuie())
                .field("rpsie", &self.rpsie())
                .field("rwtie", &self.rwtie())
                .field("etie", &self.etie())
                .field("fbeie", &self.fbeie())
                .field("erie", &self.erie())
                .field("aise", &self.aise())
                .field("nise", &self.nise())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmaier {{ tie: {=bool:?}, tpsie: {=bool:?}, tbuie: {=bool:?}, tjtie: {=bool:?}, roie: {=bool:?}, tuie: {=bool:?}, rie: {=bool:?}, rbuie: {=bool:?}, rpsie: {=bool:?}, rwtie: {=bool:?}, etie: {=bool:?}, fbeie: {=bool:?}, erie: {=bool:?}, aise: {=bool:?}, nise: {=bool:?} }}" , self . tie () , self . tpsie () , self . tbuie () , self . tjtie () , self . roie () , self . tuie () , self . rie () , self . rbuie () , self . rpsie () , self . rwtie () , self . etie () , self . fbeie () , self . erie () , self . aise () , self . nise ())
        }
    }
    #[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamfbocr(pub u32);
    impl Dmamfbocr {
        #[doc = "Missed frames by the controller"]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Missed frames by the controller"]
        #[inline(always)]
        pub fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Overflow bit for missed frame counter"]
        #[inline(always)]
        pub const fn omfc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for missed frame counter"]
        #[inline(always)]
        pub fn set_omfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Missed frames by the application"]
        #[inline(always)]
        pub const fn mfa(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "Missed frames by the application"]
        #[inline(always)]
        pub fn set_mfa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "Overflow bit for FIFO overflow counter"]
        #[inline(always)]
        pub const fn ofoc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for FIFO overflow counter"]
        #[inline(always)]
        pub fn set_ofoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Dmamfbocr {
        #[inline(always)]
        fn default() -> Dmamfbocr {
            Dmamfbocr(0)
        }
    }
    impl core::fmt::Debug for Dmamfbocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmamfbocr")
                .field("mfc", &self.mfc())
                .field("omfc", &self.omfc())
                .field("mfa", &self.mfa())
                .field("ofoc", &self.ofoc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmamfbocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmamfbocr {{ mfc: {=u16:?}, omfc: {=bool:?}, mfa: {=u16:?}, ofoc: {=bool:?} }}",
                self.mfc(),
                self.omfc(),
                self.mfa(),
                self.ofoc()
            )
        }
    }
    #[doc = "Ethernet DMA operation mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaomr(pub u32);
    impl Dmaomr {
        #[doc = "Start/stop receive"]
        #[inline(always)]
        pub const fn sr(&self) -> super::vals::DmaomrSr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::DmaomrSr::from_bits(val as u8)
        }
        #[doc = "Start/stop receive"]
        #[inline(always)]
        pub fn set_sr(&mut self, val: super::vals::DmaomrSr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Operate on second frame"]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Operate on second frame"]
        #[inline(always)]
        pub fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive threshold control"]
        #[inline(always)]
        pub const fn rtc(&self) -> super::vals::Rtc {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Rtc::from_bits(val as u8)
        }
        #[doc = "Receive threshold control"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: super::vals::Rtc) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Forward undersized good frames"]
        #[inline(always)]
        pub const fn fugf(&self) -> super::vals::Fugf {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Fugf::from_bits(val as u8)
        }
        #[doc = "Forward undersized good frames"]
        #[inline(always)]
        pub fn set_fugf(&mut self, val: super::vals::Fugf) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Forward error frames"]
        #[inline(always)]
        pub const fn fef(&self) -> super::vals::Fef {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Fef::from_bits(val as u8)
        }
        #[doc = "Forward error frames"]
        #[inline(always)]
        pub fn set_fef(&mut self, val: super::vals::Fef) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Start/stop transmission"]
        #[inline(always)]
        pub const fn st(&self) -> super::vals::St {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::St::from_bits(val as u8)
        }
        #[doc = "Start/stop transmission"]
        #[inline(always)]
        pub fn set_st(&mut self, val: super::vals::St) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit threshold control"]
        #[inline(always)]
        pub const fn ttc(&self) -> super::vals::Ttc {
            let val = (self.0 >> 14usize) & 0x07;
            super::vals::Ttc::from_bits(val as u8)
        }
        #[doc = "Transmit threshold control"]
        #[inline(always)]
        pub fn set_ttc(&mut self, val: super::vals::Ttc) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
        }
        #[doc = "Flush transmit FIFO"]
        #[inline(always)]
        pub const fn ftf(&self) -> super::vals::Ftf {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ftf::from_bits(val as u8)
        }
        #[doc = "Flush transmit FIFO"]
        #[inline(always)]
        pub fn set_ftf(&mut self, val: super::vals::Ftf) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit store and forward"]
        #[inline(always)]
        pub const fn tsf(&self) -> super::vals::Tsf {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Tsf::from_bits(val as u8)
        }
        #[doc = "Transmit store and forward"]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: super::vals::Tsf) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Disable flushing of received frames"]
        #[inline(always)]
        pub const fn dfrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable flushing of received frames"]
        #[inline(always)]
        pub fn set_dfrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Receive store and forward"]
        #[inline(always)]
        pub const fn rsf(&self) -> super::vals::Rsf {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Rsf::from_bits(val as u8)
        }
        #[doc = "Receive store and forward"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: super::vals::Rsf) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Dropping of TCP/IP checksum error frames disable"]
        #[inline(always)]
        pub const fn dtcefd(&self) -> super::vals::Dtcefd {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Dtcefd::from_bits(val as u8)
        }
        #[doc = "Dropping of TCP/IP checksum error frames disable"]
        #[inline(always)]
        pub fn set_dtcefd(&mut self, val: super::vals::Dtcefd) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dmaomr {
        #[inline(always)]
        fn default() -> Dmaomr {
            Dmaomr(0)
        }
    }
    impl core::fmt::Debug for Dmaomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaomr")
                .field("sr", &self.sr())
                .field("osf", &self.osf())
                .field("rtc", &self.rtc())
                .field("fugf", &self.fugf())
                .field("fef", &self.fef())
                .field("st", &self.st())
                .field("ttc", &self.ttc())
                .field("ftf", &self.ftf())
                .field("tsf", &self.tsf())
                .field("dfrf", &self.dfrf())
                .field("rsf", &self.rsf())
                .field("dtcefd", &self.dtcefd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaomr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmaomr {{ sr: {:?}, osf: {=bool:?}, rtc: {:?}, fugf: {:?}, fef: {:?}, st: {:?}, ttc: {:?}, ftf: {:?}, tsf: {:?}, dfrf: {=bool:?}, rsf: {:?}, dtcefd: {:?} }}" , self . sr () , self . osf () , self . rtc () , self . fugf () , self . fef () , self . st () , self . ttc () , self . ftf () , self . tsf () , self . dfrf () , self . rsf () , self . dtcefd ())
        }
    }
    #[doc = "Ethernet DMA receive descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmardlar(pub u32);
    impl Dmardlar {
        #[doc = "Start of receive list"]
        #[inline(always)]
        pub const fn srl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of receive list"]
        #[inline(always)]
        pub fn set_srl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmardlar {
        #[inline(always)]
        fn default() -> Dmardlar {
            Dmardlar(0)
        }
    }
    impl core::fmt::Debug for Dmardlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmardlar").field("srl", &self.srl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmardlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmardlar {{ srl: {=u32:?} }}", self.srl())
        }
    }
    #[doc = "EHERNET DMA receive poll demand register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmarpdr(pub u32);
    impl Dmarpdr {
        #[doc = "Receive poll demand"]
        #[inline(always)]
        pub const fn rpd(&self) -> super::vals::Rpd {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Rpd::from_bits(val as u32)
        }
        #[doc = "Receive poll demand"]
        #[inline(always)]
        pub fn set_rpd(&mut self, val: super::vals::Rpd) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmarpdr {
        #[inline(always)]
        fn default() -> Dmarpdr {
            Dmarpdr(0)
        }
    }
    impl core::fmt::Debug for Dmarpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmarpdr").field("rpd", &self.rpd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmarpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmarpdr {{ rpd: {:?} }}", self.rpd())
        }
    }
    #[doc = "Ethernet DMA receive status watchdog timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmarswtr(pub u32);
    impl Dmarswtr {
        #[doc = "Receive status watchdog timer count"]
        #[inline(always)]
        pub const fn rswtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive status watchdog timer count"]
        #[inline(always)]
        pub fn set_rswtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dmarswtr {
        #[inline(always)]
        fn default() -> Dmarswtr {
            Dmarswtr(0)
        }
    }
    impl core::fmt::Debug for Dmarswtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmarswtr").field("rswtc", &self.rswtc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmarswtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmarswtr {{ rswtc: {=u8:?} }}", self.rswtc())
        }
    }
    #[doc = "Ethernet DMA status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasr(pub u32);
    impl Dmasr {
        #[doc = "Transmit status"]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit status"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped status"]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped status"]
        #[inline(always)]
        pub fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable status"]
        #[inline(always)]
        pub const fn tbus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable status"]
        #[inline(always)]
        pub fn set_tbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout status"]
        #[inline(always)]
        pub const fn tjts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout status"]
        #[inline(always)]
        pub fn set_tjts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow status"]
        #[inline(always)]
        pub const fn ros(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow status"]
        #[inline(always)]
        pub fn set_ros(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow status"]
        #[inline(always)]
        pub const fn tus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow status"]
        #[inline(always)]
        pub fn set_tus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive status"]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive status"]
        #[inline(always)]
        pub fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable status"]
        #[inline(always)]
        pub const fn rbus(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable status"]
        #[inline(always)]
        pub fn set_rbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped status"]
        #[inline(always)]
        pub const fn rpss(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped status"]
        #[inline(always)]
        pub fn set_rpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PWTS"]
        #[inline(always)]
        pub const fn pwts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PWTS"]
        #[inline(always)]
        pub fn set_pwts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit status"]
        #[inline(always)]
        pub const fn ets(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit status"]
        #[inline(always)]
        pub fn set_ets(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal bus error status"]
        #[inline(always)]
        pub const fn fbes(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal bus error status"]
        #[inline(always)]
        pub fn set_fbes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early receive status"]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive status"]
        #[inline(always)]
        pub fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary"]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary"]
        #[inline(always)]
        pub fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary"]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary"]
        #[inline(always)]
        pub fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive process state"]
        #[inline(always)]
        pub const fn rps(&self) -> super::vals::Rps {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Rps::from_bits(val as u8)
        }
        #[doc = "Receive process state"]
        #[inline(always)]
        pub fn set_rps(&mut self, val: super::vals::Rps) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "Transmit process state"]
        #[inline(always)]
        pub const fn tps(&self) -> super::vals::Tps {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Tps::from_bits(val as u8)
        }
        #[doc = "Transmit process state"]
        #[inline(always)]
        pub fn set_tps(&mut self, val: super::vals::Tps) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "Error bits status"]
        #[inline(always)]
        pub const fn ebs(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "Error bits status"]
        #[inline(always)]
        pub fn set_ebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Dmasr {
        #[inline(always)]
        fn default() -> Dmasr {
            Dmasr(0)
        }
    }
    impl core::fmt::Debug for Dmasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmasr")
                .field("ts", &self.ts())
                .field("tpss", &self.tpss())
                .field("tbus", &self.tbus())
                .field("tjts", &self.tjts())
                .field("ros", &self.ros())
                .field("tus", &self.tus())
                .field("rs", &self.rs())
                .field("rbus", &self.rbus())
                .field("rpss", &self.rpss())
                .field("pwts", &self.pwts())
                .field("ets", &self.ets())
                .field("fbes", &self.fbes())
                .field("ers", &self.ers())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("rps", &self.rps())
                .field("tps", &self.tps())
                .field("ebs", &self.ebs())
                .field("mmcs", &self.mmcs())
                .field("pmts", &self.pmts())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmasr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmasr {{ ts: {=bool:?}, tpss: {=bool:?}, tbus: {=bool:?}, tjts: {=bool:?}, ros: {=bool:?}, tus: {=bool:?}, rs: {=bool:?}, rbus: {=bool:?}, rpss: {=bool:?}, pwts: {=bool:?}, ets: {=bool:?}, fbes: {=bool:?}, ers: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, rps: {:?}, tps: {:?}, ebs: {=u8:?}, mmcs: {=bool:?}, pmts: {=bool:?}, tsts: {=bool:?} }}" , self . ts () , self . tpss () , self . tbus () , self . tjts () , self . ros () , self . tus () , self . rs () , self . rbus () , self . rpss () , self . pwts () , self . ets () , self . fbes () , self . ers () , self . ais () , self . nis () , self . rps () , self . tps () , self . ebs () , self . mmcs () , self . pmts () , self . tsts ())
        }
    }
    #[doc = "Ethernet DMA transmit descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatdlar(pub u32);
    impl Dmatdlar {
        #[doc = "Start of transmit list"]
        #[inline(always)]
        pub const fn stl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of transmit list"]
        #[inline(always)]
        pub fn set_stl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatdlar {
        #[inline(always)]
        fn default() -> Dmatdlar {
            Dmatdlar(0)
        }
    }
    impl core::fmt::Debug for Dmatdlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatdlar").field("stl", &self.stl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmatdlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmatdlar {{ stl: {=u32:?} }}", self.stl())
        }
    }
    #[doc = "Ethernet DMA transmit poll demand register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatpdr(pub u32);
    impl Dmatpdr {
        #[doc = "Transmit poll demand"]
        #[inline(always)]
        pub const fn tpd(&self) -> super::vals::Tpd {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Tpd::from_bits(val as u32)
        }
        #[doc = "Transmit poll demand"]
        #[inline(always)]
        pub fn set_tpd(&mut self, val: super::vals::Tpd) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatpdr {
        #[inline(always)]
        fn default() -> Dmatpdr {
            Dmatpdr(0)
        }
    }
    impl core::fmt::Debug for Dmatpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatpdr").field("tpd", &self.tpd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmatpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmatpdr {{ tpd: {:?} }}", self.tpd())
        }
    }
    #[doc = "Ethernet MAC address 0 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "Ethernet MAC address 0 high"]
        #[inline(always)]
        pub const fn maca0h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Ethernet MAC address 0 high"]
        #[inline(always)]
        pub fn set_maca0h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Always 1"]
        #[inline(always)]
        pub const fn mo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Always 1"]
        #[inline(always)]
        pub fn set_mo(&mut self, val: bool) {
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
                .field("maca0h", &self.maca0h())
                .field("mo", &self.mo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maca0hr {{ maca0h: {=u16:?}, mo: {=bool:?} }}",
                self.maca0h(),
                self.mo()
            )
        }
    }
    #[doc = "Ethernet MAC address 0 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "Ethernet MAC address 0 low"]
        #[inline(always)]
        pub const fn maca0l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Ethernet MAC address 0 low"]
        #[inline(always)]
        pub fn set_maca0l(&mut self, val: u32) {
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
            f.debug_struct("Maca0lr").field("maca0l", &self.maca0l()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maca0lr {{ maca0l: {=u32:?} }}", self.maca0l())
        }
    }
    #[doc = "Ethernet MAC address 1/2/3 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macahr(pub u32);
    impl Macahr {
        #[doc = "Ethernet MAC address 1/2/3 high"]
        #[inline(always)]
        pub const fn macah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Ethernet MAC address 1/2/3 high"]
        #[inline(always)]
        pub fn set_macah(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "MBC"]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "MBC"]
        #[inline(always)]
        pub fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "SA"]
        #[inline(always)]
        pub const fn sa(&self) -> super::vals::MacahrSa {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::MacahrSa::from_bits(val as u8)
        }
        #[doc = "SA"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: super::vals::MacahrSa) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "AE"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AE"]
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
                .field("macah", &self.macah())
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
                "Macahr {{ macah: {=u16:?}, mbc: {=u8:?}, sa: {:?}, ae: {=bool:?} }}",
                self.macah(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[doc = "Ethernet MAC address 1/2/3 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macalr(pub u32);
    impl Macalr {
        #[doc = "Ethernet MAC address 1/2/3 low"]
        #[inline(always)]
        pub const fn macal(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Ethernet MAC address 1/2/3 low"]
        #[inline(always)]
        pub fn set_macal(&mut self, val: u32) {
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
            f.debug_struct("Macalr").field("macal", &self.macal()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macalr {{ macal: {=u32:?} }}", self.macal())
        }
    }
    #[doc = "Ethernet MAC configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Deferral check"]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Deferral check"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Back-off limit"]
        #[inline(always)]
        pub const fn bl(&self) -> super::vals::Bl {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Bl::from_bits(val as u8)
        }
        #[doc = "Back-off limit"]
        #[inline(always)]
        pub fn set_bl(&mut self, val: super::vals::Bl) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Automatic pad/CRC stripping"]
        #[inline(always)]
        pub const fn apcs(&self) -> super::vals::Apcs {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Apcs::from_bits(val as u8)
        }
        #[doc = "Automatic pad/CRC stripping"]
        #[inline(always)]
        pub fn set_apcs(&mut self, val: super::vals::Apcs) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Retry disable"]
        #[inline(always)]
        pub const fn rd(&self) -> super::vals::Rd {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Rd::from_bits(val as u8)
        }
        #[doc = "Retry disable"]
        #[inline(always)]
        pub fn set_rd(&mut self, val: super::vals::Rd) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "IPv4 checksum offload"]
        #[inline(always)]
        pub const fn ipco(&self) -> super::vals::Ipco {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ipco::from_bits(val as u8)
        }
        #[doc = "IPv4 checksum offload"]
        #[inline(always)]
        pub fn set_ipco(&mut self, val: super::vals::Ipco) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Duplex mode"]
        #[inline(always)]
        pub const fn dm(&self) -> super::vals::Dm {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Dm::from_bits(val as u8)
        }
        #[doc = "Duplex mode"]
        #[inline(always)]
        pub fn set_dm(&mut self, val: super::vals::Dm) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback mode"]
        #[inline(always)]
        pub const fn lm(&self) -> super::vals::Lm {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Lm::from_bits(val as u8)
        }
        #[doc = "Loopback mode"]
        #[inline(always)]
        pub fn set_lm(&mut self, val: super::vals::Lm) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Receive own disable"]
        #[inline(always)]
        pub const fn rod(&self) -> super::vals::Rod {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Rod::from_bits(val as u8)
        }
        #[doc = "Receive own disable"]
        #[inline(always)]
        pub fn set_rod(&mut self, val: super::vals::Rod) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Fast Ethernet speed"]
        #[inline(always)]
        pub const fn fes(&self) -> super::vals::Fes {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Fes::from_bits(val as u8)
        }
        #[doc = "Fast Ethernet speed"]
        #[inline(always)]
        pub fn set_fes(&mut self, val: super::vals::Fes) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Carrier sense disable"]
        #[inline(always)]
        pub const fn csd(&self) -> super::vals::Csd {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Csd::from_bits(val as u8)
        }
        #[doc = "Carrier sense disable"]
        #[inline(always)]
        pub fn set_csd(&mut self, val: super::vals::Csd) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Interframe gap"]
        #[inline(always)]
        pub const fn ifg(&self) -> super::vals::Ifg {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Ifg::from_bits(val as u8)
        }
        #[doc = "Interframe gap"]
        #[inline(always)]
        pub fn set_ifg(&mut self, val: super::vals::Ifg) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "Jabber disable"]
        #[inline(always)]
        pub const fn jd(&self) -> super::vals::Jd {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Jd::from_bits(val as u8)
        }
        #[doc = "Jabber disable"]
        #[inline(always)]
        pub fn set_jd(&mut self, val: super::vals::Jd) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Watchdog disable"]
        #[inline(always)]
        pub const fn wd(&self) -> super::vals::Wd {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Wd::from_bits(val as u8)
        }
        #[doc = "Watchdog disable"]
        #[inline(always)]
        pub fn set_wd(&mut self, val: super::vals::Wd) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "CRC stripping for type frames"]
        #[inline(always)]
        pub const fn cstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CRC stripping for type frames"]
        #[inline(always)]
        pub fn set_cstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("apcs", &self.apcs())
                .field("rd", &self.rd())
                .field("ipco", &self.ipco())
                .field("dm", &self.dm())
                .field("lm", &self.lm())
                .field("rod", &self.rod())
                .field("fes", &self.fes())
                .field("csd", &self.csd())
                .field("ifg", &self.ifg())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .field("cstf", &self.cstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maccr {{ re: {=bool:?}, te: {=bool:?}, dc: {=bool:?}, bl: {:?}, apcs: {:?}, rd: {:?}, ipco: {:?}, dm: {:?}, lm: {:?}, rod: {:?}, fes: {:?}, csd: {:?}, ifg: {:?}, jd: {:?}, wd: {:?}, cstf: {=bool:?} }}" , self . re () , self . te () , self . dc () , self . bl () , self . apcs () , self . rd () , self . ipco () , self . dm () , self . lm () , self . rod () , self . fes () , self . csd () , self . ifg () , self . jd () , self . wd () , self . cstf ())
        }
    }
    #[doc = "Ethernet MAC debug register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macdbgr(pub u32);
    impl Macdbgr {
        #[doc = "MAC MII receive protocol engine active"]
        #[inline(always)]
        pub const fn mmrpea(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MAC MII receive protocol engine active"]
        #[inline(always)]
        pub fn set_mmrpea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MAC small FIFO read/write controllers status"]
        #[inline(always)]
        pub const fn msfrwcs(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MAC small FIFO read/write controllers status"]
        #[inline(always)]
        pub fn set_msfrwcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Rx FIFO write controller active"]
        #[inline(always)]
        pub const fn rfwra(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO write controller active"]
        #[inline(always)]
        pub fn set_rfwra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO read controller status"]
        #[inline(always)]
        pub const fn rfrcs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Rx FIFO read controller status"]
        #[inline(always)]
        pub fn set_rfrcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Rx FIFO fill level"]
        #[inline(always)]
        pub const fn rffl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Rx FIFO fill level"]
        #[inline(always)]
        pub fn set_rffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "MAC MII transmit engine active"]
        #[inline(always)]
        pub const fn mmtea(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MAC MII transmit engine active"]
        #[inline(always)]
        pub fn set_mmtea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC transmit frame controller status"]
        #[inline(always)]
        pub const fn mtfcs(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "MAC transmit frame controller status"]
        #[inline(always)]
        pub fn set_mtfcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "MAC transmitter in pause"]
        #[inline(always)]
        pub const fn mtp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MAC transmitter in pause"]
        #[inline(always)]
        pub fn set_mtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Tx FIFO read status"]
        #[inline(always)]
        pub const fn tfrs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Tx FIFO read status"]
        #[inline(always)]
        pub fn set_tfrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Tx FIFO write active"]
        #[inline(always)]
        pub const fn tfwa(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO write active"]
        #[inline(always)]
        pub fn set_tfwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Tx FIFO not empty"]
        #[inline(always)]
        pub const fn tfne(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO not empty"]
        #[inline(always)]
        pub fn set_tfne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Tx FIFO full"]
        #[inline(always)]
        pub const fn tff(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO full"]
        #[inline(always)]
        pub fn set_tff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Macdbgr {
        #[inline(always)]
        fn default() -> Macdbgr {
            Macdbgr(0)
        }
    }
    impl core::fmt::Debug for Macdbgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macdbgr")
                .field("mmrpea", &self.mmrpea())
                .field("msfrwcs", &self.msfrwcs())
                .field("rfwra", &self.rfwra())
                .field("rfrcs", &self.rfrcs())
                .field("rffl", &self.rffl())
                .field("mmtea", &self.mmtea())
                .field("mtfcs", &self.mtfcs())
                .field("mtp", &self.mtp())
                .field("tfrs", &self.tfrs())
                .field("tfwa", &self.tfwa())
                .field("tfne", &self.tfne())
                .field("tff", &self.tff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macdbgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macdbgr {{ mmrpea: {=bool:?}, msfrwcs: {=u8:?}, rfwra: {=bool:?}, rfrcs: {=u8:?}, rffl: {=u8:?}, mmtea: {=bool:?}, mtfcs: {=u8:?}, mtp: {=bool:?}, tfrs: {=u8:?}, tfwa: {=bool:?}, tfne: {=bool:?}, tff: {=bool:?} }}" , self . mmrpea () , self . msfrwcs () , self . rfwra () , self . rfrcs () , self . rffl () , self . mmtea () , self . mtfcs () , self . mtp () , self . tfrs () , self . tfwa () , self . tfne () , self . tff ())
        }
    }
    #[doc = "Ethernet MAC flow control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macfcr(pub u32);
    impl Macfcr {
        #[doc = "Flow control busy/back pressure activate"]
        #[inline(always)]
        pub const fn fcb(&self) -> super::vals::Fcb {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Fcb::from_bits(val as u8)
        }
        #[doc = "Flow control busy/back pressure activate"]
        #[inline(always)]
        pub fn set_fcb(&mut self, val: super::vals::Fcb) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit flow control enable"]
        #[inline(always)]
        pub const fn tfce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit flow control enable"]
        #[inline(always)]
        pub fn set_tfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive flow control enable"]
        #[inline(always)]
        pub const fn rfce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive flow control enable"]
        #[inline(always)]
        pub fn set_rfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unicast pause frame detect"]
        #[inline(always)]
        pub const fn upfd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast pause frame detect"]
        #[inline(always)]
        pub fn set_upfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pause low threshold"]
        #[inline(always)]
        pub const fn plt(&self) -> super::vals::Plt {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Plt::from_bits(val as u8)
        }
        #[doc = "Pause low threshold"]
        #[inline(always)]
        pub fn set_plt(&mut self, val: super::vals::Plt) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Zero-quanta pause disable"]
        #[inline(always)]
        pub const fn zqpd(&self) -> super::vals::Zqpd {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Zqpd::from_bits(val as u8)
        }
        #[doc = "Zero-quanta pause disable"]
        #[inline(always)]
        pub fn set_zqpd(&mut self, val: super::vals::Zqpd) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Pause time"]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause time"]
        #[inline(always)]
        pub fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macfcr {
        #[inline(always)]
        fn default() -> Macfcr {
            Macfcr(0)
        }
    }
    impl core::fmt::Debug for Macfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macfcr")
                .field("fcb", &self.fcb())
                .field("tfce", &self.tfce())
                .field("rfce", &self.rfce())
                .field("upfd", &self.upfd())
                .field("plt", &self.plt())
                .field("zqpd", &self.zqpd())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macfcr {{ fcb: {:?}, tfce: {=bool:?}, rfce: {=bool:?}, upfd: {=bool:?}, plt: {:?}, zqpd: {:?}, pt: {=u16:?} }}" , self . fcb () , self . tfce () , self . rfce () , self . upfd () , self . plt () , self . zqpd () , self . pt ())
        }
    }
    #[doc = "Ethernet MAC frame filter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macffr(pub u32);
    impl Macffr {
        #[doc = "Promiscuous mode"]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous mode"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash unicast"]
        #[inline(always)]
        pub const fn hu(&self) -> super::vals::Hu {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hu::from_bits(val as u8)
        }
        #[doc = "Hash unicast"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: super::vals::Hu) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash multicast"]
        #[inline(always)]
        pub const fn hm(&self) -> super::vals::Hm {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Hm::from_bits(val as u8)
        }
        #[doc = "Hash multicast"]
        #[inline(always)]
        pub fn set_hm(&mut self, val: super::vals::Hm) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Destination address unique filtering"]
        #[inline(always)]
        pub const fn daif(&self) -> super::vals::Daif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Daif::from_bits(val as u8)
        }
        #[doc = "Destination address unique filtering"]
        #[inline(always)]
        pub fn set_daif(&mut self, val: super::vals::Daif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass all multicast"]
        #[inline(always)]
        pub const fn pam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass all multicast"]
        #[inline(always)]
        pub fn set_pam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Broadcast frames disable"]
        #[inline(always)]
        pub const fn bfd(&self) -> super::vals::Bfd {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Bfd::from_bits(val as u8)
        }
        #[doc = "Broadcast frames disable"]
        #[inline(always)]
        pub fn set_bfd(&mut self, val: super::vals::Bfd) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass control frames"]
        #[inline(always)]
        pub const fn pcf(&self) -> super::vals::Pcf {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Pcf::from_bits(val as u8)
        }
        #[doc = "Pass control frames"]
        #[inline(always)]
        pub fn set_pcf(&mut self, val: super::vals::Pcf) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Source address inverse filtering"]
        #[inline(always)]
        pub const fn saif(&self) -> super::vals::Saif {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Saif::from_bits(val as u8)
        }
        #[doc = "Source address inverse filtering"]
        #[inline(always)]
        pub fn set_saif(&mut self, val: super::vals::Saif) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Source address filter"]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Source address filter"]
        #[inline(always)]
        pub fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Hash or perfect filter"]
        #[inline(always)]
        pub const fn hpf(&self) -> super::vals::Hpf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Hpf::from_bits(val as u8)
        }
        #[doc = "Hash or perfect filter"]
        #[inline(always)]
        pub fn set_hpf(&mut self, val: super::vals::Hpf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive all"]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive all"]
        #[inline(always)]
        pub fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macffr {
        #[inline(always)]
        fn default() -> Macffr {
            Macffr(0)
        }
    }
    impl core::fmt::Debug for Macffr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macffr")
                .field("pm", &self.pm())
                .field("hu", &self.hu())
                .field("hm", &self.hm())
                .field("daif", &self.daif())
                .field("pam", &self.pam())
                .field("bfd", &self.bfd())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macffr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macffr {{ pm: {=bool:?}, hu: {:?}, hm: {:?}, daif: {:?}, pam: {=bool:?}, bfd: {:?}, pcf: {:?}, saif: {:?}, saf: {=bool:?}, hpf: {:?}, ra: {=bool:?} }}" , self . pm () , self . hu () , self . hm () , self . daif () , self . pam () , self . bfd () , self . pcf () , self . saif () , self . saf () , self . hpf () , self . ra ())
        }
    }
    #[doc = "Ethernet MAC hash table high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machthr(pub u32);
    impl Machthr {
        #[doc = "Upper 32 bits of hash table"]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Upper 32 bits of hash table"]
        #[inline(always)]
        pub fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machthr {
        #[inline(always)]
        fn default() -> Machthr {
            Machthr(0)
        }
    }
    impl core::fmt::Debug for Machthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machthr").field("hth", &self.hth()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machthr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Machthr {{ hth: {=u32:?} }}", self.hth())
        }
    }
    #[doc = "Ethernet MAC hash table low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtlr(pub u32);
    impl Machtlr {
        #[doc = "Lower 32 bits of hash table"]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Lower 32 bits of hash table"]
        #[inline(always)]
        pub fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtlr {
        #[inline(always)]
        fn default() -> Machtlr {
            Machtlr(0)
        }
    }
    impl core::fmt::Debug for Machtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machtlr").field("htl", &self.htl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machtlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Machtlr {{ htl: {=u32:?} }}", self.htl())
        }
    }
    #[doc = "Ethernet MAC interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macimr(pub u32);
    impl Macimr {
        #[doc = "PMT interrupt mask"]
        #[inline(always)]
        pub const fn pmtim(&self) -> super::vals::Pmtim {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Pmtim::from_bits(val as u8)
        }
        #[doc = "PMT interrupt mask"]
        #[inline(always)]
        pub fn set_pmtim(&mut self, val: super::vals::Pmtim) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Time stamp trigger interrupt mask"]
        #[inline(always)]
        pub const fn tstim(&self) -> super::vals::Tstim {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Tstim::from_bits(val as u8)
        }
        #[doc = "Time stamp trigger interrupt mask"]
        #[inline(always)]
        pub fn set_tstim(&mut self, val: super::vals::Tstim) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macimr {
        #[inline(always)]
        fn default() -> Macimr {
            Macimr(0)
        }
    }
    impl core::fmt::Debug for Macimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macimr")
                .field("pmtim", &self.pmtim())
                .field("tstim", &self.tstim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macimr {{ pmtim: {:?}, tstim: {:?} }}", self.pmtim(), self.tstim())
        }
    }
    #[doc = "Ethernet MAC MII address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiiar(pub u32);
    impl Macmiiar {
        #[doc = "MII busy"]
        #[inline(always)]
        pub const fn mb(&self) -> super::vals::MbProgress {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::MbProgress::from_bits(val as u8)
        }
        #[doc = "MII busy"]
        #[inline(always)]
        pub fn set_mb(&mut self, val: super::vals::MbProgress) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "MII write"]
        #[inline(always)]
        pub const fn mw(&self) -> super::vals::Mw {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Mw::from_bits(val as u8)
        }
        #[doc = "MII write"]
        #[inline(always)]
        pub fn set_mw(&mut self, val: super::vals::Mw) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock range"]
        #[inline(always)]
        pub const fn cr(&self) -> super::vals::Cr {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Cr::from_bits(val as u8)
        }
        #[doc = "Clock range"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: super::vals::Cr) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "MII register - select the desired MII register in the PHY device"]
        #[inline(always)]
        pub const fn mr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "MII register - select the desired MII register in the PHY device"]
        #[inline(always)]
        pub fn set_mr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PHY address - select which of possible 32 PHYs is being accessed"]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address - select which of possible 32 PHYs is being accessed"]
        #[inline(always)]
        pub fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for Macmiiar {
        #[inline(always)]
        fn default() -> Macmiiar {
            Macmiiar(0)
        }
    }
    impl core::fmt::Debug for Macmiiar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiiar")
                .field("mb", &self.mb())
                .field("mw", &self.mw())
                .field("cr", &self.cr())
                .field("mr", &self.mr())
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmiiar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macmiiar {{ mb: {:?}, mw: {:?}, cr: {:?}, mr: {=u8:?}, pa: {=u8:?} }}",
                self.mb(),
                self.mw(),
                self.cr(),
                self.mr(),
                self.pa()
            )
        }
    }
    #[doc = "Ethernet MAC MII data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiidr(pub u32);
    impl Macmiidr {
        #[doc = "MII data read from/written to the PHY"]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MII data read from/written to the PHY"]
        #[inline(always)]
        pub fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macmiidr {
        #[inline(always)]
        fn default() -> Macmiidr {
            Macmiidr(0)
        }
    }
    impl core::fmt::Debug for Macmiidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiidr").field("md", &self.md()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmiidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macmiidr {{ md: {=u16:?} }}", self.md())
        }
    }
    #[doc = "Ethernet MAC PMT control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpmtcsr(pub u32);
    impl Macpmtcsr {
        #[doc = "Power down"]
        #[inline(always)]
        pub const fn pd(&self) -> super::vals::Pd {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pd::from_bits(val as u8)
        }
        #[doc = "Power down"]
        #[inline(always)]
        pub fn set_pd(&mut self, val: super::vals::Pd) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic packet enable"]
        #[inline(always)]
        pub const fn mpe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet enable"]
        #[inline(always)]
        pub fn set_mpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup frame enable"]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame enable"]
        #[inline(always)]
        pub fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Magic packet received"]
        #[inline(always)]
        pub const fn mpr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet received"]
        #[inline(always)]
        pub fn set_mpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup frame received"]
        #[inline(always)]
        pub const fn wfr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame received"]
        #[inline(always)]
        pub fn set_wfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global unicast"]
        #[inline(always)]
        pub const fn gu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global unicast"]
        #[inline(always)]
        pub fn set_gu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup frame filter register pointer reset"]
        #[inline(always)]
        pub const fn wffrpr(&self) -> super::vals::Wffrpr {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Wffrpr::from_bits(val as u8)
        }
        #[doc = "Wakeup frame filter register pointer reset"]
        #[inline(always)]
        pub fn set_wffrpr(&mut self, val: super::vals::Wffrpr) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpmtcsr {
        #[inline(always)]
        fn default() -> Macpmtcsr {
            Macpmtcsr(0)
        }
    }
    impl core::fmt::Debug for Macpmtcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpmtcsr")
                .field("pd", &self.pd())
                .field("mpe", &self.mpe())
                .field("wfe", &self.wfe())
                .field("mpr", &self.mpr())
                .field("wfr", &self.wfr())
                .field("gu", &self.gu())
                .field("wffrpr", &self.wffrpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpmtcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macpmtcsr {{ pd: {:?}, mpe: {=bool:?}, wfe: {=bool:?}, mpr: {=bool:?}, wfr: {=bool:?}, gu: {=bool:?}, wffrpr: {:?} }}" , self . pd () , self . mpe () , self . wfe () , self . mpr () , self . wfr () , self . gu () , self . wffrpr ())
        }
    }
    #[doc = "Ethernet MAC interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macsr(pub u32);
    impl Macsr {
        #[doc = "PMT status"]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC receive status"]
        #[inline(always)]
        pub const fn mmcrs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC receive status"]
        #[inline(always)]
        pub fn set_mmcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC transmit status"]
        #[inline(always)]
        pub const fn mmcts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC transmit status"]
        #[inline(always)]
        pub fn set_mmcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macsr {
        #[inline(always)]
        fn default() -> Macsr {
            Macsr(0)
        }
    }
    impl core::fmt::Debug for Macsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macsr")
                .field("pmts", &self.pmts())
                .field("mmcs", &self.mmcs())
                .field("mmcrs", &self.mmcrs())
                .field("mmcts", &self.mmcts())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macsr {{ pmts: {=bool:?}, mmcs: {=bool:?}, mmcrs: {=bool:?}, mmcts: {=bool:?}, tsts: {=bool:?} }}",
                self.pmts(),
                self.mmcs(),
                self.mmcrs(),
                self.mmcts(),
                self.tsts()
            )
        }
    }
    #[doc = "Ethernet MAC VLAN tag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvlantr(pub u32);
    impl Macvlantr {
        #[doc = "VLAN tag identifier (for receive frames)"]
        #[inline(always)]
        pub const fn vlanti(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN tag identifier (for receive frames)"]
        #[inline(always)]
        pub fn set_vlanti(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "12-bit VLAN tag comparison"]
        #[inline(always)]
        pub const fn vlantc(&self) -> super::vals::Vlantc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vlantc::from_bits(val as u8)
        }
        #[doc = "12-bit VLAN tag comparison"]
        #[inline(always)]
        pub fn set_vlantc(&mut self, val: super::vals::Vlantc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Macvlantr {
        #[inline(always)]
        fn default() -> Macvlantr {
            Macvlantr(0)
        }
    }
    impl core::fmt::Debug for Macvlantr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvlantr")
                .field("vlanti", &self.vlanti())
                .field("vlantc", &self.vlantc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvlantr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvlantr {{ vlanti: {=u16:?}, vlantc: {:?} }}",
                self.vlanti(),
                self.vlantc()
            )
        }
    }
    #[doc = "Ethernet MMC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmccr(pub u32);
    impl Mmccr {
        #[doc = "Counter reset"]
        #[inline(always)]
        pub const fn cr(&self) -> super::vals::CounterReset {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::CounterReset::from_bits(val as u8)
        }
        #[doc = "Counter reset"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: super::vals::CounterReset) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter stop rollover"]
        #[inline(always)]
        pub const fn csr(&self) -> super::vals::Csr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Csr::from_bits(val as u8)
        }
        #[doc = "Counter stop rollover"]
        #[inline(always)]
        pub fn set_csr(&mut self, val: super::vals::Csr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset on read"]
        #[inline(always)]
        pub const fn ror(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on read"]
        #[inline(always)]
        pub fn set_ror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC counter freeze"]
        #[inline(always)]
        pub const fn mcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC counter freeze"]
        #[inline(always)]
        pub fn set_mcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC counter preset"]
        #[inline(always)]
        pub const fn mcp(&self) -> super::vals::Mcp {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Mcp::from_bits(val as u8)
        }
        #[doc = "MMC counter preset"]
        #[inline(always)]
        pub fn set_mcp(&mut self, val: super::vals::Mcp) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC counter Full-Half preset"]
        #[inline(always)]
        pub const fn mcfhp(&self) -> super::vals::Mcfhp {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Mcfhp::from_bits(val as u8)
        }
        #[doc = "MMC counter Full-Half preset"]
        #[inline(always)]
        pub fn set_mcfhp(&mut self, val: super::vals::Mcfhp) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Mmccr {
        #[inline(always)]
        fn default() -> Mmccr {
            Mmccr(0)
        }
    }
    impl core::fmt::Debug for Mmccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmccr")
                .field("cr", &self.cr())
                .field("csr", &self.csr())
                .field("ror", &self.ror())
                .field("mcf", &self.mcf())
                .field("mcp", &self.mcp())
                .field("mcfhp", &self.mcfhp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmccr {{ cr: {:?}, csr: {:?}, ror: {=bool:?}, mcf: {=bool:?}, mcp: {:?}, mcfhp: {:?} }}",
                self.cr(),
                self.csr(),
                self.ror(),
                self.mcf(),
                self.mcp(),
                self.mcfhp()
            )
        }
    }
    #[doc = "Ethernet MMC received frames with alignment error counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfaecr(pub u32);
    impl Mmcrfaecr {
        #[doc = "RFAEC"]
        #[inline(always)]
        pub const fn rfaec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RFAEC"]
        #[inline(always)]
        pub fn set_rfaec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfaecr {
        #[inline(always)]
        fn default() -> Mmcrfaecr {
            Mmcrfaecr(0)
        }
    }
    impl core::fmt::Debug for Mmcrfaecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrfaecr").field("rfaec", &self.rfaec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmcrfaecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmcrfaecr {{ rfaec: {=u32:?} }}", self.rfaec())
        }
    }
    #[doc = "Ethernet MMC received frames with CRC error counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfcecr(pub u32);
    impl Mmcrfcecr {
        #[doc = "RFCFC"]
        #[inline(always)]
        pub const fn rfcfc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RFCFC"]
        #[inline(always)]
        pub fn set_rfcfc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfcecr {
        #[inline(always)]
        fn default() -> Mmcrfcecr {
            Mmcrfcecr(0)
        }
    }
    impl core::fmt::Debug for Mmcrfcecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrfcecr").field("rfcfc", &self.rfcfc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmcrfcecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmcrfcecr {{ rfcfc: {=u32:?} }}", self.rfcfc())
        }
    }
    #[doc = "MMC received good unicast frames counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrgufcr(pub u32);
    impl Mmcrgufcr {
        #[doc = "RGUFC"]
        #[inline(always)]
        pub const fn rgufc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RGUFC"]
        #[inline(always)]
        pub fn set_rgufc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrgufcr {
        #[inline(always)]
        fn default() -> Mmcrgufcr {
            Mmcrgufcr(0)
        }
    }
    impl core::fmt::Debug for Mmcrgufcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrgufcr").field("rgufc", &self.rgufc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmcrgufcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmcrgufcr {{ rgufc: {=u32:?} }}", self.rgufc())
        }
    }
    #[doc = "Ethernet MMC receive interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrimr(pub u32);
    impl Mmcrimr {
        #[doc = "Received frame CRC error mask"]
        #[inline(always)]
        pub const fn rfcem(&self) -> super::vals::Rfcem {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Rfcem::from_bits(val as u8)
        }
        #[doc = "Received frame CRC error mask"]
        #[inline(always)]
        pub fn set_rfcem(&mut self, val: super::vals::Rfcem) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Received frames alignment error mask"]
        #[inline(always)]
        pub const fn rfaem(&self) -> super::vals::Rfaem {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Rfaem::from_bits(val as u8)
        }
        #[doc = "Received frames alignment error mask"]
        #[inline(always)]
        pub fn set_rfaem(&mut self, val: super::vals::Rfaem) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Received good Unicast frames mask"]
        #[inline(always)]
        pub const fn rgufm(&self) -> super::vals::Rgufm {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Rgufm::from_bits(val as u8)
        }
        #[doc = "Received good Unicast frames mask"]
        #[inline(always)]
        pub fn set_rgufm(&mut self, val: super::vals::Rgufm) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrimr {
        #[inline(always)]
        fn default() -> Mmcrimr {
            Mmcrimr(0)
        }
    }
    impl core::fmt::Debug for Mmcrimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrimr")
                .field("rfcem", &self.rfcem())
                .field("rfaem", &self.rfaem())
                .field("rgufm", &self.rgufm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmcrimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmcrimr {{ rfcem: {:?}, rfaem: {:?}, rgufm: {:?} }}",
                self.rfcem(),
                self.rfaem(),
                self.rgufm()
            )
        }
    }
    #[doc = "Ethernet MMC receive interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrir(pub u32);
    impl Mmcrir {
        #[doc = "Received frames CRC error status"]
        #[inline(always)]
        pub const fn rfces(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received frames CRC error status"]
        #[inline(always)]
        pub fn set_rfces(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Received frames alignment error status"]
        #[inline(always)]
        pub const fn rfaes(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Received frames alignment error status"]
        #[inline(always)]
        pub fn set_rfaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Received good Unicast frames status"]
        #[inline(always)]
        pub const fn rgufs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Received good Unicast frames status"]
        #[inline(always)]
        pub fn set_rgufs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrir {
        #[inline(always)]
        fn default() -> Mmcrir {
            Mmcrir(0)
        }
    }
    impl core::fmt::Debug for Mmcrir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrir")
                .field("rfces", &self.rfces())
                .field("rfaes", &self.rfaes())
                .field("rgufs", &self.rgufs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmcrir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmcrir {{ rfces: {=bool:?}, rfaes: {=bool:?}, rgufs: {=bool:?} }}",
                self.rfces(),
                self.rfaes(),
                self.rgufs()
            )
        }
    }
    #[doc = "Ethernet MMC transmitted good frames counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfcr(pub u32);
    impl Mmctgfcr {
        #[doc = "HTL"]
        #[inline(always)]
        pub const fn tgfc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HTL"]
        #[inline(always)]
        pub fn set_tgfc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfcr {
        #[inline(always)]
        fn default() -> Mmctgfcr {
            Mmctgfcr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfcr").field("tgfc", &self.tgfc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmctgfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmctgfcr {{ tgfc: {=u32:?} }}", self.tgfc())
        }
    }
    #[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfmsccr(pub u32);
    impl Mmctgfmsccr {
        #[doc = "TGFMSCC"]
        #[inline(always)]
        pub const fn tgfmscc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TGFMSCC"]
        #[inline(always)]
        pub fn set_tgfmscc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfmsccr {
        #[inline(always)]
        fn default() -> Mmctgfmsccr {
            Mmctgfmsccr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfmsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfmsccr").field("tgfmscc", &self.tgfmscc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmctgfmsccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmctgfmsccr {{ tgfmscc: {=u32:?} }}", self.tgfmscc())
        }
    }
    #[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfsccr(pub u32);
    impl Mmctgfsccr {
        #[doc = "Transmitted good frames single collision counter"]
        #[inline(always)]
        pub const fn tgfscc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted good frames single collision counter"]
        #[inline(always)]
        pub fn set_tgfscc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfsccr {
        #[inline(always)]
        fn default() -> Mmctgfsccr {
            Mmctgfsccr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfsccr").field("tgfscc", &self.tgfscc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmctgfsccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mmctgfsccr {{ tgfscc: {=u32:?} }}", self.tgfscc())
        }
    }
    #[doc = "Ethernet MMC transmit interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctimr(pub u32);
    impl Mmctimr {
        #[doc = "Transmitted good frames single collision mask"]
        #[inline(always)]
        pub const fn tgfscm(&self) -> super::vals::Tgfscm {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Tgfscm::from_bits(val as u8)
        }
        #[doc = "Transmitted good frames single collision mask"]
        #[inline(always)]
        pub fn set_tgfscm(&mut self, val: super::vals::Tgfscm) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitted good frames more than single collision mask"]
        #[inline(always)]
        pub const fn tgfmscm(&self) -> super::vals::Tgfmscm {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Tgfmscm::from_bits(val as u8)
        }
        #[doc = "Transmitted good frames more than single collision mask"]
        #[inline(always)]
        pub fn set_tgfmscm(&mut self, val: super::vals::Tgfmscm) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmitted good frames mask"]
        #[inline(always)]
        pub const fn tgfm(&self) -> super::vals::Tgfm {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Tgfm::from_bits(val as u8)
        }
        #[doc = "Transmitted good frames mask"]
        #[inline(always)]
        pub fn set_tgfm(&mut self, val: super::vals::Tgfm) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mmctimr {
        #[inline(always)]
        fn default() -> Mmctimr {
            Mmctimr(0)
        }
    }
    impl core::fmt::Debug for Mmctimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctimr")
                .field("tgfscm", &self.tgfscm())
                .field("tgfmscm", &self.tgfmscm())
                .field("tgfm", &self.tgfm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmctimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmctimr {{ tgfscm: {:?}, tgfmscm: {:?}, tgfm: {:?} }}",
                self.tgfscm(),
                self.tgfmscm(),
                self.tgfm()
            )
        }
    }
    #[doc = "Ethernet MMC transmit interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctir(pub u32);
    impl Mmctir {
        #[doc = "Transmitted good frames single collision status"]
        #[inline(always)]
        pub const fn tgfscs(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames single collision status"]
        #[inline(always)]
        pub fn set_tgfscs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitted good frames more than single collision status"]
        #[inline(always)]
        pub const fn tgfmscs(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames more than single collision status"]
        #[inline(always)]
        pub fn set_tgfmscs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmitted good frames status"]
        #[inline(always)]
        pub const fn tgfs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames status"]
        #[inline(always)]
        pub fn set_tgfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Mmctir {
        #[inline(always)]
        fn default() -> Mmctir {
            Mmctir(0)
        }
    }
    impl core::fmt::Debug for Mmctir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctir")
                .field("tgfscs", &self.tgfscs())
                .field("tgfmscs", &self.tgfmscs())
                .field("tgfs", &self.tgfs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmctir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmctir {{ tgfscs: {=bool:?}, tgfmscs: {=bool:?}, tgfs: {=bool:?} }}",
                self.tgfscs(),
                self.tgfmscs(),
                self.tgfs()
            )
        }
    }
    #[doc = "Ethernet PTP PPS control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpppscr(pub u32);
    impl Ptpppscr {
        #[doc = "TSSO"]
        #[inline(always)]
        pub const fn tsso(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TSSO"]
        #[inline(always)]
        pub fn set_tsso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TSTTR"]
        #[inline(always)]
        pub const fn tsttr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TSTTR"]
        #[inline(always)]
        pub fn set_tsttr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ptpppscr {
        #[inline(always)]
        fn default() -> Ptpppscr {
            Ptpppscr(0)
        }
    }
    impl core::fmt::Debug for Ptpppscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptpppscr")
                .field("tsso", &self.tsso())
                .field("tsttr", &self.tsttr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptpppscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptpppscr {{ tsso: {=bool:?}, tsttr: {=bool:?} }}",
                self.tsso(),
                self.tsttr()
            )
        }
    }
    #[doc = "Ethernet PTP subsecond increment register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpssir(pub u32);
    impl Ptpssir {
        #[doc = "STSSI"]
        #[inline(always)]
        pub const fn stssi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "STSSI"]
        #[inline(always)]
        pub fn set_stssi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ptpssir {
        #[inline(always)]
        fn default() -> Ptpssir {
            Ptpssir(0)
        }
    }
    impl core::fmt::Debug for Ptpssir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptpssir").field("stssi", &self.stssi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptpssir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptpssir {{ stssi: {=u8:?} }}", self.stssi())
        }
    }
    #[doc = "Ethernet PTP time stamp addend register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptsar(pub u32);
    impl Ptptsar {
        #[doc = "TSA"]
        #[inline(always)]
        pub const fn tsa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TSA"]
        #[inline(always)]
        pub fn set_tsa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptsar {
        #[inline(always)]
        fn default() -> Ptptsar {
            Ptptsar(0)
        }
    }
    impl core::fmt::Debug for Ptptsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptsar").field("tsa", &self.tsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptsar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptptsar {{ tsa: {=u32:?} }}", self.tsa())
        }
    }
    #[doc = "Ethernet PTP time stamp control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptscr(pub u32);
    impl Ptptscr {
        #[doc = "TSE"]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TSE"]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TSFCU"]
        #[inline(always)]
        pub const fn tsfcu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TSFCU"]
        #[inline(always)]
        pub fn set_tsfcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TSSTI"]
        #[inline(always)]
        pub const fn tssti(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TSSTI"]
        #[inline(always)]
        pub fn set_tssti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TSSTU"]
        #[inline(always)]
        pub const fn tsstu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TSSTU"]
        #[inline(always)]
        pub fn set_tsstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TSITE"]
        #[inline(always)]
        pub const fn tsite(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TSITE"]
        #[inline(always)]
        pub fn set_tsite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TTSARU"]
        #[inline(always)]
        pub const fn ttsaru(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TTSARU"]
        #[inline(always)]
        pub fn set_ttsaru(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TSSARFE"]
        #[inline(always)]
        pub const fn tssarfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TSSARFE"]
        #[inline(always)]
        pub fn set_tssarfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TSSSR"]
        #[inline(always)]
        pub const fn tsssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TSSSR"]
        #[inline(always)]
        pub fn set_tsssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TSPTPPSV2E"]
        #[inline(always)]
        pub const fn tsptppsv2e(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TSPTPPSV2E"]
        #[inline(always)]
        pub fn set_tsptppsv2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TSSPTPOEFE"]
        #[inline(always)]
        pub const fn tssptpoefe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TSSPTPOEFE"]
        #[inline(always)]
        pub fn set_tssptpoefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TSSIPV6FE"]
        #[inline(always)]
        pub const fn tssipv6fe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TSSIPV6FE"]
        #[inline(always)]
        pub fn set_tssipv6fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSSIPV4FE"]
        #[inline(always)]
        pub const fn tssipv4fe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TSSIPV4FE"]
        #[inline(always)]
        pub fn set_tssipv4fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TSSEME"]
        #[inline(always)]
        pub const fn tsseme(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TSSEME"]
        #[inline(always)]
        pub fn set_tsseme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TSSMRME"]
        #[inline(always)]
        pub const fn tssmrme(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TSSMRME"]
        #[inline(always)]
        pub fn set_tssmrme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TSCNT"]
        #[inline(always)]
        pub const fn tscnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "TSCNT"]
        #[inline(always)]
        pub fn set_tscnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "TSPFFMAE"]
        #[inline(always)]
        pub const fn tspffmae(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TSPFFMAE"]
        #[inline(always)]
        pub fn set_tspffmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ptptscr {
        #[inline(always)]
        fn default() -> Ptptscr {
            Ptptscr(0)
        }
    }
    impl core::fmt::Debug for Ptptscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptscr")
                .field("tse", &self.tse())
                .field("tsfcu", &self.tsfcu())
                .field("tssti", &self.tssti())
                .field("tsstu", &self.tsstu())
                .field("tsite", &self.tsite())
                .field("ttsaru", &self.ttsaru())
                .field("tssarfe", &self.tssarfe())
                .field("tsssr", &self.tsssr())
                .field("tsptppsv2e", &self.tsptppsv2e())
                .field("tssptpoefe", &self.tssptpoefe())
                .field("tssipv6fe", &self.tssipv6fe())
                .field("tssipv4fe", &self.tssipv4fe())
                .field("tsseme", &self.tsseme())
                .field("tssmrme", &self.tssmrme())
                .field("tscnt", &self.tscnt())
                .field("tspffmae", &self.tspffmae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ptptscr {{ tse: {=bool:?}, tsfcu: {=bool:?}, tssti: {=bool:?}, tsstu: {=bool:?}, tsite: {=bool:?}, ttsaru: {=bool:?}, tssarfe: {=bool:?}, tsssr: {=bool:?}, tsptppsv2e: {=bool:?}, tssptpoefe: {=bool:?}, tssipv6fe: {=bool:?}, tssipv4fe: {=bool:?}, tsseme: {=bool:?}, tssmrme: {=bool:?}, tscnt: {=u8:?}, tspffmae: {=bool:?} }}" , self . tse () , self . tsfcu () , self . tssti () , self . tsstu () , self . tsite () , self . ttsaru () , self . tssarfe () , self . tsssr () , self . tsptppsv2e () , self . tssptpoefe () , self . tssipv6fe () , self . tssipv4fe () , self . tsseme () , self . tssmrme () , self . tscnt () , self . tspffmae ())
        }
    }
    #[doc = "Ethernet PTP time stamp high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshr(pub u32);
    impl Ptptshr {
        #[doc = "STS"]
        #[inline(always)]
        pub const fn sts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "STS"]
        #[inline(always)]
        pub fn set_sts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshr {
        #[inline(always)]
        fn default() -> Ptptshr {
            Ptptshr(0)
        }
    }
    impl core::fmt::Debug for Ptptshr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptshr").field("sts", &self.sts()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptshr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptptshr {{ sts: {=u32:?} }}", self.sts())
        }
    }
    #[doc = "Ethernet PTP time stamp high update register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshur(pub u32);
    impl Ptptshur {
        #[doc = "TSUS"]
        #[inline(always)]
        pub const fn tsus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TSUS"]
        #[inline(always)]
        pub fn set_tsus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshur {
        #[inline(always)]
        fn default() -> Ptptshur {
            Ptptshur(0)
        }
    }
    impl core::fmt::Debug for Ptptshur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptshur").field("tsus", &self.tsus()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptshur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptptshur {{ tsus: {=u32:?} }}", self.tsus())
        }
    }
    #[doc = "Ethernet PTP time stamp low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslr(pub u32);
    impl Ptptslr {
        #[doc = "STSS"]
        #[inline(always)]
        pub const fn stss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "STSS"]
        #[inline(always)]
        pub fn set_stss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "STPNS"]
        #[inline(always)]
        pub const fn stpns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "STPNS"]
        #[inline(always)]
        pub fn set_stpns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ptptslr {
        #[inline(always)]
        fn default() -> Ptptslr {
            Ptptslr(0)
        }
    }
    impl core::fmt::Debug for Ptptslr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptslr")
                .field("stss", &self.stss())
                .field("stpns", &self.stpns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptslr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptptslr {{ stss: {=u32:?}, stpns: {=bool:?} }}",
                self.stss(),
                self.stpns()
            )
        }
    }
    #[doc = "Ethernet PTP time stamp low update register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslur(pub u32);
    impl Ptptslur {
        #[doc = "TSUSS"]
        #[inline(always)]
        pub const fn tsuss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "TSUSS"]
        #[inline(always)]
        pub fn set_tsuss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "TSUPNS"]
        #[inline(always)]
        pub const fn tsupns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TSUPNS"]
        #[inline(always)]
        pub fn set_tsupns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ptptslur {
        #[inline(always)]
        fn default() -> Ptptslur {
            Ptptslur(0)
        }
    }
    impl core::fmt::Debug for Ptptslur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptslur")
                .field("tsuss", &self.tsuss())
                .field("tsupns", &self.tsupns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptslur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptptslur {{ tsuss: {=u32:?}, tsupns: {=bool:?} }}",
                self.tsuss(),
                self.tsupns()
            )
        }
    }
    #[doc = "Ethernet PTP time stamp status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptssr(pub u32);
    impl Ptptssr {
        #[doc = "TSSO"]
        #[inline(always)]
        pub const fn tsso(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TSSO"]
        #[inline(always)]
        pub fn set_tsso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TSSO"]
        #[inline(always)]
        pub const fn tsttr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TSSO"]
        #[inline(always)]
        pub fn set_tsttr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ptptssr {
        #[inline(always)]
        fn default() -> Ptptssr {
            Ptptssr(0)
        }
    }
    impl core::fmt::Debug for Ptptssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptssr")
                .field("tsso", &self.tsso())
                .field("tsttr", &self.tsttr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptptssr {{ tsso: {=bool:?}, tsttr: {=bool:?} }}",
                self.tsso(),
                self.tsttr()
            )
        }
    }
    #[doc = "Ethernet PTP target time high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptthr(pub u32);
    impl Ptptthr {
        #[doc = "0"]
        #[inline(always)]
        pub const fn ttsh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "0"]
        #[inline(always)]
        pub fn set_ttsh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptthr {
        #[inline(always)]
        fn default() -> Ptptthr {
            Ptptthr(0)
        }
    }
    impl core::fmt::Debug for Ptptthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptthr").field("ttsh", &self.ttsh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptptthr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptptthr {{ ttsh: {=u32:?} }}", self.ttsh())
        }
    }
    #[doc = "Ethernet PTP target time low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpttlr(pub u32);
    impl Ptpttlr {
        #[doc = "TTSL"]
        #[inline(always)]
        pub const fn ttsl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TTSL"]
        #[inline(always)]
        pub fn set_ttsl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptpttlr {
        #[inline(always)]
        fn default() -> Ptpttlr {
            Ptpttlr(0)
        }
    }
    impl core::fmt::Debug for Ptpttlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptpttlr").field("ttsl", &self.ttsl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptpttlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptpttlr {{ ttsl: {=u32:?} }}", self.ttsl())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Apcs {
        #[doc = "MAC passes all incoming frames unmodified"]
        DISABLED = 0x0,
        #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
        STRIP = 0x01,
    }
    impl Apcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Apcs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Apcs {
        #[inline(always)]
        fn from(val: u8) -> Apcs {
            Apcs::from_bits(val)
        }
    }
    impl From<Apcs> for u8 {
        #[inline(always)]
        fn from(val: Apcs) -> u8 {
            Apcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bfd {
        #[doc = "Address filters pass all received broadcast frames"]
        ENABLED = 0x0,
        #[doc = "Address filters filter all incoming broadcast frames"]
        DISABLED = 0x01,
    }
    impl Bfd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bfd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bfd {
        #[inline(always)]
        fn from(val: u8) -> Bfd {
            Bfd::from_bits(val)
        }
    }
    impl From<Bfd> for u8 {
        #[inline(always)]
        fn from(val: Bfd) -> u8 {
            Bfd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bl {
        #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
        BL10 = 0x0,
        #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
        BL8 = 0x01,
        #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
        BL4 = 0x02,
        #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
        BL1 = 0x03,
    }
    impl Bl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bl {
        #[inline(always)]
        fn from(val: u8) -> Bl {
            Bl::from_bits(val)
        }
    }
    impl From<Bl> for u8 {
        #[inline(always)]
        fn from(val: Bl) -> u8 {
            Bl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CounterReset {
        _RESERVED_0 = 0x0,
        #[doc = "Reset all counters. Cleared automatically"]
        RESET = 0x01,
    }
    impl CounterReset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CounterReset {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CounterReset {
        #[inline(always)]
        fn from(val: u8) -> CounterReset {
            CounterReset::from_bits(val)
        }
    }
    impl From<CounterReset> for u8 {
        #[inline(always)]
        fn from(val: CounterReset) -> u8 {
            CounterReset::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cr {
        #[doc = "60-100MHz HCLK/42"]
        CR_60_100 = 0x0,
        #[doc = "100-150 MHz HCLK/62"]
        CR_100_150 = 0x01,
        #[doc = "20-35MHz HCLK/16"]
        CR_20_35 = 0x02,
        #[doc = "35-60MHz HCLK/16"]
        CR_35_60 = 0x03,
        #[doc = "150-168MHz HCLK/102"]
        CR_150_168 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cr {
        #[inline(always)]
        fn from(val: u8) -> Cr {
            Cr::from_bits(val)
        }
    }
    impl From<Cr> for u8 {
        #[inline(always)]
        fn from(val: Cr) -> u8 {
            Cr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Csd {
        #[doc = "Errors generated due to loss of carrier"]
        ENABLED = 0x0,
        #[doc = "No error generated due to loss of carrier"]
        DISABLED = 0x01,
    }
    impl Csd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Csd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Csd {
        #[inline(always)]
        fn from(val: u8) -> Csd {
            Csd::from_bits(val)
        }
    }
    impl From<Csd> for u8 {
        #[inline(always)]
        fn from(val: Csd) -> u8 {
            Csd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Csr {
        #[doc = "Counters roll over to zero after reaching the maximum value"]
        ROLLOVER = 0x0,
        #[doc = "Counters do not roll over to zero after reaching the maximum value"]
        NOT_ROLLOVER = 0x01,
    }
    impl Csr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Csr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Csr {
        #[inline(always)]
        fn from(val: u8) -> Csr {
            Csr::from_bits(val)
        }
    }
    impl From<Csr> for u8 {
        #[inline(always)]
        fn from(val: Csr) -> u8 {
            Csr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Da {
        #[doc = "Round-robin with Rx:Tx priority given by PM"]
        ROUND_ROBIN = 0x0,
        #[doc = "Rx has priority over Tx"]
        RX_PRIORITY = 0x01,
    }
    impl Da {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Da {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Da {
        #[inline(always)]
        fn from(val: u8) -> Da {
            Da::from_bits(val)
        }
    }
    impl From<Da> for u8 {
        #[inline(always)]
        fn from(val: Da) -> u8 {
            Da::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Daif {
        #[doc = "Normal filtering of frames"]
        NORMAL = 0x0,
        #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
        INVERT = 0x01,
    }
    impl Daif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Daif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Daif {
        #[inline(always)]
        fn from(val: u8) -> Daif {
            Daif::from_bits(val)
        }
    }
    impl From<Daif> for u8 {
        #[inline(always)]
        fn from(val: Daif) -> u8 {
            Daif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dm {
        #[doc = "MAC operates in half-duplex mode"]
        HALF_DUPLEX = 0x0,
        #[doc = "MAC operates in full-duplex mode"]
        FULL_DUPLEX = 0x01,
    }
    impl Dm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dm {
        #[inline(always)]
        fn from(val: u8) -> Dm {
            Dm::from_bits(val)
        }
    }
    impl From<Dm> for u8 {
        #[inline(always)]
        fn from(val: Dm) -> u8 {
            Dm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DmaomrSr {
        #[doc = "Reception is stopped after transfer of the current frame"]
        STOPPED = 0x0,
        #[doc = "Reception is placed in the Running state"]
        STARTED = 0x01,
    }
    impl DmaomrSr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DmaomrSr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DmaomrSr {
        #[inline(always)]
        fn from(val: u8) -> DmaomrSr {
            DmaomrSr::from_bits(val)
        }
    }
    impl From<DmaomrSr> for u8 {
        #[inline(always)]
        fn from(val: DmaomrSr) -> u8 {
            DmaomrSr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dtcefd {
        #[doc = "Drop frames with errors only in the receive checksum offload engine"]
        ENABLED = 0x0,
        #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
        DISABLED = 0x01,
    }
    impl Dtcefd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dtcefd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dtcefd {
        #[inline(always)]
        fn from(val: u8) -> Dtcefd {
            Dtcefd::from_bits(val)
        }
    }
    impl From<Dtcefd> for u8 {
        #[inline(always)]
        fn from(val: Dtcefd) -> u8 {
            Dtcefd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fb {
        #[doc = "AHB uses SINGLE and INCR burst transfers"]
        VARIABLE = 0x0,
        #[doc = "AHB uses only fixed burst transfers"]
        FIXED = 0x01,
    }
    impl Fb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fb {
        #[inline(always)]
        fn from(val: u8) -> Fb {
            Fb::from_bits(val)
        }
    }
    impl From<Fb> for u8 {
        #[inline(always)]
        fn from(val: Fb) -> u8 {
            Fb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fcb {
        #[doc = "In half duplex only, deasserts back pressure"]
        DISABLE_BACK_PRESSURE = 0x0,
        #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
        PAUSE_OR_BACK_PRESSURE = 0x01,
    }
    impl Fcb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fcb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fcb {
        #[inline(always)]
        fn from(val: u8) -> Fcb {
            Fcb::from_bits(val)
        }
    }
    impl From<Fcb> for u8 {
        #[inline(always)]
        fn from(val: Fcb) -> u8 {
            Fcb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fef {
        #[doc = "Rx FIFO drops frames with error status"]
        DROP = 0x0,
        #[doc = "All frames except runt error frames are forwarded to the DMA"]
        FORWARD = 0x01,
    }
    impl Fef {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fef {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fef {
        #[inline(always)]
        fn from(val: u8) -> Fef {
            Fef::from_bits(val)
        }
    }
    impl From<Fef> for u8 {
        #[inline(always)]
        fn from(val: Fef) -> u8 {
            Fef::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fes {
        #[doc = "10 Mbit/s"]
        FES10 = 0x0,
        #[doc = "100 Mbit/s"]
        FES100 = 0x01,
    }
    impl Fes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fes {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fes {
        #[inline(always)]
        fn from(val: u8) -> Fes {
            Fes::from_bits(val)
        }
    }
    impl From<Fes> for u8 {
        #[inline(always)]
        fn from(val: Fes) -> u8 {
            Fes::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fpm {
        #[doc = "PBL values used as-is"]
        X1 = 0x0,
        #[doc = "PBL values multiplied by 4"]
        X4 = 0x01,
    }
    impl Fpm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpm {
        #[inline(always)]
        fn from(val: u8) -> Fpm {
            Fpm::from_bits(val)
        }
    }
    impl From<Fpm> for u8 {
        #[inline(always)]
        fn from(val: Fpm) -> u8 {
            Fpm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ftf {
        _RESERVED_0 = 0x0,
        #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
        FLUSH = 0x01,
    }
    impl Ftf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ftf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ftf {
        #[inline(always)]
        fn from(val: u8) -> Ftf {
            Ftf::from_bits(val)
        }
    }
    impl From<Ftf> for u8 {
        #[inline(always)]
        fn from(val: Ftf) -> u8 {
            Ftf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fugf {
        #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
        DROP = 0x0,
        #[doc = "Rx FIFO forwards undersized frames"]
        FORWARD = 0x01,
    }
    impl Fugf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fugf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fugf {
        #[inline(always)]
        fn from(val: u8) -> Fugf {
            Fugf::from_bits(val)
        }
    }
    impl From<Fugf> for u8 {
        #[inline(always)]
        fn from(val: Fugf) -> u8 {
            Fugf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hm {
        #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
        PERFECT = 0x0,
        #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
        HASH = 0x01,
    }
    impl Hm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hm {
        #[inline(always)]
        fn from(val: u8) -> Hm {
            Hm::from_bits(val)
        }
    }
    impl From<Hm> for u8 {
        #[inline(always)]
        fn from(val: Hm) -> u8 {
            Hm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpf {
        #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
        HASH_ONLY = 0x0,
        #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
        HASH_OR_PERFECT = 0x01,
    }
    impl Hpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpf {
        #[inline(always)]
        fn from(val: u8) -> Hpf {
            Hpf::from_bits(val)
        }
    }
    impl From<Hpf> for u8 {
        #[inline(always)]
        fn from(val: Hpf) -> u8 {
            Hpf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hu {
        #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
        PERFECT = 0x0,
        #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
        HASH = 0x01,
    }
    impl Hu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hu {
        #[inline(always)]
        fn from(val: u8) -> Hu {
            Hu::from_bits(val)
        }
    }
    impl From<Hu> for u8 {
        #[inline(always)]
        fn from(val: Hu) -> u8 {
            Hu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ifg {
        #[doc = "96 bit times"]
        IFG96 = 0x0,
        #[doc = "88 bit times"]
        IFG88 = 0x01,
        #[doc = "80 bit times"]
        IFG80 = 0x02,
        #[doc = "72 bit times"]
        IFG72 = 0x03,
        #[doc = "64 bit times"]
        IFG64 = 0x04,
        #[doc = "56 bit times"]
        IFG56 = 0x05,
        #[doc = "48 bit times"]
        IFG48 = 0x06,
        #[doc = "40 bit times"]
        IFG40 = 0x07,
    }
    impl Ifg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ifg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ifg {
        #[inline(always)]
        fn from(val: u8) -> Ifg {
            Ifg::from_bits(val)
        }
    }
    impl From<Ifg> for u8 {
        #[inline(always)]
        fn from(val: Ifg) -> u8 {
            Ifg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ipco {
        #[doc = "IPv4 checksum offload disabled"]
        DISABLED = 0x0,
        #[doc = "IPv4 checksums are checked in received frames"]
        OFFLOAD = 0x01,
    }
    impl Ipco {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ipco {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ipco {
        #[inline(always)]
        fn from(val: u8) -> Ipco {
            Ipco::from_bits(val)
        }
    }
    impl From<Ipco> for u8 {
        #[inline(always)]
        fn from(val: Ipco) -> u8 {
            Ipco::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Jd {
        #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
        ENABLED = 0x0,
        #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
        DISABLED = 0x01,
    }
    impl Jd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jd {
        #[inline(always)]
        fn from(val: u8) -> Jd {
            Jd::from_bits(val)
        }
    }
    impl From<Jd> for u8 {
        #[inline(always)]
        fn from(val: Jd) -> u8 {
            Jd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lm {
        #[doc = "Normal mode"]
        NORMAL = 0x0,
        #[doc = "MAC operates in loopback mode at the MII"]
        LOOPBACK = 0x01,
    }
    impl Lm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lm {
        #[inline(always)]
        fn from(val: u8) -> Lm {
            Lm::from_bits(val)
        }
    }
    impl From<Lm> for u8 {
        #[inline(always)]
        fn from(val: Lm) -> u8 {
            Lm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MacahrSa {
        #[doc = "This address is used for comparison with DA fields of the received frame"]
        DESTINATION = 0x0,
        #[doc = "This address is used for comparison with SA fields of received frames"]
        SOURCE = 0x01,
    }
    impl MacahrSa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MacahrSa {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MacahrSa {
        #[inline(always)]
        fn from(val: u8) -> MacahrSa {
            MacahrSa::from_bits(val)
        }
    }
    impl From<MacahrSa> for u8 {
        #[inline(always)]
        fn from(val: MacahrSa) -> u8 {
            MacahrSa::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mb {
        #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
        NORMAL = 0x0,
        #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
        MIXED = 0x01,
    }
    impl Mb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mb {
        #[inline(always)]
        fn from(val: u8) -> Mb {
            Mb::from_bits(val)
        }
    }
    impl From<Mb> for u8 {
        #[inline(always)]
        fn from(val: Mb) -> u8 {
            Mb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MbProgress {
        _RESERVED_0 = 0x0,
        #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
        BUSY = 0x01,
    }
    impl MbProgress {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MbProgress {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MbProgress {
        #[inline(always)]
        fn from(val: u8) -> MbProgress {
            MbProgress::from_bits(val)
        }
    }
    impl From<MbProgress> for u8 {
        #[inline(always)]
        fn from(val: MbProgress) -> u8 {
            MbProgress::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcfhp {
        #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
        ALMOST_HALF = 0x0,
        #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
        ALMOST_FULL = 0x01,
    }
    impl Mcfhp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcfhp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcfhp {
        #[inline(always)]
        fn from(val: u8) -> Mcfhp {
            Mcfhp::from_bits(val)
        }
    }
    impl From<Mcfhp> for u8 {
        #[inline(always)]
        fn from(val: Mcfhp) -> u8 {
            Mcfhp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcp {
        _RESERVED_0 = 0x0,
        #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
        PRESET = 0x01,
    }
    impl Mcp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcp {
        #[inline(always)]
        fn from(val: u8) -> Mcp {
            Mcp::from_bits(val)
        }
    }
    impl From<Mcp> for u8 {
        #[inline(always)]
        fn from(val: Mcp) -> u8 {
            Mcp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mw {
        #[doc = "Read operation"]
        READ = 0x0,
        #[doc = "Write operation"]
        WRITE = 0x01,
    }
    impl Mw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mw {
        #[inline(always)]
        fn from(val: u8) -> Mw {
            Mw::from_bits(val)
        }
    }
    impl From<Mw> for u8 {
        #[inline(always)]
        fn from(val: Mw) -> u8 {
            Mw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pbl {
        _RESERVED_0 = 0x0,
        #[doc = "Maximum of 1 beat per DMA transaction"]
        PBL1 = 0x01,
        #[doc = "Maximum of 2 beats per DMA transaction"]
        PBL2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Maximum of 4 beats per DMA transaction"]
        PBL4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Maximum of 8 beats per DMA transaction"]
        PBL8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "Maximum of 16 beats per DMA transaction"]
        PBL16 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        #[doc = "Maximum of 32 beats per DMA transaction"]
        PBL32 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Pbl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pbl {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pbl {
        #[inline(always)]
        fn from(val: u8) -> Pbl {
            Pbl::from_bits(val)
        }
    }
    impl From<Pbl> for u8 {
        #[inline(always)]
        fn from(val: Pbl) -> u8 {
            Pbl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcf {
        #[doc = "MAC prevents all control frames from reaching the application"]
        PREVENT_ALL = 0x0,
        #[doc = "MAC forwards all control frames to application except Pause"]
        FORWARD_ALL_EXCEPT_PAUSE = 0x01,
        #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
        FORWARD_ALL = 0x02,
        #[doc = "MAC forwards control frames that pass the address filter"]
        FORWARD_ALL_FILTERED = 0x03,
    }
    impl Pcf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcf {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcf {
        #[inline(always)]
        fn from(val: u8) -> Pcf {
            Pcf::from_bits(val)
        }
    }
    impl From<Pcf> for u8 {
        #[inline(always)]
        fn from(val: Pcf) -> u8 {
            Pcf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pd {
        _RESERVED_0 = 0x0,
        #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
        ENABLED = 0x01,
    }
    impl Pd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pd {
        #[inline(always)]
        fn from(val: u8) -> Pd {
            Pd::from_bits(val)
        }
    }
    impl From<Pd> for u8 {
        #[inline(always)]
        fn from(val: Pd) -> u8 {
            Pd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plt {
        #[doc = "Pause time minus 4 slot times"]
        PLT4 = 0x0,
        #[doc = "Pause time minus 28 slot times"]
        PLT28 = 0x01,
        #[doc = "Pause time minus 144 slot times"]
        PLT144 = 0x02,
        #[doc = "Pause time minus 256 slot times"]
        PLT256 = 0x03,
    }
    impl Plt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plt {
        #[inline(always)]
        fn from(val: u8) -> Plt {
            Plt::from_bits(val)
        }
    }
    impl From<Plt> for u8 {
        #[inline(always)]
        fn from(val: Plt) -> u8 {
            Plt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pmtim {
        #[doc = "PMT Status interrupt generation enabled"]
        UNMASKED = 0x0,
        #[doc = "PMT Status interrupt generation disabled"]
        MASKED = 0x01,
    }
    impl Pmtim {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pmtim {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pmtim {
        #[inline(always)]
        fn from(val: u8) -> Pmtim {
            Pmtim::from_bits(val)
        }
    }
    impl From<Pmtim> for u8 {
        #[inline(always)]
        fn from(val: Pmtim) -> u8 {
            Pmtim::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PriorityRxOverTx {
        #[doc = "RxDMA priority over TxDMA is 1:1"]
        ONE_TO_ONE = 0x0,
        #[doc = "RxDMA priority over TxDMA is 2:1"]
        TWO_TO_ONE = 0x01,
        #[doc = "RxDMA priority over TxDMA is 3:1"]
        THREE_TO_ONE = 0x02,
        #[doc = "RxDMA priority over TxDMA is 4:1"]
        FOUR_TO_ONE = 0x03,
    }
    impl PriorityRxOverTx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PriorityRxOverTx {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PriorityRxOverTx {
        #[inline(always)]
        fn from(val: u8) -> PriorityRxOverTx {
            PriorityRxOverTx::from_bits(val)
        }
    }
    impl From<PriorityRxOverTx> for u8 {
        #[inline(always)]
        fn from(val: PriorityRxOverTx) -> u8 {
            PriorityRxOverTx::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rd {
        #[doc = "MAC attempts retries based on the settings of BL"]
        ENABLED = 0x0,
        #[doc = "MAC attempts only 1 transmission"]
        DISABLED = 0x01,
    }
    impl Rd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rd {
        #[inline(always)]
        fn from(val: u8) -> Rd {
            Rd::from_bits(val)
        }
    }
    impl From<Rd> for u8 {
        #[inline(always)]
        fn from(val: Rd) -> u8 {
            Rd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rdp {
        _RESERVED_0 = 0x0,
        #[doc = "1 beat per RxDMA transaction"]
        RDP1 = 0x01,
        #[doc = "2 beats per RxDMA transaction"]
        RDP2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "4 beats per RxDMA transaction"]
        RDP4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "8 beats per RxDMA transaction"]
        RDP8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "16 beats per RxDMA transaction"]
        RDP16 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        #[doc = "32 beats per RxDMA transaction"]
        RDP32 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Rdp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rdp {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rdp {
        #[inline(always)]
        fn from(val: u8) -> Rdp {
            Rdp::from_bits(val)
        }
    }
    impl From<Rdp> for u8 {
        #[inline(always)]
        fn from(val: Rdp) -> u8 {
            Rdp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rfaem {
        #[doc = "Received-alignment-error counter half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Received-alignment-error counter half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Rfaem {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfaem {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfaem {
        #[inline(always)]
        fn from(val: u8) -> Rfaem {
            Rfaem::from_bits(val)
        }
    }
    impl From<Rfaem> for u8 {
        #[inline(always)]
        fn from(val: Rfaem) -> u8 {
            Rfaem::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rfcem {
        #[doc = "Received-crc-error counter half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Received-crc-error counter half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Rfcem {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfcem {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfcem {
        #[inline(always)]
        fn from(val: u8) -> Rfcem {
            Rfcem::from_bits(val)
        }
    }
    impl From<Rfcem> for u8 {
        #[inline(always)]
        fn from(val: Rfcem) -> u8 {
            Rfcem::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rgufm {
        #[doc = "Received-good-unicast counter half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Received-good-unicast counter half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Rgufm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rgufm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rgufm {
        #[inline(always)]
        fn from(val: u8) -> Rgufm {
            Rgufm::from_bits(val)
        }
    }
    impl From<Rgufm> for u8 {
        #[inline(always)]
        fn from(val: Rgufm) -> u8 {
            Rgufm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rod {
        #[doc = "MAC receives all packets from PHY while transmitting"]
        ENABLED = 0x0,
        #[doc = "MAC disables reception of frames in half-duplex mode"]
        DISABLED = 0x01,
    }
    impl Rod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rod {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rod {
        #[inline(always)]
        fn from(val: u8) -> Rod {
            Rod::from_bits(val)
        }
    }
    impl From<Rod> for u8 {
        #[inline(always)]
        fn from(val: Rod) -> u8 {
            Rod::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpd(u32);
    impl Rpd {
        #[doc = "Poll the receive descriptor list"]
        pub const POLL: Self = Self(0x0);
    }
    impl Rpd {
        pub const fn from_bits(val: u32) -> Rpd {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Rpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("POLL"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rpd {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "POLL"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Rpd {
        #[inline(always)]
        fn from(val: u32) -> Rpd {
            Rpd::from_bits(val)
        }
    }
    impl From<Rpd> for u32 {
        #[inline(always)]
        fn from(val: Rpd) -> u32 {
            Rpd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rps {
        #[doc = "Stopped, reset or Stop Receive command issued"]
        STOPPED = 0x0,
        #[doc = "Running, fetching receive transfer descriptor"]
        RUNNING_FETCHING = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Running, waiting for receive packet"]
        RUNNING_WAITING = 0x03,
        #[doc = "Suspended, receive descriptor unavailable"]
        SUSPENDED = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "Running, writing data to host memory buffer"]
        RUNNING_WRITING = 0x07,
    }
    impl Rps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rps {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rps {
        #[inline(always)]
        fn from(val: u8) -> Rps {
            Rps::from_bits(val)
        }
    }
    impl From<Rps> for u8 {
        #[inline(always)]
        fn from(val: Rps) -> u8 {
            Rps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rsf {
        #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
        CUT_THROUGH = 0x0,
        #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
        STORE_FORWARD = 0x01,
    }
    impl Rsf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rsf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rsf {
        #[inline(always)]
        fn from(val: u8) -> Rsf {
            Rsf::from_bits(val)
        }
    }
    impl From<Rsf> for u8 {
        #[inline(always)]
        fn from(val: Rsf) -> u8 {
            Rsf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtc {
        #[doc = "64 bytes"]
        RTC64 = 0x0,
        #[doc = "32 bytes"]
        RTC32 = 0x01,
        #[doc = "96 bytes"]
        RTC96 = 0x02,
        #[doc = "128 bytes"]
        RTC128 = 0x03,
    }
    impl Rtc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtc {
        #[inline(always)]
        fn from(val: u8) -> Rtc {
            Rtc::from_bits(val)
        }
    }
    impl From<Rtc> for u8 {
        #[inline(always)]
        fn from(val: Rtc) -> u8 {
            Rtc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Saif {
        #[doc = "Source address filter operates normally"]
        NORMAL = 0x0,
        #[doc = "Source address filter operation inverted"]
        INVERT = 0x01,
    }
    impl Saif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saif {
        #[inline(always)]
        fn from(val: u8) -> Saif {
            Saif::from_bits(val)
        }
    }
    impl From<Saif> for u8 {
        #[inline(always)]
        fn from(val: Saif) -> u8 {
            Saif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum St {
        #[doc = "Transmission is placed in the Stopped state"]
        STOPPED = 0x0,
        #[doc = "Transmission is placed in Running state"]
        STARTED = 0x01,
    }
    impl St {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> St {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for St {
        #[inline(always)]
        fn from(val: u8) -> St {
            St::from_bits(val)
        }
    }
    impl From<St> for u8 {
        #[inline(always)]
        fn from(val: St) -> u8 {
            St::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tgfm {
        #[doc = "Transmitted-good counter half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Transmitted-good counter half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Tgfm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tgfm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tgfm {
        #[inline(always)]
        fn from(val: u8) -> Tgfm {
            Tgfm::from_bits(val)
        }
    }
    impl From<Tgfm> for u8 {
        #[inline(always)]
        fn from(val: Tgfm) -> u8 {
            Tgfm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tgfmscm {
        #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Tgfmscm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tgfmscm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tgfmscm {
        #[inline(always)]
        fn from(val: u8) -> Tgfmscm {
            Tgfmscm::from_bits(val)
        }
    }
    impl From<Tgfmscm> for u8 {
        #[inline(always)]
        fn from(val: Tgfmscm) -> u8 {
            Tgfmscm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tgfscm {
        #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
        UNMASKED = 0x0,
        #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
        MASKED = 0x01,
    }
    impl Tgfscm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tgfscm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tgfscm {
        #[inline(always)]
        fn from(val: u8) -> Tgfscm {
            Tgfscm::from_bits(val)
        }
    }
    impl From<Tgfscm> for u8 {
        #[inline(always)]
        fn from(val: Tgfscm) -> u8 {
            Tgfscm::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpd(u32);
    impl Tpd {
        #[doc = "Poll the transmit descriptor list"]
        pub const POLL: Self = Self(0x0);
    }
    impl Tpd {
        pub const fn from_bits(val: u32) -> Tpd {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Tpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("POLL"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tpd {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "POLL"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Tpd {
        #[inline(always)]
        fn from(val: u32) -> Tpd {
            Tpd::from_bits(val)
        }
    }
    impl From<Tpd> for u32 {
        #[inline(always)]
        fn from(val: Tpd) -> u32 {
            Tpd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tps {
        #[doc = "Stopped, Reset or Stop Transmit command issued"]
        STOPPED = 0x0,
        #[doc = "Running, fetching transmit transfer descriptor"]
        RUNNING_FETCHING = 0x01,
        #[doc = "Running, waiting for status"]
        RUNNING_WAITING = 0x02,
        #[doc = "Running, reading data from host memory buffer"]
        RUNNING_READING = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "Suspended, transmit descriptor unavailable or transmit buffer underflow"]
        SUSPENDED = 0x06,
        #[doc = "Running, closing transmit descriptor"]
        RUNNING = 0x07,
    }
    impl Tps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tps {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tps {
        #[inline(always)]
        fn from(val: u8) -> Tps {
            Tps::from_bits(val)
        }
    }
    impl From<Tps> for u8 {
        #[inline(always)]
        fn from(val: Tps) -> u8 {
            Tps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsf {
        #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
        CUT_THROUGH = 0x0,
        #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
        STORE_FORWARD = 0x01,
    }
    impl Tsf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsf {
        #[inline(always)]
        fn from(val: u8) -> Tsf {
            Tsf::from_bits(val)
        }
    }
    impl From<Tsf> for u8 {
        #[inline(always)]
        fn from(val: Tsf) -> u8 {
            Tsf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tstim {
        #[doc = "Time stamp interrupt generation enabled"]
        UNMASKED = 0x0,
        #[doc = "Time stamp interrupt generation disabled"]
        MASKED = 0x01,
    }
    impl Tstim {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tstim {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tstim {
        #[inline(always)]
        fn from(val: u8) -> Tstim {
            Tstim::from_bits(val)
        }
    }
    impl From<Tstim> for u8 {
        #[inline(always)]
        fn from(val: Tstim) -> u8 {
            Tstim::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ttc {
        #[doc = "64 bytes"]
        TTC64 = 0x0,
        #[doc = "128 bytes"]
        TTC128 = 0x01,
        #[doc = "192 bytes"]
        TTC192 = 0x02,
        #[doc = "256 bytes"]
        TTC256 = 0x03,
        #[doc = "40 bytes"]
        TTC40 = 0x04,
        #[doc = "32 bytes"]
        TTC32 = 0x05,
        #[doc = "24 bytes"]
        TTC24 = 0x06,
        #[doc = "16 bytes"]
        TTC16 = 0x07,
    }
    impl Ttc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ttc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ttc {
        #[inline(always)]
        fn from(val: u8) -> Ttc {
            Ttc::from_bits(val)
        }
    }
    impl From<Ttc> for u8 {
        #[inline(always)]
        fn from(val: Ttc) -> u8 {
            Ttc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usp {
        #[doc = "PBL value used for both Rx and Tx DMA"]
        COMBINED = 0x0,
        #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
        SEPARATE = 0x01,
    }
    impl Usp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usp {
        #[inline(always)]
        fn from(val: u8) -> Usp {
            Usp::from_bits(val)
        }
    }
    impl From<Usp> for u8 {
        #[inline(always)]
        fn from(val: Usp) -> u8 {
            Usp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vlantc {
        #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
        VLANTC16 = 0x0,
        #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
        VLANTC12 = 0x01,
    }
    impl Vlantc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vlantc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vlantc {
        #[inline(always)]
        fn from(val: u8) -> Vlantc {
            Vlantc::from_bits(val)
        }
    }
    impl From<Vlantc> for u8 {
        #[inline(always)]
        fn from(val: Vlantc) -> u8 {
            Vlantc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wd {
        #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
        ENABLED = 0x0,
        #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
        DISABLED = 0x01,
    }
    impl Wd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wd {
        #[inline(always)]
        fn from(val: u8) -> Wd {
            Wd::from_bits(val)
        }
    }
    impl From<Wd> for u8 {
        #[inline(always)]
        fn from(val: Wd) -> u8 {
            Wd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wffrpr {
        _RESERVED_0 = 0x0,
        #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
        RESET = 0x01,
    }
    impl Wffrpr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wffrpr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wffrpr {
        #[inline(always)]
        fn from(val: u8) -> Wffrpr {
            Wffrpr::from_bits(val)
        }
    }
    impl From<Wffrpr> for u8 {
        #[inline(always)]
        fn from(val: Wffrpr) -> u8 {
            Wffrpr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Zqpd {
        #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
        ENABLED = 0x0,
        #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
        DISABLED = 0x01,
    }
    impl Zqpd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Zqpd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Zqpd {
        #[inline(always)]
        fn from(val: u8) -> Zqpd {
            Zqpd::from_bits(val)
        }
    }
    impl From<Zqpd> for u8 {
        #[inline(always)]
        fn from(val: Zqpd) -> u8 {
            Zqpd::to_bits(val)
        }
    }
}
