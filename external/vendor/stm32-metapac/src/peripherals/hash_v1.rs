#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hash processor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hash {
    ptr: *mut u8,
}
unsafe impl Send for Hash {}
unsafe impl Sync for Hash {}
impl Hash {
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
    #[doc = "data input register."]
    #[inline(always)]
    pub const fn din(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "start register."]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "digest registers."]
    #[inline(always)]
    pub const fn hr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 4usize) as _) }
    }
    #[doc = "interrupt enable register."]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "context swap registers."]
    #[inline(always)]
    pub const fn csr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 51usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Initialize message digest calculation."]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Initialize message digest calculation."]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable."]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable."]
        #[inline(always)]
        pub fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Mode selection."]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mode selection."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Algorithm selection."]
        #[inline(always)]
        pub const fn algo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Algorithm selection."]
        #[inline(always)]
        pub fn set_algo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Number of words already pushed."]
        #[inline(always)]
        pub const fn nbw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of words already pushed."]
        #[inline(always)]
        pub fn set_nbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DIN not empty."]
        #[inline(always)]
        pub const fn dinne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DIN not empty."]
        #[inline(always)]
        pub fn set_dinne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Long key selection."]
        #[inline(always)]
        pub const fn lkey(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Long key selection."]
        #[inline(always)]
        pub fn set_lkey(&mut self, val: bool) {
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
                .field("init", &self.init())
                .field("dmae", &self.dmae())
                .field("datatype", &self.datatype())
                .field("mode", &self.mode())
                .field("algo", &self.algo())
                .field("nbw", &self.nbw())
                .field("dinne", &self.dinne())
                .field("lkey", &self.lkey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ init: {=bool:?}, dmae: {=bool:?}, datatype: {=u8:?}, mode: {=bool:?}, algo: {=bool:?}, nbw: {=u8:?}, dinne: {=bool:?}, lkey: {=bool:?} }}" , self . init () , self . dmae () , self . datatype () , self . mode () , self . algo () , self . nbw () , self . dinne () , self . lkey ())
        }
    }
    #[doc = "interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr(pub u32);
    impl Imr {
        #[doc = "Data input interrupt enable."]
        #[inline(always)]
        pub const fn dinie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data input interrupt enable."]
        #[inline(always)]
        pub fn set_dinie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Digest calculation completion interrupt enable."]
        #[inline(always)]
        pub const fn dcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digest calculation completion interrupt enable."]
        #[inline(always)]
        pub fn set_dcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Imr {
        #[inline(always)]
        fn default() -> Imr {
            Imr(0)
        }
    }
    impl core::fmt::Debug for Imr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr")
                .field("dinie", &self.dinie())
                .field("dcie", &self.dcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imr {{ dinie: {=bool:?}, dcie: {=bool:?} }}",
                self.dinie(),
                self.dcie()
            )
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Data input interrupt status."]
        #[inline(always)]
        pub const fn dinis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data input interrupt status."]
        #[inline(always)]
        pub fn set_dinis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Digest calculation completion interrupt status."]
        #[inline(always)]
        pub const fn dcis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digest calculation completion interrupt status."]
        #[inline(always)]
        pub fn set_dcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA Status."]
        #[inline(always)]
        pub const fn dmas(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Status."]
        #[inline(always)]
        pub fn set_dmas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("dinis", &self.dinis())
                .field("dcis", &self.dcis())
                .field("dmas", &self.dmas())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ dinis: {=bool:?}, dcis: {=bool:?}, dmas: {=bool:?}, busy: {=bool:?} }}",
                self.dinis(),
                self.dcis(),
                self.dmas(),
                self.busy()
            )
        }
    }
    #[doc = "start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u32);
    impl Str {
        #[doc = "Number of valid bits in the last word of the message."]
        #[inline(always)]
        pub const fn nblw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of valid bits in the last word of the message."]
        #[inline(always)]
        pub fn set_nblw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Digest calculation."]
        #[inline(always)]
        pub const fn dcal(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Digest calculation."]
        #[inline(always)]
        pub fn set_dcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Str {
        #[inline(always)]
        fn default() -> Str {
            Str(0)
        }
    }
    impl core::fmt::Debug for Str {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Str")
                .field("nblw", &self.nblw())
                .field("dcal", &self.dcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Str {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Str {{ nblw: {=u8:?}, dcal: {=bool:?} }}", self.nblw(), self.dcal())
        }
    }
}
