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
    #[doc = "Flash option control register"]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Flash option control register 1"]
    #[inline(always)]
    pub const fn optcr1(self) -> crate::common::Reg<regs::Optcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Flash option control register"]
    #[inline(always)]
    pub const fn optcr2(self) -> crate::common::Reg<regs::Optcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
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
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Latency::from_bits(val as u8)
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: super::vals::Latency) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ART Accelerator Enable"]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ART Accelerator Enable"]
        #[inline(always)]
        pub fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ART Accelerator reset"]
        #[inline(always)]
        pub const fn artrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ART Accelerator reset"]
        #[inline(always)]
        pub fn set_artrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("arten", &self.arten())
                .field("artrst", &self.artrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {:?}, prften: {=bool:?}, arten: {=bool:?}, artrst: {=bool:?} }}",
                self.latency(),
                self.prften(),
                self.arten(),
                self.artrst()
            )
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
        #[doc = "Sector Erase"]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sector Erase"]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass Erase of sectors 0 to 11"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase of sectors 0 to 11"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sector number"]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Sector number"]
        #[inline(always)]
        pub fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Program size"]
        #[inline(always)]
        pub const fn psize(&self) -> super::vals::Psize {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Psize::from_bits(val as u8)
        }
        #[doc = "Program size"]
        #[inline(always)]
        pub fn set_psize(&mut self, val: super::vals::Psize) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCROP error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
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
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("mer", &self.mer())
                .field("snb", &self.snb())
                .field("psize", &self.psize())
                .field("strt", &self.strt())
                .field("eopie", &self.eopie())
                .field("errie", &self.errie())
                .field("rderrie", &self.rderrie())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ pg: {=bool:?}, ser: {=bool:?}, mer: {=bool:?}, snb: {=u8:?}, psize: {:?}, strt: {=bool:?}, eopie: {=bool:?}, errie: {=bool:?}, rderrie: {=bool:?}, lock: {=bool:?} }}" , self . pg () , self . ser () , self . mer () , self . snb () , self . psize () , self . strt () , self . eopie () , self . errie () , self . rderrie () , self . lock ())
        }
    }
    #[doc = "Flash option control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "Option lock"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option lock"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option start"]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option start"]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "User option bytes"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "User option bytes"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WDG_SW User option bytes"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WDG_SW User option bytes"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "nRST_STOP User option bytes"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STOP User option bytes"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "nRST_STDBY User option bytes"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY User option bytes"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Read protect"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Read protect"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Not write protect"]
        #[inline(always)]
        pub const fn n_wrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Not write protect"]
        #[inline(always)]
        pub fn set_n_wrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Dual Boot mode (valid only when nDBANK=0)"]
        #[inline(always)]
        pub const fn n_dboot(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Dual Boot mode (valid only when nDBANK=0)"]
        #[inline(always)]
        pub fn set_n_dboot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Not dual bank mode"]
        #[inline(always)]
        pub const fn n_dbank(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Not dual bank mode"]
        #[inline(always)]
        pub fn set_n_dbank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Independent watchdog counter freeze in standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("optstrt", &self.optstrt())
                .field("bor_lev", &self.bor_lev())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("rdp", &self.rdp())
                .field("n_wrp", &self.n_wrp())
                .field("n_dboot", &self.n_dboot())
                .field("n_dbank", &self.n_dbank())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("iwdg_stop", &self.iwdg_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Optcr {{ optlock: {=bool:?}, optstrt: {=bool:?}, bor_lev: {=u8:?}, wwdg_sw: {=bool:?}, iwdg_sw: {=bool:?}, n_rst_stop: {=bool:?}, n_rst_stdby: {=bool:?}, rdp: {=u8:?}, n_wrp: {=u8:?}, n_dboot: {=bool:?}, n_dbank: {=bool:?}, iwdg_stdby: {=bool:?}, iwdg_stop: {=bool:?} }}" , self . optlock () , self . optstrt () , self . bor_lev () , self . wwdg_sw () , self . iwdg_sw () , self . n_rst_stop () , self . n_rst_stdby () , self . rdp () , self . n_wrp () , self . n_dboot () , self . n_dbank () , self . iwdg_stdby () , self . iwdg_stop ())
        }
    }
    #[doc = "Flash option control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr1(pub u32);
    impl Optcr1 {
        #[doc = "Boot base address when Boot pin =0"]
        #[inline(always)]
        pub const fn boot_add0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot base address when Boot pin =0"]
        #[inline(always)]
        pub fn set_boot_add0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Boot base address when Boot pin =1"]
        #[inline(always)]
        pub const fn boot_add1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot base address when Boot pin =1"]
        #[inline(always)]
        pub fn set_boot_add1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Optcr1 {
        #[inline(always)]
        fn default() -> Optcr1 {
            Optcr1(0)
        }
    }
    impl core::fmt::Debug for Optcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr1")
                .field("boot_add0", &self.boot_add0())
                .field("boot_add1", &self.boot_add1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr1 {{ boot_add0: {=u16:?}, boot_add1: {=u16:?} }}",
                self.boot_add0(),
                self.boot_add1()
            )
        }
    }
    #[doc = "Flash option control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr2(pub u32);
    impl Optcr2 {
        #[doc = "PCROP option byte"]
        #[inline(always)]
        pub const fn pcropi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PCROP option byte"]
        #[inline(always)]
        pub fn set_pcropi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "PCROP zone preserved when RDP level decreased"]
        #[inline(always)]
        pub const fn pcrop_rdp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP zone preserved when RDP level decreased"]
        #[inline(always)]
        pub fn set_pcrop_rdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr2 {
        #[inline(always)]
        fn default() -> Optcr2 {
            Optcr2(0)
        }
    }
    impl core::fmt::Debug for Optcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr2")
                .field("pcropi", &self.pcropi())
                .field("pcrop_rdp", &self.pcrop_rdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr2 {{ pcropi: {=u8:?}, pcrop_rdp: {=bool:?} }}",
                self.pcropi(),
                self.pcrop_rdp()
            )
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Programming parallelism error"]
        #[inline(always)]
        pub const fn pgperr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Programming parallelism error"]
        #[inline(always)]
        pub fn set_pgperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Erase Sequence Error"]
        #[inline(always)]
        pub const fn erserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Erase Sequence Error"]
        #[inline(always)]
        pub fn set_erserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RDERR"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RDERR"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("eop", &self.eop())
                .field("operr", &self.operr())
                .field("wrperr", &self.wrperr())
                .field("pgaerr", &self.pgaerr())
                .field("pgperr", &self.pgperr())
                .field("erserr", &self.erserr())
                .field("rderr", &self.rderr())
                .field("bsy", &self.bsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ eop: {=bool:?}, operr: {=bool:?}, wrperr: {=bool:?}, pgaerr: {=bool:?}, pgperr: {=bool:?}, erserr: {=bool:?}, rderr: {=bool:?}, bsy: {=bool:?} }}" , self . eop () , self . operr () , self . wrperr () , self . pgaerr () , self . pgperr () , self . erserr () , self . rderr () , self . bsy ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Latency {
        #[doc = "0 wait states"]
        WS0 = 0x0,
        #[doc = "1 wait states"]
        WS1 = 0x01,
        #[doc = "2 wait states"]
        WS2 = 0x02,
        #[doc = "3 wait states"]
        WS3 = 0x03,
        #[doc = "4 wait states"]
        WS4 = 0x04,
        #[doc = "5 wait states"]
        WS5 = 0x05,
        #[doc = "6 wait states"]
        WS6 = 0x06,
        #[doc = "7 wait states"]
        WS7 = 0x07,
        #[doc = "8 wait states"]
        WS8 = 0x08,
        #[doc = "9 wait states"]
        WS9 = 0x09,
        #[doc = "10 wait states"]
        WS10 = 0x0a,
        #[doc = "11 wait states"]
        WS11 = 0x0b,
        #[doc = "12 wait states"]
        WS12 = 0x0c,
        #[doc = "13 wait states"]
        WS13 = 0x0d,
        #[doc = "14 wait states"]
        WS14 = 0x0e,
        #[doc = "15 wait states"]
        WS15 = 0x0f,
    }
    impl Latency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Latency {
            unsafe { core::mem::transmute(val & 0x0f) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Psize {
        #[doc = "Program x8"]
        PSIZE8 = 0x0,
        #[doc = "Program x16"]
        PSIZE16 = 0x01,
        #[doc = "Program x32"]
        PSIZE32 = 0x02,
        #[doc = "Program x64"]
        PSIZE64 = 0x03,
    }
    impl Psize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Psize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Psize {
        #[inline(always)]
        fn from(val: u8) -> Psize {
            Psize::from_bits(val)
        }
    }
    impl From<Psize> for u8 {
        #[inline(always)]
        fn from(val: Psize) -> u8 {
            Psize::to_bits(val)
        }
    }
}
