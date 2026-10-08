#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Management data input/output slave"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdios {
    ptr: *mut u8,
}
unsafe impl Send for Mdios {}
unsafe impl Sync for Mdios {}
impl Mdios {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MDIOS configuration register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "MDIOS write flag register"]
    #[inline(always)]
    pub const fn wrfr(self) -> crate::common::Reg<regs::Wrfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "MDIOS clear write flag register"]
    #[inline(always)]
    pub const fn cwrfr(self) -> crate::common::Reg<regs::Cwrfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "MDIOS read flag register"]
    #[inline(always)]
    pub const fn rdfr(self) -> crate::common::Reg<regs::Rdfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "MDIOS clear read flag register"]
    #[inline(always)]
    pub const fn crdfr(self) -> crate::common::Reg<regs::Crdfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "MDIOS status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "MDIOS clear flag register"]
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<regs::Clrfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "MDIOS input data register %s"]
    #[inline(always)]
    pub const fn dinr(self, n: usize) -> crate::common::Reg<regs::Dinr, crate::common::R> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "MDIOS output data register %s"]
    #[inline(always)]
    pub const fn doutr(self, n: usize) -> crate::common::Reg<regs::Doutr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "MDIOS clear flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrfr(pub u32);
    impl Clrfr {
        #[doc = "Clear the preamble error flag"]
        #[inline(always)]
        pub const fn cperf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the preamble error flag"]
        #[inline(always)]
        pub fn set_cperf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear the start error flag"]
        #[inline(always)]
        pub const fn cserf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the start error flag"]
        #[inline(always)]
        pub fn set_cserf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear the turnaround error flag"]
        #[inline(always)]
        pub const fn cterf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the turnaround error flag"]
        #[inline(always)]
        pub fn set_cterf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Clrfr {
        #[inline(always)]
        fn default() -> Clrfr {
            Clrfr(0)
        }
    }
    impl core::fmt::Debug for Clrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clrfr")
                .field("cperf", &self.cperf())
                .field("cserf", &self.cserf())
                .field("cterf", &self.cterf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clrfr {{ cperf: {=bool:?}, cserf: {=bool:?}, cterf: {=bool:?} }}",
                self.cperf(),
                self.cserf(),
                self.cterf()
            )
        }
    }
    #[doc = "MDIOS configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Register write interrupt enable"]
        #[inline(always)]
        pub const fn wrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Register write interrupt enable"]
        #[inline(always)]
        pub fn set_wrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Register Read Interrupt Enable"]
        #[inline(always)]
        pub const fn rdie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Register Read Interrupt Enable"]
        #[inline(always)]
        pub fn set_rdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Disable Preamble Check"]
        #[inline(always)]
        pub const fn dpc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Preamble Check"]
        #[inline(always)]
        pub fn set_dpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Slaves's address"]
        #[inline(always)]
        pub const fn port_address(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Slaves's address"]
        #[inline(always)]
        pub fn set_port_address(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
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
                .field("en", &self.en())
                .field("wrie", &self.wrie())
                .field("rdie", &self.rdie())
                .field("eie", &self.eie())
                .field("dpc", &self.dpc())
                .field("port_address", &self.port_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, wrie: {=bool:?}, rdie: {=bool:?}, eie: {=bool:?}, dpc: {=bool:?}, port_address: {=u8:?} }}" , self . en () , self . wrie () , self . rdie () , self . eie () , self . dpc () , self . port_address ())
        }
    }
    #[doc = "MDIOS clear read flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crdfr(pub u32);
    impl Crdfr {
        #[doc = "Clear the read flag"]
        #[inline(always)]
        pub const fn crdf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Clear the read flag"]
        #[inline(always)]
        pub fn set_crdf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crdfr {
        #[inline(always)]
        fn default() -> Crdfr {
            Crdfr(0)
        }
    }
    impl core::fmt::Debug for Crdfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crdfr").field("crdf", &self.crdf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crdfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crdfr {{ crdf: {=u32:?} }}", self.crdf())
        }
    }
    #[doc = "MDIOS clear write flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cwrfr(pub u32);
    impl Cwrfr {
        #[doc = "Clear the write flag"]
        #[inline(always)]
        pub const fn cwrf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Clear the write flag"]
        #[inline(always)]
        pub fn set_cwrf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cwrfr {
        #[inline(always)]
        fn default() -> Cwrfr {
            Cwrfr(0)
        }
    }
    impl core::fmt::Debug for Cwrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cwrfr").field("cwrf", &self.cwrf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cwrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cwrfr {{ cwrf: {=u32:?} }}", self.cwrf())
        }
    }
    #[doc = "MDIOS input data register %s"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dinr(pub u32);
    impl Dinr {
        #[doc = "Input data received from MDIO Master during write frames"]
        #[inline(always)]
        pub const fn din(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Input data received from MDIO Master during write frames"]
        #[inline(always)]
        pub fn set_din(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dinr {
        #[inline(always)]
        fn default() -> Dinr {
            Dinr(0)
        }
    }
    impl core::fmt::Debug for Dinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dinr").field("din", &self.din()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dinr {{ din: {=u16:?} }}", self.din())
        }
    }
    #[doc = "MDIOS output data register %s"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doutr(pub u32);
    impl Doutr {
        #[doc = "Output data sent to MDIO Master during read frames"]
        #[inline(always)]
        pub const fn dout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Output data sent to MDIO Master during read frames"]
        #[inline(always)]
        pub fn set_dout(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Doutr {
        #[inline(always)]
        fn default() -> Doutr {
            Doutr(0)
        }
    }
    impl core::fmt::Debug for Doutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doutr").field("dout", &self.dout()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Doutr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Doutr {{ dout: {=u16:?} }}", self.dout())
        }
    }
    #[doc = "MDIOS read flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdfr(pub u32);
    impl Rdfr {
        #[doc = "Read flags for MDIO registers 0 to 31"]
        #[inline(always)]
        pub const fn rdf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Read flags for MDIO registers 0 to 31"]
        #[inline(always)]
        pub fn set_rdf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rdfr {
        #[inline(always)]
        fn default() -> Rdfr {
            Rdfr(0)
        }
    }
    impl core::fmt::Debug for Rdfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdfr").field("rdf", &self.rdf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rdfr {{ rdf: {=u32:?} }}", self.rdf())
        }
    }
    #[doc = "MDIOS status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Preamble error flag"]
        #[inline(always)]
        pub const fn perf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Preamble error flag"]
        #[inline(always)]
        pub fn set_perf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start error flag"]
        #[inline(always)]
        pub const fn serf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start error flag"]
        #[inline(always)]
        pub fn set_serf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Turnaround error flag"]
        #[inline(always)]
        pub const fn terf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Turnaround error flag"]
        #[inline(always)]
        pub fn set_terf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("perf", &self.perf())
                .field("serf", &self.serf())
                .field("terf", &self.terf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ perf: {=bool:?}, serf: {=bool:?}, terf: {=bool:?} }}",
                self.perf(),
                self.serf(),
                self.terf()
            )
        }
    }
    #[doc = "MDIOS write flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrfr(pub u32);
    impl Wrfr {
        #[doc = "Write flags for MDIO registers 0 to 31"]
        #[inline(always)]
        pub const fn wrf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write flags for MDIO registers 0 to 31"]
        #[inline(always)]
        pub fn set_wrf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrfr {
        #[inline(always)]
        fn default() -> Wrfr {
            Wrfr(0)
        }
    }
    impl core::fmt::Debug for Wrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrfr").field("wrf", &self.wrf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrfr {{ wrf: {=u32:?} }}", self.wrf())
        }
    }
}
