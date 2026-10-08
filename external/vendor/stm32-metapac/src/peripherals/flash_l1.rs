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
    pub const fn obr(self) -> crate::common::Reg<regs::Obr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wrpr1(self) -> crate::common::Reg<regs::Wrpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wrpr2(self) -> crate::common::Reg<regs::Wrpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wrpr3(self) -> crate::common::Reg<regs::Wrpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
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
        #[doc = "64-bit access"]
        #[inline(always)]
        pub const fn acc64(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "64-bit access"]
        #[inline(always)]
        pub fn set_acc64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("acc64", &self.acc64())
                .field("sleep_pd", &self.sleep_pd())
                .field("run_pd", &self.run_pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Acr {{ latency: {=bool:?}, prften: {=bool:?}, acc64: {=bool:?}, sleep_pd: {=bool:?}, run_pd: {=bool:?} }}" , self . latency () , self . prften () , self . acc64 () , self . sleep_pd () , self . run_pd ())
        }
    }
    #[doc = "Option byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Read protection"]
        #[inline(always)]
        pub const fn rdprt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read protection"]
        #[inline(always)]
        pub fn set_rdprt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        #[doc = "IWDG_SW"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG_SW"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "nRTS_STOP"]
        #[inline(always)]
        pub const fn n_rts_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "nRTS_STOP"]
        #[inline(always)]
        pub fn set_n_rts_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot From Bank 2"]
        #[inline(always)]
        pub const fn bfb2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Boot From Bank 2"]
        #[inline(always)]
        pub fn set_bfb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Obr {
        #[inline(always)]
        fn default() -> Obr {
            Obr(0)
        }
    }
    impl core::fmt::Debug for Obr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obr")
                .field("rdprt", &self.rdprt())
                .field("bor_lev", &self.bor_lev())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("n_rts_stop", &self.n_rts_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("bfb2", &self.bfb2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obr {{ rdprt: {=u8:?}, bor_lev: {=u8:?}, iwdg_sw: {=bool:?}, n_rts_stop: {=bool:?}, n_rst_stdby: {=bool:?}, bfb2: {=bool:?} }}" , self . rdprt () , self . bor_lev () , self . iwdg_sw () , self . n_rts_stop () , self . n_rst_stdby () , self . bfb2 ())
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
        pub const fn ftdw(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed time data write for Byte, Half Word and Word programming"]
        #[inline(always)]
        pub fn set_ftdw(&mut self, val: bool) {
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
                .field("ftdw", &self.ftdw())
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
            defmt :: write ! (f , "Pecr {{ pelock: {=bool:?}, prglock: {=bool:?}, optlock: {=bool:?}, prog: {=bool:?}, data: {=bool:?}, ftdw: {=bool:?}, erase: {=bool:?}, fprg: {=bool:?}, parallelbank: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, obl_launch: {=bool:?} }}" , self . pelock () , self . prglock () , self . optlock () , self . prog () , self . data () , self . ftdw () , self . erase () , self . fprg () , self . parallelbank () , self . eopie () , self . errie () , self . obl_launch ())
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
        #[doc = "Option UserValidity Error"]
        #[inline(always)]
        pub const fn optverrusr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Option UserValidity Error"]
        #[inline(always)]
        pub fn set_optverrusr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("optverrusr", &self.optverrusr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ bsy: {=bool:?}, eop: {=bool:?}, endhv: {=bool:?}, ready: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, sizerr: {=bool:?}, optverr: {=bool:?}, optverrusr: {=bool:?} }}" , self . bsy () , self . eop () , self . endhv () , self . ready () , self . wrperr () , self . pgaerr () , self . sizerr () , self . optverr () , self . optverrusr ())
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpr1(pub u32);
    impl Wrpr1 {
        #[doc = "Write protection"]
        #[inline(always)]
        pub const fn wrp1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub fn set_wrp1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrpr1 {
        #[inline(always)]
        fn default() -> Wrpr1 {
            Wrpr1(0)
        }
    }
    impl core::fmt::Debug for Wrpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpr1").field("wrp1", &self.wrp1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpr1 {{ wrp1: {=u32:?} }}", self.wrp1())
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpr2(pub u32);
    impl Wrpr2 {
        #[doc = "WRP2"]
        #[inline(always)]
        pub const fn wrp2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "WRP2"]
        #[inline(always)]
        pub fn set_wrp2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrpr2 {
        #[inline(always)]
        fn default() -> Wrpr2 {
            Wrpr2(0)
        }
    }
    impl core::fmt::Debug for Wrpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpr2").field("wrp2", &self.wrp2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpr2 {{ wrp2: {=u32:?} }}", self.wrp2())
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpr3(pub u32);
    impl Wrpr3 {
        #[doc = "WRP3"]
        #[inline(always)]
        pub const fn wrp3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "WRP3"]
        #[inline(always)]
        pub fn set_wrp3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrpr3 {
        #[inline(always)]
        fn default() -> Wrpr3 {
            Wrpr3(0)
        }
    }
    impl core::fmt::Debug for Wrpr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpr3").field("wrp3", &self.wrp3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpr3 {{ wrp3: {=u32:?} }}", self.wrp3())
        }
    }
}
