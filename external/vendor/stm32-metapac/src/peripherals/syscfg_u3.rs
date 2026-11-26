#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SYSCFG address block description."]
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
    #[doc = "SYSCFG secure configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SYSCFG configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SYSCFG FPU interrupt mask register."]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SYSCFG CPU nonsecure lock register."]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SYSCFG CPU secure lock register."]
    #[inline(always)]
    pub const fn cslckr(self) -> crate::common::Reg<regs::Cslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SYSCFG configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SYSCFG compensation cell control/status register."]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SYSCFG compensation cell value register."]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SYSCFG compensation cell code register."]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG RSS command register."]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG compensation cell code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "NMOS compensation code of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub const fn ncc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub fn set_ncc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub const fn pcc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub fn set_pcc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NMOS compensation code of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub const fn ncc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub fn set_ncc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PMOS compensation code of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub const fn pcc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub fn set_pcc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    impl core::fmt::Debug for Cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccr")
                .field("ncc1", &self.ncc1())
                .field("pcc1", &self.pcc1())
                .field("ncc2", &self.ncc2())
                .field("pcc2", &self.pcc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cccr {{ ncc1: {=u8:?}, pcc1: {=u8:?}, ncc2: {=u8:?}, pcc2: {=u8:?} }}",
                self.ncc1(),
                self.pcc1(),
                self.ncc2(),
                self.pcc2()
            )
        }
    }
    #[doc = "SYSCFG compensation cell control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "VDD I/O compensation cell enable."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/O compensation cell enable."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VDD I/O code selection."]
        #[inline(always)]
        pub const fn cs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/O code selection."]
        #[inline(always)]
        pub fn set_cs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VDDIO2 I/O compensation cell enable."]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/O compensation cell enable."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VDDIO2 I/O code selection."]
        #[inline(always)]
        pub const fn cs2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/O code selection."]
        #[inline(always)]
        pub fn set_cs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "VDD I/O compensation cell ready flag."]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/O compensation cell ready flag."]
        #[inline(always)]
        pub fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VDDIO2 I/O compensation cell ready flag."]
        #[inline(always)]
        pub const fn rdy2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/O compensation cell ready flag."]
        #[inline(always)]
        pub fn set_rdy2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cccsr {
        #[inline(always)]
        fn default() -> Cccsr {
            Cccsr(0)
        }
    }
    impl core::fmt::Debug for Cccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccsr")
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("en2", &self.en2())
                .field("cs2", &self.cs2())
                .field("rdy1", &self.rdy1())
                .field("rdy2", &self.rdy2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccsr {{ en1: {=bool:?}, cs1: {=bool:?}, en2: {=bool:?}, cs2: {=bool:?}, rdy1: {=bool:?}, rdy2: {=bool:?} }}" , self . en1 () , self . cs1 () , self . en2 () , self . cs2 () , self . rdy1 () , self . rdy2 ())
        }
    }
    #[doc = "SYSCFG compensation cell value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "NMOS compensation value of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub const fn ncv1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub fn set_ncv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation value of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub const fn pcv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value of the I/Os supplied by Vless thansub>DDless than/sub>."]
        #[inline(always)]
        pub fn set_pcv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NMOS compensation value of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub const fn ncv2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub fn set_ncv2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PMOS compensation value of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub const fn pcv2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value of the I/Os supplied by Vless thansub>DDIO2less than/sub>."]
        #[inline(always)]
        pub fn set_pcv2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ccvr {
        #[inline(always)]
        fn default() -> Ccvr {
            Ccvr(0)
        }
    }
    impl core::fmt::Debug for Ccvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvr")
                .field("ncv1", &self.ncv1())
                .field("pcv1", &self.pcv1())
                .field("ncv2", &self.ncv2())
                .field("pcv2", &self.pcv2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccvr {{ ncv1: {=u8:?}, pcv1: {=u8:?}, ncv2: {=u8:?}, pcv2: {=u8:?} }}",
                self.ncv1(),
                self.pcv1(),
                self.ncv2(),
                self.pcv2()
            )
        }
    }
    #[doc = "SYSCFG configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "IR output polarity selection."]
        #[inline(always)]
        pub const fn ir_pol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IR output polarity selection."]
        #[inline(always)]
        pub fn set_ir_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IR modulation envelope signal selection."]
        #[inline(always)]
        pub const fn ir_mod(&self) -> super::vals::IrMod {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::IrMod::from_bits(val as u8)
        }
        #[doc = "IR modulation envelope signal selection."]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: super::vals::IrMod) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "I/O analog switch voltage booster enable."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO analog switch control voltage selection."]
        #[inline(always)]
        pub const fn anaswvdd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO analog switch control voltage selection."]
        #[inline(always)]
        pub fn set_anaswvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub const fn pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub fn set_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub const fn pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub fn set_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub const fn pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub fn set_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub const fn pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PBi."]
        #[inline(always)]
        pub fn set_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("ir_pol", &self.ir_pol())
                .field("ir_mod", &self.ir_mod())
                .field("boosten", &self.boosten())
                .field("anaswvdd", &self.anaswvdd())
                .field("pb6_fmp", &self.pb6_fmp())
                .field("pb7_fmp", &self.pb7_fmp())
                .field("pb8_fmp", &self.pb8_fmp())
                .field("pb9_fmp", &self.pb9_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ ir_pol: {=bool:?}, ir_mod: {:?}, boosten: {=bool:?}, anaswvdd: {=bool:?}, pb6_fmp: {=bool:?}, pb7_fmp: {=bool:?}, pb8_fmp: {=bool:?}, pb9_fmp: {=bool:?} }}" , self . ir_pol () , self . ir_mod () , self . boosten () , self . anaswvdd () , self . pb6_fmp () , self . pb7_fmp () , self . pb8_fmp () , self . pb9_fmp ())
        }
    }
    #[doc = "SYSCFG configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M33 LOCKUP (HardFault) output enable."]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M33 LOCKUP (HardFault) output enable."]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity bit."]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity bit."]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit."]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit."]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC lock."]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC lock."]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("cll", &self.cll())
                .field("spl", &self.spl())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ cll: {=bool:?}, spl: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?} }}",
                self.cll(),
                self.spl(),
                self.pvdl(),
                self.eccl()
            )
        }
    }
    #[doc = "SYSCFG CPU nonsecure lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnslckr(pub u32);
    impl Cnslckr {
        #[doc = "VTOR_NS register lock."]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_NS register lock."]
        #[inline(always)]
        pub fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Nonsecure MPU registers lock."]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Nonsecure MPU registers lock."]
        #[inline(always)]
        pub fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cnslckr {
        #[inline(always)]
        fn default() -> Cnslckr {
            Cnslckr(0)
        }
    }
    impl core::fmt::Debug for Cnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnslckr {{ locknsvtor: {=bool:?}, locknsmpu: {=bool:?} }}",
                self.locknsvtor(),
                self.locknsmpu()
            )
        }
    }
    #[doc = "SYSCFG CPU secure lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cslckr(pub u32);
    impl Cslckr {
        #[doc = "VTOR_S register and AIRCR register bits lock."]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_S register and AIRCR register bits lock."]
        #[inline(always)]
        pub fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure MPU registers lock."]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure MPU registers lock."]
        #[inline(always)]
        pub fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SAU register lock."]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SAU register lock."]
        #[inline(always)]
        pub fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cslckr {
        #[inline(always)]
        fn default() -> Cslckr {
            Cslckr(0)
        }
    }
    impl core::fmt::Debug for Cslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cslckr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cslckr {{ locksvtaircr: {=bool:?}, locksmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locksmpu(),
                self.locksau()
            )
        }
    }
    #[doc = "SYSCFG FPU interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "Floating point unit interrupt enable bit-invalid operation."]
        #[inline(always)]
        pub const fn fpu_ioie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-invalid operation."]
        #[inline(always)]
        pub fn set_fpu_ioie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Floating point unit interrupt enable bit-divide-by-zero."]
        #[inline(always)]
        pub const fn fpu_dzie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-divide-by-zero."]
        #[inline(always)]
        pub fn set_fpu_dzie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Floating point unit interrupt enable bit-underflow."]
        #[inline(always)]
        pub const fn fpu_ufie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-underflow."]
        #[inline(always)]
        pub fn set_fpu_ufie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Floating point unit interrupt enable bit-overflow."]
        #[inline(always)]
        pub const fn fpu_ofie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-overflow."]
        #[inline(always)]
        pub fn set_fpu_ofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Floating point unit interrupt enable bit-input denormal."]
        #[inline(always)]
        pub const fn fpu_idie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-input denormal."]
        #[inline(always)]
        pub fn set_fpu_idie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Floating point unit interrupt enable bit-inexact."]
        #[inline(always)]
        pub const fn fpu_ix_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Floating point unit interrupt enable bit-inexact."]
        #[inline(always)]
        pub fn set_fpu_ix_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Fpuimr {
        #[inline(always)]
        fn default() -> Fpuimr {
            Fpuimr(0)
        }
    }
    impl core::fmt::Debug for Fpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuimr")
                .field("fpu_ioie", &self.fpu_ioie())
                .field("fpu_dzie", &self.fpu_dzie())
                .field("fpu_ufie", &self.fpu_ufie())
                .field("fpu_ofie", &self.fpu_ofie())
                .field("fpu_idie", &self.fpu_idie())
                .field("fpu_ix_ie", &self.fpu_ix_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fpuimr {{ fpu_ioie: {=bool:?}, fpu_dzie: {=bool:?}, fpu_ufie: {=bool:?}, fpu_ofie: {=bool:?}, fpu_idie: {=bool:?}, fpu_ix_ie: {=bool:?} }}" , self . fpu_ioie () , self . fpu_dzie () , self . fpu_ufie () , self . fpu_ofie () , self . fpu_idie () , self . fpu_ix_ie ())
        }
    }
    #[doc = "SYSCFG RSS command register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS commands."]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS commands."]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            defmt::write!(f, "Rsscmdr {{ rsscmd: {=u16:?} }}", self.rsscmd())
        }
    }
    #[doc = "SYSCFG secure configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Security of SYSCFG clock control, memory erase status, and compensation cell registers."]
        #[inline(always)]
        pub const fn syscfgsec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Security of SYSCFG clock control, memory erase status, and compensation cell registers."]
        #[inline(always)]
        pub fn set_syscfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Class B security."]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Class B security."]
        #[inline(always)]
        pub fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FPU security."]
        #[inline(always)]
        pub const fn fpusec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FPU security."]
        #[inline(always)]
        pub fn set_fpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("syscfgsec", &self.syscfgsec())
                .field("classbsec", &self.classbsec())
                .field("fpusec", &self.fpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ syscfgsec: {=bool:?}, classbsec: {=bool:?}, fpusec: {=bool:?} }}",
                self.syscfgsec(),
                self.classbsec(),
                self.fpusec()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMod {
        #[doc = "TIM16."]
        TIM16 = 0x0,
        #[doc = "USART1."]
        USART1 = 0x01,
        #[doc = "UART4."]
        UART4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl IrMod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMod {
        #[inline(always)]
        fn from(val: u8) -> IrMod {
            IrMod::from_bits(val)
        }
    }
    impl From<IrMod> for u8 {
        #[inline(always)]
        fn from(val: IrMod) -> u8 {
            IrMod::to_bits(val)
        }
    }
}
