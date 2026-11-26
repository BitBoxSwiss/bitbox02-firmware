#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RAMCFG address block description."]
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
    #[doc = "RAMCFG SRAM1 control register."]
    #[inline(always)]
    pub const fn m1cr(self) -> crate::common::Reg<regs::M1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RAMCFG SRAM1 interrupt status register."]
    #[inline(always)]
    pub const fn m1isr(self) -> crate::common::Reg<regs::M1isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RAMCFG SRAM1 erase key register."]
    #[inline(always)]
    pub const fn m1erkeyr(self) -> crate::common::Reg<regs::M1erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 control register."]
    #[inline(always)]
    pub const fn m2cr(self) -> crate::common::Reg<regs::M2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 interrupt enable register."]
    #[inline(always)]
    pub const fn m2ier(self) -> crate::common::Reg<regs::M2ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 interrupt status register."]
    #[inline(always)]
    pub const fn m2isr(self) -> crate::common::Reg<regs::M2isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 parity error address register."]
    #[inline(always)]
    pub const fn m2pear(self) -> crate::common::Reg<regs::M2pear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 interrupt clear register."]
    #[inline(always)]
    pub const fn m2icr(self) -> crate::common::Reg<regs::M2icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 write protection register 1."]
    #[inline(always)]
    pub const fn m2wpr1(self) -> crate::common::Reg<regs::M2wpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RAMCFG SRAM2 write protection register 2."]
    #[inline(always)]
    pub const fn m2wpr2(self) -> crate::common::Reg<regs::M2wpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "RAMCFG SRAM2 erase key register."]
    #[inline(always)]
    pub const fn m2erkeyr(self) -> crate::common::Reg<regs::M2erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
}
pub mod regs {
    #[doc = "RAMCFG SRAM1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1cr(pub u32);
    impl M1cr {
        #[doc = "SRAM1 erase."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM1 wait state configuration."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "SRAM1 wait state configuration."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M1cr {{ sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG SRAM1 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1erkeyr(pub u32);
    impl M1erkeyr {
        #[doc = "Erase write protection key."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key."]
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
    #[doc = "RAMCFG SRAM1 interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1isr(pub u32);
    impl M1isr {
        #[doc = "SRAM busy with erase operation."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation."]
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
            f.debug_struct("M1isr").field("srambusy", &self.srambusy()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M1isr {{ srambusy: {=bool:?} }}", self.srambusy())
        }
    }
    #[doc = "RAMCFG SRAM2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2cr(pub u32);
    impl M2cr {
        #[doc = "SRAM2 parity fail address latch enable."]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity fail address latch enable."]
        #[inline(always)]
        pub fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM2 erase."]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase."]
        #[inline(always)]
        pub fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 wait state configuration."]
        #[inline(always)]
        pub const fn wsc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "SRAM2 wait state configuration."]
        #[inline(always)]
        pub fn set_wsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .field("wsc", &self.wsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2cr {{ ale: {=bool:?}, sramer: {=bool:?}, wsc: {=u8:?} }}",
                self.ale(),
                self.sramer(),
                self.wsc()
            )
        }
    }
    #[doc = "RAMCFG SRAM2 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2erkeyr(pub u32);
    impl M2erkeyr {
        #[doc = "Erase write protection key."]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key."]
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
    #[doc = "RAMCFG SRAM2 interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2icr(pub u32);
    impl M2icr {
        #[doc = "Clear parity error detect bit."]
        #[inline(always)]
        pub const fn cped(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear parity error detect bit."]
        #[inline(always)]
        pub fn set_cped(&mut self, val: bool) {
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
            f.debug_struct("M2icr").field("cped", &self.cped()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2icr {{ cped: {=bool:?} }}", self.cped())
        }
    }
    #[doc = "RAMCFG SRAM2 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ier(pub u32);
    impl M2ier {
        #[doc = "Parity error interrupt enable."]
        #[inline(always)]
        pub const fn peie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error interrupt enable."]
        #[inline(always)]
        pub fn set_peie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error NMI."]
        #[inline(always)]
        pub const fn penmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error NMI."]
        #[inline(always)]
        pub fn set_penmi(&mut self, val: bool) {
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
                .field("peie", &self.peie())
                .field("penmi", &self.penmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2ier {{ peie: {=bool:?}, penmi: {=bool:?} }}",
                self.peie(),
                self.penmi()
            )
        }
    }
    #[doc = "RAMCFG SRAM2 interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2isr(pub u32);
    impl M2isr {
        #[doc = "Parity error detected."]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error detected."]
        #[inline(always)]
        pub fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 busy with erase operation."]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 busy with erase operation."]
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
                .field("ped", &self.ped())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2isr {{ ped: {=bool:?}, srambusy: {=bool:?} }}",
                self.ped(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG SRAM2 parity error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2pear(pub u32);
    impl M2pear {
        #[doc = "Parity error SRAM word aligned address offset."]
        #[inline(always)]
        pub const fn pea(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Parity error SRAM word aligned address offset."]
        #[inline(always)]
        pub fn set_pea(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Parity error AHB bus master ID."]
        #[inline(always)]
        pub const fn id(&self) -> super::vals::Id {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Id::from_bits(val as u8)
        }
        #[doc = "Parity error AHB bus master ID."]
        #[inline(always)]
        pub fn set_id(&mut self, val: super::vals::Id) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Byte parity error flag."]
        #[inline(always)]
        pub const fn byte(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Byte parity error flag."]
        #[inline(always)]
        pub fn set_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for M2pear {
        #[inline(always)]
        fn default() -> M2pear {
            M2pear(0)
        }
    }
    impl core::fmt::Debug for M2pear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2pear")
                .field("pea", &self.pea())
                .field("id", &self.id())
                .field("byte", &self.byte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2pear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2pear {{ pea: {=u16:?}, id: {:?}, byte: {=u8:?} }}",
                self.pea(),
                self.id(),
                self.byte()
            )
        }
    }
    #[doc = "RAMCFG SRAM2 write protection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr1(pub u32);
    impl M2wpr1 {
        #[doc = "SRAM2 1-Kbyte write protect page y write protection."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte write protect page y write protection."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "M2wpr1 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
    #[doc = "RAMCFG SRAM2 write protection register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr2(pub u32);
    impl M2wpr2 {
        #[doc = "SRAM2 1-Kbyte write protect page y write protection."]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte write protect page y write protection."]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr2 {
        #[inline(always)]
        fn default() -> M2wpr2 {
            M2wpr2(0)
        }
    }
    impl core::fmt::Debug for M2wpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr2")
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
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "M2wpr2 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}" , self . pwp (0usize) , self . pwp (1usize) , self . pwp (2usize) , self . pwp (3usize) , self . pwp (4usize) , self . pwp (5usize) , self . pwp (6usize) , self . pwp (7usize) , self . pwp (8usize) , self . pwp (9usize) , self . pwp (10usize) , self . pwp (11usize) , self . pwp (12usize) , self . pwp (13usize) , self . pwp (14usize) , self . pwp (15usize) , self . pwp (16usize) , self . pwp (17usize) , self . pwp (18usize) , self . pwp (19usize) , self . pwp (20usize) , self . pwp (21usize) , self . pwp (22usize) , self . pwp (23usize) , self . pwp (24usize) , self . pwp (25usize) , self . pwp (26usize) , self . pwp (27usize) , self . pwp (28usize) , self . pwp (29usize) , self . pwp (30usize) , self . pwp (31usize))
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Id {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "parity error detected on CPU access."]
        B_0X2 = 0x02,
        #[doc = "parity error detected on Debugger access."]
        B_0X3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "parity error detected on DMA master port o access."]
        B_0X6 = 0x06,
        #[doc = "parity error detected on DMA master port 1 access."]
        B_0X7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Id {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Id {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Id {
        #[inline(always)]
        fn from(val: u8) -> Id {
            Id::from_bits(val)
        }
    }
    impl From<Id> for u8 {
        #[inline(always)]
        fn from(val: Id) -> u8 {
            Id::to_bits(val)
        }
    }
}
