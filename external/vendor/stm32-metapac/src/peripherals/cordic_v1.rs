#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CORDIC co-processor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cordic {
    ptr: *mut u8,
}
unsafe impl Send for Cordic {}
unsafe impl Sync for Cordic {}
impl Cordic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Argument register."]
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Result register."]
    #[inline(always)]
    pub const fn rdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Function."]
        #[inline(always)]
        pub const fn func(&self) -> super::vals::Func {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Func::from_bits(val as u8)
        }
        #[doc = "Function."]
        #[inline(always)]
        pub fn set_func(&mut self, val: super::vals::Func) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)."]
        #[inline(always)]
        pub const fn precision(&self) -> super::vals::Precision {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Precision::from_bits(val as u8)
        }
        #[doc = "Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)."]
        #[inline(always)]
        pub fn set_precision(&mut self, val: super::vals::Precision) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Scaling factor. Input value has been multiplied by 2^(-n) before for argument. Output value will need to be multiplied by 2^n later for results."]
        #[inline(always)]
        pub const fn scale(&self) -> super::vals::Scale {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Scale::from_bits(val as u8)
        }
        #[doc = "Scaling factor. Input value has been multiplied by 2^(-n) before for argument. Output value will need to be multiplied by 2^n later for results."]
        #[inline(always)]
        pub fn set_scale(&mut self, val: super::vals::Scale) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Enable interrupt."]
        #[inline(always)]
        pub const fn ien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt."]
        #[inline(always)]
        pub fn set_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable DMA wread channel."]
        #[inline(always)]
        pub const fn dmaren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA wread channel."]
        #[inline(always)]
        pub fn set_dmaren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable DMA write channel."]
        #[inline(always)]
        pub const fn dmawen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA write channel."]
        #[inline(always)]
        pub fn set_dmawen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Number of results in the RDATA register."]
        #[inline(always)]
        pub const fn nres(&self) -> super::vals::Num {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Num::from_bits(val as u8)
        }
        #[doc = "Number of results in the RDATA register."]
        #[inline(always)]
        pub fn set_nres(&mut self, val: super::vals::Num) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Number of arguments expected by the WDATA register."]
        #[inline(always)]
        pub const fn nargs(&self) -> super::vals::Num {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Num::from_bits(val as u8)
        }
        #[doc = "Number of arguments expected by the WDATA register."]
        #[inline(always)]
        pub fn set_nargs(&mut self, val: super::vals::Num) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Width of output data."]
        #[inline(always)]
        pub const fn ressize(&self) -> super::vals::Size {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Width of output data."]
        #[inline(always)]
        pub fn set_ressize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Width of input data."]
        #[inline(always)]
        pub const fn argsize(&self) -> super::vals::Size {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Width of input data."]
        #[inline(always)]
        pub fn set_argsize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Result ready flag."]
        #[inline(always)]
        pub const fn rrdy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Result ready flag."]
        #[inline(always)]
        pub fn set_rrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("func", &self.func())
                .field("precision", &self.precision())
                .field("scale", &self.scale())
                .field("ien", &self.ien())
                .field("dmaren", &self.dmaren())
                .field("dmawen", &self.dmawen())
                .field("nres", &self.nres())
                .field("nargs", &self.nargs())
                .field("ressize", &self.ressize())
                .field("argsize", &self.argsize())
                .field("rrdy", &self.rrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr {{ func: {:?}, precision: {:?}, scale: {:?}, ien: {=bool:?}, dmaren: {=bool:?}, dmawen: {=bool:?}, nres: {:?}, nargs: {:?}, ressize: {:?}, argsize: {:?}, rrdy: {=bool:?} }}" , self . func () , self . precision () , self . scale () , self . ien () , self . dmaren () , self . dmawen () , self . nres () , self . nargs () , self . ressize () , self . argsize () , self . rrdy ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Func {
        #[doc = "Cosine function."]
        COSINE = 0x0,
        #[doc = "Sine function."]
        SINE = 0x01,
        #[doc = "Phase function."]
        PHASE = 0x02,
        #[doc = "Modulus function."]
        MODULUS = 0x03,
        #[doc = "Arctangent function."]
        ARCTANGENT = 0x04,
        #[doc = "Hyperbolic Cosine function."]
        HYPERBOLIC_COSINE = 0x05,
        #[doc = "Hyperbolic Sine function."]
        HYPERBOLIC_SINE = 0x06,
        #[doc = "Arctanh function."]
        ARCTANH = 0x07,
        #[doc = "Natural Logarithm function."]
        NATURAL_LOGARITHM = 0x08,
        #[doc = "Square Root function."]
        SQUARE_ROOT = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Func {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Func {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Func {
        #[inline(always)]
        fn from(val: u8) -> Func {
            Func::from_bits(val)
        }
    }
    impl From<Func> for u8 {
        #[inline(always)]
        fn from(val: Func) -> u8 {
            Func::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Num {
        #[doc = "1 input/output"]
        NUM1 = 0x0,
        #[doc = "2 input/output"]
        NUM2 = 0x01,
    }
    impl Num {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Num {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Num {
        #[inline(always)]
        fn from(val: u8) -> Num {
            Num::from_bits(val)
        }
    }
    impl From<Num> for u8 {
        #[inline(always)]
        fn from(val: Num) -> u8 {
            Num::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Precision {
        _RESERVED_0 = 0x0,
        #[doc = "4 iterations."]
        ITERS4 = 0x01,
        #[doc = "8 iterations."]
        ITERS8 = 0x02,
        #[doc = "12 iterations."]
        ITERS12 = 0x03,
        #[doc = "16 iterations."]
        ITERS16 = 0x04,
        #[doc = "20 iterations."]
        ITERS20 = 0x05,
        #[doc = "24 iterations."]
        ITERS24 = 0x06,
        #[doc = "28 iterations."]
        ITERS28 = 0x07,
        #[doc = "32 iterations."]
        ITERS32 = 0x08,
        #[doc = "36 iterations."]
        ITERS36 = 0x09,
        #[doc = "40 iterations."]
        ITERS40 = 0x0a,
        #[doc = "44 iterations."]
        ITERS44 = 0x0b,
        #[doc = "48 iterations."]
        ITERS48 = 0x0c,
        #[doc = "52 iterations."]
        ITERS52 = 0x0d,
        #[doc = "56 iterations."]
        ITERS56 = 0x0e,
        #[doc = "60 iterations."]
        ITERS60 = 0x0f,
    }
    impl Precision {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Precision {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Precision {
        #[inline(always)]
        fn from(val: u8) -> Precision {
            Precision::from_bits(val)
        }
    }
    impl From<Precision> for u8 {
        #[inline(always)]
        fn from(val: Precision) -> u8 {
            Precision::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scale {
        #[doc = "Argument multiplied by 1, result multiplied by 1"]
        A1_R1 = 0x0,
        #[doc = "Argument multiplied by 1/2, result multiplied by 2"]
        A1O2_R2 = 0x01,
        #[doc = "Argument multiplied by 1/4, result multiplied by 4"]
        A1O4_R4 = 0x02,
        #[doc = "Argument multiplied by 1/8, result multiplied by 8"]
        A1O8_R8 = 0x03,
        #[doc = "Argument multiplied by 1/16, result multiplied by 16"]
        A1O16_R16 = 0x04,
        #[doc = "Argument multiplied by 1/32, result multiplied by 32"]
        A1O32_R32 = 0x05,
        #[doc = "Argument multiplied by 1/64, result multiplied by 64"]
        A1O64_R64 = 0x06,
        #[doc = "Argument multiplied by 1/128, result multiplied by 128"]
        A1O128_R128 = 0x07,
    }
    impl Scale {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scale {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scale {
        #[inline(always)]
        fn from(val: u8) -> Scale {
            Scale::from_bits(val)
        }
    }
    impl From<Scale> for u8 {
        #[inline(always)]
        fn from(val: Scale) -> u8 {
            Scale::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Size {
        #[doc = "Use 32 bit input/output values."]
        BITS32 = 0x0,
        #[doc = "Use 16 bit input/output values."]
        BITS16 = 0x01,
    }
    impl Size {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Size {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Size {
        #[inline(always)]
        fn from(val: u8) -> Size {
            Size::from_bits(val)
        }
    }
    impl From<Size> for u8 {
        #[inline(always)]
        fn from(val: Size) -> u8 {
            Size::to_bits(val)
        }
    }
}
