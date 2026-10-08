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
    #[doc = "Program/erase control register"]
    #[inline(always)]
    pub const fn pecr(self) -> crate::common::Reg<regs::Pecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Program/erase key register"]
    #[inline(always)]
    pub const fn pekeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Program memory key register"]
    #[inline(always)]
    pub const fn prgkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Option byte register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write Protection Register 1"]
    #[inline(always)]
    pub const fn wrprot(self) -> crate::common::Reg<regs::Wrprot, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Write Protection Register 2"]
    #[inline(always)]
    pub const fn wrprot2(self) -> crate::common::Reg<regs::Wrprot, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
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
        pub const fn latency(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash mode during Sleep"]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash mode during Sleep"]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash mode during Run"]
        #[inline(always)]
        pub const fn run_pd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash mode during Run"]
        #[inline(always)]
        pub fn set_run_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable Buffer"]
        #[inline(always)]
        pub const fn disab_buf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Buffer"]
        #[inline(always)]
        pub fn set_disab_buf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pre-read data address"]
        #[inline(always)]
        pub const fn pre_read(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Pre-read data address"]
        #[inline(always)]
        pub fn set_pre_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("prften", &self.prften())
                .field("sleep_pd", &self.sleep_pd())
                .field("run_pd", &self.run_pd())
                .field("disab_buf", &self.disab_buf())
                .field("pre_read", &self.pre_read())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Acr {{ latency: {=bool:?}, prften: {=bool:?}, sleep_pd: {=bool:?}, run_pd: {=bool:?}, disab_buf: {=bool:?}, pre_read: {=bool:?} }}" , self . latency () , self . prften () , self . sleep_pd () , self . run_pd () , self . disab_buf () , self . pre_read ())
        }
    }
    #[doc = "Option byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Read protection"]
        #[inline(always)]
        pub const fn rdprot(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read protection"]
        #[inline(always)]
        pub fn set_rdprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Selection of protection mode of WPR bits"]
        #[inline(always)]
        pub const fn wprmod(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Selection of protection mode of WPR bits"]
        #[inline(always)]
        pub fn set_wprmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BOR_LEV"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "BOR_LEV"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
                .field("rdprot", &self.rdprot())
                .field("wprmod", &self.wprmod())
                .field("bor_lev", &self.bor_lev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optr {{ rdprot: {=u8:?}, wprmod: {=bool:?}, bor_lev: {=u8:?} }}",
                self.rdprot(),
                self.wprmod(),
                self.bor_lev()
            )
        }
    }
    #[doc = "Program/erase control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pecr(pub u32);
    impl Pecr {
        #[doc = "FLASH_PECR and data EEPROM lock"]
        #[inline(always)]
        pub const fn pelock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_PECR and data EEPROM lock"]
        #[inline(always)]
        pub fn set_pelock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Program memory lock"]
        #[inline(always)]
        pub const fn prglock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Program memory lock"]
        #[inline(always)]
        pub fn set_prglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Option bytes block lock"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Option bytes block lock"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Program memory selection"]
        #[inline(always)]
        pub const fn prog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Program memory selection"]
        #[inline(always)]
        pub fn set_prog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data EEPROM selection"]
        #[inline(always)]
        pub const fn data(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data EEPROM selection"]
        #[inline(always)]
        pub fn set_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Fixed time data write for Byte, Half Word and Word programming"]
        #[inline(always)]
        pub const fn fix(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed time data write for Byte, Half Word and Word programming"]
        #[inline(always)]
        pub fn set_fix(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page or Double Word erase mode"]
        #[inline(always)]
        pub const fn erase(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page or Double Word erase mode"]
        #[inline(always)]
        pub fn set_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Half Page/Double Word programming mode"]
        #[inline(always)]
        pub const fn fprg(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Half Page/Double Word programming mode"]
        #[inline(always)]
        pub fn set_fprg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Parallel bank mode"]
        #[inline(always)]
        pub const fn parallelbank(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Parallel bank mode"]
        #[inline(always)]
        pub fn set_parallelbank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "End of programming interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End of programming interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Launch the option byte loading"]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Launch the option byte loading"]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Pecr {
        #[inline(always)]
        fn default() -> Pecr {
            Pecr(0)
        }
    }
    impl core::fmt::Debug for Pecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pecr")
                .field("pelock", &self.pelock())
                .field("prglock", &self.prglock())
                .field("optlock", &self.optlock())
                .field("prog", &self.prog())
                .field("data", &self.data())
                .field("fix", &self.fix())
                .field("erase", &self.erase())
                .field("fprg", &self.fprg())
                .field("parallelbank", &self.parallelbank())
                .field("eopie", &self.eopie())
                .field("errie", &self.errie())
                .field("obl_launch", &self.obl_launch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pecr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pecr {{ pelock: {=bool:?}, prglock: {=bool:?}, optlock: {=bool:?}, prog: {=bool:?}, data: {=bool:?}, fix: {=bool:?}, erase: {=bool:?}, fprg: {=bool:?}, parallelbank: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, obl_launch: {=bool:?} }}" , self . pelock () , self . prglock () , self . optlock () , self . prog () , self . data () , self . fix () , self . erase () , self . fprg () , self . parallelbank () , self . eopie () , self . errie () , self . obl_launch ())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Write/erase operations in progress"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write/erase operations in progress"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of high voltage"]
        #[inline(always)]
        pub const fn endhv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of high voltage"]
        #[inline(always)]
        pub fn set_endhv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash memory module ready after low power mode"]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory module ready after low power mode"]
        #[inline(always)]
        pub fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub const fn optverr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub fn set_optverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RDERR"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RDERR"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "NOTZEROERR"]
        #[inline(always)]
        pub const fn notzeroerr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "NOTZEROERR"]
        #[inline(always)]
        pub fn set_notzeroerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "FWWERR"]
        #[inline(always)]
        pub const fn fwwerr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "FWWERR"]
        #[inline(always)]
        pub fn set_fwwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("bsy", &self.bsy())
                .field("eop", &self.eop())
                .field("endhv", &self.endhv())
                .field("ready", &self.ready())
                .field("wrperr", &self.wrperr())
                .field("pgaerr", &self.pgaerr())
                .field("sizerr", &self.sizerr())
                .field("optverr", &self.optverr())
                .field("rderr", &self.rderr())
                .field("notzeroerr", &self.notzeroerr())
                .field("fwwerr", &self.fwwerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ bsy: {=bool:?}, eop: {=bool:?}, endhv: {=bool:?}, ready: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, optverr: {=bool:?}, rderr: {=bool:?}, notzeroerr: {=bool:?}, fwwerr: {=bool:?} }}" , self . bsy () , self . eop () , self . endhv () , self . ready () , self . wrperr () , self . pgaerr () , self . sizerr () , self . optverr () , self . rderr () , self . notzeroerr () , self . fwwerr ())
        }
    }
    #[doc = "Write Protection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrprot(pub u32);
    impl Wrprot {
        #[doc = "Write Protection"]
        #[inline(always)]
        pub const fn wrprot(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Write Protection"]
        #[inline(always)]
        pub fn set_wrprot(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wrprot {
        #[inline(always)]
        fn default() -> Wrprot {
            Wrprot(0)
        }
    }
    impl core::fmt::Debug for Wrprot {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrprot")
                .field("wrprot[0]", &self.wrprot(0usize))
                .field("wrprot[1]", &self.wrprot(1usize))
                .field("wrprot[2]", &self.wrprot(2usize))
                .field("wrprot[3]", &self.wrprot(3usize))
                .field("wrprot[4]", &self.wrprot(4usize))
                .field("wrprot[5]", &self.wrprot(5usize))
                .field("wrprot[6]", &self.wrprot(6usize))
                .field("wrprot[7]", &self.wrprot(7usize))
                .field("wrprot[8]", &self.wrprot(8usize))
                .field("wrprot[9]", &self.wrprot(9usize))
                .field("wrprot[10]", &self.wrprot(10usize))
                .field("wrprot[11]", &self.wrprot(11usize))
                .field("wrprot[12]", &self.wrprot(12usize))
                .field("wrprot[13]", &self.wrprot(13usize))
                .field("wrprot[14]", &self.wrprot(14usize))
                .field("wrprot[15]", &self.wrprot(15usize))
                .field("wrprot[16]", &self.wrprot(16usize))
                .field("wrprot[17]", &self.wrprot(17usize))
                .field("wrprot[18]", &self.wrprot(18usize))
                .field("wrprot[19]", &self.wrprot(19usize))
                .field("wrprot[20]", &self.wrprot(20usize))
                .field("wrprot[21]", &self.wrprot(21usize))
                .field("wrprot[22]", &self.wrprot(22usize))
                .field("wrprot[23]", &self.wrprot(23usize))
                .field("wrprot[24]", &self.wrprot(24usize))
                .field("wrprot[25]", &self.wrprot(25usize))
                .field("wrprot[26]", &self.wrprot(26usize))
                .field("wrprot[27]", &self.wrprot(27usize))
                .field("wrprot[28]", &self.wrprot(28usize))
                .field("wrprot[29]", &self.wrprot(29usize))
                .field("wrprot[30]", &self.wrprot(30usize))
                .field("wrprot[31]", &self.wrprot(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrprot {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wrprot {{ wrprot[0]: {=bool:?}, wrprot[1]: {=bool:?}, wrprot[2]: {=bool:?}, wrprot[3]: {=bool:?}, wrprot[4]: {=bool:?}, wrprot[5]: {=bool:?}, wrprot[6]: {=bool:?}, wrprot[7]: {=bool:?}, wrprot[8]: {=bool:?}, wrprot[9]: {=bool:?}, wrprot[10]: {=bool:?}, wrprot[11]: {=bool:?}, wrprot[12]: {=bool:?}, wrprot[13]: {=bool:?}, wrprot[14]: {=bool:?}, wrprot[15]: {=bool:?}, wrprot[16]: {=bool:?}, wrprot[17]: {=bool:?}, wrprot[18]: {=bool:?}, wrprot[19]: {=bool:?}, wrprot[20]: {=bool:?}, wrprot[21]: {=bool:?}, wrprot[22]: {=bool:?}, wrprot[23]: {=bool:?}, wrprot[24]: {=bool:?}, wrprot[25]: {=bool:?}, wrprot[26]: {=bool:?}, wrprot[27]: {=bool:?}, wrprot[28]: {=bool:?}, wrprot[29]: {=bool:?}, wrprot[30]: {=bool:?}, wrprot[31]: {=bool:?} }}" , self . wrprot (0usize) , self . wrprot (1usize) , self . wrprot (2usize) , self . wrprot (3usize) , self . wrprot (4usize) , self . wrprot (5usize) , self . wrprot (6usize) , self . wrprot (7usize) , self . wrprot (8usize) , self . wrprot (9usize) , self . wrprot (10usize) , self . wrprot (11usize) , self . wrprot (12usize) , self . wrprot (13usize) , self . wrprot (14usize) , self . wrprot (15usize) , self . wrprot (16usize) , self . wrprot (17usize) , self . wrprot (18usize) , self . wrprot (19usize) , self . wrprot (20usize) , self . wrprot (21usize) , self . wrprot (22usize) , self . wrprot (23usize) , self . wrprot (24usize) , self . wrprot (25usize) , self . wrprot (26usize) , self . wrprot (27usize) , self . wrprot (28usize) , self . wrprot (29usize) , self . wrprot (30usize) , self . wrprot (31usize))
        }
    }
}
