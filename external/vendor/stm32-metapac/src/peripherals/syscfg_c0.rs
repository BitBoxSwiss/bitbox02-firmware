#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "register block"]
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
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "interrupt line 0 status register"]
    #[inline(always)]
    pub const fn itline0(self) -> crate::common::Reg<regs::Itline0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "interrupt line 2 status register"]
    #[inline(always)]
    pub const fn itline2(self) -> crate::common::Reg<regs::Itline2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "interrupt line 3 status register"]
    #[inline(always)]
    pub const fn itline3(self) -> crate::common::Reg<regs::Itline3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "interrupt line 4 status register"]
    #[inline(always)]
    pub const fn itline4(self) -> crate::common::Reg<regs::Itline4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "interrupt line 5 status register"]
    #[inline(always)]
    pub const fn itline5(self) -> crate::common::Reg<regs::Itline5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "interrupt line 6 status register"]
    #[inline(always)]
    pub const fn itline6(self) -> crate::common::Reg<regs::Itline6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "interrupt line 7 status register"]
    #[inline(always)]
    pub const fn itline7(self) -> crate::common::Reg<regs::Itline7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "interrupt line 9 status register"]
    #[inline(always)]
    pub const fn itline9(self) -> crate::common::Reg<regs::Itline9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "interrupt line 10 status register"]
    #[inline(always)]
    pub const fn itline10(self) -> crate::common::Reg<regs::Itline10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "interrupt line 11 status register"]
    #[inline(always)]
    pub const fn itline11(self) -> crate::common::Reg<regs::Itline11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "interrupt line 12 status register"]
    #[inline(always)]
    pub const fn itline12(self) -> crate::common::Reg<regs::Itline12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "interrupt line 13 status register"]
    #[inline(always)]
    pub const fn itline13(self) -> crate::common::Reg<regs::Itline13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "interrupt line 14 status register"]
    #[inline(always)]
    pub const fn itline14(self) -> crate::common::Reg<regs::Itline14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "interrupt line 16 status register"]
    #[inline(always)]
    pub const fn itline16(self) -> crate::common::Reg<regs::Itline16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "interrupt line 19 status register"]
    #[inline(always)]
    pub const fn itline19(self) -> crate::common::Reg<regs::Itline19, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "interrupt line 21 status register"]
    #[inline(always)]
    pub const fn itline21(self) -> crate::common::Reg<regs::Itline21, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "interrupt line 22 status register"]
    #[inline(always)]
    pub const fn itline22(self) -> crate::common::Reg<regs::Itline22, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "interrupt line 23 status register"]
    #[inline(always)]
    pub const fn itline23(self) -> crate::common::Reg<regs::Itline23, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "interrupt line 25 status register"]
    #[inline(always)]
    pub const fn itline25(self) -> crate::common::Reg<regs::Itline25, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "interrupt line 27 status register"]
    #[inline(always)]
    pub const fn itline27(self) -> crate::common::Reg<regs::Itline27, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "interrupt line 28 status register"]
    #[inline(always)]
    pub const fn itline28(self) -> crate::common::Reg<regs::Itline28, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details."]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details."]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
        #[inline(always)]
        pub const fn pa11_rmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
        #[inline(always)]
        pub fn set_pa11_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
        #[inline(always)]
        pub const fn pa12_rmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
        #[inline(always)]
        pub fn set_pa12_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub const fn ir_pol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub fn set_ir_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
        #[inline(always)]
        pub const fn ir_mod(&self) -> super::vals::IrMod {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::IrMod::from_bits(val as u8)
        }
        #[doc = "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: super::vals::IrMod) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers. With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pa9_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pa9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pa10_fmp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pa10_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub const fn i2c_pc14_fmp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port. With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored."]
        #[inline(always)]
        pub fn set_i2c_pc14_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("mem_mode", &self.mem_mode())
                .field("pa11_rmp", &self.pa11_rmp())
                .field("pa12_rmp", &self.pa12_rmp())
                .field("ir_pol", &self.ir_pol())
                .field("ir_mod", &self.ir_mod())
                .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
                .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
                .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
                .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
                .field("i2c1_fmp", &self.i2c1_fmp())
                .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
                .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
                .field("i2c_pc14_fmp", &self.i2c_pc14_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ mem_mode: {:?}, pa11_rmp: {=bool:?}, pa12_rmp: {=bool:?}, ir_pol: {=bool:?}, ir_mod: {:?}, i2c_pb6_fmp: {=bool:?}, i2c_pb7_fmp: {=bool:?}, i2c_pb8_fmp: {=bool:?}, i2c_pb9_fmp: {=bool:?}, i2c1_fmp: {=bool:?}, i2c_pa9_fmp: {=bool:?}, i2c_pa10_fmp: {=bool:?}, i2c_pc14_fmp: {=bool:?} }}" , self . mem_mode () , self . pa11_rmp () , self . pa12_rmp () , self . ir_pol () , self . ir_mod () , self . i2c_pb6_fmp () , self . i2c_pb7_fmp () , self . i2c_pb8_fmp () , self . i2c_pb9_fmp () , self . i2c1_fmp () , self . i2c_pa9_fmp () , self . i2c_pa10_fmp () , self . i2c_pc14_fmp ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
        #[inline(always)]
        pub const fn lockup_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
        #[inline(always)]
        pub fn set_lockup_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
                .field("lockup_lock", &self.lockup_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgr2 {{ lockup_lock: {=bool:?} }}", self.lockup_lock())
        }
    }
    #[doc = "configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux0(&self) -> super::vals::Pinmux0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pinmux0::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux0(&mut self, val: super::vals::Pinmux0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux1(&self) -> super::vals::Pinmux1 {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pinmux1::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux1(&mut self, val: super::vals::Pinmux1) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux2(&self) -> super::vals::Pinmux2 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Pinmux2::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux2(&mut self, val: super::vals::Pinmux2) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux3(&self) -> super::vals::Pinmux3 {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Pinmux3::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux3(&mut self, val: super::vals::Pinmux3) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux4(&self) -> super::vals::Pinmux4 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Pinmux4::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux4(&mut self, val: super::vals::Pinmux4) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub const fn pinmux5(&self) -> super::vals::Pinmux5 {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Pinmux5::from_bits(val as u8)
        }
        #[doc = "Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
        #[inline(always)]
        pub fn set_pinmux5(&mut self, val: super::vals::Pinmux5) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    impl core::fmt::Debug for Cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr3")
                .field("pinmux0", &self.pinmux0())
                .field("pinmux1", &self.pinmux1())
                .field("pinmux2", &self.pinmux2())
                .field("pinmux3", &self.pinmux3())
                .field("pinmux4", &self.pinmux4())
                .field("pinmux5", &self.pinmux5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr3 {{ pinmux0: {:?}, pinmux1: {:?}, pinmux2: {:?}, pinmux3: {:?}, pinmux4: {:?}, pinmux5: {:?} }}",
                self.pinmux0(),
                self.pinmux1(),
                self.pinmux2(),
                self.pinmux3(),
                self.pinmux4(),
                self.pinmux5()
            )
        }
    }
    #[doc = "interrupt line 0 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline0(pub u32);
    impl Itline0 {
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline0 {
        #[inline(always)]
        fn default() -> Itline0 {
            Itline0(0)
        }
    }
    impl core::fmt::Debug for Itline0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline0").field("wwdg", &self.wwdg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline0 {{ wwdg: {=bool:?} }}", self.wwdg())
        }
    }
    #[doc = "interrupt line 10 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline10(pub u32);
    impl Itline10 {
        #[doc = "DMA1 channel 2 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1 channel 3 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline10 {
        #[inline(always)]
        fn default() -> Itline10 {
            Itline10(0)
        }
    }
    impl core::fmt::Debug for Itline10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline10")
                .field("dma1_ch2", &self.dma1_ch2())
                .field("dma1_ch3", &self.dma1_ch3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline10 {{ dma1_ch2: {=bool:?}, dma1_ch3: {=bool:?} }}",
                self.dma1_ch2(),
                self.dma1_ch3()
            )
        }
    }
    #[doc = "interrupt line 11 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline11(pub u32);
    impl Itline11 {
        #[doc = "DMAMUX interrupt request pending"]
        #[inline(always)]
        pub const fn dmamux(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX interrupt request pending"]
        #[inline(always)]
        pub fn set_dmamux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline11 {
        #[inline(always)]
        fn default() -> Itline11 {
            Itline11(0)
        }
    }
    impl core::fmt::Debug for Itline11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline11").field("dmamux", &self.dmamux()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline11 {{ dmamux: {=bool:?} }}", self.dmamux())
        }
    }
    #[doc = "interrupt line 12 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline12(pub u32);
    impl Itline12 {
        #[doc = "ADC interrupt request pending"]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interrupt request pending"]
        #[inline(always)]
        pub fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline12 {
        #[inline(always)]
        fn default() -> Itline12 {
            Itline12(0)
        }
    }
    impl core::fmt::Debug for Itline12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline12").field("adc", &self.adc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline12 {{ adc: {=bool:?} }}", self.adc())
        }
    }
    #[doc = "interrupt line 13 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline13(pub u32);
    impl Itline13 {
        #[doc = "Timer 1 commutation interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_ccu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 commutation interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_ccu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 1 trigger interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_trg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 trigger interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_trg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 1 update interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_upd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 update interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_upd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 1 break interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_brk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 break interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_brk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Itline13 {
        #[inline(always)]
        fn default() -> Itline13 {
            Itline13(0)
        }
    }
    impl core::fmt::Debug for Itline13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline13")
                .field("tim1_ccu", &self.tim1_ccu())
                .field("tim1_trg", &self.tim1_trg())
                .field("tim1_upd", &self.tim1_upd())
                .field("tim1_brk", &self.tim1_brk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline13 {{ tim1_ccu: {=bool:?}, tim1_trg: {=bool:?}, tim1_upd: {=bool:?}, tim1_brk: {=bool:?} }}",
                self.tim1_ccu(),
                self.tim1_trg(),
                self.tim1_upd(),
                self.tim1_brk()
            )
        }
    }
    #[doc = "interrupt line 14 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline14(pub u32);
    impl Itline14 {
        #[doc = "Timer 1 capture compare interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_cc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 capture compare interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_cc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline14 {
        #[inline(always)]
        fn default() -> Itline14 {
            Itline14(0)
        }
    }
    impl core::fmt::Debug for Itline14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline14").field("tim1_cc", &self.tim1_cc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline14 {{ tim1_cc: {=bool:?} }}", self.tim1_cc())
        }
    }
    #[doc = "interrupt line 16 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline16(pub u32);
    impl Itline16 {
        #[doc = "Timer 3 interrupt request pending"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline16 {
        #[inline(always)]
        fn default() -> Itline16 {
            Itline16(0)
        }
    }
    impl core::fmt::Debug for Itline16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline16").field("tim3", &self.tim3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline16 {{ tim3: {=bool:?} }}", self.tim3())
        }
    }
    #[doc = "interrupt line 19 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline19(pub u32);
    impl Itline19 {
        #[doc = "Timer 14 interrupt request pending"]
        #[inline(always)]
        pub const fn tim14(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 14 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline19 {
        #[inline(always)]
        fn default() -> Itline19 {
            Itline19(0)
        }
    }
    impl core::fmt::Debug for Itline19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline19").field("tim14", &self.tim14()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline19 {{ tim14: {=bool:?} }}", self.tim14())
        }
    }
    #[doc = "interrupt line 2 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline2(pub u32);
    impl Itline2 {
        #[doc = "RTC interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTC interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline2 {
        #[inline(always)]
        fn default() -> Itline2 {
            Itline2(0)
        }
    }
    impl core::fmt::Debug for Itline2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline2").field("rtc", &self.rtc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline2 {{ rtc: {=bool:?} }}", self.rtc())
        }
    }
    #[doc = "interrupt line 21 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline21(pub u32);
    impl Itline21 {
        #[doc = "Timer 16 interrupt request pending"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 16 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline21 {
        #[inline(always)]
        fn default() -> Itline21 {
            Itline21(0)
        }
    }
    impl core::fmt::Debug for Itline21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline21").field("tim16", &self.tim16()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline21 {{ tim16: {=bool:?} }}", self.tim16())
        }
    }
    #[doc = "interrupt line 22 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline22(pub u32);
    impl Itline22 {
        #[doc = "Timer 17 interrupt request pending"]
        #[inline(always)]
        pub const fn tim17(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 17 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline22 {
        #[inline(always)]
        fn default() -> Itline22 {
            Itline22(0)
        }
    }
    impl core::fmt::Debug for Itline22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline22").field("tim17", &self.tim17()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline22 {{ tim17: {=bool:?} }}", self.tim17())
        }
    }
    #[doc = "interrupt line 23 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline23(pub u32);
    impl Itline23 {
        #[doc = "I2C1 interrupt request pending, combined with EXTI line 23"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 interrupt request pending, combined with EXTI line 23"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline23 {
        #[inline(always)]
        fn default() -> Itline23 {
            Itline23(0)
        }
    }
    impl core::fmt::Debug for Itline23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline23").field("i2c1", &self.i2c1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline23 {{ i2c1: {=bool:?} }}", self.i2c1())
        }
    }
    #[doc = "interrupt line 25 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline25(pub u32);
    impl Itline25 {
        #[doc = "SPI1 interrupt request pending"]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 interrupt request pending"]
        #[inline(always)]
        pub fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline25 {
        #[inline(always)]
        fn default() -> Itline25 {
            Itline25(0)
        }
    }
    impl core::fmt::Debug for Itline25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline25").field("spi1", &self.spi1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline25 {{ spi1: {=bool:?} }}", self.spi1())
        }
    }
    #[doc = "interrupt line 27 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline27(pub u32);
    impl Itline27 {
        #[doc = "USART1 interrupt request pending, combined with EXTI line 25"]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 interrupt request pending, combined with EXTI line 25"]
        #[inline(always)]
        pub fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline27 {
        #[inline(always)]
        fn default() -> Itline27 {
            Itline27(0)
        }
    }
    impl core::fmt::Debug for Itline27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline27").field("usart1", &self.usart1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline27 {{ usart1: {=bool:?} }}", self.usart1())
        }
    }
    #[doc = "interrupt line 28 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline28(pub u32);
    impl Itline28 {
        #[doc = "USART2 interrupt request pending (EXTI line 26)"]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 interrupt request pending (EXTI line 26)"]
        #[inline(always)]
        pub fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline28 {
        #[inline(always)]
        fn default() -> Itline28 {
            Itline28(0)
        }
    }
    impl core::fmt::Debug for Itline28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline28").field("usart2", &self.usart2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline28 {{ usart2: {=bool:?} }}", self.usart2())
        }
    }
    #[doc = "interrupt line 3 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline3(pub u32);
    impl Itline3 {
        #[doc = "Flash interface interrupt request pending"]
        #[inline(always)]
        pub const fn flash_itf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface interrupt request pending"]
        #[inline(always)]
        pub fn set_flash_itf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline3 {
        #[inline(always)]
        fn default() -> Itline3 {
            Itline3(0)
        }
    }
    impl core::fmt::Debug for Itline3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline3").field("flash_itf", &self.flash_itf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline3 {{ flash_itf: {=bool:?} }}", self.flash_itf())
        }
    }
    #[doc = "interrupt line 4 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline4(pub u32);
    impl Itline4 {
        #[doc = "Reset and clock control interrupt request pending"]
        #[inline(always)]
        pub const fn rcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reset and clock control interrupt request pending"]
        #[inline(always)]
        pub fn set_rcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline4 {
        #[inline(always)]
        fn default() -> Itline4 {
            Itline4(0)
        }
    }
    impl core::fmt::Debug for Itline4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline4").field("rcc", &self.rcc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline4 {{ rcc: {=bool:?} }}", self.rcc())
        }
    }
    #[doc = "interrupt line 5 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline5(pub u32);
    impl Itline5 {
        #[doc = "EXTI"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Itline5 {
        #[inline(always)]
        fn default() -> Itline5 {
            Itline5(0)
        }
    }
    impl core::fmt::Debug for Itline5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline5")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline5 {{ exti[0]: {=bool:?}, exti[1]: {=bool:?} }}",
                self.exti(0usize),
                self.exti(1usize)
            )
        }
    }
    #[doc = "interrupt line 6 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline6(pub u32);
    impl Itline6 {
        #[doc = "EXTI"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Itline6 {
        #[inline(always)]
        fn default() -> Itline6 {
            Itline6(0)
        }
    }
    impl core::fmt::Debug for Itline6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline6")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Itline6 {{ exti[0]: {=bool:?}, exti[1]: {=bool:?} }}",
                self.exti(0usize),
                self.exti(1usize)
            )
        }
    }
    #[doc = "interrupt line 7 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline7(pub u32);
    impl Itline7 {
        #[doc = "EXTI"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> bool {
            assert!(n < 12usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: bool) {
            assert!(n < 12usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Itline7 {
        #[inline(always)]
        fn default() -> Itline7 {
            Itline7(0)
        }
    }
    impl core::fmt::Debug for Itline7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline7")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .field("exti[4]", &self.exti(4usize))
                .field("exti[5]", &self.exti(5usize))
                .field("exti[6]", &self.exti(6usize))
                .field("exti[7]", &self.exti(7usize))
                .field("exti[8]", &self.exti(8usize))
                .field("exti[9]", &self.exti(9usize))
                .field("exti[10]", &self.exti(10usize))
                .field("exti[11]", &self.exti(11usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline7 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Itline7 {{ exti[0]: {=bool:?}, exti[1]: {=bool:?}, exti[2]: {=bool:?}, exti[3]: {=bool:?}, exti[4]: {=bool:?}, exti[5]: {=bool:?}, exti[6]: {=bool:?}, exti[7]: {=bool:?}, exti[8]: {=bool:?}, exti[9]: {=bool:?}, exti[10]: {=bool:?}, exti[11]: {=bool:?} }}" , self . exti (0usize) , self . exti (1usize) , self . exti (2usize) , self . exti (3usize) , self . exti (4usize) , self . exti (5usize) , self . exti (6usize) , self . exti (7usize) , self . exti (8usize) , self . exti (9usize) , self . exti (10usize) , self . exti (11usize))
        }
    }
    #[doc = "interrupt line 9 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline9(pub u32);
    impl Itline9 {
        #[doc = "DMA1 channel 1interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 1interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline9 {
        #[inline(always)]
        fn default() -> Itline9 {
            Itline9(0)
        }
    }
    impl core::fmt::Debug for Itline9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itline9").field("dma1_ch1", &self.dma1_ch1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itline9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itline9 {{ dma1_ch1: {=bool:?} }}", self.dma1_ch1())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMod {
        #[doc = "TIM16"]
        TIM16 = 0x0,
        #[doc = "USART1"]
        USART1 = 0x01,
        #[doc = "USART2"]
        USART2 = 0x02,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemMode {
        #[doc = "Main Flash memory mapped at address 0"]
        MAIN_FLASH = 0x0,
        #[doc = "System Flash memory mapped at address 0"]
        SYSTEM_FLASH = 0x01,
        #[doc = "Main Flash memory mapped at address 0 (alternate encoding)"]
        MAIN_FLASH_ALT = 0x02,
        #[doc = "Embedded SRAM mapped at address 0"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux0 {
        #[doc = "PB7"]
        PB7 = 0x0,
        #[doc = "PC14"]
        PC14 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pinmux0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux0 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux0 {
            Pinmux0::from_bits(val)
        }
    }
    impl From<Pinmux0> for u8 {
        #[inline(always)]
        fn from(val: Pinmux0) -> u8 {
            Pinmux0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux1 {
        #[doc = "PF2"]
        PF2 = 0x0,
        #[doc = "PA0"]
        PA0 = 0x01,
        #[doc = "PA1"]
        PA1 = 0x02,
        #[doc = "PA2"]
        PA2 = 0x03,
    }
    impl Pinmux1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux1 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux1 {
            Pinmux1::from_bits(val)
        }
    }
    impl From<Pinmux1> for u8 {
        #[inline(always)]
        fn from(val: Pinmux1) -> u8 {
            Pinmux1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux2 {
        #[doc = "PA8"]
        PA8 = 0x0,
        #[doc = "PA11"]
        PA11 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pinmux2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux2 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux2 {
            Pinmux2::from_bits(val)
        }
    }
    impl From<Pinmux2> for u8 {
        #[inline(always)]
        fn from(val: Pinmux2) -> u8 {
            Pinmux2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux3 {
        #[doc = "PA14"]
        PA14 = 0x0,
        #[doc = "PB6"]
        PB6 = 0x01,
        #[doc = "PC15"]
        PC15 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pinmux3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux3 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux3 {
            Pinmux3::from_bits(val)
        }
    }
    impl From<Pinmux3> for u8 {
        #[inline(always)]
        fn from(val: Pinmux3) -> u8 {
            Pinmux3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux4 {
        #[doc = "PA7"]
        PA7 = 0x0,
        #[doc = "PA12"]
        PA12 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pinmux4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux4 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux4 {
            Pinmux4::from_bits(val)
        }
    }
    impl From<Pinmux4> for u8 {
        #[inline(always)]
        fn from(val: Pinmux4) -> u8 {
            Pinmux4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinmux5 {
        #[doc = "PA3"]
        PA3 = 0x0,
        #[doc = "PA4"]
        PA4 = 0x01,
        #[doc = "PA5"]
        PA5 = 0x02,
        #[doc = "PA6"]
        PA6 = 0x03,
    }
    impl Pinmux5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinmux5 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinmux5 {
        #[inline(always)]
        fn from(val: u8) -> Pinmux5 {
            Pinmux5::from_bits(val)
        }
    }
    impl From<Pinmux5> for u8 {
        #[inline(always)]
        fn from(val: Pinmux5) -> u8 {
            Pinmux5::to_bits(val)
        }
    }
}
