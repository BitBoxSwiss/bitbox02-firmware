#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Advanced encryption standard hardware accelerator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aes {
    ptr: *mut u8,
}
unsafe impl Send for Aes {}
unsafe impl Sync for Aes {}
impl Aes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Data input register"]
    #[inline(always)]
    pub const fn dinr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Data output register"]
    #[inline(always)]
    pub const fn doutr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Key register"]
    #[inline(always)]
    pub const fn keyr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr.add(
                    0x10usize + ([0usize, 4usize, 8usize, 12usize, 32usize, 36usize, 40usize, 44usize][n] as usize),
                ) as _,
            )
        }
    }
    #[doc = "Initialization vector register"]
    #[inline(always)]
    pub const fn ivr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Suspend register"]
    #[inline(always)]
    pub const fn suspr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "interrupt status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "AES enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type selection"]
        #[inline(always)]
        pub const fn datatype(&self) -> super::vals::Datatype {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Datatype::from_bits(val as u8)
        }
        #[doc = "Data type selection"]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: super::vals::Datatype) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Operating mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Operating mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Chaining mode selection"]
        #[inline(always)]
        pub const fn chmod(&self) -> super::vals::Chmod {
            let mut val = 0;
            val += (((self.0 >> 5usize) & 0x03) << 0usize);
            val += (((self.0 >> 16usize) & 0x01) << 2usize);
            super::vals::Chmod::from_bits(val as u8)
        }
        #[doc = "Chaining mode selection"]
        #[inline(always)]
        pub fn set_chmod(&mut self, val: super::vals::Chmod) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32 >> 0usize) & 0x03) << 5usize);
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32 >> 2usize) & 0x01) << 16usize);
        }
        #[doc = "Enable DMA management of data input phase"]
        #[inline(always)]
        pub const fn dmainen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA management of data input phase"]
        #[inline(always)]
        pub fn set_dmainen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable DMA management of data output phase"]
        #[inline(always)]
        pub const fn dmaouten(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA management of data output phase"]
        #[inline(always)]
        pub fn set_dmaouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GCM or CCM phase selection"]
        #[inline(always)]
        pub const fn gcmph(&self) -> super::vals::Gcmph {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Gcmph::from_bits(val as u8)
        }
        #[doc = "GCM or CCM phase selection"]
        #[inline(always)]
        pub fn set_gcmph(&mut self, val: super::vals::Gcmph) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Key size selection"]
        #[inline(always)]
        pub const fn keysize(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Key size selection"]
        #[inline(always)]
        pub fn set_keysize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Number of padding bytes in last block of payload"]
        #[inline(always)]
        pub const fn npblb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of padding bytes in last block of payload"]
        #[inline(always)]
        pub fn set_npblb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Key mode selection"]
        #[inline(always)]
        pub const fn kmod(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Key mode selection"]
        #[inline(always)]
        pub fn set_kmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "AES peripheral software reset"]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AES peripheral software reset"]
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
                .field("en", &self.en())
                .field("datatype", &self.datatype())
                .field("mode", &self.mode())
                .field("chmod", &self.chmod())
                .field("dmainen", &self.dmainen())
                .field("dmaouten", &self.dmaouten())
                .field("gcmph", &self.gcmph())
                .field("keysize", &self.keysize())
                .field("npblb", &self.npblb())
                .field("kmod", &self.kmod())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, datatype: {:?}, mode: {:?}, chmod: {:?}, dmainen: {=bool:?}, dmaouten: {=bool:?}, gcmph: {:?}, keysize: {=bool:?}, npblb: {=u8:?}, kmod: {=u8:?}, iprst: {=bool:?} }}" , self . en () , self . datatype () , self . mode () , self . chmod () , self . dmainen () , self . dmaouten () , self . gcmph () , self . keysize () , self . npblb () , self . kmod () , self . iprst ())
        }
    }
    #[doc = "Interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Computation complete flag clear"]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag clear"]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag clear"]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag clear"]
        #[inline(always)]
        pub fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag clear"]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag clear"]
        #[inline(always)]
        pub fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif()
            )
        }
    }
    #[doc = "Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Computation complete flag interrupt enable"]
        #[inline(always)]
        pub const fn ccfie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag interrupt enable"]
        #[inline(always)]
        pub fn set_ccfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt enable"]
        #[inline(always)]
        pub const fn rweie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt enable"]
        #[inline(always)]
        pub fn set_rweie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt enable"]
        #[inline(always)]
        pub const fn keie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt enable"]
        #[inline(always)]
        pub fn set_keie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("ccfie", &self.ccfie())
                .field("rweie", &self.rweie())
                .field("keie", &self.keie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ ccfie: {=bool:?}, rweie: {=bool:?}, keie: {=bool:?} }}",
                self.ccfie(),
                self.rweie(),
                self.keie()
            )
        }
    }
    #[doc = "Interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag"]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag"]
        #[inline(always)]
        pub fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag"]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag"]
        #[inline(always)]
        pub fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif()
            )
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read error flag"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read error flag"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write error flag"]
        #[inline(always)]
        pub const fn wrerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write error flag"]
        #[inline(always)]
        pub fn set_wrerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Key valid flag"]
        #[inline(always)]
        pub const fn keyvalid(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Key valid flag"]
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
                .field("ccf", &self.ccf())
                .field("rderr", &self.rderr())
                .field("wrerr", &self.wrerr())
                .field("busy", &self.busy())
                .field("keyvalid", &self.keyvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ ccf: {=bool:?}, rderr: {=bool:?}, wrerr: {=bool:?}, busy: {=bool:?}, keyvalid: {=bool:?} }}",
                self.ccf(),
                self.rderr(),
                self.wrerr(),
                self.busy(),
                self.keyvalid()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chmod {
        #[doc = "Electronic codebook"]
        ECB = 0x0,
        #[doc = "Cipher-block chaining"]
        CBC = 0x01,
        #[doc = "Counter mode"]
        CTR = 0x02,
        #[doc = "Galois counter mode and Galois message authentication code"]
        GCM_GMAC = 0x03,
        #[doc = "Counter with CBC-MAC"]
        CCM = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Chmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chmod {
        #[inline(always)]
        fn from(val: u8) -> Chmod {
            Chmod::from_bits(val)
        }
    }
    impl From<Chmod> for u8 {
        #[inline(always)]
        fn from(val: Chmod) -> u8 {
            Chmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datatype {
        #[doc = "Word"]
        NONE = 0x0,
        #[doc = "Half-word (16-bit)"]
        HALF_WORD = 0x01,
        #[doc = "Byte (8-bit)"]
        BYTE = 0x02,
        #[doc = "Bit"]
        BIT = 0x03,
    }
    impl Datatype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datatype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datatype {
        #[inline(always)]
        fn from(val: u8) -> Datatype {
            Datatype::from_bits(val)
        }
    }
    impl From<Datatype> for u8 {
        #[inline(always)]
        fn from(val: Datatype) -> u8 {
            Datatype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gcmph {
        #[doc = "Init phase"]
        INIT_PHASE = 0x0,
        #[doc = "Header phase"]
        HEADER_PHASE = 0x01,
        #[doc = "Payload phase"]
        PAYLOAD_PHASE = 0x02,
        #[doc = "Final phase"]
        FINAL_PHASE = 0x03,
    }
    impl Gcmph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gcmph {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gcmph {
        #[inline(always)]
        fn from(val: u8) -> Gcmph {
            Gcmph::from_bits(val)
        }
    }
    impl From<Gcmph> for u8 {
        #[inline(always)]
        fn from(val: Gcmph) -> u8 {
            Gcmph::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Encryption"]
        MODE1 = 0x0,
        #[doc = "Key derivation (or key preparation for ECB/CBC decryption)"]
        MODE2 = 0x01,
        #[doc = "Decryption"]
        MODE3 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
