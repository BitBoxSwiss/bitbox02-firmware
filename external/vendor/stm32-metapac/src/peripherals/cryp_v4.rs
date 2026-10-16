#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cryptographic processor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cryp {
    ptr: *mut u8,
}
unsafe impl Send for Cryp {}
unsafe impl Sync for Cryp {}
impl Cryp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "data input register."]
    #[inline(always)]
    pub const fn din(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "data output register."]
    #[inline(always)]
    pub const fn dout(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "interrupt mask set/clear register."]
    #[inline(always)]
    pub const fn imscr(self) -> crate::common::Reg<regs::Imscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "raw interrupt status register."]
    #[inline(always)]
    pub const fn risr(self) -> crate::common::Reg<regs::Risr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Cluster KEY%s, containing K?LR, K?RR."]
    #[inline(always)]
    pub const fn key(self, n: usize) -> Key {
        assert!(n < 4usize);
        unsafe { Key::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
    #[inline(always)]
    pub const fn init(self, n: usize) -> Init {
        assert!(n < 2usize);
        unsafe { Init::from_ptr(self.ptr.add(0x40usize + n * 8usize) as _) }
    }
    #[doc = "context swap register."]
    #[inline(always)]
    pub const fn csgcmccmr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "context swap register."]
    #[inline(always)]
    pub const fn csgcmr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
}
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Init {
    ptr: *mut u8,
}
unsafe impl Send for Init {}
unsafe impl Sync for Init {}
impl Init {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "initialization vector registers."]
    #[inline(always)]
    pub const fn ivlr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "initialization vector registers."]
    #[inline(always)]
    pub const fn ivrr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Cluster KEY%s, containing K?LR, K?RR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    ptr: *mut u8,
}
unsafe impl Send for Key {}
unsafe impl Sync for Key {}
impl Key {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "key registers."]
    #[inline(always)]
    pub const fn klr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "key registers."]
    #[inline(always)]
    pub const fn krr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Algorithm direction."]
        #[inline(always)]
        pub const fn algodir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Algorithm direction."]
        #[inline(always)]
        pub fn set_algodir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Algorithm mode."]
        #[inline(always)]
        pub const fn algomode0(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Algorithm mode."]
        #[inline(always)]
        pub fn set_algomode0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Key size selection (AES mode only)."]
        #[inline(always)]
        pub const fn keysize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Key size selection (AES mode only)."]
        #[inline(always)]
        pub fn set_keysize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "FIFO flush."]
        #[inline(always)]
        pub const fn fflush(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO flush."]
        #[inline(always)]
        pub fn set_fflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Cryptographic processor enable."]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic processor enable."]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "GCM_CCMPH."]
        #[inline(always)]
        pub const fn gcm_ccmph(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "GCM_CCMPH."]
        #[inline(always)]
        pub fn set_gcm_ccmph(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "ALGOMODE."]
        #[inline(always)]
        pub const fn algomode3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ALGOMODE."]
        #[inline(always)]
        pub fn set_algomode3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Number of Padding Bytes in Last Block of payload."]
        #[inline(always)]
        pub const fn npblb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Padding Bytes in Last Block of payload."]
        #[inline(always)]
        pub fn set_npblb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\\[1:0\\]
different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set."]
        #[inline(always)]
        pub const fn kmod(&self) -> super::vals::Kmod {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Kmod::from_bits(val as u8)
        }
        #[doc = "Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\\[1:0\\]
different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set."]
        #[inline(always)]
        pub fn set_kmod(&mut self, val: super::vals::Kmod) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers."]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers."]
        #[inline(always)]
        pub fn set_iprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("algodir", &self.algodir())
                .field("algomode0", &self.algomode0())
                .field("datatype", &self.datatype())
                .field("keysize", &self.keysize())
                .field("fflush", &self.fflush())
                .field("crypen", &self.crypen())
                .field("gcm_ccmph", &self.gcm_ccmph())
                .field("algomode3", &self.algomode3())
                .field("npblb", &self.npblb())
                .field("kmod", &self.kmod())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ algodir: {=bool:?}, algomode0: {=u8:?}, datatype: {=u8:?}, keysize: {=u8:?}, fflush: {=bool:?}, crypen: {=bool:?}, gcm_ccmph: {=u8:?}, algomode3: {=bool:?}, npblb: {=u8:?}, kmod: {:?}, iprst: {=bool:?} }}" , self . algodir () , self . algomode0 () , self . datatype () , self . keysize () , self . fflush () , self . crypen () , self . gcm_ccmph () , self . algomode3 () , self . npblb () , self . kmod () , self . iprst ())
        }
    }
    #[doc = "DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacr(pub u32);
    impl Dmacr {
        #[doc = "DMA input enable."]
        #[inline(always)]
        pub const fn dien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA input enable."]
        #[inline(always)]
        pub fn set_dien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA output enable."]
        #[inline(always)]
        pub const fn doen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA output enable."]
        #[inline(always)]
        pub fn set_doen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dmacr {
        #[inline(always)]
        fn default() -> Dmacr {
            Dmacr(0)
        }
    }
    impl core::fmt::Debug for Dmacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacr")
                .field("dien", &self.dien())
                .field("doen", &self.doen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacr {{ dien: {=bool:?}, doen: {=bool:?} }}",
                self.dien(),
                self.doen()
            )
        }
    }
    #[doc = "interrupt mask set/clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imscr(pub u32);
    impl Imscr {
        #[doc = "Input FIFO service interrupt mask."]
        #[inline(always)]
        pub const fn inim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service interrupt mask."]
        #[inline(always)]
        pub fn set_inim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service interrupt mask."]
        #[inline(always)]
        pub const fn outim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service interrupt mask."]
        #[inline(always)]
        pub fn set_outim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Imscr {
        #[inline(always)]
        fn default() -> Imscr {
            Imscr(0)
        }
    }
    impl core::fmt::Debug for Imscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imscr")
                .field("inim", &self.inim())
                .field("outim", &self.outim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imscr {{ inim: {=bool:?}, outim: {=bool:?} }}",
                self.inim(),
                self.outim()
            )
        }
    }
    #[doc = "masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Input FIFO service masked interrupt status."]
        #[inline(always)]
        pub const fn inmis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service masked interrupt status."]
        #[inline(always)]
        pub fn set_inmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service masked interrupt status."]
        #[inline(always)]
        pub const fn outmis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service masked interrupt status."]
        #[inline(always)]
        pub fn set_outmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("inmis", &self.inmis())
                .field("outmis", &self.outmis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misr {{ inmis: {=bool:?}, outmis: {=bool:?} }}",
                self.inmis(),
                self.outmis()
            )
        }
    }
    #[doc = "raw interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Risr(pub u32);
    impl Risr {
        #[doc = "Input FIFO service raw interrupt status."]
        #[inline(always)]
        pub const fn inris(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service raw interrupt status."]
        #[inline(always)]
        pub fn set_inris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service raw interrupt status."]
        #[inline(always)]
        pub const fn outris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service raw interrupt status."]
        #[inline(always)]
        pub fn set_outris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Risr {
        #[inline(always)]
        fn default() -> Risr {
            Risr(0)
        }
    }
    impl core::fmt::Debug for Risr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Risr")
                .field("inris", &self.inris())
                .field("outris", &self.outris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Risr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Risr {{ inris: {=bool:?}, outris: {=bool:?} }}",
                self.inris(),
                self.outris()
            )
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Input FIFO empty."]
        #[inline(always)]
        pub const fn ifem(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO empty."]
        #[inline(always)]
        pub fn set_ifem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Input FIFO not full."]
        #[inline(always)]
        pub const fn ifnf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO not full."]
        #[inline(always)]
        pub fn set_ifnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Output FIFO not empty."]
        #[inline(always)]
        pub const fn ofne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO not empty."]
        #[inline(always)]
        pub fn set_ofne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO full."]
        #[inline(always)]
        pub const fn offu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO full."]
        #[inline(always)]
        pub fn set_offu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Key error flag This read-only bit is set by hardware when key information failed to load into key registers. KERF is triggered upon any of the following errors: CRYP_KxR/LR register write does not respect the correct order (refer to Section 60.4.16: CRYP key registers for details). CRYP fails to load the key shared by SAES peripheral (KMOD = 0x2). KERF must be cleared by the application software, otherwise KEYVALID cannot be set. It can be done through IPRST bit of CRYP_CR, or when a correct key writing sequence starts."]
        #[inline(always)]
        pub const fn kerf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Key error flag This read-only bit is set by hardware when key information failed to load into key registers. KERF is triggered upon any of the following errors: CRYP_KxR/LR register write does not respect the correct order (refer to Section 60.4.16: CRYP key registers for details). CRYP fails to load the key shared by SAES peripheral (KMOD = 0x2). KERF must be cleared by the application software, otherwise KEYVALID cannot be set. It can be done through IPRST bit of CRYP_CR, or when a correct key writing sequence starts."]
        #[inline(always)]
        pub fn set_kerf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Key valid flag This read-only bit is set by hardware when the key of size defined by KEYSIZE is loaded in CRYP_KxR/LR key registers. The CRYPEN bit can only be set when KEYVALID is set. In normal mode when KMOD\\[1:0\\]
is at zero, the key must be written in the key registers in the correct sequence, otherwise the KERF flag is set and KEYVALID remains cleared. When KMOD\\[1:0\\]
is different from zero, the BUSY flag is automatically set by CRYP. When the key is loaded successfully, BUSY is cleared and KEYVALID set. Upon an error, KERF is set, BUSY cleared and KEYVALID remains cleared. If set, KERF must be cleared, otherwise KEYVALID cannot be set. For further information on key loading, refer to Section 60.4.16: CRYP key registers."]
        #[inline(always)]
        pub const fn keyvalid(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Key valid flag This read-only bit is set by hardware when the key of size defined by KEYSIZE is loaded in CRYP_KxR/LR key registers. The CRYPEN bit can only be set when KEYVALID is set. In normal mode when KMOD\\[1:0\\]
is at zero, the key must be written in the key registers in the correct sequence, otherwise the KERF flag is set and KEYVALID remains cleared. When KMOD\\[1:0\\]
is different from zero, the BUSY flag is automatically set by CRYP. When the key is loaded successfully, BUSY is cleared and KEYVALID set. Upon an error, KERF is set, BUSY cleared and KEYVALID remains cleared. If set, KERF must be cleared, otherwise KEYVALID cannot be set. For further information on key loading, refer to Section 60.4.16: CRYP key registers."]
        #[inline(always)]
        pub fn set_keyvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("ifem", &self.ifem())
                .field("ifnf", &self.ifnf())
                .field("ofne", &self.ofne())
                .field("offu", &self.offu())
                .field("busy", &self.busy())
                .field("kerf", &self.kerf())
                .field("keyvalid", &self.keyvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ ifem: {=bool:?}, ifnf: {=bool:?}, ofne: {=bool:?}, offu: {=bool:?}, busy: {=bool:?}, kerf: {=bool:?}, keyvalid: {=bool:?} }}" , self . ifem () , self . ifnf () , self . ofne () , self . offu () , self . busy () , self . kerf () , self . keyvalid ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Kmod {
        #[doc = "Normal-key mode. Key registers are freely usable."]
        NORMAL = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Shared-key mode. If shared-key mode is properly initialized in SAES peripheral, the CRYP peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in CRYP key registers when BUSY bit is cleared and KEYVALID is set in the CRYP_SR register. Key error flag KERF is set otherwise in the CRYP_SR register."]
        SHARED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Kmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Kmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Kmod {
        #[inline(always)]
        fn from(val: u8) -> Kmod {
            Kmod::from_bits(val)
        }
    }
    impl From<Kmod> for u8 {
        #[inline(always)]
        fn from(val: Kmod) -> u8 {
            Kmod::to_bits(val)
        }
    }
}
