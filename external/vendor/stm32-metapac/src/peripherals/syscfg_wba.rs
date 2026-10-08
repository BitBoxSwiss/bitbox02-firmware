#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
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
    #[doc = "secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CPU non-secure lock register"]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CPU secure lock register"]
    #[inline(always)]
    pub const fn cslockr(self) -> crate::common::Reg<regs::Cslockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "memory erase status register"]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RSS command register"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "OTG_HS PHY register"]
    #[inline(always)]
    pub const fn otghsphycr(self) -> crate::common::Reg<regs::Otghsphycr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "OTG_HS PHY tune register 2"]
    #[inline(always)]
    pub const fn otghsphytuner2(self) -> crate::common::Reg<regs::Otghsphytuner2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "compensation cell code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn ncc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub fn set_ncc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn pcc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub fn set_pcc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn ncc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set."]
        #[inline(always)]
        pub fn set_ncc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn pcc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set."]
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
    #[doc = "compensation cell control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub const fn cs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub fn set_cs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VDDIO2 I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>."]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VDDIO2 I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>."]
        #[inline(always)]
        pub const fn cs2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>."]
        #[inline(always)]
        pub fn set_cs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DD</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI clock is not enabled (HSION)."]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DD</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI clock is not enabled (HSION)."]
        #[inline(always)]
        pub fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VDDIO2 I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DDIO2</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY2) is not set if the HSI clock is not enabled (HSION)."]
        #[inline(always)]
        pub const fn rdy2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO2 I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DDIO2</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY2) is not set if the HSI clock is not enabled (HSION)."]
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
    #[doc = "compensation cell value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn ncv1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub fn set_ncv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn pcv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub fn set_pcv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn ncv2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset."]
        #[inline(always)]
        pub fn set_ncv2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn pcv2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset."]
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
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub const fn anaswvdd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub fn set_anaswvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6."]
        #[inline(always)]
        pub const fn pa6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6."]
        #[inline(always)]
        pub fn set_pa6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7."]
        #[inline(always)]
        pub const fn pa7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7."]
        #[inline(always)]
        pub fn set_pa7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15."]
        #[inline(always)]
        pub const fn pa15_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15."]
        #[inline(always)]
        pub fn set_pa15_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3."]
        #[inline(always)]
        pub const fn pb3_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3."]
        #[inline(always)]
        pub fn set_pb3_fmp(&mut self, val: bool) {
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
                .field("boosten", &self.boosten())
                .field("anaswvdd", &self.anaswvdd())
                .field("pa6_fmp", &self.pa6_fmp())
                .field("pa7_fmp", &self.pa7_fmp())
                .field("pa15_fmp", &self.pa15_fmp())
                .field("pb3_fmp", &self.pb3_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ boosten: {=bool:?}, anaswvdd: {=bool:?}, pa6_fmp: {=bool:?}, pa7_fmp: {=bool:?}, pa15_fmp: {=bool:?}, pb3_fmp: {=bool:?} }}" , self . boosten () , self . anaswvdd () , self . pa6_fmp () , self . pa7_fmp () , self . pa15_fmp () , self . pb3_fmp ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M33 LOCKUP (hardfault) output enable This bit is set by software and cleared only by a system reset. It can be used to enable and lock the connection of Cortex-M33 LOCKUP (hardfault) output to TIM1/16/17 break input."]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M33 LOCKUP (hardfault) output enable This bit is set by software and cleared only by a system reset. It can be used to enable and lock the connection of Cortex-M33 LOCKUP (hardfault) output to TIM1/16/17 break input."]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity lock bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/16/17 break inputs."]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity lock bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/16/17 break inputs."]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection to TIM1/16/17 break input, as well as the PVDE and PVDLS\\[2:0\\]
in the PWR register."]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection to TIM1/16/17 break input, as well as the PVDE and PVDLS\\[2:0\\]
in the PWR register."]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the Flash ECC double error signal connection to TIM1/16/17 break input."]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the Flash ECC double error signal connection to TIM1/16/17 break input."]
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
    #[doc = "CPU non-secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnslckr(pub u32);
    impl Cnslckr {
        #[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
        #[inline(always)]
        pub fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
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
    #[doc = "CPU secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cslockr(pub u32);
    impl Cslockr {
        #[doc = "VTOR_S register and AIRCR register bits lock This bit is set by software and cleared only by a system reset. When set, it disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_S register and AIRCR register bits lock This bit is set by software and cleared only by a system reset. When set, it disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
        #[inline(always)]
        pub fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
        #[inline(always)]
        pub fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SAU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SAU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
        #[inline(always)]
        pub fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cslockr {
        #[inline(always)]
        fn default() -> Cslockr {
            Cslockr(0)
        }
    }
    impl core::fmt::Debug for Cslockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cslockr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cslockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cslockr {{ locksvtaircr: {=bool:?}, locksmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locksmpu(),
                self.locksau()
            )
        }
    }
    #[doc = "FPU interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "Floating point unit interrupts enable bits FPU_IE\\[5\\]: Inexact interrupt enable (interrupt disable at reset) FPU_IE\\[4\\]: Input abnormal interrupt enable FPU_IE\\[3\\]: Overflow interrupt enable FPU_IE\\[2\\]: Underflow interrupt enable FPU_IE\\[1\\]: Divide-by-zero interrupt enable FPU_IE\\[0\\]: Invalid operation Interrupt enable"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Floating point unit interrupts enable bits FPU_IE\\[5\\]: Inexact interrupt enable (interrupt disable at reset) FPU_IE\\[4\\]: Input abnormal interrupt enable FPU_IE\\[3\\]: Overflow interrupt enable FPU_IE\\[2\\]: Underflow interrupt enable FPU_IE\\[1\\]: Divide-by-zero interrupt enable FPU_IE\\[0\\]: Invalid operation Interrupt enable"]
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
    #[doc = "memory erase status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "Device memories erase status This bit is set by hardware when SRAM2, ICACHE, PKA SRAM erase is completed after power-on reset or tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Device memories erase status This bit is set by hardware when SRAM2, ICACHE, PKA SRAM erase is completed after power-on reset or tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ICACHE and PKA SRAM erase status This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE and PKA SRAM erase status This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_ipmee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            f.debug_struct("Mesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mesr {{ mclr: {=bool:?}, ipmee: {=bool:?} }}",
                self.mclr(),
                self.ipmee()
            )
        }
    }
    #[doc = "OTG_HS PHY register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otghsphycr(pub u32);
    impl Otghsphycr {
        #[doc = "PHY Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Common block power-down control"]
        #[inline(always)]
        pub const fn pdctrl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Common block power-down control"]
        #[inline(always)]
        pub fn set_pdctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reference clock frequency selection"]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Usbrefcksel {
            let val = (self.0 >> 2usize) & 0x0f;
            super::vals::Usbrefcksel::from_bits(val as u8)
        }
        #[doc = "Reference clock frequency selection"]
        #[inline(always)]
        pub fn set_clksel(&mut self, val: super::vals::Usbrefcksel) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
        }
    }
    impl Default for Otghsphycr {
        #[inline(always)]
        fn default() -> Otghsphycr {
            Otghsphycr(0)
        }
    }
    impl core::fmt::Debug for Otghsphycr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otghsphycr")
                .field("en", &self.en())
                .field("pdctrl", &self.pdctrl())
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otghsphycr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otghsphycr {{ en: {=bool:?}, pdctrl: {=bool:?}, clksel: {:?} }}",
                self.en(),
                self.pdctrl(),
                self.clksel()
            )
        }
    }
    #[doc = "OTG_HS tune register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otghsphytuner2(pub u32);
    impl Otghsphytuner2 {
        #[doc = "Disconnect threshold adjustment"]
        #[inline(always)]
        pub const fn compdistune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Disconnect threshold adjustment"]
        #[inline(always)]
        pub fn set_compdistune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Squelch threshold adjustment"]
        #[inline(always)]
        pub const fn sqrxtune(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Squelch threshold adjustment"]
        #[inline(always)]
        pub fn set_sqrxtune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "HS transmitter preemphasis current control"]
        #[inline(always)]
        pub const fn txpreempamptune(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "HS transmitter preemphasis current control"]
        #[inline(always)]
        pub fn set_txpreempamptune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Otghsphytuner2 {
        #[inline(always)]
        fn default() -> Otghsphytuner2 {
            Otghsphytuner2(0)
        }
    }
    impl core::fmt::Debug for Otghsphytuner2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otghsphytuner2")
                .field("compdistune", &self.compdistune())
                .field("sqrxtune", &self.sqrxtune())
                .field("txpreempamptune", &self.txpreempamptune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otghsphytuner2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otghsphytuner2 {{ compdistune: {=u8:?}, sqrxtune: {=u8:?}, txpreempamptune: {=u8:?} }}",
                self.compdistune(),
                self.sqrxtune(),
                self.txpreempamptune()
            )
        }
    }
    #[doc = "RSS command register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS commands This field defines a command to be executed by the RSS."]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS commands This field defines a command to be executed by the RSS."]
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
    #[doc = "secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "clock control, memory erase status and compensation cell registers security"]
        #[inline(always)]
        pub const fn syscfgsec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clock control, memory erase status and compensation cell registers security"]
        #[inline(always)]
        pub fn set_syscfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Class B security"]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Class B security"]
        #[inline(always)]
        pub fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FPU security"]
        #[inline(always)]
        pub const fn fpusec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FPU security"]
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
    pub enum Usbrefcksel {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 16 MHz."]
        MHZ16 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 19.2 MHz."]
        MHZ19_2 = 0x08,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 20MHz."]
        MHZ20 = 0x09,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 24 MHz (default after reset)."]
        MHZ24 = 0x0a,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 32 MHz."]
        MHZ32 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        #[doc = "The kernel clock frequency provided to the OTG_HS PHY is 26 MHz."]
        MHZ26 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Usbrefcksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbrefcksel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbrefcksel {
        #[inline(always)]
        fn from(val: u8) -> Usbrefcksel {
            Usbrefcksel::from_bits(val)
        }
    }
    impl From<Usbrefcksel> for u8 {
        #[inline(always)]
        fn from(val: Usbrefcksel) -> u8 {
            Usbrefcksel::to_bits(val)
        }
    }
}
