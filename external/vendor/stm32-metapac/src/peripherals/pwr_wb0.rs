#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

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
    #[doc = "CR1 register."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CR2 register."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CR3 register."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CR4 register."]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SR1 register."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SR2 register."]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CR5 register."]
    #[inline(always)]
    pub const fn cr5(self) -> crate::common::Reg<regs::Cr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PUCRA register."]
    #[inline(always)]
    pub const fn pucra(self) -> crate::common::Reg<regs::Pucra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PDCRA register."]
    #[inline(always)]
    pub const fn pdcra(self) -> crate::common::Reg<regs::Pdcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PUCRB register."]
    #[inline(always)]
    pub const fn pucrb(self) -> crate::common::Reg<regs::Pucrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PDCRB register."]
    #[inline(always)]
    pub const fn pdcrb(self) -> crate::common::Reg<regs::Pdcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "CR6 register."]
    #[inline(always)]
    pub const fn cr6(self) -> crate::common::Reg<regs::Cr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "CR7 register."]
    #[inline(always)]
    pub const fn cr7(self) -> crate::common::Reg<regs::Cr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SR3 register."]
    #[inline(always)]
    pub const fn sr3(self) -> crate::common::Reg<regs::Sr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "IOxCFG register."]
    #[inline(always)]
    pub const fn iox_cfg(self) -> crate::common::Reg<regs::IoxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DBGR register."]
    #[inline(always)]
    pub const fn dbgr(self) -> crate::common::Reg<regs::Dbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "EXTSRR register."]
    #[inline(always)]
    pub const fn extsrr(self) -> crate::common::Reg<regs::Extsrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
}
pub mod regs {
    #[doc = "CR1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep."]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep."]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "ENSDNBOR: Enable BOR supply monitoring during shutdown mode."]
        #[inline(always)]
        pub const fn ensdnbor(&self) -> super::vals::Ensdnbor {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ensdnbor::from_bits(val as u8)
        }
        #[doc = "ENSDNBOR: Enable BOR supply monitoring during shutdown mode."]
        #[inline(always)]
        pub fn set_ensdnbor(&mut self, val: super::vals::Ensdnbor) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default)."]
        #[inline(always)]
        pub const fn ibias_run_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default)."]
        #[inline(always)]
        pub fn set_ibias_run_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled."]
        #[inline(always)]
        pub const fn ibias_run_state(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled."]
        #[inline(always)]
        pub fn set_ibias_run_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "APC Apply Pull-up and pull-down configuration from CPU."]
        #[inline(always)]
        pub const fn apc(&self) -> super::vals::Apc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Apc::from_bits(val as u8)
        }
        #[doc = "APC Apply Pull-up and pull-down configuration from CPU."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: super::vals::Apc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "ENBORH: enable BORH configuration."]
        #[inline(always)]
        pub const fn enborh(&self) -> super::vals::Enborh {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Enborh::from_bits(val as u8)
        }
        #[doc = "ENBORH: enable BORH configuration."]
        #[inline(always)]
        pub fn set_enborh(&mut self, val: super::vals::Enborh) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "SELBORH\\[1:0\\]: BORH selection of Vbor threshold."]
        #[inline(always)]
        pub const fn selborh(&self) -> super::vals::Selborh {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Selborh::from_bits(val as u8)
        }
        #[doc = "SELBORH\\[1:0\\]: BORH selection of Vbor threshold."]
        #[inline(always)]
        pub fn set_selborh(&mut self, val: super::vals::Selborh) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
        #[inline(always)]
        pub const fn enborl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
        #[inline(always)]
        pub fn set_enborl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("ensdnbor", &self.ensdnbor())
                .field("ibias_run_auto", &self.ibias_run_auto())
                .field("ibias_run_state", &self.ibias_run_state())
                .field("apc", &self.apc())
                .field("enborh", &self.enborh())
                .field("selborh", &self.selborh())
                .field("enborl", &self.enborl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ lpms: {:?}, ensdnbor: {:?}, ibias_run_auto: {=bool:?}, ibias_run_state: {=bool:?}, apc: {:?}, enborh: {:?}, selborh: {:?}, enborl: {=bool:?} }}" , self . lpms () , self . ensdnbor () , self . ibias_run_auto () , self . ibias_run_state () , self . apc () , self . enborh () , self . selborh () , self . enborl ())
        }
    }
    #[doc = "CR2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled."]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled."]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PVDLS\\[2:0\\]
Programmable Voltage Detector Level selection then PVDO=1)."]
        #[inline(always)]
        pub const fn pvdls(&self) -> super::vals::Pvdls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pvdls::from_bits(val as u8)
        }
        #[doc = "PVDLS\\[2:0\\]
Programmable Voltage Detector Level selection then PVDO=1)."]
        #[inline(always)]
        pub fn set_pvdls(&mut self, val: super::vals::Pvdls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
        #[inline(always)]
        pub const fn dbgret(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
        #[inline(always)]
        pub fn set_dbgret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enables the RAM2 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub const fn ramret1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enables the RAM2 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub fn set_ramret1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enables the RAM2 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub const fn ramret2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enables the RAM2 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub fn set_ramret2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enables the RAM3 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub const fn ramret3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enables the RAM3 bank retention in DEEPSTOP mode."]
        #[inline(always)]
        pub fn set_ramret3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set."]
        #[inline(always)]
        pub const fn gpioret(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set."]
        #[inline(always)]
        pub fn set_gpioret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ENTS: Enable Temperature Sensor."]
        #[inline(always)]
        pub const fn ents(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ENTS: Enable Temperature Sensor."]
        #[inline(always)]
        pub fn set_ents(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LSI LPMU force enable."]
        #[inline(always)]
        pub const fn lsilpmufen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LSI LPMU force enable."]
        #[inline(always)]
        pub fn set_lsilpmufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("pvdls", &self.pvdls())
                .field("dbgret", &self.dbgret())
                .field("ramret1", &self.ramret1())
                .field("ramret2", &self.ramret2())
                .field("ramret3", &self.ramret3())
                .field("gpioret", &self.gpioret())
                .field("ents", &self.ents())
                .field("lsilpmufen", &self.lsilpmufen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ pvde: {=bool:?}, pvdls: {:?}, dbgret: {=bool:?}, ramret1: {=bool:?}, ramret2: {=bool:?}, ramret3: {=bool:?}, gpioret: {=bool:?}, ents: {=bool:?}, lsilpmufen: {=bool:?} }}" , self . pvde () , self . pvdls () , self . dbgret () , self . ramret1 () , self . ramret2 () , self . ramret3 () , self . gpioret () , self . ents () , self . lsilpmufen ())
        }
    }
    #[doc = "CR3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit."]
        #[inline(always)]
        pub const fn ewu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EWU0 Enable WakeUp line 0 (PB0) When this bit is set the wakeup line 0 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR4.WP0 bit."]
        #[inline(always)]
        pub fn set_ewu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit."]
        #[inline(always)]
        pub const fn ewu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EWU1 Enable WakeUp line 1 (PB1) When this bit is set the wakeup line 1 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR4.WP1 bit."]
        #[inline(always)]
        pub fn set_ewu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit."]
        #[inline(always)]
        pub const fn ewu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EWU2 Enable WakeUp line 2 (PB2) When this bit is set the wakeup line 2 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR4.WP2 bit."]
        #[inline(always)]
        pub fn set_ewu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit."]
        #[inline(always)]
        pub const fn ewu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EWU3 Enable WakeUp line 3 (PB3) When this bit is set the wakeup line 3 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR4.WP3 bit."]
        #[inline(always)]
        pub fn set_ewu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit."]
        #[inline(always)]
        pub const fn ewu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EWU4 Enable WakeUp line 4 (PB4) When this bit is set the wakeup line 4 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR4.WP4 bit."]
        #[inline(always)]
        pub fn set_ewu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit."]
        #[inline(always)]
        pub const fn ewu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EWU5 Enable WakeUp line 5 (PB5) When this bit is set the wakeup line 5 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR4.WP5 bit."]
        #[inline(always)]
        pub fn set_ewu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit."]
        #[inline(always)]
        pub const fn ewu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EWU6 Enable WakeUp line 6 (PB6) When this bit is set the wakeup line 6 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR4.WP6 bit."]
        #[inline(always)]
        pub fn set_ewu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit."]
        #[inline(always)]
        pub const fn ewu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EWU7 Enable WakeUp line 7 (PB7) When this bit is set the wakeup line 7 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR4.WP7 bit."]
        #[inline(always)]
        pub fn set_ewu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit."]
        #[inline(always)]
        pub const fn ewu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EWU8 Enable WakeUp line 8 (PA8) When this bit is set the wakeup line 8 is enabled and a rising or falling edge on wakeup line 8 will trigger a CPU wakeup event depending on CR4.WP8 bit."]
        #[inline(always)]
        pub fn set_ewu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit."]
        #[inline(always)]
        pub const fn ewu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EWU9 Enable WakeUp line 9 (PA9) When this bit is set the wakeup line 9 is enabled and a rising or falling edge on wakeup line 9 will trigger a CPU wakeup event depending on CR4.WP9 bit."]
        #[inline(always)]
        pub fn set_ewu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit."]
        #[inline(always)]
        pub const fn ewu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "EWU10 Enable WakeUp line 10 (PA10) When this bit is set the wakeup line 10 is enabled and a rising or falling edge on wakeup line 10 will trigger a CPU wakeup event depending on CR4.WP10 bit."]
        #[inline(always)]
        pub fn set_ewu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit."]
        #[inline(always)]
        pub const fn ewu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EWU11 Enable WakeUp line 11 (PA11) When this bit is set the wakeup line 11 is enabled and a rising or falling edge on wakeup line 11 will trigger a CPU wakeup event depending on CR4.WP11 bit."]
        #[inline(always)]
        pub fn set_ewu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled."]
        #[inline(always)]
        pub const fn ewble(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "EWBLE: Enable wakeup on BLE event. 0: Wakeup on BLE line is disabled (default). 1: Wakeup on BLE line is enabled."]
        #[inline(always)]
        pub fn set_ewble(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled."]
        #[inline(always)]
        pub const fn ewblehcpu(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "EWBLEHCPU: Enable wakeup on BLE Host CPU event. 0: Wakeup on BLE Host CPU line is disabled (default). 1: Wakeup on BLE Host CPU line is enabled."]
        #[inline(always)]
        pub fn set_ewblehcpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled."]
        #[inline(always)]
        pub const fn eiwl2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EIWL2: Enable wakeup on Internal event (LPUART). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled."]
        #[inline(always)]
        pub fn set_eiwl2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled."]
        #[inline(always)]
        pub const fn eiwl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EIWL: Enable wakeup on Internal event (RTC). 0: Wakeup on internal line is disabled (default). 1: Wakeup on internal line is enabled."]
        #[inline(always)]
        pub fn set_eiwl(&mut self, val: bool) {
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
                .field("ewu0", &self.ewu0())
                .field("ewu1", &self.ewu1())
                .field("ewu2", &self.ewu2())
                .field("ewu3", &self.ewu3())
                .field("ewu4", &self.ewu4())
                .field("ewu5", &self.ewu5())
                .field("ewu6", &self.ewu6())
                .field("ewu7", &self.ewu7())
                .field("ewu8", &self.ewu8())
                .field("ewu9", &self.ewu9())
                .field("ewu10", &self.ewu10())
                .field("ewu11", &self.ewu11())
                .field("ewble", &self.ewble())
                .field("ewblehcpu", &self.ewblehcpu())
                .field("eiwl2", &self.eiwl2())
                .field("eiwl", &self.eiwl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ ewu0: {=bool:?}, ewu1: {=bool:?}, ewu2: {=bool:?}, ewu3: {=bool:?}, ewu4: {=bool:?}, ewu5: {=bool:?}, ewu6: {=bool:?}, ewu7: {=bool:?}, ewu8: {=bool:?}, ewu9: {=bool:?}, ewu10: {=bool:?}, ewu11: {=bool:?}, ewble: {=bool:?}, ewblehcpu: {=bool:?}, eiwl2: {=bool:?}, eiwl: {=bool:?} }}" , self . ewu0 () , self . ewu1 () , self . ewu2 () , self . ewu3 () , self . ewu4 () , self . ewu5 () , self . ewu6 () , self . ewu7 () , self . ewu8 () , self . ewu9 () , self . ewu10 () , self . ewu11 () , self . ewble () , self . ewblehcpu () , self . eiwl2 () , self . eiwl ())
        }
    }
    #[doc = "CR4 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0."]
        #[inline(always)]
        pub const fn wup0(&self) -> super::vals::Wup {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP0 Wake-up Line Polarity 0 (PB0) This bit defines the polarity used for event detection on external wake-up line 0."]
        #[inline(always)]
        pub fn set_wup0(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1."]
        #[inline(always)]
        pub const fn wup1(&self) -> super::vals::Wup {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP1 Wake-up Line Polarity 1 (PB1) This bit defines the polarity used for event detection on external wake-up line 1."]
        #[inline(always)]
        pub fn set_wup1(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2."]
        #[inline(always)]
        pub const fn wup2(&self) -> super::vals::Wup {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP2 Wake-up Line Polarity 2 (PB2) This bit defines the polarity used for event detection on external wake-up line 2."]
        #[inline(always)]
        pub fn set_wup2(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3."]
        #[inline(always)]
        pub const fn wup3(&self) -> super::vals::Wup {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP3 Wake-up Line Polarity 3 (PB3) This bit defines the polarity used for event detection on external wake-up line 3."]
        #[inline(always)]
        pub fn set_wup3(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4."]
        #[inline(always)]
        pub const fn wup4(&self) -> super::vals::Wup {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP4 Wake-up Line Polarity 4 (PB4) This bit defines the polarity used for event detection on external wake-up line 4."]
        #[inline(always)]
        pub fn set_wup4(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5."]
        #[inline(always)]
        pub const fn wup5(&self) -> super::vals::Wup {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP5 Wake-up Line Polarity 5 (PB5) This bit defines the polarity used for event detection on external wake-up line 5."]
        #[inline(always)]
        pub fn set_wup5(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6."]
        #[inline(always)]
        pub const fn wup6(&self) -> super::vals::Wup {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP6 Wake-up Line Polarity 6 (PB6) This bit defines the polarity used for event detection on external wake-up line 6."]
        #[inline(always)]
        pub fn set_wup6(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7."]
        #[inline(always)]
        pub const fn wup7(&self) -> super::vals::Wup {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP7 Wake-up Line Polarity 7 (PB7) This bit defines the polarity used for event detection on external wake-up line 7."]
        #[inline(always)]
        pub fn set_wup7(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8."]
        #[inline(always)]
        pub const fn wup8(&self) -> super::vals::Wup {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP8 Wake-up Line Polarity 8 (PA8) This bit defines the polarity used for event detection on external wake-up line 8."]
        #[inline(always)]
        pub fn set_wup8(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9."]
        #[inline(always)]
        pub const fn wup9(&self) -> super::vals::Wup {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP9 Wake-up Line Polarity 9 (PA9) This bit defines the polarity used for event detection on external wake-up line 9."]
        #[inline(always)]
        pub fn set_wup9(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10."]
        #[inline(always)]
        pub const fn wup10(&self) -> super::vals::Wup {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP10 Wake-up Line Polarity 10 (PA10) This bit defines the polarity used for event detection on external wake-up line 10."]
        #[inline(always)]
        pub fn set_wup10(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11."]
        #[inline(always)]
        pub const fn wup11(&self) -> super::vals::Wup {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP11 Wake-up Line Polarity 11 (PA11) This bit defines the polarity used for event detection on external wake-up line 11."]
        #[inline(always)]
        pub fn set_wup11(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
                .field("wup0", &self.wup0())
                .field("wup1", &self.wup1())
                .field("wup2", &self.wup2())
                .field("wup3", &self.wup3())
                .field("wup4", &self.wup4())
                .field("wup5", &self.wup5())
                .field("wup6", &self.wup6())
                .field("wup7", &self.wup7())
                .field("wup8", &self.wup8())
                .field("wup9", &self.wup9())
                .field("wup10", &self.wup10())
                .field("wup11", &self.wup11())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr4 {{ wup0: {:?}, wup1: {:?}, wup2: {:?}, wup3: {:?}, wup4: {:?}, wup5: {:?}, wup6: {:?}, wup7: {:?}, wup8: {:?}, wup9: {:?}, wup10: {:?}, wup11: {:?} }}" , self . wup0 () , self . wup1 () , self . wup2 () , self . wup3 () , self . wup4 () , self . wup5 () , self . wup6 () , self . wup7 () , self . wup8 () , self . wup9 () , self . wup10 () , self . wup11 ())
        }
    }
    #[doc = "CR5 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr5(pub u32);
    impl Cr5 {
        #[doc = "SMPSLVL\\[3:0\\]
SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)."]
        #[inline(always)]
        pub const fn smpslvl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SMPSLVL\\[3:0\\]
SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)."]
        #[inline(always)]
        pub fn set_smpslvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SMPSBOMSEL: SMPS BOM Selection:."]
        #[inline(always)]
        pub const fn smpsbomsel(&self) -> super::vals::Smpsbomsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Smpsbomsel::from_bits(val as u8)
        }
        #[doc = "SMPSBOMSEL: SMPS BOM Selection:."]
        #[inline(always)]
        pub fn set_smpsbomsel(&mut self, val: super::vals::Smpsbomsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready."]
        #[inline(always)]
        pub const fn smpsfrdy(&self) -> super::vals::Smpsfrdy {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Smpsfrdy::from_bits(val as u8)
        }
        #[doc = "SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready."]
        #[inline(always)]
        pub fn set_smpsfrdy(&mut self, val: super::vals::Smpsfrdy) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed."]
        #[inline(always)]
        pub const fn smpslpopen(&self) -> super::vals::Smpslpopen {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Smpslpopen::from_bits(val as u8)
        }
        #[doc = "SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed."]
        #[inline(always)]
        pub fn set_smpslpopen(&mut self, val: super::vals::Smpslpopen) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR."]
        #[inline(always)]
        pub const fn smpsfbyp(&self) -> super::vals::Smpsfbyp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Smpsfbyp::from_bits(val as u8)
        }
        #[doc = "SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR."]
        #[inline(always)]
        pub fn set_smpsfbyp(&mut self, val: super::vals::Smpsfbyp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM."]
        #[inline(always)]
        pub const fn nosmps(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM."]
        #[inline(always)]
        pub fn set_nosmps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SMPS_ENA_DCM: enable discontinuous conduction mode."]
        #[inline(always)]
        pub const fn smps_ena_dcm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS_ENA_DCM: enable discontinuous conduction mode."]
        #[inline(always)]
        pub fn set_smps_ena_dcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock."]
        #[inline(always)]
        pub const fn clkdetr_disable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock."]
        #[inline(always)]
        pub fn set_clkdetr_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SMPS_PRECH_CUR_SEL\\[1:0\\]
Selection for SMPS PRECHARGE limit current."]
        #[inline(always)]
        pub const fn smps_prech_cur_sel(&self) -> super::vals::SmpsPrechCurSel {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::SmpsPrechCurSel::from_bits(val as u8)
        }
        #[doc = "SMPS_PRECH_CUR_SEL\\[1:0\\]
Selection for SMPS PRECHARGE limit current."]
        #[inline(always)]
        pub fn set_smps_prech_cur_sel(&mut self, val: super::vals::SmpsPrechCurSel) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
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
                .field("smpslvl", &self.smpslvl())
                .field("smpsbomsel", &self.smpsbomsel())
                .field("smpsfrdy", &self.smpsfrdy())
                .field("smpslpopen", &self.smpslpopen())
                .field("smpsfbyp", &self.smpsfbyp())
                .field("nosmps", &self.nosmps())
                .field("smps_ena_dcm", &self.smps_ena_dcm())
                .field("clkdetr_disable", &self.clkdetr_disable())
                .field("smps_prech_cur_sel", &self.smps_prech_cur_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr5 {{ smpslvl: {=u8:?}, smpsbomsel: {:?}, smpsfrdy: {:?}, smpslpopen: {:?}, smpsfbyp: {:?}, nosmps: {=bool:?}, smps_ena_dcm: {=bool:?}, clkdetr_disable: {=bool:?}, smps_prech_cur_sel: {:?} }}" , self . smpslvl () , self . smpsbomsel () , self . smpsfrdy () , self . smpslpopen () , self . smpsfbyp () , self . nosmps () , self . smps_ena_dcm () , self . clkdetr_disable () , self . smps_prech_cur_sel ())
        }
    }
    #[doc = "CR6 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr6(pub u32);
    impl Cr6 {
        #[doc = "EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit."]
        #[inline(always)]
        pub const fn ewu12(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EWU12 Enable WakeUp line 12 (PA0) When this bit is set the wakeup line 12 is enabled and a rising or falling edge on wakeup line 0 will trigger a CPU wakeup event depending on CR7.WP0 bit."]
        #[inline(always)]
        pub fn set_ewu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit."]
        #[inline(always)]
        pub const fn ewu13(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EWU13 Enable WakeUp line 13 (PA1) When this bit is set the wakeup line 13 is enabled and a rising or falling edge on wakeup line 1 will trigger a CPU wakeup event depending on CR7.WP1 bit."]
        #[inline(always)]
        pub fn set_ewu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit."]
        #[inline(always)]
        pub const fn ewu14(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EWU14 Enable WakeUp line 14 (PA2) When this bit is set the wakeup line 14 is enabled and a rising or falling edge on wakeup line 2 will trigger a CPU wakeup event depending on CR7.WP2 bit."]
        #[inline(always)]
        pub fn set_ewu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit."]
        #[inline(always)]
        pub const fn ewu15(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EWU15 Enable WakeUp line 15 (PA3) When this bit is set the wakeup line 15 is enabled and a rising or falling edge on wakeup line 3 will trigger a CPU wakeup event depending on CR7.WP3 bit."]
        #[inline(always)]
        pub fn set_ewu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit."]
        #[inline(always)]
        pub const fn ewu16(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EWU16 Enable WakeUp line 16 (PB12) When this bit is set the wakeup line 16 is enabled and a rising or falling edge on wakeup line 4 will trigger a CPU wakeup event depending on CR7.WP4 bit."]
        #[inline(always)]
        pub fn set_ewu16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit."]
        #[inline(always)]
        pub const fn ewu17(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EWU17 Enable WakeUp line 17 (PB13) When this bit is set the wakeup line 17 is enabled and a rising or falling edge on wakeup line 5 will trigger a CPU wakeup event depending on CR7.WP5 bit."]
        #[inline(always)]
        pub fn set_ewu17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit."]
        #[inline(always)]
        pub const fn ewu18(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EWU18 Enable WakeUp line 18 (PB14) When this bit is set the wakeup line 18 is enabled and a rising or falling edge on wakeup line 6 will trigger a CPU wakeup event depending on CR7.WP6 bit."]
        #[inline(always)]
        pub fn set_ewu18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit."]
        #[inline(always)]
        pub const fn ewu19(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EWU19 Enable WakeUp line 19 (PB15) When this bit is set the wakeup line 19 is enabled and a rising or falling edge on wakeup line 7 will trigger a CPU wakeup event depending on CR7.WP7 bit."]
        #[inline(always)]
        pub fn set_ewu19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable wakeup on PB8 I/O event."]
        #[inline(always)]
        pub const fn ewu20(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PB8 I/O event."]
        #[inline(always)]
        pub fn set_ewu20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable wakeup on PB9 I/O event."]
        #[inline(always)]
        pub const fn ewu21(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PB9 I/O event."]
        #[inline(always)]
        pub fn set_ewu21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable wakeup on PB10 I/O event."]
        #[inline(always)]
        pub const fn ewu22(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PB10 I/O event."]
        #[inline(always)]
        pub fn set_ewu22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable wakeup on PB11 I/O event."]
        #[inline(always)]
        pub const fn ewu23(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PB11 I/O event."]
        #[inline(always)]
        pub fn set_ewu23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable wakeup on PA12 I/O event."]
        #[inline(always)]
        pub const fn ewu24(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PA12 I/O event."]
        #[inline(always)]
        pub fn set_ewu24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable wakeup on PA13 I/O event."]
        #[inline(always)]
        pub const fn ewu25(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PA13 I/O event."]
        #[inline(always)]
        pub fn set_ewu25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable wakeup on PA14 I/O event."]
        #[inline(always)]
        pub const fn ewu26(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PA14 I/O event."]
        #[inline(always)]
        pub fn set_ewu26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable wakeup on PA15 I/O event."]
        #[inline(always)]
        pub const fn ewu27(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup on PA15 I/O event."]
        #[inline(always)]
        pub fn set_ewu27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr6 {
        #[inline(always)]
        fn default() -> Cr6 {
            Cr6(0)
        }
    }
    impl core::fmt::Debug for Cr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr6")
                .field("ewu12", &self.ewu12())
                .field("ewu13", &self.ewu13())
                .field("ewu14", &self.ewu14())
                .field("ewu15", &self.ewu15())
                .field("ewu16", &self.ewu16())
                .field("ewu17", &self.ewu17())
                .field("ewu18", &self.ewu18())
                .field("ewu19", &self.ewu19())
                .field("ewu20", &self.ewu20())
                .field("ewu21", &self.ewu21())
                .field("ewu22", &self.ewu22())
                .field("ewu23", &self.ewu23())
                .field("ewu24", &self.ewu24())
                .field("ewu25", &self.ewu25())
                .field("ewu26", &self.ewu26())
                .field("ewu27", &self.ewu27())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr6 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr6 {{ ewu12: {=bool:?}, ewu13: {=bool:?}, ewu14: {=bool:?}, ewu15: {=bool:?}, ewu16: {=bool:?}, ewu17: {=bool:?}, ewu18: {=bool:?}, ewu19: {=bool:?}, ewu20: {=bool:?}, ewu21: {=bool:?}, ewu22: {=bool:?}, ewu23: {=bool:?}, ewu24: {=bool:?}, ewu25: {=bool:?}, ewu26: {=bool:?}, ewu27: {=bool:?} }}" , self . ewu12 () , self . ewu13 () , self . ewu14 () , self . ewu15 () , self . ewu16 () , self . ewu17 () , self . ewu18 () , self . ewu19 () , self . ewu20 () , self . ewu21 () , self . ewu22 () , self . ewu23 () , self . ewu24 () , self . ewu25 () , self . ewu26 () , self . ewu27 ())
        }
    }
    #[doc = "CR7 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr7(pub u32);
    impl Cr7 {
        #[doc = "WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12."]
        #[inline(always)]
        pub const fn wup12(&self) -> super::vals::Wup {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP12 Wake-up Line Polarity 12 (PA0) This bit defines the polarity used for event detection on external wake-up line 12."]
        #[inline(always)]
        pub fn set_wup12(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13."]
        #[inline(always)]
        pub const fn wup13(&self) -> super::vals::Wup {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP13 Wake-up Line Polarity 13 (PA1) This bit defines the polarity used for event detection on external wake-up line 13."]
        #[inline(always)]
        pub fn set_wup13(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14."]
        #[inline(always)]
        pub const fn wup14(&self) -> super::vals::Wup {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP14 Wake-up Line Polarity 14 (PA2) This bit defines the polarity used for event detection on external wake-up line 14."]
        #[inline(always)]
        pub fn set_wup14(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15."]
        #[inline(always)]
        pub const fn wup15(&self) -> super::vals::Wup {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP15 Wake-up Line Polarity 15 (PA3) This bit defines the polarity used for event detection on external wake-up line 15."]
        #[inline(always)]
        pub fn set_wup15(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16."]
        #[inline(always)]
        pub const fn wup16(&self) -> super::vals::Wup {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP16 Wake-up Line Polarity 16 (PB12) This bit defines the polarity used for event detection on external wake-up line 16."]
        #[inline(always)]
        pub fn set_wup16(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17."]
        #[inline(always)]
        pub const fn wup17(&self) -> super::vals::Wup {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP17 Wake-up Line Polarity 17 (PB13) This bit defines the polarity used for event detection on external wake-up line 17."]
        #[inline(always)]
        pub fn set_wup17(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18."]
        #[inline(always)]
        pub const fn wup18(&self) -> super::vals::Wup {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP18 Wake-up Line Polarity 18 (PB14) This bit defines the polarity used for event detection on external wake-up line 18."]
        #[inline(always)]
        pub fn set_wup18(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19."]
        #[inline(always)]
        pub const fn wup19(&self) -> super::vals::Wup {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Wup::from_bits(val as u8)
        }
        #[doc = "WUP19 Wake-up Line Polarity 19 (PB15) This bit defines the polarity used for event detection on external wake-up line 19."]
        #[inline(always)]
        pub fn set_wup19(&mut self, val: super::vals::Wup) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Wake-up polarity for PB8 IO event."]
        #[inline(always)]
        pub const fn wup20(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB8 IO event."]
        #[inline(always)]
        pub fn set_wup20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wake-up polarity for PB9 IO event."]
        #[inline(always)]
        pub const fn wup21(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB9 IO event."]
        #[inline(always)]
        pub fn set_wup21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wake-up polarity for PB10 IO event."]
        #[inline(always)]
        pub const fn wup22(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB10 IO event."]
        #[inline(always)]
        pub fn set_wup22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wake-up polarity for PB11 IO event."]
        #[inline(always)]
        pub const fn wup23(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB11 IO event."]
        #[inline(always)]
        pub fn set_wup23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Wake-up polarity for PB12 IO event."]
        #[inline(always)]
        pub const fn wup24(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB12 IO event."]
        #[inline(always)]
        pub fn set_wup24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wake-up polarity for PB13 IO event."]
        #[inline(always)]
        pub const fn wup25(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB13 IO event."]
        #[inline(always)]
        pub fn set_wup25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Wake-up polarity for PB14 IO event."]
        #[inline(always)]
        pub const fn wup26(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB14 IO event."]
        #[inline(always)]
        pub fn set_wup26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wake-up polarity for PB15 IO event."]
        #[inline(always)]
        pub const fn wup27(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up polarity for PB15 IO event."]
        #[inline(always)]
        pub fn set_wup27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr7 {
        #[inline(always)]
        fn default() -> Cr7 {
            Cr7(0)
        }
    }
    impl core::fmt::Debug for Cr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr7")
                .field("wup12", &self.wup12())
                .field("wup13", &self.wup13())
                .field("wup14", &self.wup14())
                .field("wup15", &self.wup15())
                .field("wup16", &self.wup16())
                .field("wup17", &self.wup17())
                .field("wup18", &self.wup18())
                .field("wup19", &self.wup19())
                .field("wup20", &self.wup20())
                .field("wup21", &self.wup21())
                .field("wup22", &self.wup22())
                .field("wup23", &self.wup23())
                .field("wup24", &self.wup24())
                .field("wup25", &self.wup25())
                .field("wup26", &self.wup26())
                .field("wup27", &self.wup27())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr7 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr7 {{ wup12: {:?}, wup13: {:?}, wup14: {:?}, wup15: {:?}, wup16: {:?}, wup17: {:?}, wup18: {:?}, wup19: {:?}, wup20: {=bool:?}, wup21: {=bool:?}, wup22: {=bool:?}, wup23: {=bool:?}, wup24: {=bool:?}, wup25: {=bool:?}, wup26: {=bool:?}, wup27: {=bool:?} }}" , self . wup12 () , self . wup13 () , self . wup14 () , self . wup15 () , self . wup16 () , self . wup17 () , self . wup18 () , self . wup19 () , self . wup20 () , self . wup21 () , self . wup22 () , self . wup23 () , self . wup24 () , self . wup25 () , self . wup26 () , self . wup27 ())
        }
    }
    #[doc = "DBGR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgr(pub u32);
    impl Dbgr {
        #[doc = "DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP."]
        #[inline(always)]
        pub const fn deepstop2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP."]
        #[inline(always)]
        pub fn set_deepstop2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) - else: No effect (default 0x0)."]
        #[inline(always)]
        pub const fn dis_prech(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) - else: No effect (default 0x0)."]
        #[inline(always)]
        pub fn set_dis_prech(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Dbgr {
        #[inline(always)]
        fn default() -> Dbgr {
            Dbgr(0)
        }
    }
    impl core::fmt::Debug for Dbgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgr")
                .field("deepstop2", &self.deepstop2())
                .field("dis_prech", &self.dis_prech())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbgr {{ deepstop2: {=bool:?}, dis_prech: {=u8:?} }}",
                self.deepstop2(),
                self.dis_prech()
            )
        }
    }
    #[doc = "EXTSRR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extsrr(pub u32);
    impl Extsrr {
        #[doc = "DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field."]
        #[inline(always)]
        pub const fn deepstopf(&self) -> super::vals::Deepstopf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Deepstopf::from_bits(val as u8)
        }
        #[doc = "DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field."]
        #[inline(always)]
        pub fn set_deepstopf(&mut self, val: super::vals::Deepstopf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP."]
        #[inline(always)]
        pub const fn rfphasef(&self) -> super::vals::Rfphasef {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rfphasef::from_bits(val as u8)
        }
        #[doc = "RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP."]
        #[inline(always)]
        pub fn set_rfphasef(&mut self, val: super::vals::Rfphasef) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Extsrr {
        #[inline(always)]
        fn default() -> Extsrr {
            Extsrr(0)
        }
    }
    impl core::fmt::Debug for Extsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extsrr")
                .field("deepstopf", &self.deepstopf())
                .field("rfphasef", &self.rfphasef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extsrr {{ deepstopf: {:?}, rfphasef: {:?} }}",
                self.deepstopf(),
                self.rfphasef()
            )
        }
    }
    #[doc = "IOxCFG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoxCfg(pub u32);
    impl IoxCfg {
        #[doc = "Drive configuration for PA8."]
        #[inline(always)]
        pub const fn iocfg0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA8."]
        #[inline(always)]
        pub fn set_iocfg0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Drive configuration for PA9."]
        #[inline(always)]
        pub const fn iocfg1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA9."]
        #[inline(always)]
        pub fn set_iocfg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Drive configuration for PA10."]
        #[inline(always)]
        pub const fn iocfg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA10."]
        #[inline(always)]
        pub fn set_iocfg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Drive configuration for PA11."]
        #[inline(always)]
        pub const fn iocfg3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA11."]
        #[inline(always)]
        pub fn set_iocfg3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Drive configuration for PA4."]
        #[inline(always)]
        pub const fn iocfg4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA4."]
        #[inline(always)]
        pub fn set_iocfg4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Drive configuration for PA5."]
        #[inline(always)]
        pub const fn iocfg5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA5."]
        #[inline(always)]
        pub fn set_iocfg5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Drive configuration for PA6."]
        #[inline(always)]
        pub const fn iocfg6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA6."]
        #[inline(always)]
        pub fn set_iocfg6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Drive configuration for PA7."]
        #[inline(always)]
        pub const fn iocfg7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Drive configuration for PA7."]
        #[inline(always)]
        pub fn set_iocfg7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for IoxCfg {
        #[inline(always)]
        fn default() -> IoxCfg {
            IoxCfg(0)
        }
    }
    impl core::fmt::Debug for IoxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoxCfg")
                .field("iocfg0", &self.iocfg0())
                .field("iocfg1", &self.iocfg1())
                .field("iocfg2", &self.iocfg2())
                .field("iocfg3", &self.iocfg3())
                .field("iocfg4", &self.iocfg4())
                .field("iocfg5", &self.iocfg5())
                .field("iocfg6", &self.iocfg6())
                .field("iocfg7", &self.iocfg7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoxCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoxCfg {{ iocfg0: {=u8:?}, iocfg1: {=u8:?}, iocfg2: {=u8:?}, iocfg3: {=u8:?}, iocfg4: {=u8:?}, iocfg5: {=u8:?}, iocfg6: {=u8:?}, iocfg7: {=u8:?} }}" , self . iocfg0 () , self . iocfg1 () , self . iocfg2 () , self . iocfg3 () , self . iocfg4 () , self . iocfg5 () , self . iocfg6 () , self . iocfg7 ())
        }
    }
    #[doc = "PDCRA register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcra(pub u32);
    impl Pdcra {
        #[doc = "PDA\\[x\\]: Pull Down Pull Down activation on port A\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub const fn pda(&self) -> super::vals::Pda {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pda::from_bits(val as u16)
        }
        #[doc = "PDA\\[x\\]: Pull Down Pull Down activation on port A\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub fn set_pda(&mut self, val: super::vals::Pda) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pdcra {
        #[inline(always)]
        fn default() -> Pdcra {
            Pdcra(0)
        }
    }
    impl core::fmt::Debug for Pdcra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcra").field("pda", &self.pda()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcra {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdcra {{ pda: {:?} }}", self.pda())
        }
    }
    #[doc = "PDCRB register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrb(pub u32);
    impl Pdcrb {
        #[doc = "PDB\\[x\\]: Pull Down Pull Down activation on port B\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub const fn pdb(&self) -> super::vals::Pdb {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pdb::from_bits(val as u16)
        }
        #[doc = "PDB\\[x\\]: Pull Down Pull Down activation on port B\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub fn set_pdb(&mut self, val: super::vals::Pdb) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pdcrb {
        #[inline(always)]
        fn default() -> Pdcrb {
            Pdcrb(0)
        }
    }
    impl core::fmt::Debug for Pdcrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdcrb").field("pdb", &self.pdb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdcrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdcrb {{ pdb: {:?} }}", self.pdb())
        }
    }
    #[doc = "PUCRA register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucra(pub u32);
    impl Pucra {
        #[doc = "PUA\\[x\\]
: Pull Up Pull up activation on port A\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub const fn pua(&self) -> super::vals::Pua {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pua::from_bits(val as u16)
        }
        #[doc = "PUA\\[x\\]
: Pull Up Pull up activation on port A\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub fn set_pua(&mut self, val: super::vals::Pua) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pucra {
        #[inline(always)]
        fn default() -> Pucra {
            Pucra(0)
        }
    }
    impl core::fmt::Debug for Pucra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucra").field("pua", &self.pua()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucra {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pucra {{ pua: {:?} }}", self.pua())
        }
    }
    #[doc = "PUCRB register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrb(pub u32);
    impl Pucrb {
        #[doc = "PUB\\[x\\]
: Pull Up Pull up activation on port B\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub const fn pub_(&self) -> super::vals::Pub {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pub::from_bits(val as u16)
        }
        #[doc = "PUB\\[x\\]
: Pull Up Pull up activation on port B\\[i\\]
pad when APC bit of PWRC CR3 is set."]
        #[inline(always)]
        pub fn set_pub_(&mut self, val: super::vals::Pub) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pucrb {
        #[inline(always)]
        fn default() -> Pucrb {
            Pucrb(0)
        }
    }
    impl core::fmt::Debug for Pucrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucrb").field("pub_", &self.pub_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pucrb {{ pub_: {:?} }}", self.pub_())
        }
    }
    #[doc = "SR1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf0(&self) -> super::vals::Wuf {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF0 WakeUp Flag 0 (PB0) This bit is set when a wakeup is detected on wakeup line 0. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf0(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf1(&self) -> super::vals::Wuf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF1 WakeUp Flag 1 (PB1) This bit is set when a wakeup is detected on wakeup line 1. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf1(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf2(&self) -> super::vals::Wuf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF2 WakeUp Flag 2 (PB2) This bit is set when a wakeup is detected on wakeup line 2. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf2(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf3(&self) -> super::vals::Wuf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF3 WakeUp Flag 3 (PB3) This bit is set when a wakeup is detected on wakeup line 3. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf3(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf4(&self) -> super::vals::Wuf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF4 WakeUp Flag 4 (PB4) This bit is set when a wakeup is detected on wakeup line 4. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf4(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf5(&self) -> super::vals::Wuf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF5 WakeUp Flag 5 (PB5) This bit is set when a wakeup is detected on wakeup line 5. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf5(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf6(&self) -> super::vals::Wuf {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF6 WakeUp Flag 6 (PB6) This bit is set when a wakeup is detected on wakeup line 6. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf6(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf7(&self) -> super::vals::Wuf {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF7 WakeUp Flag 7 (PB7) This bit is set when a wakeup is detected on wakeup line 7. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf7(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf8(&self) -> super::vals::Wuf {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF8 WakeUp Flag 8 (PA8) This bit is set when a wakeup is detected on wakeup line 8. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf8(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf9(&self) -> super::vals::Wuf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF9 WakeUp Flag 9 (PA9) This bit is set when a wakeup is detected on wakeup line 9. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf9(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf10(&self) -> super::vals::Wuf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF10 WakeUp Flag 10 (PA10) This bit is set when a wakeup is detected on wakeup line 10. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf10(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf11(&self) -> super::vals::Wuf {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF11 WakeUp Flag 11 (PA11) This bit is set when a wakeup is detected on wakeup line 11. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf11(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit."]
        #[inline(always)]
        pub const fn wblef(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "WBLEF: BLE wakeup flag. 0: no wakeup from BLE occurred since last clear. 1: a wakeup from BLE occurred since last clear. Cleared by writing 1 in this bit."]
        #[inline(always)]
        pub fn set_wblef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit."]
        #[inline(always)]
        pub const fn wblehcpuf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "WBLEHCPUF: BLE Host CPU wakeup flag. 0: no wakeup from BLE Host CPU occurred since last clear. 1: a wakeup from BLE Host CPU occurred since last clear. Cleared by writing 1 in this bit."]
        #[inline(always)]
        pub fn set_wblehcpuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "IWUF2: Internal wakeup 2 flag (LPUART). 0: no wakeup from LPUART occurred since last clear. 1: a wakeup from LPUART occurred since last clear. Note: The user must clear the LPUART wakeup flag inside the LPUART IP to clear this bit (mirror of the LPUART wakeup line on the PWRC block)."]
        #[inline(always)]
        pub const fn iwuf2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IWUF2: Internal wakeup 2 flag (LPUART). 0: no wakeup from LPUART occurred since last clear. 1: a wakeup from LPUART occurred since last clear. Note: The user must clear the LPUART wakeup flag inside the LPUART IP to clear this bit (mirror of the LPUART wakeup line on the PWRC block)."]
        #[inline(always)]
        pub fn set_iwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "IWUF: Internal wakeup flag (RTC). 0: no wakeup from RTC occurred since last clear. 1: a wakeup from RTC occurred since last clear. Note: The user must clear the RTC wakeup flag inside the RTC IP to clear this bit (mirror of the RTC wakeup line on the PWRC block)."]
        #[inline(always)]
        pub const fn iwuf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IWUF: Internal wakeup flag (RTC). 0: no wakeup from RTC occurred since last clear. 1: a wakeup from RTC occurred since last clear. Note: The user must clear the RTC wakeup flag inside the RTC IP to clear this bit (mirror of the RTC wakeup line on the PWRC block)."]
        #[inline(always)]
        pub fn set_iwuf(&mut self, val: bool) {
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
                .field("wuf0", &self.wuf0())
                .field("wuf1", &self.wuf1())
                .field("wuf2", &self.wuf2())
                .field("wuf3", &self.wuf3())
                .field("wuf4", &self.wuf4())
                .field("wuf5", &self.wuf5())
                .field("wuf6", &self.wuf6())
                .field("wuf7", &self.wuf7())
                .field("wuf8", &self.wuf8())
                .field("wuf9", &self.wuf9())
                .field("wuf10", &self.wuf10())
                .field("wuf11", &self.wuf11())
                .field("wblef", &self.wblef())
                .field("wblehcpuf", &self.wblehcpuf())
                .field("iwuf2", &self.iwuf2())
                .field("iwuf", &self.iwuf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ wuf0: {:?}, wuf1: {:?}, wuf2: {:?}, wuf3: {:?}, wuf4: {:?}, wuf5: {:?}, wuf6: {:?}, wuf7: {:?}, wuf8: {:?}, wuf9: {:?}, wuf10: {:?}, wuf11: {:?}, wblef: {=bool:?}, wblehcpuf: {=bool:?}, iwuf2: {=bool:?}, iwuf: {=bool:?} }}" , self . wuf0 () , self . wuf1 () , self . wuf2 () , self . wuf3 () , self . wuf4 () , self . wuf5 () , self . wuf6 () , self . wuf7 () , self . wuf8 () , self . wuf9 () , self . wuf10 () , self . wuf11 () , self . wblef () , self . wblehcpuf () , self . iwuf2 () , self . iwuf ())
        }
    }
    #[doc = "SR2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
        #[inline(always)]
        pub const fn smpsbypr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
        #[inline(always)]
        pub fn set_smpsbypr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
        #[inline(always)]
        pub const fn smpsenr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
        #[inline(always)]
        pub fn set_smpsenr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready."]
        #[inline(always)]
        pub const fn smpsrdy(&self) -> super::vals::Smpsrdy {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Smpsrdy::from_bits(val as u8)
        }
        #[doc = "SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready."]
        #[inline(always)]
        pub fn set_smpsrdy(&mut self, val: super::vals::Smpsrdy) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR."]
        #[inline(always)]
        pub const fn iobootval2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR."]
        #[inline(always)]
        pub fn set_iobootval2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready."]
        #[inline(always)]
        pub const fn reglps(&self) -> super::vals::Reglps {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Reglps::from_bits(val as u8)
        }
        #[doc = "REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready."]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: super::vals::Reglps) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "REGMS: Regulator Main LDO Started This bit provides the information whether main regulator is ready."]
        #[inline(always)]
        pub const fn regms(&self) -> super::vals::Regms {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Regms::from_bits(val as u8)
        }
        #[doc = "REGMS: Regulator Main LDO Started This bit provides the information whether main regulator is ready."]
        #[inline(always)]
        pub fn set_regms(&mut self, val: super::vals::Regms) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)."]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR."]
        #[inline(always)]
        pub const fn iobootval(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR."]
        #[inline(always)]
        pub fn set_iobootval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("smpsbypr", &self.smpsbypr())
                .field("smpsenr", &self.smpsenr())
                .field("smpsrdy", &self.smpsrdy())
                .field("iobootval2", &self.iobootval2())
                .field("reglps", &self.reglps())
                .field("regms", &self.regms())
                .field("pvdo", &self.pvdo())
                .field("iobootval", &self.iobootval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2 {{ smpsbypr: {=bool:?}, smpsenr: {=bool:?}, smpsrdy: {:?}, iobootval2: {=u8:?}, reglps: {:?}, regms: {:?}, pvdo: {=bool:?}, iobootval: {=u8:?} }}" , self . smpsbypr () , self . smpsenr () , self . smpsrdy () , self . iobootval2 () , self . reglps () , self . regms () , self . pvdo () , self . iobootval ())
        }
    }
    #[doc = "SR3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr3(pub u32);
    impl Sr3 {
        #[doc = "WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf12(&self) -> super::vals::Wuf {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF12 WakeUp Flag 12 PA0 This bit is set when a wakeup is detected on wakeup line 12. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf12(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf13(&self) -> super::vals::Wuf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF13 WakeUp Flag 13 PA1 This bit is set when a wakeup is detected on wakeup line 13. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf13(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf14(&self) -> super::vals::Wuf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF14 WakeUp Flag 14 PA2 This bit is set when a wakeup is detected on wakeup line 14. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf14(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf15(&self) -> super::vals::Wuf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF15 WakeUp Flag 15 PA3 This bit is set when a wakeup is detected on wakeup line 15. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf15(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf16(&self) -> super::vals::Wuf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF16 WakeUp Flag 16 PB12 This bit is set when a wakeup is detected on wakeup line 16. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf16(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf17(&self) -> super::vals::Wuf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF17 WakeUp Flag 17 PB13 This bit is set when a wakeup is detected on wakeup line 17. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf17(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf18(&self) -> super::vals::Wuf {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF18 WakeUp Flag 18 PB14 This bit is set when a wakeup is detected on wakeup line 18. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf18(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub const fn wuf19(&self) -> super::vals::Wuf {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "WUF19 WakeUp Flag 19 PB15 This bit is set when a wakeup is detected on wakeup line 19. It is cleared by a reset pad or by writing 1 in this bit field. writting this bit, clears the interrupt:."]
        #[inline(always)]
        pub fn set_wuf19(&mut self, val: super::vals::Wuf) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "PB8 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf20(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PB8 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PB9 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf21(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PB9 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PB10 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf22(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PB10 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PB11 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf23(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PB12 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf24(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PB12 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PB13 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf25(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PB13 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PB14 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf26(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PB14 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PB15 I/O wake-up flag."]
        #[inline(always)]
        pub const fn wuf27(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 I/O wake-up flag."]
        #[inline(always)]
        pub fn set_wuf27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr3 {
        #[inline(always)]
        fn default() -> Sr3 {
            Sr3(0)
        }
    }
    impl core::fmt::Debug for Sr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr3")
                .field("wuf12", &self.wuf12())
                .field("wuf13", &self.wuf13())
                .field("wuf14", &self.wuf14())
                .field("wuf15", &self.wuf15())
                .field("wuf16", &self.wuf16())
                .field("wuf17", &self.wuf17())
                .field("wuf18", &self.wuf18())
                .field("wuf19", &self.wuf19())
                .field("wuf20", &self.wuf20())
                .field("wuf21", &self.wuf21())
                .field("wuf22", &self.wuf22())
                .field("wuf23", &self.wuf23())
                .field("wuf24", &self.wuf24())
                .field("wuf25", &self.wuf25())
                .field("wuf26", &self.wuf26())
                .field("wuf27", &self.wuf27())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr3 {{ wuf12: {:?}, wuf13: {:?}, wuf14: {:?}, wuf15: {:?}, wuf16: {:?}, wuf17: {:?}, wuf18: {:?}, wuf19: {:?}, wuf20: {=bool:?}, wuf21: {=bool:?}, wuf22: {=bool:?}, wuf23: {=bool:?}, wuf24: {=bool:?}, wuf25: {=bool:?}, wuf26: {=bool:?}, wuf27: {=bool:?} }}" , self . wuf12 () , self . wuf13 () , self . wuf14 () , self . wuf15 () , self . wuf16 () , self . wuf17 () , self . wuf18 () , self . wuf19 () , self . wuf20 () , self . wuf21 () , self . wuf22 () , self . wuf23 () , self . wuf24 () , self . wuf25 () , self . wuf26 () , self . wuf27 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Apc {
        #[doc = "the PUCRx and PDCRx are not used to control the I/O pull-up and pull-down configuration of the product I/Os."]
        B_0X0 = 0x0,
        #[doc = "the I/O pull-up and pull-down configurations defined in the PUCRx and PDCRx registers is applied."]
        B_0X1 = 0x01,
    }
    impl Apc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Apc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Apc {
        #[inline(always)]
        fn from(val: u8) -> Apc {
            Apc::from_bits(val)
        }
    }
    impl From<Apc> for u8 {
        #[inline(always)]
        fn from(val: Apc) -> u8 {
            Apc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Deepstopf {
        #[doc = "System has not been in DEEPSTOP mode."]
        B_0X0 = 0x0,
        #[doc = "System has been in DEEPSTOP mode."]
        B_0X1 = 0x01,
    }
    impl Deepstopf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Deepstopf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Deepstopf {
        #[inline(always)]
        fn from(val: u8) -> Deepstopf {
            Deepstopf::from_bits(val)
        }
    }
    impl From<Deepstopf> for u8 {
        #[inline(always)]
        fn from(val: Deepstopf) -> u8 {
            Deepstopf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Enborh {
        #[doc = "BORH off (VBOR0): threshold level for above 1.60V voltage operation."]
        B_0X0 = 0x0,
        #[doc = "BORH is enabled, threshold level depends on SELBOR\\[1:0\\]."]
        B_0X1 = 0x01,
    }
    impl Enborh {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Enborh {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Enborh {
        #[inline(always)]
        fn from(val: u8) -> Enborh {
            Enborh::from_bits(val)
        }
    }
    impl From<Enborh> for u8 {
        #[inline(always)]
        fn from(val: Enborh) -> u8 {
            Enborh::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ensdnbor {
        #[doc = "the PD_ALL_SHUTDOWN signal is set during SHUTDOWN mode."]
        B_0X0 = 0x0,
        #[doc = "the PD_ALL_SHUTDOWN signal is not set during SHUTDOWN mode."]
        B_0X1 = 0x01,
    }
    impl Ensdnbor {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ensdnbor {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ensdnbor {
        #[inline(always)]
        fn from(val: u8) -> Ensdnbor {
            Ensdnbor::from_bits(val)
        }
    }
    impl From<Ensdnbor> for u8 {
        #[inline(always)]
        fn from(val: Ensdnbor) -> u8 {
            Ensdnbor::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Deep Stop mode (default)."]
        B_0X0 = 0x0,
        #[doc = "Shutdown mode."]
        B_0X1 = 0x01,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pda(u16);
    impl Pda {
        #[doc = "Pull-Down not activated on Port A\\[i\\]."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "Pull-Down activated on Port A\\[i\\]
when APC bit of PWRC CR3 bit is set."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Pda {
        pub const fn from_bits(val: u16) -> Pda {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pda {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pda {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pda {
        #[inline(always)]
        fn from(val: u16) -> Pda {
            Pda::from_bits(val)
        }
    }
    impl From<Pda> for u16 {
        #[inline(always)]
        fn from(val: Pda) -> u16 {
            Pda::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdb(u16);
    impl Pdb {
        #[doc = "Pull-Down not activated on Port B\\[i\\]."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "Pull-Down activated on Port B\\[i\\]
when APC bit of PWRC CR3 bit is set."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Pdb {
        pub const fn from_bits(val: u16) -> Pdb {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pdb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdb {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pdb {
        #[inline(always)]
        fn from(val: u16) -> Pdb {
            Pdb::from_bits(val)
        }
    }
    impl From<Pdb> for u16 {
        #[inline(always)]
        fn from(val: Pdb) -> u16 {
            Pdb::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pua(u16);
    impl Pua {
        #[doc = "Pull-Up not activated on port A\\[i\\]."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "Pull-Up activated on port A\\[i\\]
when APC bit of PWRC CR3 bit is set and PWR_PDCRA\\[x\\]
is reset."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Pua {
        pub const fn from_bits(val: u16) -> Pua {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pua {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pua {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pua {
        #[inline(always)]
        fn from(val: u16) -> Pua {
            Pua::from_bits(val)
        }
    }
    impl From<Pua> for u16 {
        #[inline(always)]
        fn from(val: Pua) -> u16 {
            Pua::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pub(u16);
    impl Pub {
        #[doc = "Pull-Up not activated on port B\\[i\\]."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "Pull-Up activated on port B\\[i\\]
when APC bit of PWRC CR3 bit is set and PWR_PDCRB\\[x\\]
is reset."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Pub {
        pub const fn from_bits(val: u16) -> Pub {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pub {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pub {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pub {
        #[inline(always)]
        fn from(val: u16) -> Pub {
            Pub::from_bits(val)
        }
    }
    impl From<Pub> for u16 {
        #[inline(always)]
        fn from(val: Pub) -> u16 {
            Pub::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdls {
        #[doc = "2.05 V - Lowest level."]
        B_0X0 = 0x0,
        #[doc = "2.20 V."]
        B_0X1 = 0x01,
        #[doc = "2.36 V."]
        B_0X2 = 0x02,
        #[doc = "2.52 V."]
        B_0X3 = 0x03,
        #[doc = "2.64 V."]
        B_0X4 = 0x04,
        #[doc = "2.81 V."]
        B_0X5 = 0x05,
        #[doc = "2.91 V - Highest level."]
        B_0X6 = 0x06,
        #[doc = "External input analog voltage (compare internally to VBGP; When external input <VBGP."]
        B_0X7 = 0x07,
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
    pub enum Reglps {
        #[doc = "LP regulator is not ready."]
        B_0X0 = 0x0,
        #[doc = "LP regulator is ready."]
        B_0X1 = 0x01,
    }
    impl Reglps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reglps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reglps {
        #[inline(always)]
        fn from(val: u8) -> Reglps {
            Reglps::from_bits(val)
        }
    }
    impl From<Reglps> for u8 {
        #[inline(always)]
        fn from(val: Reglps) -> u8 {
            Reglps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Regms {
        #[doc = "Main regulator is not ready."]
        B_0X0 = 0x0,
        #[doc = "Main regulator is ready."]
        B_0X1 = 0x01,
    }
    impl Regms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regms {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regms {
        #[inline(always)]
        fn from(val: u8) -> Regms {
            Regms::from_bits(val)
        }
    }
    impl From<Regms> for u8 {
        #[inline(always)]
        fn from(val: Regms) -> u8 {
            Regms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rfphasef {
        #[doc = "RF IP does not require attention."]
        B_0X0 = 0x0,
        #[doc = "RF IP awake and requesting system attention."]
        B_0X1 = 0x01,
    }
    impl Rfphasef {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfphasef {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfphasef {
        #[inline(always)]
        fn from(val: u8) -> Rfphasef {
            Rfphasef::from_bits(val)
        }
    }
    impl From<Rfphasef> for u8 {
        #[inline(always)]
        fn from(val: Rfphasef) -> u8 {
            Rfphasef::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Selborh {
        #[doc = "BORH Level 1 (VBOR1): threshold level for above 2.0V voltage operation."]
        B_0X0 = 0x0,
        #[doc = "BORH Level 2 (VBOR2): threshold level for above 2.21 V voltage operation."]
        B_0X1 = 0x01,
        #[doc = "BORH Level 3 (VBOR3): threshold level for above 2.52 V voltage operation."]
        B_0X2 = 0x02,
        #[doc = "BORH Level 4(VBOR4): threshold level for above 2.81 V voltage operation."]
        B_0X3 = 0x03,
    }
    impl Selborh {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Selborh {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Selborh {
        #[inline(always)]
        fn from(val: u8) -> Selborh {
            Selborh::from_bits(val)
        }
    }
    impl From<Selborh> for u8 {
        #[inline(always)]
        fn from(val: Selborh) -> u8 {
            Selborh::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SmpsPrechCurSel {
        #[doc = "2.5mA."]
        B_0X0 = 0x0,
        #[doc = "5mA."]
        B_0X1 = 0x01,
        #[doc = "10mA."]
        B_0X2 = 0x02,
        #[doc = "20mA (default)."]
        B_0X3 = 0x03,
    }
    impl SmpsPrechCurSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SmpsPrechCurSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SmpsPrechCurSel {
        #[inline(always)]
        fn from(val: u8) -> SmpsPrechCurSel {
            SmpsPrechCurSel::from_bits(val)
        }
    }
    impl From<SmpsPrechCurSel> for u8 {
        #[inline(always)]
        fn from(val: SmpsPrechCurSel) -> u8 {
            SmpsPrechCurSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsbomsel {
        #[doc = "BOM1."]
        B_0X0 = 0x0,
        #[doc = "BOM2 (default)."]
        B_0X1 = 0x01,
        #[doc = "BOM3."]
        B_0X2 = 0x02,
        #[doc = "n/a."]
        B_0X3 = 0x03,
    }
    impl Smpsbomsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsbomsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsbomsel {
        #[inline(always)]
        fn from(val: u8) -> Smpsbomsel {
            Smpsbomsel::from_bits(val)
        }
    }
    impl From<Smpsbomsel> for u8 {
        #[inline(always)]
        fn from(val: Smpsbomsel) -> u8 {
            Smpsbomsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsfbyp {
        #[doc = "no effect (by default)."]
        B_0X0 = 0x0,
        #[doc = "SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)."]
        B_0X1 = 0x01,
    }
    impl Smpsfbyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsfbyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsfbyp {
        #[inline(always)]
        fn from(val: u8) -> Smpsfbyp {
            Smpsfbyp::from_bits(val)
        }
    }
    impl From<Smpsfbyp> for u8 {
        #[inline(always)]
        fn from(val: Smpsfbyp) -> u8 {
            Smpsfbyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsfrdy {
        #[doc = "no effect (by default)."]
        B_0X0 = 0x0,
        #[doc = "SMPS is considered READY."]
        B_0X1 = 0x01,
    }
    impl Smpsfrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsfrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsfrdy {
        #[inline(always)]
        fn from(val: u8) -> Smpsfrdy {
            Smpsfrdy::from_bits(val)
        }
    }
    impl From<Smpsfrdy> for u8 {
        #[inline(always)]
        fn from(val: Smpsfrdy) -> u8 {
            Smpsfrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpslpopen {
        #[doc = "in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO."]
        B_0X0 = 0x0,
        #[doc = "in Low Power mode, SMPS is disabled, output is floating."]
        B_0X1 = 0x01,
    }
    impl Smpslpopen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpslpopen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpslpopen {
        #[inline(always)]
        fn from(val: u8) -> Smpslpopen {
            Smpslpopen::from_bits(val)
        }
    }
    impl From<Smpslpopen> for u8 {
        #[inline(always)]
        fn from(val: Smpslpopen) -> u8 {
            Smpslpopen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpsrdy {
        #[doc = "SMPS regulator is not ready."]
        B_0X0 = 0x0,
        #[doc = "SMPS regulator is ready."]
        B_0X1 = 0x01,
    }
    impl Smpsrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsrdy {
        #[inline(always)]
        fn from(val: u8) -> Smpsrdy {
            Smpsrdy::from_bits(val)
        }
    }
    impl From<Smpsrdy> for u8 {
        #[inline(always)]
        fn from(val: Smpsrdy) -> u8 {
            Smpsrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wuf {
        #[doc = "no effect."]
        B_0X0 = 0x0,
        #[doc = "clear the interrupt."]
        B_0X1 = 0x01,
    }
    impl Wuf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wuf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wuf {
        #[inline(always)]
        fn from(val: u8) -> Wuf {
            Wuf::from_bits(val)
        }
    }
    impl From<Wuf> for u8 {
        #[inline(always)]
        fn from(val: Wuf) -> u8 {
            Wuf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wup {
        #[doc = "Detection on high level (rising edge)."]
        B_0X0 = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        B_0X1 = 0x01,
    }
    impl Wup {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wup {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wup {
        #[inline(always)]
        fn from(val: u8) -> Wup {
            Wup::from_bits(val)
        }
    }
    impl From<Wup> for u8 {
        #[inline(always)]
        fn from(val: Wup) -> u8 {
            Wup::to_bits(val)
        }
    }
}
