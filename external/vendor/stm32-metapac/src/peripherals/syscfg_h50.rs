#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration, boot and security"]
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
    #[doc = "SBS temporal isolation control register"]
    #[inline(always)]
    pub const fn hdplcr(self) -> crate::common::Reg<regs::Hdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SBS temporal isolation status register"]
    #[inline(always)]
    pub const fn hdplsr(self) -> crate::common::Reg<regs::Hdplsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SBS debug control register"]
    #[inline(always)]
    pub const fn dbgcr(self) -> crate::common::Reg<regs::Dbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SBS debug lock register"]
    #[inline(always)]
    pub const fn dbglockr(self) -> crate::common::Reg<regs::Dbglockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SBS product mode and configuration register"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SBS FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SBS memory erase status register"]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os control and status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os value register"]
    #[inline(always)]
    pub const fn ccvalr(self) -> crate::common::Reg<regs::Ccvalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os software code register"]
    #[inline(always)]
    pub const fn ccswcr(self) -> crate::common::Reg<regs::Ccswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "SBS Class B register"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "SBS CPU lock register"]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "SBS flift ECC NMI mask register"]
    #[inline(always)]
    pub const fn eccnmir(self) -> crate::common::Reg<regs::Eccnmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
}
pub mod regs {
    #[doc = "SBS compensation cell for I/Os control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
        #[inline(always)]
        pub const fn en(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
        #[inline(always)]
        pub fn set_en(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
        #[inline(always)]
        pub const fn cs(&self, n: usize) -> super::vals::Cs {
            assert!(n < 2usize);
            let offs = 1usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Cs::from_bits(val as u8)
        }
        #[doc = "code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
        #[inline(always)]
        pub fn set_cs(&mut self, n: usize, val: super::vals::Cs) {
            assert!(n < 2usize);
            let offs = 1usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
        #[inline(always)]
        pub const fn rdy(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
        #[inline(always)]
        pub fn set_rdy(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("en[0]", &self.en(0usize))
                .field("en[1]", &self.en(1usize))
                .field("cs[0]", &self.cs(0usize))
                .field("cs[1]", &self.cs(1usize))
                .field("rdy[0]", &self.rdy(0usize))
                .field("rdy[1]", &self.rdy(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccsr {{ en[0]: {=bool:?}, en[1]: {=bool:?}, cs[0]: {:?}, cs[1]: {:?}, rdy[0]: {=bool:?}, rdy[1]: {=bool:?} }}" , self . en (0usize) , self . en (1usize) , self . cs (0usize) , self . cs (1usize) , self . rdy (0usize) , self . rdy (1usize))
        }
    }
    #[doc = "SBS compensation cell for I/Os software code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccswcr(pub u32);
    impl Ccswcr {
        #[doc = "NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
        #[inline(always)]
        pub const fn sw_ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
        #[inline(always)]
        pub fn set_sw_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
        #[inline(always)]
        pub const fn sw_apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
        #[inline(always)]
        pub fn set_sw_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
        #[inline(always)]
        pub const fn sw_ansrc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
        #[inline(always)]
        pub fn set_sw_ansrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PMOS compensation code for the V<sub>DDIO</sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
        #[inline(always)]
        pub const fn sw_apsrc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code for the V<sub>DDIO</sub> power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
        #[inline(always)]
        pub fn set_sw_apsrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ccswcr {
        #[inline(always)]
        fn default() -> Ccswcr {
            Ccswcr(0)
        }
    }
    impl core::fmt::Debug for Ccswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccswcr")
                .field("sw_ansrc1", &self.sw_ansrc1())
                .field("sw_apsrc1", &self.sw_apsrc1())
                .field("sw_ansrc2", &self.sw_ansrc2())
                .field("sw_apsrc2", &self.sw_apsrc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccswcr {{ sw_ansrc1: {=u8:?}, sw_apsrc1: {=u8:?}, sw_ansrc2: {=u8:?}, sw_apsrc2: {=u8:?} }}",
                self.sw_ansrc1(),
                self.sw_apsrc1(),
                self.sw_ansrc2(),
                self.sw_apsrc2()
            )
        }
    }
    #[doc = "SBS compensation cell for I/Os value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvalr(pub u32);
    impl Ccvalr {
        #[doc = "compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub const fn ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub fn set_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub const fn apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub fn set_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub const fn ansrc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub fn set_ansrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub const fn apsrc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
        #[inline(always)]
        pub fn set_apsrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("ansrc1", &self.ansrc1())
                .field("apsrc1", &self.apsrc1())
                .field("ansrc2", &self.ansrc2())
                .field("apsrc2", &self.apsrc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccvalr {{ ansrc1: {=u8:?}, apsrc1: {=u8:?}, ansrc2: {=u8:?}, apsrc2: {=u8:?} }}",
                self.ansrc1(),
                self.apsrc1(),
                self.ansrc2(),
                self.apsrc2()
            )
        }
    }
    #[doc = "SBS Class B register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
        #[inline(always)]
        pub const fn sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
        #[inline(always)]
        pub fn set_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
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
                .field("sel", &self.sel())
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
                "Cfgr2 {{ cll: {=bool:?}, sel: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?} }}",
                self.cll(),
                self.sel(),
                self.pvdl(),
                self.eccl()
            )
        }
    }
    #[doc = "SBS CPU lock register"]
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
        #[doc = "MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
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
    #[doc = "SBS debug control register"]
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
        #[doc = "debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
        #[inline(always)]
        pub const fn dbg_unlock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
        #[inline(always)]
        pub fn set_dbg_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
        #[inline(always)]
        pub const fn dbg_auth_hdpl(&self) -> super::vals::DbgAuthHdpl {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::DbgAuthHdpl::from_bits(val as u8)
        }
        #[doc = "authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
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
    #[doc = "SBS debug lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbglockr(pub u32);
    impl Dbglockr {
        #[doc = "debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
        #[inline(always)]
        pub const fn dbgcfg_lock(&self) -> super::vals::DbgcfgLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::DbgcfgLock::from_bits(val as u8)
        }
        #[doc = "debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
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
    #[doc = "SBS flift ECC NMI mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccnmir(pub u32);
    impl Eccnmir {
        #[doc = "NMI behavior setup when a double ECC error occurs on flitf data part"]
        #[inline(always)]
        pub const fn eccnmi_mask_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NMI behavior setup when a double ECC error occurs on flitf data part"]
        #[inline(always)]
        pub fn set_eccnmi_mask_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Eccnmir {
        #[inline(always)]
        fn default() -> Eccnmir {
            Eccnmir(0)
        }
    }
    impl core::fmt::Debug for Eccnmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccnmir")
                .field("eccnmi_mask_en", &self.eccnmi_mask_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccnmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccnmir {{ eccnmi_mask_en: {=bool:?} }}", self.eccnmi_mask_en())
        }
    }
    #[doc = "SBS FPU interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
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
    #[doc = "SBS temporal isolation control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplcr(pub u32);
    impl Hdplcr {
        #[doc = "increment HDPL value Other: all other values allow a HDPL level increment."]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> super::vals::IncrHdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::IncrHdpl::from_bits(val as u8)
        }
        #[doc = "increment HDPL value Other: all other values allow a HDPL level increment."]
        #[inline(always)]
        pub fn set_incr_hdpl(&mut self, val: super::vals::IncrHdpl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
            defmt::write!(f, "Hdplcr {{ incr_hdpl: {:?} }}", self.incr_hdpl())
        }
    }
    #[doc = "SBS temporal isolation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplsr(pub u32);
    impl Hdplsr {
        #[doc = "temporal isolation level This bitfield returns the current temporal isolation level."]
        #[inline(always)]
        pub const fn hdpl(&self) -> super::vals::Hdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Hdpl::from_bits(val as u8)
        }
        #[doc = "temporal isolation level This bitfield returns the current temporal isolation level."]
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
    #[doc = "SBS memory erase status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
        #[inline(always)]
        pub fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
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
    #[doc = "SBS product mode and configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "booster V<sub>DD</sub> selection Note: Booster must not be used when V<sub>DDA</sub> < 2.7 V, but V<sub>DD</sub> > 2.7 V (add current consumption). Note: When both V<sub>DD</sub> < 2.7 V and V<sub>DDA</sub> < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
        #[inline(always)]
        pub const fn boostvddsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "booster V<sub>DD</sub> selection Note: Booster must not be used when V<sub>DDA</sub> < 2.7 V, but V<sub>DD</sub> > 2.7 V (add current consumption). Note: When both V<sub>DD</sub> < 2.7 V and V<sub>DDA</sub> < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
        #[inline(always)]
        pub fn set_boostvddsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fast-mode Plus command on PB(6)"]
        #[inline(always)]
        pub const fn pb6_fmplus(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(6)"]
        #[inline(always)]
        pub fn set_pb6_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus command on PB(7)"]
        #[inline(always)]
        pub const fn pb7_fmplus(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(7)"]
        #[inline(always)]
        pub fn set_pb7_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus command on PB(8)"]
        #[inline(always)]
        pub const fn pb8_fmplus(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(8)"]
        #[inline(always)]
        pub fn set_pb8_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("boosten", &self.boosten())
                .field("boostvddsel", &self.boostvddsel())
                .field("pb6_fmplus", &self.pb6_fmplus())
                .field("pb7_fmplus", &self.pb7_fmplus())
                .field("pb8_fmplus", &self.pb8_fmplus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmcr {{ boosten: {=bool:?}, boostvddsel: {=bool:?}, pb6_fmplus: {=bool:?}, pb7_fmplus: {=bool:?}, pb8_fmplus: {=bool:?} }}" , self . boosten () , self . boostvddsel () , self . pb6_fmplus () , self . pb7_fmplus () , self . pb8_fmplus ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cs {
        #[doc = "Code from the cell (available in SBS_CCVR)"]
        CELL = 0x0,
        #[doc = "Code from SBS_CCCR"]
        SOFTWARE = 0x01,
    }
    impl Cs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cs {
        #[inline(always)]
        fn from(val: u8) -> Cs {
            Cs::from_bits(val)
        }
    }
    impl From<Cs> for u8 {
        #[inline(always)]
        fn from(val: Cs) -> u8 {
            Cs::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgAuthHdpl(u8);
    impl DbgAuthHdpl {
        #[doc = "HDPL1"]
        pub const B_0X51: Self = Self(0x51);
        #[doc = "HDPL3"]
        pub const B_0X6F: Self = Self(0x6f);
        #[doc = "HDPL2"]
        pub const B_0X8A: Self = Self(0x8a);
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
                0x51 => f.write_str("B_0X51"),
                0x6f => f.write_str("B_0X6F"),
                0x8a => f.write_str("B_0X8A"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "B_0X51"),
                0x6f => defmt::write!(f, "B_0X6F"),
                0x8a => defmt::write!(f, "B_0X8A"),
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
        #[doc = "Writes to SBS_DBGCR allowed (default)"]
        pub const B_0X_B4: Self = Self(0xb4);
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
                0xb4 => f.write_str("B_0X_B4"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgcfgLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B_0X_B4"),
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdpl(u8);
    impl Hdpl {
        #[doc = "HDPL1, iRoT"]
        pub const B_0X51: Self = Self(0x51);
        #[doc = "HDPL3, application"]
        pub const B_0X6F: Self = Self(0x6f);
        #[doc = "HDPL2, uRoT"]
        pub const B_0X8A: Self = Self(0x8a);
        #[doc = "HDPL0, RSS"]
        pub const B_0X_B4: Self = Self(0xb4);
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
                0x51 => f.write_str("B_0X51"),
                0x6f => f.write_str("B_0X6F"),
                0x8a => f.write_str("B_0X8A"),
                0xb4 => f.write_str("B_0X_B4"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "B_0X51"),
                0x6f => defmt::write!(f, "B_0X6F"),
                0x8a => defmt::write!(f, "B_0X8A"),
                0xb4 => defmt::write!(f, "B_0X_B4"),
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct IncrHdpl(u8);
    impl IncrHdpl {
        #[doc = "recommended value to increment HDPL level by one"]
        pub const B_0X6A: Self = Self(0x6a);
        #[doc = "no increment"]
        pub const B_0X_B4: Self = Self(0xb4);
    }
    impl IncrHdpl {
        pub const fn from_bits(val: u8) -> IncrHdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for IncrHdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x6a => f.write_str("B_0X6A"),
                0xb4 => f.write_str("B_0X_B4"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IncrHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x6a => defmt::write!(f, "B_0X6A"),
                0xb4 => defmt::write!(f, "B_0X_B4"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for IncrHdpl {
        #[inline(always)]
        fn from(val: u8) -> IncrHdpl {
            IncrHdpl::from_bits(val)
        }
    }
    impl From<IncrHdpl> for u8 {
        #[inline(always)]
        fn from(val: IncrHdpl) -> u8 {
            IncrHdpl::to_bits(val)
        }
    }
}
