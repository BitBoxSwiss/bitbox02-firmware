#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control"]
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
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "voltage scaling register"]
    #[inline(always)]
    pub const fn vosr(self) -> crate::common::Reg<regs::Vosr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "supply voltage monitoring control register"]
    #[inline(always)]
    pub const fn svmcr(self) -> crate::common::Reg<regs::Svmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "wakeup control register 1"]
    #[inline(always)]
    pub const fn wucr1(self) -> crate::common::Reg<regs::Wucr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "wakeup control register 2"]
    #[inline(always)]
    pub const fn wucr2(self) -> crate::common::Reg<regs::Wucr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "wakeup control register 3"]
    #[inline(always)]
    pub const fn wucr3(self) -> crate::common::Reg<regs::Wucr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "disable Backup domain register."]
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "security configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "privilege control register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "supply voltage monitoring status register"]
    #[inline(always)]
    pub const fn svmsr(self) -> crate::common::Reg<regs::Svmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "wakeup status register"]
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "wakeup status clear register"]
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "port Standby IO retention enable register"]
    #[inline(always)]
    pub const fn ioretenr(self, n: usize) -> crate::common::Reg<regs::Ioretenr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 8usize) as _) }
    }
    #[doc = "port Standby IO retention status register"]
    #[inline(always)]
    pub const fn ioretr(self, n: usize) -> crate::common::Reg<regs::Ioretr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize + n * 8usize) as _) }
    }
    #[doc = "2.4 GHz RADIO status and control register"]
    #[inline(always)]
    pub const fn radioscr(self) -> crate::common::Reg<regs::Radioscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Stop 2 peripheral IOs retention register"]
    #[inline(always)]
    pub const fn s2retr(self) -> crate::common::Reg<regs::S2retr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode."]
        #[inline(always)]
        pub const fn r2rsb1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode."]
        #[inline(always)]
        pub fn set_r2rsb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN."]
        #[inline(always)]
        pub const fn ulpmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN."]
        #[inline(always)]
        pub fn set_ulpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational."]
        #[inline(always)]
        pub const fn radiorsb(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational."]
        #[inline(always)]
        pub fn set_radiorsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM1 192 KB page 5 to 7 retention in Standby mode Used to keep SRAM1 page 5 to 7 content in Standby retention mode."]
        #[inline(always)]
        pub const fn r1rsb567(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 192 KB page 5 to 7 retention in Standby mode Used to keep SRAM1 page 5 to 7 content in Standby retention mode."]
        #[inline(always)]
        pub fn set_r1rsb567(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SRAM1 page X retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode."]
        #[inline(always)]
        pub const fn r1rsb(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 page X retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode."]
        #[inline(always)]
        pub fn set_r1rsb(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("lpms", &self.lpms())
                .field("r2rsb1", &self.r2rsb1())
                .field("ulpmen", &self.ulpmen())
                .field("radiorsb", &self.radiorsb())
                .field("r1rsb567", &self.r1rsb567())
                .field("r1rsb[0]", &self.r1rsb(0usize))
                .field("r1rsb[1]", &self.r1rsb(1usize))
                .field("r1rsb[2]", &self.r1rsb(2usize))
                .field("r1rsb[3]", &self.r1rsb(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpms: {:?}, r2rsb1: {=bool:?}, ulpmen: {=bool:?}, radiorsb: {=bool:?}, r1rsb567: {=bool:?}, r1rsb[0]: {=bool:?}, r1rsb[1]: {=bool:?}, r1rsb[2]: {=bool:?}, r1rsb[3]: {=bool:?} }}" , self . lpms () , self . r2rsb1 () , self . ulpmen () , self . radiorsb () , self . r1rsb567 () , self . r1rsb (0usize) , self . r1rsb (1usize) , self . r1rsb (2usize) , self . r1rsb (3usize))
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "SRAM1 page X power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSBX bit in CR1."]
        #[inline(always)]
        pub const fn sram1pds(&self, n: usize) -> super::vals::Srampds {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "SRAM1 page X power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSBX bit in CR1."]
        #[inline(always)]
        pub fn set_sram1pds(&mut self, n: usize, val: super::vals::Srampds) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in CR1."]
        #[inline(always)]
        pub const fn sram2pds1(&self) -> super::vals::Srampds {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in CR1."]
        #[inline(always)]
        pub fn set_sram2pds1(&mut self, val: super::vals::Srampds) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM1 192KB, page 5 to 7 power-down in Stop modes"]
        #[inline(always)]
        pub const fn sram1pds567(&self) -> super::vals::Sram1pds567 {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Sram1pds567::from_bits(val as u8)
        }
        #[doc = "SRAM1 192KB, page 5 to 7 power-down in Stop modes"]
        #[inline(always)]
        pub fn set_sram1pds567(&mut self, val: super::vals::Sram1pds567) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "ICACHE SRAM power-down in Stop modes (Stop 0, 1)"]
        #[inline(always)]
        pub const fn icrampds(&self) -> super::vals::Icrampds {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Icrampds::from_bits(val as u8)
        }
        #[doc = "ICACHE SRAM power-down in Stop modes (Stop 0, 1)"]
        #[inline(always)]
        pub fn set_icrampds(&mut self, val: super::vals::Icrampds) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "OTG SRAM power-down in Stop modes."]
        #[inline(always)]
        pub const fn prampds(&self) -> super::vals::Prampds {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Prampds::from_bits(val as u8)
        }
        #[doc = "OTG SRAM power-down in Stop modes."]
        #[inline(always)]
        pub fn set_prampds(&mut self, val: super::vals::Prampds) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "PKA SRAM power-down in Stop modes."]
        #[inline(always)]
        pub const fn pkarampds(&self) -> super::vals::Pkarampds {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Pkarampds::from_bits(val as u8)
        }
        #[doc = "PKA SRAM power-down in Stop modes."]
        #[inline(always)]
        pub fn set_pkarampds(&mut self, val: super::vals::Pkarampds) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
        #[inline(always)]
        pub const fn flashfwu(&self) -> super::vals::Flashfwu {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Flashfwu::from_bits(val as u8)
        }
        #[doc = "Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption."]
        #[inline(always)]
        pub fn set_flashfwu(&mut self, val: super::vals::Flashfwu) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
                .field("sram1pds[0]", &self.sram1pds(0usize))
                .field("sram1pds[1]", &self.sram1pds(1usize))
                .field("sram1pds[2]", &self.sram1pds(2usize))
                .field("sram1pds[3]", &self.sram1pds(3usize))
                .field("sram2pds1", &self.sram2pds1())
                .field("sram1pds567", &self.sram1pds567())
                .field("icrampds", &self.icrampds())
                .field("prampds", &self.prampds())
                .field("pkarampds", &self.pkarampds())
                .field("flashfwu", &self.flashfwu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ sram1pds[0]: {:?}, sram1pds[1]: {:?}, sram1pds[2]: {:?}, sram1pds[3]: {:?}, sram2pds1: {:?}, sram1pds567: {:?}, icrampds: {:?}, prampds: {:?}, pkarampds: {:?}, flashfwu: {:?} }}" , self . sram1pds (0usize) , self . sram1pds (1usize) , self . sram1pds (2usize) , self . sram1pds (3usize) , self . sram2pds1 () , self . sram1pds567 () , self . icrampds () , self . prampds () , self . pkarampds () , self . flashfwu ())
        }
    }
    #[doc = "control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Regulator selection."]
        #[inline(always)]
        pub const fn regsel(&self) -> super::vals::Regsel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regsel::from_bits(val as u8)
        }
        #[doc = "Regulator selection."]
        #[inline(always)]
        pub fn set_regsel(&mut self, val: super::vals::Regsel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Fast soft start"]
        #[inline(always)]
        pub const fn fsten(&self) -> super::vals::Fsten {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Fsten::from_bits(val as u8)
        }
        #[doc = "Fast soft start"]
        #[inline(always)]
        pub fn set_fsten(&mut self, val: super::vals::Fsten) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Low power mode regulator clock division."]
        #[inline(always)]
        pub const fn divclp(&self) -> super::vals::Divclp {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Divclp::from_bits(val as u8)
        }
        #[doc = "Low power mode regulator clock division."]
        #[inline(always)]
        pub fn set_divclp(&mut self, val: super::vals::Divclp) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Low power mode regulator replica selection."]
        #[inline(always)]
        pub const fn selrep(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Low power mode regulator replica selection."]
        #[inline(always)]
        pub fn set_selrep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "V11 feedback switch enable (non user bit)."]
        #[inline(always)]
        pub const fn v11fbsw(&self) -> super::vals::V11fbsw {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::V11fbsw::from_bits(val as u8)
        }
        #[doc = "V11 feedback switch enable (non user bit)."]
        #[inline(always)]
        pub fn set_v11fbsw(&mut self, val: super::vals::V11fbsw) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
                .field("regsel", &self.regsel())
                .field("fsten", &self.fsten())
                .field("divclp", &self.divclp())
                .field("selrep", &self.selrep())
                .field("v11fbsw", &self.v11fbsw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ regsel: {:?}, fsten: {:?}, divclp: {:?}, selrep: {=u8:?}, v11fbsw: {:?} }}",
                self.regsel(),
                self.fsten(),
                self.divclp(),
                self.selrep(),
                self.v11fbsw()
            )
        }
    }
    #[doc = "disable Backup domain register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers."]
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
    #[doc = "port A Standby IO retention enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioretenr(pub u32);
    impl Ioretenr {
        #[doc = "Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy"]
        #[inline(always)]
        pub const fn en(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy"]
        #[inline(always)]
        pub fn set_en(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ioretenr {
        #[inline(always)]
        fn default() -> Ioretenr {
            Ioretenr(0)
        }
    }
    impl core::fmt::Debug for Ioretenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ioretenr")
                .field("en[0]", &self.en(0usize))
                .field("en[1]", &self.en(1usize))
                .field("en[2]", &self.en(2usize))
                .field("en[3]", &self.en(3usize))
                .field("en[4]", &self.en(4usize))
                .field("en[5]", &self.en(5usize))
                .field("en[6]", &self.en(6usize))
                .field("en[7]", &self.en(7usize))
                .field("en[8]", &self.en(8usize))
                .field("en[9]", &self.en(9usize))
                .field("en[10]", &self.en(10usize))
                .field("en[11]", &self.en(11usize))
                .field("en[12]", &self.en(12usize))
                .field("en[13]", &self.en(13usize))
                .field("en[14]", &self.en(14usize))
                .field("en[15]", &self.en(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ioretenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ioretenr {{ en[0]: {=bool:?}, en[1]: {=bool:?}, en[2]: {=bool:?}, en[3]: {=bool:?}, en[4]: {=bool:?}, en[5]: {=bool:?}, en[6]: {=bool:?}, en[7]: {=bool:?}, en[8]: {=bool:?}, en[9]: {=bool:?}, en[10]: {=bool:?}, en[11]: {=bool:?}, en[12]: {=bool:?}, en[13]: {=bool:?}, en[14]: {=bool:?}, en[15]: {=bool:?} }}" , self . en (0usize) , self . en (1usize) , self . en (2usize) , self . en (3usize) , self . en (4usize) , self . en (5usize) , self . en (6usize) , self . en (7usize) , self . en (8usize) , self . en (9usize) , self . en (10usize) , self . en (11usize) , self . en (12usize) , self . en (13usize) , self . en (14usize) , self . en (15usize))
        }
    }
    #[doc = "port A Standby IO retention status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioretr(pub u32);
    impl Ioretr {
        #[doc = "Port A Standby GPIO retention active Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV."]
        #[inline(always)]
        pub const fn ret(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port A Standby GPIO retention active Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV."]
        #[inline(always)]
        pub fn set_ret(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ioretr {
        #[inline(always)]
        fn default() -> Ioretr {
            Ioretr(0)
        }
    }
    impl core::fmt::Debug for Ioretr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ioretr")
                .field("ret[0]", &self.ret(0usize))
                .field("ret[1]", &self.ret(1usize))
                .field("ret[2]", &self.ret(2usize))
                .field("ret[3]", &self.ret(3usize))
                .field("ret[4]", &self.ret(4usize))
                .field("ret[5]", &self.ret(5usize))
                .field("ret[6]", &self.ret(6usize))
                .field("ret[7]", &self.ret(7usize))
                .field("ret[8]", &self.ret(8usize))
                .field("ret[9]", &self.ret(9usize))
                .field("ret[10]", &self.ret(10usize))
                .field("ret[11]", &self.ret(11usize))
                .field("ret[12]", &self.ret(12usize))
                .field("ret[13]", &self.ret(13usize))
                .field("ret[14]", &self.ret(14usize))
                .field("ret[15]", &self.ret(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ioretr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ioretr {{ ret[0]: {=bool:?}, ret[1]: {=bool:?}, ret[2]: {=bool:?}, ret[3]: {=bool:?}, ret[4]: {=bool:?}, ret[5]: {=bool:?}, ret[6]: {=bool:?}, ret[7]: {=bool:?}, ret[8]: {=bool:?}, ret[9]: {=bool:?}, ret[10]: {=bool:?}, ret[11]: {=bool:?}, ret[12]: {=bool:?}, ret[13]: {=bool:?}, ret[14]: {=bool:?}, ret[15]: {=bool:?} }}" , self . ret (0usize) , self . ret (1usize) , self . ret (2usize) , self . ret (3usize) , self . ret (4usize) , self . ret (5usize) , self . ret (6usize) , self . ret (7usize) , self . ret (8usize) , self . ret (9usize) , self . ret (10usize) , self . ret (11usize) , self . ret (12usize) , self . ret (13usize) , self . ret (14usize) , self . ret (15usize))
        }
    }
    #[doc = "privilege control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radioscr(pub u32);
    impl Radioscr {
        #[doc = "2.4 GHz RADIO operating mode. 1x: 2.4 GHz RADIO active mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "2.4 GHz RADIO operating mode. 1x: 2.4 GHz RADIO active mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "2.4 GHz RADIO PHY operating mode"]
        #[inline(always)]
        pub const fn phymode(&self) -> super::vals::Phymode {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Phymode::from_bits(val as u8)
        }
        #[doc = "2.4 GHz RADIO PHY operating mode"]
        #[inline(always)]
        pub fn set_phymode(&mut self, val: super::vals::Phymode) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "2.4 GHz RADIO encryption function operating mode"]
        #[inline(always)]
        pub const fn encmode(&self) -> super::vals::Encmode {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Encmode::from_bits(val as u8)
        }
        #[doc = "2.4 GHz RADIO encryption function operating mode"]
        #[inline(always)]
        pub fn set_encmode(&mut self, val: super::vals::Encmode) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "2.4 GHz RADIO VDDHPA control word. Bits \\[3:0\\]
see Table 81: PA output power table format for definition. Bit \\[4\\]
rf_event."]
        #[inline(always)]
        pub const fn rfvddhpa(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "2.4 GHz RADIO VDDHPA control word. Bits \\[3:0\\]
see Table 81: PA output power table format for definition. Bit \\[4\\]
rf_event."]
        #[inline(always)]
        pub fn set_rfvddhpa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Ready bit for Vless thansub DDHPAless than/sub voltage level when selecting VDD11 input."]
        #[inline(always)]
        pub const fn regpardyv11(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for Vless thansub DDHPAless than/sub voltage level when selecting VDD11 input."]
        #[inline(always)]
        pub fn set_regpardyv11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ready bit for V<sub>DDHPA</sub> voltage level when selecting VDDRFPA input. Note: REGPARDYVDDRFPA does not allow to detect correct V<sub>DDHPA</sub> voltage level when request to lower the level."]
        #[inline(always)]
        pub const fn regpardyvddrfpa(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for V<sub>DDHPA</sub> voltage level when selecting VDDRFPA input. Note: REGPARDYVDDRFPA does not allow to detect correct V<sub>DDHPA</sub> voltage level when request to lower the level."]
        #[inline(always)]
        pub fn set_regpardyvddrfpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "regulator REG_VDDHPA input supply selection."]
        #[inline(always)]
        pub const fn regpasel(&self) -> super::vals::Regpasel {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Regpasel::from_bits(val as u8)
        }
        #[doc = "regulator REG_VDDHPA input supply selection."]
        #[inline(always)]
        pub fn set_regpasel(&mut self, val: super::vals::Regpasel) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "regulator REG_VDDHPA bypass enable."]
        #[inline(always)]
        pub const fn regpabypen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "regulator REG_VDDHPA bypass enable."]
        #[inline(always)]
        pub fn set_regpabypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Radioscr {
        #[inline(always)]
        fn default() -> Radioscr {
            Radioscr(0)
        }
    }
    impl core::fmt::Debug for Radioscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Radioscr")
                .field("mode", &self.mode())
                .field("phymode", &self.phymode())
                .field("encmode", &self.encmode())
                .field("rfvddhpa", &self.rfvddhpa())
                .field("regpardyv11", &self.regpardyv11())
                .field("regpardyvddrfpa", &self.regpardyvddrfpa())
                .field("regpasel", &self.regpasel())
                .field("regpabypen", &self.regpabypen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Radioscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Radioscr {{ mode: {:?}, phymode: {:?}, encmode: {:?}, rfvddhpa: {=u8:?}, regpardyv11: {=bool:?}, regpardyvddrfpa: {=bool:?}, regpasel: {:?}, regpabypen: {=bool:?} }}" , self . mode () , self . phymode () , self . encmode () , self . rfvddhpa () , self . regpardyv11 () , self . regpardyvddrfpa () , self . regpasel () , self . regpabypen ())
        }
    }
    #[doc = "Stop 2 peripheral IOs retention register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2retr(pub u32);
    impl S2retr {
        #[doc = "PTA output signals Stop 2 mode retention enable Access can be secured by GTZC_TZSC PTACONVSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn ptasren(&self) -> super::vals::Ptasren {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ptasren::from_bits(val as u8)
        }
        #[doc = "PTA output signals Stop 2 mode retention enable Access can be secured by GTZC_TZSC PTACONVSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_ptasren(&mut self, val: super::vals::Ptasren) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "PTA interface output signals state retention in Stop 2 mode active Access can be secured by GTZC_TZSC PTACONVSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn ptasr(&self) -> super::vals::Ptasr {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Ptasr::from_bits(val as u8)
        }
        #[doc = "PTA interface output signals state retention in Stop 2 mode active Access can be secured by GTZC_TZSC PTACONVSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_ptasr(&mut self, val: super::vals::Ptasr) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for S2retr {
        #[inline(always)]
        fn default() -> S2retr {
            S2retr(0)
        }
    }
    impl core::fmt::Debug for S2retr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2retr")
                .field("ptasren", &self.ptasren())
                .field("ptasr", &self.ptasr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2retr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "S2retr {{ ptasren: {:?}, ptasr: {:?} }}",
                self.ptasren(),
                self.ptasr()
            )
        }
    }
    #[doc = "security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "WUP1 secure protection"]
        #[inline(always)]
        pub const fn wup1sec(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "WUP1 secure protection"]
        #[inline(always)]
        pub fn set_wup1sec(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Low-power modes secure protection"]
        #[inline(always)]
        pub const fn lpmsec(&self) -> super::vals::Lpmsec {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Lpmsec::from_bits(val as u8)
        }
        #[doc = "Low-power modes secure protection"]
        #[inline(always)]
        pub fn set_lpmsec(&mut self, val: super::vals::Lpmsec) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Voltage detection secure protection"]
        #[inline(always)]
        pub const fn vdmsec(&self) -> super::vals::Vdmsec {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Vdmsec::from_bits(val as u8)
        }
        #[doc = "Voltage detection secure protection"]
        #[inline(always)]
        pub fn set_vdmsec(&mut self, val: super::vals::Vdmsec) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Backup domain secure protection"]
        #[inline(always)]
        pub const fn vbsec(&self) -> super::vals::Vbsec {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Vbsec::from_bits(val as u8)
        }
        #[doc = "Backup domain secure protection"]
        #[inline(always)]
        pub fn set_vbsec(&mut self, val: super::vals::Vbsec) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
                .field("wup1sec[0]", &self.wup1sec(0usize))
                .field("wup1sec[1]", &self.wup1sec(1usize))
                .field("wup1sec[2]", &self.wup1sec(2usize))
                .field("wup1sec[3]", &self.wup1sec(3usize))
                .field("wup1sec[4]", &self.wup1sec(4usize))
                .field("wup1sec[5]", &self.wup1sec(5usize))
                .field("wup1sec[6]", &self.wup1sec(6usize))
                .field("wup1sec[7]", &self.wup1sec(7usize))
                .field("lpmsec", &self.lpmsec())
                .field("vdmsec", &self.vdmsec())
                .field("vbsec", &self.vbsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ wup1sec[0]: {=bool:?}, wup1sec[1]: {=bool:?}, wup1sec[2]: {=bool:?}, wup1sec[3]: {=bool:?}, wup1sec[4]: {=bool:?}, wup1sec[5]: {=bool:?}, wup1sec[6]: {=bool:?}, wup1sec[7]: {=bool:?}, lpmsec: {:?}, vdmsec: {:?}, vbsec: {:?} }}" , self . wup1sec (0usize) , self . wup1sec (1usize) , self . wup1sec (2usize) , self . wup1sec (3usize) , self . wup1sec (4usize) , self . wup1sec (5usize) , self . wup1sec (6usize) , self . wup1sec (7usize) , self . lpmsec () , self . vdmsec () , self . vbsec ())
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Clear Stop and Standby flags Access can be secured by LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Stop and Standby flags Access can be secured by LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. Writing 1 to this bit clears the STOPF and SBF flags."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop flag This bit is set by hardware when the device enters a Stop or Standby mode at the same time as the sysclk has been set by hardware to select HSI. It’s cleared by software by writing 1 to the CSSF bit and by hardware when SBF is set."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop flag This bit is set by hardware when the device enters a Stop or Standby mode at the same time as the sysclk has been set by hardware to select HSI. It’s cleared by software by writing 1 to the CSSF bit and by hardware when SBF is set."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and the CPU restart from its reset vector. It’s cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn sbf(&self) -> super::vals::Sbf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Sbf::from_bits(val as u8)
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and the CPU restart from its reset vector. It’s cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: super::vals::Sbf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Stop 2 mode peripherals power down flag."]
        #[inline(always)]
        pub const fn stop2f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Stop 2 mode peripherals power down flag."]
        #[inline(always)]
        pub fn set_stop2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("cssf", &self.cssf())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .field("stop2f", &self.stop2f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ cssf: {=bool:?}, stopf: {=bool:?}, sbf: {:?}, stop2f: {=bool:?} }}",
                self.cssf(),
                self.stopf(),
                self.sbf(),
                self.stop2f()
            )
        }
    }
    #[doc = "supply voltage monitoring control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmcr(pub u32);
    impl Svmcr {
        #[doc = "Programmable voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:"]
        #[inline(always)]
        pub const fn pvdls(&self) -> super::vals::Pvdls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pvdls::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector level selection These bits select the voltage threshold detected by the programmable voltage detector:"]
        #[inline(always)]
        pub fn set_pvdls(&mut self, val: super::vals::Pvdls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "VDDUSB supply valid."]
        #[inline(always)]
        pub const fn usv(&self) -> super::vals::Usv {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Usv::from_bits(val as u8)
        }
        #[doc = "VDDUSB supply valid."]
        #[inline(always)]
        pub fn set_usv(&mut self, val: super::vals::Usv) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "VDDIO2 supply valid."]
        #[inline(always)]
        pub const fn io2sv(&self) -> super::vals::Io2sv {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Io2sv::from_bits(val as u8)
        }
        #[doc = "VDDIO2 supply valid."]
        #[inline(always)]
        pub fn set_io2sv(&mut self, val: super::vals::Io2sv) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Svmcr {
        #[inline(always)]
        fn default() -> Svmcr {
            Svmcr(0)
        }
    }
    impl core::fmt::Debug for Svmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmcr")
                .field("pvde", &self.pvde())
                .field("pvdls", &self.pvdls())
                .field("usv", &self.usv())
                .field("io2sv", &self.io2sv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Svmcr {{ pvde: {=bool:?}, pvdls: {:?}, usv: {:?}, io2sv: {:?} }}",
                self.pvde(),
                self.pvdls(),
                self.usv(),
                self.io2sv()
            )
        }
    }
    #[doc = "supply voltage monitoring status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Svmsr(pub u32);
    impl Svmsr {
        #[doc = "Regulator selection."]
        #[inline(always)]
        pub const fn regs(&self) -> super::vals::Regs {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Regs::from_bits(val as u8)
        }
        #[doc = "Regulator selection."]
        #[inline(always)]
        pub fn set_regs(&mut self, val: super::vals::Regs) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Programmable voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> super::vals::Actvosrdy {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Actvosrdy::from_bits(val as u8)
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: super::vals::Actvosrdy) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "VOS currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Actvos {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Actvos::from_bits(val as u8)
        }
        #[doc = "VOS currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: super::vals::Actvos) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Svmsr {
        #[inline(always)]
        fn default() -> Svmsr {
            Svmsr(0)
        }
    }
    impl core::fmt::Debug for Svmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Svmsr")
                .field("regs", &self.regs())
                .field("pvdo", &self.pvdo())
                .field("actvosrdy", &self.actvosrdy())
                .field("actvos", &self.actvos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Svmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Svmsr {{ regs: {:?}, pvdo: {:?}, actvosrdy: {:?}, actvos: {:?} }}",
                self.regs(),
                self.pvdo(),
                self.actvosrdy(),
                self.actvos()
            )
        }
    }
    #[doc = "voltage scaling register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vosr(pub u32);
    impl Vosr {
        #[doc = "USB OTG VDD11USB ready."]
        #[inline(always)]
        pub const fn vdd11usbrdy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG VDD11USB ready."]
        #[inline(always)]
        pub fn set_vdd11usbrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USB OTG booster ready."]
        #[inline(always)]
        pub const fn usbboostrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG booster ready."]
        #[inline(always)]
        pub fn set_usbboostrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Ready bit for V<sub>CORE</sub> voltage scaling output selection Set and cleared by hardware. When decreasing the voltage scaling range, VOSRDY must be one before increasing the SYSCLK frequency."]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for V<sub>CORE</sub> voltage scaling output selection Set and cleared by hardware. When decreasing the voltage scaling range, VOSRDY must be one before increasing the SYSCLK frequency."]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "USB OTG power enable."]
        #[inline(always)]
        pub const fn usbpwren(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG power enable."]
        #[inline(always)]
        pub fn set_usbpwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USB OTG booster enable."]
        #[inline(always)]
        pub const fn usbboosten(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG booster enable."]
        #[inline(always)]
        pub fn set_usbboosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "USB OTG VDD11USB disable."]
        #[inline(always)]
        pub const fn vdd11usbdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG VDD11USB disable."]
        #[inline(always)]
        pub fn set_vdd11usbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USB OTG VDD11USB switch delay."]
        #[inline(always)]
        pub const fn vdd11usbswdly(&self) -> u16 {
            let val = (self.0 >> 22usize) & 0x03ff;
            val as u16
        }
        #[doc = "USB OTG VDD11USB switch delay."]
        #[inline(always)]
        pub fn set_vdd11usbswdly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
        }
    }
    impl Default for Vosr {
        #[inline(always)]
        fn default() -> Vosr {
            Vosr(0)
        }
    }
    impl core::fmt::Debug for Vosr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vosr")
                .field("vdd11usbrdy", &self.vdd11usbrdy())
                .field("usbboostrdy", &self.usbboostrdy())
                .field("vosrdy", &self.vosrdy())
                .field("vos", &self.vos())
                .field("usbpwren", &self.usbpwren())
                .field("usbboosten", &self.usbboosten())
                .field("vdd11usbdis", &self.vdd11usbdis())
                .field("vdd11usbswdly", &self.vdd11usbswdly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vosr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vosr {{ vdd11usbrdy: {=bool:?}, usbboostrdy: {=bool:?}, vosrdy: {=bool:?}, vos: {:?}, usbpwren: {=bool:?}, usbboosten: {=bool:?}, vdd11usbdis: {=bool:?}, vdd11usbswdly: {=u16:?} }}" , self . vdd11usbrdy () , self . usbboostrdy () , self . vosrdy () , self . vos () , self . usbpwren () , self . usbboosten () , self . vdd11usbdis () , self . vdd11usbswdly ())
        }
    }
    #[doc = "wake-up control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr1(pub u32);
    impl Wucr1 {
        #[doc = "Wakeup and interrupt pin WKUP1 enable Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn wupen(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup and interrupt pin WKUP1 enable Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_wupen(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wucr1 {
        #[inline(always)]
        fn default() -> Wucr1 {
            Wucr1(0)
        }
    }
    impl core::fmt::Debug for Wucr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr1")
                .field("wupen[0]", &self.wupen(0usize))
                .field("wupen[1]", &self.wupen(1usize))
                .field("wupen[2]", &self.wupen(2usize))
                .field("wupen[3]", &self.wupen(3usize))
                .field("wupen[4]", &self.wupen(4usize))
                .field("wupen[5]", &self.wupen(5usize))
                .field("wupen[6]", &self.wupen(6usize))
                .field("wupen[7]", &self.wupen(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr1 {{ wupen[0]: {=bool:?}, wupen[1]: {=bool:?}, wupen[2]: {=bool:?}, wupen[3]: {=bool:?}, wupen[4]: {=bool:?}, wupen[5]: {=bool:?}, wupen[6]: {=bool:?}, wupen[7]: {=bool:?} }}" , self . wupen (0usize) , self . wupen (1usize) , self . wupen (2usize) , self . wupen (3usize) , self . wupen (4usize) , self . wupen (5usize) , self . wupen (6usize) , self . wupen (7usize))
        }
    }
    #[doc = "wake-up control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr2(pub u32);
    impl Wucr2 {
        #[doc = "Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> super::vals::Wupp {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_wupp(&mut self, n: usize, val: super::vals::Wupp) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Wucr2 {
        #[inline(always)]
        fn default() -> Wucr2 {
            Wucr2(0)
        }
    }
    impl core::fmt::Debug for Wucr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr2")
                .field("wupp[0]", &self.wupp(0usize))
                .field("wupp[1]", &self.wupp(1usize))
                .field("wupp[2]", &self.wupp(2usize))
                .field("wupp[3]", &self.wupp(3usize))
                .field("wupp[4]", &self.wupp(4usize))
                .field("wupp[5]", &self.wupp(5usize))
                .field("wupp[6]", &self.wupp(6usize))
                .field("wupp[7]", &self.wupp(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr2 {{ wupp[0]: {:?}, wupp[1]: {:?}, wupp[2]: {:?}, wupp[3]: {:?}, wupp[4]: {:?}, wupp[5]: {:?}, wupp[6]: {:?}, wupp[7]: {:?} }}" , self . wupp (0usize) , self . wupp (1usize) , self . wupp (2usize) , self . wupp (3usize) , self . wupp (4usize) , self . wupp (5usize) , self . wupp (6usize) , self . wupp (7usize))
        }
    }
    #[doc = "wake-up control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr3(pub u32);
    impl Wucr3 {
        #[doc = "Wakeup and interrupt pin WKUPX selection This field must be configured when WUPENX = 0. Access can be secured by WUPXSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn wusel(&self, n: usize) -> super::vals::Wusel {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wusel::from_bits(val as u8)
        }
        #[doc = "Wakeup and interrupt pin WKUPX selection This field must be configured when WUPENX = 0. Access can be secured by WUPXSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_wusel(&mut self, n: usize, val: super::vals::Wusel) {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Wucr3 {
        #[inline(always)]
        fn default() -> Wucr3 {
            Wucr3(0)
        }
    }
    impl core::fmt::Debug for Wucr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr3")
                .field("wusel[0]", &self.wusel(0usize))
                .field("wusel[1]", &self.wusel(1usize))
                .field("wusel[2]", &self.wusel(2usize))
                .field("wusel[3]", &self.wusel(3usize))
                .field("wusel[4]", &self.wusel(4usize))
                .field("wusel[5]", &self.wusel(5usize))
                .field("wusel[6]", &self.wusel(6usize))
                .field("wusel[7]", &self.wusel(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr3 {{ wusel[0]: {:?}, wusel[1]: {:?}, wusel[2]: {:?}, wusel[3]: {:?}, wusel[4]: {:?}, wusel[5]: {:?}, wusel[6]: {:?}, wusel[7]: {:?} }}" , self . wusel (0usize) , self . wusel (1usize) , self . wusel (2usize) , self . wusel (3usize) , self . wusel (4usize) , self . wusel (5usize) , self . wusel (6usize) , self . wusel (7usize))
        }
    }
    #[doc = "wake-up status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "Clear wakeup flag 1 Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 1 Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. Writing 1 to this bit clears the WUF1 flag in WUSR."]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wuscr {
        #[inline(always)]
        fn default() -> Wuscr {
            Wuscr(0)
        }
    }
    impl core::fmt::Debug for Wuscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wuscr")
                .field("cwuf[0]", &self.cwuf(0usize))
                .field("cwuf[1]", &self.cwuf(1usize))
                .field("cwuf[2]", &self.cwuf(2usize))
                .field("cwuf[3]", &self.cwuf(3usize))
                .field("cwuf[4]", &self.cwuf(4usize))
                .field("cwuf[5]", &self.cwuf(5usize))
                .field("cwuf[6]", &self.cwuf(6usize))
                .field("cwuf[7]", &self.cwuf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wuscr {{ cwuf[0]: {=bool:?}, cwuf[1]: {=bool:?}, cwuf[2]: {=bool:?}, cwuf[3]: {=bool:?}, cwuf[4]: {=bool:?}, cwuf[5]: {=bool:?}, cwuf[6]: {=bool:?}, cwuf[7]: {=bool:?} }}" , self . cwuf (0usize) , self . cwuf (1usize) , self . cwuf (2usize) , self . cwuf (3usize) , self . cwuf (4usize) , self . cwuf (5usize) , self . cwuf (6usize) , self . cwuf (7usize))
        }
    }
    #[doc = "wake-up status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "Wakeup and interrupt pending flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR or by hardware when WUPEN1 = 0."]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup and interrupt pending flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR or by hardware when WUPEN1 = 0."]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wusr {
        #[inline(always)]
        fn default() -> Wusr {
            Wusr(0)
        }
    }
    impl core::fmt::Debug for Wusr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wusr")
                .field("wuf[0]", &self.wuf(0usize))
                .field("wuf[1]", &self.wuf(1usize))
                .field("wuf[2]", &self.wuf(2usize))
                .field("wuf[3]", &self.wuf(3usize))
                .field("wuf[4]", &self.wuf(4usize))
                .field("wuf[5]", &self.wuf(5usize))
                .field("wuf[6]", &self.wuf(6usize))
                .field("wuf[7]", &self.wuf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wusr {{ wuf[0]: {=bool:?}, wuf[1]: {=bool:?}, wuf[2]: {=bool:?}, wuf[3]: {=bool:?}, wuf[4]: {=bool:?}, wuf[5]: {=bool:?}, wuf[6]: {=bool:?}, wuf[7]: {=bool:?} }}" , self . wuf (0usize) , self . wuf (1usize) , self . wuf (2usize) , self . wuf (3usize) , self . wuf (4usize) , self . wuf (5usize) , self . wuf (6usize) , self . wuf (7usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Actvos {
        #[doc = "Range 2 (lowest power)"]
        RANGE2 = 0x0,
        #[doc = "Range 1 (highest frequency)"]
        RANGE1 = 0x01,
    }
    impl Actvos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Actvos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Actvos {
        #[inline(always)]
        fn from(val: u8) -> Actvos {
            Actvos::from_bits(val)
        }
    }
    impl From<Actvos> for u8 {
        #[inline(always)]
        fn from(val: Actvos) -> u8 {
            Actvos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Actvosrdy {
        #[doc = "VsubCORE/sub is above or below the current voltage scaling provided by ACTVOS."]
        B_0X0 = 0x0,
        #[doc = "VsubCORE /subis equal to the current voltage scaling provided by ACTVOS."]
        B_0X1 = 0x01,
    }
    impl Actvosrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Actvosrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Actvosrdy {
        #[inline(always)]
        fn from(val: u8) -> Actvosrdy {
            Actvosrdy::from_bits(val)
        }
    }
    impl From<Actvosrdy> for u8 {
        #[inline(always)]
        fn from(val: Actvosrdy) -> u8 {
            Actvosrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Divclp {
        #[doc = "Low power regulator clock not divided."]
        B_0X0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Divclp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Divclp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Divclp {
        #[inline(always)]
        fn from(val: u8) -> Divclp {
            Divclp::from_bits(val)
        }
    }
    impl From<Divclp> for u8 {
        #[inline(always)]
        fn from(val: Divclp) -> u8 {
            Divclp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Encmode {
        #[doc = "2.4 GHz RADIO encryption function disabled."]
        B_0X0 = 0x0,
        #[doc = "2.4 GHz RADIO encryption function enabled."]
        B_0X1 = 0x01,
    }
    impl Encmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Encmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Encmode {
        #[inline(always)]
        fn from(val: u8) -> Encmode {
            Encmode::from_bits(val)
        }
    }
    impl From<Encmode> for u8 {
        #[inline(always)]
        fn from(val: Encmode) -> u8 {
            Encmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Flashfwu {
        #[doc = "Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption)."]
        LOW_POWER = 0x0,
        #[doc = "Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time)."]
        NORMAL = 0x01,
    }
    impl Flashfwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flashfwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flashfwu {
        #[inline(always)]
        fn from(val: u8) -> Flashfwu {
            Flashfwu::from_bits(val)
        }
    }
    impl From<Flashfwu> for u8 {
        #[inline(always)]
        fn from(val: Flashfwu) -> u8 {
            Flashfwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fsten {
        #[doc = "LDO fast startup disabled (limited inrush current)."]
        B_0X0 = 0x0,
        #[doc = "LDO fast startup enabled."]
        B_0X1 = 0x01,
    }
    impl Fsten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fsten {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fsten {
        #[inline(always)]
        fn from(val: u8) -> Fsten {
            Fsten::from_bits(val)
        }
    }
    impl From<Fsten> for u8 {
        #[inline(always)]
        fn from(val: Fsten) -> u8 {
            Fsten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Icrampds {
        #[doc = "ICACHE SRAM content retained in Stop modes"]
        RETAINED = 0x0,
        #[doc = "ICACHE SRAM content lost in Stop modes"]
        NOT_RETAINED = 0x01,
    }
    impl Icrampds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Icrampds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Icrampds {
        #[inline(always)]
        fn from(val: u8) -> Icrampds {
            Icrampds::from_bits(val)
        }
    }
    impl From<Icrampds> for u8 {
        #[inline(always)]
        fn from(val: Icrampds) -> u8 {
            Icrampds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Io2sv {
        #[doc = "VDDIO2 not supplied, electrical and logical isolation enabled."]
        B_0X0 = 0x0,
        #[doc = "VDDIO2 supply present, electrical and logical isolation disabled."]
        B_0X1 = 0x01,
    }
    impl Io2sv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Io2sv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Io2sv {
        #[inline(always)]
        fn from(val: u8) -> Io2sv {
            Io2sv::from_bits(val)
        }
    }
    impl From<Io2sv> for u8 {
        #[inline(always)]
        fn from(val: Io2sv) -> u8 {
            Io2sv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Stop 0 mode"]
        STOP0 = 0x0,
        #[doc = "Stop 1 mode"]
        STOP1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpmsec {
        #[doc = "CR1, CR2 and CSSF in the SR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "CR1, CR2, and CSSF in the SR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Lpmsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpmsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpmsec {
        #[inline(always)]
        fn from(val: u8) -> Lpmsec {
            Lpmsec::from_bits(val)
        }
    }
    impl From<Lpmsec> for u8 {
        #[inline(always)]
        fn from(val: Lpmsec) -> u8 {
            Lpmsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "2.4 GHz RADIO deep sleep mode"]
        DEEP_SLEEP = 0x0,
        #[doc = "2.4 GHz RADIO sleep mode"]
        SLEEP = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
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
    pub enum Phymode {
        #[doc = "2.4 GHz RADIO Sleep mode."]
        B_0X0 = 0x0,
        #[doc = "2.4 GHz RADIO Standby mode."]
        B_0X1 = 0x01,
    }
    impl Phymode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Phymode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Phymode {
        #[inline(always)]
        fn from(val: u8) -> Phymode {
            Phymode::from_bits(val)
        }
    }
    impl From<Phymode> for u8 {
        #[inline(always)]
        fn from(val: Phymode) -> u8 {
            Phymode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pkarampds {
        #[doc = "PKA SRAM content retained in Stop modes."]
        B_0X0 = 0x0,
        #[doc = "PKA SRAM content lost in Stop modes."]
        B_0X1 = 0x01,
    }
    impl Pkarampds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pkarampds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pkarampds {
        #[inline(always)]
        fn from(val: u8) -> Pkarampds {
            Pkarampds::from_bits(val)
        }
    }
    impl From<Pkarampds> for u8 {
        #[inline(always)]
        fn from(val: Pkarampds) -> u8 {
            Pkarampds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prampds {
        #[doc = "OTG SRAM content retained in Stop modes."]
        B_0X0 = 0x0,
        #[doc = "OTG SRAM content lost in Stop modes."]
        B_0X1 = 0x01,
    }
    impl Prampds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prampds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prampds {
        #[inline(always)]
        fn from(val: u8) -> Prampds {
            Prampds::from_bits(val)
        }
    }
    impl From<Prampds> for u8 {
        #[inline(always)]
        fn from(val: Prampds) -> u8 {
            Prampds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ptasr {
        #[doc = "Cleared by software, writing 0."]
        B_0X0 = 0x0,
        #[doc = "Set by hardware when Stop 2 mode PTA retention is enabled in PTASREN and Stop 2 mode is entered."]
        B_0X1 = 0x01,
    }
    impl Ptasr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ptasr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ptasr {
        #[inline(always)]
        fn from(val: u8) -> Ptasr {
            Ptasr::from_bits(val)
        }
    }
    impl From<Ptasr> for u8 {
        #[inline(always)]
        fn from(val: Ptasr) -> u8 {
            Ptasr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ptasren {
        #[doc = "PTA output signals Stop 2 retention feature disabled."]
        B_0X0 = 0x0,
        #[doc = "PTA output signals Stop 2 retention feature enabled."]
        B_0X1 = 0x01,
    }
    impl Ptasren {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ptasren {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ptasren {
        #[inline(always)]
        fn from(val: u8) -> Ptasren {
            Ptasren::from_bits(val)
        }
    }
    impl From<Ptasren> for u8 {
        #[inline(always)]
        fn from(val: Ptasren) -> u8 {
            Ptasren::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdls {
        #[doc = "VPVD0 around 2.0 V"]
        V20 = 0x0,
        #[doc = "VPVD1 around 2.2 V"]
        V22 = 0x01,
        #[doc = "VPVD2 around 2.4 V"]
        V24 = 0x02,
        #[doc = "VPVD3 around 2.5 V"]
        V25 = 0x03,
        #[doc = "VPVD4 around 2.6 V"]
        V26 = 0x04,
        #[doc = "VPVD5 around 2.8 V"]
        V28 = 0x05,
        #[doc = "VPVD6 around 2.9 V"]
        V29 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
        PVD_IN = 0x07,
    }
    impl Pvdls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdls {
        #[inline(always)]
        fn from(val: u8) -> Pvdls {
            Pvdls::from_bits(val)
        }
    }
    impl From<Pvdls> for u8 {
        #[inline(always)]
        fn from(val: Pvdls) -> u8 {
            Pvdls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdo {
        #[doc = "VDD is equal or above the PVD threshold selected by PVDLS\\[2:0\\]."]
        ABOVE_OR_EQUAL = 0x0,
        #[doc = "VDD is below the PVD threshold selected by PVDLS\\[2:0\\]."]
        BELOW = 0x01,
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
    pub enum Regpasel {
        #[doc = "VDDRFPA pin selected as regulator REG_VDDHPA input supply."]
        B_0X0 = 0x0,
        #[doc = "regulator REG_VDDHPA input supply selection between VDDRFPA and VDD11, dependent on requested regulated output voltage."]
        B_0X1 = 0x01,
    }
    impl Regpasel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regpasel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regpasel {
        #[inline(always)]
        fn from(val: u8) -> Regpasel {
            Regpasel::from_bits(val)
        }
    }
    impl From<Regpasel> for u8 {
        #[inline(always)]
        fn from(val: Regpasel) -> u8 {
            Regpasel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Regs {
        #[doc = "LDO selected."]
        B_0X0 = 0x0,
        #[doc = "SMPS selected."]
        B_0X1 = 0x01,
    }
    impl Regs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regs {
        #[inline(always)]
        fn from(val: u8) -> Regs {
            Regs::from_bits(val)
        }
    }
    impl From<Regs> for u8 {
        #[inline(always)]
        fn from(val: Regs) -> u8 {
            Regs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Regsel {
        #[doc = "LDO selected."]
        B_0X0 = 0x0,
        #[doc = "SMPS selected."]
        B_0X1 = 0x01,
    }
    impl Regsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regsel {
        #[inline(always)]
        fn from(val: u8) -> Regsel {
            Regsel::from_bits(val)
        }
    }
    impl From<Regsel> for u8 {
        #[inline(always)]
        fn from(val: Regsel) -> u8 {
            Regsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sbf {
        #[doc = "The device did not enter Standby mode."]
        B_0X0 = 0x0,
        #[doc = "The device entered Standby mode."]
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
    pub enum Sram1pds567 {
        #[doc = "SRAM1 192KB, page 5 to 7 content retained in Stop modes"]
        POWERED_ON = 0x0,
        #[doc = "SRAM1 192KB, page 5 to 7 content lost in Stop modes"]
        POWERED_OFF = 0x01,
    }
    impl Sram1pds567 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sram1pds567 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sram1pds567 {
        #[inline(always)]
        fn from(val: u8) -> Sram1pds567 {
            Sram1pds567::from_bits(val)
        }
    }
    impl From<Sram1pds567> for u8 {
        #[inline(always)]
        fn from(val: Sram1pds567) -> u8 {
            Sram1pds567::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Srampds {
        #[doc = "SRAM1 content retained in Stop modes"]
        POWERED_ON = 0x0,
        #[doc = "SRAM1 content lost in Stop modes"]
        POWERED_OFF = 0x01,
    }
    impl Srampds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Srampds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Srampds {
        #[inline(always)]
        fn from(val: u8) -> Srampds {
            Srampds::from_bits(val)
        }
    }
    impl From<Srampds> for u8 {
        #[inline(always)]
        fn from(val: Srampds) -> u8 {
            Srampds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usv {
        #[doc = "VDDUSB not supplied, electrical and logical isolation enabled."]
        B_0X0 = 0x0,
        #[doc = "VDDUSB supply present, electrical and logical isolation disabled."]
        B_0X1 = 0x01,
    }
    impl Usv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usv {
        #[inline(always)]
        fn from(val: u8) -> Usv {
            Usv::from_bits(val)
        }
    }
    impl From<Usv> for u8 {
        #[inline(always)]
        fn from(val: Usv) -> u8 {
            Usv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum V11fbsw {
        #[doc = "V11 feedback fixed before Epod."]
        B_0X0 = 0x0,
        #[doc = "V11 feedback switch enabled:."]
        B_0X1 = 0x01,
    }
    impl V11fbsw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> V11fbsw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for V11fbsw {
        #[inline(always)]
        fn from(val: u8) -> V11fbsw {
            V11fbsw::from_bits(val)
        }
    }
    impl From<V11fbsw> for u8 {
        #[inline(always)]
        fn from(val: V11fbsw) -> u8 {
            V11fbsw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbsec {
        #[doc = "DBPCR can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "DBPCR can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Vbsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbsec {
        #[inline(always)]
        fn from(val: u8) -> Vbsec {
            Vbsec::from_bits(val)
        }
    }
    impl From<Vbsec> for u8 {
        #[inline(always)]
        fn from(val: Vbsec) -> u8 {
            Vbsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vdmsec {
        #[doc = "SVMCR and CR3 can be read and written with secure or non-secure access."]
        B_0X0 = 0x0,
        #[doc = "SVMCR and CR3 can be read and written only with secure access."]
        B_0X1 = 0x01,
    }
    impl Vdmsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vdmsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vdmsec {
        #[inline(always)]
        fn from(val: u8) -> Vdmsec {
            Vdmsec::from_bits(val)
        }
    }
    impl From<Vdmsec> for u8 {
        #[inline(always)]
        fn from(val: Vdmsec) -> u8 {
            Vdmsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        #[doc = "Range 2 (lowest power)"]
        RANGE2 = 0x0,
        #[doc = "Range 1 (highest frequency)."]
        RANGE1 = 0x01,
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
    pub enum Wupp {
        #[doc = "Detection on high level (rising edge)"]
        HIGH = 0x0,
        #[doc = "Detection on low level (falling edge)"]
        LOW = 0x01,
    }
    impl Wupp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wupp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wupp {
        #[inline(always)]
        fn from(val: u8) -> Wupp {
            Wupp::from_bits(val)
        }
    }
    impl From<Wupp> for u8 {
        #[inline(always)]
        fn from(val: Wupp) -> u8 {
            Wupp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wusel {
        #[doc = "reserved"]
        B_0X0 = 0x0,
        #[doc = "WKUP3_1"]
        B_0X1 = 0x01,
        #[doc = "WKUP3_2"]
        B_0X2 = 0x02,
        #[doc = "reserved"]
        B_0X3 = 0x03,
    }
    impl Wusel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wusel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wusel {
        #[inline(always)]
        fn from(val: u8) -> Wusel {
            Wusel::from_bits(val)
        }
    }
    impl From<Wusel> for u8 {
        #[inline(always)]
        fn from(val: Wusel) -> u8 {
            Wusel::to_bits(val)
        }
    }
}
