#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmc(self) -> crate::common::Reg<regs::Pmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "Compensation cell control register"]
    #[inline(always)]
    pub const fn cmpcr(self) -> crate::common::Reg<regs::Cmpcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Compensation cell control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpcr(pub u32);
    impl Cmpcr {
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub const fn cmp_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub fn set_cmp_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "READY"]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "READY"]
        #[inline(always)]
        pub fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cmpcr {
        #[inline(always)]
        fn default() -> Cmpcr {
            Cmpcr(0)
        }
    }
    impl core::fmt::Debug for Cmpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpcr")
                .field("cmp_pd", &self.cmp_pd())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmpcr {{ cmp_pd: {=bool:?}, ready: {=bool:?} }}",
                self.cmp_pd(),
                self.ready()
            )
        }
    }
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration (x = 0 to 3)"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration (x = 0 to 3)"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
        #[doc = "Memory boot mapping"]
        #[inline(always)]
        pub const fn mem_boot(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory boot mapping"]
        #[inline(always)]
        pub fn set_mem_boot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Flash bank mode selection"]
        #[inline(always)]
        pub const fn fb_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash bank mode selection"]
        #[inline(always)]
        pub fn set_fb_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC memory mapping swap"]
        #[inline(always)]
        pub const fn swp_fmc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "FMC memory mapping swap"]
        #[inline(always)]
        pub fn set_swp_fmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    impl core::fmt::Debug for Memrmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memrmp")
                .field("mem_boot", &self.mem_boot())
                .field("fb_mode", &self.fb_mode())
                .field("swp_fmc", &self.swp_fmc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memrmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Memrmp {{ mem_boot: {=bool:?}, fb_mode: {=bool:?}, swp_fmc: {=u8:?} }}",
                self.mem_boot(),
                self.fb_mode(),
                self.swp_fmc()
            )
        }
    }
    #[doc = "peripheral mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmc(pub u32);
    impl Pmc {
        #[doc = "I2C1_FMP I2C1 Fast Mode + Enable"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1_FMP I2C1 Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C2_FMP I2C2 Fast Mode + Enable"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2_FMP I2C2 Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I2C3_FMP I2C3 Fast Mode + Enable"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3_FMP I2C3 Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I2C4 Fast Mode + Enable"]
        #[inline(always)]
        pub const fn i2c4_fmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_i2c4_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PB6_FMP Fast Mode"]
        #[inline(always)]
        pub const fn pb6_fmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_FMP Fast Mode"]
        #[inline(always)]
        pub fn set_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PB7_FMP Fast Mode + Enable"]
        #[inline(always)]
        pub const fn pb7_fmp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_FMP Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PB8_FMP Fast Mode + Enable"]
        #[inline(always)]
        pub const fn pb8_fmp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_FMP Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fast Mode + Enable"]
        #[inline(always)]
        pub const fn pb9_fmp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode + Enable"]
        #[inline(always)]
        pub fn set_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub const fn adc1dc2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub fn set_adc1dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC2DC2"]
        #[inline(always)]
        pub const fn adc2dc2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2DC2"]
        #[inline(always)]
        pub fn set_adc2dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub const fn adc3dc2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub fn set_adc3dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Pmc {
        #[inline(always)]
        fn default() -> Pmc {
            Pmc(0)
        }
    }
    impl core::fmt::Debug for Pmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmc")
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c2_fmp", &self.i2c2_fmp())
                .field("i2c3_fmp", &self.i2c3_fmp())
                .field("i2c4_fmp", &self.i2c4_fmp())
                .field("pb6_fmp", &self.pb6_fmp())
                .field("pb7_fmp", &self.pb7_fmp())
                .field("pb8_fmp", &self.pb8_fmp())
                .field("pb9_fmp", &self.pb9_fmp())
                .field("adc1dc2", &self.adc1dc2())
                .field("adc2dc2", &self.adc2dc2())
                .field("adc3dc2", &self.adc3dc2())
                .field("mii_rmii_sel", &self.mii_rmii_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmc {{ i2c1_fmp: {=bool:?}, i2c2_fmp: {=bool:?}, i2c3_fmp: {=bool:?}, i2c4_fmp: {=bool:?}, pb6_fmp: {=bool:?}, pb7_fmp: {=bool:?}, pb8_fmp: {=bool:?}, pb9_fmp: {=bool:?}, adc1dc2: {=bool:?}, adc2dc2: {=bool:?}, adc3dc2: {=bool:?}, mii_rmii_sel: {=bool:?} }}" , self . i2c1_fmp () , self . i2c2_fmp () , self . i2c3_fmp () , self . i2c4_fmp () , self . pb6_fmp () , self . pb7_fmp () , self . pb8_fmp () , self . pb9_fmp () , self . adc1dc2 () , self . adc2dc2 () , self . adc3dc2 () , self . mii_rmii_sel ())
        }
    }
}
