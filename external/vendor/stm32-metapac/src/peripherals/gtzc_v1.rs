#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Global TrustZone controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtzc1Tzsc {
    ptr: *mut u8,
}
unsafe impl Send for Gtzc1Tzsc {}
unsafe impl Sync for Gtzc1Tzsc {}
impl Gtzc1Tzsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GTZC1 TZSC control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[inline(always)]
    pub const fn seccfgr1(self) -> crate::common::Reg<regs::Seccfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[inline(always)]
    pub const fn seccfgr2(self) -> crate::common::Reg<regs::Seccfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 3."]
    #[inline(always)]
    pub const fn seccfgr3(self) -> crate::common::Reg<regs::Seccfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[inline(always)]
    pub const fn privcfgr1(self) -> crate::common::Reg<regs::Privcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[inline(always)]
    pub const fn privcfgr2(self) -> crate::common::Reg<regs::Privcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[inline(always)]
    pub const fn privcfgr3(self) -> crate::common::Reg<regs::Privcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region A watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm1acfgr(self) -> crate::common::Reg<regs::Mpcwm1acfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region A watermark register."]
    #[inline(always)]
    pub const fn mpcwm1ar(self) -> crate::common::Reg<regs::Mpcwm1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region B watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm1bcfgr(self) -> crate::common::Reg<regs::Mpcwm1bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region B watermark register."]
    #[inline(always)]
    pub const fn mpcwm1br(self) -> crate::common::Reg<regs::Mpcwm1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region A watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm2acfgr(self) -> crate::common::Reg<regs::Mpcwm2acfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region A watermark register."]
    #[inline(always)]
    pub const fn mpcwm2ar(self) -> crate::common::Reg<regs::Mpcwm2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region B watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm2bcfgr(self) -> crate::common::Reg<regs::Mpcwm2bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region B watermark register."]
    #[inline(always)]
    pub const fn mpcwm2br(self) -> crate::common::Reg<regs::Mpcwm2br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region A watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm3acfgr(self) -> crate::common::Reg<regs::Mpcwm3acfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region A watermark register."]
    #[inline(always)]
    pub const fn mpcwm3ar(self) -> crate::common::Reg<regs::Mpcwm3ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region B watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm3bcfgr(self) -> crate::common::Reg<regs::Mpcwm3bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region B watermark register."]
    #[inline(always)]
    pub const fn mpcwm3br(self) -> crate::common::Reg<regs::Mpcwm3br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region A watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm4acfgr(self) -> crate::common::Reg<regs::Mpcwm4acfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region A watermark register."]
    #[inline(always)]
    pub const fn mpcwm4ar(self) -> crate::common::Reg<regs::Mpcwm4ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region B watermark configuration register."]
    #[inline(always)]
    pub const fn mpcwm4bcfgr(self) -> crate::common::Reg<regs::Mpcwm4bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region B watermark register."]
    #[inline(always)]
    pub const fn mpcwm4br(self) -> crate::common::Reg<regs::Mpcwm4br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "GTZC1 TZSC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "lock the configuration of TZSC_SECCFGRx and TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "lock the configuration of TZSC_SECCFGRx and TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
        #[inline(always)]
        pub fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cr").field("lck", &self.lck()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ lck: {=bool:?} }}", self.lck())
        }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region A watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm1acfgr(pub u32);
    impl Mpcwm1acfgr {
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm1acfgr {
        #[inline(always)]
        fn default() -> Mpcwm1acfgr {
            Mpcwm1acfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm1acfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm1acfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm1acfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm1acfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region A watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm1ar(pub u32);
    impl Mpcwm1ar {
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn suba_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_suba_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub const fn suba_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub fn set_suba_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm1ar {
        #[inline(always)]
        fn default() -> Mpcwm1ar {
            Mpcwm1ar(0)
        }
    }
    impl core::fmt::Debug for Mpcwm1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm1ar")
                .field("suba_start", &self.suba_start())
                .field("suba_length", &self.suba_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm1ar {{ suba_start: {=u16:?}, suba_length: {=u16:?} }}",
                self.suba_start(),
                self.suba_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region B watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm1bcfgr(pub u32);
    impl Mpcwm1bcfgr {
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm1bcfgr {
        #[inline(always)]
        fn default() -> Mpcwm1bcfgr {
            Mpcwm1bcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm1bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm1bcfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm1bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm1bcfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 1 sub-region B watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm1br(pub u32);
    impl Mpcwm1br {
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn subb_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_subb_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub const fn subb_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub fn set_subb_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm1br {
        #[inline(always)]
        fn default() -> Mpcwm1br {
            Mpcwm1br(0)
        }
    }
    impl core::fmt::Debug for Mpcwm1br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm1br")
                .field("subb_start", &self.subb_start())
                .field("subb_length", &self.subb_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm1br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm1br {{ subb_start: {=u16:?}, subb_length: {=u16:?} }}",
                self.subb_start(),
                self.subb_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region A watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm2acfgr(pub u32);
    impl Mpcwm2acfgr {
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm2acfgr {
        #[inline(always)]
        fn default() -> Mpcwm2acfgr {
            Mpcwm2acfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm2acfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm2acfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm2acfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm2acfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region A watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm2ar(pub u32);
    impl Mpcwm2ar {
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn suba_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_suba_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub const fn suba_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub fn set_suba_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm2ar {
        #[inline(always)]
        fn default() -> Mpcwm2ar {
            Mpcwm2ar(0)
        }
    }
    impl core::fmt::Debug for Mpcwm2ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm2ar")
                .field("suba_start", &self.suba_start())
                .field("suba_length", &self.suba_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm2ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm2ar {{ suba_start: {=u16:?}, suba_length: {=u16:?} }}",
                self.suba_start(),
                self.suba_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region B watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm2bcfgr(pub u32);
    impl Mpcwm2bcfgr {
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm2bcfgr {
        #[inline(always)]
        fn default() -> Mpcwm2bcfgr {
            Mpcwm2bcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm2bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm2bcfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm2bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm2bcfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 2 sub-region B watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm2br(pub u32);
    impl Mpcwm2br {
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn subb_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_subb_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub const fn subb_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub fn set_subb_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm2br {
        #[inline(always)]
        fn default() -> Mpcwm2br {
            Mpcwm2br(0)
        }
    }
    impl core::fmt::Debug for Mpcwm2br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm2br")
                .field("subb_start", &self.subb_start())
                .field("subb_length", &self.subb_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm2br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm2br {{ subb_start: {=u16:?}, subb_length: {=u16:?} }}",
                self.subb_start(),
                self.subb_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region A watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm3acfgr(pub u32);
    impl Mpcwm3acfgr {
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm3acfgr {
        #[inline(always)]
        fn default() -> Mpcwm3acfgr {
            Mpcwm3acfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm3acfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm3acfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm3acfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm3acfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region A watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm3ar(pub u32);
    impl Mpcwm3ar {
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn suba_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_suba_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub const fn suba_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub fn set_suba_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm3ar {
        #[inline(always)]
        fn default() -> Mpcwm3ar {
            Mpcwm3ar(0)
        }
    }
    impl core::fmt::Debug for Mpcwm3ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm3ar")
                .field("suba_start", &self.suba_start())
                .field("suba_length", &self.suba_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm3ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm3ar {{ suba_start: {=u16:?}, suba_length: {=u16:?} }}",
                self.suba_start(),
                self.suba_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region B watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm3bcfgr(pub u32);
    impl Mpcwm3bcfgr {
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm3bcfgr {
        #[inline(always)]
        fn default() -> Mpcwm3bcfgr {
            Mpcwm3bcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm3bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm3bcfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm3bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm3bcfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 3 sub-region B watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm3br(pub u32);
    impl Mpcwm3br {
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn subb_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_subb_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub const fn subb_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub fn set_subb_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm3br {
        #[inline(always)]
        fn default() -> Mpcwm3br {
            Mpcwm3br(0)
        }
    }
    impl core::fmt::Debug for Mpcwm3br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm3br")
                .field("subb_start", &self.subb_start())
                .field("subb_length", &self.subb_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm3br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm3br {{ subb_start: {=u16:?}, subb_length: {=u16:?} }}",
                self.subb_start(),
                self.subb_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region A watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm4acfgr(pub u32);
    impl Mpcwm4acfgr {
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region A lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm4acfgr {
        #[inline(always)]
        fn default() -> Mpcwm4acfgr {
            Mpcwm4acfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm4acfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm4acfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm4acfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm4acfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region A watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm4ar(pub u32);
    impl Mpcwm4ar {
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn suba_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_suba_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub const fn suba_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in TZSC_MPCMWxACFGR is cleared)."]
        #[inline(always)]
        pub fn set_suba_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm4ar {
        #[inline(always)]
        fn default() -> Mpcwm4ar {
            Mpcwm4ar(0)
        }
    }
    impl core::fmt::Debug for Mpcwm4ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm4ar")
                .field("suba_start", &self.suba_start())
                .field("suba_length", &self.suba_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm4ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm4ar {{ suba_start: {=u16:?}, suba_length: {=u16:?} }}",
                self.suba_start(),
                self.suba_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region B watermark configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm4bcfgr(pub u32);
    impl Mpcwm4bcfgr {
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub const fn sren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
        #[inline(always)]
        pub fn set_sren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub const fn srlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sub-region B lock This bit, once set, can be cleared only by a system reset."]
        #[inline(always)]
        pub fn set_srlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mpcwm4bcfgr {
        #[inline(always)]
        fn default() -> Mpcwm4bcfgr {
            Mpcwm4bcfgr(0)
        }
    }
    impl core::fmt::Debug for Mpcwm4bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm4bcfgr")
                .field("sren", &self.sren())
                .field("srlock", &self.srlock())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm4bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm4bcfgr {{ sren: {=bool:?}, srlock: {=bool:?}, sec: {=bool:?}, priv_: {=bool:?} }}",
                self.sren(),
                self.srlock(),
                self.sec(),
                self.priv_()
            )
        }
    }
    #[doc = "GTZC1 TZSC memory 4 sub-region B watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpcwm4br(pub u32);
    impl Mpcwm4br {
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub const fn subb_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
        #[inline(always)]
        pub fn set_subb_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub const fn subb_length(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in TZSC_MPCMWxBCFGR is cleared)."]
        #[inline(always)]
        pub fn set_subb_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mpcwm4br {
        #[inline(always)]
        fn default() -> Mpcwm4br {
            Mpcwm4br(0)
        }
    }
    impl core::fmt::Debug for Mpcwm4br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mpcwm4br")
                .field("subb_start", &self.subb_start())
                .field("subb_length", &self.subb_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mpcwm4br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mpcwm4br {{ subb_start: {=u16:?}, subb_length: {=u16:?} }}",
                self.subb_start(),
                self.subb_length()
            )
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr1(pub u32);
    impl Privcfgr1 {
        #[doc = "privileged access mode for TIM2."]
        #[inline(always)]
        pub const fn tim2priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM2."]
        #[inline(always)]
        pub fn set_tim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "privileged access mode for TIM3."]
        #[inline(always)]
        pub const fn tim3priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM3."]
        #[inline(always)]
        pub fn set_tim3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "privileged access mode for TIM4."]
        #[inline(always)]
        pub const fn tim4priv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM4."]
        #[inline(always)]
        pub fn set_tim4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "privileged access mode for TIM5."]
        #[inline(always)]
        pub const fn tim5priv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM5."]
        #[inline(always)]
        pub fn set_tim5priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "privileged access mode for TIM6."]
        #[inline(always)]
        pub const fn tim6priv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM6."]
        #[inline(always)]
        pub fn set_tim6priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "privileged access mode for TIM7."]
        #[inline(always)]
        pub const fn tim7priv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM7."]
        #[inline(always)]
        pub fn set_tim7priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "privileged access mode for TIM12."]
        #[inline(always)]
        pub const fn tim12priv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM12."]
        #[inline(always)]
        pub fn set_tim12priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "privileged access mode for TIM13."]
        #[inline(always)]
        pub const fn tim13priv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM13."]
        #[inline(always)]
        pub fn set_tim13priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "privileged access mode for TIM14."]
        #[inline(always)]
        pub const fn tim14priv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM14."]
        #[inline(always)]
        pub fn set_tim14priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "privileged access mode for WWDG."]
        #[inline(always)]
        pub const fn wwdgpriv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for WWDG."]
        #[inline(always)]
        pub fn set_wwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "privileged access mode for IWDG."]
        #[inline(always)]
        pub const fn iwdgpriv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for IWDG."]
        #[inline(always)]
        pub fn set_iwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "privileged access mode for SPI2."]
        #[inline(always)]
        pub const fn spi2priv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI2."]
        #[inline(always)]
        pub fn set_spi2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "privileged access mode for SPI3."]
        #[inline(always)]
        pub const fn spi3priv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI3."]
        #[inline(always)]
        pub fn set_spi3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "privileged access mode for USART2."]
        #[inline(always)]
        pub const fn usart2priv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART2."]
        #[inline(always)]
        pub fn set_usart2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "privileged access mode for USART3."]
        #[inline(always)]
        pub const fn usart3priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART3."]
        #[inline(always)]
        pub fn set_usart3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "privileged access mode for UART4."]
        #[inline(always)]
        pub const fn uart4priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART4."]
        #[inline(always)]
        pub fn set_uart4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "privileged access mode for UART5."]
        #[inline(always)]
        pub const fn uart5priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART5."]
        #[inline(always)]
        pub fn set_uart5priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "privileged access mode for I2C1."]
        #[inline(always)]
        pub const fn i2c1priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C1."]
        #[inline(always)]
        pub fn set_i2c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "privileged access mode for I2C2."]
        #[inline(always)]
        pub const fn i2c2priv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C2."]
        #[inline(always)]
        pub fn set_i2c2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "privileged access mode for I3C1."]
        #[inline(always)]
        pub const fn i3c1priv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I3C1."]
        #[inline(always)]
        pub fn set_i3c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "privileged access mode for CRS."]
        #[inline(always)]
        pub const fn crspriv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for CRS."]
        #[inline(always)]
        pub fn set_crspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "privileged access mode for USART6."]
        #[inline(always)]
        pub const fn usart6priv(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART6."]
        #[inline(always)]
        pub fn set_usart6priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "privileged access mode for USART10."]
        #[inline(always)]
        pub const fn usart10priv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART10."]
        #[inline(always)]
        pub fn set_usart10priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "privileged access mode for USART11."]
        #[inline(always)]
        pub const fn usart11priv(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART11."]
        #[inline(always)]
        pub fn set_usart11priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "privileged access mode for HDMICEC."]
        #[inline(always)]
        pub const fn hdmicecpriv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for HDMICEC."]
        #[inline(always)]
        pub fn set_hdmicecpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "privileged access mode for DAC1."]
        #[inline(always)]
        pub const fn dac1priv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DAC1."]
        #[inline(always)]
        pub fn set_dac1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "privileged access mode for UART7."]
        #[inline(always)]
        pub const fn uart7priv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART7."]
        #[inline(always)]
        pub fn set_uart7priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "privileged access mode for UART8."]
        #[inline(always)]
        pub const fn uart8priv(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART8."]
        #[inline(always)]
        pub fn set_uart8priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "privileged access mode for UART9."]
        #[inline(always)]
        pub const fn uart9priv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART9."]
        #[inline(always)]
        pub fn set_uart9priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "privileged access mode for UART12."]
        #[inline(always)]
        pub const fn uart12priv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UART12."]
        #[inline(always)]
        pub fn set_uart12priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "privileged access mode for DTS."]
        #[inline(always)]
        pub const fn dtspriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DTS."]
        #[inline(always)]
        pub fn set_dtspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "privileged access mode for LPTIM2."]
        #[inline(always)]
        pub const fn lptim2priv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM2."]
        #[inline(always)]
        pub fn set_lptim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privcfgr1 {
        #[inline(always)]
        fn default() -> Privcfgr1 {
            Privcfgr1(0)
        }
    }
    impl core::fmt::Debug for Privcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr1")
                .field("tim2priv", &self.tim2priv())
                .field("tim3priv", &self.tim3priv())
                .field("tim4priv", &self.tim4priv())
                .field("tim5priv", &self.tim5priv())
                .field("tim6priv", &self.tim6priv())
                .field("tim7priv", &self.tim7priv())
                .field("tim12priv", &self.tim12priv())
                .field("tim13priv", &self.tim13priv())
                .field("tim14priv", &self.tim14priv())
                .field("wwdgpriv", &self.wwdgpriv())
                .field("iwdgpriv", &self.iwdgpriv())
                .field("spi2priv", &self.spi2priv())
                .field("spi3priv", &self.spi3priv())
                .field("usart2priv", &self.usart2priv())
                .field("usart3priv", &self.usart3priv())
                .field("uart4priv", &self.uart4priv())
                .field("uart5priv", &self.uart5priv())
                .field("i2c1priv", &self.i2c1priv())
                .field("i2c2priv", &self.i2c2priv())
                .field("i3c1priv", &self.i3c1priv())
                .field("crspriv", &self.crspriv())
                .field("usart6priv", &self.usart6priv())
                .field("usart10priv", &self.usart10priv())
                .field("usart11priv", &self.usart11priv())
                .field("hdmicecpriv", &self.hdmicecpriv())
                .field("dac1priv", &self.dac1priv())
                .field("uart7priv", &self.uart7priv())
                .field("uart8priv", &self.uart8priv())
                .field("uart9priv", &self.uart9priv())
                .field("uart12priv", &self.uart12priv())
                .field("dtspriv", &self.dtspriv())
                .field("lptim2priv", &self.lptim2priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr1 {{ tim2priv: {=bool:?}, tim3priv: {=bool:?}, tim4priv: {=bool:?}, tim5priv: {=bool:?}, tim6priv: {=bool:?}, tim7priv: {=bool:?}, tim12priv: {=bool:?}, tim13priv: {=bool:?}, tim14priv: {=bool:?}, wwdgpriv: {=bool:?}, iwdgpriv: {=bool:?}, spi2priv: {=bool:?}, spi3priv: {=bool:?}, usart2priv: {=bool:?}, usart3priv: {=bool:?}, uart4priv: {=bool:?}, uart5priv: {=bool:?}, i2c1priv: {=bool:?}, i2c2priv: {=bool:?}, i3c1priv: {=bool:?}, crspriv: {=bool:?}, usart6priv: {=bool:?}, usart10priv: {=bool:?}, usart11priv: {=bool:?}, hdmicecpriv: {=bool:?}, dac1priv: {=bool:?}, uart7priv: {=bool:?}, uart8priv: {=bool:?}, uart9priv: {=bool:?}, uart12priv: {=bool:?}, dtspriv: {=bool:?}, lptim2priv: {=bool:?} }}" , self . tim2priv () , self . tim3priv () , self . tim4priv () , self . tim5priv () , self . tim6priv () , self . tim7priv () , self . tim12priv () , self . tim13priv () , self . tim14priv () , self . wwdgpriv () , self . iwdgpriv () , self . spi2priv () , self . spi3priv () , self . usart2priv () , self . usart3priv () , self . uart4priv () , self . uart5priv () , self . i2c1priv () , self . i2c2priv () , self . i3c1priv () , self . crspriv () , self . usart6priv () , self . usart10priv () , self . usart11priv () , self . hdmicecpriv () , self . dac1priv () , self . uart7priv () , self . uart8priv () , self . uart9priv () , self . uart12priv () , self . dtspriv () , self . lptim2priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr2(pub u32);
    impl Privcfgr2 {
        #[doc = "privileged access mode for FDCAN1."]
        #[inline(always)]
        pub const fn fdcan1priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for FDCAN1."]
        #[inline(always)]
        pub fn set_fdcan1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "privileged access mode for FDCAN2."]
        #[inline(always)]
        pub const fn fdcan2priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for FDCAN2."]
        #[inline(always)]
        pub fn set_fdcan2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "privileged access mode for UCPD."]
        #[inline(always)]
        pub const fn ucpdpriv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for UCPD."]
        #[inline(always)]
        pub fn set_ucpdpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "privileged access mode for TIM1."]
        #[inline(always)]
        pub const fn tim1priv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM1."]
        #[inline(always)]
        pub fn set_tim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "privileged access mode for SPI1."]
        #[inline(always)]
        pub const fn spi1priv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI1."]
        #[inline(always)]
        pub fn set_spi1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "privileged access mode for TIM8."]
        #[inline(always)]
        pub const fn tim8priv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM8."]
        #[inline(always)]
        pub fn set_tim8priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "privileged access mode for USART1."]
        #[inline(always)]
        pub const fn usart1priv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USART1."]
        #[inline(always)]
        pub fn set_usart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "privileged access mode for TIM15."]
        #[inline(always)]
        pub const fn tim15priv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM15."]
        #[inline(always)]
        pub fn set_tim15priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "privileged access mode for TIM16."]
        #[inline(always)]
        pub const fn tim16priv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM16."]
        #[inline(always)]
        pub fn set_tim16priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "privileged access mode for TIM17."]
        #[inline(always)]
        pub const fn tim17priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for TIM17."]
        #[inline(always)]
        pub fn set_tim17priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "privileged access mode for SPI4."]
        #[inline(always)]
        pub const fn spi4priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI4."]
        #[inline(always)]
        pub fn set_spi4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "privileged access mode for SPI6."]
        #[inline(always)]
        pub const fn spi6priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI6."]
        #[inline(always)]
        pub fn set_spi6priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "privileged access mode for SAI1."]
        #[inline(always)]
        pub const fn sai1priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SAI1."]
        #[inline(always)]
        pub fn set_sai1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "privileged access mode for SAI2."]
        #[inline(always)]
        pub const fn sai2priv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SAI2."]
        #[inline(always)]
        pub fn set_sai2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "privileged access mode for USB."]
        #[inline(always)]
        pub const fn usbpriv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for USB."]
        #[inline(always)]
        pub fn set_usbpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "privileged access mode for SPI5."]
        #[inline(always)]
        pub const fn spi5priv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SPI5."]
        #[inline(always)]
        pub fn set_spi5priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "privileged access mode for LPUART."]
        #[inline(always)]
        pub const fn lpuart1priv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPUART."]
        #[inline(always)]
        pub fn set_lpuart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "privileged access mode for I2C3."]
        #[inline(always)]
        pub const fn i2c3priv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C3."]
        #[inline(always)]
        pub fn set_i2c3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "privileged access mode for I2C4."]
        #[inline(always)]
        pub const fn i2c4priv(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for I2C4."]
        #[inline(always)]
        pub fn set_i2c4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "privileged access mode for LPTIM1."]
        #[inline(always)]
        pub const fn lptim1priv(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM1."]
        #[inline(always)]
        pub fn set_lptim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "privileged access mode for LPTIM3."]
        #[inline(always)]
        pub const fn lptim3priv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM3."]
        #[inline(always)]
        pub fn set_lptim3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "privileged access mode for LPTIM4."]
        #[inline(always)]
        pub const fn lptim4priv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM4."]
        #[inline(always)]
        pub fn set_lptim4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "privileged access mode for LPTIM5."]
        #[inline(always)]
        pub const fn lptim5priv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM5."]
        #[inline(always)]
        pub fn set_lptim5priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privcfgr2 {
        #[inline(always)]
        fn default() -> Privcfgr2 {
            Privcfgr2(0)
        }
    }
    impl core::fmt::Debug for Privcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr2")
                .field("fdcan1priv", &self.fdcan1priv())
                .field("fdcan2priv", &self.fdcan2priv())
                .field("ucpdpriv", &self.ucpdpriv())
                .field("tim1priv", &self.tim1priv())
                .field("spi1priv", &self.spi1priv())
                .field("tim8priv", &self.tim8priv())
                .field("usart1priv", &self.usart1priv())
                .field("tim15priv", &self.tim15priv())
                .field("tim16priv", &self.tim16priv())
                .field("tim17priv", &self.tim17priv())
                .field("spi4priv", &self.spi4priv())
                .field("spi6priv", &self.spi6priv())
                .field("sai1priv", &self.sai1priv())
                .field("sai2priv", &self.sai2priv())
                .field("usbpriv", &self.usbpriv())
                .field("spi5priv", &self.spi5priv())
                .field("lpuart1priv", &self.lpuart1priv())
                .field("i2c3priv", &self.i2c3priv())
                .field("i2c4priv", &self.i2c4priv())
                .field("lptim1priv", &self.lptim1priv())
                .field("lptim3priv", &self.lptim3priv())
                .field("lptim4priv", &self.lptim4priv())
                .field("lptim5priv", &self.lptim5priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr2 {{ fdcan1priv: {=bool:?}, fdcan2priv: {=bool:?}, ucpdpriv: {=bool:?}, tim1priv: {=bool:?}, spi1priv: {=bool:?}, tim8priv: {=bool:?}, usart1priv: {=bool:?}, tim15priv: {=bool:?}, tim16priv: {=bool:?}, tim17priv: {=bool:?}, spi4priv: {=bool:?}, spi6priv: {=bool:?}, sai1priv: {=bool:?}, sai2priv: {=bool:?}, usbpriv: {=bool:?}, spi5priv: {=bool:?}, lpuart1priv: {=bool:?}, i2c3priv: {=bool:?}, i2c4priv: {=bool:?}, lptim1priv: {=bool:?}, lptim3priv: {=bool:?}, lptim4priv: {=bool:?}, lptim5priv: {=bool:?} }}" , self . fdcan1priv () , self . fdcan2priv () , self . ucpdpriv () , self . tim1priv () , self . spi1priv () , self . tim8priv () , self . usart1priv () , self . tim15priv () , self . tim16priv () , self . tim17priv () , self . spi4priv () , self . spi6priv () , self . sai1priv () , self . sai2priv () , self . usbpriv () , self . spi5priv () , self . lpuart1priv () , self . i2c3priv () , self . i2c4priv () , self . lptim1priv () , self . lptim3priv () , self . lptim4priv () , self . lptim5priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr3(pub u32);
    impl Privcfgr3 {
        #[doc = "privileged access mode for LPTIM6."]
        #[inline(always)]
        pub const fn lptim6priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for LPTIM6."]
        #[inline(always)]
        pub fn set_lptim6priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "privileged access mode for VREFBUF."]
        #[inline(always)]
        pub const fn vrefbufpriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for VREFBUF."]
        #[inline(always)]
        pub fn set_vrefbufpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "privileged access mode for CRC."]
        #[inline(always)]
        pub const fn crcpriv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for CRC."]
        #[inline(always)]
        pub fn set_crcpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "privileged access mode for CORDIC."]
        #[inline(always)]
        pub const fn cordicpriv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for CORDIC."]
        #[inline(always)]
        pub fn set_cordicpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "privileged access mode for FMAC."]
        #[inline(always)]
        pub const fn fmacpriv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for FMAC."]
        #[inline(always)]
        pub fn set_fmacpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "privileged access mode for ICACHE."]
        #[inline(always)]
        pub const fn icachepriv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for ICACHE."]
        #[inline(always)]
        pub fn set_icachepriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "privileged access mode for DCACHE."]
        #[inline(always)]
        pub const fn dcachepriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DCACHE."]
        #[inline(always)]
        pub fn set_dcachepriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "privileged access mode for ADC1 and ADC2."]
        #[inline(always)]
        pub const fn adc12priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for ADC1 and ADC2."]
        #[inline(always)]
        pub fn set_adc12priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "privileged access mode for DCMI."]
        #[inline(always)]
        pub const fn dcmipriv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for DCMI."]
        #[inline(always)]
        pub fn set_dcmipriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "privileged access mode for HASH."]
        #[inline(always)]
        pub const fn hashpriv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for HASH."]
        #[inline(always)]
        pub fn set_hashpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "privileged access mode for RNG."]
        #[inline(always)]
        pub const fn rngpriv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for RNG."]
        #[inline(always)]
        pub fn set_rngpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "privileged access mode for SDMMC1."]
        #[inline(always)]
        pub const fn sdmmc1priv(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for SDMMC1."]
        #[inline(always)]
        pub fn set_sdmmc1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "privileged access mode for FMC."]
        #[inline(always)]
        pub const fn fmcpriv(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for FMC."]
        #[inline(always)]
        pub fn set_fmcpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "privileged access mode for OCTOSPI1."]
        #[inline(always)]
        pub const fn octospi1priv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for OCTOSPI1."]
        #[inline(always)]
        pub fn set_octospi1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "privileged access mode for RAMSCFG."]
        #[inline(always)]
        pub const fn ramcfgpriv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "privileged access mode for RAMSCFG."]
        #[inline(always)]
        pub fn set_ramcfgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Privcfgr3 {
        #[inline(always)]
        fn default() -> Privcfgr3 {
            Privcfgr3(0)
        }
    }
    impl core::fmt::Debug for Privcfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr3")
                .field("lptim6priv", &self.lptim6priv())
                .field("vrefbufpriv", &self.vrefbufpriv())
                .field("crcpriv", &self.crcpriv())
                .field("cordicpriv", &self.cordicpriv())
                .field("fmacpriv", &self.fmacpriv())
                .field("icachepriv", &self.icachepriv())
                .field("dcachepriv", &self.dcachepriv())
                .field("adc12priv", &self.adc12priv())
                .field("dcmipriv", &self.dcmipriv())
                .field("hashpriv", &self.hashpriv())
                .field("rngpriv", &self.rngpriv())
                .field("sdmmc1priv", &self.sdmmc1priv())
                .field("fmcpriv", &self.fmcpriv())
                .field("octospi1priv", &self.octospi1priv())
                .field("ramcfgpriv", &self.ramcfgpriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr3 {{ lptim6priv: {=bool:?}, vrefbufpriv: {=bool:?}, crcpriv: {=bool:?}, cordicpriv: {=bool:?}, fmacpriv: {=bool:?}, icachepriv: {=bool:?}, dcachepriv: {=bool:?}, adc12priv: {=bool:?}, dcmipriv: {=bool:?}, hashpriv: {=bool:?}, rngpriv: {=bool:?}, sdmmc1priv: {=bool:?}, fmcpriv: {=bool:?}, octospi1priv: {=bool:?}, ramcfgpriv: {=bool:?} }}" , self . lptim6priv () , self . vrefbufpriv () , self . crcpriv () , self . cordicpriv () , self . fmacpriv () , self . icachepriv () , self . dcachepriv () , self . adc12priv () , self . dcmipriv () , self . hashpriv () , self . rngpriv () , self . sdmmc1priv () , self . fmcpriv () , self . octospi1priv () , self . ramcfgpriv ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr1(pub u32);
    impl Seccfgr1 {
        #[doc = "secure access mode for TIM2."]
        #[inline(always)]
        pub const fn tim2sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM2."]
        #[inline(always)]
        pub fn set_tim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "secure access mode for TIM3."]
        #[inline(always)]
        pub const fn tim3sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM3."]
        #[inline(always)]
        pub fn set_tim3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "secure access mode for TIM4."]
        #[inline(always)]
        pub const fn tim4sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM4."]
        #[inline(always)]
        pub fn set_tim4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "secure access mode for TIM5."]
        #[inline(always)]
        pub const fn tim5sec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM5."]
        #[inline(always)]
        pub fn set_tim5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "secure access mode for TIM6."]
        #[inline(always)]
        pub const fn tim6sec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM6."]
        #[inline(always)]
        pub fn set_tim6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "secure access mode for TIM7."]
        #[inline(always)]
        pub const fn tim7sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM7."]
        #[inline(always)]
        pub fn set_tim7sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "secure access mode for TIM12."]
        #[inline(always)]
        pub const fn tim12sec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM12."]
        #[inline(always)]
        pub fn set_tim12sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "secure access mode for TIM13."]
        #[inline(always)]
        pub const fn tim13sec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM13."]
        #[inline(always)]
        pub fn set_tim13sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "secure access mode for TIM14."]
        #[inline(always)]
        pub const fn tim14sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM14."]
        #[inline(always)]
        pub fn set_tim14sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "secure access mode for WWDG."]
        #[inline(always)]
        pub const fn wwdgsec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for WWDG."]
        #[inline(always)]
        pub fn set_wwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "secure access mode for IWDG."]
        #[inline(always)]
        pub const fn iwdgsec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for IWDG."]
        #[inline(always)]
        pub fn set_iwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "secure access mode for SPI2."]
        #[inline(always)]
        pub const fn spi2sec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI2."]
        #[inline(always)]
        pub fn set_spi2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "secure access mode for SPI3."]
        #[inline(always)]
        pub const fn spi3sec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI3."]
        #[inline(always)]
        pub fn set_spi3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "secure access mode for USART2."]
        #[inline(always)]
        pub const fn usart2sec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART2."]
        #[inline(always)]
        pub fn set_usart2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "secure access mode for USART3."]
        #[inline(always)]
        pub const fn usart3sec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART3."]
        #[inline(always)]
        pub fn set_usart3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "secure access mode for UART4."]
        #[inline(always)]
        pub const fn uart4sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART4."]
        #[inline(always)]
        pub fn set_uart4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "secure access mode for UART5."]
        #[inline(always)]
        pub const fn uart5sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART5."]
        #[inline(always)]
        pub fn set_uart5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "secure access mode for I2C1."]
        #[inline(always)]
        pub const fn i2c1sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for I2C1."]
        #[inline(always)]
        pub fn set_i2c1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "secure access mode for I2C2."]
        #[inline(always)]
        pub const fn i2c2sec(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for I2C2."]
        #[inline(always)]
        pub fn set_i2c2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "secure access mode for I3C1."]
        #[inline(always)]
        pub const fn i3c1sec(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for I3C1."]
        #[inline(always)]
        pub fn set_i3c1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "secure access mode for CRS."]
        #[inline(always)]
        pub const fn crssec(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for CRS."]
        #[inline(always)]
        pub fn set_crssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "secure access mode for USART6."]
        #[inline(always)]
        pub const fn usart6sec(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART6."]
        #[inline(always)]
        pub fn set_usart6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "secure access mode for USART10."]
        #[inline(always)]
        pub const fn usart10sec(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART10."]
        #[inline(always)]
        pub fn set_usart10sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "secure access mode for USART11."]
        #[inline(always)]
        pub const fn usart11sec(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART11."]
        #[inline(always)]
        pub fn set_usart11sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "secure access mode for HDMICEC."]
        #[inline(always)]
        pub const fn hdmicecsec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for HDMICEC."]
        #[inline(always)]
        pub fn set_hdmicecsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "secure access mode for DAC1."]
        #[inline(always)]
        pub const fn dac1sec(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for DAC1."]
        #[inline(always)]
        pub fn set_dac1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "secure access mode for UART7."]
        #[inline(always)]
        pub const fn uart7sec(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART7."]
        #[inline(always)]
        pub fn set_uart7sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "secure access mode for UART8."]
        #[inline(always)]
        pub const fn uart8sec(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART8."]
        #[inline(always)]
        pub fn set_uart8sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "secure access mode for UART9."]
        #[inline(always)]
        pub const fn uart9sec(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART9."]
        #[inline(always)]
        pub fn set_uart9sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "secure access mode for UART12."]
        #[inline(always)]
        pub const fn uart12sec(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UART12."]
        #[inline(always)]
        pub fn set_uart12sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "secure access mode for DTS."]
        #[inline(always)]
        pub const fn dtssec(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for DTS."]
        #[inline(always)]
        pub fn set_dtssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "secure access mode for LPTIM2."]
        #[inline(always)]
        pub const fn lptim2sec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM2."]
        #[inline(always)]
        pub fn set_lptim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccfgr1 {
        #[inline(always)]
        fn default() -> Seccfgr1 {
            Seccfgr1(0)
        }
    }
    impl core::fmt::Debug for Seccfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr1")
                .field("tim2sec", &self.tim2sec())
                .field("tim3sec", &self.tim3sec())
                .field("tim4sec", &self.tim4sec())
                .field("tim5sec", &self.tim5sec())
                .field("tim6sec", &self.tim6sec())
                .field("tim7sec", &self.tim7sec())
                .field("tim12sec", &self.tim12sec())
                .field("tim13sec", &self.tim13sec())
                .field("tim14sec", &self.tim14sec())
                .field("wwdgsec", &self.wwdgsec())
                .field("iwdgsec", &self.iwdgsec())
                .field("spi2sec", &self.spi2sec())
                .field("spi3sec", &self.spi3sec())
                .field("usart2sec", &self.usart2sec())
                .field("usart3sec", &self.usart3sec())
                .field("uart4sec", &self.uart4sec())
                .field("uart5sec", &self.uart5sec())
                .field("i2c1sec", &self.i2c1sec())
                .field("i2c2sec", &self.i2c2sec())
                .field("i3c1sec", &self.i3c1sec())
                .field("crssec", &self.crssec())
                .field("usart6sec", &self.usart6sec())
                .field("usart10sec", &self.usart10sec())
                .field("usart11sec", &self.usart11sec())
                .field("hdmicecsec", &self.hdmicecsec())
                .field("dac1sec", &self.dac1sec())
                .field("uart7sec", &self.uart7sec())
                .field("uart8sec", &self.uart8sec())
                .field("uart9sec", &self.uart9sec())
                .field("uart12sec", &self.uart12sec())
                .field("dtssec", &self.dtssec())
                .field("lptim2sec", &self.lptim2sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr1 {{ tim2sec: {=bool:?}, tim3sec: {=bool:?}, tim4sec: {=bool:?}, tim5sec: {=bool:?}, tim6sec: {=bool:?}, tim7sec: {=bool:?}, tim12sec: {=bool:?}, tim13sec: {=bool:?}, tim14sec: {=bool:?}, wwdgsec: {=bool:?}, iwdgsec: {=bool:?}, spi2sec: {=bool:?}, spi3sec: {=bool:?}, usart2sec: {=bool:?}, usart3sec: {=bool:?}, uart4sec: {=bool:?}, uart5sec: {=bool:?}, i2c1sec: {=bool:?}, i2c2sec: {=bool:?}, i3c1sec: {=bool:?}, crssec: {=bool:?}, usart6sec: {=bool:?}, usart10sec: {=bool:?}, usart11sec: {=bool:?}, hdmicecsec: {=bool:?}, dac1sec: {=bool:?}, uart7sec: {=bool:?}, uart8sec: {=bool:?}, uart9sec: {=bool:?}, uart12sec: {=bool:?}, dtssec: {=bool:?}, lptim2sec: {=bool:?} }}" , self . tim2sec () , self . tim3sec () , self . tim4sec () , self . tim5sec () , self . tim6sec () , self . tim7sec () , self . tim12sec () , self . tim13sec () , self . tim14sec () , self . wwdgsec () , self . iwdgsec () , self . spi2sec () , self . spi3sec () , self . usart2sec () , self . usart3sec () , self . uart4sec () , self . uart5sec () , self . i2c1sec () , self . i2c2sec () , self . i3c1sec () , self . crssec () , self . usart6sec () , self . usart10sec () , self . usart11sec () , self . hdmicecsec () , self . dac1sec () , self . uart7sec () , self . uart8sec () , self . uart9sec () , self . uart12sec () , self . dtssec () , self . lptim2sec ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr2(pub u32);
    impl Seccfgr2 {
        #[doc = "secure access mode for FDCAN1."]
        #[inline(always)]
        pub const fn fdcan1sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for FDCAN1."]
        #[inline(always)]
        pub fn set_fdcan1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "secure access mode for FDCAN2."]
        #[inline(always)]
        pub const fn fdcan2sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for FDCAN2."]
        #[inline(always)]
        pub fn set_fdcan2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "secure access mode for UCPD."]
        #[inline(always)]
        pub const fn ucpdsec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for UCPD."]
        #[inline(always)]
        pub fn set_ucpdsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "secure access mode for TIM1."]
        #[inline(always)]
        pub const fn tim1sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM1."]
        #[inline(always)]
        pub fn set_tim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "secure access mode for SPI1."]
        #[inline(always)]
        pub const fn spi1sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI1."]
        #[inline(always)]
        pub fn set_spi1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "secure access mode for TIM8."]
        #[inline(always)]
        pub const fn tim8sec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM8."]
        #[inline(always)]
        pub fn set_tim8sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "secure access mode for USART1."]
        #[inline(always)]
        pub const fn usart1sec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USART1."]
        #[inline(always)]
        pub fn set_usart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "secure access mode for TIM15."]
        #[inline(always)]
        pub const fn tim15sec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM15."]
        #[inline(always)]
        pub fn set_tim15sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "secure access mode for TIM16."]
        #[inline(always)]
        pub const fn tim16sec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM16."]
        #[inline(always)]
        pub fn set_tim16sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "secure access mode for TIM17."]
        #[inline(always)]
        pub const fn tim17sec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for TIM17."]
        #[inline(always)]
        pub fn set_tim17sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "secure access mode for SPI4."]
        #[inline(always)]
        pub const fn spi4sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI4."]
        #[inline(always)]
        pub fn set_spi4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "secure access mode for SPI6."]
        #[inline(always)]
        pub const fn spi6sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI6."]
        #[inline(always)]
        pub fn set_spi6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "secure access mode for SAI1."]
        #[inline(always)]
        pub const fn sai1sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SAI1."]
        #[inline(always)]
        pub fn set_sai1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "secure access mode for SAI2."]
        #[inline(always)]
        pub const fn sai2sec(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SAI2."]
        #[inline(always)]
        pub fn set_sai2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "secure access mode for USB."]
        #[inline(always)]
        pub const fn usbsec(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for USB."]
        #[inline(always)]
        pub fn set_usbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "secure access mode for SPI5."]
        #[inline(always)]
        pub const fn spi5sec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SPI5."]
        #[inline(always)]
        pub fn set_spi5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "secure access mode for LPUART."]
        #[inline(always)]
        pub const fn lpuart1sec(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPUART."]
        #[inline(always)]
        pub fn set_lpuart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "secure access mode for I2C3."]
        #[inline(always)]
        pub const fn i2c3sec(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for I2C3."]
        #[inline(always)]
        pub fn set_i2c3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "secure access mode for I2C4."]
        #[inline(always)]
        pub const fn i2c4sec(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for I2C4."]
        #[inline(always)]
        pub fn set_i2c4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "secure access mode for LPTIM1."]
        #[inline(always)]
        pub const fn lptim1sec(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM1."]
        #[inline(always)]
        pub fn set_lptim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "secure access mode for LPTIM3."]
        #[inline(always)]
        pub const fn lptim3sec(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM3."]
        #[inline(always)]
        pub fn set_lptim3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "secure access mode for LPTIM4."]
        #[inline(always)]
        pub const fn lptim4sec(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM4."]
        #[inline(always)]
        pub fn set_lptim4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "secure access mode for LPTIM5."]
        #[inline(always)]
        pub const fn lptim5sec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM5."]
        #[inline(always)]
        pub fn set_lptim5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccfgr2 {
        #[inline(always)]
        fn default() -> Seccfgr2 {
            Seccfgr2(0)
        }
    }
    impl core::fmt::Debug for Seccfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr2")
                .field("fdcan1sec", &self.fdcan1sec())
                .field("fdcan2sec", &self.fdcan2sec())
                .field("ucpdsec", &self.ucpdsec())
                .field("tim1sec", &self.tim1sec())
                .field("spi1sec", &self.spi1sec())
                .field("tim8sec", &self.tim8sec())
                .field("usart1sec", &self.usart1sec())
                .field("tim15sec", &self.tim15sec())
                .field("tim16sec", &self.tim16sec())
                .field("tim17sec", &self.tim17sec())
                .field("spi4sec", &self.spi4sec())
                .field("spi6sec", &self.spi6sec())
                .field("sai1sec", &self.sai1sec())
                .field("sai2sec", &self.sai2sec())
                .field("usbsec", &self.usbsec())
                .field("spi5sec", &self.spi5sec())
                .field("lpuart1sec", &self.lpuart1sec())
                .field("i2c3sec", &self.i2c3sec())
                .field("i2c4sec", &self.i2c4sec())
                .field("lptim1sec", &self.lptim1sec())
                .field("lptim3sec", &self.lptim3sec())
                .field("lptim4sec", &self.lptim4sec())
                .field("lptim5sec", &self.lptim5sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr2 {{ fdcan1sec: {=bool:?}, fdcan2sec: {=bool:?}, ucpdsec: {=bool:?}, tim1sec: {=bool:?}, spi1sec: {=bool:?}, tim8sec: {=bool:?}, usart1sec: {=bool:?}, tim15sec: {=bool:?}, tim16sec: {=bool:?}, tim17sec: {=bool:?}, spi4sec: {=bool:?}, spi6sec: {=bool:?}, sai1sec: {=bool:?}, sai2sec: {=bool:?}, usbsec: {=bool:?}, spi5sec: {=bool:?}, lpuart1sec: {=bool:?}, i2c3sec: {=bool:?}, i2c4sec: {=bool:?}, lptim1sec: {=bool:?}, lptim3sec: {=bool:?}, lptim4sec: {=bool:?}, lptim5sec: {=bool:?} }}" , self . fdcan1sec () , self . fdcan2sec () , self . ucpdsec () , self . tim1sec () , self . spi1sec () , self . tim8sec () , self . usart1sec () , self . tim15sec () , self . tim16sec () , self . tim17sec () , self . spi4sec () , self . spi6sec () , self . sai1sec () , self . sai2sec () , self . usbsec () , self . spi5sec () , self . lpuart1sec () , self . i2c3sec () , self . i2c4sec () , self . lptim1sec () , self . lptim3sec () , self . lptim4sec () , self . lptim5sec ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr3(pub u32);
    impl Seccfgr3 {
        #[doc = "secure access mode for LPTIM6."]
        #[inline(always)]
        pub const fn lptim6sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for LPTIM6."]
        #[inline(always)]
        pub fn set_lptim6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "secure access mode for VREFBUF."]
        #[inline(always)]
        pub const fn vrefbufsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for VREFBUF."]
        #[inline(always)]
        pub fn set_vrefbufsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "secure access mode for CRC."]
        #[inline(always)]
        pub const fn crcsec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for CRC."]
        #[inline(always)]
        pub fn set_crcsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "secure access mode for CORDIC."]
        #[inline(always)]
        pub const fn cordicsec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for CORDIC."]
        #[inline(always)]
        pub fn set_cordicsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "secure access mode for FMAC."]
        #[inline(always)]
        pub const fn fmacsec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for FMAC."]
        #[inline(always)]
        pub fn set_fmacsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "secure access mode for ICACHE."]
        #[inline(always)]
        pub const fn icachesec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for ICACHE."]
        #[inline(always)]
        pub fn set_icachesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "secure access mode for DCACHE."]
        #[inline(always)]
        pub const fn dcachesec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for DCACHE."]
        #[inline(always)]
        pub fn set_dcachesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "secure access mode for ADC1 and ADC2."]
        #[inline(always)]
        pub const fn adc12sec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for ADC1 and ADC2."]
        #[inline(always)]
        pub fn set_adc12sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "secure access mode for DCMI."]
        #[inline(always)]
        pub const fn dcmisec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for DCMI."]
        #[inline(always)]
        pub fn set_dcmisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "secure access mode for HASH."]
        #[inline(always)]
        pub const fn hashsec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for HASH."]
        #[inline(always)]
        pub fn set_hashsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "secure access mode for RNG."]
        #[inline(always)]
        pub const fn rngsec(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for RNG."]
        #[inline(always)]
        pub fn set_rngsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "secure access mode for SDMMC1."]
        #[inline(always)]
        pub const fn sdmmc1sec(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for SDMMC1."]
        #[inline(always)]
        pub fn set_sdmmc1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "secure access mode for FMC."]
        #[inline(always)]
        pub const fn fmcsec(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for FMC."]
        #[inline(always)]
        pub fn set_fmcsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "secure access mode for OCTOSPI1."]
        #[inline(always)]
        pub const fn octospi1sec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for OCTOSPI1."]
        #[inline(always)]
        pub fn set_octospi1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "secure access mode for RAMSCFG."]
        #[inline(always)]
        pub const fn ramcfgsec(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "secure access mode for RAMSCFG."]
        #[inline(always)]
        pub fn set_ramcfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Seccfgr3 {
        #[inline(always)]
        fn default() -> Seccfgr3 {
            Seccfgr3(0)
        }
    }
    impl core::fmt::Debug for Seccfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr3")
                .field("lptim6sec", &self.lptim6sec())
                .field("vrefbufsec", &self.vrefbufsec())
                .field("crcsec", &self.crcsec())
                .field("cordicsec", &self.cordicsec())
                .field("fmacsec", &self.fmacsec())
                .field("icachesec", &self.icachesec())
                .field("dcachesec", &self.dcachesec())
                .field("adc12sec", &self.adc12sec())
                .field("dcmisec", &self.dcmisec())
                .field("hashsec", &self.hashsec())
                .field("rngsec", &self.rngsec())
                .field("sdmmc1sec", &self.sdmmc1sec())
                .field("fmcsec", &self.fmcsec())
                .field("octospi1sec", &self.octospi1sec())
                .field("ramcfgsec", &self.ramcfgsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr3 {{ lptim6sec: {=bool:?}, vrefbufsec: {=bool:?}, crcsec: {=bool:?}, cordicsec: {=bool:?}, fmacsec: {=bool:?}, icachesec: {=bool:?}, dcachesec: {=bool:?}, adc12sec: {=bool:?}, dcmisec: {=bool:?}, hashsec: {=bool:?}, rngsec: {=bool:?}, sdmmc1sec: {=bool:?}, fmcsec: {=bool:?}, octospi1sec: {=bool:?}, ramcfgsec: {=bool:?} }}" , self . lptim6sec () , self . vrefbufsec () , self . crcsec () , self . cordicsec () , self . fmacsec () , self . icachesec () , self . dcachesec () , self . adc12sec () , self . dcmisec () , self . hashsec () , self . rngsec () , self . sdmmc1sec () , self . fmcsec () , self . octospi1sec () , self . ramcfgsec ())
        }
    }
}
