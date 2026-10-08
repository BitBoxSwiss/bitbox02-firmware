#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BCH error correction code."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bch {
    ptr: *mut u8,
}
unsafe impl Send for Bch {}
unsafe impl Sync for Bch {}
impl Bch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BCH interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Bchier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "BCH interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Bchisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "BCH interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Bchicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "BCH parity bits registers."]
    #[inline(always)]
    pub const fn pbr(self, n: usize) -> crate::common::Reg<regs::Bchpbr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "BCH decoder status register 0."]
    #[inline(always)]
    pub const fn dsr0(self) -> crate::common::Reg<regs::Bchdsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "BCH decoder status register 1."]
    #[inline(always)]
    pub const fn dsr1(self) -> crate::common::Reg<regs::Bchdsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "BCH decoder status register 2."]
    #[inline(always)]
    pub const fn dsr2(self) -> crate::common::Reg<regs::Bchdsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "BCH decoder status register 3."]
    #[inline(always)]
    pub const fn dsr3(self) -> crate::common::Reg<regs::Bchdsr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "BCH decoder status register 4."]
    #[inline(always)]
    pub const fn dsr4(self) -> crate::common::Reg<regs::Bchdsr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
#[doc = "NAND Command Sequencer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csq {
    ptr: *mut u8,
}
unsafe impl Send for Csq {}
unsafe impl Sync for Csq {}
impl Csq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command Sequencer control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Csqcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Csqcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Csqcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 3."]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Csqcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Command Sequencer address register 1."]
    #[inline(always)]
    pub const fn ar1(self) -> crate::common::Reg<regs::Csqar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Command Sequencer address register 2."]
    #[inline(always)]
    pub const fn ar2(self) -> crate::common::Reg<regs::Csqar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Command Sequencer interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Csqier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Command Sequencer interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Csqisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Command Sequencer interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Csqicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Command Sequencer error mapping status register."]
    #[inline(always)]
    pub const fn emsr(self) -> crate::common::Reg<regs::Csqemsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
#[doc = "Flexible memory controller (N6 with BCH ECC and Command Sequencer)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc {
    ptr: *mut u8,
}
unsafe impl Send for Fmc {}
unsafe impl Sync for Fmc {}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn nor_psram(self) -> NorPsram {
        unsafe { NorPsram::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn nand(self) -> Nand {
        unsafe { Nand::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn sdram(self) -> Sdram {
        unsafe { Sdram::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn nand_ext(self) -> NandExt {
        unsafe { NandExt::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn csq(self) -> Csq {
        unsafe { Csq::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn bch(self) -> Bch {
        unsafe { Bch::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nand {
    ptr: *mut u8,
}
unsafe impl Send for Nand {}
unsafe impl Sync for Nand {}
impl Nand {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NAND Flash control registers."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FIFO status and interrupt register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Common memory space timing register."]
    #[inline(always)]
    pub const fn pmem(self) -> crate::common::Reg<regs::Pmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Attribute memory space timing register."]
    #[inline(always)]
    pub const fn patt(self) -> crate::common::Reg<regs::Patt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ECC result registers."]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
#[doc = "NAND extended interrupt registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NandExt {
    ptr: *mut u8,
}
unsafe impl Send for NandExt {}
unsafe impl Sync for NandExt {}
impl NandExt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NAND interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::NandIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "NAND interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::NandIsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "NAND interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::NandIcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NorPsram {
    ptr: *mut u8,
}
unsafe impl Send for NorPsram {}
unsafe impl Sync for NorPsram {}
impl NorPsram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register for bank 1."]
    #[inline(always)]
    pub const fn bcr1(self) -> crate::common::Reg<regs::Bcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register for bank 1."]
    #[inline(always)]
    pub const fn btr(self, n: usize) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register for bank 2."]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 8usize) as _) }
    }
    #[doc = "PSRAM chip select counter register."]
    #[inline(always)]
    pub const fn pcscntr(self) -> crate::common::Reg<regs::Pcscntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1."]
    #[inline(always)]
    pub const fn bwtr(self, n: usize) -> crate::common::Reg<regs::Bwtr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdram {
    ptr: *mut u8,
}
unsafe impl Send for Sdram {}
unsafe impl Sync for Sdram {}
impl Sdram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDRAM control registers 1."]
    #[inline(always)]
    pub const fn sdcr(self, n: usize) -> crate::common::Reg<regs::Sdcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM timing registers 1."]
    #[inline(always)]
    pub const fn sdtr(self, n: usize) -> crate::common::Reg<regs::Sdtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM Command Mode register."]
    #[inline(always)]
    pub const fn sdcmr(self) -> crate::common::Reg<regs::Sdcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SDRAM refresh timer register."]
    #[inline(always)]
    pub const fn sdrtr(self) -> crate::common::Reg<regs::Sdrtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SDRAM status register."]
    #[inline(always)]
    pub const fn sdsr(self) -> crate::common::Reg<regs::Sdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "BCH decoder status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr0(pub u32);
    impl Bchdsr0 {
        #[doc = "Decoder uncorrectable error."]
        #[inline(always)]
        pub const fn due(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error."]
        #[inline(always)]
        pub fn set_due(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error found."]
        #[inline(always)]
        pub const fn def(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found."]
        #[inline(always)]
        pub fn set_def(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error number."]
        #[inline(always)]
        pub const fn den(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Decoder error number."]
        #[inline(always)]
        pub fn set_den(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Bchdsr0 {
        #[inline(always)]
        fn default() -> Bchdsr0 {
            Bchdsr0(0)
        }
    }
    impl core::fmt::Debug for Bchdsr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr0")
                .field("due", &self.due())
                .field("def", &self.def())
                .field("den", &self.den())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr0 {{ due: {=bool:?}, def: {=bool:?}, den: {=u8:?} }}",
                self.due(),
                self.def(),
                self.den()
            )
        }
    }
    #[doc = "BCH decoder status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr1(pub u32);
    impl Bchdsr1 {
        #[doc = "Error bit position 1."]
        #[inline(always)]
        pub const fn ebp1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 1."]
        #[inline(always)]
        pub fn set_ebp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 2."]
        #[inline(always)]
        pub const fn ebp2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 2."]
        #[inline(always)]
        pub fn set_ebp2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr1 {
        #[inline(always)]
        fn default() -> Bchdsr1 {
            Bchdsr1(0)
        }
    }
    impl core::fmt::Debug for Bchdsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr1")
                .field("ebp1", &self.ebp1())
                .field("ebp2", &self.ebp2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr1 {{ ebp1: {=u16:?}, ebp2: {=u16:?} }}",
                self.ebp1(),
                self.ebp2()
            )
        }
    }
    #[doc = "BCH decoder status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr2(pub u32);
    impl Bchdsr2 {
        #[doc = "Error bit position 3."]
        #[inline(always)]
        pub const fn ebp3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 3."]
        #[inline(always)]
        pub fn set_ebp3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 4."]
        #[inline(always)]
        pub const fn ebp4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 4."]
        #[inline(always)]
        pub fn set_ebp4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr2 {
        #[inline(always)]
        fn default() -> Bchdsr2 {
            Bchdsr2(0)
        }
    }
    impl core::fmt::Debug for Bchdsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr2")
                .field("ebp3", &self.ebp3())
                .field("ebp4", &self.ebp4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr2 {{ ebp3: {=u16:?}, ebp4: {=u16:?} }}",
                self.ebp3(),
                self.ebp4()
            )
        }
    }
    #[doc = "BCH decoder status register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr3(pub u32);
    impl Bchdsr3 {
        #[doc = "Error bit position 5."]
        #[inline(always)]
        pub const fn ebp5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 5."]
        #[inline(always)]
        pub fn set_ebp5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 6."]
        #[inline(always)]
        pub const fn ebp6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 6."]
        #[inline(always)]
        pub fn set_ebp6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr3 {
        #[inline(always)]
        fn default() -> Bchdsr3 {
            Bchdsr3(0)
        }
    }
    impl core::fmt::Debug for Bchdsr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr3")
                .field("ebp5", &self.ebp5())
                .field("ebp6", &self.ebp6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr3 {{ ebp5: {=u16:?}, ebp6: {=u16:?} }}",
                self.ebp5(),
                self.ebp6()
            )
        }
    }
    #[doc = "BCH decoder status register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr4(pub u32);
    impl Bchdsr4 {
        #[doc = "Error bit position 7."]
        #[inline(always)]
        pub const fn ebp7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 7."]
        #[inline(always)]
        pub fn set_ebp7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 8."]
        #[inline(always)]
        pub const fn ebp8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 8."]
        #[inline(always)]
        pub fn set_ebp8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr4 {
        #[inline(always)]
        fn default() -> Bchdsr4 {
            Bchdsr4(0)
        }
    }
    impl core::fmt::Debug for Bchdsr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr4")
                .field("ebp7", &self.ebp7())
                .field("ebp8", &self.ebp8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr4 {{ ebp7: {=u16:?}, ebp8: {=u16:?} }}",
                self.ebp7(),
                self.ebp8()
            )
        }
    }
    #[doc = "BCH interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchicr(pub u32);
    impl Bchicr {
        #[doc = "Clear decoder uncorrectable error flag."]
        #[inline(always)]
        pub const fn cduef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder uncorrectable error flag."]
        #[inline(always)]
        pub fn set_cduef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear decoder error ready flag."]
        #[inline(always)]
        pub const fn cderf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder error ready flag."]
        #[inline(always)]
        pub fn set_cderf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear decoder error found flag."]
        #[inline(always)]
        pub const fn cdeff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder error found flag."]
        #[inline(always)]
        pub fn set_cdeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear decoder syndrome ready flag."]
        #[inline(always)]
        pub const fn cdsrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder syndrome ready flag."]
        #[inline(always)]
        pub fn set_cdsrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear encoder parity bits ready flag."]
        #[inline(always)]
        pub const fn cepbrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear encoder parity bits ready flag."]
        #[inline(always)]
        pub fn set_cepbrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchicr {
        #[inline(always)]
        fn default() -> Bchicr {
            Bchicr(0)
        }
    }
    impl core::fmt::Debug for Bchicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchicr")
                .field("cduef", &self.cduef())
                .field("cderf", &self.cderf())
                .field("cdeff", &self.cdeff())
                .field("cdsrf", &self.cdsrf())
                .field("cepbrf", &self.cepbrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bchicr {{ cduef: {=bool:?}, cderf: {=bool:?}, cdeff: {=bool:?}, cdsrf: {=bool:?}, cepbrf: {=bool:?} }}" , self . cduef () , self . cderf () , self . cdeff () , self . cdsrf () , self . cepbrf ())
        }
    }
    #[doc = "BCH interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchier(pub u32);
    impl Bchier {
        #[doc = "Decoder uncorrectable error interrupt enable."]
        #[inline(always)]
        pub const fn dueie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error interrupt enable."]
        #[inline(always)]
        pub fn set_dueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error ready interrupt enable."]
        #[inline(always)]
        pub const fn derie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error ready interrupt enable."]
        #[inline(always)]
        pub fn set_derie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error found interrupt enable."]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found interrupt enable."]
        #[inline(always)]
        pub fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Decoder syndrome ready interrupt enable."]
        #[inline(always)]
        pub const fn dsrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder syndrome ready interrupt enable."]
        #[inline(always)]
        pub fn set_dsrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Encoder parity bits ready interrupt enable."]
        #[inline(always)]
        pub const fn epbrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder parity bits ready interrupt enable."]
        #[inline(always)]
        pub fn set_epbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchier {
        #[inline(always)]
        fn default() -> Bchier {
            Bchier(0)
        }
    }
    impl core::fmt::Debug for Bchier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchier")
                .field("dueie", &self.dueie())
                .field("derie", &self.derie())
                .field("defie", &self.defie())
                .field("dsrie", &self.dsrie())
                .field("epbrie", &self.epbrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bchier {{ dueie: {=bool:?}, derie: {=bool:?}, defie: {=bool:?}, dsrie: {=bool:?}, epbrie: {=bool:?} }}" , self . dueie () , self . derie () , self . defie () , self . dsrie () , self . epbrie ())
        }
    }
    #[doc = "BCH interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchisr(pub u32);
    impl Bchisr {
        #[doc = "Decoder uncorrectable error flag."]
        #[inline(always)]
        pub const fn duef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error flag."]
        #[inline(always)]
        pub fn set_duef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error ready flag."]
        #[inline(always)]
        pub const fn derf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error ready flag."]
        #[inline(always)]
        pub fn set_derf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error found flag."]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found flag."]
        #[inline(always)]
        pub fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Decoder syndrome ready flag."]
        #[inline(always)]
        pub const fn dsrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder syndrome ready flag."]
        #[inline(always)]
        pub fn set_dsrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Encoder parity bits ready flag."]
        #[inline(always)]
        pub const fn epbrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder parity bits ready flag."]
        #[inline(always)]
        pub fn set_epbrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchisr {
        #[inline(always)]
        fn default() -> Bchisr {
            Bchisr(0)
        }
    }
    impl core::fmt::Debug for Bchisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchisr")
                .field("duef", &self.duef())
                .field("derf", &self.derf())
                .field("deff", &self.deff())
                .field("dsrf", &self.dsrf())
                .field("epbrf", &self.epbrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchisr {{ duef: {=bool:?}, derf: {=bool:?}, deff: {=bool:?}, dsrf: {=bool:?}, epbrf: {=bool:?} }}",
                self.duef(),
                self.derf(),
                self.deff(),
                self.dsrf(),
                self.epbrf()
            )
        }
    }
    #[doc = "BCH parity bits register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchpbr(pub u32);
    impl Bchpbr {
        #[doc = "BCH parity bits."]
        #[inline(always)]
        pub const fn bchpb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BCH parity bits."]
        #[inline(always)]
        pub fn set_bchpb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bchpbr {
        #[inline(always)]
        fn default() -> Bchpbr {
            Bchpbr(0)
        }
    }
    impl core::fmt::Debug for Bchpbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchpbr").field("bchpb", &self.bchpb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchpbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bchpbr {{ bchpb: {=u32:?} }}", self.bchpb())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register for bank 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:."]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:."]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type Defines the type of external memory attached to the corresponding memory bank."]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type Defines the type of external memory attached to the corresponding memory bank."]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width Defines the external memory device width, valid for all type of memories."]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width Defines the external memory device width, valid for all type of memories."]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable Enables NOR Flash memory access operations."]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable Enables NOR Flash memory access operations."]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode."]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode."]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode."]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode."]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:."]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:."]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC."]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC."]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode."]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode."]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved."]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved."]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
        #[inline(always)]
        pub const fn cburstrw(&self) -> super::vals::Cburstrw {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cburstrw::from_bits(val as u8)
        }
        #[doc = "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: super::vals::Cburstrw) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)."]
        #[inline(always)]
        pub const fn cclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)."]
        #[inline(always)]
        pub fn set_cclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub const fn wfdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub fn set_wfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low."]
        #[inline(always)]
        pub const fn nblset(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low."]
        #[inline(always)]
        pub fn set_nblset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .field("cclken", &self.cclken())
                .field("wfdis", &self.wfdis())
                .field("nblset", &self.nblset())
                .field("fmcen", &self.fmcen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bcr {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {:?}, cclken: {=bool:?}, wfdis: {=bool:?}, nblset: {=u8:?}, fmcen: {=bool:?} }}" , self . mbken () , self . muxen () , self . mtyp () , self . mwid () , self . faccen () , self . bursten () , self . waitpol () , self . waitcfg () , self . wren () , self . waiten () , self . extmod () , self . asyncwait () , self . cpsize () , self . cburstrw () , self . cclken () , self . wfdis () , self . nblset () , self . fmcen ())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register for bank 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr1(pub u32);
    impl Bcr1 {
        #[doc = "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:."]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:."]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type Defines the type of external memory attached to the corresponding memory bank."]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type Defines the type of external memory attached to the corresponding memory bank."]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width Defines the external memory device width, valid for all type of memories."]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width Defines the external memory device width, valid for all type of memories."]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable Enables NOR Flash memory access operations."]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable Enables NOR Flash memory access operations."]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode."]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode."]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode."]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous mode."]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:."]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in Synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:."]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC."]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC."]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode."]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in Synchronous mode."]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows: Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP = 0x0 or 0x01) Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved."]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Others: reserved."]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
        #[inline(always)]
        pub const fn cburstrw(&self) -> super::vals::Cburstrw {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cburstrw::from_bits(val as u8)
        }
        #[doc = "Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: super::vals::Cburstrw) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)."]
        #[inline(always)]
        pub const fn cclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous clock enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the FMC_CLK continuous clock. Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don’t care. Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)."]
        #[inline(always)]
        pub fn set_cclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub const fn wfdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Write FIFO disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub fn set_wfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low."]
        #[inline(always)]
        pub const fn nblset(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Byte lane (NBL) setup These bits configure the NBL setup timing from NBLx low to chip select NEx low."]
        #[inline(always)]
        pub fn set_nblset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FMC controller enable This bit enables or disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the FMC_BCR1 register."]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bcr1 {
        #[inline(always)]
        fn default() -> Bcr1 {
            Bcr1(0)
        }
    }
    impl core::fmt::Debug for Bcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr1")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .field("cclken", &self.cclken())
                .field("wfdis", &self.wfdis())
                .field("nblset", &self.nblset())
                .field("fmcen", &self.fmcen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bcr1 {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {:?}, cclken: {=bool:?}, wfdis: {=bool:?}, nblset: {=u8:?}, fmcen: {=bool:?} }}" , self . mbken () , self . muxen () , self . mtyp () , self . mwid () , self . faccen () , self . bursten () , self . waitpol () , self . waitcfg () , self . wren () , self . waiten () , self . extmod () , self . asyncwait () , self . cpsize () , self . cburstrw () , self . cclken () , self . wfdis () , self . nblset () , self . fmcen ())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register for bank 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr(pub u32);
    impl Btr {
        #[doc = "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)."]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)."]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "(see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
        #[inline(always)]
        pub const fn datlat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "(see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
        #[inline(always)]
        pub fn set_datlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:."]
        #[inline(always)]
        pub const fn datahld(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:."]
        #[inline(always)]
        pub fn set_datahld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Btr {
        #[inline(always)]
        fn default() -> Btr {
            Btr(0)
        }
    }
    impl core::fmt::Debug for Btr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("clkdiv", &self.clkdiv())
                .field("datlat", &self.datlat())
                .field("accmod", &self.accmod())
                .field("datahld", &self.datahld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Btr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Btr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, clkdiv: {=u8:?}, datlat: {=u8:?}, accmod: {:?}, datahld: {=u8:?} }}" , self . addset () , self . addhld () , self . datast () , self . busturn () , self . clkdiv () , self . datlat () , self . accmod () , self . datahld ())
        }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bwtr(pub u32);
    impl Bwtr {
        #[doc = "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:."]
        #[inline(always)]
        pub const fn datahld(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:."]
        #[inline(always)]
        pub fn set_datahld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Bwtr {
        #[inline(always)]
        fn default() -> Bwtr {
            Bwtr(0)
        }
    }
    impl core::fmt::Debug for Bwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bwtr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("accmod", &self.accmod())
                .field("datahld", &self.datahld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bwtr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, accmod: {:?}, datahld: {=u8:?} }}" , self . addset () , self . addhld () , self . datast () , self . busturn () , self . accmod () , self . datahld ())
        }
    }
    #[doc = "Command Sequencer address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqar1(pub u32);
    impl Csqar1 {
        #[doc = "Address cycle 1."]
        #[inline(always)]
        pub const fn addc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 1."]
        #[inline(always)]
        pub fn set_addc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Address cycle 2."]
        #[inline(always)]
        pub const fn addc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 2."]
        #[inline(always)]
        pub fn set_addc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Address cycle 3."]
        #[inline(always)]
        pub const fn addc3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 3."]
        #[inline(always)]
        pub fn set_addc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Address cycle 4."]
        #[inline(always)]
        pub const fn addc4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 4."]
        #[inline(always)]
        pub fn set_addc4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Csqar1 {
        #[inline(always)]
        fn default() -> Csqar1 {
            Csqar1(0)
        }
    }
    impl core::fmt::Debug for Csqar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqar1")
                .field("addc1", &self.addc1())
                .field("addc2", &self.addc2())
                .field("addc3", &self.addc3())
                .field("addc4", &self.addc4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqar1 {{ addc1: {=u8:?}, addc2: {=u8:?}, addc3: {=u8:?}, addc4: {=u8:?} }}",
                self.addc1(),
                self.addc2(),
                self.addc3(),
                self.addc4()
            )
        }
    }
    #[doc = "Command Sequencer address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqar2(pub u32);
    impl Csqar2 {
        #[doc = "Address cycle 5."]
        #[inline(always)]
        pub const fn addc5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 5."]
        #[inline(always)]
        pub fn set_addc5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Spare area address offset."]
        #[inline(always)]
        pub const fn sao(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Spare area address offset."]
        #[inline(always)]
        pub fn set_sao(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Csqar2 {
        #[inline(always)]
        fn default() -> Csqar2 {
            Csqar2(0)
        }
    }
    impl core::fmt::Debug for Csqar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqar2")
                .field("addc5", &self.addc5())
                .field("sao", &self.sao())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqar2 {{ addc5: {=u8:?}, sao: {=u16:?} }}",
                self.addc5(),
                self.sao()
            )
        }
    }
    #[doc = "Command Sequencer configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr1(pub u32);
    impl Csqcfgr1 {
        #[doc = "Command cycle 2 enable."]
        #[inline(always)]
        pub const fn cmd2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command cycle 2 enable."]
        #[inline(always)]
        pub fn set_cmd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA request data enable."]
        #[inline(always)]
        pub const fn dmaden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request data enable."]
        #[inline(always)]
        pub fn set_dmaden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address cycle number."]
        #[inline(always)]
        pub const fn acynbr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Address cycle number."]
        #[inline(always)]
        pub fn set_acynbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Command 1."]
        #[inline(always)]
        pub const fn cmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Command 1."]
        #[inline(always)]
        pub fn set_cmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Command 2."]
        #[inline(always)]
        pub const fn cmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Command 2."]
        #[inline(always)]
        pub fn set_cmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Command 1 timing."]
        #[inline(always)]
        pub const fn cmd1t(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Command 1 timing."]
        #[inline(always)]
        pub fn set_cmd1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Command 2 timing."]
        #[inline(always)]
        pub const fn cmd2t(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Command 2 timing."]
        #[inline(always)]
        pub fn set_cmd2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Csqcfgr1 {
        #[inline(always)]
        fn default() -> Csqcfgr1 {
            Csqcfgr1(0)
        }
    }
    impl core::fmt::Debug for Csqcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr1")
                .field("cmd2en", &self.cmd2en())
                .field("dmaden", &self.dmaden())
                .field("acynbr", &self.acynbr())
                .field("cmd1", &self.cmd1())
                .field("cmd2", &self.cmd2())
                .field("cmd1t", &self.cmd1t())
                .field("cmd2t", &self.cmd2t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csqcfgr1 {{ cmd2en: {=bool:?}, dmaden: {=bool:?}, acynbr: {=u8:?}, cmd1: {=u8:?}, cmd2: {=u8:?}, cmd1t: {=bool:?}, cmd2t: {=bool:?} }}" , self . cmd2en () , self . dmaden () , self . acynbr () , self . cmd1 () , self . cmd2 () , self . cmd1t () , self . cmd2t ())
        }
    }
    #[doc = "Command Sequencer configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr2(pub u32);
    impl Csqcfgr2 {
        #[doc = "Sequencer spare data transfer enable."]
        #[inline(always)]
        pub const fn sqsdten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sequencer spare data transfer enable."]
        #[inline(always)]
        pub fn set_sqsdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Random command 2 sequencer enable."]
        #[inline(always)]
        pub const fn rcmd2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Random command 2 sequencer enable."]
        #[inline(always)]
        pub fn set_rcmd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA request decoding status enable."]
        #[inline(always)]
        pub const fn dmasen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request decoding status enable."]
        #[inline(always)]
        pub fn set_dmasen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Random command 1."]
        #[inline(always)]
        pub const fn rcmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Random command 1."]
        #[inline(always)]
        pub fn set_rcmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Random command 2."]
        #[inline(always)]
        pub const fn rcmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Random command 2."]
        #[inline(always)]
        pub fn set_rcmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Random command 1 timing."]
        #[inline(always)]
        pub const fn rcmd1t(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Random command 1 timing."]
        #[inline(always)]
        pub fn set_rcmd1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Csqcfgr2 {
        #[inline(always)]
        fn default() -> Csqcfgr2 {
            Csqcfgr2(0)
        }
    }
    impl core::fmt::Debug for Csqcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr2")
                .field("sqsdten", &self.sqsdten())
                .field("rcmd2en", &self.rcmd2en())
                .field("dmasen", &self.dmasen())
                .field("rcmd1", &self.rcmd1())
                .field("rcmd2", &self.rcmd2())
                .field("rcmd1t", &self.rcmd1t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csqcfgr2 {{ sqsdten: {=bool:?}, rcmd2en: {=bool:?}, dmasen: {=bool:?}, rcmd1: {=u8:?}, rcmd2: {=u8:?}, rcmd1t: {=bool:?} }}" , self . sqsdten () , self . rcmd2en () , self . dmasen () , self . rcmd1 () , self . rcmd2 () , self . rcmd1t ())
        }
    }
    #[doc = "Command Sequencer configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr3(pub u32);
    impl Csqcfgr3 {
        #[doc = "Number of sectors to read/write."]
        #[inline(always)]
        pub const fn snbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of sectors to read/write."]
        #[inline(always)]
        pub fn set_snbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Address cycle 1 timing."]
        #[inline(always)]
        pub const fn ac1t(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 1 timing."]
        #[inline(always)]
        pub fn set_ac1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Address cycle 2 timing."]
        #[inline(always)]
        pub const fn ac2t(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 2 timing."]
        #[inline(always)]
        pub fn set_ac2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Address cycle 3 timing."]
        #[inline(always)]
        pub const fn ac3t(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 3 timing."]
        #[inline(always)]
        pub fn set_ac3t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Address cycle 4 timing."]
        #[inline(always)]
        pub const fn ac4t(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 4 timing."]
        #[inline(always)]
        pub fn set_ac4t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address cycle 5 timing."]
        #[inline(always)]
        pub const fn ac5t(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 5 timing."]
        #[inline(always)]
        pub fn set_ac5t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Spare data transfer timing."]
        #[inline(always)]
        pub const fn sdt(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Spare data transfer timing."]
        #[inline(always)]
        pub fn set_sdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Random address cycle 1 timing."]
        #[inline(always)]
        pub const fn rac1t(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Random address cycle 1 timing."]
        #[inline(always)]
        pub fn set_rac1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Random address cycle 2 timing."]
        #[inline(always)]
        pub const fn rac2t(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Random address cycle 2 timing."]
        #[inline(always)]
        pub fn set_rac2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Csqcfgr3 {
        #[inline(always)]
        fn default() -> Csqcfgr3 {
            Csqcfgr3(0)
        }
    }
    impl core::fmt::Debug for Csqcfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr3")
                .field("snbr", &self.snbr())
                .field("ac1t", &self.ac1t())
                .field("ac2t", &self.ac2t())
                .field("ac3t", &self.ac3t())
                .field("ac4t", &self.ac4t())
                .field("ac5t", &self.ac5t())
                .field("sdt", &self.sdt())
                .field("rac1t", &self.rac1t())
                .field("rac2t", &self.rac2t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csqcfgr3 {{ snbr: {=u8:?}, ac1t: {=bool:?}, ac2t: {=bool:?}, ac3t: {=bool:?}, ac4t: {=bool:?}, ac5t: {=bool:?}, sdt: {=bool:?}, rac1t: {=bool:?}, rac2t: {=bool:?} }}" , self . snbr () , self . ac1t () , self . ac2t () , self . ac3t () , self . ac4t () , self . ac5t () , self . sdt () , self . rac1t () , self . rac2t ())
        }
    }
    #[doc = "Command Sequencer control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcr(pub u32);
    impl Csqcr {
        #[doc = "Command Sequencer start."]
        #[inline(always)]
        pub const fn csqstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command Sequencer start."]
        #[inline(always)]
        pub fn set_csqstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Csqcr {
        #[inline(always)]
        fn default() -> Csqcr {
            Csqcr(0)
        }
    }
    impl core::fmt::Debug for Csqcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcr").field("csqstart", &self.csqstart()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csqcr {{ csqstart: {=bool:?} }}", self.csqstart())
        }
    }
    #[doc = "Command Sequencer error mapping status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqemsr(pub u32);
    impl Csqemsr {
        #[doc = "Sector error mapping."]
        #[inline(always)]
        pub const fn sem(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sector error mapping."]
        #[inline(always)]
        pub fn set_sem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Csqemsr {
        #[inline(always)]
        fn default() -> Csqemsr {
            Csqemsr(0)
        }
    }
    impl core::fmt::Debug for Csqemsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqemsr").field("sem", &self.sem()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqemsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csqemsr {{ sem: {=u16:?} }}", self.sem())
        }
    }
    #[doc = "Command Sequencer interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqicr(pub u32);
    impl Csqicr {
        #[doc = "Clear transfer complete flag."]
        #[inline(always)]
        pub const fn ctcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer complete flag."]
        #[inline(always)]
        pub fn set_ctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear sector complete flag."]
        #[inline(always)]
        pub const fn cscf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector complete flag."]
        #[inline(always)]
        pub fn set_cscf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear sector error flag."]
        #[inline(always)]
        pub const fn csef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector error flag."]
        #[inline(always)]
        pub fn set_csef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear sector uncorrectable error flag."]
        #[inline(always)]
        pub const fn csuef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector uncorrectable error flag."]
        #[inline(always)]
        pub fn set_csuef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear command transfer complete flag."]
        #[inline(always)]
        pub const fn ccmdtcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear command transfer complete flag."]
        #[inline(always)]
        pub fn set_ccmdtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqicr {
        #[inline(always)]
        fn default() -> Csqicr {
            Csqicr(0)
        }
    }
    impl core::fmt::Debug for Csqicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqicr")
                .field("ctcf", &self.ctcf())
                .field("cscf", &self.cscf())
                .field("csef", &self.csef())
                .field("csuef", &self.csuef())
                .field("ccmdtcf", &self.ccmdtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqicr {{ ctcf: {=bool:?}, cscf: {=bool:?}, csef: {=bool:?}, csuef: {=bool:?}, ccmdtcf: {=bool:?} }}",
                self.ctcf(),
                self.cscf(),
                self.csef(),
                self.csuef(),
                self.ccmdtcf()
            )
        }
    }
    #[doc = "Command Sequencer interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqier(pub u32);
    impl Csqier {
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sector complete interrupt enable."]
        #[inline(always)]
        pub const fn scie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sector complete interrupt enable."]
        #[inline(always)]
        pub fn set_scie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sector error interrupt enable."]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sector error interrupt enable."]
        #[inline(always)]
        pub fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sector uncorrectable error interrupt enable."]
        #[inline(always)]
        pub const fn sueie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Sector uncorrectable error interrupt enable."]
        #[inline(always)]
        pub fn set_sueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn cmdtcie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer complete interrupt enable."]
        #[inline(always)]
        pub fn set_cmdtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqier {
        #[inline(always)]
        fn default() -> Csqier {
            Csqier(0)
        }
    }
    impl core::fmt::Debug for Csqier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqier")
                .field("tcie", &self.tcie())
                .field("scie", &self.scie())
                .field("seie", &self.seie())
                .field("sueie", &self.sueie())
                .field("cmdtcie", &self.cmdtcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqier {{ tcie: {=bool:?}, scie: {=bool:?}, seie: {=bool:?}, sueie: {=bool:?}, cmdtcie: {=bool:?} }}",
                self.tcie(),
                self.scie(),
                self.seie(),
                self.sueie(),
                self.cmdtcie()
            )
        }
    }
    #[doc = "Command Sequencer interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqisr(pub u32);
    impl Csqisr {
        #[doc = "Transfer complete flag."]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete flag."]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sector complete flag."]
        #[inline(always)]
        pub const fn scf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sector complete flag."]
        #[inline(always)]
        pub fn set_scf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sector error flag."]
        #[inline(always)]
        pub const fn sef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sector error flag."]
        #[inline(always)]
        pub fn set_sef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sector uncorrectable error flag."]
        #[inline(always)]
        pub const fn suef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Sector uncorrectable error flag."]
        #[inline(always)]
        pub fn set_suef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command transfer complete flag."]
        #[inline(always)]
        pub const fn cmdtcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer complete flag."]
        #[inline(always)]
        pub fn set_cmdtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqisr {
        #[inline(always)]
        fn default() -> Csqisr {
            Csqisr(0)
        }
    }
    impl core::fmt::Debug for Csqisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqisr")
                .field("tcf", &self.tcf())
                .field("scf", &self.scf())
                .field("sef", &self.sef())
                .field("suef", &self.suef())
                .field("cmdtcf", &self.cmdtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqisr {{ tcf: {=bool:?}, scf: {=bool:?}, sef: {=bool:?}, suef: {=bool:?}, cmdtcf: {=bool:?} }}",
                self.tcf(),
                self.scf(),
                self.sef(),
                self.suef(),
                self.cmdtcf()
            )
        }
    }
    #[doc = "NAND interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NandIcr(pub u32);
    impl NandIcr {
        #[doc = "Clear interrupt rising edge."]
        #[inline(always)]
        pub const fn circ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear interrupt rising edge."]
        #[inline(always)]
        pub fn set_circ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear interrupt high-level."]
        #[inline(always)]
        pub const fn cilc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear interrupt high-level."]
        #[inline(always)]
        pub fn set_cilc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear interrupt falling edge."]
        #[inline(always)]
        pub const fn cifc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear interrupt falling edge."]
        #[inline(always)]
        pub fn set_cifc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for NandIcr {
        #[inline(always)]
        fn default() -> NandIcr {
            NandIcr(0)
        }
    }
    impl core::fmt::Debug for NandIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NandIcr")
                .field("circ", &self.circ())
                .field("cilc", &self.cilc())
                .field("cifc", &self.cifc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NandIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "NandIcr {{ circ: {=bool:?}, cilc: {=bool:?}, cifc: {=bool:?} }}",
                self.circ(),
                self.cilc(),
                self.cifc()
            )
        }
    }
    #[doc = "NAND interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NandIer(pub u32);
    impl NandIer {
        #[doc = "Interrupt rising edge enable."]
        #[inline(always)]
        pub const fn irsen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge enable."]
        #[inline(always)]
        pub fn set_irsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt high-level enable."]
        #[inline(always)]
        pub const fn ilsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level enable."]
        #[inline(always)]
        pub fn set_ilsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt falling edge enable."]
        #[inline(always)]
        pub const fn ifsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge enable."]
        #[inline(always)]
        pub fn set_ifsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for NandIer {
        #[inline(always)]
        fn default() -> NandIer {
            NandIer(0)
        }
    }
    impl core::fmt::Debug for NandIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NandIer")
                .field("irsen", &self.irsen())
                .field("ilsen", &self.ilsen())
                .field("ifsen", &self.ifsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NandIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "NandIer {{ irsen: {=bool:?}, ilsen: {=bool:?}, ifsen: {=bool:?} }}",
                self.irsen(),
                self.ilsen(),
                self.ifsen()
            )
        }
    }
    #[doc = "NAND interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NandIsr(pub u32);
    impl NandIsr {
        #[doc = "Interrupt rising edge status."]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge status."]
        #[inline(always)]
        pub fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt high-level status."]
        #[inline(always)]
        pub const fn ils(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level status."]
        #[inline(always)]
        pub fn set_ils(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt falling edge status."]
        #[inline(always)]
        pub const fn ifs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge status."]
        #[inline(always)]
        pub fn set_ifs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for NandIsr {
        #[inline(always)]
        fn default() -> NandIsr {
            NandIsr(0)
        }
    }
    impl core::fmt::Debug for NandIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NandIsr")
                .field("irs", &self.irs())
                .field("ils", &self.ils())
                .field("ifs", &self.ifs())
                .field("fempt", &self.fempt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NandIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "NandIsr {{ irs: {=bool:?}, ils: {=bool:?}, ifs: {=bool:?}, fempt: {=bool:?} }}",
                self.irs(),
                self.ils(),
                self.ifs(),
                self.fempt()
            )
        }
    }
    #[doc = "Attribute memory space timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Patt(pub u32);
    impl Patt {
        #[doc = "Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:."]
        #[inline(always)]
        pub const fn attset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:."]
        #[inline(always)]
        pub fn set_attset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:."]
        #[inline(always)]
        pub const fn attwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:."]
        #[inline(always)]
        pub fn set_attwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:."]
        #[inline(always)]
        pub const fn atthold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:."]
        #[inline(always)]
        pub fn set_atthold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:."]
        #[inline(always)]
        pub const fn atthiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:."]
        #[inline(always)]
        pub fn set_atthiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Patt {
        #[inline(always)]
        fn default() -> Patt {
            Patt(0)
        }
    }
    impl core::fmt::Debug for Patt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Patt")
                .field("attset", &self.attset())
                .field("attwait", &self.attwait())
                .field("atthold", &self.atthold())
                .field("atthiz", &self.atthiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Patt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Patt {{ attset: {=u8:?}, attwait: {=u8:?}, atthold: {=u8:?}, atthiz: {=u8:?} }}",
                self.attset(),
                self.attwait(),
                self.atthold(),
                self.atthiz()
            )
        }
    }
    #[doc = "NAND Flash control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:."]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:."]
        #[inline(always)]
        pub fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus."]
        #[inline(always)]
        pub fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Memory type Defines the type of device attached to the corresponding memory bank:."]
        #[inline(always)]
        pub const fn ptyp(&self) -> super::vals::Ptyp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ptyp::from_bits(val as u8)
        }
        #[doc = "Memory type Defines the type of device attached to the corresponding memory bank:."]
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: super::vals::Ptyp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Data bus width Defines the external memory device width."]
        #[inline(always)]
        pub const fn pwid(&self) -> super::vals::Pwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Pwid::from_bits(val as u8)
        }
        #[doc = "Data bus width Defines the external memory device width."]
        #[inline(always)]
        pub fn set_pwid(&mut self, val: super::vals::Pwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "ECC computation logic enable bit."]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECC computation logic enable bit."]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
        #[inline(always)]
        pub fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
        #[inline(always)]
        pub fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "ECC page size Defines the page size for the extended ECC:."]
        #[inline(always)]
        pub const fn eccps(&self) -> super::vals::Eccps {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Eccps::from_bits(val as u8)
        }
        #[doc = "ECC page size Defines the page size for the extended ECC:."]
        #[inline(always)]
        pub fn set_eccps(&mut self, val: super::vals::Eccps) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "BCH error correction capability enable. Enables the BCH error correction engine."]
        #[inline(always)]
        pub const fn bchecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BCH error correction capability enable. Enables the BCH error correction engine."]
        #[inline(always)]
        pub fn set_bchecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("pwaiten", &self.pwaiten())
                .field("pbken", &self.pbken())
                .field("ptyp", &self.ptyp())
                .field("pwid", &self.pwid())
                .field("eccen", &self.eccen())
                .field("tclr", &self.tclr())
                .field("tar", &self.tar())
                .field("eccps", &self.eccps())
                .field("bchecc", &self.bchecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ pwaiten: {=bool:?}, pbken: {=bool:?}, ptyp: {:?}, pwid: {:?}, eccen: {=bool:?}, tclr: {=u8:?}, tar: {=u8:?}, eccps: {:?}, bchecc: {=bool:?} }}" , self . pwaiten () , self . pbken () , self . ptyp () , self . pwid () , self . eccen () , self . tclr () , self . tar () , self . eccps () , self . bchecc ())
        }
    }
    #[doc = "PSRAM chip select counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcscntr(pub u32);
    impl Pcscntr {
        #[doc = "Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
        #[inline(always)]
        pub const fn cscount(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
        #[inline(always)]
        pub fn set_cscount(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
        #[inline(always)]
        pub const fn cntben(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
        #[inline(always)]
        pub fn set_cntben(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcscntr {
        #[inline(always)]
        fn default() -> Pcscntr {
            Pcscntr(0)
        }
    }
    impl core::fmt::Debug for Pcscntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcscntr")
                .field("cscount", &self.cscount())
                .field("cntben[0]", &self.cntben(0usize))
                .field("cntben[1]", &self.cntben(1usize))
                .field("cntben[2]", &self.cntben(2usize))
                .field("cntben[3]", &self.cntben(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcscntr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcscntr {{ cscount: {=u16:?}, cntben[0]: {=bool:?}, cntben[1]: {=bool:?}, cntben[2]: {=bool:?}, cntben[3]: {=bool:?} }}" , self . cscount () , self . cntben (0usize) , self . cntben (1usize) , self . cntben (2usize) , self . cntben (3usize))
        }
    }
    #[doc = "Common memory space timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmem(pub u32);
    impl Pmem {
        #[doc = "Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:."]
        #[inline(always)]
        pub const fn memset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:."]
        #[inline(always)]
        pub fn set_memset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:."]
        #[inline(always)]
        pub const fn memwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:."]
        #[inline(always)]
        pub fn set_memwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:."]
        #[inline(always)]
        pub const fn memhold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:."]
        #[inline(always)]
        pub fn set_memhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:."]
        #[inline(always)]
        pub const fn memhiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:."]
        #[inline(always)]
        pub fn set_memhiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pmem {
        #[inline(always)]
        fn default() -> Pmem {
            Pmem(0)
        }
    }
    impl core::fmt::Debug for Pmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmem")
                .field("memset", &self.memset())
                .field("memwait", &self.memwait())
                .field("memhold", &self.memhold())
                .field("memhiz", &self.memhiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmem {{ memset: {=u8:?}, memwait: {=u8:?}, memhold: {=u8:?}, memhiz: {=u8:?} }}",
                self.memset(),
                self.memwait(),
                self.memhold(),
                self.memhiz()
            )
        }
    }
    #[doc = "SDRAM Command Mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcmr(pub u32);
    impl Sdcmr {
        #[doc = "Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
        #[inline(always)]
        pub const fn ctb(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
        #[inline(always)]
        pub fn set_ctb(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
        #[inline(always)]
        pub const fn nrfs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
        #[inline(always)]
        pub fn set_nrfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
        #[inline(always)]
        pub const fn mrd(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x1fff;
            val as u16
        }
        #[doc = "Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
        #[inline(always)]
        pub fn set_mrd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 9usize)) | (((val as u32) & 0x1fff) << 9usize);
        }
    }
    impl Default for Sdcmr {
        #[inline(always)]
        fn default() -> Sdcmr {
            Sdcmr(0)
        }
    }
    impl core::fmt::Debug for Sdcmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcmr")
                .field("mode", &self.mode())
                .field("ctb[0]", &self.ctb(0usize))
                .field("ctb[1]", &self.ctb(1usize))
                .field("nrfs", &self.nrfs())
                .field("mrd", &self.mrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdcmr {{ mode: {:?}, ctb[0]: {=bool:?}, ctb[1]: {=bool:?}, nrfs: {=u8:?}, mrd: {=u16:?} }}",
                self.mode(),
                self.ctb(0usize),
                self.ctb(1usize),
                self.nrfs(),
                self.mrd()
            )
        }
    }
    #[doc = "SDRAM control registers 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcr(pub u32);
    impl Sdcr {
        #[doc = "Number of column address bits These bits define the number of bits of a column address."]
        #[inline(always)]
        pub const fn nc(&self) -> super::vals::Nc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Nc::from_bits(val as u8)
        }
        #[doc = "Number of column address bits These bits define the number of bits of a column address."]
        #[inline(always)]
        pub fn set_nc(&mut self, val: super::vals::Nc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of row address bits These bits define the number of bits of a row address."]
        #[inline(always)]
        pub const fn nr(&self) -> super::vals::Nr {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Nr::from_bits(val as u8)
        }
        #[doc = "Number of row address bits These bits define the number of bits of a row address."]
        #[inline(always)]
        pub fn set_nr(&mut self, val: super::vals::Nr) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width. These bits define the memory device width."]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width. These bits define the memory device width."]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of internal banks This bit sets the number of internal banks."]
        #[inline(always)]
        pub const fn nb(&self) -> super::vals::Nb {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Nb::from_bits(val as u8)
        }
        #[doc = "Number of internal banks This bit sets the number of internal banks."]
        #[inline(always)]
        pub fn set_nb(&mut self, val: super::vals::Nb) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles."]
        #[inline(always)]
        pub const fn cas(&self) -> super::vals::Cas {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Cas::from_bits(val as u8)
        }
        #[doc = "CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles."]
        #[inline(always)]
        pub fn set_cas(&mut self, val: super::vals::Cas) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "Write protection This bit enables write mode access to the SDRAM bank."]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection This bit enables write mode access to the SDRAM bank."]
        #[inline(always)]
        pub fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
        #[inline(always)]
        pub const fn sdclk(&self) -> super::vals::Sdclk {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Sdclk::from_bits(val as u8)
        }
        #[doc = "SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
        #[inline(always)]
        pub fn set_sdclk(&mut self, val: super::vals::Sdclk) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
        #[inline(always)]
        pub const fn rburst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
        #[inline(always)]
        pub fn set_rburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
        #[inline(always)]
        pub const fn rpipe(&self) -> super::vals::Rpipe {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Rpipe::from_bits(val as u8)
        }
        #[doc = "Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
        #[inline(always)]
        pub fn set_rpipe(&mut self, val: super::vals::Rpipe) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Sdcr {
        #[inline(always)]
        fn default() -> Sdcr {
            Sdcr(0)
        }
    }
    impl core::fmt::Debug for Sdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcr")
                .field("nc", &self.nc())
                .field("nr", &self.nr())
                .field("mwid", &self.mwid())
                .field("nb", &self.nb())
                .field("cas", &self.cas())
                .field("wp", &self.wp())
                .field("sdclk", &self.sdclk())
                .field("rburst", &self.rburst())
                .field("rpipe", &self.rpipe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdcr {{ nc: {:?}, nr: {:?}, mwid: {:?}, nb: {:?}, cas: {:?}, wp: {=bool:?}, sdclk: {:?}, rburst: {=bool:?}, rpipe: {:?} }}" , self . nc () , self . nr () , self . mwid () , self . nb () , self . cas () , self . wp () , self . sdclk () , self . rburst () , self . rpipe ())
        }
    }
    #[doc = "SDRAM refresh timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrtr(pub u32);
    impl Sdrtr {
        #[doc = "Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register."]
        #[inline(always)]
        pub const fn cre(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register."]
        #[inline(always)]
        pub fn set_cre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20."]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x1fff;
            val as u16
        }
        #[doc = "Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20."]
        #[inline(always)]
        pub fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
        }
        #[doc = "RES Interrupt Enable."]
        #[inline(always)]
        pub const fn reie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RES Interrupt Enable."]
        #[inline(always)]
        pub fn set_reie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sdrtr {
        #[inline(always)]
        fn default() -> Sdrtr {
            Sdrtr(0)
        }
    }
    impl core::fmt::Debug for Sdrtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrtr")
                .field("cre", &self.cre())
                .field("count", &self.count())
                .field("reie", &self.reie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdrtr {{ cre: {=bool:?}, count: {=u16:?}, reie: {=bool:?} }}",
                self.cre(),
                self.count(),
                self.reie()
            )
        }
    }
    #[doc = "SDRAM status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdsr(pub u32);
    impl Sdsr {
        #[doc = "Refresh error flag An interrupt is generated if REIE = 1 and RE = 1."]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh error flag An interrupt is generated if REIE = 1 and RE = 1."]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1."]
        #[inline(always)]
        pub const fn modes(&self, n: usize) -> super::vals::Modes {
            assert!(n < 2usize);
            let offs = 1usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Modes::from_bits(val as u8)
        }
        #[doc = "Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1."]
        #[inline(always)]
        pub fn set_modes(&mut self, n: usize, val: super::vals::Modes) {
            assert!(n < 2usize);
            let offs = 1usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sdsr {
        #[inline(always)]
        fn default() -> Sdsr {
            Sdsr(0)
        }
    }
    impl core::fmt::Debug for Sdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdsr")
                .field("re", &self.re())
                .field("modes[0]", &self.modes(0usize))
                .field("modes[1]", &self.modes(1usize))
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdsr {{ re: {=bool:?}, modes[0]: {:?}, modes[1]: {:?}, busy: {=bool:?} }}",
                self.re(),
                self.modes(0usize),
                self.modes(1usize),
                self.busy()
            )
        }
    }
    #[doc = "SDRAM timing registers 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdtr(pub u32);
    impl Sdtr {
        #[doc = "Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
        #[inline(always)]
        pub const fn tmrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
        #[inline(always)]
        pub fn set_tmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
        #[inline(always)]
        pub const fn txsr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
        #[inline(always)]
        pub fn set_txsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
        #[inline(always)]
        pub const fn tras(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
        #[inline(always)]
        pub fn set_tras(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care."]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are don’t care."]
        #[inline(always)]
        pub fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t<sub>WR</sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank."]
        #[inline(always)]
        pub const fn twr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (t<sub>WR</sub>) defined in the SDRAM datasheet, and to guarantee that: Note: TWR ≥ TRAS - TRCD and TWR ≥TRC - TRCD - TRP Note: Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1. Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device. Note: If only one SDRAM device is used, the TWR timing must be kept at reset value (0xF) for the not used bank."]
        #[inline(always)]
        pub fn set_twr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care."]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are don’t care."]
        #[inline(always)]
        pub fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
        #[inline(always)]
        pub const fn trcd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
        #[inline(always)]
        pub fn set_trcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Sdtr {
        #[inline(always)]
        fn default() -> Sdtr {
            Sdtr(0)
        }
    }
    impl core::fmt::Debug for Sdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdtr")
                .field("tmrd", &self.tmrd())
                .field("txsr", &self.txsr())
                .field("tras", &self.tras())
                .field("trc", &self.trc())
                .field("twr", &self.twr())
                .field("trp", &self.trp())
                .field("trcd", &self.trcd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdtr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdtr {{ tmrd: {=u8:?}, txsr: {=u8:?}, tras: {=u8:?}, trc: {=u8:?}, twr: {=u8:?}, trp: {=u8:?}, trcd: {=u8:?} }}" , self . tmrd () , self . txsr () , self . tras () , self . trc () , self . twr () , self . trp () , self . trcd ())
        }
    }
    #[doc = "FIFO status and interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
        #[inline(always)]
        pub fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt high-level status The flag is set by hardware and reset by software."]
        #[inline(always)]
        pub const fn ils(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level status The flag is set by hardware and reset by software."]
        #[inline(always)]
        pub fn set_ils(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
        #[inline(always)]
        pub const fn ifs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
        #[inline(always)]
        pub fn set_ifs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt rising edge detection enable bit."]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge detection enable bit."]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt high-level detection enable bit."]
        #[inline(always)]
        pub const fn ilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level detection enable bit."]
        #[inline(always)]
        pub fn set_ilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt falling edge detection enable bit."]
        #[inline(always)]
        pub const fn ifen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge detection enable bit."]
        #[inline(always)]
        pub fn set_ifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO empty Read-only bit that provides the status of the FIFO."]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty Read-only bit that provides the status of the FIFO."]
        #[inline(always)]
        pub fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("irs", &self.irs())
                .field("ils", &self.ils())
                .field("ifs", &self.ifs())
                .field("iren", &self.iren())
                .field("ilen", &self.ilen())
                .field("ifen", &self.ifen())
                .field("fempt", &self.fempt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ irs: {=bool:?}, ils: {=bool:?}, ifs: {=bool:?}, iren: {=bool:?}, ilen: {=bool:?}, ifen: {=bool:?}, fempt: {=bool:?} }}" , self . irs () , self . ils () , self . ifs () , self . iren () , self . ilen () , self . ifen () , self . fempt ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Accmod {
        #[doc = "Access mode A"]
        A = 0x0,
        #[doc = "Access mode B"]
        B = 0x01,
        #[doc = "Access mode C"]
        C = 0x02,
        #[doc = "Access mode D"]
        D = 0x03,
    }
    impl Accmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Accmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Accmod {
        #[inline(always)]
        fn from(val: u8) -> Accmod {
            Accmod::from_bits(val)
        }
    }
    impl From<Accmod> for u8 {
        #[inline(always)]
        fn from(val: Accmod) -> u8 {
            Accmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cas {
        _RESERVED_0 = 0x0,
        #[doc = "1 cycle"]
        CLOCKS1 = 0x01,
        #[doc = "2 cycles"]
        CLOCKS2 = 0x02,
        #[doc = "3 cycles"]
        CLOCKS3 = 0x03,
    }
    impl Cas {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cas {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cas {
        #[inline(always)]
        fn from(val: u8) -> Cas {
            Cas::from_bits(val)
        }
    }
    impl From<Cas> for u8 {
        #[inline(always)]
        fn from(val: Cas) -> u8 {
            Cas::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cburstrw {
        #[doc = "Write operations are always performed in Asynchronous mode."]
        ASYNCHRONOUS = 0x0,
        #[doc = "Write operations are performed in Synchronous mode."]
        SYNCHRONOUS = 0x01,
    }
    impl Cburstrw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cburstrw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cburstrw {
        #[inline(always)]
        fn from(val: u8) -> Cburstrw {
            Cburstrw::from_bits(val)
        }
    }
    impl From<Cburstrw> for u8 {
        #[inline(always)]
        fn from(val: Cburstrw) -> u8 {
            Cburstrw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpsize {
        #[doc = "No burst split when crossing page boundary"]
        NO_BURST_SPLIT = 0x0,
        #[doc = "128 bytes CRAM page size"]
        BYTES128 = 0x01,
        #[doc = "256 bytes CRAM page size"]
        BYTES256 = 0x02,
        #[doc = "512 bytes CRAM page size"]
        BYTES512 = 0x03,
        #[doc = "1024 bytes CRAM page size"]
        BYTES1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cpsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpsize {
        #[inline(always)]
        fn from(val: u8) -> Cpsize {
            Cpsize::from_bits(val)
        }
    }
    impl From<Cpsize> for u8 {
        #[inline(always)]
        fn from(val: Cpsize) -> u8 {
            Cpsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eccps {
        #[doc = "ECC page size 256 bytes"]
        BYTES256 = 0x0,
        #[doc = "ECC page size 512 bytes"]
        BYTES512 = 0x01,
        #[doc = "ECC page size 1024 bytes"]
        BYTES1024 = 0x02,
        #[doc = "ECC page size 2048 bytes"]
        BYTES2048 = 0x03,
        #[doc = "ECC page size 4096 bytes"]
        BYTES4096 = 0x04,
        #[doc = "ECC page size 8192 bytes"]
        BYTES8192 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Eccps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccps {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccps {
        #[inline(always)]
        fn from(val: u8) -> Eccps {
            Eccps::from_bits(val)
        }
    }
    impl From<Eccps> for u8 {
        #[inline(always)]
        fn from(val: Eccps) -> u8 {
            Eccps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Clock Configuration Enable"]
        CLOCK_CONFIGURATION_ENABLE = 0x01,
        #[doc = "PALL (All Bank Precharge) command"]
        PALL = 0x02,
        #[doc = "Auto-refresh command"]
        AUTO_REFRESH_COMMAND = 0x03,
        #[doc = "Load Mode Resgier"]
        LOAD_MODE_REGISTER = 0x04,
        #[doc = "Self-refresh command"]
        SELF_REFRESH_COMMAND = 0x05,
        #[doc = "Power-down command"]
        POWER_DOWN_COMMAND = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Modes {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Self-refresh mode"]
        SELF_REFRESH = 0x01,
        #[doc = "Power-down mode"]
        POWER_DOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Modes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Modes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Modes {
        #[inline(always)]
        fn from(val: u8) -> Modes {
            Modes::from_bits(val)
        }
    }
    impl From<Modes> for u8 {
        #[inline(always)]
        fn from(val: Modes) -> u8 {
            Modes::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mtyp {
        #[doc = "SRAM memory type"]
        SRAM = 0x0,
        #[doc = "PSRAM (CRAM) memory type"]
        PSRAM = 0x01,
        #[doc = "NOR Flash/OneNAND Flash"]
        FLASH = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mtyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtyp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtyp {
        #[inline(always)]
        fn from(val: u8) -> Mtyp {
            Mtyp::from_bits(val)
        }
    }
    impl From<Mtyp> for u8 {
        #[inline(always)]
        fn from(val: Mtyp) -> u8 {
            Mtyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mwid {
        #[doc = "Memory data bus width 8 bits"]
        BITS8 = 0x0,
        #[doc = "Memory data bus width 16 bits"]
        BITS16 = 0x01,
        #[doc = "Memory data bus width 32 bits"]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mwid {
        #[inline(always)]
        fn from(val: u8) -> Mwid {
            Mwid::from_bits(val)
        }
    }
    impl From<Mwid> for u8 {
        #[inline(always)]
        fn from(val: Mwid) -> u8 {
            Mwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nb {
        #[doc = "Two internal Banks"]
        NB2 = 0x0,
        #[doc = "Four internal Banks"]
        NB4 = 0x01,
    }
    impl Nb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nb {
        #[inline(always)]
        fn from(val: u8) -> Nb {
            Nb::from_bits(val)
        }
    }
    impl From<Nb> for u8 {
        #[inline(always)]
        fn from(val: Nb) -> u8 {
            Nb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nc {
        #[doc = "8 bits"]
        BITS8 = 0x0,
        #[doc = "9 bits"]
        BITS9 = 0x01,
        #[doc = "10 bits"]
        BITS10 = 0x02,
        #[doc = "11 bits"]
        BITS11 = 0x03,
    }
    impl Nc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nc {
        #[inline(always)]
        fn from(val: u8) -> Nc {
            Nc::from_bits(val)
        }
    }
    impl From<Nc> for u8 {
        #[inline(always)]
        fn from(val: Nc) -> u8 {
            Nc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nr {
        #[doc = "11 bits"]
        BITS11 = 0x0,
        #[doc = "12 bits"]
        BITS12 = 0x01,
        #[doc = "13 bits"]
        BITS13 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Nr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nr {
        #[inline(always)]
        fn from(val: u8) -> Nr {
            Nr::from_bits(val)
        }
    }
    impl From<Nr> for u8 {
        #[inline(always)]
        fn from(val: Nr) -> u8 {
            Nr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ptyp {
        _RESERVED_0 = 0x0,
        #[doc = "NAND flash"]
        NAND = 0x01,
    }
    impl Ptyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ptyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ptyp {
        #[inline(always)]
        fn from(val: u8) -> Ptyp {
            Ptyp::from_bits(val)
        }
    }
    impl From<Ptyp> for u8 {
        #[inline(always)]
        fn from(val: Ptyp) -> u8 {
            Ptyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwid {
        #[doc = "External memory device width 8 bits"]
        BITS8 = 0x0,
        #[doc = "External memory device width 16 bits"]
        BITS16 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwid {
        #[inline(always)]
        fn from(val: u8) -> Pwid {
            Pwid::from_bits(val)
        }
    }
    impl From<Pwid> for u8 {
        #[inline(always)]
        fn from(val: Pwid) -> u8 {
            Pwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rpipe {
        #[doc = "No clock cycle delay"]
        NO_DELAY = 0x0,
        #[doc = "One clock cycle delay"]
        CLOCKS1 = 0x01,
        #[doc = "Two clock cycles delay"]
        CLOCKS2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Rpipe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rpipe {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rpipe {
        #[inline(always)]
        fn from(val: u8) -> Rpipe {
            Rpipe::from_bits(val)
        }
    }
    impl From<Rpipe> for u8 {
        #[inline(always)]
        fn from(val: Rpipe) -> u8 {
            Rpipe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdclk {
        #[doc = "SDCLK clock disabled"]
        DISABLED = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "SDCLK period = 2 x HCLK period"]
        DIV2 = 0x02,
        #[doc = "SDCLK period = 3 x HCLK period"]
        DIV3 = 0x03,
    }
    impl Sdclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdclk {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdclk {
        #[inline(always)]
        fn from(val: u8) -> Sdclk {
            Sdclk::from_bits(val)
        }
    }
    impl From<Sdclk> for u8 {
        #[inline(always)]
        fn from(val: Sdclk) -> u8 {
            Sdclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitcfg {
        #[doc = "NWAIT signal is active one data cycle before wait state"]
        BEFORE_WAIT_STATE = 0x0,
        #[doc = "NWAIT signal is active during wait state"]
        DURING_WAIT_STATE = 0x01,
    }
    impl Waitcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitcfg {
        #[inline(always)]
        fn from(val: u8) -> Waitcfg {
            Waitcfg::from_bits(val)
        }
    }
    impl From<Waitcfg> for u8 {
        #[inline(always)]
        fn from(val: Waitcfg) -> u8 {
            Waitcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitpol {
        #[doc = "NWAIT active low"]
        ACTIVE_LOW = 0x0,
        #[doc = "NWAIT active high"]
        ACTIVE_HIGH = 0x01,
    }
    impl Waitpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitpol {
        #[inline(always)]
        fn from(val: u8) -> Waitpol {
            Waitpol::from_bits(val)
        }
    }
    impl From<Waitpol> for u8 {
        #[inline(always)]
        fn from(val: Waitpol) -> u8 {
            Waitpol::to_bits(val)
        }
    }
}
