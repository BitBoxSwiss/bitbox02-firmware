#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWR control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PWR control register 3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWR control register 4."]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PWR voltage scaling control register."]
    #[inline(always)]
    pub const fn voscr(self) -> crate::common::Reg<regs::Voscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PWR backup domain control register 1."]
    #[inline(always)]
    pub const fn bdcr1(self) -> crate::common::Reg<regs::Bdcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PWR backup domain control register 2."]
    #[inline(always)]
    pub const fn bdcr2(self) -> crate::common::Reg<regs::Bdcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PWR disable backup protection control register."]
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "PWR CPU control register."]
    #[inline(always)]
    pub const fn cpucr(self) -> crate::common::Reg<regs::Cpucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PWR supply voltage monitoring control register 1."]
    #[inline(always)]
    pub const fn svmcr1(self) -> crate::common::Reg<regs::Svmcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PWR supply voltage monitoring control register 2."]
    #[inline(always)]
    pub const fn svmcr2(self) -> crate::common::Reg<regs::Svmcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "PWR supply voltage monitoring control register 3."]
    #[inline(always)]
    pub const fn svmcr3(self) -> crate::common::Reg<regs::Svmcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PWR wake-up clear register."]
    #[inline(always)]
    pub const fn wkupcr(self) -> crate::common::Reg<regs::Wkupcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "PWR wake-up status register."]
    #[inline(always)]
    pub const fn wkupsr(self) -> crate::common::Reg<regs::Wkupsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "PWR wake-up enable and polarity register."]
    #[inline(always)]
    pub const fn wkupepr(self) -> crate::common::Reg<regs::Wkupepr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "PWR security configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "PWR privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR backup domain control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr1(pub u32);
    impl Bdcr1 {
        #[doc = "V less than sub>BAT less than /sub> and temperature monitoring enable."]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>BAT less than /sub> and temperature monitoring enable."]
        #[inline(always)]
        pub fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V less than sub>BAT less than /sub> level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn vbatl(&self) -> super::vals::Vbatl {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vbatl::from_bits(val as u8)
        }
        #[doc = "V less than sub>BAT less than /sub> level monitoring versus low threshold."]
        #[inline(always)]
        pub fn set_vbatl(&mut self, val: super::vals::Vbatl) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "V less than sub>BAT less than /sub> level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn vbath(&self) -> super::vals::Vbath {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Vbath::from_bits(val as u8)
        }
        #[doc = "V less than sub>BAT less than /sub> level monitoring versus high threshold."]
        #[inline(always)]
        pub fn set_vbath(&mut self, val: super::vals::Vbath) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn templ(&self) -> super::vals::Templ {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Templ::from_bits(val as u8)
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub fn set_templ(&mut self, val: super::vals::Templ) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn temph(&self) -> super::vals::Temph {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Temph::from_bits(val as u8)
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub fn set_temph(&mut self, val: super::vals::Temph) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Bdcr1 {
        #[inline(always)]
        fn default() -> Bdcr1 {
            Bdcr1(0)
        }
    }
    impl core::fmt::Debug for Bdcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr1")
                .field("monen", &self.monen())
                .field("vbatl", &self.vbatl())
                .field("vbath", &self.vbath())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr1 {{ monen: {=bool:?}, vbatl: {:?}, vbath: {:?}, templ: {:?}, temph: {:?} }}",
                self.monen(),
                self.vbatl(),
                self.vbath(),
                self.templ(),
                self.temph()
            )
        }
    }
    #[doc = "PWR backup domain control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr2(pub u32);
    impl Bdcr2 {
        #[doc = "Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and V less than sub>BAT less than /sub> modes)."]
        #[inline(always)]
        pub const fn bkprbsen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM backup supply enable (used to maintain BKPRAM content in Standby and V less than sub>BAT less than /sub> modes)."]
        #[inline(always)]
        pub fn set_bkprbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Bdcr2 {
        #[inline(always)]
        fn default() -> Bdcr2 {
            Bdcr2(0)
        }
    }
    impl core::fmt::Debug for Bdcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr2").field("bkprbsen", &self.bkprbsen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bdcr2 {{ bkprbsen: {=bool:?} }}", self.bkprbsen())
        }
    }
    #[doc = "PWR CPU control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpucr(pub u32);
    impl Cpucr {
        #[doc = "Power-down deepsleep selection."]
        #[inline(always)]
        pub const fn pdds(&self) -> super::vals::Pdds {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pdds::from_bits(val as u8)
        }
        #[doc = "Power-down deepsleep selection."]
        #[inline(always)]
        pub fn set_pdds(&mut self, val: super::vals::Pdds) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[inline(always)]
        pub const fn cssf(&self) -> super::vals::Cssf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cssf::from_bits(val as u8)
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: super::vals::Cssf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop flag."]
        #[inline(always)]
        pub const fn stopf(&self) -> super::vals::Stopf {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Stopf::from_bits(val as u8)
        }
        #[doc = "Stop flag."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: super::vals::Stopf) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Standby flag."]
        #[inline(always)]
        pub const fn sbf(&self) -> super::vals::Sbf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Sbf::from_bits(val as u8)
        }
        #[doc = "Standby flag."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: super::vals::Sbf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "System Stop mode voltage scaling selection."]
        #[inline(always)]
        pub const fn svos(&self) -> super::vals::Svos {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Svos::from_bits(val as u8)
        }
        #[doc = "System Stop mode voltage scaling selection."]
        #[inline(always)]
        pub fn set_svos(&mut self, val: super::vals::Svos) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cpucr {
        #[inline(always)]
        fn default() -> Cpucr {
            Cpucr(0)
        }
    }
    impl core::fmt::Debug for Cpucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cpucr")
                .field("pdds", &self.pdds())
                .field("cssf", &self.cssf())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .field("svos", &self.svos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cpucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cpucr {{ pdds: {:?}, cssf: {:?}, stopf: {:?}, sbf: {:?}, svos: {:?} }}",
                self.pdds(),
                self.cssf(),
                self.stopf(),
                self.sbf(),
                self.svos()
            )
        }
    }
    #[doc = "PWR control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "SMPS step-down converter enable."]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS step-down converter enable."]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enables the pull down on output voltage during power-down mode."]
        #[inline(always)]
        pub const fn mode_pdn(&self) -> super::vals::ModePdn {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::ModePdn::from_bits(val as u8)
        }
        #[doc = "Enables the pull down on output voltage during power-down mode."]
        #[inline(always)]
        pub fn set_mode_pdn(&mut self, val: super::vals::ModePdn) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SMPS low-power mode enable (SVOS high only)."]
        #[inline(always)]
        pub const fn lpds08v(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS low-power mode enable (SVOS high only)."]
        #[inline(always)]
        pub fn set_lpds08v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "pwr_on pulse low configuration."]
        #[inline(always)]
        pub const fn popl(&self) -> super::vals::Popl {
            let val = (self.0 >> 16usize) & 0x1f;
            super::vals::Popl::from_bits(val as u8)
        }
        #[doc = "pwr_on pulse low configuration."]
        #[inline(always)]
        pub fn set_popl(&mut self, val: super::vals::Popl) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("sden", &self.sden())
                .field("mode_pdn", &self.mode_pdn())
                .field("lpds08v", &self.lpds08v())
                .field("popl", &self.popl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ sden: {=bool:?}, mode_pdn: {:?}, lpds08v: {=bool:?}, popl: {:?} }}",
                self.sden(),
                self.mode_pdn(),
                self.lpds08v(),
                self.popl()
            )
        }
    }
    #[doc = "PWR control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Programmable voltage detector enable."]
        #[inline(always)]
        pub const fn pvden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable."]
        #[inline(always)]
        pub fn set_pvden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detect output."]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detect output."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("pvden", &self.pvden())
                .field("pvdo", &self.pvdo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr2 {{ pvden: {=bool:?}, pvdo: {:?} }}", self.pvden(), self.pvdo())
        }
    }
    #[doc = "PWR control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "V less than sub>DDCORE less than /sub> monitoring enable."]
        #[inline(always)]
        pub const fn vcoremonen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDCORE less than /sub> monitoring enable."]
        #[inline(always)]
        pub fn set_vcoremonen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V less than sub>DDCORE less than /sub> voltage detector low-level selection."]
        #[inline(always)]
        pub const fn vcorells(&self) -> super::vals::Vcorells {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Vcorells::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDCORE less than /sub> voltage detector low-level selection."]
        #[inline(always)]
        pub fn set_vcorells(&mut self, val: super::vals::Vcorells) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Monitored V less than sub>DDCORE less than /sub> level above low threshold."]
        #[inline(always)]
        pub const fn vcorel(&self) -> super::vals::Vcorel {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vcorel::from_bits(val as u8)
        }
        #[doc = "Monitored V less than sub>DDCORE less than /sub> level above low threshold."]
        #[inline(always)]
        pub fn set_vcorel(&mut self, val: super::vals::Vcorel) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Monitored V less than sub>DDCORE less than /sub> level above high threshold."]
        #[inline(always)]
        pub const fn vcoreh(&self) -> super::vals::Vcoreh {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vcoreh::from_bits(val as u8)
        }
        #[doc = "Monitored V less than sub>DDCORE less than /sub> level above high threshold."]
        #[inline(always)]
        pub fn set_vcoreh(&mut self, val: super::vals::Vcoreh) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("vcoremonen", &self.vcoremonen())
                .field("vcorells", &self.vcorells())
                .field("vcorel", &self.vcorel())
                .field("vcoreh", &self.vcoreh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ vcoremonen: {=bool:?}, vcorells: {:?}, vcorel: {:?}, vcoreh: {:?} }}",
                self.vcoremonen(),
                self.vcorells(),
                self.vcorel(),
                self.vcoreh()
            )
        }
    }
    #[doc = "PWR control register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)."]
        #[inline(always)]
        pub const fn tcmrbsen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I-TCM and D-TCM RAMs backup supply enable (used to maintain TCM RAMs content in Standby mode)."]
        #[inline(always)]
        pub fn set_tcmrbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)."]
        #[inline(always)]
        pub const fn tcmflxrbsen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I-TCM FLEXMEM backup supply enable (used to maintain I-TCM FLEX MEM content in Standby mode)."]
        #[inline(always)]
        pub fn set_tcmflxrbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cr4 {
        #[inline(always)]
        fn default() -> Cr4 {
            Cr4(0)
        }
    }
    impl core::fmt::Debug for Cr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr4")
                .field("tcmrbsen", &self.tcmrbsen())
                .field("tcmflxrbsen", &self.tcmflxrbsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr4 {{ tcmrbsen: {=bool:?}, tcmflxrbsen: {=bool:?} }}",
                self.tcmrbsen(),
                self.tcmflxrbsen()
            )
        }
    }
    #[doc = "PWR disable backup protection control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[doc = "Disable backup domain write protection."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dbpcr {
        #[inline(always)]
        fn default() -> Dbpcr {
            Dbpcr(0)
        }
    }
    impl core::fmt::Debug for Dbpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbpcr").field("dbp", &self.dbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbpcr {{ dbp: {=bool:?} }}", self.dbp())
        }
    }
    #[doc = "PWR privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "System supply configuration privileged protection."]
        #[inline(always)]
        pub const fn priv0(&self) -> super::vals::Priv0 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Priv0::from_bits(val as u8)
        }
        #[doc = "System supply configuration privileged protection."]
        #[inline(always)]
        pub fn set_priv0(&mut self, val: super::vals::Priv0) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector privileged protection."]
        #[inline(always)]
        pub const fn priv1(&self) -> super::vals::Priv1 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Priv1::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector privileged protection."]
        #[inline(always)]
        pub fn set_priv1(&mut self, val: super::vals::Priv1) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "V less than sub>DDCORE less than /sub> monitor privileged protection."]
        #[inline(always)]
        pub const fn priv2(&self) -> super::vals::Priv2 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Priv2::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDCORE less than /sub> monitor privileged protection."]
        #[inline(always)]
        pub fn set_priv2(&mut self, val: super::vals::Priv2) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection."]
        #[inline(always)]
        pub const fn priv3(&self) -> super::vals::Priv3 {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Priv3::from_bits(val as u8)
        }
        #[doc = "I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection."]
        #[inline(always)]
        pub fn set_priv3(&mut self, val: super::vals::Priv3) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Voltage scaling selection privileged protection."]
        #[inline(always)]
        pub const fn priv4(&self) -> super::vals::Priv4 {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Priv4::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection privileged protection."]
        #[inline(always)]
        pub fn set_priv4(&mut self, val: super::vals::Priv4) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup domain privileged protection."]
        #[inline(always)]
        pub const fn priv5(&self) -> super::vals::Priv5 {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Priv5::from_bits(val as u8)
        }
        #[doc = "Backup domain privileged protection."]
        #[inline(always)]
        pub fn set_priv5(&mut self, val: super::vals::Priv5) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CPU power control privileged protection."]
        #[inline(always)]
        pub const fn priv6(&self) -> super::vals::Priv6 {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Priv6::from_bits(val as u8)
        }
        #[doc = "CPU power control privileged protection."]
        #[inline(always)]
        pub fn set_priv6(&mut self, val: super::vals::Priv6) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Peripheral voltage monitor privileged protection."]
        #[inline(always)]
        pub const fn priv7(&self) -> super::vals::Priv7 {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Priv7::from_bits(val as u8)
        }
        #[doc = "Peripheral voltage monitor privileged protection."]
        #[inline(always)]
        pub fn set_priv7(&mut self, val: super::vals::Priv7) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "WKUP1 pin privileged protection."]
        #[inline(always)]
        pub const fn wkuppriv1(&self) -> super::vals::Wkuppriv1 {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Wkuppriv1::from_bits(val as u8)
        }
        #[doc = "WKUP1 pin privileged protection."]
        #[inline(always)]
        pub fn set_wkuppriv1(&mut self, val: super::vals::Wkuppriv1) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "WKUP2 pin privileged protection."]
        #[inline(always)]
        pub const fn wkuppriv2(&self) -> super::vals::Wkuppriv2 {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Wkuppriv2::from_bits(val as u8)
        }
        #[doc = "WKUP2 pin privileged protection."]
        #[inline(always)]
        pub fn set_wkuppriv2(&mut self, val: super::vals::Wkuppriv2) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "WKUP3 pin privileged protection."]
        #[inline(always)]
        pub const fn wkuppriv3(&self) -> super::vals::Wkuppriv3 {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Wkuppriv3::from_bits(val as u8)
        }
        #[doc = "WKUP3 pin privileged protection."]
        #[inline(always)]
        pub fn set_wkuppriv3(&mut self, val: super::vals::Wkuppriv3) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "WKUP4 pin privileged protection."]
        #[inline(always)]
        pub const fn wkuppriv4(&self) -> super::vals::Wkuppriv4 {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Wkuppriv4::from_bits(val as u8)
        }
        #[doc = "WKUP4 pin privileged protection."]
        #[inline(always)]
        pub fn set_wkuppriv4(&mut self, val: super::vals::Wkuppriv4) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
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
            f.debug_struct("Privcfgr")
                .field("priv0", &self.priv0())
                .field("priv1", &self.priv1())
                .field("priv2", &self.priv2())
                .field("priv3", &self.priv3())
                .field("priv4", &self.priv4())
                .field("priv5", &self.priv5())
                .field("priv6", &self.priv6())
                .field("priv7", &self.priv7())
                .field("wkuppriv1", &self.wkuppriv1())
                .field("wkuppriv2", &self.wkuppriv2())
                .field("wkuppriv3", &self.wkuppriv3())
                .field("wkuppriv4", &self.wkuppriv4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr {{ priv0: {:?}, priv1: {:?}, priv2: {:?}, priv3: {:?}, priv4: {:?}, priv5: {:?}, priv6: {:?}, priv7: {:?}, wkuppriv1: {:?}, wkuppriv2: {:?}, wkuppriv3: {:?}, wkuppriv4: {:?} }}" , self . priv0 () , self . priv1 () , self . priv2 () , self . priv3 () , self . priv4 () , self . priv5 () , self . priv6 () , self . priv7 () , self . wkuppriv1 () , self . wkuppriv2 () , self . wkuppriv3 () , self . wkuppriv4 ())
        }
    }
    #[doc = "PWR security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "System supply configuration secure protection."]
        #[inline(always)]
        pub const fn sec0(&self) -> super::vals::Sec0 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Sec0::from_bits(val as u8)
        }
        #[doc = "System supply configuration secure protection."]
        #[inline(always)]
        pub fn set_sec0(&mut self, val: super::vals::Sec0) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector secure protection."]
        #[inline(always)]
        pub const fn sec1(&self) -> super::vals::Sec1 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Sec1::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector secure protection."]
        #[inline(always)]
        pub fn set_sec1(&mut self, val: super::vals::Sec1) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "V less than sub>DDCORE less than /sub> monitor secure protection."]
        #[inline(always)]
        pub const fn sec2(&self) -> super::vals::Sec2 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Sec2::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDCORE less than /sub> monitor secure protection."]
        #[inline(always)]
        pub fn set_sec2(&mut self, val: super::vals::Sec2) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection."]
        #[inline(always)]
        pub const fn sec3(&self) -> super::vals::Sec3 {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Sec3::from_bits(val as u8)
        }
        #[doc = "I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection."]
        #[inline(always)]
        pub fn set_sec3(&mut self, val: super::vals::Sec3) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Voltage scaling selection secure protection."]
        #[inline(always)]
        pub const fn sec4(&self) -> super::vals::Sec4 {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Sec4::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection secure protection."]
        #[inline(always)]
        pub fn set_sec4(&mut self, val: super::vals::Sec4) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup domain secure protection."]
        #[inline(always)]
        pub const fn sec5(&self) -> super::vals::Sec5 {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Sec5::from_bits(val as u8)
        }
        #[doc = "Backup domain secure protection."]
        #[inline(always)]
        pub fn set_sec5(&mut self, val: super::vals::Sec5) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CPU power control secure protection."]
        #[inline(always)]
        pub const fn sec6(&self) -> super::vals::Sec6 {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Sec6::from_bits(val as u8)
        }
        #[doc = "CPU power control secure protection."]
        #[inline(always)]
        pub fn set_sec6(&mut self, val: super::vals::Sec6) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Peripheral voltage monitor secure protection."]
        #[inline(always)]
        pub const fn sec7(&self) -> super::vals::Sec7 {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Sec7::from_bits(val as u8)
        }
        #[doc = "Peripheral voltage monitor secure protection."]
        #[inline(always)]
        pub fn set_sec7(&mut self, val: super::vals::Sec7) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "WKUP1 pin secure protection."]
        #[inline(always)]
        pub const fn wkupsec1(&self) -> super::vals::Wkupsec1 {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Wkupsec1::from_bits(val as u8)
        }
        #[doc = "WKUP1 pin secure protection."]
        #[inline(always)]
        pub fn set_wkupsec1(&mut self, val: super::vals::Wkupsec1) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "WKUP2 pin secure protection."]
        #[inline(always)]
        pub const fn wkupsec2(&self) -> super::vals::Wkupsec2 {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Wkupsec2::from_bits(val as u8)
        }
        #[doc = "WKUP2 pin secure protection."]
        #[inline(always)]
        pub fn set_wkupsec2(&mut self, val: super::vals::Wkupsec2) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "WKUP3 pin secure protection."]
        #[inline(always)]
        pub const fn wkupsec3(&self) -> super::vals::Wkupsec3 {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Wkupsec3::from_bits(val as u8)
        }
        #[doc = "WKUP3 pin secure protection."]
        #[inline(always)]
        pub fn set_wkupsec3(&mut self, val: super::vals::Wkupsec3) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "WKUP4 pin secure protection."]
        #[inline(always)]
        pub const fn wkupsec4(&self) -> super::vals::Wkupsec4 {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Wkupsec4::from_bits(val as u8)
        }
        #[doc = "WKUP4 pin secure protection."]
        #[inline(always)]
        pub fn set_wkupsec4(&mut self, val: super::vals::Wkupsec4) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("sec0", &self.sec0())
                .field("sec1", &self.sec1())
                .field("sec2", &self.sec2())
                .field("sec3", &self.sec3())
                .field("sec4", &self.sec4())
                .field("sec5", &self.sec5())
                .field("sec6", &self.sec6())
                .field("sec7", &self.sec7())
                .field("wkupsec1", &self.wkupsec1())
                .field("wkupsec2", &self.wkupsec2())
                .field("wkupsec3", &self.wkupsec3())
                .field("wkupsec4", &self.wkupsec4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ sec0: {:?}, sec1: {:?}, sec2: {:?}, sec3: {:?}, sec4: {:?}, sec5: {:?}, sec6: {:?}, sec7: {:?}, wkupsec1: {:?}, wkupsec2: {:?}, wkupsec3: {:?}, wkupsec4: {:?} }}" , self . sec0 () , self . sec1 () , self . sec2 () , self . sec3 () , self . sec4 () , self . sec5 () , self . sec6 () , self . sec7 () , self . wkupsec1 () , self . wkupsec2 () , self . wkupsec3 () , self . wkupsec4 ())
        }
    }
    #[doc = "PWR supply voltage monitoring control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr1(pub u32);
    impl Svmcr1 {
        #[doc = "V less than sub>DDIO4 less than /sub>independent I/O voltage monitor enable."]
        #[inline(always)]
        pub const fn vddio4vmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDIO4 less than /sub>independent I/O voltage monitor enable."]
        #[inline(always)]
        pub fn set_vddio4vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V less than sub>DDIO4 less than /sub>independent I/O supply valid."]
        #[inline(always)]
        pub const fn vddio4sv(&self) -> super::vals::Vddio4sv {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio4sv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO4 less than /sub>independent I/O supply valid."]
        #[inline(always)]
        pub fn set_vddio4sv(&mut self, val: super::vals::Vddio4sv) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "V less than sub>DDIO4 less than /sub>ready."]
        #[inline(always)]
        pub const fn vddio4rdy(&self) -> super::vals::Vddio4rdy {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vddio4rdy::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO4 less than /sub>ready."]
        #[inline(always)]
        pub fn set_vddio4rdy(&mut self, val: super::vals::Vddio4rdy) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "V less than sub>DDIO4 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub const fn vddio4vrsel(&self) -> super::vals::Vddio4vrsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Vddio4vrsel::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO4 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub fn set_vddio4vrsel(&mut self, val: super::vals::Vddio4vrsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "V less than sub>DDIO4 less than /sub> I/O voltage range Standby mode."]
        #[inline(always)]
        pub const fn vddio4vrstby(&self) -> super::vals::Vddio4vrstby {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Vddio4vrstby::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO4 less than /sub> I/O voltage range Standby mode."]
        #[inline(always)]
        pub fn set_vddio4vrstby(&mut self, val: super::vals::Vddio4vrstby) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Svmcr1 {
        #[inline(always)]
        fn default() -> Svmcr1 {
            Svmcr1(0)
        }
    }
    impl core::fmt::Debug for Svmcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmcr1")
                .field("vddio4vmen", &self.vddio4vmen())
                .field("vddio4sv", &self.vddio4sv())
                .field("vddio4rdy", &self.vddio4rdy())
                .field("vddio4vrsel", &self.vddio4vrsel())
                .field("vddio4vrstby", &self.vddio4vrstby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Svmcr1 {{ vddio4vmen: {=bool:?}, vddio4sv: {:?}, vddio4rdy: {:?}, vddio4vrsel: {:?}, vddio4vrstby: {:?} }}" , self . vddio4vmen () , self . vddio4sv () , self . vddio4rdy () , self . vddio4vrsel () , self . vddio4vrstby ())
        }
    }
    #[doc = "PWR supply voltage monitoring control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr2(pub u32);
    impl Svmcr2 {
        #[doc = "V less than sub>DDIO5 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub const fn vddio5vmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDIO5 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub fn set_vddio5vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V less than sub>DDIO5 less than /sub>independent supply valid."]
        #[inline(always)]
        pub const fn vddio5sv(&self) -> super::vals::Vddio5sv {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio5sv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO5 less than /sub>independent supply valid."]
        #[inline(always)]
        pub fn set_vddio5sv(&mut self, val: super::vals::Vddio5sv) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "V less than sub>DDIO5 less than /sub>ready."]
        #[inline(always)]
        pub const fn vddio5rdy(&self) -> super::vals::Vddio5rdy {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vddio5rdy::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO5 less than /sub>ready."]
        #[inline(always)]
        pub fn set_vddio5rdy(&mut self, val: super::vals::Vddio5rdy) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "V less than sub>DDIO5 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub const fn vddio5vrsel(&self) -> super::vals::Vddio5vrsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Vddio5vrsel::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO5 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub fn set_vddio5vrsel(&mut self, val: super::vals::Vddio5vrsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "V less than sub>DDIO5 less than /sub> I/O voltage range Standby mode."]
        #[inline(always)]
        pub const fn vddio5vrstby(&self) -> super::vals::Vddio5vrstby {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Vddio5vrstby::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO5 less than /sub> I/O voltage range Standby mode."]
        #[inline(always)]
        pub fn set_vddio5vrstby(&mut self, val: super::vals::Vddio5vrstby) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Svmcr2 {
        #[inline(always)]
        fn default() -> Svmcr2 {
            Svmcr2(0)
        }
    }
    impl core::fmt::Debug for Svmcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmcr2")
                .field("vddio5vmen", &self.vddio5vmen())
                .field("vddio5sv", &self.vddio5sv())
                .field("vddio5rdy", &self.vddio5rdy())
                .field("vddio5vrsel", &self.vddio5vrsel())
                .field("vddio5vrstby", &self.vddio5vrstby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Svmcr2 {{ vddio5vmen: {=bool:?}, vddio5sv: {:?}, vddio5rdy: {:?}, vddio5vrsel: {:?}, vddio5vrstby: {:?} }}" , self . vddio5vmen () , self . vddio5sv () , self . vddio5rdy () , self . vddio5vrsel () , self . vddio5vrstby ())
        }
    }
    #[doc = "PWR supply voltage monitoring control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr3(pub u32);
    impl Svmcr3 {
        #[doc = "V less than sub>DDIO2 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub const fn vddio2vmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDIO2 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub fn set_vddio2vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V less than sub>DDIO3 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub const fn vddio3vmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDIO3 less than /sub>independent voltage monitor enable."]
        #[inline(always)]
        pub fn set_vddio3vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable."]
        #[inline(always)]
        pub const fn usb33vmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable."]
        #[inline(always)]
        pub fn set_usb33vmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable."]
        #[inline(always)]
        pub const fn avmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable."]
        #[inline(always)]
        pub fn set_avmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "V less than sub>DDIO2 less than /sub>independent supply valid."]
        #[inline(always)]
        pub const fn vddio2sv(&self) -> super::vals::Vddio2sv {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vddio2sv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO2 less than /sub>independent supply valid."]
        #[inline(always)]
        pub fn set_vddio2sv(&mut self, val: super::vals::Vddio2sv) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "V less than sub>DDIO3 less than /sub>independent supply valid."]
        #[inline(always)]
        pub const fn vddio3sv(&self) -> super::vals::Vddio3sv {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vddio3sv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO3 less than /sub>independent supply valid."]
        #[inline(always)]
        pub fn set_vddio3sv(&mut self, val: super::vals::Vddio3sv) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "V less than sub>DD33USB less than /sub>independent supply valid."]
        #[inline(always)]
        pub const fn usb33sv(&self) -> super::vals::Usb33sv {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Usb33sv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DD33USB less than /sub>independent supply valid."]
        #[inline(always)]
        pub fn set_usb33sv(&mut self, val: super::vals::Usb33sv) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>independent supply valid."]
        #[inline(always)]
        pub const fn asv(&self) -> super::vals::Asv {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Asv::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>independent supply valid."]
        #[inline(always)]
        pub fn set_asv(&mut self, val: super::vals::Asv) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "V less than sub>DDIO2 less than /sub>ready."]
        #[inline(always)]
        pub const fn vddio2rdy(&self) -> super::vals::Vddio2rdy {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vddio2rdy::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO2 less than /sub>ready."]
        #[inline(always)]
        pub fn set_vddio2rdy(&mut self, val: super::vals::Vddio2rdy) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "V less than sub>DDIO3 less than /sub>ready."]
        #[inline(always)]
        pub const fn vddio3rdy(&self) -> super::vals::Vddio3rdy {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Vddio3rdy::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO3 less than /sub>ready."]
        #[inline(always)]
        pub fn set_vddio3rdy(&mut self, val: super::vals::Vddio3rdy) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "V less than sub>DD33USB less than /sub>ready."]
        #[inline(always)]
        pub const fn usb33rdy(&self) -> super::vals::Usb33rdy {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Usb33rdy::from_bits(val as u8)
        }
        #[doc = "V less than sub>DD33USB less than /sub>ready."]
        #[inline(always)]
        pub fn set_usb33rdy(&mut self, val: super::vals::Usb33rdy) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>ready."]
        #[inline(always)]
        pub const fn ardy(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "V less than sub>DDA18ADC less than /sub>ready."]
        #[inline(always)]
        pub fn set_ardy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "V less than sub>DD less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub const fn vddiovrsel(&self) -> super::vals::Vddiovrsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Vddiovrsel::from_bits(val as u8)
        }
        #[doc = "V less than sub>DD less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub fn set_vddiovrsel(&mut self, val: super::vals::Vddiovrsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "V less than sub>DDIO2 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub const fn vddio2vrsel(&self) -> super::vals::Vddio2vrsel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Vddio2vrsel::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO2 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub fn set_vddio2vrsel(&mut self, val: super::vals::Vddio2vrsel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "V less than sub>DDIO3 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub const fn vddio3vrsel(&self) -> super::vals::Vddio3vrsel {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Vddio3vrsel::from_bits(val as u8)
        }
        #[doc = "V less than sub>DDIO3 less than /sub> I/O voltage range selection."]
        #[inline(always)]
        pub fn set_vddio3vrsel(&mut self, val: super::vals::Vddio3vrsel) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Svmcr3 {
        #[inline(always)]
        fn default() -> Svmcr3 {
            Svmcr3(0)
        }
    }
    impl core::fmt::Debug for Svmcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmcr3")
                .field("vddio2vmen", &self.vddio2vmen())
                .field("vddio3vmen", &self.vddio3vmen())
                .field("usb33vmen", &self.usb33vmen())
                .field("avmen", &self.avmen())
                .field("vddio2sv", &self.vddio2sv())
                .field("vddio3sv", &self.vddio3sv())
                .field("usb33sv", &self.usb33sv())
                .field("asv", &self.asv())
                .field("vddio2rdy", &self.vddio2rdy())
                .field("vddio3rdy", &self.vddio3rdy())
                .field("usb33rdy", &self.usb33rdy())
                .field("ardy", &self.ardy())
                .field("vddiovrsel", &self.vddiovrsel())
                .field("vddio2vrsel", &self.vddio2vrsel())
                .field("vddio3vrsel", &self.vddio3vrsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Svmcr3 {{ vddio2vmen: {=bool:?}, vddio3vmen: {=bool:?}, usb33vmen: {=bool:?}, avmen: {=bool:?}, vddio2sv: {:?}, vddio3sv: {:?}, usb33sv: {:?}, asv: {:?}, vddio2rdy: {:?}, vddio3rdy: {:?}, usb33rdy: {:?}, ardy: {=bool:?}, vddiovrsel: {:?}, vddio2vrsel: {:?}, vddio3vrsel: {:?} }}" , self . vddio2vmen () , self . vddio3vmen () , self . usb33vmen () , self . avmen () , self . vddio2sv () , self . vddio3sv () , self . usb33sv () , self . asv () , self . vddio2rdy () , self . vddio3rdy () , self . usb33rdy () , self . ardy () , self . vddiovrsel () , self . vddio2vrsel () , self . vddio3vrsel ())
        }
    }
    #[doc = "PWR voltage scaling control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Voscr(pub u32);
    impl Voscr {
        #[doc = "Voltage scaling selection according to performance."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection according to performance."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "VOS ready bit for V less than sub>CORE less than /sub> voltage scaling output selection."]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VOS ready bit for V less than sub>CORE less than /sub> voltage scaling output selection."]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VOS currently applied for V less than sub>CORE less than /sub> voltage scaling selection."]
        #[inline(always)]
        pub const fn actvos(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VOS currently applied for V less than sub>CORE less than /sub> voltage scaling selection."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Voltage level ready bit for currently used ACTVOS."]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage level ready bit for currently used ACTVOS."]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Voscr {
        #[inline(always)]
        fn default() -> Voscr {
            Voscr(0)
        }
    }
    impl core::fmt::Debug for Voscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Voscr")
                .field("vos", &self.vos())
                .field("vosrdy", &self.vosrdy())
                .field("actvos", &self.actvos())
                .field("actvosrdy", &self.actvosrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Voscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Voscr {{ vos: {:?}, vosrdy: {=bool:?}, actvos: {=bool:?}, actvosrdy: {=bool:?} }}",
                self.vos(),
                self.vosrdy(),
                self.actvos(),
                self.actvosrdy()
            )
        }
    }
    #[doc = "PWR wake-up clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupcr(pub u32);
    impl Wkupcr {
        #[doc = "Clear wake-up flag for WKUP1 pin."]
        #[inline(always)]
        pub const fn wkupc1(&self) -> super::vals::Wkupc1 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wkupc1::from_bits(val as u8)
        }
        #[doc = "Clear wake-up flag for WKUP1 pin."]
        #[inline(always)]
        pub fn set_wkupc1(&mut self, val: super::vals::Wkupc1) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear wake-up flag for WKUP2 pin."]
        #[inline(always)]
        pub const fn wkupc2(&self) -> super::vals::Wkupc2 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wkupc2::from_bits(val as u8)
        }
        #[doc = "Clear wake-up flag for WKUP2 pin."]
        #[inline(always)]
        pub fn set_wkupc2(&mut self, val: super::vals::Wkupc2) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wake-up flag for WKUP3 pin."]
        #[inline(always)]
        pub const fn wkupc3(&self) -> super::vals::Wkupc3 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wkupc3::from_bits(val as u8)
        }
        #[doc = "Clear wake-up flag for WKUP3 pin."]
        #[inline(always)]
        pub fn set_wkupc3(&mut self, val: super::vals::Wkupc3) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear wake-up flag for WKUP4 pin."]
        #[inline(always)]
        pub const fn wkupc4(&self) -> super::vals::Wkupc4 {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wkupc4::from_bits(val as u8)
        }
        #[doc = "Clear wake-up flag for WKUP4 pin."]
        #[inline(always)]
        pub fn set_wkupc4(&mut self, val: super::vals::Wkupc4) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Wkupcr {
        #[inline(always)]
        fn default() -> Wkupcr {
            Wkupcr(0)
        }
    }
    impl core::fmt::Debug for Wkupcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupcr")
                .field("wkupc1", &self.wkupc1())
                .field("wkupc2", &self.wkupc2())
                .field("wkupc3", &self.wkupc3())
                .field("wkupc4", &self.wkupc4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupcr {{ wkupc1: {:?}, wkupc2: {:?}, wkupc3: {:?}, wkupc4: {:?} }}",
                self.wkupc1(),
                self.wkupc2(),
                self.wkupc3(),
                self.wkupc4()
            )
        }
    }
    #[doc = "PWR wake-up enable and polarity register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupepr(pub u32);
    impl Wkupepr {
        #[doc = "Enable WKUP1 pin."]
        #[inline(always)]
        pub const fn wkupen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP1 pin."]
        #[inline(always)]
        pub fn set_wkupen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable WKUP2 pin."]
        #[inline(always)]
        pub const fn wkupen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP2 pin."]
        #[inline(always)]
        pub fn set_wkupen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable WKUP3 pin."]
        #[inline(always)]
        pub const fn wkupen3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP3 pin."]
        #[inline(always)]
        pub fn set_wkupen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable WKUP4 pin."]
        #[inline(always)]
        pub const fn wkupen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP4 pin."]
        #[inline(always)]
        pub fn set_wkupen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wake-up polarity bit for WKUP1 pin."]
        #[inline(always)]
        pub const fn wkupp1(&self) -> super::vals::Wkupp1 {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wkupp1::from_bits(val as u8)
        }
        #[doc = "Wake-up polarity bit for WKUP1 pin."]
        #[inline(always)]
        pub fn set_wkupp1(&mut self, val: super::vals::Wkupp1) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Wake-up polarity bit for WKUP2 pin."]
        #[inline(always)]
        pub const fn wkupp2(&self) -> super::vals::Wkupp2 {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Wkupp2::from_bits(val as u8)
        }
        #[doc = "Wake-up polarity bit for WKUP2 pin."]
        #[inline(always)]
        pub fn set_wkupp2(&mut self, val: super::vals::Wkupp2) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wake-up polarity bit for WKUP3 pin."]
        #[inline(always)]
        pub const fn wkupp3(&self) -> super::vals::Wkupp3 {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Wkupp3::from_bits(val as u8)
        }
        #[doc = "Wake-up polarity bit for WKUP3 pin."]
        #[inline(always)]
        pub fn set_wkupp3(&mut self, val: super::vals::Wkupp3) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Wake-up polarity bit for WKUP4 pin."]
        #[inline(always)]
        pub const fn wkupp4(&self) -> super::vals::Wkupp4 {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wkupp4::from_bits(val as u8)
        }
        #[doc = "Wake-up polarity bit for WKUP4 pin."]
        #[inline(always)]
        pub fn set_wkupp4(&mut self, val: super::vals::Wkupp4) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Wake-up pull configuration for WKUP1 pin."]
        #[inline(always)]
        pub const fn wkuppupd1(&self) -> super::vals::Wkuppupd1 {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Wkuppupd1::from_bits(val as u8)
        }
        #[doc = "Wake-up pull configuration for WKUP1 pin."]
        #[inline(always)]
        pub fn set_wkuppupd1(&mut self, val: super::vals::Wkuppupd1) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Wake-up pull configuration for WKUP2 pin."]
        #[inline(always)]
        pub const fn wkuppupd2(&self) -> super::vals::Wkuppupd2 {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Wkuppupd2::from_bits(val as u8)
        }
        #[doc = "Wake-up pull configuration for WKUP2 pin."]
        #[inline(always)]
        pub fn set_wkuppupd2(&mut self, val: super::vals::Wkuppupd2) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Wake-up pull configuration for WKUP3 pin."]
        #[inline(always)]
        pub const fn wkuppupd3(&self) -> super::vals::Wkuppupd3 {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Wkuppupd3::from_bits(val as u8)
        }
        #[doc = "Wake-up pull configuration for WKUP3 pin."]
        #[inline(always)]
        pub fn set_wkuppupd3(&mut self, val: super::vals::Wkuppupd3) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Wake-up pull configuration for WKUP4 pin."]
        #[inline(always)]
        pub const fn wkuppupd4(&self) -> super::vals::Wkuppupd4 {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Wkuppupd4::from_bits(val as u8)
        }
        #[doc = "Wake-up pull configuration for WKUP4 pin."]
        #[inline(always)]
        pub fn set_wkuppupd4(&mut self, val: super::vals::Wkuppupd4) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Wkupepr {
        #[inline(always)]
        fn default() -> Wkupepr {
            Wkupepr(0)
        }
    }
    impl core::fmt::Debug for Wkupepr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupepr")
                .field("wkupen1", &self.wkupen1())
                .field("wkupen2", &self.wkupen2())
                .field("wkupen3", &self.wkupen3())
                .field("wkupen4", &self.wkupen4())
                .field("wkupp1", &self.wkupp1())
                .field("wkupp2", &self.wkupp2())
                .field("wkupp3", &self.wkupp3())
                .field("wkupp4", &self.wkupp4())
                .field("wkuppupd1", &self.wkuppupd1())
                .field("wkuppupd2", &self.wkuppupd2())
                .field("wkuppupd3", &self.wkuppupd3())
                .field("wkuppupd4", &self.wkuppupd4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupepr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wkupepr {{ wkupen1: {=bool:?}, wkupen2: {=bool:?}, wkupen3: {=bool:?}, wkupen4: {=bool:?}, wkupp1: {:?}, wkupp2: {:?}, wkupp3: {:?}, wkupp4: {:?}, wkuppupd1: {:?}, wkuppupd2: {:?}, wkuppupd3: {:?}, wkuppupd4: {:?} }}" , self . wkupen1 () , self . wkupen2 () , self . wkupen3 () , self . wkupen4 () , self . wkupp1 () , self . wkupp2 () , self . wkupp3 () , self . wkupp4 () , self . wkuppupd1 () , self . wkuppupd2 () , self . wkuppupd3 () , self . wkuppupd4 ())
        }
    }
    #[doc = "PWR wake-up status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupsr(pub u32);
    impl Wkupsr {
        #[doc = "Wake-up flag for WKUP1 pin before enable."]
        #[inline(always)]
        pub const fn wkupf1(&self) -> super::vals::Wkupf1 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wkupf1::from_bits(val as u8)
        }
        #[doc = "Wake-up flag for WKUP1 pin before enable."]
        #[inline(always)]
        pub fn set_wkupf1(&mut self, val: super::vals::Wkupf1) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Wake-up flag for WKUP2 pin before enable."]
        #[inline(always)]
        pub const fn wkupf2(&self) -> super::vals::Wkupf2 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wkupf2::from_bits(val as u8)
        }
        #[doc = "Wake-up flag for WKUP2 pin before enable."]
        #[inline(always)]
        pub fn set_wkupf2(&mut self, val: super::vals::Wkupf2) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Wake-up flag for WKUP3 pin before enable."]
        #[inline(always)]
        pub const fn wkupf3(&self) -> super::vals::Wkupf3 {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wkupf3::from_bits(val as u8)
        }
        #[doc = "Wake-up flag for WKUP3 pin before enable."]
        #[inline(always)]
        pub fn set_wkupf3(&mut self, val: super::vals::Wkupf3) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Wake-up flag for WKUP4 pin before enable."]
        #[inline(always)]
        pub const fn wkupf4(&self) -> super::vals::Wkupf4 {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wkupf4::from_bits(val as u8)
        }
        #[doc = "Wake-up flag for WKUP4 pin before enable."]
        #[inline(always)]
        pub fn set_wkupf4(&mut self, val: super::vals::Wkupf4) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Wkupsr {
        #[inline(always)]
        fn default() -> Wkupsr {
            Wkupsr(0)
        }
    }
    impl core::fmt::Debug for Wkupsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupsr")
                .field("wkupf1", &self.wkupf1())
                .field("wkupf2", &self.wkupf2())
                .field("wkupf3", &self.wkupf3())
                .field("wkupf4", &self.wkupf4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupsr {{ wkupf1: {:?}, wkupf2: {:?}, wkupf3: {:?}, wkupf4: {:?} }}",
                self.wkupf1(),
                self.wkupf2(),
                self.wkupf3(),
                self.wkupf4()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Asv {
        #[doc = "V less than sub>DDA18ADC less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDA18ADC less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Asv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Asv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Asv {
        #[inline(always)]
        fn from(val: u8) -> Asv {
            Asv::from_bits(val)
        }
    }
    impl From<Asv> for u8 {
        #[inline(always)]
        fn from(val: Asv) -> u8 {
            Asv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cssf {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "When written, clear the CPU flags (STOPF, SBF)."]
        B_0X1 = 0x01,
    }
    impl Cssf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cssf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cssf {
        #[inline(always)]
        fn from(val: u8) -> Cssf {
            Cssf::from_bits(val)
        }
    }
    impl From<Cssf> for u8 {
        #[inline(always)]
        fn from(val: Cssf) -> u8 {
            Cssf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ModePdn {
        #[doc = "Pull-down disabled. The output is in high impedance during the shutdown (default)."]
        B_0X0 = 0x0,
        #[doc = "Pull-down enabled."]
        B_0X1 = 0x01,
    }
    impl ModePdn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ModePdn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ModePdn {
        #[inline(always)]
        fn from(val: u8) -> ModePdn {
            ModePdn::from_bits(val)
        }
    }
    impl From<ModePdn> for u8 {
        #[inline(always)]
        fn from(val: ModePdn) -> u8 {
            ModePdn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pdds {
        #[doc = "Stop mode when device enters deepsleep."]
        B_0X0 = 0x0,
        #[doc = "Standby mode when device enters deepsleep."]
        B_0X1 = 0x01,
    }
    impl Pdds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pdds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pdds {
        #[inline(always)]
        fn from(val: u8) -> Pdds {
            Pdds::from_bits(val)
        }
    }
    impl From<Pdds> for u8 {
        #[inline(always)]
        fn from(val: Pdds) -> u8 {
            Pdds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Popl {
        #[doc = "No guaranteed minimum low time."]
        B_0X0 = 0x0,
        #[doc = "~ 1 ms guaranteed minimum low time (1 x 32 LSI cycles)."]
        B_0X1 = 0x01,
        #[doc = "~ 2 ms guaranteed minimum low time (2 x 32 LSI cycles)."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        #[doc = "~ 31 ms guaranteed minimum low time (31 x 32 LSI cycles)."]
        B_0X1F = 0x1f,
    }
    impl Popl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Popl {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Popl {
        #[inline(always)]
        fn from(val: u8) -> Popl {
            Popl::from_bits(val)
        }
    }
    impl From<Popl> for u8 {
        #[inline(always)]
        fn from(val: Popl) -> u8 {
            Popl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv0 {
        #[doc = "CR1 can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "CR1 can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv0 {
        #[inline(always)]
        fn from(val: u8) -> Priv0 {
            Priv0::from_bits(val)
        }
    }
    impl From<Priv0> for u8 {
        #[inline(always)]
        fn from(val: Priv0) -> u8 {
            Priv0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv1 {
        #[doc = "CR2 can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "CR2 can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv1 {
        #[inline(always)]
        fn from(val: u8) -> Priv1 {
            Priv1::from_bits(val)
        }
    }
    impl From<Priv1> for u8 {
        #[inline(always)]
        fn from(val: Priv1) -> u8 {
            Priv1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv2 {
        #[doc = "CR3 can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "CR3 can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv2 {
        #[inline(always)]
        fn from(val: u8) -> Priv2 {
            Priv2::from_bits(val)
        }
    }
    impl From<Priv2> for u8 {
        #[inline(always)]
        fn from(val: Priv2) -> u8 {
            Priv2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv3 {
        #[doc = "CR4 can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "CR4 can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv3 {
        #[inline(always)]
        fn from(val: u8) -> Priv3 {
            Priv3::from_bits(val)
        }
    }
    impl From<Priv3> for u8 {
        #[inline(always)]
        fn from(val: Priv3) -> u8 {
            Priv3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv4 {
        #[doc = "VOSCR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "VOSCR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv4 {
        #[inline(always)]
        fn from(val: u8) -> Priv4 {
            Priv4::from_bits(val)
        }
    }
    impl From<Priv4> for u8 {
        #[inline(always)]
        fn from(val: Priv4) -> u8 {
            Priv4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv5 {
        #[doc = "BDCR1, BDCR2, and DBPCR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "BDCR1, BDCR2, and DBPCR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv5 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv5 {
        #[inline(always)]
        fn from(val: u8) -> Priv5 {
            Priv5::from_bits(val)
        }
    }
    impl From<Priv5> for u8 {
        #[inline(always)]
        fn from(val: Priv5) -> u8 {
            Priv5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv6 {
        #[doc = "CPUCR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "CPUCR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv6 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv6 {
        #[inline(always)]
        fn from(val: u8) -> Priv6 {
            Priv6::from_bits(val)
        }
    }
    impl From<Priv6> for u8 {
        #[inline(always)]
        fn from(val: Priv6) -> u8 {
            Priv6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv7 {
        #[doc = "SVMCR1, SVMCR2, and SVMCR3 can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "SVMCR1, SVMCR2, and SVMCR3 can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Priv7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv7 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv7 {
        #[inline(always)]
        fn from(val: u8) -> Priv7 {
            Priv7::from_bits(val)
        }
    }
    impl From<Priv7> for u8 {
        #[inline(always)]
        fn from(val: Priv7) -> u8 {
            Priv7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdo {
        #[doc = "Voltage level on PVD_IN is equal or higher than the internal VREFINT."]
        B_0X0 = 0x0,
        #[doc = "Voltage level on PVD_IN is lower than the internal VREFINT."]
        B_0X1 = 0x01,
    }
    impl Pvdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdo {
        #[inline(always)]
        fn from(val: u8) -> Pvdo {
            Pvdo::from_bits(val)
        }
    }
    impl From<Pvdo> for u8 {
        #[inline(always)]
        fn from(val: Pvdo) -> u8 {
            Pvdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sbf {
        #[doc = "System has not been in Standby mode."]
        B_0X0 = 0x0,
        #[doc = "System has been in Standby mode."]
        B_0X1 = 0x01,
    }
    impl Sbf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sbf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sbf {
        #[inline(always)]
        fn from(val: u8) -> Sbf {
            Sbf::from_bits(val)
        }
    }
    impl From<Sbf> for u8 {
        #[inline(always)]
        fn from(val: Sbf) -> u8 {
            Sbf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec0 {
        #[doc = "CR1 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CR1 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec0 {
        #[inline(always)]
        fn from(val: u8) -> Sec0 {
            Sec0::from_bits(val)
        }
    }
    impl From<Sec0> for u8 {
        #[inline(always)]
        fn from(val: Sec0) -> u8 {
            Sec0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec1 {
        #[doc = "CR2 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CR2 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec1 {
        #[inline(always)]
        fn from(val: u8) -> Sec1 {
            Sec1::from_bits(val)
        }
    }
    impl From<Sec1> for u8 {
        #[inline(always)]
        fn from(val: Sec1) -> u8 {
            Sec1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec2 {
        #[doc = "CR3 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CR3 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec2 {
        #[inline(always)]
        fn from(val: u8) -> Sec2 {
            Sec2::from_bits(val)
        }
    }
    impl From<Sec2> for u8 {
        #[inline(always)]
        fn from(val: Sec2) -> u8 {
            Sec2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec3 {
        #[doc = "CR4 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CR4 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec3 {
        #[inline(always)]
        fn from(val: u8) -> Sec3 {
            Sec3::from_bits(val)
        }
    }
    impl From<Sec3> for u8 {
        #[inline(always)]
        fn from(val: Sec3) -> u8 {
            Sec3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec4 {
        #[doc = "VOSCR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "VOSCR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec4 {
        #[inline(always)]
        fn from(val: u8) -> Sec4 {
            Sec4::from_bits(val)
        }
    }
    impl From<Sec4> for u8 {
        #[inline(always)]
        fn from(val: Sec4) -> u8 {
            Sec4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec5 {
        #[doc = "BDCR1, BDCR2, and DBPCR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "BDCR1, BDCR2, and DBPCR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec5 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec5 {
        #[inline(always)]
        fn from(val: u8) -> Sec5 {
            Sec5::from_bits(val)
        }
    }
    impl From<Sec5> for u8 {
        #[inline(always)]
        fn from(val: Sec5) -> u8 {
            Sec5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec6 {
        #[doc = "CPUCR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CPUCR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec6 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec6 {
        #[inline(always)]
        fn from(val: u8) -> Sec6 {
            Sec6::from_bits(val)
        }
    }
    impl From<Sec6> for u8 {
        #[inline(always)]
        fn from(val: Sec6) -> u8 {
            Sec6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec7 {
        #[doc = "SVMCR1, SVMCR2, and SVMCR3 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "SVMCR1, SVMCR2, and SVMCR3 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Sec7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec7 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec7 {
        #[inline(always)]
        fn from(val: u8) -> Sec7 {
            Sec7::from_bits(val)
        }
    }
    impl From<Sec7> for u8 {
        #[inline(always)]
        fn from(val: Sec7) -> u8 {
            Sec7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopf {
        #[doc = "System has not been in Stop mode."]
        B_0X0 = 0x0,
        #[doc = "System has been in Stop mode."]
        B_0X1 = 0x01,
    }
    impl Stopf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopf {
        #[inline(always)]
        fn from(val: u8) -> Stopf {
            Stopf::from_bits(val)
        }
    }
    impl From<Stopf> for u8 {
        #[inline(always)]
        fn from(val: Stopf) -> u8 {
            Stopf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Svos {
        #[doc = "SVOS low."]
        B_0X0 = 0x0,
        #[doc = "SVOS high (default)."]
        B_0X1 = 0x01,
    }
    impl Svos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Svos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Svos {
        #[inline(always)]
        fn from(val: u8) -> Svos {
            Svos::from_bits(val)
        }
    }
    impl From<Svos> for u8 {
        #[inline(always)]
        fn from(val: Svos) -> u8 {
            Svos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Temph {
        #[doc = "Temperature below high threshold level."]
        B_0X0 = 0x0,
        #[doc = "Temperature equal or above high threshold level."]
        B_0X1 = 0x01,
    }
    impl Temph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Temph {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Temph {
        #[inline(always)]
        fn from(val: u8) -> Temph {
            Temph::from_bits(val)
        }
    }
    impl From<Temph> for u8 {
        #[inline(always)]
        fn from(val: Temph) -> u8 {
            Temph::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Templ {
        #[doc = "Temperature above low threshold level."]
        B_0X0 = 0x0,
        #[doc = "Temperature equal or below low threshold level."]
        B_0X1 = 0x01,
    }
    impl Templ {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Templ {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Templ {
        #[inline(always)]
        fn from(val: u8) -> Templ {
            Templ::from_bits(val)
        }
    }
    impl From<Templ> for u8 {
        #[inline(always)]
        fn from(val: Templ) -> u8 {
            Templ::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usb33rdy {
        #[doc = "V less than sub>DD33USB less than /sub> is below the threshold of the USB33 voltage monitor."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DD33USB less than /sub> is equal or above the threshold of the USB33 voltage monitor."]
        B_0X1 = 0x01,
    }
    impl Usb33rdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usb33rdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usb33rdy {
        #[inline(always)]
        fn from(val: u8) -> Usb33rdy {
            Usb33rdy::from_bits(val)
        }
    }
    impl From<Usb33rdy> for u8 {
        #[inline(always)]
        fn from(val: Usb33rdy) -> u8 {
            Usb33rdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usb33sv {
        #[doc = "V less than sub>DD33USB less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DD33USB less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Usb33sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usb33sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usb33sv {
        #[inline(always)]
        fn from(val: u8) -> Usb33sv {
            Usb33sv::from_bits(val)
        }
    }
    impl From<Usb33sv> for u8 {
        #[inline(always)]
        fn from(val: Usb33sv) -> u8 {
            Usb33sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbath {
        #[doc = "V less than sub>BAT less than /sub> level below high threshold level."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>BAT less than /sub> level equal or above high threshold level."]
        B_0X1 = 0x01,
    }
    impl Vbath {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbath {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbath {
        #[inline(always)]
        fn from(val: u8) -> Vbath {
            Vbath::from_bits(val)
        }
    }
    impl From<Vbath> for u8 {
        #[inline(always)]
        fn from(val: Vbath) -> u8 {
            Vbath::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbatl {
        #[doc = "V less than sub>BAT less than /sub> level above low threshold level."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>BAT less than /sub> level equal or below low threshold level."]
        B_0X1 = 0x01,
    }
    impl Vbatl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbatl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbatl {
        #[inline(always)]
        fn from(val: u8) -> Vbatl {
            Vbatl::from_bits(val)
        }
    }
    impl From<Vbatl> for u8 {
        #[inline(always)]
        fn from(val: Vbatl) -> u8 {
            Vbatl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vcoreh {
        #[doc = "V less than sub>DDCORE less than /sub> level below high threshold level, or monitor disabled."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDCORE less than /sub> level equal or above high threshold level."]
        B_0X1 = 0x01,
    }
    impl Vcoreh {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vcoreh {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vcoreh {
        #[inline(always)]
        fn from(val: u8) -> Vcoreh {
            Vcoreh::from_bits(val)
        }
    }
    impl From<Vcoreh> for u8 {
        #[inline(always)]
        fn from(val: Vcoreh) -> u8 {
            Vcoreh::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vcorel {
        #[doc = "V less than sub>DDCORE less than /sub> level above low threshold level, or monitor disabled."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDCORE less than /sub> level equal or below low threshold level."]
        B_0X1 = 0x01,
    }
    impl Vcorel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vcorel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vcorel {
        #[inline(always)]
        fn from(val: u8) -> Vcorel {
            Vcorel::from_bits(val)
        }
    }
    impl From<Vcorel> for u8 {
        #[inline(always)]
        fn from(val: Vcorel) -> u8 {
            Vcorel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vcorells {
        #[doc = "V less than sub>DDCORE less than /sub> low-voltage threshold 1 selected (VOS low)."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDCORE less than /sub> low-voltage threshold 2 selected (VOS high)."]
        B_0X1 = 0x01,
    }
    impl Vcorells {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vcorells {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vcorells {
        #[inline(always)]
        fn from(val: u8) -> Vcorells {
            Vcorells::from_bits(val)
        }
    }
    impl From<Vcorells> for u8 {
        #[inline(always)]
        fn from(val: Vcorells) -> u8 {
            Vcorells::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2rdy {
        #[doc = "V less than sub>DDIO2 less than /sub> is below the threshold of the VDDIO2 voltage monitor."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO2 less than /sub> is equal or above the threshold of the VDDIO2 voltage monitor."]
        B_0X1 = 0x01,
    }
    impl Vddio2rdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2rdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2rdy {
        #[inline(always)]
        fn from(val: u8) -> Vddio2rdy {
            Vddio2rdy::from_bits(val)
        }
    }
    impl From<Vddio2rdy> for u8 {
        #[inline(always)]
        fn from(val: Vddio2rdy) -> u8 {
            Vddio2rdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2sv {
        #[doc = "V less than sub>DDIO2 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO2 less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Vddio2sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2sv {
        #[inline(always)]
        fn from(val: u8) -> Vddio2sv {
            Vddio2sv::from_bits(val)
        }
    }
    impl From<Vddio2sv> for u8 {
        #[inline(always)]
        fn from(val: Vddio2sv) -> u8 {
            Vddio2sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio2vrsel {
        #[doc = "3v3 voltage range selected. If V less than sub>DDIO2 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode."]
        B_0X0 = 0x0,
        #[doc = "1v8 voltage range selected. HSLV_VDDIO2 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO2 less than /sub> is in 3v3 range damages the device."]
        B_0X1 = 0x01,
    }
    impl Vddio2vrsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio2vrsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio2vrsel {
        #[inline(always)]
        fn from(val: u8) -> Vddio2vrsel {
            Vddio2vrsel::from_bits(val)
        }
    }
    impl From<Vddio2vrsel> for u8 {
        #[inline(always)]
        fn from(val: Vddio2vrsel) -> u8 {
            Vddio2vrsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3rdy {
        #[doc = "V less than sub>DDIO3 less than /sub> is below the threshold of the VDDIO3 voltage monitor."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO3 less than /sub> is equal or above the threshold of the VDDIO3 voltage monitor."]
        B_0X1 = 0x01,
    }
    impl Vddio3rdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3rdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3rdy {
        #[inline(always)]
        fn from(val: u8) -> Vddio3rdy {
            Vddio3rdy::from_bits(val)
        }
    }
    impl From<Vddio3rdy> for u8 {
        #[inline(always)]
        fn from(val: Vddio3rdy) -> u8 {
            Vddio3rdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3sv {
        #[doc = "V less than sub>DDIO3 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO3 less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Vddio3sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3sv {
        #[inline(always)]
        fn from(val: u8) -> Vddio3sv {
            Vddio3sv::from_bits(val)
        }
    }
    impl From<Vddio3sv> for u8 {
        #[inline(always)]
        fn from(val: Vddio3sv) -> u8 {
            Vddio3sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio3vrsel {
        #[doc = "3v3 voltage range selected. If V less than sub>DDIO3 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode."]
        B_0X0 = 0x0,
        #[doc = "1v8 voltage range selected. HSLV_VDDIO3 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO3 less than /sub> is in 3v3 range damages the device."]
        B_0X1 = 0x01,
    }
    impl Vddio3vrsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio3vrsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio3vrsel {
        #[inline(always)]
        fn from(val: u8) -> Vddio3vrsel {
            Vddio3vrsel::from_bits(val)
        }
    }
    impl From<Vddio3vrsel> for u8 {
        #[inline(always)]
        fn from(val: Vddio3vrsel) -> u8 {
            Vddio3vrsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4rdy {
        #[doc = "V less than sub>DDIO4 less than /sub> is below the threshold of the V less than sub>DDIO4 less than /sub> voltage monitor."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO4 less than /sub> is equal or above the threshold of the V less than sub>DDIO4 less than /sub> voltage monitor."]
        B_0X1 = 0x01,
    }
    impl Vddio4rdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4rdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4rdy {
        #[inline(always)]
        fn from(val: u8) -> Vddio4rdy {
            Vddio4rdy::from_bits(val)
        }
    }
    impl From<Vddio4rdy> for u8 {
        #[inline(always)]
        fn from(val: Vddio4rdy) -> u8 {
            Vddio4rdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4sv {
        #[doc = "V less than sub>DDIO4 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO4 less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Vddio4sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4sv {
        #[inline(always)]
        fn from(val: u8) -> Vddio4sv {
            Vddio4sv::from_bits(val)
        }
    }
    impl From<Vddio4sv> for u8 {
        #[inline(always)]
        fn from(val: Vddio4sv) -> u8 {
            Vddio4sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4vrsel {
        #[doc = "3v3 voltage range selected. If V less than sub>DDIO4 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode."]
        B_0X0 = 0x0,
        #[doc = "1v8 voltage range selected. HSLV_VDDIO4 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO4 less than /sub> is in 3v3 range damages the device."]
        B_0X1 = 0x01,
    }
    impl Vddio4vrsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4vrsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4vrsel {
        #[inline(always)]
        fn from(val: u8) -> Vddio4vrsel {
            Vddio4vrsel::from_bits(val)
        }
    }
    impl From<Vddio4vrsel> for u8 {
        #[inline(always)]
        fn from(val: Vddio4vrsel) -> u8 {
            Vddio4vrsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio4vrstby {
        #[doc = "VDDIO4VRSEL not retained in Standby mode."]
        B_0X0 = 0x0,
        #[doc = "VDDIO4VRSEL retained in Standby mode."]
        B_0X1 = 0x01,
    }
    impl Vddio4vrstby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio4vrstby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio4vrstby {
        #[inline(always)]
        fn from(val: u8) -> Vddio4vrstby {
            Vddio4vrstby::from_bits(val)
        }
    }
    impl From<Vddio4vrstby> for u8 {
        #[inline(always)]
        fn from(val: Vddio4vrstby) -> u8 {
            Vddio4vrstby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio5rdy {
        #[doc = "V less than sub>DDIO5 less than /sub> is below the threshold of the V less than sub>DDIO5 less than /sub> voltage monitor."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO5 less than /sub> is equal or above the threshold of the V less than sub>DDIO5 less than /sub> voltage monitor."]
        B_0X1 = 0x01,
    }
    impl Vddio5rdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio5rdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio5rdy {
        #[inline(always)]
        fn from(val: u8) -> Vddio5rdy {
            Vddio5rdy::from_bits(val)
        }
    }
    impl From<Vddio5rdy> for u8 {
        #[inline(always)]
        fn from(val: Vddio5rdy) -> u8 {
            Vddio5rdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio5sv {
        #[doc = "V less than sub>DDIO5 less than /sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
        B_0X0 = 0x0,
        #[doc = "V less than sub>DDIO5 less than /sub> is valid."]
        B_0X1 = 0x01,
    }
    impl Vddio5sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio5sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio5sv {
        #[inline(always)]
        fn from(val: u8) -> Vddio5sv {
            Vddio5sv::from_bits(val)
        }
    }
    impl From<Vddio5sv> for u8 {
        #[inline(always)]
        fn from(val: Vddio5sv) -> u8 {
            Vddio5sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio5vrsel {
        #[doc = "3v3 voltage range selected. If V less than sub>DDIO5 less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode."]
        B_0X0 = 0x0,
        #[doc = "1v8 voltage range selected. HSLV_VDDIO5 option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DDIO5 less than /sub> is in 3v3 range damages the device."]
        B_0X1 = 0x01,
    }
    impl Vddio5vrsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio5vrsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio5vrsel {
        #[inline(always)]
        fn from(val: u8) -> Vddio5vrsel {
            Vddio5vrsel::from_bits(val)
        }
    }
    impl From<Vddio5vrsel> for u8 {
        #[inline(always)]
        fn from(val: Vddio5vrsel) -> u8 {
            Vddio5vrsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddio5vrstby {
        #[doc = "VDDIO5VRSEL not retained in Standby mode."]
        B_0X0 = 0x0,
        #[doc = "VDDIO5VRSEL retained in Standby mod."]
        B_0X1 = 0x01,
    }
    impl Vddio5vrstby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddio5vrstby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddio5vrstby {
        #[inline(always)]
        fn from(val: u8) -> Vddio5vrstby {
            Vddio5vrstby::from_bits(val)
        }
    }
    impl From<Vddio5vrstby> for u8 {
        #[inline(always)]
        fn from(val: Vddio5vrstby) -> u8 {
            Vddio5vrstby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vddiovrsel {
        #[doc = "3v3 voltage range selected. If V less than sub>DD less than /sub> is in 1v8 range with this setting, I/Os work in degraded mode."]
        B_0X0 = 0x0,
        #[doc = "1v8 voltage range selected. HSLV_VDD option bit must be set to allow 1v8 voltage range operation. Setting this configuration while V less than sub>DD less than /sub> is in 3v3 range damages the device."]
        B_0X1 = 0x01,
    }
    impl Vddiovrsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vddiovrsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vddiovrsel {
        #[inline(always)]
        fn from(val: u8) -> Vddiovrsel {
            Vddiovrsel::from_bits(val)
        }
    }
    impl From<Vddiovrsel> for u8 {
        #[inline(always)]
        fn from(val: Vddiovrsel) -> u8 {
            Vddiovrsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        #[doc = "VOS low level (default)."]
        B_0X0 = 0x0,
        #[doc = "VOS high level."]
        B_0X1 = 0x01,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupc1 {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "Writing 1 clears WKUPF1 in WKUPSR."]
        B_0X1 = 0x01,
    }
    impl Wkupc1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupc1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupc1 {
        #[inline(always)]
        fn from(val: u8) -> Wkupc1 {
            Wkupc1::from_bits(val)
        }
    }
    impl From<Wkupc1> for u8 {
        #[inline(always)]
        fn from(val: Wkupc1) -> u8 {
            Wkupc1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupc2 {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "Writing 1 clears WKUPF2 in WKUPSR."]
        B_0X1 = 0x01,
    }
    impl Wkupc2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupc2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupc2 {
        #[inline(always)]
        fn from(val: u8) -> Wkupc2 {
            Wkupc2::from_bits(val)
        }
    }
    impl From<Wkupc2> for u8 {
        #[inline(always)]
        fn from(val: Wkupc2) -> u8 {
            Wkupc2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupc3 {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "Writing 1 clears WKUPF3 in WKUPSR."]
        B_0X1 = 0x01,
    }
    impl Wkupc3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupc3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupc3 {
        #[inline(always)]
        fn from(val: u8) -> Wkupc3 {
            Wkupc3::from_bits(val)
        }
    }
    impl From<Wkupc3> for u8 {
        #[inline(always)]
        fn from(val: Wkupc3) -> u8 {
            Wkupc3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupc4 {
        #[doc = "No effect."]
        B_0X0 = 0x0,
        #[doc = "Writing 1 clears WKUPF4 in WKUPSR."]
        B_0X1 = 0x01,
    }
    impl Wkupc4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupc4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupc4 {
        #[inline(always)]
        fn from(val: u8) -> Wkupc4 {
            Wkupc4::from_bits(val)
        }
    }
    impl From<Wkupc4> for u8 {
        #[inline(always)]
        fn from(val: Wkupc4) -> u8 {
            Wkupc4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupf1 {
        #[doc = "No wake-up event occurred."]
        B_0X0 = 0x0,
        #[doc = "A wake-up event was received from WKUP1 pin."]
        B_0X1 = 0x01,
    }
    impl Wkupf1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupf1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupf1 {
        #[inline(always)]
        fn from(val: u8) -> Wkupf1 {
            Wkupf1::from_bits(val)
        }
    }
    impl From<Wkupf1> for u8 {
        #[inline(always)]
        fn from(val: Wkupf1) -> u8 {
            Wkupf1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupf2 {
        #[doc = "No wake-up event occurred."]
        B_0X0 = 0x0,
        #[doc = "A wake-up event was received from WKUP2 pin."]
        B_0X1 = 0x01,
    }
    impl Wkupf2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupf2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupf2 {
        #[inline(always)]
        fn from(val: u8) -> Wkupf2 {
            Wkupf2::from_bits(val)
        }
    }
    impl From<Wkupf2> for u8 {
        #[inline(always)]
        fn from(val: Wkupf2) -> u8 {
            Wkupf2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupf3 {
        #[doc = "No wake-up event occurred."]
        B_0X0 = 0x0,
        #[doc = "A wake-up event was received from WKUP3 pin."]
        B_0X1 = 0x01,
    }
    impl Wkupf3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupf3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupf3 {
        #[inline(always)]
        fn from(val: u8) -> Wkupf3 {
            Wkupf3::from_bits(val)
        }
    }
    impl From<Wkupf3> for u8 {
        #[inline(always)]
        fn from(val: Wkupf3) -> u8 {
            Wkupf3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupf4 {
        #[doc = "No wake-up event occurred."]
        B_0X0 = 0x0,
        #[doc = "A wake-up event was received from WKUP4 pin."]
        B_0X1 = 0x01,
    }
    impl Wkupf4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupf4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupf4 {
        #[inline(always)]
        fn from(val: u8) -> Wkupf4 {
            Wkupf4::from_bits(val)
        }
    }
    impl From<Wkupf4> for u8 {
        #[inline(always)]
        fn from(val: Wkupf4) -> u8 {
            Wkupf4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupp1 {
        #[doc = "Detection on high level (rising edge)."]
        B_0X0 = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        B_0X1 = 0x01,
    }
    impl Wkupp1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupp1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupp1 {
        #[inline(always)]
        fn from(val: u8) -> Wkupp1 {
            Wkupp1::from_bits(val)
        }
    }
    impl From<Wkupp1> for u8 {
        #[inline(always)]
        fn from(val: Wkupp1) -> u8 {
            Wkupp1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupp2 {
        #[doc = "Detection on high level (rising edge)."]
        B_0X0 = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        B_0X1 = 0x01,
    }
    impl Wkupp2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupp2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupp2 {
        #[inline(always)]
        fn from(val: u8) -> Wkupp2 {
            Wkupp2::from_bits(val)
        }
    }
    impl From<Wkupp2> for u8 {
        #[inline(always)]
        fn from(val: Wkupp2) -> u8 {
            Wkupp2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupp3 {
        #[doc = "Detection on high level (rising edge)."]
        B_0X0 = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        B_0X1 = 0x01,
    }
    impl Wkupp3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupp3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupp3 {
        #[inline(always)]
        fn from(val: u8) -> Wkupp3 {
            Wkupp3::from_bits(val)
        }
    }
    impl From<Wkupp3> for u8 {
        #[inline(always)]
        fn from(val: Wkupp3) -> u8 {
            Wkupp3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupp4 {
        #[doc = "Detection on high level (rising edge)."]
        B_0X0 = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        B_0X1 = 0x01,
    }
    impl Wkupp4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupp4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupp4 {
        #[inline(always)]
        fn from(val: u8) -> Wkupp4 {
            Wkupp4::from_bits(val)
        }
    }
    impl From<Wkupp4> for u8 {
        #[inline(always)]
        fn from(val: Wkupp4) -> u8 {
            Wkupp4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppriv1 {
        #[doc = "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Wkuppriv1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppriv1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppriv1 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppriv1 {
            Wkuppriv1::from_bits(val)
        }
    }
    impl From<Wkuppriv1> for u8 {
        #[inline(always)]
        fn from(val: Wkuppriv1) -> u8 {
            Wkuppriv1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppriv2 {
        #[doc = "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Wkuppriv2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppriv2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppriv2 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppriv2 {
            Wkuppriv2::from_bits(val)
        }
    }
    impl From<Wkuppriv2> for u8 {
        #[inline(always)]
        fn from(val: Wkuppriv2) -> u8 {
            Wkuppriv2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppriv3 {
        #[doc = "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Wkuppriv3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppriv3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppriv3 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppriv3 {
            Wkuppriv3::from_bits(val)
        }
    }
    impl From<Wkuppriv3> for u8 {
        #[inline(always)]
        fn from(val: Wkuppriv3) -> u8 {
            Wkuppriv3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppriv4 {
        #[doc = "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with privileged access."]
        B_0X1 = 0x01,
    }
    impl Wkuppriv4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppriv4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppriv4 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppriv4 {
            Wkuppriv4::from_bits(val)
        }
    }
    impl From<Wkuppriv4> for u8 {
        #[inline(always)]
        fn from(val: Wkuppriv4) -> u8 {
            Wkuppriv4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd1 {
        #[doc = "No pulls."]
        B_0X0 = 0x0,
        #[doc = "Pull-up."]
        B_0X1 = 0x01,
        #[doc = "Pull-down."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd1 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd1 {
            Wkuppupd1::from_bits(val)
        }
    }
    impl From<Wkuppupd1> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd1) -> u8 {
            Wkuppupd1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd2 {
        #[doc = "No pulls."]
        B_0X0 = 0x0,
        #[doc = "Pull-up."]
        B_0X1 = 0x01,
        #[doc = "Pull-down."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd2 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd2 {
            Wkuppupd2::from_bits(val)
        }
    }
    impl From<Wkuppupd2> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd2) -> u8 {
            Wkuppupd2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd3 {
        #[doc = "No pulls."]
        B_0X0 = 0x0,
        #[doc = "Pull-up."]
        B_0X1 = 0x01,
        #[doc = "Pull-down."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd3 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd3 {
            Wkuppupd3::from_bits(val)
        }
    }
    impl From<Wkuppupd3> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd3) -> u8 {
            Wkuppupd3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd4 {
        #[doc = "No pulls."]
        B_0X0 = 0x0,
        #[doc = "Pull-up."]
        B_0X1 = 0x01,
        #[doc = "Pull-down."]
        B_0X2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd4 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd4 {
            Wkuppupd4::from_bits(val)
        }
    }
    impl From<Wkuppupd4> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd4) -> u8 {
            Wkuppupd4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupsec1 {
        #[doc = "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP1 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Wkupsec1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupsec1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupsec1 {
        #[inline(always)]
        fn from(val: u8) -> Wkupsec1 {
            Wkupsec1::from_bits(val)
        }
    }
    impl From<Wkupsec1> for u8 {
        #[inline(always)]
        fn from(val: Wkupsec1) -> u8 {
            Wkupsec1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupsec2 {
        #[doc = "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP2 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Wkupsec2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupsec2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupsec2 {
        #[inline(always)]
        fn from(val: u8) -> Wkupsec2 {
            Wkupsec2::from_bits(val)
        }
    }
    impl From<Wkupsec2> for u8 {
        #[inline(always)]
        fn from(val: Wkupsec2) -> u8 {
            Wkupsec2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupsec3 {
        #[doc = "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP3 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Wkupsec3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupsec3 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupsec3 {
        #[inline(always)]
        fn from(val: u8) -> Wkupsec3 {
            Wkupsec3::from_bits(val)
        }
    }
    impl From<Wkupsec3> for u8 {
        #[inline(always)]
        fn from(val: Wkupsec3) -> u8 {
            Wkupsec3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkupsec4 {
        #[doc = "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "Bits related to WKUP4 pin in WKUPCR, WKUPSR, and WKUPEPR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Wkupsec4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkupsec4 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkupsec4 {
        #[inline(always)]
        fn from(val: u8) -> Wkupsec4 {
            Wkupsec4::from_bits(val)
        }
    }
    impl From<Wkupsec4> for u8 {
        #[inline(always)]
        fn from(val: Wkupsec4) -> u8 {
            Wkupsec4::to_bits(val)
        }
    }
}
