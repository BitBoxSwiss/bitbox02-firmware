#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MDMA channel x interrupt/status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "MDMA channel x interrupt flag clear register."]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "MDMA Channel x error status register."]
    #[inline(always)]
    pub const fn esr(self) -> crate::common::Reg<regs::Esr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "MDMA Channel x block number of data register."]
    #[inline(always)]
    pub const fn bndtr(self) -> crate::common::Reg<regs::Bndtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "MDMA channel x source address register."]
    #[inline(always)]
    pub const fn sar(self) -> crate::common::Reg<regs::Sar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "MDMA channel x destination address register."]
    #[inline(always)]
    pub const fn dar(self) -> crate::common::Reg<regs::Dar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "MDMA channel x Block Repeat address Update register."]
    #[inline(always)]
    pub const fn brur(self) -> crate::common::Reg<regs::Brur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MDMA channel x Link Address register."]
    #[inline(always)]
    pub const fn lar(self) -> crate::common::Reg<regs::Lar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MDMA channel x Trigger and Bus selection Register."]
    #[inline(always)]
    pub const fn tbr(self) -> crate::common::Reg<regs::Tbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "MDMA channel x Mask address register."]
    #[inline(always)]
    pub const fn mar(self) -> crate::common::Reg<regs::Mar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "MDMA channel x Mask Data register."]
    #[inline(always)]
    pub const fn mdr(self) -> crate::common::Reg<regs::Mdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
#[doc = "MDMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdma {
    ptr: *mut u8,
}
unsafe impl Send for Mdma {}
unsafe impl Sync for Mdma {}
impl Mdma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MDMA Global Interrupt/Status Register."]
    #[inline(always)]
    pub const fn gisr0(self) -> crate::common::Reg<regs::Gisr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 16usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x40usize + n * 64usize) as _) }
    }
}
pub mod regs {
    #[doc = "MDMA Channel x block number of data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bndtr(pub u32);
    impl Bndtr {
        #[doc = "block number of data to transfer."]
        #[inline(always)]
        pub const fn bndt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "block number of data to transfer."]
        #[inline(always)]
        pub fn set_bndt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
        #[doc = "Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn brsum(&self) -> super::vals::Updatemode {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Updatemode::from_bits(val as u8)
        }
        #[doc = "Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_brsum(&mut self, val: super::vals::Updatemode) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn brdum(&self) -> super::vals::Updatemode {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Updatemode::from_bits(val as u8)
        }
        #[doc = "Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_brdum(&mut self, val: super::vals::Updatemode) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn brc(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_brc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Bndtr {
        #[inline(always)]
        fn default() -> Bndtr {
            Bndtr(0)
        }
    }
    impl core::fmt::Debug for Bndtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bndtr")
                .field("bndt", &self.bndt())
                .field("brsum", &self.brsum())
                .field("brdum", &self.brdum())
                .field("brc", &self.brc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bndtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bndtr {{ bndt: {=u32:?}, brsum: {:?}, brdum: {:?}, brc: {=u16:?} }}",
                self.bndt(),
                self.brsum(),
                self.brdum(),
                self.brc()
            )
        }
    }
    #[doc = "MDMA channel x Block Repeat address Update register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brur(pub u32);
    impl Brur {
        #[doc = "source adresse update value."]
        #[inline(always)]
        pub const fn suv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "source adresse update value."]
        #[inline(always)]
        pub fn set_suv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "destination address update."]
        #[inline(always)]
        pub const fn duv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "destination address update."]
        #[inline(always)]
        pub fn set_duv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Brur {
        #[inline(always)]
        fn default() -> Brur {
            Brur(0)
        }
    }
    impl core::fmt::Debug for Brur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brur")
                .field("suv", &self.suv())
                .field("duv", &self.duv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brur {{ suv: {=u16:?}, duv: {=u16:?} }}", self.suv(), self.duv())
        }
    }
    #[doc = "This register is used to control the concerned channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "channel enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "channel enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer error interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn ctcie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_ctcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Block Repeat transfer interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn brtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Block Repeat transfer interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_brtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Block Transfer interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn btie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Block Transfer interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_btie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn pl(&self) -> super::vals::Pl {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Pl::from_bits(val as u8)
        }
        #[doc = "Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_pl(&mut self, val: super::vals::Pl) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "byte Endianness exchange."]
        #[inline(always)]
        pub const fn bex(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "byte Endianness exchange."]
        #[inline(always)]
        pub fn set_bex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Half word Endianes exchange."]
        #[inline(always)]
        pub const fn hex(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Half word Endianes exchange."]
        #[inline(always)]
        pub fn set_hex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Word Endianness exchange."]
        #[inline(always)]
        pub const fn wex(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Word Endianness exchange."]
        #[inline(always)]
        pub fn set_wex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
        #[inline(always)]
        pub const fn swrq(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
        #[inline(always)]
        pub fn set_swrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("en", &self.en())
                .field("teie", &self.teie())
                .field("ctcie", &self.ctcie())
                .field("brtie", &self.brtie())
                .field("btie", &self.btie())
                .field("tcie", &self.tcie())
                .field("pl", &self.pl())
                .field("bex", &self.bex())
                .field("hex", &self.hex())
                .field("wex", &self.wex())
                .field("swrq", &self.swrq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, teie: {=bool:?}, ctcie: {=bool:?}, brtie: {=bool:?}, btie: {=bool:?}, tcie: {=bool:?}, pl: {:?}, bex: {=bool:?}, hex: {=bool:?}, wex: {=bool:?}, swrq: {=bool:?} }}" , self . en () , self . teie () , self . ctcie () , self . brtie () , self . btie () , self . tcie () , self . pl () , self . bex () , self . hex () , self . wex () , self . swrq ())
        }
    }
    #[doc = "MDMA channel x destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dar(pub u32);
    impl Dar {
        #[doc = "Destination adr base."]
        #[inline(always)]
        pub const fn dar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination adr base."]
        #[inline(always)]
        pub fn set_dar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dar {
        #[inline(always)]
        fn default() -> Dar {
            Dar(0)
        }
    }
    impl core::fmt::Debug for Dar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dar").field("dar", &self.dar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dar {{ dar: {=u32:?} }}", self.dar())
        }
    }
    #[doc = "MDMA Channel x error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esr(pub u32);
    impl Esr {
        #[doc = "Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error."]
        #[inline(always)]
        pub const fn tea(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error."]
        #[inline(always)]
        pub fn set_tea(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error."]
        #[inline(always)]
        pub const fn ted(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error."]
        #[inline(always)]
        pub fn set_ted(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn teld(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_teld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn temd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_temd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn ase(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_ase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn bse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_bse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Esr {
        #[inline(always)]
        fn default() -> Esr {
            Esr(0)
        }
    }
    impl core::fmt::Debug for Esr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esr")
                .field("tea", &self.tea())
                .field("ted", &self.ted())
                .field("teld", &self.teld())
                .field("temd", &self.temd())
                .field("ase", &self.ase())
                .field("bse", &self.bse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Esr {{ tea: {=u8:?}, ted: {=bool:?}, teld: {=bool:?}, temd: {=bool:?}, ase: {=bool:?}, bse: {=bool:?} }}" , self . tea () , self . ted () , self . teld () , self . temd () , self . ase () , self . bse ())
        }
    }
    #[doc = "MDMA Global Interrupt/Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gisr0(pub u32);
    impl Gisr0 {
        #[doc = "Channel x global interrupt flag (x=0..15) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)."]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel x global interrupt flag (x=0..15) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)."]
        #[inline(always)]
        pub fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Gisr0 {
        #[inline(always)]
        fn default() -> Gisr0 {
            Gisr0(0)
        }
    }
    impl core::fmt::Debug for Gisr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gisr0")
                .field("gif[0]", &self.gif(0usize))
                .field("gif[1]", &self.gif(1usize))
                .field("gif[2]", &self.gif(2usize))
                .field("gif[3]", &self.gif(3usize))
                .field("gif[4]", &self.gif(4usize))
                .field("gif[5]", &self.gif(5usize))
                .field("gif[6]", &self.gif(6usize))
                .field("gif[7]", &self.gif(7usize))
                .field("gif[8]", &self.gif(8usize))
                .field("gif[9]", &self.gif(9usize))
                .field("gif[10]", &self.gif(10usize))
                .field("gif[11]", &self.gif(11usize))
                .field("gif[12]", &self.gif(12usize))
                .field("gif[13]", &self.gif(13usize))
                .field("gif[14]", &self.gif(14usize))
                .field("gif[15]", &self.gif(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gisr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gisr0 {{ gif[0]: {=bool:?}, gif[1]: {=bool:?}, gif[2]: {=bool:?}, gif[3]: {=bool:?}, gif[4]: {=bool:?}, gif[5]: {=bool:?}, gif[6]: {=bool:?}, gif[7]: {=bool:?}, gif[8]: {=bool:?}, gif[9]: {=bool:?}, gif[10]: {=bool:?}, gif[11]: {=bool:?}, gif[12]: {=bool:?}, gif[13]: {=bool:?}, gif[14]: {=bool:?}, gif[15]: {=bool:?} }}" , self . gif (0usize) , self . gif (1usize) , self . gif (2usize) , self . gif (3usize) , self . gif (4usize) , self . gif (5usize) , self . gif (6usize) , self . gif (7usize) , self . gif (8usize) , self . gif (9usize) , self . gif (10usize) , self . gif (11usize) , self . gif (12usize) , self . gif (13usize) , self . gif (14usize) , self . gif (15usize))
        }
    }
    #[doc = "MDMA channel x interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub const fn cteif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub const fn cctcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub fn set_cctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub const fn cbrtif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub fn set_cbrtif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub const fn cbtif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub fn set_cbtif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub const fn cltcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register."]
        #[inline(always)]
        pub fn set_cltcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Ifcr {
        #[inline(always)]
        fn default() -> Ifcr {
            Ifcr(0)
        }
    }
    impl core::fmt::Debug for Ifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifcr")
                .field("cteif", &self.cteif())
                .field("cctcif", &self.cctcif())
                .field("cbrtif", &self.cbrtif())
                .field("cbtif", &self.cbtif())
                .field("cltcif", &self.cltcif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ifcr {{ cteif: {=bool:?}, cctcif: {=bool:?}, cbrtif: {=bool:?}, cbtif: {=bool:?}, cltcif: {=bool:?} }}" , self . cteif () , self . cctcif () , self . cbrtif () , self . cbtif () , self . cltcif ())
        }
    }
    #[doc = "MDMA channel x interrupt/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
        #[inline(always)]
        pub const fn ctcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
        #[inline(always)]
        pub fn set_ctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn brtif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_brtif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub const fn btif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
        #[inline(always)]
        pub fn set_btif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "channel x buffer transfer complete."]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "channel x buffer transfer complete."]
        #[inline(always)]
        pub fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "channel x request active flag."]
        #[inline(always)]
        pub const fn crqa(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "channel x request active flag."]
        #[inline(always)]
        pub fn set_crqa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("teif", &self.teif())
                .field("ctcif", &self.ctcif())
                .field("brtif", &self.brtif())
                .field("btif", &self.btif())
                .field("tcif", &self.tcif())
                .field("crqa", &self.crqa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ teif: {=bool:?}, ctcif: {=bool:?}, brtif: {=bool:?}, btif: {=bool:?}, tcif: {=bool:?}, crqa: {=bool:?} }}" , self . teif () , self . ctcif () , self . brtif () , self . btif () , self . tcif () , self . crqa ())
        }
    }
    #[doc = "MDMA channel x Link Address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lar(pub u32);
    impl Lar {
        #[doc = "Link address register."]
        #[inline(always)]
        pub const fn lar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Link address register."]
        #[inline(always)]
        pub fn set_lar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lar {
        #[inline(always)]
        fn default() -> Lar {
            Lar(0)
        }
    }
    impl core::fmt::Debug for Lar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lar").field("lar", &self.lar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lar {{ lar: {=u32:?} }}", self.lar())
        }
    }
    #[doc = "MDMA channel x Mask address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mar(pub u32);
    impl Mar {
        #[doc = "Mask address."]
        #[inline(always)]
        pub const fn mar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mask address."]
        #[inline(always)]
        pub fn set_mar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mar {
        #[inline(always)]
        fn default() -> Mar {
            Mar(0)
        }
    }
    impl core::fmt::Debug for Mar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mar").field("mar", &self.mar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mar {{ mar: {=u32:?} }}", self.mar())
        }
    }
    #[doc = "MDMA channel x Mask Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdr(pub u32);
    impl Mdr {
        #[doc = "Mask data."]
        #[inline(always)]
        pub const fn mdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mask data."]
        #[inline(always)]
        pub fn set_mdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mdr {
        #[inline(always)]
        fn default() -> Mdr {
            Mdr(0)
        }
    }
    impl core::fmt::Debug for Mdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdr").field("mdr", &self.mdr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mdr {{ mdr: {=u32:?} }}", self.mdr())
        }
    }
    #[doc = "MDMA channel x source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sar(pub u32);
    impl Sar {
        #[doc = "source adr base."]
        #[inline(always)]
        pub const fn sar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "source adr base."]
        #[inline(always)]
        pub fn set_sar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sar {
        #[inline(always)]
        fn default() -> Sar {
            Sar(0)
        }
    }
    impl core::fmt::Debug for Sar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sar").field("sar", &self.sar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sar {{ sar: {=u32:?} }}", self.sar())
        }
    }
    #[doc = "MDMA channel x Trigger and Bus selection Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tbr(pub u32);
    impl Tbr {
        #[doc = "Trigger selection."]
        #[inline(always)]
        pub const fn tsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Trigger selection."]
        #[inline(always)]
        pub fn set_tsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Source BUS select This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn sbus(&self) -> super::vals::Bus {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Bus::from_bits(val as u8)
        }
        #[doc = "Source BUS select This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_sbus(&mut self, val: super::vals::Bus) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Destination BUS slect This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn dbus(&self) -> super::vals::Bus {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Bus::from_bits(val as u8)
        }
        #[doc = "Destination BUS slect This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_dbus(&mut self, val: super::vals::Bus) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Tbr {
        #[inline(always)]
        fn default() -> Tbr {
            Tbr(0)
        }
    }
    impl core::fmt::Debug for Tbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tbr")
                .field("tsel", &self.tsel())
                .field("sbus", &self.sbus())
                .field("dbus", &self.dbus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tbr {{ tsel: {=u8:?}, sbus: {:?}, dbus: {:?} }}",
                self.tsel(),
                self.sbus(),
                self.dbus()
            )
        }
    }
    #[doc = "This register is used to configure the concerned channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcr(pub u32);
    impl Tcr {
        #[doc = "Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
        #[inline(always)]
        pub const fn sinc(&self) -> super::vals::Incmode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Incmode::from_bits(val as u8)
        }
        #[doc = "Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
        #[inline(always)]
        pub fn set_sinc(&mut self, val: super::vals::Incmode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
        #[inline(always)]
        pub const fn dinc(&self) -> super::vals::Incmode {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Incmode::from_bits(val as u8)
        }
        #[doc = "Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
        #[inline(always)]
        pub fn set_dinc(&mut self, val: super::vals::Incmode) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
        #[inline(always)]
        pub const fn ssize(&self) -> super::vals::Wordsize {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Wordsize::from_bits(val as u8)
        }
        #[doc = "Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
        #[inline(always)]
        pub fn set_ssize(&mut self, val: super::vals::Wordsize) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
        #[inline(always)]
        pub const fn dsize(&self) -> super::vals::Wordsize {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Wordsize::from_bits(val as u8)
        }
        #[doc = "Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
        #[inline(always)]
        pub fn set_dsize(&mut self, val: super::vals::Wordsize) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "source increment offset size."]
        #[inline(always)]
        pub const fn sincos(&self) -> super::vals::Wordsize {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Wordsize::from_bits(val as u8)
        }
        #[doc = "source increment offset size."]
        #[inline(always)]
        pub fn set_sincos(&mut self, val: super::vals::Wordsize) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Destination increment offset."]
        #[inline(always)]
        pub const fn dincos(&self) -> super::vals::Wordsize {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Wordsize::from_bits(val as u8)
        }
        #[doc = "Destination increment offset."]
        #[inline(always)]
        pub fn set_dincos(&mut self, val: super::vals::Wordsize) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "source burst transfer configuration."]
        #[inline(always)]
        pub const fn sburst(&self) -> super::vals::Burst {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Burst::from_bits(val as u8)
        }
        #[doc = "source burst transfer configuration."]
        #[inline(always)]
        pub fn set_sburst(&mut self, val: super::vals::Burst) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "Destination burst transfer configuration."]
        #[inline(always)]
        pub const fn dburst(&self) -> super::vals::Burst {
            let val = (self.0 >> 15usize) & 0x07;
            super::vals::Burst::from_bits(val as u8)
        }
        #[doc = "Destination burst transfer configuration."]
        #[inline(always)]
        pub fn set_dburst(&mut self, val: super::vals::Burst) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
        }
        #[doc = "buffer transfer lengh."]
        #[inline(always)]
        pub const fn tlen(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x7f;
            val as u8
        }
        #[doc = "buffer transfer lengh."]
        #[inline(always)]
        pub fn set_tlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
        }
        #[doc = "PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn pke(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_pke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn pam(&self) -> super::vals::Pam {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Pam::from_bits(val as u8)
        }
        #[doc = "Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_pam(&mut self, val: super::vals::Pam) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn trgm(&self) -> super::vals::Trgm {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Trgm::from_bits(val as u8)
        }
        #[doc = "Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_trgm(&mut self, val: super::vals::Trgm) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub const fn swrm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
        #[inline(always)]
        pub fn set_swrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
        #[inline(always)]
        pub const fn bwm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
        #[inline(always)]
        pub fn set_bwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tcr {
        #[inline(always)]
        fn default() -> Tcr {
            Tcr(0)
        }
    }
    impl core::fmt::Debug for Tcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tcr")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("ssize", &self.ssize())
                .field("dsize", &self.dsize())
                .field("sincos", &self.sincos())
                .field("dincos", &self.dincos())
                .field("sburst", &self.sburst())
                .field("dburst", &self.dburst())
                .field("tlen", &self.tlen())
                .field("pke", &self.pke())
                .field("pam", &self.pam())
                .field("trgm", &self.trgm())
                .field("swrm", &self.swrm())
                .field("bwm", &self.bwm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tcr {{ sinc: {:?}, dinc: {:?}, ssize: {:?}, dsize: {:?}, sincos: {:?}, dincos: {:?}, sburst: {:?}, dburst: {:?}, tlen: {=u8:?}, pke: {=bool:?}, pam: {:?}, trgm: {:?}, swrm: {=bool:?}, bwm: {=bool:?} }}" , self . sinc () , self . dinc () , self . ssize () , self . dsize () , self . sincos () , self . dincos () , self . sburst () , self . dburst () , self . tlen () , self . pke () , self . pam () , self . trgm () , self . swrm () , self . bwm ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Burst {
        #[doc = "Single transfer"]
        SINGLE = 0x0,
        #[doc = "Incremental burst of 4 beats"]
        INCR4 = 0x01,
        #[doc = "Incremental burst of 8 beats"]
        INCR8 = 0x02,
        #[doc = "Incremental burst of 16 beats"]
        INCR16 = 0x03,
        #[doc = "Incremental burst of 32 beats"]
        INCR32 = 0x04,
        #[doc = "Incremental burst of 64 beats"]
        INCR64 = 0x05,
        #[doc = "Incremental burst of 128 beats"]
        INCR128 = 0x06,
        #[doc = "Incremental burst of 256 beats"]
        INCR256 = 0x07,
    }
    impl Burst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Burst {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Burst {
        #[inline(always)]
        fn from(val: u8) -> Burst {
            Burst::from_bits(val)
        }
    }
    impl From<Burst> for u8 {
        #[inline(always)]
        fn from(val: Burst) -> u8 {
            Burst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bus {
        #[doc = "System/AXI bus"]
        SYSTEM = 0x0,
        #[doc = "AHB bus/TCM"]
        AHB = 0x01,
    }
    impl Bus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bus {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bus {
        #[inline(always)]
        fn from(val: u8) -> Bus {
            Bus::from_bits(val)
        }
    }
    impl From<Bus> for u8 {
        #[inline(always)]
        fn from(val: Bus) -> u8 {
            Bus::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Incmode {
        #[doc = "Address pointer is fixed"]
        FIXED = 0x0,
        #[doc = "Reserved"]
        RESERVED = 0x01,
        #[doc = "Address pointer is incremented after each data transfer"]
        INCREMENT = 0x02,
        #[doc = "Address pointer is decremented after each data transfer"]
        DECREMENT = 0x03,
    }
    impl Incmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Incmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Incmode {
        #[inline(always)]
        fn from(val: u8) -> Incmode {
            Incmode::from_bits(val)
        }
    }
    impl From<Incmode> for u8 {
        #[inline(always)]
        fn from(val: Incmode) -> u8 {
            Incmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pam {
        #[doc = "Right aligned, padded with 0s (default). If source data is larger than destination size, only LSB part of the source is written to the destination address. The reminder part is discarded"]
        RIGHT_PADDED = 0x0,
        #[doc = "Right aligned, sign extended"]
        RIGHT_SIGN_EXTENDED = 0x01,
        #[doc = "Left aligned (padded with 0s). if source data is larger than destination size, only MSB part of the source is written to the destination address. The reminder part is discarded"]
        LEFT_PADDED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pam {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pam {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pam {
        #[inline(always)]
        fn from(val: u8) -> Pam {
            Pam::from_bits(val)
        }
    }
    impl From<Pam> for u8 {
        #[inline(always)]
        fn from(val: Pam) -> u8 {
            Pam::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pl {
        #[doc = "Low"]
        LOW = 0x0,
        #[doc = "Medium"]
        MEDIUM = 0x01,
        #[doc = "High"]
        HIGH = 0x02,
        #[doc = "Very high"]
        VERY_HIGH = 0x03,
    }
    impl Pl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pl {
        #[inline(always)]
        fn from(val: u8) -> Pl {
            Pl::from_bits(val)
        }
    }
    impl From<Pl> for u8 {
        #[inline(always)]
        fn from(val: Pl) -> u8 {
            Pl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trgm {
        #[doc = "Each MDMA request (software or hardware) triggers a buffer transfer"]
        BUFFER = 0x0,
        #[doc = "Each MDMA request (software or hardware) triggers a block transfer"]
        BLOCK = 0x01,
        #[doc = "Each MDMA request (software or hardware) triggers a repeated block transfer"]
        REPEATED = 0x02,
        #[doc = "Each MDMA request (software or hardware) triggers the transfer of the whole data for the respective channel (for example linked list) until the channel reach the end and it is disabled."]
        WHOLE_DATA = 0x03,
    }
    impl Trgm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trgm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trgm {
        #[inline(always)]
        fn from(val: u8) -> Trgm {
            Trgm::from_bits(val)
        }
    }
    impl From<Trgm> for u8 {
        #[inline(always)]
        fn from(val: Trgm) -> u8 {
            Trgm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Updatemode {
        #[doc = "Add"]
        ADD = 0x0,
        #[doc = "Subtract"]
        SUBTRACT = 0x01,
    }
    impl Updatemode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Updatemode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Updatemode {
        #[inline(always)]
        fn from(val: u8) -> Updatemode {
            Updatemode::from_bits(val)
        }
    }
    impl From<Updatemode> for u8 {
        #[inline(always)]
        fn from(val: Updatemode) -> u8 {
            Updatemode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wordsize {
        #[doc = "Byte (8-bit)"]
        BYTE = 0x0,
        #[doc = "HalfWord (16-bit)"]
        HALF_WORD = 0x01,
        #[doc = "Word (32-bit)"]
        WORD = 0x02,
        #[doc = "DoubleWord (64-bit)"]
        DOUBLE_WORD = 0x03,
    }
    impl Wordsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wordsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wordsize {
        #[inline(always)]
        fn from(val: u8) -> Wordsize {
            Wordsize::from_bits(val)
        }
    }
    impl From<Wordsize> for u8 {
        #[inline(always)]
        fn from(val: Wordsize) -> u8 {
            Wordsize::to_bits(val)
        }
    }
}
