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
    #[doc = "Power control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Power control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Power status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Power status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Power status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Power control register 5"]
    #[inline(always)]
    pub const fn cr5(self) -> crate::common::Reg<regs::Cr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Power Port pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "Power Port pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 8usize) as _) }
    }
    #[doc = "Power CPU2 control register 1 \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn c2cr1(self) -> crate::common::Reg<regs::C2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Power CPU2 control register 3 \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn c2cr3(self) -> crate::common::Reg<regs::C2cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Power extended status and status clear register"]
    #[inline(always)]
    pub const fn extscr(self) -> crate::common::Reg<regs::Extscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Power security configuration register \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Power SPI3 control register"]
    #[inline(always)]
    pub const fn subghzspicr(self) -> crate::common::Reg<regs::Subghzspicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RSS Command register \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power CPU2 control register 1 \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr1(pub u32);
    impl C2cr1 {
        #[doc = "Low-power mode selection for CPU2"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection for CPU2"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash memory power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub const fn fpdr(&self) -> super::vals::Fpdr {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Fpdr::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: super::vals::Fpdr) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub const fn fpds(&self) -> super::vals::Fpds {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Fpds::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: super::vals::Fpds) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for C2cr1 {
        #[inline(always)]
        fn default() -> C2cr1 {
            C2cr1(0)
        }
    }
    impl core::fmt::Debug for C2cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2cr1")
                .field("lpms", &self.lpms())
                .field("fpdr", &self.fpdr())
                .field("fpds", &self.fpds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "C2cr1 {{ lpms: {:?}, fpdr: {:?}, fpds: {:?} }}",
                self.lpms(),
                self.fpdr(),
                self.fpds()
            )
        }
    }
    #[doc = "Power CPU2 control register 3 \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr3(pub u32);
    impl C2cr3 {
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Enable wakeup PVD for CPU2"]
        #[inline(always)]
        pub const fn ewpvd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup PVD for CPU2"]
        #[inline(always)]
        pub fn set_ewpvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EWRFBUSY"]
        #[inline(always)]
        pub const fn ewrfbusy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EWRFBUSY"]
        #[inline(always)]
        pub fn set_ewrfbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "akeup for CPU2"]
        #[inline(always)]
        pub const fn ewrfirq(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "akeup for CPU2"]
        #[inline(always)]
        pub fn set_ewrfirq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable internal wakeup line for CPU2"]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line for CPU2"]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for C2cr3 {
        #[inline(always)]
        fn default() -> C2cr3 {
            C2cr3(0)
        }
    }
    impl core::fmt::Debug for C2cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C2cr3")
                .field("ewup[0]", &self.ewup(0usize))
                .field("ewup[1]", &self.ewup(1usize))
                .field("ewup[2]", &self.ewup(2usize))
                .field("ewpvd", &self.ewpvd())
                .field("apc", &self.apc())
                .field("ewrfbusy", &self.ewrfbusy())
                .field("ewrfirq", &self.ewrfirq())
                .field("eiwul", &self.eiwul())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C2cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "C2cr3 {{ ewup[0]: {=bool:?}, ewup[1]: {=bool:?}, ewup[2]: {=bool:?}, ewpvd: {=bool:?}, apc: {=bool:?}, ewrfbusy: {=bool:?}, ewrfirq: {=bool:?}, eiwul: {=bool:?} }}" , self . ewup (0usize) , self . ewup (1usize) , self . ewup (2usize) , self . ewpvd () , self . apc () , self . ewrfbusy () , self . ewrfirq () , self . eiwul ())
        }
    }
    #[doc = "Power control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection for CPU1"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection for CPU1"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "sub-GHz SPI NSS source select"]
        #[inline(always)]
        pub const fn subghzspinsssel(&self) -> super::vals::Subghzspinsssel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Subghzspinsssel::from_bits(val as u8)
        }
        #[doc = "sub-GHz SPI NSS source select"]
        #[inline(always)]
        pub fn set_subghzspinsssel(&mut self, val: super::vals::Subghzspinsssel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash memory power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub const fn fpdr(&self) -> super::vals::Fpdr {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Fpdr::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: super::vals::Fpdr) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU1"]
        #[inline(always)]
        pub const fn fpds(&self) -> super::vals::Fpds {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Fpds::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU1"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: super::vals::Fpds) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub const fn lpr(&self) -> super::vals::Lpr {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Lpr::from_bits(val as u8)
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub fn set_lpr(&mut self, val: super::vals::Lpr) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
                .field("subghzspinsssel", &self.subghzspinsssel())
                .field("fpdr", &self.fpdr())
                .field("fpds", &self.fpds())
                .field("dbp", &self.dbp())
                .field("vos", &self.vos())
                .field("lpr", &self.lpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpms: {:?}, subghzspinsssel: {:?}, fpdr: {:?}, fpds: {:?}, dbp: {=bool:?}, vos: {:?}, lpr: {:?} }}" , self . lpms () , self . subghzspinsssel () , self . fpdr () , self . fpds () , self . dbp () , self . vos () , self . lpr ())
        }
    }
    #[doc = "Power control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power voltage detector level selection."]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Power voltage detector level selection."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub const fn pvme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub fn set_pvme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("pvme", &self.pvme())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ pvde: {=bool:?}, pls: {:?}, pvme: {=bool:?} }}",
                self.pvde(),
                self.pls(),
                self.pvme()
            )
        }
    }
    #[doc = "Power control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Enable Wakeup pin WKUP1 for CPU1"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP1 for CPU1"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Ultra-low-power enable"]
        #[inline(always)]
        pub const fn eulpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ultra-low-power enable"]
        #[inline(always)]
        pub fn set_eulpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable wakeup PVD for CPU1"]
        #[inline(always)]
        pub const fn ewpvd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup PVD for CPU1"]
        #[inline(always)]
        pub fn set_ewpvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub const fn rrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Apply pull-up and pull-down configuration from CPU1"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration from CPU1"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Radio BUSY Wakeup from Standby for CPU1"]
        #[inline(always)]
        pub const fn ewrfbusy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Radio BUSY Wakeup from Standby for CPU1"]
        #[inline(always)]
        pub fn set_ewrfbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Wakeup for CPU1"]
        #[inline(always)]
        pub const fn ewrfirq(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup for CPU1"]
        #[inline(always)]
        pub fn set_ewrfirq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "nable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub const fn ec2h(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "nable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub fn set_ec2h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable internal wakeup line for CPU1"]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line for CPU1"]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("ewup[0]", &self.ewup(0usize))
                .field("ewup[1]", &self.ewup(1usize))
                .field("ewup[2]", &self.ewup(2usize))
                .field("eulpen", &self.eulpen())
                .field("ewpvd", &self.ewpvd())
                .field("rrs", &self.rrs())
                .field("apc", &self.apc())
                .field("ewrfbusy", &self.ewrfbusy())
                .field("ewrfirq", &self.ewrfirq())
                .field("ec2h", &self.ec2h())
                .field("eiwul", &self.eiwul())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ ewup[0]: {=bool:?}, ewup[1]: {=bool:?}, ewup[2]: {=bool:?}, eulpen: {=bool:?}, ewpvd: {=bool:?}, rrs: {=bool:?}, apc: {=bool:?}, ewrfbusy: {=bool:?}, ewrfirq: {=bool:?}, ec2h: {=bool:?}, eiwul: {=bool:?} }}" , self . ewup (0usize) , self . ewup (1usize) , self . ewup (2usize) , self . eulpen () , self . ewpvd () , self . rrs () , self . apc () , self . ewrfbusy () , self . ewrfirq () , self . ec2h () , self . eiwul ())
        }
    }
    #[doc = "Power control register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp(&self, n: usize) -> super::vals::Wp {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wp::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp(&mut self, n: usize, val: super::vals::Wp) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup Radio BUSY polarity"]
        #[inline(always)]
        pub const fn wrfbusyp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup Radio BUSY polarity"]
        #[inline(always)]
        pub fn set_wrfbusyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "oot CPU2 after reset or wakeup from Stop or Standby modes."]
        #[inline(always)]
        pub const fn c2boot(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "oot CPU2 after reset or wakeup from Stop or Standby modes."]
        #[inline(always)]
        pub fn set_c2boot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("wp[0]", &self.wp(0usize))
                .field("wp[1]", &self.wp(1usize))
                .field("wp[2]", &self.wp(2usize))
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .field("wrfbusyp", &self.wrfbusyp())
                .field("c2boot", &self.c2boot())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr4 {{ wp[0]: {:?}, wp[1]: {:?}, wp[2]: {:?}, vbe: {=bool:?}, vbrs: {:?}, wrfbusyp: {=bool:?}, c2boot: {=bool:?} }}" , self . wp (0usize) , self . wp (1usize) , self . wp (2usize) , self . vbe () , self . vbrs () , self . wrfbusyp () , self . c2boot ())
        }
    }
    #[doc = "Power control register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr5(pub u32);
    impl Cr5 {
        #[doc = "Enable Radio End Of Life detector enabled"]
        #[inline(always)]
        pub const fn rfeolen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Radio End Of Life detector enabled"]
        #[inline(always)]
        pub fn set_rfeolen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable SMPS Step Down converter SMPS mode enabled."]
        #[inline(always)]
        pub const fn smpsen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SMPS Step Down converter SMPS mode enabled."]
        #[inline(always)]
        pub fn set_smpsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr5 {
        #[inline(always)]
        fn default() -> Cr5 {
            Cr5(0)
        }
    }
    impl core::fmt::Debug for Cr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr5")
                .field("rfeolen", &self.rfeolen())
                .field("smpsen", &self.smpsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr5 {{ rfeolen: {=bool:?}, smpsen: {=bool:?} }}",
                self.rfeolen(),
                self.smpsen()
            )
        }
    }
    #[doc = "Power extended status and status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extscr(pub u32);
    impl Extscr {
        #[doc = "Clear CPU1 Stop Standby flags"]
        #[inline(always)]
        pub const fn c1cssf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear CPU1 Stop Standby flags"]
        #[inline(always)]
        pub fn set_c1cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "lear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub const fn c2cssf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "lear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub fn set_c2cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "System Standby flag for CPU1. (no core states retained)"]
        #[inline(always)]
        pub const fn c1sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag for CPU1. (no core states retained)"]
        #[inline(always)]
        pub fn set_c1sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "System Stop2 flag for CPU1. (partial core states retained)"]
        #[inline(always)]
        pub const fn c1stop2f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "System Stop2 flag for CPU1. (partial core states retained)"]
        #[inline(always)]
        pub fn set_c1stop2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "System Stop0, 1 flag for CPU1. (All core states retained)"]
        #[inline(always)]
        pub const fn c1stopf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "System Stop0, 1 flag for CPU1. (All core states retained)"]
        #[inline(always)]
        pub fn set_c1stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ystem Standby flag for CPU2. (no core states retained)"]
        #[inline(always)]
        pub const fn c2sbf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Standby flag for CPU2. (no core states retained)"]
        #[inline(always)]
        pub fn set_c2sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ystem Stop2 flag for CPU2. (partial core states retained)"]
        #[inline(always)]
        pub const fn c2stop2f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Stop2 flag for CPU2. (partial core states retained)"]
        #[inline(always)]
        pub fn set_c2stop2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ystem Stop0, 1 flag for CPU2. (All core states retained)"]
        #[inline(always)]
        pub const fn c2stopf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Stop0, 1 flag for CPU2. (All core states retained)"]
        #[inline(always)]
        pub fn set_c2stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub const fn c1ds(&self) -> super::vals::Cds {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Cds::from_bits(val as u8)
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub fn set_c1ds(&mut self, val: super::vals::Cds) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "PU2 deepsleep mode"]
        #[inline(always)]
        pub const fn c2ds(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 deepsleep mode"]
        #[inline(always)]
        pub fn set_c2ds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Extscr {
        #[inline(always)]
        fn default() -> Extscr {
            Extscr(0)
        }
    }
    impl core::fmt::Debug for Extscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extscr")
                .field("c1cssf", &self.c1cssf())
                .field("c2cssf", &self.c2cssf())
                .field("c1sbf", &self.c1sbf())
                .field("c1stop2f", &self.c1stop2f())
                .field("c1stopf", &self.c1stopf())
                .field("c2sbf", &self.c2sbf())
                .field("c2stop2f", &self.c2stop2f())
                .field("c2stopf", &self.c2stopf())
                .field("c1ds", &self.c1ds())
                .field("c2ds", &self.c2ds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Extscr {{ c1cssf: {=bool:?}, c2cssf: {=bool:?}, c1sbf: {=bool:?}, c1stop2f: {=bool:?}, c1stopf: {=bool:?}, c2sbf: {=bool:?}, c2stop2f: {=bool:?}, c2stopf: {=bool:?}, c1ds: {:?}, c2ds: {=bool:?} }}" , self . c1cssf () , self . c2cssf () , self . c1sbf () , self . c1stop2f () , self . c1stopf () , self . c2sbf () , self . c2stop2f () , self . c2stopf () , self . c1ds () , self . c2ds ())
        }
    }
    #[doc = "Power Port pull control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub const fn p(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_p(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("p[0]", &self.p(0usize))
                .field("p[1]", &self.p(1usize))
                .field("p[2]", &self.p(2usize))
                .field("p[3]", &self.p(3usize))
                .field("p[4]", &self.p(4usize))
                .field("p[5]", &self.p(5usize))
                .field("p[6]", &self.p(6usize))
                .field("p[7]", &self.p(7usize))
                .field("p[8]", &self.p(8usize))
                .field("p[9]", &self.p(9usize))
                .field("p[10]", &self.p(10usize))
                .field("p[11]", &self.p(11usize))
                .field("p[12]", &self.p(12usize))
                .field("p[13]", &self.p(13usize))
                .field("p[14]", &self.p(14usize))
                .field("p[15]", &self.p(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ p[0]: {=bool:?}, p[1]: {=bool:?}, p[2]: {=bool:?}, p[3]: {=bool:?}, p[4]: {=bool:?}, p[5]: {=bool:?}, p[6]: {=bool:?}, p[7]: {=bool:?}, p[8]: {=bool:?}, p[9]: {=bool:?}, p[10]: {=bool:?}, p[11]: {=bool:?}, p[12]: {=bool:?}, p[13]: {=bool:?}, p[14]: {=bool:?}, p[15]: {=bool:?} }}" , self . p (0usize) , self . p (1usize) , self . p (2usize) , self . p (3usize) , self . p (4usize) , self . p (5usize) , self . p (6usize) , self . p (7usize) , self . p (8usize) , self . p (9usize) , self . p (10usize) , self . p (11usize) , self . p (12usize) , self . p (13usize) , self . p (14usize) , self . p (15usize))
        }
    }
    #[doc = "RSS Command register \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS command"]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RSS command"]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rsscmdr {
        #[inline(always)]
        fn default() -> Rsscmdr {
            Rsscmdr(0)
        }
    }
    impl core::fmt::Debug for Rsscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsscmdr").field("rsscmd", &self.rsscmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsscmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rsscmdr {{ rsscmd: {=u8:?} }}", self.rsscmd())
        }
    }
    #[doc = "Power status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear wakeup PVD interrupt flag"]
        #[inline(always)]
        pub const fn cwpvdf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup PVD interrupt flag"]
        #[inline(always)]
        pub fn set_cwpvdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear wakeup Radio BUSY flag"]
        #[inline(always)]
        pub const fn cwrfbusyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup Radio BUSY flag"]
        #[inline(always)]
        pub fn set_cwrfbusyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "lear CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn cc2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "lear CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub fn set_cc2hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("cwuf[0]", &self.cwuf(0usize))
                .field("cwuf[1]", &self.cwuf(1usize))
                .field("cwuf[2]", &self.cwuf(2usize))
                .field("cwpvdf", &self.cwpvdf())
                .field("cwrfbusyf", &self.cwrfbusyf())
                .field("cc2hf", &self.cc2hf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Scr {{ cwuf[0]: {=bool:?}, cwuf[1]: {=bool:?}, cwuf[2]: {=bool:?}, cwpvdf: {=bool:?}, cwrfbusyf: {=bool:?}, cc2hf: {=bool:?} }}" , self . cwuf (0usize) , self . cwuf (1usize) , self . cwuf (2usize) , self . cwpvdf () , self . cwrfbusyf () , self . cc2hf ())
        }
    }
    #[doc = "Power security configuration register \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "wakeup on CPU2 illegal access interrupt enable"]
        #[inline(always)]
        pub const fn c2ewila(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup on CPU2 illegal access interrupt enable"]
        #[inline(always)]
        pub fn set_c2ewila(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            f.debug_struct("Seccfgr").field("c2ewila", &self.c2ewila()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Seccfgr {{ c2ewila: {=bool:?} }}", self.c2ewila())
        }
    }
    #[doc = "Power status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup PVD flag"]
        #[inline(always)]
        pub const fn wpvdf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup PVD flag"]
        #[inline(always)]
        pub fn set_wpvdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Radio BUSY wakeup flag"]
        #[inline(always)]
        pub const fn wrfbusyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Radio BUSY wakeup flag"]
        #[inline(always)]
        pub fn set_wrfbusyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn c2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 Hold interrupt flag"]
        #[inline(always)]
        pub fn set_c2hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Internal wakeup interrupt flag"]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Internal wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_wufi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    impl core::fmt::Debug for Sr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1")
                .field("wuf[0]", &self.wuf(0usize))
                .field("wuf[1]", &self.wuf(1usize))
                .field("wuf[2]", &self.wuf(2usize))
                .field("wpvdf", &self.wpvdf())
                .field("wrfbusyf", &self.wrfbusyf())
                .field("c2hf", &self.c2hf())
                .field("wufi", &self.wufi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ wuf[0]: {=bool:?}, wuf[1]: {=bool:?}, wuf[2]: {=bool:?}, wpvdf: {=bool:?}, wrfbusyf: {=bool:?}, c2hf: {=bool:?}, wufi: {=bool:?} }}" , self . wuf (0usize) , self . wuf (1usize) , self . wuf (2usize) , self . wpvdf () , self . wrfbusyf () , self . c2hf () , self . wufi ())
        }
    }
    #[doc = "Power status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "PU2 boot/wakeup request source information"]
        #[inline(always)]
        pub const fn c2boots(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 boot/wakeup request source information"]
        #[inline(always)]
        pub fn set_c2boots(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Radio BUSY signal status"]
        #[inline(always)]
        pub const fn rfbusys(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Radio BUSY signal status"]
        #[inline(always)]
        pub fn set_rfbusys(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Radio BUSY masked signal status"]
        #[inline(always)]
        pub const fn rfbusyms(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Radio BUSY masked signal status"]
        #[inline(always)]
        pub fn set_rfbusyms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SMPS ready flag"]
        #[inline(always)]
        pub const fn smpsrdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS ready flag"]
        #[inline(always)]
        pub fn set_smpsrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LDO ready flag"]
        #[inline(always)]
        pub const fn ldordy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LDO ready flag"]
        #[inline(always)]
        pub fn set_ldordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Radio end of life flag"]
        #[inline(always)]
        pub const fn rfeolf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Radio end of life flag"]
        #[inline(always)]
        pub fn set_rfeolf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "regulator2 low power flag"]
        #[inline(always)]
        pub const fn regmrs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "regulator2 low power flag"]
        #[inline(always)]
        pub fn set_regmrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Flash ready"]
        #[inline(always)]
        pub const fn flashrdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ready"]
        #[inline(always)]
        pub fn set_flashrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "regulator1 started"]
        #[inline(always)]
        pub const fn reglps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "regulator1 started"]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "regulator1 low power flag"]
        #[inline(always)]
        pub const fn reglpf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "regulator1 low power flag"]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub const fn vosf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub const fn pvmo(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub fn set_pvmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    impl core::fmt::Debug for Sr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2")
                .field("c2boots", &self.c2boots())
                .field("rfbusys", &self.rfbusys())
                .field("rfbusyms", &self.rfbusyms())
                .field("smpsrdy", &self.smpsrdy())
                .field("ldordy", &self.ldordy())
                .field("rfeolf", &self.rfeolf())
                .field("regmrs", &self.regmrs())
                .field("flashrdy", &self.flashrdy())
                .field("reglps", &self.reglps())
                .field("reglpf", &self.reglpf())
                .field("vosf", &self.vosf())
                .field("pvdo", &self.pvdo())
                .field("pvmo", &self.pvmo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2 {{ c2boots: {=bool:?}, rfbusys: {=bool:?}, rfbusyms: {=bool:?}, smpsrdy: {=bool:?}, ldordy: {=bool:?}, rfeolf: {=bool:?}, regmrs: {=bool:?}, flashrdy: {=bool:?}, reglps: {=bool:?}, reglpf: {=bool:?}, vosf: {=bool:?}, pvdo: {=bool:?}, pvmo: {=bool:?} }}" , self . c2boots () , self . rfbusys () , self . rfbusyms () , self . smpsrdy () , self . ldordy () , self . rfeolf () , self . regmrs () , self . flashrdy () , self . reglps () , self . reglpf () , self . vosf () , self . pvdo () , self . pvmo ())
        }
    }
    #[doc = "Power SPI3 control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Subghzspicr(pub u32);
    impl Subghzspicr {
        #[doc = "sub-GHz SPI NSS control"]
        #[inline(always)]
        pub const fn nss(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "sub-GHz SPI NSS control"]
        #[inline(always)]
        pub fn set_nss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Subghzspicr {
        #[inline(always)]
        fn default() -> Subghzspicr {
            Subghzspicr(0)
        }
    }
    impl core::fmt::Debug for Subghzspicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Subghzspicr").field("nss", &self.nss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Subghzspicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Subghzspicr {{ nss: {=bool:?} }}", self.nss())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cds {
        #[doc = "CPU is running or in sleep"]
        RUNNING_OR_SLEEP = 0x0,
        #[doc = "CPU is in Deep-Sleep"]
        DEEP_SLEEP = 0x01,
    }
    impl Cds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cds {
        #[inline(always)]
        fn from(val: u8) -> Cds {
            Cds::from_bits(val)
        }
    }
    impl From<Cds> for u8 {
        #[inline(always)]
        fn from(val: Cds) -> u8 {
            Cds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fpdr {
        #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
        IDLE = 0x0,
        #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
        POWER_DOWN = 0x01,
    }
    impl Fpdr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpdr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpdr {
        #[inline(always)]
        fn from(val: u8) -> Fpdr {
            Fpdr::from_bits(val)
        }
    }
    impl From<Fpdr> for u8 {
        #[inline(always)]
        fn from(val: Fpdr) -> u8 {
            Fpdr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fpds {
        #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
        IDLE = 0x0,
        #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
        POWER_DOWN = 0x01,
    }
    impl Fpds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpds {
        #[inline(always)]
        fn from(val: u8) -> Fpds {
            Fpds::from_bits(val)
        }
    }
    impl From<Fpds> for u8 {
        #[inline(always)]
        fn from(val: Fpds) -> u8 {
            Fpds::to_bits(val)
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
        #[doc = "Stop 2 mode"]
        STOP2 = 0x02,
        #[doc = "Standby mode"]
        STANDBY = 0x03,
        #[doc = "Shutdown mode"]
        SHUTDOWN = 0x04,
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
    pub enum Lpr {
        #[doc = "Voltage regulator in Main mode in Low-power run mode"]
        MAIN_MODE = 0x0,
        #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
        LOW_POWER_MODE = 0x01,
    }
    impl Lpr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpr {
        #[inline(always)]
        fn from(val: u8) -> Lpr {
            Lpr::from_bits(val)
        }
    }
    impl From<Lpr> for u8 {
        #[inline(always)]
        fn from(val: Lpr) -> u8 {
            Lpr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "2.0V"]
        V2_0 = 0x0,
        #[doc = "2.2V"]
        V2_2 = 0x01,
        #[doc = "2.4V"]
        V2_4 = 0x02,
        #[doc = "2.5V"]
        V2_5 = 0x03,
        #[doc = "2.6V"]
        V2_6 = 0x04,
        #[doc = "2.8V"]
        V2_8 = 0x05,
        #[doc = "2.9V"]
        V2_9 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
        EXTERNAL = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Subghzspinsssel {
        #[doc = "sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
        SUBGHZSPICR = 0x0,
        #[doc = "sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
        LPTIM3 = 0x01,
    }
    impl Subghzspinsssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Subghzspinsssel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Subghzspinsssel {
        #[inline(always)]
        fn from(val: u8) -> Subghzspinsssel {
            Subghzspinsssel::from_bits(val)
        }
    }
    impl From<Subghzspinsssel> for u8 {
        #[inline(always)]
        fn from(val: Subghzspinsssel) -> u8 {
            Subghzspinsssel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbrs {
        #[doc = "VBAT charging through a 5 kΩ resistor"]
        R5K = 0x0,
        #[doc = "VBAT charging through a 1.5 kΩ resistor"]
        R1_5K = 0x01,
    }
    impl Vbrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbrs {
        #[inline(always)]
        fn from(val: u8) -> Vbrs {
            Vbrs::from_bits(val)
        }
    }
    impl From<Vbrs> for u8 {
        #[inline(always)]
        fn from(val: Vbrs) -> u8 {
            Vbrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "1.2 V (range 1)"]
        RANGE1 = 0x01,
        #[doc = "1.0 V (range 2)"]
        RANGE2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Wp {
        #[doc = "Detection on high level (rising edge)"]
        RISING_EDGE = 0x0,
        #[doc = "Detection on low level (falling edge)"]
        FALLING_EDGE = 0x01,
    }
    impl Wp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wp {
        #[inline(always)]
        fn from(val: u8) -> Wp {
            Wp::from_bits(val)
        }
    }
    impl From<Wp> for u8 {
        #[inline(always)]
        fn from(val: Wp) -> u8 {
            Wp::to_bits(val)
        }
    }
}
