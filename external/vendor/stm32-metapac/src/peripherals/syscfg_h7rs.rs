#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration, boot and security."]
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
    #[doc = "SBS boot status register."]
    #[inline(always)]
    pub const fn bootsr(self) -> crate::common::Reg<regs::Bootsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SBS hide protection control register."]
    #[inline(always)]
    pub const fn hdplcr(self) -> crate::common::Reg<regs::Hdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SBS hide protection status register."]
    #[inline(always)]
    pub const fn hdplsr(self) -> crate::common::Reg<regs::Hdplsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SBS debug control register."]
    #[inline(always)]
    pub const fn dbgcr(self) -> crate::common::Reg<regs::Dbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SBS debug lock register."]
    #[inline(always)]
    pub const fn dbglockr(self) -> crate::common::Reg<regs::Dbglockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SBS RSS command register."]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SBS product mode and configuration register."]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SBS FPU interrupt mask register."]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SBS memory erase status register."]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SBS I/O compensation cell control and status register."]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os value register."]
    #[inline(always)]
    pub const fn ccvalr(self) -> crate::common::Reg<regs::Ccvalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os software value register."]
    #[inline(always)]
    pub const fn ccswvalr(self) -> crate::common::Reg<regs::Ccswvalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "SBS break lockup register."]
    #[inline(always)]
    pub const fn bklockr(self) -> crate::common::Reg<regs::Bklockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "external interrupt configuration register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "SBS break lockup register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bklockr(pub u32);
    impl Bklockr {
        #[doc = "PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\\[2:0\\]
bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn pvd_bl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\\[2:0\\]
bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_pvd_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn flashecc_bl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_flashecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn cm7lckup_bl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_cm7lckup_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn bkramecc_bl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_bkramecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC."]
        #[inline(always)]
        pub const fn dtcmecc_bl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC."]
        #[inline(always)]
        pub fn set_dtcmecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn itcmecc_bl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_itcmecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn aram3ecc_bl(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_aram3ecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub const fn aram1ecc_bl(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset."]
        #[inline(always)]
        pub fn set_aram1ecc_bl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Bklockr {
        #[inline(always)]
        fn default() -> Bklockr {
            Bklockr(0)
        }
    }
    impl core::fmt::Debug for Bklockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bklockr")
                .field("pvd_bl", &self.pvd_bl())
                .field("flashecc_bl", &self.flashecc_bl())
                .field("cm7lckup_bl", &self.cm7lckup_bl())
                .field("bkramecc_bl", &self.bkramecc_bl())
                .field("dtcmecc_bl", &self.dtcmecc_bl())
                .field("itcmecc_bl", &self.itcmecc_bl())
                .field("aram3ecc_bl", &self.aram3ecc_bl())
                .field("aram1ecc_bl", &self.aram1ecc_bl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bklockr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bklockr {{ pvd_bl: {=bool:?}, flashecc_bl: {=bool:?}, cm7lckup_bl: {=bool:?}, bkramecc_bl: {=bool:?}, dtcmecc_bl: {=bool:?}, itcmecc_bl: {=bool:?}, aram3ecc_bl: {=bool:?}, aram1ecc_bl: {=bool:?} }}" , self . pvd_bl () , self . flashecc_bl () , self . cm7lckup_bl () , self . bkramecc_bl () , self . dtcmecc_bl () , self . itcmecc_bl () , self . aram3ecc_bl () , self . aram1ecc_bl ())
        }
    }
    #[doc = "SBS boot status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootsr(pub u32);
    impl Bootsr {
        #[doc = "initial vector for Cortex-M7 This register includes the physical boot address used by the Cortex-M7 after reset."]
        #[inline(always)]
        pub const fn initvtor(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "initial vector for Cortex-M7 This register includes the physical boot address used by the Cortex-M7 after reset."]
        #[inline(always)]
        pub fn set_initvtor(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bootsr {
        #[inline(always)]
        fn default() -> Bootsr {
            Bootsr(0)
        }
    }
    impl core::fmt::Debug for Bootsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootsr").field("initvtor", &self.initvtor()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bootsr {{ initvtor: {=u32:?} }}", self.initvtor())
        }
    }
    #[doc = "SBS I/O compensation cell control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "Compensation cell enable Set this bit to enable the compensation cell."]
        #[inline(always)]
        pub const fn comp_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell enable Set this bit to enable the compensation cell."]
        #[inline(always)]
        pub fn set_comp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell."]
        #[inline(always)]
        pub const fn comp_codesel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell."]
        #[inline(always)]
        pub fn set_comp_codesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell."]
        #[inline(always)]
        pub const fn octo1_comp_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell."]
        #[inline(always)]
        pub fn set_octo1_comp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell."]
        #[inline(always)]
        pub const fn octo1_comp_codesel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell."]
        #[inline(always)]
        pub fn set_octo1_comp_codesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "XSPIM_P2 compensation cell enable Set this bit to enable the XSPIM_P2 compensation cell."]
        #[inline(always)]
        pub const fn octo2_comp_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 compensation cell enable Set this bit to enable the XSPIM_P2 compensation cell."]
        #[inline(always)]
        pub fn set_octo2_comp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPIM_P2 compensation cell code selection This bit selects the code to be applied for the XSPIM_P2 I/O compensation cell."]
        #[inline(always)]
        pub const fn octo2_comp_codesel(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 compensation cell code selection This bit selects the code to be applied for the XSPIM_P2 I/O compensation cell."]
        #[inline(always)]
        pub fn set_octo2_comp_codesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Compensation cell ready This bit provides the status of the compensation cell."]
        #[inline(always)]
        pub const fn comp_rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell ready This bit provides the status of the compensation cell."]
        #[inline(always)]
        pub fn set_comp_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "XSPIM_P1 compensation cell ready This bit provides the status of the XSPIM_P1 compensation cell."]
        #[inline(always)]
        pub const fn octo1_comp_rdy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 compensation cell ready This bit provides the status of the XSPIM_P1 compensation cell."]
        #[inline(always)]
        pub fn set_octo1_comp_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "XSPIM_P2 compensation cell ready This bit provides the status of the XSPIM_P2 compensation cell."]
        #[inline(always)]
        pub const fn octo2_comp_rdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 compensation cell ready This bit provides the status of the XSPIM_P2 compensation cell."]
        #[inline(always)]
        pub fn set_octo2_comp_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub const fn iohslv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub fn set_iohslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub const fn octo1_iohslv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub fn set_octo1_iohslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub const fn octo2_iohslv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive."]
        #[inline(always)]
        pub fn set_octo2_iohslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("comp_en", &self.comp_en())
                .field("comp_codesel", &self.comp_codesel())
                .field("octo1_comp_en", &self.octo1_comp_en())
                .field("octo1_comp_codesel", &self.octo1_comp_codesel())
                .field("octo2_comp_en", &self.octo2_comp_en())
                .field("octo2_comp_codesel", &self.octo2_comp_codesel())
                .field("comp_rdy", &self.comp_rdy())
                .field("octo1_comp_rdy", &self.octo1_comp_rdy())
                .field("octo2_comp_rdy", &self.octo2_comp_rdy())
                .field("iohslv", &self.iohslv())
                .field("octo1_iohslv", &self.octo1_iohslv())
                .field("octo2_iohslv", &self.octo2_iohslv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccsr {{ comp_en: {=bool:?}, comp_codesel: {=bool:?}, octo1_comp_en: {=bool:?}, octo1_comp_codesel: {=bool:?}, octo2_comp_en: {=bool:?}, octo2_comp_codesel: {=bool:?}, comp_rdy: {=bool:?}, octo1_comp_rdy: {=bool:?}, octo2_comp_rdy: {=bool:?}, iohslv: {=bool:?}, octo1_iohslv: {=bool:?}, octo2_iohslv: {=bool:?} }}" , self . comp_en () , self . comp_codesel () , self . octo1_comp_en () , self . octo1_comp_codesel () , self . octo2_comp_en () , self . octo2_comp_codesel () , self . comp_rdy () , self . octo1_comp_rdy () , self . octo2_comp_rdy () , self . iohslv () , self . octo1_iohslv () , self . octo2_iohslv ())
        }
    }
    #[doc = "SBS compensation cell for I/Os software value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccswvalr(pub u32);
    impl Ccswvalr {
        #[doc = "Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn sw_nsrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_sw_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn sw_psrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_sw_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo1_sw_nsrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo1_sw_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo1_sw_psrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo1_sw_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo2_sw_nsrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo2_sw_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo2_sw_psrc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo2_sw_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Ccswvalr {
        #[inline(always)]
        fn default() -> Ccswvalr {
            Ccswvalr(0)
        }
    }
    impl core::fmt::Debug for Ccswvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccswvalr")
                .field("sw_nsrc", &self.sw_nsrc())
                .field("sw_psrc", &self.sw_psrc())
                .field("octo1_sw_nsrc", &self.octo1_sw_nsrc())
                .field("octo1_sw_psrc", &self.octo1_sw_psrc())
                .field("octo2_sw_nsrc", &self.octo2_sw_nsrc())
                .field("octo2_sw_psrc", &self.octo2_sw_psrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccswvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccswvalr {{ sw_nsrc: {=u8:?}, sw_psrc: {=u8:?}, octo1_sw_nsrc: {=u8:?}, octo1_sw_psrc: {=u8:?}, octo2_sw_nsrc: {=u8:?}, octo2_sw_psrc: {=u8:?} }}" , self . sw_nsrc () , self . sw_psrc () , self . octo1_sw_nsrc () , self . octo1_sw_psrc () , self . octo2_sw_nsrc () , self . octo2_sw_psrc ())
        }
    }
    #[doc = "SBS compensation cell for I/Os value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvalr(pub u32);
    impl Ccvalr {
        #[doc = "NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn nsrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn psrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "XSPIM_P1 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo1_nsrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P1 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo1_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "XSPIM_P1 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo1_psrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P1 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo1_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "XSPIM_P2 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo2_nsrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P2 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo2_nsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "XSPIM_P2 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub const fn octo2_psrc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "XSPIM_P2 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register."]
        #[inline(always)]
        pub fn set_octo2_psrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Ccvalr {
        #[inline(always)]
        fn default() -> Ccvalr {
            Ccvalr(0)
        }
    }
    impl core::fmt::Debug for Ccvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvalr")
                .field("nsrc", &self.nsrc())
                .field("psrc", &self.psrc())
                .field("octo1_nsrc", &self.octo1_nsrc())
                .field("octo1_psrc", &self.octo1_psrc())
                .field("octo2_nsrc", &self.octo2_nsrc())
                .field("octo2_psrc", &self.octo2_psrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccvalr {{ nsrc: {=u8:?}, psrc: {=u8:?}, octo1_nsrc: {=u8:?}, octo1_psrc: {=u8:?}, octo2_nsrc: {=u8:?}, octo2_psrc: {=u8:?} }}" , self . nsrc () , self . psrc () , self . octo1_nsrc () , self . octo1_psrc () , self . octo2_nsrc () , self . octo2_psrc ())
        }
    }
    #[doc = "SBS debug control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcr(pub u32);
    impl Dbgcr {
        #[doc = "access port unlock Write 0xB4 to this bitfield to open the device access port."]
        #[inline(always)]
        pub const fn ap_unlock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "access port unlock Write 0xB4 to this bitfield to open the device access port."]
        #[inline(always)]
        pub fn set_ap_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "debug unlock Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
        #[inline(always)]
        pub const fn dbg_unlock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "debug unlock Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
        #[inline(always)]
        pub fn set_dbg_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "authenticated debug hide protection level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the authenticated debug always fails."]
        #[inline(always)]
        pub const fn dbg_auth_hdpl(&self) -> super::vals::DbgAuthHdpl {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::DbgAuthHdpl::from_bits(val as u8)
        }
        #[doc = "authenticated debug hide protection level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the authenticated debug always fails."]
        #[inline(always)]
        pub fn set_dbg_auth_hdpl(&mut self, val: super::vals::DbgAuthHdpl) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dbgcr {
        #[inline(always)]
        fn default() -> Dbgcr {
            Dbgcr(0)
        }
    }
    impl core::fmt::Debug for Dbgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcr")
                .field("ap_unlock", &self.ap_unlock())
                .field("dbg_unlock", &self.dbg_unlock())
                .field("dbg_auth_hdpl", &self.dbg_auth_hdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbgcr {{ ap_unlock: {=u8:?}, dbg_unlock: {=u8:?}, dbg_auth_hdpl: {:?} }}",
                self.ap_unlock(),
                self.dbg_unlock(),
                self.dbg_auth_hdpl()
            )
        }
    }
    #[doc = "SBS debug lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbglockr(pub u32);
    impl Dbglockr {
        #[doc = "debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. Other: Writes to SBS_DBGCR ignored Note: 0xC3 is the recommended value to lock the debug configuration using this bitfield."]
        #[inline(always)]
        pub const fn dbgcfg_lock(&self) -> super::vals::DbgcfgLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::DbgcfgLock::from_bits(val as u8)
        }
        #[doc = "debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. Other: Writes to SBS_DBGCR ignored Note: 0xC3 is the recommended value to lock the debug configuration using this bitfield."]
        #[inline(always)]
        pub fn set_dbgcfg_lock(&mut self, val: super::vals::DbgcfgLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dbglockr {
        #[inline(always)]
        fn default() -> Dbglockr {
            Dbglockr(0)
        }
    }
    impl core::fmt::Debug for Dbglockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbglockr")
                .field("dbgcfg_lock", &self.dbgcfg_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbglockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbglockr {{ dbgcfg_lock: {:?} }}", self.dbgcfg_lock())
        }
    }
    #[doc = "external interrupt configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration (x = 4 to 7)"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration (x = 4 to 7)"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "SBS FPU interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default."]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default."]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
            f.debug_struct("Fpuimr").field("fpu_ie", &self.fpu_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fpuimr {{ fpu_ie: {=u8:?} }}", self.fpu_ie())
        }
    }
    #[doc = "SBS hide protection control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplcr(pub u32);
    impl Hdplcr {
        #[doc = "increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4)."]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4)."]
        #[inline(always)]
        pub fn set_incr_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Hdplcr {
        #[inline(always)]
        fn default() -> Hdplcr {
            Hdplcr(0)
        }
    }
    impl core::fmt::Debug for Hdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplcr").field("incr_hdpl", &self.incr_hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplcr {{ incr_hdpl: {=u8:?} }}", self.incr_hdpl())
        }
    }
    #[doc = "SBS hide protection status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplsr(pub u32);
    impl Hdplsr {
        #[doc = "hide protection level This bitfield returns the current HDPL of the device. 0x6F and other codes: HDPL3, corresponding to non-boot application. Note: The device state (open/close) is defined in FLASH_NVSTATER register of the embedded Flash memory."]
        #[inline(always)]
        pub const fn hdpl(&self) -> super::vals::Hdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Hdpl::from_bits(val as u8)
        }
        #[doc = "hide protection level This bitfield returns the current HDPL of the device. 0x6F and other codes: HDPL3, corresponding to non-boot application. Note: The device state (open/close) is defined in FLASH_NVSTATER register of the embedded Flash memory."]
        #[inline(always)]
        pub fn set_hdpl(&mut self, val: super::vals::Hdpl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Hdplsr {
        #[inline(always)]
        fn default() -> Hdplsr {
            Hdplsr(0)
        }
    }
    impl core::fmt::Debug for Hdplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplsr").field("hdpl", &self.hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplsr {{ hdpl: {:?} }}", self.hdpl())
        }
    }
    #[doc = "SBS memory erase status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "memory erase flag This bit is set by hardware when BKPRAM and PKA SRAM erase is ongoing after a POWER ON reset or one tamper event (see Section 50: Tamper and backup registers (TAMP) for details). This bit is cleared when the erase is done."]
        #[inline(always)]
        pub const fn mef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "memory erase flag This bit is set by hardware when BKPRAM and PKA SRAM erase is ongoing after a POWER ON reset or one tamper event (see Section 50: Tamper and backup registers (TAMP) for details). This bit is cleared when the erase is done."]
        #[inline(always)]
        pub fn set_mef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Mesr {
        #[inline(always)]
        fn default() -> Mesr {
            Mesr(0)
        }
    }
    impl core::fmt::Debug for Mesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mesr").field("mef", &self.mef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mesr {{ mef: {=bool:?} }}", self.mef())
        }
    }
    #[doc = "SBS product mode and configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "Fast-mode Plus on PB(6)."]
        #[inline(always)]
        pub const fn fmplus_pb6(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus on PB(6)."]
        #[inline(always)]
        pub fn set_fmplus_pb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Fast-mode Plus on PB(7)."]
        #[inline(always)]
        pub const fn fmplus_pb7(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus on PB(7)."]
        #[inline(always)]
        pub fn set_fmplus_pb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Fast-mode Plus on PB(8)."]
        #[inline(always)]
        pub const fn fmplus_pb8(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus on PB(8)."]
        #[inline(always)]
        pub fn set_fmplus_pb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fast-mode Plus on PB(9)."]
        #[inline(always)]
        pub const fn fmplus_pb9(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus on PB(9)."]
        #[inline(always)]
        pub fn set_fmplus_pb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches."]
        #[inline(always)]
        pub const fn boostvddsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches."]
        #[inline(always)]
        pub fn set_boostvddsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Ethernet PHY interface selection."]
        #[inline(always)]
        pub const fn eth_sel_phy(&self) -> super::vals::EthSelPhy {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::EthSelPhy::from_bits(val as u8)
        }
        #[doc = "Ethernet PHY interface selection."]
        #[inline(always)]
        pub fn set_eth_sel_phy(&mut self, val: super::vals::EthSelPhy) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default."]
        #[inline(always)]
        pub const fn axiram_ws(&self) -> super::vals::AxiramWs {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::AxiramWs::from_bits(val as u8)
        }
        #[doc = "AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default."]
        #[inline(always)]
        pub fn set_axiram_ws(&mut self, val: super::vals::AxiramWs) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    impl core::fmt::Debug for Pmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmcr")
                .field("fmplus_pb6", &self.fmplus_pb6())
                .field("fmplus_pb7", &self.fmplus_pb7())
                .field("fmplus_pb8", &self.fmplus_pb8())
                .field("fmplus_pb9", &self.fmplus_pb9())
                .field("boosten", &self.boosten())
                .field("boostvddsel", &self.boostvddsel())
                .field("eth_sel_phy", &self.eth_sel_phy())
                .field("axiram_ws", &self.axiram_ws())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmcr {{ fmplus_pb6: {=bool:?}, fmplus_pb7: {=bool:?}, fmplus_pb8: {=bool:?}, fmplus_pb9: {=bool:?}, boosten: {=bool:?}, boostvddsel: {=bool:?}, eth_sel_phy: {:?}, axiram_ws: {:?} }}" , self . fmplus_pb6 () , self . fmplus_pb7 () , self . fmplus_pb8 () , self . fmplus_pb9 () , self . boosten () , self . boostvddsel () , self . eth_sel_phy () , self . axiram_ws ())
        }
    }
    #[doc = "SBS RSS command register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset."]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset."]
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AxiramWs {
        #[doc = "No wait state added when accessing any AXIRAM with ECC = 0."]
        WS0 = 0x0,
        #[doc = "One wait state added when accessing any AXIRAM with ECC = 0. In this case, Fmax = 500 MHz is not guaranteed. (TBC)."]
        WS1 = 0x01,
    }
    impl AxiramWs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AxiramWs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AxiramWs {
        #[inline(always)]
        fn from(val: u8) -> AxiramWs {
            AxiramWs::from_bits(val)
        }
    }
    impl From<AxiramWs> for u8 {
        #[inline(always)]
        fn from(val: AxiramWs) -> u8 {
            AxiramWs::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgAuthHdpl(u8);
    impl DbgAuthHdpl {
        #[doc = "HDPL1."]
        pub const HDPL1: Self = Self(0x51);
        #[doc = "HDPL3."]
        pub const HDPL3: Self = Self(0x6f);
        #[doc = "HDPL2."]
        pub const HDPL2: Self = Self(0x8a);
    }
    impl DbgAuthHdpl {
        pub const fn from_bits(val: u8) -> DbgAuthHdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for DbgAuthHdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("HDPL1"),
                0x6f => f.write_str("HDPL3"),
                0x8a => f.write_str("HDPL2"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "HDPL1"),
                0x6f => defmt::write!(f, "HDPL3"),
                0x8a => defmt::write!(f, "HDPL2"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for DbgAuthHdpl {
        #[inline(always)]
        fn from(val: u8) -> DbgAuthHdpl {
            DbgAuthHdpl::from_bits(val)
        }
    }
    impl From<DbgAuthHdpl> for u8 {
        #[inline(always)]
        fn from(val: DbgAuthHdpl) -> u8 {
            DbgAuthHdpl::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgcfgLock(u8);
    impl DbgcfgLock {
        #[doc = "Writes to SBS_DBGCR allowed (default)."]
        pub const UNLOCK: Self = Self(0xb4);
    }
    impl DbgcfgLock {
        pub const fn from_bits(val: u8) -> DbgcfgLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for DbgcfgLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("UNLOCK"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgcfgLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "UNLOCK"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for DbgcfgLock {
        #[inline(always)]
        fn from(val: u8) -> DbgcfgLock {
            DbgcfgLock::from_bits(val)
        }
    }
    impl From<DbgcfgLock> for u8 {
        #[inline(always)]
        fn from(val: DbgcfgLock) -> u8 {
            DbgcfgLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EthSelPhy {
        #[doc = "GMII or MII"]
        MII_GMII = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "RMII"]
        RMII = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl EthSelPhy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EthSelPhy {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EthSelPhy {
        #[inline(always)]
        fn from(val: u8) -> EthSelPhy {
            EthSelPhy::from_bits(val)
        }
    }
    impl From<EthSelPhy> for u8 {
        #[inline(always)]
        fn from(val: EthSelPhy) -> u8 {
            EthSelPhy::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdpl(u8);
    impl Hdpl {
        #[doc = "HDPL1."]
        pub const HDPL1: Self = Self(0x51);
        #[doc = "HDPL2."]
        pub const HDPL2: Self = Self(0x8a);
        #[doc = "HDPL0, corresponding to ST-RSS (default when device is close)."]
        pub const HDPL0: Self = Self(0xb4);
    }
    impl Hdpl {
        pub const fn from_bits(val: u8) -> Hdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Hdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("HDPL1"),
                0x8a => f.write_str("HDPL2"),
                0xb4 => f.write_str("HDPL0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "HDPL1"),
                0x8a => defmt::write!(f, "HDPL2"),
                0xb4 => defmt::write!(f, "HDPL0"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Hdpl {
        #[inline(always)]
        fn from(val: u8) -> Hdpl {
            Hdpl::from_bits(val)
        }
    }
    impl From<Hdpl> for u8 {
        #[inline(always)]
        fn from(val: Hdpl) -> u8 {
            Hdpl::to_bits(val)
        }
    }
}
