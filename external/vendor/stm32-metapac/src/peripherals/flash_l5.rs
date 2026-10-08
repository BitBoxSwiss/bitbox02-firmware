#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flash non-secure key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Flash secure key register"]
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Flash option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash low voltage key register"]
    #[inline(always)]
    pub const fn lvekeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Flash status register"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Flash status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Flash non-secure control register"]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Flash secure control register"]
    #[inline(always)]
    pub const fn seccr(self) -> crate::common::Reg<regs::Seccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Flash option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Flash non-secure boot address 0 register"]
    #[inline(always)]
    pub const fn nsbootadd0r(self) -> crate::common::Reg<regs::Nsbootadd0r, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Flash non-secure boot address 1 register"]
    #[inline(always)]
    pub const fn nsbootadd1r(self) -> crate::common::Reg<regs::Nsbootadd1r, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FFlash secure boot address 0 register"]
    #[inline(always)]
    pub const fn secbootadd0r(self) -> crate::common::Reg<regs::Secbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Flash bank 1 secure watermak1 register"]
    #[inline(always)]
    pub const fn secwm1r1(self) -> crate::common::Reg<regs::Secwm1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Flash secure watermak1 register 2"]
    #[inline(always)]
    pub const fn secwm1r2(self) -> crate::common::Reg<regs::Secwm1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Flash secure watermak2 register"]
    #[inline(always)]
    pub const fn secwm2r1(self) -> crate::common::Reg<regs::Secwm2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Flash secure watermak2 register2"]
    #[inline(always)]
    pub const fn secwm2r2(self) -> crate::common::Reg<regs::Secwm2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Flash WPR2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(self) -> crate::common::Reg<regs::Wrp2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Flash WPR2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(self) -> crate::common::Reg<regs::Wrp2br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[inline(always)]
    pub const fn secbb1r1(self) -> crate::common::Reg<regs::Secbb1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[inline(always)]
    pub const fn secbb1r2(self) -> crate::common::Reg<regs::Secbb1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[inline(always)]
    pub const fn secbb1r3(self) -> crate::common::Reg<regs::Secbb1r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[inline(always)]
    pub const fn secbb1r4(self) -> crate::common::Reg<regs::Secbb1r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[inline(always)]
    pub const fn secbb2r1(self) -> crate::common::Reg<regs::Secbb2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[inline(always)]
    pub const fn secbb2r2(self) -> crate::common::Reg<regs::Secbb2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[inline(always)]
    pub const fn secbb2r3(self) -> crate::common::Reg<regs::Secbb2r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[inline(always)]
    pub const fn secbb2r4(self) -> crate::common::Reg<regs::Secbb2r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FLASH secure HDP control register"]
    #[inline(always)]
    pub const fn sechdpcr(self) -> crate::common::Reg<regs::Sechdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Power privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency"]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flash Power-down mode during Low-power run mode"]
        #[inline(always)]
        pub const fn run_pd(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power-down mode during Low-power run mode"]
        #[inline(always)]
        pub fn set_run_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash Power-down mode during Low-power sleep mode"]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power-down mode during Low-power sleep mode"]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LVEN"]
        #[inline(always)]
        pub const fn lven(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "LVEN"]
        #[inline(always)]
        pub fn set_lven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("run_pd", &self.run_pd())
                .field("sleep_pd", &self.sleep_pd())
                .field("lven", &self.lven())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {=u8:?}, run_pd: {=bool:?}, sleep_pd: {=bool:?}, lven: {=bool:?} }}",
                self.latency(),
                self.run_pd(),
                self.sleep_pd(),
                self.lven()
            )
        }
    }
    #[doc = "Flash ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "BK_ECC"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BK_ECC"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SYSF_ECC"]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SYSF_ECC"]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECCC2"]
        #[inline(always)]
        pub const fn eccc2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ECCC2"]
        #[inline(always)]
        pub fn set_eccc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ECCD2"]
        #[inline(always)]
        pub const fn eccd2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ECCD2"]
        #[inline(always)]
        pub fn set_eccd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection"]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection"]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    impl core::fmt::Debug for Eccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr")
                .field("addr_ecc", &self.addr_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("eccie", &self.eccie())
                .field("eccc2", &self.eccc2())
                .field("eccd2", &self.eccd2())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Eccr {{ addr_ecc: {=u32:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, eccie: {=bool:?}, eccc2: {=bool:?}, eccd2: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}" , self . addr_ecc () , self . bk_ecc () , self . sysf_ecc () , self . eccie () , self . eccc2 () , self . eccd2 () , self . eccc () , self . eccd ())
        }
    }
    #[doc = "Flash non-secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd0r(pub u32);
    impl Nsbootadd0r {
        #[doc = "NSBOOTADD0"]
        #[inline(always)]
        pub const fn nsbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "NSBOOTADD0"]
        #[inline(always)]
        pub fn set_nsbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd0r {
        #[inline(always)]
        fn default() -> Nsbootadd0r {
            Nsbootadd0r(0)
        }
    }
    impl core::fmt::Debug for Nsbootadd0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootadd0r")
                .field("nsbootadd0", &self.nsbootadd0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootadd0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsbootadd0r {{ nsbootadd0: {=u32:?} }}", self.nsbootadd0())
        }
    }
    #[doc = "Flash non-secure boot address 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd1r(pub u32);
    impl Nsbootadd1r {
        #[doc = "NSBOOTADD1"]
        #[inline(always)]
        pub const fn nsbootadd1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "NSBOOTADD1"]
        #[inline(always)]
        pub fn set_nsbootadd1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd1r {
        #[inline(always)]
        fn default() -> Nsbootadd1r {
            Nsbootadd1r(0)
        }
    }
    impl core::fmt::Debug for Nsbootadd1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootadd1r")
                .field("nsbootadd1", &self.nsbootadd1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootadd1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsbootadd1r {{ nsbootadd1: {=u32:?} }}", self.nsbootadd1())
        }
    }
    #[doc = "Flash non-secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "NSPG"]
        #[inline(always)]
        pub const fn nspg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NSPG"]
        #[inline(always)]
        pub fn set_nspg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NSPER"]
        #[inline(always)]
        pub const fn nsper(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "NSPER"]
        #[inline(always)]
        pub fn set_nsper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NSMER1"]
        #[inline(always)]
        pub const fn nsmer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NSMER1"]
        #[inline(always)]
        pub fn set_nsmer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "NSPNB"]
        #[inline(always)]
        pub const fn nspnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "NSPNB"]
        #[inline(always)]
        pub fn set_nspnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "NSBKER"]
        #[inline(always)]
        pub const fn nsbker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "NSBKER"]
        #[inline(always)]
        pub fn set_nsbker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "NSMER2"]
        #[inline(always)]
        pub const fn nsmer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "NSMER2"]
        #[inline(always)]
        pub fn set_nsmer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub const fn nsstrt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub fn set_nsstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "NSEOPIE"]
        #[inline(always)]
        pub const fn nseopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "NSEOPIE"]
        #[inline(always)]
        pub fn set_nseopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "NSERRIE"]
        #[inline(always)]
        pub const fn nserrie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "NSERRIE"]
        #[inline(always)]
        pub fn set_nserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "NSLOCK"]
        #[inline(always)]
        pub const fn nslock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "NSLOCK"]
        #[inline(always)]
        pub fn set_nslock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
                .field("nspg", &self.nspg())
                .field("nsper", &self.nsper())
                .field("nsmer1", &self.nsmer1())
                .field("nspnb", &self.nspnb())
                .field("nsbker", &self.nsbker())
                .field("nsmer2", &self.nsmer2())
                .field("nsstrt", &self.nsstrt())
                .field("optstrt", &self.optstrt())
                .field("nseopie", &self.nseopie())
                .field("nserrie", &self.nserrie())
                .field("obl_launch", &self.obl_launch())
                .field("optlock", &self.optlock())
                .field("nslock", &self.nslock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nscr {{ nspg: {=bool:?}, nsper: {=bool:?}, nsmer1: {=bool:?}, nspnb: {=u8:?}, nsbker: {=bool:?}, nsmer2: {=bool:?}, nsstrt: {=bool:?}, optstrt: {=bool:?}, nseopie: {=bool:?}, nserrie: {=bool:?}, obl_launch: {=bool:?}, optlock: {=bool:?}, nslock: {=bool:?} }}" , self . nspg () , self . nsper () , self . nsmer1 () , self . nspnb () , self . nsbker () , self . nsmer2 () , self . nsstrt () , self . optstrt () , self . nseopie () , self . nserrie () , self . obl_launch () , self . optlock () , self . nslock ())
        }
    }
    #[doc = "Flash status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[doc = "NSEOP"]
        #[inline(always)]
        pub const fn nseop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NSEOP"]
        #[inline(always)]
        pub fn set_nseop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NSOPERR"]
        #[inline(always)]
        pub const fn nsoperr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "NSOPERR"]
        #[inline(always)]
        pub fn set_nsoperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NSPROGERR"]
        #[inline(always)]
        pub const fn nsprogerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "NSPROGERR"]
        #[inline(always)]
        pub fn set_nsprogerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "NSWRPERR"]
        #[inline(always)]
        pub const fn nswrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "NSWRPERR"]
        #[inline(always)]
        pub fn set_nswrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "NSPGAERR"]
        #[inline(always)]
        pub const fn nspgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "NSPGAERR"]
        #[inline(always)]
        pub fn set_nspgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "NSSIZERR"]
        #[inline(always)]
        pub const fn nssizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "NSSIZERR"]
        #[inline(always)]
        pub fn set_nssizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "NSPGSERR"]
        #[inline(always)]
        pub const fn nspgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "NSPGSERR"]
        #[inline(always)]
        pub fn set_nspgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "OPTWERR"]
        #[inline(always)]
        pub const fn optwerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPTWERR"]
        #[inline(always)]
        pub fn set_optwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPTVERR"]
        #[inline(always)]
        pub const fn optverr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OPTVERR"]
        #[inline(always)]
        pub fn set_optverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "NSBusy"]
        #[inline(always)]
        pub const fn nsbsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "NSBusy"]
        #[inline(always)]
        pub fn set_nsbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    impl core::fmt::Debug for Nssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nssr")
                .field("nseop", &self.nseop())
                .field("nsoperr", &self.nsoperr())
                .field("nsprogerr", &self.nsprogerr())
                .field("nswrperr", &self.nswrperr())
                .field("nspgaerr", &self.nspgaerr())
                .field("nssizerr", &self.nssizerr())
                .field("nspgserr", &self.nspgserr())
                .field("optwerr", &self.optwerr())
                .field("optverr", &self.optverr())
                .field("nsbsy", &self.nsbsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nssr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Nssr {{ nseop: {=bool:?}, nsoperr: {=bool:?}, nsprogerr: {=bool:?}, nswrperr: {=bool:?}, nspgaerr: {=bool:?}, nssizerr: {=bool:?}, nspgserr: {=bool:?}, optwerr: {=bool:?}, optverr: {=bool:?}, nsbsy: {=bool:?} }}" , self . nseop () , self . nsoperr () , self . nsprogerr () , self . nswrperr () , self . nspgaerr () , self . nssizerr () , self . nspgserr () , self . optwerr () , self . optverr () , self . nsbsy ())
        }
    }
    #[doc = "Flash option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Read protection level"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read protection level"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub const fn n_rst_shdw(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub fn set_n_rst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SWAP_BANK"]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SWAP_BANK"]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DB256K"]
        #[inline(always)]
        pub const fn db256k(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DB256K"]
        #[inline(always)]
        pub fn set_db256k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DBANK"]
        #[inline(always)]
        pub const fn dbank(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DBANK"]
        #[inline(always)]
        pub fn set_dbank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub const fn sram2_pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub fn set_sram2_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "nSWBOOT0"]
        #[inline(always)]
        pub const fn n_swboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "nSWBOOT0"]
        #[inline(always)]
        pub fn set_n_swboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "nBOOT0"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "nBOOT0"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PA15_PUPEN"]
        #[inline(always)]
        pub const fn pa15_pupen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PA15_PUPEN"]
        #[inline(always)]
        pub fn set_pa15_pupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "TZEN"]
        #[inline(always)]
        pub const fn tzen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TZEN"]
        #[inline(always)]
        pub fn set_tzen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optr {
        #[inline(always)]
        fn default() -> Optr {
            Optr(0)
        }
    }
    impl core::fmt::Debug for Optr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optr")
                .field("rdp", &self.rdp())
                .field("bor_lev", &self.bor_lev())
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("n_rst_shdw", &self.n_rst_shdw())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("swap_bank", &self.swap_bank())
                .field("db256k", &self.db256k())
                .field("dbank", &self.dbank())
                .field("sram2_pe", &self.sram2_pe())
                .field("sram2_rst", &self.sram2_rst())
                .field("n_swboot0", &self.n_swboot0())
                .field("n_boot0", &self.n_boot0())
                .field("pa15_pupen", &self.pa15_pupen())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optr {{ rdp: {=u8:?}, bor_lev: {=u8:?}, n_rst_stop: {=bool:?}, n_rst_stdby: {=bool:?}, n_rst_shdw: {=bool:?}, iwdg_sw: {=bool:?}, iwdg_stop: {=bool:?}, iwdg_stdby: {=bool:?}, wwdg_sw: {=bool:?}, swap_bank: {=bool:?}, db256k: {=bool:?}, dbank: {=bool:?}, sram2_pe: {=bool:?}, sram2_rst: {=bool:?}, n_swboot0: {=bool:?}, n_boot0: {=bool:?}, pa15_pupen: {=bool:?}, tzen: {=bool:?} }}" , self . rdp () , self . bor_lev () , self . n_rst_stop () , self . n_rst_stdby () , self . n_rst_shdw () , self . iwdg_sw () , self . iwdg_stop () , self . iwdg_stdby () , self . wwdg_sw () , self . swap_bank () , self . db256k () , self . dbank () , self . sram2_pe () , self . sram2_rst () , self . n_swboot0 () , self . n_boot0 () , self . pa15_pupen () , self . tzen ())
        }
    }
    #[doc = "Power privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "PRIV"]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PRIV"]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr").field("priv_", &self.priv_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privcfgr {{ priv_: {=bool:?} }}", self.priv_())
        }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r1(pub u32);
    impl Secbb1r1 {
        #[doc = "SECBB1"]
        #[inline(always)]
        pub const fn secbb1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB1"]
        #[inline(always)]
        pub fn set_secbb1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb1r1 {
        #[inline(always)]
        fn default() -> Secbb1r1 {
            Secbb1r1(0)
        }
    }
    impl core::fmt::Debug for Secbb1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r1").field("secbb1", &self.secbb1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb1r1 {{ secbb1: {=u32:?} }}", self.secbb1())
        }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r2(pub u32);
    impl Secbb1r2 {
        #[doc = "SECBB1"]
        #[inline(always)]
        pub const fn secbb1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB1"]
        #[inline(always)]
        pub fn set_secbb1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb1r2 {
        #[inline(always)]
        fn default() -> Secbb1r2 {
            Secbb1r2(0)
        }
    }
    impl core::fmt::Debug for Secbb1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r2").field("secbb1", &self.secbb1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb1r2 {{ secbb1: {=u32:?} }}", self.secbb1())
        }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r3(pub u32);
    impl Secbb1r3 {
        #[doc = "SECBB1"]
        #[inline(always)]
        pub const fn secbb1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB1"]
        #[inline(always)]
        pub fn set_secbb1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb1r3 {
        #[inline(always)]
        fn default() -> Secbb1r3 {
            Secbb1r3(0)
        }
    }
    impl core::fmt::Debug for Secbb1r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r3").field("secbb1", &self.secbb1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb1r3 {{ secbb1: {=u32:?} }}", self.secbb1())
        }
    }
    #[doc = "FLASH secure block based bank 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb1r4(pub u32);
    impl Secbb1r4 {
        #[doc = "SECBB1"]
        #[inline(always)]
        pub const fn secbb1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB1"]
        #[inline(always)]
        pub fn set_secbb1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb1r4 {
        #[inline(always)]
        fn default() -> Secbb1r4 {
            Secbb1r4(0)
        }
    }
    impl core::fmt::Debug for Secbb1r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb1r4").field("secbb1", &self.secbb1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb1r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb1r4 {{ secbb1: {=u32:?} }}", self.secbb1())
        }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r1(pub u32);
    impl Secbb2r1 {
        #[doc = "SECBB2"]
        #[inline(always)]
        pub const fn secbb2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB2"]
        #[inline(always)]
        pub fn set_secbb2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb2r1 {
        #[inline(always)]
        fn default() -> Secbb2r1 {
            Secbb2r1(0)
        }
    }
    impl core::fmt::Debug for Secbb2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r1").field("secbb2", &self.secbb2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb2r1 {{ secbb2: {=u32:?} }}", self.secbb2())
        }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r2(pub u32);
    impl Secbb2r2 {
        #[doc = "SECBB2"]
        #[inline(always)]
        pub const fn secbb2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB2"]
        #[inline(always)]
        pub fn set_secbb2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb2r2 {
        #[inline(always)]
        fn default() -> Secbb2r2 {
            Secbb2r2(0)
        }
    }
    impl core::fmt::Debug for Secbb2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r2").field("secbb2", &self.secbb2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb2r2 {{ secbb2: {=u32:?} }}", self.secbb2())
        }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r3(pub u32);
    impl Secbb2r3 {
        #[doc = "SECBB2"]
        #[inline(always)]
        pub const fn secbb2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB2"]
        #[inline(always)]
        pub fn set_secbb2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb2r3 {
        #[inline(always)]
        fn default() -> Secbb2r3 {
            Secbb2r3(0)
        }
    }
    impl core::fmt::Debug for Secbb2r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r3").field("secbb2", &self.secbb2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb2r3 {{ secbb2: {=u32:?} }}", self.secbb2())
        }
    }
    #[doc = "FLASH secure block based bank 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb2r4(pub u32);
    impl Secbb2r4 {
        #[doc = "SECBB2"]
        #[inline(always)]
        pub const fn secbb2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SECBB2"]
        #[inline(always)]
        pub fn set_secbb2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Secbb2r4 {
        #[inline(always)]
        fn default() -> Secbb2r4 {
            Secbb2r4(0)
        }
    }
    impl core::fmt::Debug for Secbb2r4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb2r4").field("secbb2", &self.secbb2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb2r4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secbb2r4 {{ secbb2: {=u32:?} }}", self.secbb2())
        }
    }
    #[doc = "FFlash secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootadd0r(pub u32);
    impl Secbootadd0r {
        #[doc = "BOOT_LOCK"]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_LOCK"]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SECBOOTADD0"]
        #[inline(always)]
        pub const fn secbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "SECBOOTADD0"]
        #[inline(always)]
        pub fn set_secbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Secbootadd0r {
        #[inline(always)]
        fn default() -> Secbootadd0r {
            Secbootadd0r(0)
        }
    }
    impl core::fmt::Debug for Secbootadd0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbootadd0r")
                .field("boot_lock", &self.boot_lock())
                .field("secbootadd0", &self.secbootadd0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbootadd0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secbootadd0r {{ boot_lock: {=bool:?}, secbootadd0: {=u32:?} }}",
                self.boot_lock(),
                self.secbootadd0()
            )
        }
    }
    #[doc = "Flash secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr(pub u32);
    impl Seccr {
        #[doc = "SECPG"]
        #[inline(always)]
        pub const fn secpg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SECPG"]
        #[inline(always)]
        pub fn set_secpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SECPER"]
        #[inline(always)]
        pub const fn secper(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SECPER"]
        #[inline(always)]
        pub fn set_secper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SECMER1"]
        #[inline(always)]
        pub const fn secmer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SECMER1"]
        #[inline(always)]
        pub fn set_secmer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SECPNB"]
        #[inline(always)]
        pub const fn secpnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "SECPNB"]
        #[inline(always)]
        pub fn set_secpnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "SECBKER"]
        #[inline(always)]
        pub const fn secbker(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SECBKER"]
        #[inline(always)]
        pub fn set_secbker(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SECMER2"]
        #[inline(always)]
        pub const fn secmer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SECMER2"]
        #[inline(always)]
        pub fn set_secmer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SECSTRT"]
        #[inline(always)]
        pub const fn secstrt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SECSTRT"]
        #[inline(always)]
        pub fn set_secstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SECEOPIE"]
        #[inline(always)]
        pub const fn seceopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SECEOPIE"]
        #[inline(always)]
        pub fn set_seceopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SECERRIE"]
        #[inline(always)]
        pub const fn secerrie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SECERRIE"]
        #[inline(always)]
        pub fn set_secerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SECRDERRIE"]
        #[inline(always)]
        pub const fn secrderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SECRDERRIE"]
        #[inline(always)]
        pub fn set_secrderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SECINV"]
        #[inline(always)]
        pub const fn secinv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SECINV"]
        #[inline(always)]
        pub fn set_secinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SECLOCK"]
        #[inline(always)]
        pub const fn seclock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SECLOCK"]
        #[inline(always)]
        pub fn set_seclock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr {
        #[inline(always)]
        fn default() -> Seccr {
            Seccr(0)
        }
    }
    impl core::fmt::Debug for Seccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccr")
                .field("secpg", &self.secpg())
                .field("secper", &self.secper())
                .field("secmer1", &self.secmer1())
                .field("secpnb", &self.secpnb())
                .field("secbker", &self.secbker())
                .field("secmer2", &self.secmer2())
                .field("secstrt", &self.secstrt())
                .field("seceopie", &self.seceopie())
                .field("secerrie", &self.secerrie())
                .field("secrderrie", &self.secrderrie())
                .field("secinv", &self.secinv())
                .field("seclock", &self.seclock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccr {{ secpg: {=bool:?}, secper: {=bool:?}, secmer1: {=bool:?}, secpnb: {=u8:?}, secbker: {=bool:?}, secmer2: {=bool:?}, secstrt: {=bool:?}, seceopie: {=bool:?}, secerrie: {=bool:?}, secrderrie: {=bool:?}, secinv: {=bool:?}, seclock: {=bool:?} }}" , self . secpg () , self . secper () , self . secmer1 () , self . secpnb () , self . secbker () , self . secmer2 () , self . secstrt () , self . seceopie () , self . secerrie () , self . secrderrie () , self . secinv () , self . seclock ())
        }
    }
    #[doc = "FLASH secure HDP control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sechdpcr(pub u32);
    impl Sechdpcr {
        #[doc = "HDP1_ACCDIS"]
        #[inline(always)]
        pub const fn hdp1_accdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HDP1_ACCDIS"]
        #[inline(always)]
        pub fn set_hdp1_accdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HDP2_ACCDIS"]
        #[inline(always)]
        pub const fn hdp2_accdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HDP2_ACCDIS"]
        #[inline(always)]
        pub fn set_hdp2_accdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Sechdpcr {
        #[inline(always)]
        fn default() -> Sechdpcr {
            Sechdpcr(0)
        }
    }
    impl core::fmt::Debug for Sechdpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sechdpcr")
                .field("hdp1_accdis", &self.hdp1_accdis())
                .field("hdp2_accdis", &self.hdp2_accdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sechdpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sechdpcr {{ hdp1_accdis: {=bool:?}, hdp2_accdis: {=bool:?} }}",
                self.hdp1_accdis(),
                self.hdp2_accdis()
            )
        }
    }
    #[doc = "Flash status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "SECEOP"]
        #[inline(always)]
        pub const fn seceop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SECEOP"]
        #[inline(always)]
        pub fn set_seceop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SECOPERR"]
        #[inline(always)]
        pub const fn secoperr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SECOPERR"]
        #[inline(always)]
        pub fn set_secoperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SECPROGERR"]
        #[inline(always)]
        pub const fn secprogerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SECPROGERR"]
        #[inline(always)]
        pub fn set_secprogerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SECWRPERR"]
        #[inline(always)]
        pub const fn secwrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SECWRPERR"]
        #[inline(always)]
        pub fn set_secwrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SECPGAERR"]
        #[inline(always)]
        pub const fn secpgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SECPGAERR"]
        #[inline(always)]
        pub fn set_secpgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SECSIZERR"]
        #[inline(always)]
        pub const fn secsizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SECSIZERR"]
        #[inline(always)]
        pub fn set_secsizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SECPGSERR"]
        #[inline(always)]
        pub const fn secpgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SECPGSERR"]
        #[inline(always)]
        pub fn set_secpgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure read protection error"]
        #[inline(always)]
        pub const fn secrderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure read protection error"]
        #[inline(always)]
        pub fn set_secrderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SECBusy"]
        #[inline(always)]
        pub const fn secbsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SECBusy"]
        #[inline(always)]
        pub fn set_secbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    impl core::fmt::Debug for Secsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secsr")
                .field("seceop", &self.seceop())
                .field("secoperr", &self.secoperr())
                .field("secprogerr", &self.secprogerr())
                .field("secwrperr", &self.secwrperr())
                .field("secpgaerr", &self.secpgaerr())
                .field("secsizerr", &self.secsizerr())
                .field("secpgserr", &self.secpgserr())
                .field("secrderr", &self.secrderr())
                .field("secbsy", &self.secbsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Secsr {{ seceop: {=bool:?}, secoperr: {=bool:?}, secprogerr: {=bool:?}, secwrperr: {=bool:?}, secpgaerr: {=bool:?}, secsizerr: {=bool:?}, secpgserr: {=bool:?}, secrderr: {=bool:?}, secbsy: {=bool:?} }}" , self . seceop () , self . secoperr () , self . secprogerr () , self . secwrperr () , self . secpgaerr () , self . secsizerr () , self . secpgserr () , self . secrderr () , self . secbsy ())
        }
    }
    #[doc = "Flash bank 1 secure watermak1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r1(pub u32);
    impl Secwm1r1 {
        #[doc = "SECWM1_PSTRT"]
        #[inline(always)]
        pub const fn secwm1_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "SECWM1_PSTRT"]
        #[inline(always)]
        pub fn set_secwm1_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "SECWM1_PEND"]
        #[inline(always)]
        pub const fn secwm1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "SECWM1_PEND"]
        #[inline(always)]
        pub fn set_secwm1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm1r1 {
        #[inline(always)]
        fn default() -> Secwm1r1 {
            Secwm1r1(0)
        }
    }
    impl core::fmt::Debug for Secwm1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm1r1")
                .field("secwm1_pstrt", &self.secwm1_pstrt())
                .field("secwm1_pend", &self.secwm1_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r1 {{ secwm1_pstrt: {=u8:?}, secwm1_pend: {=u8:?} }}",
                self.secwm1_pstrt(),
                self.secwm1_pend()
            )
        }
    }
    #[doc = "Flash secure watermak1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r2(pub u32);
    impl Secwm1r2 {
        #[doc = "PCROP1_PSTRT"]
        #[inline(always)]
        pub const fn pcrop1_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "PCROP1_PSTRT"]
        #[inline(always)]
        pub fn set_pcrop1_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "PCROP1EN"]
        #[inline(always)]
        pub const fn pcrop1en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP1EN"]
        #[inline(always)]
        pub fn set_pcrop1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HDP1_PEND"]
        #[inline(always)]
        pub const fn hdp1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HDP1_PEND"]
        #[inline(always)]
        pub fn set_hdp1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "HDP1EN"]
        #[inline(always)]
        pub const fn hdp1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "HDP1EN"]
        #[inline(always)]
        pub fn set_hdp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secwm1r2 {
        #[inline(always)]
        fn default() -> Secwm1r2 {
            Secwm1r2(0)
        }
    }
    impl core::fmt::Debug for Secwm1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm1r2")
                .field("pcrop1_pstrt", &self.pcrop1_pstrt())
                .field("pcrop1en", &self.pcrop1en())
                .field("hdp1_pend", &self.hdp1_pend())
                .field("hdp1en", &self.hdp1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm1r2 {{ pcrop1_pstrt: {=u8:?}, pcrop1en: {=bool:?}, hdp1_pend: {=u8:?}, hdp1en: {=bool:?} }}",
                self.pcrop1_pstrt(),
                self.pcrop1en(),
                self.hdp1_pend(),
                self.hdp1en()
            )
        }
    }
    #[doc = "Flash secure watermak2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r1(pub u32);
    impl Secwm2r1 {
        #[doc = "SECWM2_PSTRT"]
        #[inline(always)]
        pub const fn secwm2_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "SECWM2_PSTRT"]
        #[inline(always)]
        pub fn set_secwm2_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "SECWM2_PEND"]
        #[inline(always)]
        pub const fn secwm2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "SECWM2_PEND"]
        #[inline(always)]
        pub fn set_secwm2_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm2r1 {
        #[inline(always)]
        fn default() -> Secwm2r1 {
            Secwm2r1(0)
        }
    }
    impl core::fmt::Debug for Secwm2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm2r1")
                .field("secwm2_pstrt", &self.secwm2_pstrt())
                .field("secwm2_pend", &self.secwm2_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r1 {{ secwm2_pstrt: {=u8:?}, secwm2_pend: {=u8:?} }}",
                self.secwm2_pstrt(),
                self.secwm2_pend()
            )
        }
    }
    #[doc = "Flash secure watermak2 register2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r2(pub u32);
    impl Secwm2r2 {
        #[doc = "PCROP2_PSTRT"]
        #[inline(always)]
        pub const fn pcrop2_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "PCROP2_PSTRT"]
        #[inline(always)]
        pub fn set_pcrop2_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "PCROP2EN"]
        #[inline(always)]
        pub const fn pcrop2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP2EN"]
        #[inline(always)]
        pub fn set_pcrop2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HDP2_PEND"]
        #[inline(always)]
        pub const fn hdp2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HDP2_PEND"]
        #[inline(always)]
        pub fn set_hdp2_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "HDP2EN"]
        #[inline(always)]
        pub const fn hdp2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "HDP2EN"]
        #[inline(always)]
        pub fn set_hdp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secwm2r2 {
        #[inline(always)]
        fn default() -> Secwm2r2 {
            Secwm2r2(0)
        }
    }
    impl core::fmt::Debug for Secwm2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm2r2")
                .field("pcrop2_pstrt", &self.pcrop2_pstrt())
                .field("pcrop2en", &self.pcrop2en())
                .field("hdp2_pend", &self.hdp2_pend())
                .field("hdp2en", &self.hdp2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm2r2 {{ pcrop2_pstrt: {=u8:?}, pcrop2en: {=bool:?}, hdp2_pend: {=u8:?}, hdp2en: {=bool:?} }}",
                self.pcrop2_pstrt(),
                self.pcrop2en(),
                self.hdp2_pend(),
                self.hdp2en()
            )
        }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "WRP1A_PSTRT"]
        #[inline(always)]
        pub const fn wrp1a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP1A_PSTRT"]
        #[inline(always)]
        pub fn set_wrp1a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP1A_PEND"]
        #[inline(always)]
        pub const fn wrp1a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP1A_PEND"]
        #[inline(always)]
        pub fn set_wrp1a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp1ar {
        #[inline(always)]
        fn default() -> Wrp1ar {
            Wrp1ar(0)
        }
    }
    impl core::fmt::Debug for Wrp1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1ar")
                .field("wrp1a_pstrt", &self.wrp1a_pstrt())
                .field("wrp1a_pend", &self.wrp1a_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1ar {{ wrp1a_pstrt: {=u8:?}, wrp1a_pend: {=u8:?} }}",
                self.wrp1a_pstrt(),
                self.wrp1a_pend()
            )
        }
    }
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "WRP1B_PSTRT"]
        #[inline(always)]
        pub const fn wrp1b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP1B_PSTRT"]
        #[inline(always)]
        pub fn set_wrp1b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP1B_PEND"]
        #[inline(always)]
        pub const fn wrp1b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP1B_PEND"]
        #[inline(always)]
        pub fn set_wrp1b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp1br {
        #[inline(always)]
        fn default() -> Wrp1br {
            Wrp1br(0)
        }
    }
    impl core::fmt::Debug for Wrp1br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1br")
                .field("wrp1b_pstrt", &self.wrp1b_pstrt())
                .field("wrp1b_pend", &self.wrp1b_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp1br {{ wrp1b_pstrt: {=u8:?}, wrp1b_pend: {=u8:?} }}",
                self.wrp1b_pstrt(),
                self.wrp1b_pend()
            )
        }
    }
    #[doc = "Flash WPR2 area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2ar(pub u32);
    impl Wrp2ar {
        #[doc = "WRP2A_PSTRT"]
        #[inline(always)]
        pub const fn wrp2a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP2A_PSTRT"]
        #[inline(always)]
        pub fn set_wrp2a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP2A_PEND"]
        #[inline(always)]
        pub const fn wrp2a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP2A_PEND"]
        #[inline(always)]
        pub fn set_wrp2a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp2ar {
        #[inline(always)]
        fn default() -> Wrp2ar {
            Wrp2ar(0)
        }
    }
    impl core::fmt::Debug for Wrp2ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2ar")
                .field("wrp2a_pstrt", &self.wrp2a_pstrt())
                .field("wrp2a_pend", &self.wrp2a_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2ar {{ wrp2a_pstrt: {=u8:?}, wrp2a_pend: {=u8:?} }}",
                self.wrp2a_pstrt(),
                self.wrp2a_pend()
            )
        }
    }
    #[doc = "Flash WPR2 area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2br(pub u32);
    impl Wrp2br {
        #[doc = "WRP2B_PSTRT"]
        #[inline(always)]
        pub const fn wrp2b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP2B_PSTRT"]
        #[inline(always)]
        pub fn set_wrp2b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP2B_PEND"]
        #[inline(always)]
        pub const fn wrp2b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP2B_PEND"]
        #[inline(always)]
        pub fn set_wrp2b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Wrp2br {
        #[inline(always)]
        fn default() -> Wrp2br {
            Wrp2br(0)
        }
    }
    impl core::fmt::Debug for Wrp2br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2br")
                .field("wrp2b_pstrt", &self.wrp2b_pstrt())
                .field("wrp2b_pend", &self.wrp2b_pend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrp2br {{ wrp2b_pstrt: {=u8:?}, wrp2b_pend: {=u8:?} }}",
                self.wrp2b_pstrt(),
                self.wrp2b_pend()
            )
        }
    }
}
