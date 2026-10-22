#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH"]
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
    #[doc = "Flash access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Flash key register"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flash option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash address register"]
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Option byte register"]
    #[inline(always)]
    pub const fn obr(self) -> crate::common::Reg<regs::Obr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wrpr(self) -> crate::common::Reg<regs::Wrpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency"]
        #[inline(always)]
        pub const fn latency(&self) -> super::vals::Latency {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Latency::from_bits(val as u8)
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: super::vals::Latency) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash half cycle access enable"]
        #[inline(always)]
        pub const fn hlfcya(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash half cycle access enable"]
        #[inline(always)]
        pub fn set_hlfcya(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Prefetch buffer enable"]
        #[inline(always)]
        pub const fn prftbe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer enable"]
        #[inline(always)]
        pub fn set_prftbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prefetch buffer status"]
        #[inline(always)]
        pub const fn prftbs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer status"]
        #[inline(always)]
        pub fn set_prftbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("hlfcya", &self.hlfcya())
                .field("prftbe", &self.prftbe())
                .field("prftbs", &self.prftbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {:?}, hlfcya: {=bool:?}, prftbe: {=bool:?}, prftbs: {=bool:?} }}",
                self.latency(),
                self.hlfcya(),
                self.prftbe(),
                self.prftbs()
            )
        }
    }
    #[doc = "Flash address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ar(pub u32);
    impl Ar {
        #[doc = "Flash Address"]
        #[inline(always)]
        pub const fn far(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash Address"]
        #[inline(always)]
        pub fn set_far(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ar {
        #[inline(always)]
        fn default() -> Ar {
            Ar(0)
        }
    }
    impl core::fmt::Debug for Ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ar").field("far", &self.far()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ar {{ far: {=u32:?} }}", self.far())
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page Erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page Erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass Erase"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Option byte programming"]
        #[inline(always)]
        pub const fn optpg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte programming"]
        #[inline(always)]
        pub fn set_optpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Option byte erase"]
        #[inline(always)]
        pub const fn opter(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase"]
        #[inline(always)]
        pub fn set_opter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option bytes write enable"]
        #[inline(always)]
        pub const fn optwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Option bytes write enable"]
        #[inline(always)]
        pub fn set_optwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("mer", &self.mer())
                .field("optpg", &self.optpg())
                .field("opter", &self.opter())
                .field("strt", &self.strt())
                .field("lock", &self.lock())
                .field("optwre", &self.optwre())
                .field("errie", &self.errie())
                .field("eopie", &self.eopie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ pg: {=bool:?}, per: {=bool:?}, mer: {=bool:?}, optpg: {=bool:?}, opter: {=bool:?}, strt: {=bool:?}, lock: {=bool:?}, optwre: {=bool:?}, errie: {=bool:?}, eopie: {=bool:?} }}" , self . pg () , self . per () , self . mer () , self . optpg () , self . opter () , self . strt () , self . lock () , self . optwre () , self . errie () , self . eopie ())
        }
    }
    #[doc = "Option byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Option byte error"]
        #[inline(always)]
        pub const fn opterr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte error"]
        #[inline(always)]
        pub fn set_opterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read protection"]
        #[inline(always)]
        pub const fn rdprt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read protection"]
        #[inline(always)]
        pub fn set_rdprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WDG_SW"]
        #[inline(always)]
        pub const fn wdg_sw(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WDG_SW"]
        #[inline(always)]
        pub fn set_wdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data0"]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "Data0"]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[doc = "Data1"]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "Data1"]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
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
                .field("opterr", &self.opterr())
                .field("rdprt", &self.rdprt())
                .field("wdg_sw", &self.wdg_sw())
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("data0", &self.data0())
                .field("data1", &self.data1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Obr {{ opterr: {=bool:?}, rdprt: {=bool:?}, wdg_sw: {=bool:?}, n_rst_stop: {=bool:?}, n_rst_stdby: {=bool:?}, data0: {=u8:?}, data1: {=u8:?} }}" , self . opterr () , self . rdprt () , self . wdg_sw () , self . n_rst_stop () , self . n_rst_stdby () , self . data0 () , self . data1 ())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub const fn pgerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub fn set_pgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub const fn wrprterr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub fn set_wrprterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pgerr", &self.pgerr())
                .field("wrprterr", &self.wrprterr())
                .field("eop", &self.eop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ bsy: {=bool:?}, pgerr: {=bool:?}, wrprterr: {=bool:?}, eop: {=bool:?} }}",
                self.bsy(),
                self.pgerr(),
                self.wrprterr(),
                self.eop()
            )
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpr(pub u32);
    impl Wrpr {
        #[doc = "Write protect"]
        #[inline(always)]
        pub const fn wrp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write protect"]
        #[inline(always)]
        pub fn set_wrp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrpr {
        #[inline(always)]
        fn default() -> Wrpr {
            Wrpr(0)
        }
    }
    impl core::fmt::Debug for Wrpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpr").field("wrp", &self.wrp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrpr {{ wrp: {=u32:?} }}", self.wrp())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Latency {
        #[doc = "Zero wait state, if 0 < SYSCLK≤ 24 MHz"]
        WS0 = 0x0,
        #[doc = "One wait state, if 24 MHz < SYSCLK ≤ 48 MHz"]
        WS1 = 0x01,
        #[doc = "Two wait states, if 48 MHz < SYSCLK ≤ 72 MHz"]
        WS2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Latency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Latency {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Latency {
        #[inline(always)]
        fn from(val: u8) -> Latency {
            Latency::from_bits(val)
        }
    }
    impl From<Latency> for u8 {
        #[inline(always)]
        fn from(val: Latency) -> u8 {
            Latency::to_bits(val)
        }
    }
}
