#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RAMs configuration controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramcfg {
    ptr: *mut u8,
}
unsafe impl Send for Ramcfg {}
unsafe impl Sync for Ramcfg {}
impl Ramcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RAMCFG memory 1 control register."]
    #[inline(always)]
    pub const fn m1cr(self) -> crate::common::Reg<regs::M1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m1isr(self) -> crate::common::Reg<regs::M1isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RAMCFG memory 1 erase key register."]
    #[inline(always)]
    pub const fn m1erkeyr(self) -> crate::common::Reg<regs::M1erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RAMCFG memory 2 control register."]
    #[inline(always)]
    pub const fn m2cr(self) -> crate::common::Reg<regs::M2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "RAMCFG memory 2 interrupt enable register."]
    #[inline(always)]
    pub const fn m2ier(self) -> crate::common::Reg<regs::M2ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m2isr(self) -> crate::common::Reg<regs::M2isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC single error address register."]
    #[inline(always)]
    pub const fn m2sear(self) -> crate::common::Reg<regs::M2sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC double error address register."]
    #[inline(always)]
    pub const fn m2dear(self) -> crate::common::Reg<regs::M2dear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RAMCFG memory 2 interrupt clear register 2."]
    #[inline(always)]
    pub const fn m2icr(self) -> crate::common::Reg<regs::M2icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RAMCFG memory 2 write protection register 1."]
    #[inline(always)]
    pub const fn m2wpr1(self) -> crate::common::Reg<regs::M2wpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC key register."]
    #[inline(always)]
    pub const fn m2ecckeyr(self) -> crate::common::Reg<regs::M2ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RAMCFG memory 2 erase key register."]
    #[inline(always)]
    pub const fn m2erkeyr(self) -> crate::common::Reg<regs::M2erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RAMCFG memory 3 interrupt enable register."]
    #[inline(always)]
    pub const fn m3ier(self) -> crate::common::Reg<regs::M3ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m3isr(self) -> crate::common::Reg<regs::M3isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC single error address register."]
    #[inline(always)]
    pub const fn m3sear(self) -> crate::common::Reg<regs::M3sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC double error address register."]
    #[inline(always)]
    pub const fn m3dear(self) -> crate::common::Reg<regs::M3dear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RAMCFG memory 3 interrupt clear register 3."]
    #[inline(always)]
    pub const fn m3icr(self) -> crate::common::Reg<regs::M3icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC key register."]
    #[inline(always)]
    pub const fn m3ecckeyr(self) -> crate::common::Reg<regs::M3ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RAMCFG memory 3 erase key register."]
    #[inline(always)]
    pub const fn m3erkeyr(self) -> crate::common::Reg<regs::M3erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RAMCFG memory 4 erase key register."]
    #[inline(always)]
    pub const fn m4erkeyr(self) -> crate::common::Reg<regs::M4erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RAMCFG memory 5 control register."]
    #[inline(always)]
    pub const fn m5cr(self) -> crate::common::Reg<regs::M5cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RAMCFG memory 5 interrupt enable register."]
    #[inline(always)]
    pub const fn m5ier(self) -> crate::common::Reg<regs::M5ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m5isr(self) -> crate::common::Reg<regs::M5isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC single error address register."]
    #[inline(always)]
    pub const fn m5sear(self) -> crate::common::Reg<regs::M5sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC double error address register."]
    #[inline(always)]
    pub const fn m5dear(self) -> crate::common::Reg<regs::M5dear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RAMCFG memory 5 interrupt clear register 5."]
    #[inline(always)]
    pub const fn m5icr(self) -> crate::common::Reg<regs::M5icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC key register."]
    #[inline(always)]
    pub const fn m5ecckeyr(self) -> crate::common::Reg<regs::M5ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "RAMCFG memory 5 erase key register."]
    #[inline(always)]
    pub const fn m5erkeyr(self) -> crate::common::Reg<regs::M5erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
}
pub mod regs {
    #[doc = "RAMCFG memory 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1cr(pub u32);
    impl M1cr {
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M1cr {
        #[inline(always)]
        fn default() -> M1cr {
            M1cr(0)
        }
    }
    impl core::fmt::Debug for M1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M1cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 1 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1erkeyr(pub u32);
    impl M1erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M1erkeyr {
        #[inline(always)]
        fn default() -> M1erkeyr {
            M1erkeyr(0)
        }
    }
    impl core::fmt::Debug for M1erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M1erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1isr(pub u32);
    impl M1isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M1isr {
        #[inline(always)]
        fn default() -> M1isr {
            M1isr(0)
        }
    }
    impl core::fmt::Debug for M1isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M1isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2cr(pub u32);
    impl M2cr {
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M2cr {
        #[inline(always)]
        fn default() -> M2cr {
            M2cr(0)
        }
    }
    impl core::fmt::Debug for M2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 2 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2dear(pub u32);
    impl M2dear {
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M2dear {
        #[inline(always)]
        fn default() -> M2dear {
            M2dear(0)
        }
    }
    impl core::fmt::Debug for M2dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 2 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ecckeyr(pub u32);
    impl M2ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M2ecckeyr {
        #[inline(always)]
        fn default() -> M2ecckeyr {
            M2ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M2ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 2 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2erkeyr(pub u32);
    impl M2erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M2erkeyr {
        #[inline(always)]
        fn default() -> M2erkeyr {
            M2erkeyr(0)
        }
    }
    impl core::fmt::Debug for M2erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 2 interrupt clear register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2icr(pub u32);
    impl M2icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M2icr {
        #[inline(always)]
        fn default() -> M2icr {
            M2icr(0)
        }
    }
    impl core::fmt::Debug for M2icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 2 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ier(pub u32);
    impl M2ier {
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M2ier {
        #[inline(always)]
        fn default() -> M2ier {
            M2ier(0)
        }
    }
    impl core::fmt::Debug for M2ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2isr(pub u32);
    impl M2isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M2isr {
        #[inline(always)]
        fn default() -> M2isr {
            M2isr(0)
        }
    }
    impl core::fmt::Debug for M2isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 2 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2sear(pub u32);
    impl M2sear {
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M2sear {
        #[inline(always)]
        fn default() -> M2sear {
            M2sear(0)
        }
    }
    impl core::fmt::Debug for M2sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG memory 2 write protection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr1(pub u32);
    impl M2wpr1 {
        #[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr1 {
        #[inline(always)]
        fn default() -> M2wpr1 {
            M2wpr1(0)
        }
    }
    impl core::fmt::Debug for M2wpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr1")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "M2wpr1 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize))
        }
    }
    #[doc = "RAMCFG memory 3 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3dear(pub u32);
    impl M3dear {
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M3dear {
        #[inline(always)]
        fn default() -> M3dear {
            M3dear(0)
        }
    }
    impl core::fmt::Debug for M3dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 3 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3ecckeyr(pub u32);
    impl M3ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M3ecckeyr {
        #[inline(always)]
        fn default() -> M3ecckeyr {
            M3ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M3ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 3 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3erkeyr(pub u32);
    impl M3erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M3erkeyr {
        #[inline(always)]
        fn default() -> M3erkeyr {
            M3erkeyr(0)
        }
    }
    impl core::fmt::Debug for M3erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 3 interrupt clear register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3icr(pub u32);
    impl M3icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M3icr {
        #[inline(always)]
        fn default() -> M3icr {
            M3icr(0)
        }
    }
    impl core::fmt::Debug for M3icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 3 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3ier(pub u32);
    impl M3ier {
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M3ier {
        #[inline(always)]
        fn default() -> M3ier {
            M3ier(0)
        }
    }
    impl core::fmt::Debug for M3ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3isr(pub u32);
    impl M3isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M3isr {
        #[inline(always)]
        fn default() -> M3isr {
            M3isr(0)
        }
    }
    impl core::fmt::Debug for M3isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 3 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3sear(pub u32);
    impl M3sear {
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M3sear {
        #[inline(always)]
        fn default() -> M3sear {
            M3sear(0)
        }
    }
    impl core::fmt::Debug for M3sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG memory 4 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M4erkeyr(pub u32);
    impl M4erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M4erkeyr {
        #[inline(always)]
        fn default() -> M4erkeyr {
            M4erkeyr(0)
        }
    }
    impl core::fmt::Debug for M4erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M4erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M4erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M4erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 5 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5cr(pub u32);
    impl M5cr {
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M5cr {
        #[inline(always)]
        fn default() -> M5cr {
            M5cr(0)
        }
    }
    impl core::fmt::Debug for M5cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 5 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5dear(pub u32);
    impl M5dear {
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M5dear {
        #[inline(always)]
        fn default() -> M5dear {
            M5dear(0)
        }
    }
    impl core::fmt::Debug for M5dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 5 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5ecckeyr(pub u32);
    impl M5ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M5ecckeyr {
        #[inline(always)]
        fn default() -> M5ecckeyr {
            M5ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M5ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 5 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5erkeyr(pub u32);
    impl M5erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M5erkeyr {
        #[inline(always)]
        fn default() -> M5erkeyr {
            M5erkeyr(0)
        }
    }
    impl core::fmt::Debug for M5erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 5 interrupt clear register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5icr(pub u32);
    impl M5icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M5icr {
        #[inline(always)]
        fn default() -> M5icr {
            M5icr(0)
        }
    }
    impl core::fmt::Debug for M5icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 5 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5ier(pub u32);
    impl M5ier {
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M5ier {
        #[inline(always)]
        fn default() -> M5ier {
            M5ier(0)
        }
    }
    impl core::fmt::Debug for M5ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5isr(pub u32);
    impl M5isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M5isr {
        #[inline(always)]
        fn default() -> M5isr {
            M5isr(0)
        }
    }
    impl core::fmt::Debug for M5isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 5 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5sear(pub u32);
    impl M5sear {
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M5sear {
        #[inline(always)]
        fn default() -> M5sear {
            M5sear(0)
        }
    }
    impl core::fmt::Debug for M5sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
}
