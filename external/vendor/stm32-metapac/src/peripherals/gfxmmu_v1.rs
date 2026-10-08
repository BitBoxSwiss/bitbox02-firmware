#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GFXMMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfxmmu {
    ptr: *mut u8,
}
unsafe impl Send for Gfxmmu {}
unsafe impl Sync for Gfxmmu {}
impl Gfxmmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GFXMMU configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GFXMMU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GFXMMU flag clear register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GFXMMU default value register."]
    #[inline(always)]
    pub const fn dvr(self) -> crate::common::Reg<regs::Dvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GFXMMU buffer 0 configuration register."]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "GFXMMU LUT entry 0 low."]
    #[inline(always)]
    pub const fn lutl(self, n: usize) -> crate::common::Reg<regs::Lutl, crate::common::RW> {
        assert!(n < 1024usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 8usize) as _) }
    }
    #[doc = "GFXMMU LUT entry 0 high."]
    #[inline(always)]
    pub const fn luth(self, n: usize) -> crate::common::Reg<regs::Luth, crate::common::RW> {
        assert!(n < 1024usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "GFXMMU buffer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Physical buffer offset. Offset of the physical buffer."]
        #[inline(always)]
        pub const fn pbo(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Physical buffer offset. Offset of the physical buffer."]
        #[inline(always)]
        pub fn set_pbo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 4usize)) | (((val as u32) & 0x0007_ffff) << 4usize);
        }
        #[doc = "Physical buffer base address. Base address MSB of the physical buffer."]
        #[inline(always)]
        pub const fn pbba(&self) -> u16 {
            let val = (self.0 >> 23usize) & 0x01ff;
            val as u16
        }
        #[doc = "Physical buffer base address. Base address MSB of the physical buffer."]
        #[inline(always)]
        pub fn set_pbba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("pbo", &self.pbo())
                .field("pbba", &self.pbba())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bcr {{ pbo: {=u32:?}, pbba: {=u16:?} }}", self.pbo(), self.pbba())
        }
    }
    #[doc = "GFXMMU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Buffer overflow interrupt enable. This bit enables the buffer 0 overflow interrupt."]
        #[inline(always)]
        pub const fn boie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer overflow interrupt enable. This bit enables the buffer 0 overflow interrupt."]
        #[inline(always)]
        pub fn set_boie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AHB master error interrupt enable. This bit enables the AHB master error interrupt."]
        #[inline(always)]
        pub const fn ameie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AHB master error interrupt enable. This bit enables the AHB master error interrupt."]
        #[inline(always)]
        pub fn set_ameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "192 Block mode. This bit defines the number of blocks per line."]
        #[inline(always)]
        pub const fn bm(&self, n: usize) -> super::vals::Bm192 {
            assert!(n < 1usize);
            let offs = 6usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bm192::from_bits(val as u8)
        }
        #[doc = "192 Block mode. This bit defines the number of blocks per line."]
        #[inline(always)]
        pub fn set_bm(&mut self, n: usize, val: super::vals::Bm192) {
            assert!(n < 1usize);
            let offs = 6usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
                .field("boie[0]", &self.boie(0usize))
                .field("boie[1]", &self.boie(1usize))
                .field("boie[2]", &self.boie(2usize))
                .field("boie[3]", &self.boie(3usize))
                .field("ameie", &self.ameie())
                .field("bm[0]", &self.bm(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ boie[0]: {=bool:?}, boie[1]: {=bool:?}, boie[2]: {=bool:?}, boie[3]: {=bool:?}, ameie: {=bool:?}, bm[0]: {:?} }}" , self . boie (0usize) , self . boie (1usize) , self . boie (2usize) , self . boie (3usize) , self . ameie () , self . bm (0usize))
        }
    }
    #[doc = "GFXMMU default value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dvr(pub u32);
    impl Dvr {
        #[doc = "Default value. This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped."]
        #[inline(always)]
        pub const fn dv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Default value. This field indicates the default 32-bit value which is returned when a master accesses a virtual memory location not physically mapped."]
        #[inline(always)]
        pub fn set_dv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dvr {
        #[inline(always)]
        fn default() -> Dvr {
            Dvr(0)
        }
    }
    impl core::fmt::Debug for Dvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dvr").field("dv", &self.dv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dvr {{ dv: {=u32:?} }}", self.dv())
        }
    }
    #[doc = "GFXMMU flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Clear buffer overflow flag. Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register."]
        #[inline(always)]
        pub const fn cbof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear buffer overflow flag. Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register."]
        #[inline(always)]
        pub fn set_cbof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear AHB master error flag. Writing 1 clears the AHB master error flag in the GFXMMU_SR register."]
        #[inline(always)]
        pub const fn camef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear AHB master error flag. Writing 1 clears the AHB master error flag in the GFXMMU_SR register."]
        #[inline(always)]
        pub fn set_camef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("cbof[0]", &self.cbof(0usize))
                .field("cbof[1]", &self.cbof(1usize))
                .field("cbof[2]", &self.cbof(2usize))
                .field("cbof[3]", &self.cbof(3usize))
                .field("camef", &self.camef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcr {{ cbof[0]: {=bool:?}, cbof[1]: {=bool:?}, cbof[2]: {=bool:?}, cbof[3]: {=bool:?}, camef: {=bool:?} }}" , self . cbof (0usize) , self . cbof (1usize) , self . cbof (2usize) , self . cbof (3usize) , self . camef ())
        }
    }
    #[doc = "GFXMMU LUT entry high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Luth(pub u32);
    impl Luth {
        #[doc = "Line offset. Line offset of line number x (i.e. offset of block 0 of line x)."]
        #[inline(always)]
        pub const fn lo(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Line offset. Line offset of line number x (i.e. offset of block 0 of line x)."]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 4usize)) | (((val as u32) & 0x0003_ffff) << 4usize);
        }
    }
    impl Default for Luth {
        #[inline(always)]
        fn default() -> Luth {
            Luth(0)
        }
    }
    impl core::fmt::Debug for Luth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Luth").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Luth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Luth {{ lo: {=u32:?} }}", self.lo())
        }
    }
    #[doc = "GFXMMU LUT entry low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lutl(pub u32);
    impl Lutl {
        #[doc = "Line enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "First Valid Block. Number of the first valid block of line number x."]
        #[inline(always)]
        pub const fn fvb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "First Valid Block. Number of the first valid block of line number x."]
        #[inline(always)]
        pub fn set_fvb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Last Valid Block. Number of the last valid block of line number X."]
        #[inline(always)]
        pub const fn lvb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Last Valid Block. Number of the last valid block of line number X."]
        #[inline(always)]
        pub fn set_lvb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lutl {
        #[inline(always)]
        fn default() -> Lutl {
            Lutl(0)
        }
    }
    impl core::fmt::Debug for Lutl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lutl")
                .field("en", &self.en())
                .field("fvb", &self.fvb())
                .field("lvb", &self.lvb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lutl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lutl {{ en: {=bool:?}, fvb: {=u8:?}, lvb: {=u8:?} }}",
                self.en(),
                self.fvb(),
                self.lvb()
            )
        }
    }
    #[doc = "GFXMMU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Buffer overflow flag. This bit is set when an overflow occurs during the offset calculation of the buffer 0. It is cleared by writing 1 to CB0OF."]
        #[inline(always)]
        pub const fn bof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer overflow flag. This bit is set when an overflow occurs during the offset calculation of the buffer 0. It is cleared by writing 1 to CB0OF."]
        #[inline(always)]
        pub fn set_bof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AHB master error flag. This bit is set when an AHB error happens during a transaction. It is cleared by writing 1 to CAMEF."]
        #[inline(always)]
        pub const fn amef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AHB master error flag. This bit is set when an AHB error happens during a transaction. It is cleared by writing 1 to CAMEF."]
        #[inline(always)]
        pub fn set_amef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("bof[0]", &self.bof(0usize))
                .field("bof[1]", &self.bof(1usize))
                .field("bof[2]", &self.bof(2usize))
                .field("bof[3]", &self.bof(3usize))
                .field("amef", &self.amef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ bof[0]: {=bool:?}, bof[1]: {=bool:?}, bof[2]: {=bool:?}, bof[3]: {=bool:?}, amef: {=bool:?} }}",
                self.bof(0usize),
                self.bof(1usize),
                self.bof(2usize),
                self.bof(3usize),
                self.amef()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bm192 {
        #[doc = "256 blocks per line."]
        _256BLOCKS_PER_LINE = 0x0,
        #[doc = "192 blocks per line."]
        _192BLOCKS_PER_LINE = 0x01,
    }
    impl Bm192 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bm192 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bm192 {
        #[inline(always)]
        fn from(val: u8) -> Bm192 {
            Bm192::from_bits(val)
        }
    }
    impl From<Bm192> for u8 {
        #[inline(always)]
        fn from(val: Bm192) -> u8 {
            Bm192::to_bits(val)
        }
    }
}
