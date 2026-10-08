#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose I/O"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port configuration register low (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn cr(self, n: usize) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Port input data register (GPIOn_IDR)"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port output data register (GPIOn_ODR)"]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port bit reset register (GPIOn_BRR)"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Port bit reset register (GPIOn_BRR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "Reset bit"]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit"]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
                .field("br[0]", &self.br(0usize))
                .field("br[1]", &self.br(1usize))
                .field("br[2]", &self.br(2usize))
                .field("br[3]", &self.br(3usize))
                .field("br[4]", &self.br(4usize))
                .field("br[5]", &self.br(5usize))
                .field("br[6]", &self.br(6usize))
                .field("br[7]", &self.br(7usize))
                .field("br[8]", &self.br(8usize))
                .field("br[9]", &self.br(9usize))
                .field("br[10]", &self.br(10usize))
                .field("br[11]", &self.br(11usize))
                .field("br[12]", &self.br(12usize))
                .field("br[13]", &self.br(13usize))
                .field("br[14]", &self.br(14usize))
                .field("br[15]", &self.br(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Brr {{ br[0]: {=bool:?}, br[1]: {=bool:?}, br[2]: {=bool:?}, br[3]: {=bool:?}, br[4]: {=bool:?}, br[5]: {=bool:?}, br[6]: {=bool:?}, br[7]: {=bool:?}, br[8]: {=bool:?}, br[9]: {=bool:?}, br[10]: {=bool:?}, br[11]: {=bool:?}, br[12]: {=bool:?}, br[13]: {=bool:?}, br[14]: {=bool:?}, br[15]: {=bool:?} }}" , self . br (0usize) , self . br (1usize) , self . br (2usize) , self . br (3usize) , self . br (4usize) , self . br (5usize) , self . br (6usize) , self . br (7usize) , self . br (8usize) , self . br (9usize) , self . br (10usize) , self . br (11usize) , self . br (12usize) , self . br (13usize) , self . br (14usize) , self . br (15usize))
        }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "Set bit"]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit"]
        #[inline(always)]
        pub fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit"]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit"]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bsrr {
        #[inline(always)]
        fn default() -> Bsrr {
            Bsrr(0)
        }
    }
    impl core::fmt::Debug for Bsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsrr")
                .field("bs[0]", &self.bs(0usize))
                .field("bs[1]", &self.bs(1usize))
                .field("bs[2]", &self.bs(2usize))
                .field("bs[3]", &self.bs(3usize))
                .field("bs[4]", &self.bs(4usize))
                .field("bs[5]", &self.bs(5usize))
                .field("bs[6]", &self.bs(6usize))
                .field("bs[7]", &self.bs(7usize))
                .field("bs[8]", &self.bs(8usize))
                .field("bs[9]", &self.bs(9usize))
                .field("bs[10]", &self.bs(10usize))
                .field("bs[11]", &self.bs(11usize))
                .field("bs[12]", &self.bs(12usize))
                .field("bs[13]", &self.bs(13usize))
                .field("bs[14]", &self.bs(14usize))
                .field("bs[15]", &self.bs(15usize))
                .field("br[0]", &self.br(0usize))
                .field("br[1]", &self.br(1usize))
                .field("br[2]", &self.br(2usize))
                .field("br[3]", &self.br(3usize))
                .field("br[4]", &self.br(4usize))
                .field("br[5]", &self.br(5usize))
                .field("br[6]", &self.br(6usize))
                .field("br[7]", &self.br(7usize))
                .field("br[8]", &self.br(8usize))
                .field("br[9]", &self.br(9usize))
                .field("br[10]", &self.br(10usize))
                .field("br[11]", &self.br(11usize))
                .field("br[12]", &self.br(12usize))
                .field("br[13]", &self.br(13usize))
                .field("br[14]", &self.br(14usize))
                .field("br[15]", &self.br(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bsrr {{ bs[0]: {=bool:?}, bs[1]: {=bool:?}, bs[2]: {=bool:?}, bs[3]: {=bool:?}, bs[4]: {=bool:?}, bs[5]: {=bool:?}, bs[6]: {=bool:?}, bs[7]: {=bool:?}, bs[8]: {=bool:?}, bs[9]: {=bool:?}, bs[10]: {=bool:?}, bs[11]: {=bool:?}, bs[12]: {=bool:?}, bs[13]: {=bool:?}, bs[14]: {=bool:?}, bs[15]: {=bool:?}, br[0]: {=bool:?}, br[1]: {=bool:?}, br[2]: {=bool:?}, br[3]: {=bool:?}, br[4]: {=bool:?}, br[5]: {=bool:?}, br[6]: {=bool:?}, br[7]: {=bool:?}, br[8]: {=bool:?}, br[9]: {=bool:?}, br[10]: {=bool:?}, br[11]: {=bool:?}, br[12]: {=bool:?}, br[13]: {=bool:?}, br[14]: {=bool:?}, br[15]: {=bool:?} }}" , self . bs (0usize) , self . bs (1usize) , self . bs (2usize) , self . bs (3usize) , self . bs (4usize) , self . bs (5usize) , self . bs (6usize) , self . bs (7usize) , self . bs (8usize) , self . bs (9usize) , self . bs (10usize) , self . bs (11usize) , self . bs (12usize) , self . bs (13usize) , self . bs (14usize) , self . bs (15usize) , self . br (0usize) , self . br (1usize) , self . br (2usize) , self . br (3usize) , self . br (4usize) , self . br (5usize) , self . br (6usize) , self . br (7usize) , self . br (8usize) , self . br (9usize) , self . br (10usize) , self . br (11usize) , self . br (12usize) , self . br (13usize) , self . br (14usize) , self . br (15usize))
        }
    }
    #[doc = "Port configuration register (GPIOn_CRx)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Port n mode bits"]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n mode bits"]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n configuration bits, for input mode"]
        #[inline(always)]
        pub const fn cnf_in(&self, n: usize) -> super::vals::CnfIn {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CnfIn::from_bits(val as u8)
        }
        #[doc = "Port n configuration bits, for input mode"]
        #[inline(always)]
        pub fn set_cnf_in(&mut self, n: usize, val: super::vals::CnfIn) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n configuration bits, for output mode"]
        #[inline(always)]
        pub const fn cnf_out(&self, n: usize) -> super::vals::CnfOut {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CnfOut::from_bits(val as u8)
        }
        #[doc = "Port n configuration bits, for output mode"]
        #[inline(always)]
        pub fn set_cnf_out(&mut self, n: usize, val: super::vals::CnfOut) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
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
                .field("mode[0]", &self.mode(0usize))
                .field("mode[1]", &self.mode(1usize))
                .field("mode[2]", &self.mode(2usize))
                .field("mode[3]", &self.mode(3usize))
                .field("mode[4]", &self.mode(4usize))
                .field("mode[5]", &self.mode(5usize))
                .field("mode[6]", &self.mode(6usize))
                .field("mode[7]", &self.mode(7usize))
                .field("cnf_in[0]", &self.cnf_in(0usize))
                .field("cnf_in[1]", &self.cnf_in(1usize))
                .field("cnf_in[2]", &self.cnf_in(2usize))
                .field("cnf_in[3]", &self.cnf_in(3usize))
                .field("cnf_in[4]", &self.cnf_in(4usize))
                .field("cnf_in[5]", &self.cnf_in(5usize))
                .field("cnf_in[6]", &self.cnf_in(6usize))
                .field("cnf_in[7]", &self.cnf_in(7usize))
                .field("cnf_out[0]", &self.cnf_out(0usize))
                .field("cnf_out[1]", &self.cnf_out(1usize))
                .field("cnf_out[2]", &self.cnf_out(2usize))
                .field("cnf_out[3]", &self.cnf_out(3usize))
                .field("cnf_out[4]", &self.cnf_out(4usize))
                .field("cnf_out[5]", &self.cnf_out(5usize))
                .field("cnf_out[6]", &self.cnf_out(6usize))
                .field("cnf_out[7]", &self.cnf_out(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ mode[0]: {:?}, mode[1]: {:?}, mode[2]: {:?}, mode[3]: {:?}, mode[4]: {:?}, mode[5]: {:?}, mode[6]: {:?}, mode[7]: {:?}, cnf_in[0]: {:?}, cnf_in[1]: {:?}, cnf_in[2]: {:?}, cnf_in[3]: {:?}, cnf_in[4]: {:?}, cnf_in[5]: {:?}, cnf_in[6]: {:?}, cnf_in[7]: {:?}, cnf_out[0]: {:?}, cnf_out[1]: {:?}, cnf_out[2]: {:?}, cnf_out[3]: {:?}, cnf_out[4]: {:?}, cnf_out[5]: {:?}, cnf_out[6]: {:?}, cnf_out[7]: {:?} }}" , self . mode (0usize) , self . mode (1usize) , self . mode (2usize) , self . mode (3usize) , self . mode (4usize) , self . mode (5usize) , self . mode (6usize) , self . mode (7usize) , self . cnf_in (0usize) , self . cnf_in (1usize) , self . cnf_in (2usize) , self . cnf_in (3usize) , self . cnf_in (4usize) , self . cnf_in (5usize) , self . cnf_in (6usize) , self . cnf_in (7usize) , self . cnf_out (0usize) , self . cnf_out (1usize) , self . cnf_out (2usize) , self . cnf_out (3usize) , self . cnf_out (4usize) , self . cnf_out (5usize) , self . cnf_out (6usize) , self . cnf_out (7usize))
        }
    }
    #[doc = "Port input data register (GPIOn_IDR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idr(pub u32);
    impl Idr {
        #[doc = "Port input data"]
        #[inline(always)]
        pub const fn idr(&self, n: usize) -> super::vals::Idr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Idr::from_bits(val as u8)
        }
        #[doc = "Port input data"]
        #[inline(always)]
        pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Idr {
        #[inline(always)]
        fn default() -> Idr {
            Idr(0)
        }
    }
    impl core::fmt::Debug for Idr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idr")
                .field("idr[0]", &self.idr(0usize))
                .field("idr[1]", &self.idr(1usize))
                .field("idr[2]", &self.idr(2usize))
                .field("idr[3]", &self.idr(3usize))
                .field("idr[4]", &self.idr(4usize))
                .field("idr[5]", &self.idr(5usize))
                .field("idr[6]", &self.idr(6usize))
                .field("idr[7]", &self.idr(7usize))
                .field("idr[8]", &self.idr(8usize))
                .field("idr[9]", &self.idr(9usize))
                .field("idr[10]", &self.idr(10usize))
                .field("idr[11]", &self.idr(11usize))
                .field("idr[12]", &self.idr(12usize))
                .field("idr[13]", &self.idr(13usize))
                .field("idr[14]", &self.idr(14usize))
                .field("idr[15]", &self.idr(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Idr {{ idr[0]: {:?}, idr[1]: {:?}, idr[2]: {:?}, idr[3]: {:?}, idr[4]: {:?}, idr[5]: {:?}, idr[6]: {:?}, idr[7]: {:?}, idr[8]: {:?}, idr[9]: {:?}, idr[10]: {:?}, idr[11]: {:?}, idr[12]: {:?}, idr[13]: {:?}, idr[14]: {:?}, idr[15]: {:?} }}" , self . idr (0usize) , self . idr (1usize) , self . idr (2usize) , self . idr (3usize) , self . idr (4usize) , self . idr (5usize) , self . idr (6usize) , self . idr (7usize) , self . idr (8usize) , self . idr (9usize) , self . idr (10usize) , self . idr (11usize) , self . idr (12usize) , self . idr (13usize) , self . idr (14usize) , self . idr (15usize))
        }
    }
    #[doc = "Port configuration lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckr(pub u32);
    impl Lckr {
        #[doc = "Port configuration locked"]
        #[inline(always)]
        pub const fn lck(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port configuration locked"]
        #[inline(always)]
        pub fn set_lck(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Port configuration lock key active"]
        #[inline(always)]
        pub const fn lckk(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port configuration lock key active"]
        #[inline(always)]
        pub fn set_lckk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Lckr {
        #[inline(always)]
        fn default() -> Lckr {
            Lckr(0)
        }
    }
    impl core::fmt::Debug for Lckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lckr")
                .field("lck[0]", &self.lck(0usize))
                .field("lck[1]", &self.lck(1usize))
                .field("lck[2]", &self.lck(2usize))
                .field("lck[3]", &self.lck(3usize))
                .field("lck[4]", &self.lck(4usize))
                .field("lck[5]", &self.lck(5usize))
                .field("lck[6]", &self.lck(6usize))
                .field("lck[7]", &self.lck(7usize))
                .field("lck[8]", &self.lck(8usize))
                .field("lck[9]", &self.lck(9usize))
                .field("lck[10]", &self.lck(10usize))
                .field("lck[11]", &self.lck(11usize))
                .field("lck[12]", &self.lck(12usize))
                .field("lck[13]", &self.lck(13usize))
                .field("lck[14]", &self.lck(14usize))
                .field("lck[15]", &self.lck(15usize))
                .field("lckk", &self.lckk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lckr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lckr {{ lck[0]: {=bool:?}, lck[1]: {=bool:?}, lck[2]: {=bool:?}, lck[3]: {=bool:?}, lck[4]: {=bool:?}, lck[5]: {=bool:?}, lck[6]: {=bool:?}, lck[7]: {=bool:?}, lck[8]: {=bool:?}, lck[9]: {=bool:?}, lck[10]: {=bool:?}, lck[11]: {=bool:?}, lck[12]: {=bool:?}, lck[13]: {=bool:?}, lck[14]: {=bool:?}, lck[15]: {=bool:?}, lckk: {=bool:?} }}" , self . lck (0usize) , self . lck (1usize) , self . lck (2usize) , self . lck (3usize) , self . lck (4usize) , self . lck (5usize) , self . lck (6usize) , self . lck (7usize) , self . lck (8usize) , self . lck (9usize) , self . lck (10usize) , self . lck (11usize) , self . lck (12usize) , self . lck (13usize) , self . lck (14usize) , self . lck (15usize) , self . lckk ())
        }
    }
    #[doc = "Port output data register (GPIOn_ODR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odr(pub u32);
    impl Odr {
        #[doc = "Port output data"]
        #[inline(always)]
        pub const fn odr(&self, n: usize) -> super::vals::Odr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Odr::from_bits(val as u8)
        }
        #[doc = "Port output data"]
        #[inline(always)]
        pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Odr {
        #[inline(always)]
        fn default() -> Odr {
            Odr(0)
        }
    }
    impl core::fmt::Debug for Odr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odr")
                .field("odr[0]", &self.odr(0usize))
                .field("odr[1]", &self.odr(1usize))
                .field("odr[2]", &self.odr(2usize))
                .field("odr[3]", &self.odr(3usize))
                .field("odr[4]", &self.odr(4usize))
                .field("odr[5]", &self.odr(5usize))
                .field("odr[6]", &self.odr(6usize))
                .field("odr[7]", &self.odr(7usize))
                .field("odr[8]", &self.odr(8usize))
                .field("odr[9]", &self.odr(9usize))
                .field("odr[10]", &self.odr(10usize))
                .field("odr[11]", &self.odr(11usize))
                .field("odr[12]", &self.odr(12usize))
                .field("odr[13]", &self.odr(13usize))
                .field("odr[14]", &self.odr(14usize))
                .field("odr[15]", &self.odr(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Odr {{ odr[0]: {:?}, odr[1]: {:?}, odr[2]: {:?}, odr[3]: {:?}, odr[4]: {:?}, odr[5]: {:?}, odr[6]: {:?}, odr[7]: {:?}, odr[8]: {:?}, odr[9]: {:?}, odr[10]: {:?}, odr[11]: {:?}, odr[12]: {:?}, odr[13]: {:?}, odr[14]: {:?}, odr[15]: {:?} }}" , self . odr (0usize) , self . odr (1usize) , self . odr (2usize) , self . odr (3usize) , self . odr (4usize) , self . odr (5usize) , self . odr (6usize) , self . odr (7usize) , self . odr (8usize) , self . odr (9usize) , self . odr (10usize) , self . odr (11usize) , self . odr (12usize) , self . odr (13usize) , self . odr (14usize) , self . odr (15usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CnfIn {
        #[doc = "Analog mode"]
        ANALOG = 0x0,
        #[doc = "Floating input (reset state)"]
        FLOATING = 0x01,
        #[doc = "Input with pull-up/pull-down"]
        PULL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CnfIn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CnfIn {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CnfIn {
        #[inline(always)]
        fn from(val: u8) -> CnfIn {
            CnfIn::from_bits(val)
        }
    }
    impl From<CnfIn> for u8 {
        #[inline(always)]
        fn from(val: CnfIn) -> u8 {
            CnfIn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CnfOut {
        #[doc = "Push-Pull mode"]
        PUSH_PULL = 0x0,
        #[doc = "Open Drain-Mode"]
        OPEN_DRAIN = 0x01,
        #[doc = "Alternate Function Push-Pull Mode"]
        ALT_PUSH_PULL = 0x02,
        #[doc = "Alternate Function Open-Drain Mode"]
        ALT_OPEN_DRAIN = 0x03,
    }
    impl CnfOut {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CnfOut {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CnfOut {
        #[inline(always)]
        fn from(val: u8) -> CnfOut {
            CnfOut::from_bits(val)
        }
    }
    impl From<CnfOut> for u8 {
        #[inline(always)]
        fn from(val: CnfOut) -> u8 {
            CnfOut::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Idr {
        #[doc = "Input is logic low"]
        LOW = 0x0,
        #[doc = "Input is logic high"]
        HIGH = 0x01,
    }
    impl Idr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idr {
        #[inline(always)]
        fn from(val: u8) -> Idr {
            Idr::from_bits(val)
        }
    }
    impl From<Idr> for u8 {
        #[inline(always)]
        fn from(val: Idr) -> u8 {
            Idr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Input mode (reset state)"]
        INPUT = 0x0,
        #[doc = "Output mode 10 MHz"]
        OUTPUT10MHZ = 0x01,
        #[doc = "Output mode 2 MHz"]
        OUTPUT2MHZ = 0x02,
        #[doc = "Output mode 50 MHz"]
        OUTPUT50MHZ = 0x03,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Odr {
        #[doc = "Set output to logic low"]
        LOW = 0x0,
        #[doc = "Set output to logic high"]
        HIGH = 0x01,
    }
    impl Odr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Odr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Odr {
        #[inline(always)]
        fn from(val: u8) -> Odr {
            Odr::from_bits(val)
        }
    }
    impl From<Odr> for u8 {
        #[inline(always)]
        fn from(val: Odr) -> u8 {
            Odr::to_bits(val)
        }
    }
}
