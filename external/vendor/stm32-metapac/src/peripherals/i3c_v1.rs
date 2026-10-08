#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataRegs {
    ptr: *mut u8,
}
unsafe impl Send for DataRegs {}
unsafe impl Sync for DataRegs {}
impl DataRegs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I3C receive data byte register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "I3C receive data word register."]
    #[inline(always)]
    pub const fn dwr(self) -> crate::common::Reg<regs::Dwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Improved inter-integrated circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I3C message control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "I3C message control register alternate."]
    #[inline(always)]
    pub const fn cr_alternate(self) -> crate::common::Reg<regs::CrAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "I3C configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_data_regs(self) -> DataRegs {
        unsafe { DataRegs::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_data_regs(self) -> DataRegs {
        unsafe { DataRegs::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "I3C IBI payload data register."]
    #[inline(always)]
    pub const fn ibidr(self) -> crate::common::Reg<regs::Ibidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "I3C target transmit configuration register."]
    #[inline(always)]
    pub const fn tgttdr(self) -> crate::common::Reg<regs::Tgttdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "I3C status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "I3C status error register."]
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::Ser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "I3C received message register."]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "I3C event register."]
    #[inline(always)]
    pub const fn evr(self) -> crate::common::Reg<regs::Evr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "I3C interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "I3C clear event register."]
    #[inline(always)]
    pub const fn cevr(self) -> crate::common::Reg<regs::Cevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "I3C own device characteristics register."]
    #[inline(always)]
    pub const fn devr0(self) -> crate::common::Reg<regs::Devr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "I3C device 1 characteristics register."]
    #[inline(always)]
    pub const fn devr(self, n: usize) -> crate::common::Reg<regs::Devr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "I3C maximum read length register."]
    #[inline(always)]
    pub const fn maxrlr(self) -> crate::common::Reg<regs::Maxrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "I3C maximum write length register."]
    #[inline(always)]
    pub const fn maxwlr(self) -> crate::common::Reg<regs::Maxwlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "I3C timing register 0."]
    #[inline(always)]
    pub const fn timingr0(self) -> crate::common::Reg<regs::Timingr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "I3C timing register 1."]
    #[inline(always)]
    pub const fn timingr1(self) -> crate::common::Reg<regs::Timingr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "I3C timing register 2."]
    #[inline(always)]
    pub const fn timingr2(self) -> crate::common::Reg<regs::Timingr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "I3C bus characteristics register."]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "I3C device characteristics register."]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "I3C get capability register."]
    #[inline(always)]
    pub const fn getcapr(self) -> crate::common::Reg<regs::Getcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "I3C controller-role capability register."]
    #[inline(always)]
    pub const fn crcapr(self) -> crate::common::Reg<regs::Crcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "I3C get capability register."]
    #[inline(always)]
    pub const fn getmxdsr(self) -> crate::common::Reg<regs::Getmxdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "I3C extended provisioned ID register."]
    #[inline(always)]
    pub const fn epidr(self) -> crate::common::Reg<regs::Epidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
}
pub mod regs {
    #[doc = "I3C bus characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "max data speed limitation."]
        #[inline(always)]
        pub const fn bcr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "max data speed limitation."]
        #[inline(always)]
        pub fn set_bcr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "in-band interrupt (IBI) payload."]
        #[inline(always)]
        pub const fn bcr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "in-band interrupt (IBI) payload."]
        #[inline(always)]
        pub fn set_bcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "controller capable."]
        #[inline(always)]
        pub const fn bcr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "controller capable."]
        #[inline(always)]
        pub fn set_bcr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("bcr0", &self.bcr0())
                .field("bcr2", &self.bcr2())
                .field("bcr6", &self.bcr6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bcr {{ bcr0: {=bool:?}, bcr2: {=bool:?}, bcr6: {=bool:?} }}",
                self.bcr0(),
                self.bcr2(),
                self.bcr6()
            )
        }
    }
    #[doc = "I3C clear event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cevr(pub u32);
    impl Cevr {
        #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn cfcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_cfcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn crxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_crxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "clear error flag (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "clear error flag (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "clear IBI request flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn cibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "clear IBI request flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_cibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "clear IBI end flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "clear IBI end flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "clear controller-role request flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn ccrf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "clear controller-role request flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_ccrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "clear controller-role update flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn ccrupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "clear controller-role update flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_ccrupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "clear hot-join flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn chjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "clear hot-join flag (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_chjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "clear wakeup flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cwkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "clear wakeup flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cwkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cgetf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cgetf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cstaf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cstaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cdaupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cdaupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cmwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cmwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cmrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cmrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "clear reset pattern flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn crstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "clear reset pattern flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_crstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn casupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_casupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cintupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cintupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cdeff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cdeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn cgrpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_cgrpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cevr {
        #[inline(always)]
        fn default() -> Cevr {
            Cevr(0)
        }
    }
    impl core::fmt::Debug for Cevr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cevr")
                .field("cfcf", &self.cfcf())
                .field("crxtgtendf", &self.crxtgtendf())
                .field("cerrf", &self.cerrf())
                .field("cibif", &self.cibif())
                .field("cibiendf", &self.cibiendf())
                .field("ccrf", &self.ccrf())
                .field("ccrupdf", &self.ccrupdf())
                .field("chjf", &self.chjf())
                .field("cwkpf", &self.cwkpf())
                .field("cgetf", &self.cgetf())
                .field("cstaf", &self.cstaf())
                .field("cdaupdf", &self.cdaupdf())
                .field("cmwlupdf", &self.cmwlupdf())
                .field("cmrlupdf", &self.cmrlupdf())
                .field("crstf", &self.crstf())
                .field("casupdf", &self.casupdf())
                .field("cintupdf", &self.cintupdf())
                .field("cdeff", &self.cdeff())
                .field("cgrpf", &self.cgrpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cevr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cevr {{ cfcf: {=bool:?}, crxtgtendf: {=bool:?}, cerrf: {=bool:?}, cibif: {=bool:?}, cibiendf: {=bool:?}, ccrf: {=bool:?}, ccrupdf: {=bool:?}, chjf: {=bool:?}, cwkpf: {=bool:?}, cgetf: {=bool:?}, cstaf: {=bool:?}, cdaupdf: {=bool:?}, cmwlupdf: {=bool:?}, cmrlupdf: {=bool:?}, crstf: {=bool:?}, casupdf: {=bool:?}, cintupdf: {=bool:?}, cdeff: {=bool:?}, cgrpf: {=bool:?} }}" , self . cfcf () , self . crxtgtendf () , self . cerrf () , self . cibif () , self . cibiendf () , self . ccrf () , self . ccrupdf () , self . chjf () , self . cwkpf () , self . cgetf () , self . cstaf () , self . cdaupdf () , self . cmwlupdf () , self . cmrlupdf () , self . crstf () , self . casupdf () , self . cintupdf () , self . cdeff () , self . cgrpf ())
        }
    }
    #[doc = "I3C configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
        #[inline(always)]
        pub const fn crinit(&self) -> super::vals::Crinit {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Crinit::from_bits(val as u8)
        }
        #[doc = "initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
        #[inline(always)]
        pub fn set_crinit(&mut self, val: super::vals::Crinit) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
        #[inline(always)]
        pub const fn noarbh(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
        #[inline(always)]
        pub fn set_noarbh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
        #[inline(always)]
        pub const fn rstptrn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
        #[inline(always)]
        pub fn set_rstptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
        #[inline(always)]
        pub const fn exitptrn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
        #[inline(always)]
        pub fn set_exitptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
        #[inline(always)]
        pub const fn hksdaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
        #[inline(always)]
        pub fn set_hksdaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
        #[inline(always)]
        pub const fn hjack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
        #[inline(always)]
        pub fn set_hjack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
        #[inline(always)]
        pub const fn rxflush(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
        #[inline(always)]
        pub fn set_rxflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
        #[inline(always)]
        pub const fn rxthres(&self) -> super::vals::Thres {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Thres::from_bits(val as u8)
        }
        #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
        #[inline(always)]
        pub fn set_rxthres(&mut self, val: super::vals::Thres) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\]
< I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
        #[inline(always)]
        pub const fn txflush(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\]
< I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
        #[inline(always)]
        pub fn set_txflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
        #[inline(always)]
        pub const fn txthres(&self) -> super::vals::Thres {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Thres::from_bits(val as u8)
        }
        #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
        #[inline(always)]
        pub fn set_txthres(&mut self, val: super::vals::Thres) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn sdmaen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_sdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
        #[inline(always)]
        pub const fn sflush(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_sflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
        #[inline(always)]
        pub const fn rmode(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
        #[inline(always)]
        pub fn set_rmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
        #[inline(always)]
        pub const fn tmode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
        #[inline(always)]
        pub fn set_tmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn cdmaen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_cdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
        #[inline(always)]
        pub const fn cflush(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
        #[inline(always)]
        pub fn set_cflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
        #[inline(always)]
        pub const fn tsfset(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
        #[inline(always)]
        pub fn set_tsfset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("en", &self.en())
                .field("crinit", &self.crinit())
                .field("noarbh", &self.noarbh())
                .field("rstptrn", &self.rstptrn())
                .field("exitptrn", &self.exitptrn())
                .field("hksdaen", &self.hksdaen())
                .field("hjack", &self.hjack())
                .field("rxdmaen", &self.rxdmaen())
                .field("rxflush", &self.rxflush())
                .field("rxthres", &self.rxthres())
                .field("txdmaen", &self.txdmaen())
                .field("txflush", &self.txflush())
                .field("txthres", &self.txthres())
                .field("sdmaen", &self.sdmaen())
                .field("sflush", &self.sflush())
                .field("rmode", &self.rmode())
                .field("tmode", &self.tmode())
                .field("cdmaen", &self.cdmaen())
                .field("cflush", &self.cflush())
                .field("tsfset", &self.tsfset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ en: {=bool:?}, crinit: {:?}, noarbh: {=bool:?}, rstptrn: {=bool:?}, exitptrn: {=bool:?}, hksdaen: {=bool:?}, hjack: {=bool:?}, rxdmaen: {=bool:?}, rxflush: {=bool:?}, rxthres: {:?}, txdmaen: {=bool:?}, txflush: {=bool:?}, txthres: {:?}, sdmaen: {=bool:?}, sflush: {=bool:?}, rmode: {=bool:?}, tmode: {=bool:?}, cdmaen: {=bool:?}, cflush: {=bool:?}, tsfset: {=bool:?} }}" , self . en () , self . crinit () , self . noarbh () , self . rstptrn () , self . exitptrn () , self . hksdaen () , self . hjack () , self . rxdmaen () , self . rxflush () , self . rxthres () , self . txdmaen () , self . txflush () , self . txthres () , self . sdmaen () , self . sflush () , self . rmode () , self . tmode () , self . cdmaen () , self . cflush () , self . tsfset ())
        }
    }
    #[doc = "I3C message control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
        #[inline(always)]
        pub const fn rnw(&self) -> super::vals::Rnw {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Rnw::from_bits(val as u8)
        }
        #[doc = "read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
        #[inline(always)]
        pub fn set_rnw(&mut self, val: super::vals::Rnw) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)."]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)."]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "message type (whatever I3C is acting as controller/target) Bits\\[26:0\\]
are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL “stuck at” recovery. Bits\\[26:0\\]
are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred private message is: {S / S+7’h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7’h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7’h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\\[6:0\\]
+ RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\\[6:0\\]
+ RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\\[15:0\\]
and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\\[2:0\\]. Others: reserved."]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "message type (whatever I3C is acting as controller/target) Bits\\[26:0\\]
are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL “stuck at” recovery. Bits\\[26:0\\]
are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred private message is: {S / S+7’h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7’h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7’h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\\[6:0\\]
+ RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\\[6:0\\]
+ RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\\[15:0\\]
and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\\[2:0\\]. Others: reserved."]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "message end type (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn mend(&self) -> super::vals::Mend {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Mend::from_bits(val as u8)
        }
        #[doc = "message end type (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_mend(&mut self, val: super::vals::Mend) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("dcnt", &self.dcnt())
                .field("rnw", &self.rnw())
                .field("add", &self.add())
                .field("mtype", &self.mtype())
                .field("mend", &self.mend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ dcnt: {=u16:?}, rnw: {:?}, add: {=u8:?}, mtype: {=u8:?}, mend: {:?} }}",
                self.dcnt(),
                self.rnw(),
                self.add(),
                self.mtype(),
                self.mend()
            )
        }
    }
    #[doc = "I3C message control register alternate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrAlternate(pub u32);
    impl CrAlternate {
        #[doc = "count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
        #[inline(always)]
        pub const fn ccc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
        #[inline(always)]
        pub fn set_ccc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "message type (when I3C is acting as controller) Bits\\[23:16\\]
(CCC\\[7:0\\]) is the emitted 8-bit CCC code If Bit\\[23\\]=CCC\\[7\\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). If Bit\\[23\\]=CCC\\[7\\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). others: reserved."]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "message type (when I3C is acting as controller) Bits\\[23:16\\]
(CCC\\[7:0\\]) is the emitted 8-bit CCC code If Bit\\[23\\]=CCC\\[7\\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). If Bit\\[23\\]=CCC\\[7\\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). others: reserved."]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "message end type (when I3C is acting as controller)."]
        #[inline(always)]
        pub const fn mend(&self) -> super::vals::Mend {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Mend::from_bits(val as u8)
        }
        #[doc = "message end type (when I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_mend(&mut self, val: super::vals::Mend) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CrAlternate {
        #[inline(always)]
        fn default() -> CrAlternate {
            CrAlternate(0)
        }
    }
    impl core::fmt::Debug for CrAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrAlternate")
                .field("dcnt", &self.dcnt())
                .field("ccc", &self.ccc())
                .field("mtype", &self.mtype())
                .field("mend", &self.mend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrAlternate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CrAlternate {{ dcnt: {=u16:?}, ccc: {=u8:?}, mtype: {=u8:?}, mend: {:?} }}",
                self.dcnt(),
                self.ccc(),
                self.mtype(),
                self.mend()
            )
        }
    }
    #[doc = "I3C controller-role capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcapr(pub u32);
    impl Crcapr {
        #[doc = "delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub const fn capdhoff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub fn set_capdhoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub const fn capgrp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub fn set_capgrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Crcapr {
        #[inline(always)]
        fn default() -> Crcapr {
            Crcapr(0)
        }
    }
    impl core::fmt::Debug for Crcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcapr")
                .field("capdhoff", &self.capdhoff())
                .field("capgrp", &self.capgrp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Crcapr {{ capdhoff: {=bool:?}, capgrp: {=bool:?} }}",
                self.capdhoff(),
                self.capgrp()
            )
        }
    }
    #[doc = "I3C device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr(pub u32);
    impl Dcr {
        #[doc = "device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register."]
        #[inline(always)]
        pub const fn dcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register."]
        #[inline(always)]
        pub fn set_dcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dcr {
        #[inline(always)]
        fn default() -> Dcr {
            Dcr(0)
        }
    }
    impl core::fmt::Debug for Dcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr").field("dcr", &self.dcr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcr {{ dcr: {=u8:?} }}", self.dcr())
        }
    }
    #[doc = "I3C device 4 characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr(pub u32);
    impl Devr {
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
        #[inline(always)]
        pub const fn ibiack(&self) -> super::vals::Ack {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Ack::from_bits(val as u8)
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: super::vals::Ack) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub const fn crack(&self) -> super::vals::Ack {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ack::from_bits(val as u8)
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: super::vals::Ack) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn ibiden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub const fn dis(&self) -> super::vals::Dis {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Dis::from_bits(val as u8)
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: super::vals::Dis) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr {
        #[inline(always)]
        fn default() -> Devr {
            Devr(0)
        }
    }
    impl core::fmt::Debug for Devr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Devr")
                .field("da", &self.da())
                .field("ibiack", &self.ibiack())
                .field("crack", &self.crack())
                .field("ibiden", &self.ibiden())
                .field("susp", &self.susp())
                .field("dis", &self.dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Devr {{ da: {=u8:?}, ibiack: {:?}, crack: {:?}, ibiden: {=bool:?}, susp: {=bool:?}, dis: {:?} }}",
                self.da(),
                self.ibiack(),
                self.crack(),
                self.ibiden(),
                self.susp(),
                self.dis()
            )
        }
    }
    #[doc = "I3C own device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr0(pub u32);
    impl Devr0 {
        #[doc = "dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
        #[inline(always)]
        pub const fn daval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
        #[inline(always)]
        pub fn set_daval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
        #[inline(always)]
        pub const fn ibien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_ibien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
        #[inline(always)]
        pub const fn cren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_cren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
        #[inline(always)]
        pub const fn hjen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_hjen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):."]
        #[inline(always)]
        pub const fn as_(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):."]
        #[inline(always)]
        pub fn set_as_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
        #[inline(always)]
        pub const fn rstact(&self) -> super::vals::Rstact {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Rstact::from_bits(val as u8)
        }
        #[doc = "reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
        #[inline(always)]
        pub fn set_rstact(&mut self, val: super::vals::Rstact) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
        #[inline(always)]
        pub const fn rstval(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
        #[inline(always)]
        pub fn set_rstval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Devr0 {
        #[inline(always)]
        fn default() -> Devr0 {
            Devr0(0)
        }
    }
    impl core::fmt::Debug for Devr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Devr0")
                .field("daval", &self.daval())
                .field("da", &self.da())
                .field("ibien", &self.ibien())
                .field("cren", &self.cren())
                .field("hjen", &self.hjen())
                .field("as_", &self.as_())
                .field("rstact", &self.rstact())
                .field("rstval", &self.rstval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Devr0 {{ daval: {=bool:?}, da: {=u8:?}, ibien: {=bool:?}, cren: {=bool:?}, hjen: {=bool:?}, as_: {=u8:?}, rstact: {:?}, rstval: {=bool:?} }}" , self . daval () , self . da () , self . ibien () , self . cren () , self . hjen () , self . as_ () , self . rstact () , self . rstval ())
        }
    }
    #[doc = "I3C receive data byte register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "8-bit received data on I3C bus."]
        #[inline(always)]
        pub const fn db(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data on I3C bus."]
        #[inline(always)]
        pub fn set_db(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("db", &self.db()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ db: {=u8:?} }}", self.db())
        }
    }
    #[doc = "I3C receive data word register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dwr(pub u32);
    impl Dwr {
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[inline(always)]
        pub const fn db(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[inline(always)]
        pub fn set_db(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Dwr {
        #[inline(always)]
        fn default() -> Dwr {
            Dwr(0)
        }
    }
    impl core::fmt::Debug for Dwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dwr")
                .field("db[0]", &self.db(0usize))
                .field("db[1]", &self.db(1usize))
                .field("db[2]", &self.db(2usize))
                .field("db[3]", &self.db(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dwr {{ db[0]: {=u8:?}, db[1]: {=u8:?}, db[2]: {=u8:?}, db[3]: {=u8:?} }}",
                self.db(0usize),
                self.db(1usize),
                self.db(2usize),
                self.db(3usize)
            )
        }
    }
    #[doc = "I3C extended provisioned ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epidr(pub u32);
    impl Epidr {
        #[doc = "4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\]
of the 48-bit provisioned ID. Note: The bits\\[11:0\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub const fn mipiid(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\]
of the 48-bit provisioned ID. Note: The bits\\[11:0\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub fn set_mipiid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\]
of the 48-bit provisioned ID. Note: The bits\\[31:16\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub const fn idtsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\]
of the 48-bit provisioned ID. Note: The bits\\[31:16\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub fn set_idtsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\]
of the 48-bit provisioned ID."]
        #[inline(always)]
        pub const fn mipimid(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x7fff;
            val as u16
        }
        #[doc = "15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\]
of the 48-bit provisioned ID."]
        #[inline(always)]
        pub fn set_mipimid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
        }
    }
    impl Default for Epidr {
        #[inline(always)]
        fn default() -> Epidr {
            Epidr(0)
        }
    }
    impl core::fmt::Debug for Epidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epidr")
                .field("mipiid", &self.mipiid())
                .field("idtsel", &self.idtsel())
                .field("mipimid", &self.mipimid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Epidr {{ mipiid: {=u8:?}, idtsel: {=bool:?}, mipimid: {=u16:?} }}",
                self.mipiid(),
                self.idtsel(),
                self.mipimid()
            )
        }
    }
    #[doc = "I3C event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evr(pub u32);
    impl Evr {
        #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the I3C_CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the I3C_CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub const fn cfef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the I3C_CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the I3C_CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub fn set_cfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub const fn txfef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub fn set_txfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to I3C_CR)."]
        #[inline(always)]
        pub const fn cfnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to I3C_CR)."]
        #[inline(always)]
        pub fn set_cfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. I3C_CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO."]
        #[inline(always)]
        pub const fn sfnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. I3C_CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO."]
        #[inline(always)]
        pub fn set_sfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to I3C_TDR or I3C_TDWR depending on I3C_CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into I3C_TDR/I3C_TDWR, it must have configured and set the TX-FIFO preload (i.e. write I3C_TGTTDR.PRELOAD)."]
        #[inline(always)]
        pub const fn txfnff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to I3C_TDR or I3C_TDWR depending on I3C_CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into I3C_TDR/I3C_TDWR, it must have configured and set the TX-FIFO preload (i.e. write I3C_TGTTDR.PRELOAD)."]
        #[inline(always)]
        pub fn set_txfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to I3C_RDR or I3C_RDWR depending on I3C_CFGR.RXTHRES)."]
        #[inline(always)]
        pub const fn rxfnef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to I3C_RDR or I3C_RDWR depending on I3C_CFGR.RXTHRES)."]
        #[inline(always)]
        pub fn set_rxfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written."]
        #[inline(always)]
        pub const fn txlastf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written."]
        #[inline(always)]
        pub fn set_txlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read."]
        #[inline(always)]
        pub const fn rxlastf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read."]
        #[inline(always)]
        pub fn set_rxlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CFCF bit."]
        #[inline(always)]
        pub const fn fcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CFCF bit."]
        #[inline(always)]
        pub fn set_fcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read I3C_SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRXTGTENDF bit."]
        #[inline(always)]
        pub const fn rxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read I3C_SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRXTGTENDF bit."]
        #[inline(always)]
        pub fn set_rxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read I3C_SER to get the error type. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CERRF bit."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read I3C_SER to get the error type. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CERRF bit."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIF bit."]
        #[inline(always)]
        pub const fn ibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIF bit."]
        #[inline(always)]
        pub fn set_ibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIENDF bit."]
        #[inline(always)]
        pub const fn ibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIENDF bit."]
        #[inline(always)]
        pub fn set_ibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRF bit."]
        #[inline(always)]
        pub const fn crf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRF bit."]
        #[inline(always)]
        pub fn set_crf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRUPDF bit."]
        #[inline(always)]
        pub const fn crupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRUPDF bit."]
        #[inline(always)]
        pub fn set_crupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CHJF bit."]
        #[inline(always)]
        pub const fn hjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CHJF bit."]
        #[inline(always)]
        pub fn set_hjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CWKPF bit."]
        #[inline(always)]
        pub const fn wkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CWKPF bit."]
        #[inline(always)]
        pub fn set_wkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGETF bit."]
        #[inline(always)]
        pub const fn getf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGETF bit."]
        #[inline(always)]
        pub fn set_getf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CSTAF bit."]
        #[inline(always)]
        pub const fn staf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CSTAF bit."]
        #[inline(always)]
        pub fn set_staf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read I3C_DEVR0.DA\\[6:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDAUPDF bit."]
        #[inline(always)]
        pub const fn daupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read I3C_DEVR0.DA\\[6:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDAUPDF bit."]
        #[inline(always)]
        pub fn set_daupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read I3C_MAXWLR.MWL\\[15:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMWLUPDF bit."]
        #[inline(always)]
        pub const fn mwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read I3C_MAXWLR.MWL\\[15:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMWLUPDF bit."]
        #[inline(always)]
        pub fn set_mwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read I3C_MAXRLR.MRL\\[15:0\\]
to get the maximum read length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMRLUPDF bit."]
        #[inline(always)]
        pub const fn mrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read I3C_MAXRLR.MRL\\[15:0\\]
to get the maximum read length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMRLUPDF bit."]
        #[inline(always)]
        pub fn set_mrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read I3C_DEVR0.RSTACT\\[1:0\\]
and I3C_DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRSTF bit."]
        #[inline(always)]
        pub const fn rstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read I3C_DEVR0.RSTACT\\[1:0\\]
and I3C_DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRSTF bit."]
        #[inline(always)]
        pub fn set_rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read I3C_DEVR0.AS\\[1:0\\]. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CASUPDF bit."]
        #[inline(always)]
        pub const fn asupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read I3C_DEVR0.AS\\[1:0\\]. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CASUPDF bit."]
        #[inline(always)]
        pub fn set_asupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively I3C_DEVR0.IBIEN, I3C_DEVR0.CREN or I3C_DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CINTUPDF bit."]
        #[inline(always)]
        pub const fn intupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively I3C_DEVR0.IBIEN, I3C_DEVR0.CREN or I3C_DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CINTUPDF bit."]
        #[inline(always)]
        pub fn set_intupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDEFF bit."]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDEFF bit."]
        #[inline(always)]
        pub fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGRPF bit."]
        #[inline(always)]
        pub const fn grpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGRPF bit."]
        #[inline(always)]
        pub fn set_grpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Evr {
        #[inline(always)]
        fn default() -> Evr {
            Evr(0)
        }
    }
    impl core::fmt::Debug for Evr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evr")
                .field("cfef", &self.cfef())
                .field("txfef", &self.txfef())
                .field("cfnff", &self.cfnff())
                .field("sfnef", &self.sfnef())
                .field("txfnff", &self.txfnff())
                .field("rxfnef", &self.rxfnef())
                .field("txlastf", &self.txlastf())
                .field("rxlastf", &self.rxlastf())
                .field("fcf", &self.fcf())
                .field("rxtgtendf", &self.rxtgtendf())
                .field("errf", &self.errf())
                .field("ibif", &self.ibif())
                .field("ibiendf", &self.ibiendf())
                .field("crf", &self.crf())
                .field("crupdf", &self.crupdf())
                .field("hjf", &self.hjf())
                .field("wkpf", &self.wkpf())
                .field("getf", &self.getf())
                .field("staf", &self.staf())
                .field("daupdf", &self.daupdf())
                .field("mwlupdf", &self.mwlupdf())
                .field("mrlupdf", &self.mrlupdf())
                .field("rstf", &self.rstf())
                .field("asupdf", &self.asupdf())
                .field("intupdf", &self.intupdf())
                .field("deff", &self.deff())
                .field("grpf", &self.grpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Evr {{ cfef: {=bool:?}, txfef: {=bool:?}, cfnff: {=bool:?}, sfnef: {=bool:?}, txfnff: {=bool:?}, rxfnef: {=bool:?}, txlastf: {=bool:?}, rxlastf: {=bool:?}, fcf: {=bool:?}, rxtgtendf: {=bool:?}, errf: {=bool:?}, ibif: {=bool:?}, ibiendf: {=bool:?}, crf: {=bool:?}, crupdf: {=bool:?}, hjf: {=bool:?}, wkpf: {=bool:?}, getf: {=bool:?}, staf: {=bool:?}, daupdf: {=bool:?}, mwlupdf: {=bool:?}, mrlupdf: {=bool:?}, rstf: {=bool:?}, asupdf: {=bool:?}, intupdf: {=bool:?}, deff: {=bool:?}, grpf: {=bool:?} }}" , self . cfef () , self . txfef () , self . cfnff () , self . sfnef () , self . txfnff () , self . rxfnef () , self . txlastf () , self . rxlastf () , self . fcf () , self . rxtgtendf () , self . errf () , self . ibif () , self . ibiendf () , self . crf () , self . crupdf () , self . hjf () , self . wkpf () , self . getf () , self . staf () , self . daupdf () , self . mwlupdf () , self . mrlupdf () , self . rstf () , self . asupdf () , self . intupdf () , self . deff () , self . grpf ())
        }
    }
    #[doc = "I3C get capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getcapr(pub u32);
    impl Getcapr {
        #[doc = "IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
        #[inline(always)]
        pub const fn cappend(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
        #[inline(always)]
        pub fn set_cappend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Getcapr {
        #[inline(always)]
        fn default() -> Getcapr {
            Getcapr(0)
        }
    }
    impl core::fmt::Debug for Getcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Getcapr").field("cappend", &self.cappend()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Getcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Getcapr {{ cappend: {=bool:?} }}", self.cappend())
        }
    }
    #[doc = "I3C get capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getmxdsr(pub u32);
    impl Getmxdsr {
        #[doc = "controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
        #[inline(always)]
        pub const fn hoffas(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
        #[inline(always)]
        pub fn set_hoffas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "GETMXDS CCC format."]
        #[inline(always)]
        pub const fn fmt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "GETMXDS CCC format."]
        #[inline(always)]
        pub fn set_fmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
        #[inline(always)]
        pub const fn rdturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
        #[inline(always)]
        pub fn set_rdturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
        #[inline(always)]
        pub const fn tsco(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
        #[inline(always)]
        pub fn set_tsco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Getmxdsr {
        #[inline(always)]
        fn default() -> Getmxdsr {
            Getmxdsr(0)
        }
    }
    impl core::fmt::Debug for Getmxdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Getmxdsr")
                .field("hoffas", &self.hoffas())
                .field("fmt", &self.fmt())
                .field("rdturn", &self.rdturn())
                .field("tsco", &self.tsco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Getmxdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Getmxdsr {{ hoffas: {=u8:?}, fmt: {=u8:?}, rdturn: {=u8:?}, tsco: {=bool:?} }}",
                self.hoffas(),
                self.fmt(),
                self.rdturn(),
                self.tsco()
            )
        }
    }
    #[doc = "I3C IBI payload data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ibidr(pub u32);
    impl Ibidr {
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
        #[inline(always)]
        pub const fn ibidb(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
        #[inline(always)]
        pub fn set_ibidb(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Ibidr {
        #[inline(always)]
        fn default() -> Ibidr {
            Ibidr(0)
        }
    }
    impl core::fmt::Debug for Ibidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ibidr")
                .field("ibidb[0]", &self.ibidb(0usize))
                .field("ibidb[1]", &self.ibidb(1usize))
                .field("ibidb[2]", &self.ibidb(2usize))
                .field("ibidb[3]", &self.ibidb(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ibidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ibidr {{ ibidb[0]: {=u8:?}, ibidb[1]: {=u8:?}, ibidb[2]: {=u8:?}, ibidb[3]: {=u8:?} }}",
                self.ibidb(0usize),
                self.ibidb(1usize),
                self.ibidb(2usize),
                self.ibidb(3usize)
            )
        }
    }
    #[doc = "I3C interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn cfnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_cfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn sfneie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_sfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn txfnfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_txfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn rxfneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_rxfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn rxtgtendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_rxtgtendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI request interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn ibiie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_ibiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn ibiendie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_ibiendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn crie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_crie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "controller-role update interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn crupdie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role update interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_crupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "hot-join interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn hjie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join interrupt enable (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_hjie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wakeup interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn wkpie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_wkpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn getie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_getie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn staie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_staie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn daupdie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_daupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn mwlupdie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_mwlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn mrlupdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_mrlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "reset pattern interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "reset pattern interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn asupdie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_asupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn intupdie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_intupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn grpie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_grpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("cfnfie", &self.cfnfie())
                .field("sfneie", &self.sfneie())
                .field("txfnfie", &self.txfnfie())
                .field("rxfneie", &self.rxfneie())
                .field("fcie", &self.fcie())
                .field("rxtgtendie", &self.rxtgtendie())
                .field("errie", &self.errie())
                .field("ibiie", &self.ibiie())
                .field("ibiendie", &self.ibiendie())
                .field("crie", &self.crie())
                .field("crupdie", &self.crupdie())
                .field("hjie", &self.hjie())
                .field("wkpie", &self.wkpie())
                .field("getie", &self.getie())
                .field("staie", &self.staie())
                .field("daupdie", &self.daupdie())
                .field("mwlupdie", &self.mwlupdie())
                .field("mrlupdie", &self.mrlupdie())
                .field("rstie", &self.rstie())
                .field("asupdie", &self.asupdie())
                .field("intupdie", &self.intupdie())
                .field("defie", &self.defie())
                .field("grpie", &self.grpie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ cfnfie: {=bool:?}, sfneie: {=bool:?}, txfnfie: {=bool:?}, rxfneie: {=bool:?}, fcie: {=bool:?}, rxtgtendie: {=bool:?}, errie: {=bool:?}, ibiie: {=bool:?}, ibiendie: {=bool:?}, crie: {=bool:?}, crupdie: {=bool:?}, hjie: {=bool:?}, wkpie: {=bool:?}, getie: {=bool:?}, staie: {=bool:?}, daupdie: {=bool:?}, mwlupdie: {=bool:?}, mrlupdie: {=bool:?}, rstie: {=bool:?}, asupdie: {=bool:?}, intupdie: {=bool:?}, defie: {=bool:?}, grpie: {=bool:?} }}" , self . cfnfie () , self . sfneie () , self . txfnfie () , self . rxfneie () , self . fcie () , self . rxtgtendie () , self . errie () , self . ibiie () , self . ibiendie () , self . crie () , self . crupdie () , self . hjie () , self . wkpie () , self . getie () , self . staie () , self . daupdie () , self . mwlupdie () , self . mrlupdie () , self . rstie () , self . asupdie () , self . intupdie () , self . defie () , self . grpie ())
        }
    }
    #[doc = "I3C maximum read length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxrlr(pub u32);
    impl Maxrlr {
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub const fn ml(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub fn set_ml(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100."]
        #[inline(always)]
        pub const fn ibip(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100."]
        #[inline(always)]
        pub fn set_ibip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Maxrlr {
        #[inline(always)]
        fn default() -> Maxrlr {
            Maxrlr(0)
        }
    }
    impl core::fmt::Debug for Maxrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maxrlr")
                .field("ml", &self.ml())
                .field("ibip", &self.ibip())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maxrlr {{ ml: {=u16:?}, ibip: {=u8:?} }}", self.ml(), self.ibip())
        }
    }
    #[doc = "I3C maximum write length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxwlr(pub u32);
    impl Maxwlr {
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub const fn ml(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub fn set_ml(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Maxwlr {
        #[inline(always)]
        fn default() -> Maxwlr {
            Maxwlr(0)
        }
    }
    impl core::fmt::Debug for Maxwlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maxwlr").field("ml", &self.ml()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxwlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maxwlr {{ ml: {=u16:?} }}", self.ml())
        }
    }
    #[doc = "I3C received message register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmr(pub u32);
    impl Rmr {
        #[doc = "IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
        #[inline(always)]
        pub const fn ibirdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
        #[inline(always)]
        pub fn set_ibirdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
        #[inline(always)]
        pub const fn rcode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
        #[inline(always)]
        pub fn set_rcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
        #[inline(always)]
        pub const fn radd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
        #[inline(always)]
        pub fn set_radd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for Rmr {
        #[inline(always)]
        fn default() -> Rmr {
            Rmr(0)
        }
    }
    impl core::fmt::Debug for Rmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rmr")
                .field("ibirdcnt", &self.ibirdcnt())
                .field("rcode", &self.rcode())
                .field("radd", &self.radd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rmr {{ ibirdcnt: {=u8:?}, rcode: {=u8:?}, radd: {=u8:?} }}",
                self.ibirdcnt(),
                self.rcode(),
                self.radd()
            )
        }
    }
    #[doc = "I3C status error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ser(pub u32);
    impl Ser {
        #[doc = "protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved."]
        #[inline(always)]
        pub const fn coderr(&self) -> super::vals::Coderr {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Coderr::from_bits(val as u8)
        }
        #[doc = "protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved."]
        #[inline(always)]
        pub fn set_coderr(&mut self, val: super::vals::Coderr) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "protocol error."]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "protocol error."]
        #[inline(always)]
        pub fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SCL stall error (when the I3C is acting as target)."]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SCL stall error (when the I3C is acting as target)."]
        #[inline(always)]
        pub fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received."]
        #[inline(always)]
        pub const fn dovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received."]
        #[inline(always)]
        pub fn set_dovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends."]
        #[inline(always)]
        pub const fn covr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends."]
        #[inline(always)]
        pub fn set_covr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer."]
        #[inline(always)]
        pub const fn anack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer."]
        #[inline(always)]
        pub fn set_anack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure."]
        #[inline(always)]
        pub const fn dnack(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure."]
        #[inline(always)]
        pub fn set_dnack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "data error (when the I3C is acting as controller)."]
        #[inline(always)]
        pub const fn derr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "data error (when the I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_derr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ser {
        #[inline(always)]
        fn default() -> Ser {
            Ser(0)
        }
    }
    impl core::fmt::Debug for Ser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ser")
                .field("coderr", &self.coderr())
                .field("perr", &self.perr())
                .field("stall", &self.stall())
                .field("dovr", &self.dovr())
                .field("covr", &self.covr())
                .field("anack", &self.anack())
                .field("dnack", &self.dnack())
                .field("derr", &self.derr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ser {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ser {{ coderr: {:?}, perr: {=bool:?}, stall: {=bool:?}, dovr: {=bool:?}, covr: {=bool:?}, anack: {=bool:?}, dnack: {=bool:?}, derr: {=bool:?} }}" , self . coderr () , self . perr () , self . stall () , self . dovr () , self . covr () , self . anack () , self . dnack () , self . derr ())
        }
    }
    #[doc = "I3C status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message."]
        #[inline(always)]
        pub const fn xdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message."]
        #[inline(always)]
        pub fn set_xdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. I3C_CR.DCNT\\[15:0\\])."]
        #[inline(always)]
        pub const fn abt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. I3C_CR.DCNT\\[15:0\\])."]
        #[inline(always)]
        pub fn set_abt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. I3C_CR) to which the I3C_SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. I3C_CR) over I3C bus. This field is reset for every new frame start."]
        #[inline(always)]
        pub const fn mid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. I3C_CR) to which the I3C_SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. I3C_CR) over I3C bus. This field is reset for every new frame start."]
        #[inline(always)]
        pub fn set_mid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("xdcnt", &self.xdcnt())
                .field("abt", &self.abt())
                .field("dir", &self.dir())
                .field("mid", &self.mid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ xdcnt: {=u16:?}, abt: {=bool:?}, dir: {:?}, mid: {=u8:?} }}",
                self.xdcnt(),
                self.abt(),
                self.dir(),
                self.mid()
            )
        }
    }
    #[doc = "I3C target transmit configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tgttdr(pub u32);
    impl Tgttdr {
        #[doc = "transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
        #[inline(always)]
        pub const fn tgttdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
        #[inline(always)]
        pub fn set_tgttdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
        #[inline(always)]
        pub fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Tgttdr {
        #[inline(always)]
        fn default() -> Tgttdr {
            Tgttdr(0)
        }
    }
    impl core::fmt::Debug for Tgttdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tgttdr")
                .field("tgttdcnt", &self.tgttdcnt())
                .field("preload", &self.preload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tgttdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tgttdr {{ tgttdcnt: {=u16:?}, preload: {=bool:?} }}",
                self.tgttdcnt(),
                self.preload()
            )
        }
    }
    #[doc = "I3C timing register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr0(pub u32);
    impl Timingr0 {
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
        #[inline(always)]
        pub const fn scll_pp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
        #[inline(always)]
        pub fn set_scll_pp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
        #[inline(always)]
        pub const fn sclh_i3c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
        #[inline(always)]
        pub fn set_sclh_i3c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
        #[inline(always)]
        pub const fn scll_od(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
        #[inline(always)]
        pub fn set_scll_od(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
        #[inline(always)]
        pub const fn sclh_i2c(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
        #[inline(always)]
        pub fn set_sclh_i2c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timingr0 {
        #[inline(always)]
        fn default() -> Timingr0 {
            Timingr0(0)
        }
    }
    impl core::fmt::Debug for Timingr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr0")
                .field("scll_pp", &self.scll_pp())
                .field("sclh_i3c", &self.sclh_i3c())
                .field("scll_od", &self.scll_od())
                .field("sclh_i2c", &self.sclh_i2c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr0 {{ scll_pp: {=u8:?}, sclh_i3c: {=u8:?}, scll_od: {=u8:?}, sclh_i2c: {=u8:?} }}",
                self.scll_pp(),
                self.sclh_i3c(),
                self.scll_od(),
                self.sclh_i2c()
            )
        }
    }
    #[doc = "I3C timing register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr1(pub u32);
    impl Timingr1 {
        #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\\[7:0\\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\\[7:0\\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 50000 x tI3CCLK."]
        #[inline(always)]
        pub const fn aval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\\[7:0\\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\\[7:0\\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 50000 x tI3CCLK."]
        #[inline(always)]
        pub fn set_aval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
        #[inline(always)]
        pub const fn asncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
        #[inline(always)]
        pub fn set_asncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\\[6:0\\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\\[6:0\\]
+ 1) x tI3CCLK."]
        #[inline(always)]
        pub const fn free(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\\[6:0\\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\\[6:0\\]
+ 1) x tI3CCLK."]
        #[inline(always)]
        pub fn set_free(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):."]
        #[inline(always)]
        pub const fn sda_hd(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):."]
        #[inline(always)]
        pub fn set_sda_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Timingr1 {
        #[inline(always)]
        fn default() -> Timingr1 {
            Timingr1(0)
        }
    }
    impl core::fmt::Debug for Timingr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr1")
                .field("aval", &self.aval())
                .field("asncr", &self.asncr())
                .field("free", &self.free())
                .field("sda_hd", &self.sda_hd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr1 {{ aval: {=u8:?}, asncr: {=u8:?}, free: {=u8:?}, sda_hd: {=bool:?} }}",
                self.aval(),
                self.asncr(),
                self.free(),
                self.sda_hd()
            )
        }
    }
    #[doc = "I3C timing register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr2(pub u32);
    impl Timingr2 {
        #[doc = "Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
        #[inline(always)]
        pub const fn stallt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
        #[inline(always)]
        pub fn set_stallt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
        #[inline(always)]
        pub const fn stalld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
        #[inline(always)]
        pub fn set_stalld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
        #[inline(always)]
        pub const fn stallc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
        #[inline(always)]
        pub fn set_stallc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
        #[inline(always)]
        pub const fn stalla(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
        #[inline(always)]
        pub fn set_stalla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK."]
        #[inline(always)]
        pub const fn stall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK."]
        #[inline(always)]
        pub fn set_stall(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Timingr2 {
        #[inline(always)]
        fn default() -> Timingr2 {
            Timingr2(0)
        }
    }
    impl core::fmt::Debug for Timingr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr2")
                .field("stallt", &self.stallt())
                .field("stalld", &self.stalld())
                .field("stallc", &self.stallc())
                .field("stalla", &self.stalla())
                .field("stall", &self.stall())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Timingr2 {{ stallt: {=bool:?}, stalld: {=bool:?}, stallc: {=bool:?}, stalla: {=bool:?}, stall: {=u8:?} }}" , self . stallt () , self . stalld () , self . stallc () , self . stalla () , self . stall ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ack {
        MUST_NACKED = 0x0,
        MUST_ACKED = 0x01,
    }
    impl Ack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ack {
        #[inline(always)]
        fn from(val: u8) -> Ack {
            Ack::from_bits(val)
        }
    }
    impl From<Ack> for u8 {
        #[inline(always)]
        fn from(val: Ack) -> u8 {
            Ack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Coderr {
        #[doc = "Transaction after sending CCC. Controller detected an illegally formatted CCC"]
        CE0 = 0x0,
        #[doc = "Monitoring error. Controller detected that transmitted data on the bus is different from expected"]
        CE1 = 0x01,
        #[doc = "No response to broadcast address. Controller detected a not acknowledged broadcast address (0b111_1110)"]
        CE2 = 0x02,
        #[doc = "Failed controller-role hand-off. Controller detected the new controller did not drive bus after controller-role hand-off"]
        CE3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Invalid broadcast address 0b111_1110 + W. Target detected an invalid broadcast address 0b111_1110 + W"]
        TE0 = 0x08,
        #[doc = "CCC code. Target detected a parity error on a CCC code via a parity check (vs. T bit)"]
        TE1 = 0x09,
        #[doc = "Write data. Target detected a parity error on a write data via a parity check (vs. T bit)"]
        TE2 = 0x0a,
        #[doc = "Assigned address during dynamic address arbitration. Target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs. PAR bit)"]
        TE3 = 0x0b,
        #[doc = "0b111_1110 + R missing after Sr during dynamic address arbitration. Target detected a 0b111_1110 + R missing after Sr during dynamic address arbitration"]
        TE4 = 0x0c,
        #[doc = "Transaction after detecting CCC. Target detected an illegally formatted CCC"]
        TE5 = 0x0d,
        #[doc = "Monitoring error. Target detected that transmitted data on the bus is different from expected"]
        TE6 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Coderr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Coderr {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Coderr {
        #[inline(always)]
        fn from(val: u8) -> Coderr {
            Coderr::from_bits(val)
        }
    }
    impl From<Coderr> for u8 {
        #[inline(always)]
        fn from(val: Coderr) -> u8 {
            Coderr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Crinit {
        #[doc = "Once enabled by setting EN = 1, the peripheral initially acts as a target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role."]
        TARGET = 0x0,
        #[doc = "Once enabled by setting EN = 1, the peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
        CONTROLLER = 0x01,
    }
    impl Crinit {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crinit {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crinit {
        #[inline(always)]
        fn from(val: u8) -> Crinit {
            Crinit::from_bits(val)
        }
    }
    impl From<Crinit> for u8 {
        #[inline(always)]
        fn from(val: Crinit) -> u8 {
            Crinit::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        WRITE = 0x0,
        READ = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dis {
        #[doc = "write to DA\\[7:0\\]
and to IBIDEN in the I3C_DEVRx register is allowed"]
        ALLOWED = 0x0,
        #[doc = "write to DA\\[7:0\\]
and to IBIDEN is disabled/locked"]
        LOCKED = 0x01,
    }
    impl Dis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dis {
        #[inline(always)]
        fn from(val: u8) -> Dis {
            Dis::from_bits(val)
        }
    }
    impl From<Dis> for u8 {
        #[inline(always)]
        fn from(val: Dis) -> u8 {
            Dis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mend {
        #[doc = "this message from controller is followed by a repeated start (Sr), before another message must be emitted"]
        REPEATED_START = 0x0,
        #[doc = "this message from controller ends with a stop (P), being the last message of a frame"]
        STOP = 0x01,
    }
    impl Mend {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mend {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mend {
        #[inline(always)]
        fn from(val: u8) -> Mend {
            Mend::from_bits(val)
        }
    }
    impl From<Mend> for u8 {
        #[inline(always)]
        fn from(val: Mend) -> u8 {
            Mend::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rnw {
        #[doc = "write message"]
        WRITE = 0x0,
        #[doc = "read message"]
        READ = 0x01,
    }
    impl Rnw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rnw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rnw {
        #[inline(always)]
        fn from(val: u8) -> Rnw {
            Rnw::from_bits(val)
        }
    }
    impl From<Rnw> for u8 {
        #[inline(always)]
        fn from(val: Rnw) -> u8 {
            Rnw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rstact {
        NO_RESET = 0x0,
        #[doc = "first level of reset: the application software must either: a) partially reset the peripheral, by a write and clear of the enable bit of the I3C configuration register (write EN = 0). This resets the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (except the EN bit). b) fully reset the peripheral, including all its registers, via a write and set of the I3C reset control bit of the RCC (reset and clock controller) register."]
        FIRST_LEVEL = 0x01,
        #[doc = "second level of reset: the application software must issue a warm reset, also known as a system reset. This (see Section 11: Reset and clock control (RCC)) has the same impact as a pin reset (NRST = 0): – the software writes and sets the SYSRESETREQ control bit of the AITR register, when the device is controlled by a Cortex®-M. – the software writes and sets SYSRST = 1 in the RCC_GRSTCSETR register, when the device is controlled by a Cortex®-A."]
        SECOND_LEVEL = 0x02,
        NO_RESET_EITHER = 0x03,
    }
    impl Rstact {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rstact {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rstact {
        #[inline(always)]
        fn from(val: u8) -> Rstact {
            Rstact::from_bits(val)
        }
    }
    impl From<Rstact> for u8 {
        #[inline(always)]
        fn from(val: Rstact) -> u8 {
            Rstact::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Thres {
        #[doc = "TXFNFF is set when 1 byte must be written in TX-FIFO (in I3C_TDR)."]
        BYTE = 0x0,
        #[doc = "TXFNFF is set when 1 word / 4 bytes must be written in TX-FIFO (in the I3C_TDWR register). If the a number of the last transmitted data is not a multiple of 4 bytes (XDCNT\\[1:0\\]
= 00 in the I3C_SR register), only the relevant 1, 2, or 3 valid LSB bytes of the last word are taken into account by the hardware, and sent on the I3C bus."]
        WORD = 0x01,
    }
    impl Thres {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Thres {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Thres {
        #[inline(always)]
        fn from(val: u8) -> Thres {
            Thres::from_bits(val)
        }
    }
    impl From<Thres> for u8 {
        #[inline(always)]
        fn from(val: Thres) -> u8 {
            Thres::to_bits(val)
        }
    }
}
