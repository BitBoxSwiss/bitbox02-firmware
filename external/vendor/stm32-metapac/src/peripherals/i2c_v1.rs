#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Inter-integrated circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Own address register 1"]
    #[inline(always)]
    pub const fn oar1(self) -> crate::common::Reg<regs::Oar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Own address register 2"]
    #[inline(always)]
    pub const fn oar2(self) -> crate::common::Reg<regs::Oar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TRISE register"]
    #[inline(always)]
    pub const fn trise(self) -> crate::common::Reg<regs::Trise, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLTR register"]
    #[inline(always)]
    pub const fn fltr(self) -> crate::common::Reg<regs::Fltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Clock control register in Fast/Standard mode (Master mode)"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Clock control register in Fast/Standard mode (Master mode)"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Fast mode duty cycle"]
        #[inline(always)]
        pub const fn duty(&self) -> super::vals::Duty {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Duty::from_bits(val as u8)
        }
        #[doc = "Fast mode duty cycle"]
        #[inline(always)]
        pub fn set_duty(&mut self, val: super::vals::Duty) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C master mode selection"]
        #[inline(always)]
        pub const fn f_s(&self) -> super::vals::FS {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::FS::from_bits(val as u8)
        }
        #[doc = "I2C master mode selection"]
        #[inline(always)]
        pub fn set_f_s(&mut self, val: super::vals::FS) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("ccr", &self.ccr())
                .field("duty", &self.duty())
                .field("f_s", &self.f_s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ ccr: {=u16:?}, duty: {:?}, f_s: {:?} }}",
                self.ccr(),
                self.duty(),
                self.f_s()
            )
        }
    }
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral enable"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SMBus mode"]
        #[inline(always)]
        pub const fn smbus(&self) -> super::vals::Smbus {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Smbus::from_bits(val as u8)
        }
        #[doc = "SMBus mode"]
        #[inline(always)]
        pub fn set_smbus(&mut self, val: super::vals::Smbus) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "SMBus type"]
        #[inline(always)]
        pub const fn smbtype(&self) -> super::vals::Smbtype {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Smbtype::from_bits(val as u8)
        }
        #[doc = "SMBus type"]
        #[inline(always)]
        pub fn set_smbtype(&mut self, val: super::vals::Smbtype) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "ARP enable"]
        #[inline(always)]
        pub const fn enarp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ARP enable"]
        #[inline(always)]
        pub fn set_enarp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PEC enable"]
        #[inline(always)]
        pub const fn enpec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PEC enable"]
        #[inline(always)]
        pub fn set_enpec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "General call enable"]
        #[inline(always)]
        pub const fn engc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "General call enable"]
        #[inline(always)]
        pub fn set_engc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clock stretching disable (Slave mode)"]
        #[inline(always)]
        pub const fn nostretch(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock stretching disable (Slave mode)"]
        #[inline(always)]
        pub fn set_nostretch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Start generation"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Start generation"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Stop generation"]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Stop generation"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge enable"]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge enable"]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge/PEC Position (for data reception)"]
        #[inline(always)]
        pub const fn pos(&self) -> super::vals::Pos {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Pos::from_bits(val as u8)
        }
        #[doc = "Acknowledge/PEC Position (for data reception)"]
        #[inline(always)]
        pub fn set_pos(&mut self, val: super::vals::Pos) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Packet error checking"]
        #[inline(always)]
        pub const fn pec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Packet error checking"]
        #[inline(always)]
        pub fn set_pec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Software reset"]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset"]
        #[inline(always)]
        pub fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pe", &self.pe())
                .field("smbus", &self.smbus())
                .field("smbtype", &self.smbtype())
                .field("enarp", &self.enarp())
                .field("enpec", &self.enpec())
                .field("engc", &self.engc())
                .field("nostretch", &self.nostretch())
                .field("start", &self.start())
                .field("stop", &self.stop())
                .field("ack", &self.ack())
                .field("pos", &self.pos())
                .field("pec", &self.pec())
                .field("alert", &self.alert())
                .field("swrst", &self.swrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ pe: {=bool:?}, smbus: {:?}, smbtype: {:?}, enarp: {=bool:?}, enpec: {=bool:?}, engc: {=bool:?}, nostretch: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, ack: {=bool:?}, pos: {:?}, pec: {=bool:?}, alert: {=bool:?}, swrst: {=bool:?} }}" , self . pe () , self . smbus () , self . smbtype () , self . enarp () , self . enpec () , self . engc () , self . nostretch () , self . start () , self . stop () , self . ack () , self . pos () , self . pec () , self . alert () , self . swrst ())
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Peripheral clock frequency"]
        #[inline(always)]
        pub const fn freq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Peripheral clock frequency"]
        #[inline(always)]
        pub fn set_freq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn iterren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_iterren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Event interrupt enable"]
        #[inline(always)]
        pub const fn itevten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Event interrupt enable"]
        #[inline(always)]
        pub fn set_itevten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Buffer interrupt enable"]
        #[inline(always)]
        pub const fn itbufen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer interrupt enable"]
        #[inline(always)]
        pub fn set_itbufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DMA requests enable"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA requests enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA last transfer"]
        #[inline(always)]
        pub const fn last(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA last transfer"]
        #[inline(always)]
        pub fn set_last(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("freq", &self.freq())
                .field("iterren", &self.iterren())
                .field("itevten", &self.itevten())
                .field("itbufen", &self.itbufen())
                .field("dmaen", &self.dmaen())
                .field("last", &self.last())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ freq: {=u8:?}, iterren: {=bool:?}, itevten: {=bool:?}, itbufen: {=bool:?}, dmaen: {=bool:?}, last: {=bool:?} }}" , self . freq () , self . iterren () , self . itevten () , self . itbufen () , self . dmaen () , self . last ())
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "8-bit data register"]
        #[inline(always)]
        pub const fn dr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit data register"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ dr: {=u8:?} }}", self.dr())
        }
    }
    #[doc = "FLTR register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltr(pub u32);
    impl Fltr {
        #[doc = "Digital noise filter"]
        #[inline(always)]
        pub const fn dnf(&self) -> super::vals::Dnf {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Dnf::from_bits(val as u8)
        }
        #[doc = "Digital noise filter"]
        #[inline(always)]
        pub fn set_dnf(&mut self, val: super::vals::Dnf) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Analog noise filter"]
        #[inline(always)]
        pub const fn anoff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog noise filter"]
        #[inline(always)]
        pub fn set_anoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Fltr {
        #[inline(always)]
        fn default() -> Fltr {
            Fltr(0)
        }
    }
    impl core::fmt::Debug for Fltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltr")
                .field("dnf", &self.dnf())
                .field("anoff", &self.anoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fltr {{ dnf: {:?}, anoff: {=bool:?} }}", self.dnf(), self.anoff())
        }
    }
    #[doc = "Own address register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oar1(pub u32);
    impl Oar1 {
        #[doc = "Interface address"]
        #[inline(always)]
        pub const fn add(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Interface address"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Addressing mode (slave mode)"]
        #[inline(always)]
        pub const fn addmode(&self) -> super::vals::Addmode {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Addmode::from_bits(val as u8)
        }
        #[doc = "Addressing mode (slave mode)"]
        #[inline(always)]
        pub fn set_addmode(&mut self, val: super::vals::Addmode) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Oar1 {
        #[inline(always)]
        fn default() -> Oar1 {
            Oar1(0)
        }
    }
    impl core::fmt::Debug for Oar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oar1")
                .field("add", &self.add())
                .field("addmode", &self.addmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oar1 {{ add: {=u16:?}, addmode: {:?} }}", self.add(), self.addmode())
        }
    }
    #[doc = "Own address register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oar2(pub u32);
    impl Oar2 {
        #[doc = "Dual addressing mode enable"]
        #[inline(always)]
        pub const fn endual(&self) -> super::vals::Endual {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Endual::from_bits(val as u8)
        }
        #[doc = "Dual addressing mode enable"]
        #[inline(always)]
        pub fn set_endual(&mut self, val: super::vals::Endual) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Interface address"]
        #[inline(always)]
        pub const fn add2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Interface address"]
        #[inline(always)]
        pub fn set_add2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Oar2 {
        #[inline(always)]
        fn default() -> Oar2 {
            Oar2(0)
        }
    }
    impl core::fmt::Debug for Oar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oar2")
                .field("endual", &self.endual())
                .field("add2", &self.add2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oar2 {{ endual: {:?}, add2: {=u8:?} }}", self.endual(), self.add2())
        }
    }
    #[doc = "Status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Start bit (Master mode)"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit (Master mode)"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address sent (master mode)/matched (slave mode)"]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address sent (master mode)/matched (slave mode)"]
        #[inline(always)]
        pub fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Byte transfer finished"]
        #[inline(always)]
        pub const fn btf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Byte transfer finished"]
        #[inline(always)]
        pub fn set_btf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "10-bit header sent (Master mode)"]
        #[inline(always)]
        pub const fn add10(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "10-bit header sent (Master mode)"]
        #[inline(always)]
        pub fn set_add10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Stop detection (slave mode)"]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Stop detection (slave mode)"]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data register not empty (receivers)"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data register not empty (receivers)"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data register empty (transmitters)"]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data register empty (transmitters)"]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bus error"]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bus error"]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Arbitration lost (master mode)"]
        #[inline(always)]
        pub const fn arlo(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost (master mode)"]
        #[inline(always)]
        pub fn set_arlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge failure"]
        #[inline(always)]
        pub const fn af(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge failure"]
        #[inline(always)]
        pub fn set_af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Overrun/Underrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/Underrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PEC Error in reception"]
        #[inline(always)]
        pub const fn pecerr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PEC Error in reception"]
        #[inline(always)]
        pub fn set_pecerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timeout or t_low detection flag"]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout or t_low detection flag"]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus alert"]
        #[inline(always)]
        pub fn set_alert(&mut self, val: bool) {
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
                .field("start", &self.start())
                .field("addr", &self.addr())
                .field("btf", &self.btf())
                .field("add10", &self.add10())
                .field("stopf", &self.stopf())
                .field("rxne", &self.rxne())
                .field("txe", &self.txe())
                .field("berr", &self.berr())
                .field("arlo", &self.arlo())
                .field("af", &self.af())
                .field("ovr", &self.ovr())
                .field("pecerr", &self.pecerr())
                .field("timeout", &self.timeout())
                .field("alert", &self.alert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ start: {=bool:?}, addr: {=bool:?}, btf: {=bool:?}, add10: {=bool:?}, stopf: {=bool:?}, rxne: {=bool:?}, txe: {=bool:?}, berr: {=bool:?}, arlo: {=bool:?}, af: {=bool:?}, ovr: {=bool:?}, pecerr: {=bool:?}, timeout: {=bool:?}, alert: {=bool:?} }}" , self . start () , self . addr () , self . btf () , self . add10 () , self . stopf () , self . rxne () , self . txe () , self . berr () , self . arlo () , self . af () , self . ovr () , self . pecerr () , self . timeout () , self . alert ())
        }
    }
    #[doc = "Status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Master/slave"]
        #[inline(always)]
        pub const fn msl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master/slave"]
        #[inline(always)]
        pub fn set_msl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bus busy"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bus busy"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter/receiver"]
        #[inline(always)]
        pub const fn tra(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter/receiver"]
        #[inline(always)]
        pub fn set_tra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "General call address (Slave mode)"]
        #[inline(always)]
        pub const fn gencall(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "General call address (Slave mode)"]
        #[inline(always)]
        pub fn set_gencall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SMBus device default address (Slave mode)"]
        #[inline(always)]
        pub const fn smbdefault(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus device default address (Slave mode)"]
        #[inline(always)]
        pub fn set_smbdefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SMBus host header (Slave mode)"]
        #[inline(always)]
        pub const fn smbhost(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus host header (Slave mode)"]
        #[inline(always)]
        pub fn set_smbhost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Dual flag (Slave mode)"]
        #[inline(always)]
        pub const fn dualf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Dual flag (Slave mode)"]
        #[inline(always)]
        pub fn set_dualf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Packet error checking register"]
        #[inline(always)]
        pub const fn pec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Packet error checking register"]
        #[inline(always)]
        pub fn set_pec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                .field("msl", &self.msl())
                .field("busy", &self.busy())
                .field("tra", &self.tra())
                .field("gencall", &self.gencall())
                .field("smbdefault", &self.smbdefault())
                .field("smbhost", &self.smbhost())
                .field("dualf", &self.dualf())
                .field("pec", &self.pec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr2 {{ msl: {=bool:?}, busy: {=bool:?}, tra: {=bool:?}, gencall: {=bool:?}, smbdefault: {=bool:?}, smbhost: {=bool:?}, dualf: {=bool:?}, pec: {=u8:?} }}" , self . msl () , self . busy () , self . tra () , self . gencall () , self . smbdefault () , self . smbhost () , self . dualf () , self . pec ())
        }
    }
    #[doc = "TRISE register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trise(pub u32);
    impl Trise {
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)"]
        #[inline(always)]
        pub const fn trise(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)"]
        #[inline(always)]
        pub fn set_trise(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Trise {
        #[inline(always)]
        fn default() -> Trise {
            Trise(0)
        }
    }
    impl core::fmt::Debug for Trise {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trise").field("trise", &self.trise()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trise {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Trise {{ trise: {=u8:?} }}", self.trise())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Addmode {
        #[doc = "7-bit addressing mode"]
        BIT7 = 0x0,
        #[doc = "10-bit addressing mode"]
        BIT10 = 0x01,
    }
    impl Addmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Addmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Addmode {
        #[inline(always)]
        fn from(val: u8) -> Addmode {
            Addmode::from_bits(val)
        }
    }
    impl From<Addmode> for u8 {
        #[inline(always)]
        fn from(val: Addmode) -> u8 {
            Addmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dnf {
        #[doc = "Digital filter disabled"]
        NO_FILTER = 0x0,
        #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
        FILTER1 = 0x01,
        #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
        FILTER2 = 0x02,
        #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
        FILTER3 = 0x03,
        #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
        FILTER4 = 0x04,
        #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
        FILTER5 = 0x05,
        #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
        FILTER6 = 0x06,
        #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
        FILTER7 = 0x07,
        #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
        FILTER8 = 0x08,
        #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
        FILTER9 = 0x09,
        #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
        FILTER10 = 0x0a,
        #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
        FILTER11 = 0x0b,
        #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
        FILTER12 = 0x0c,
        #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
        FILTER13 = 0x0d,
        #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
        FILTER14 = 0x0e,
        #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
        FILTER15 = 0x0f,
    }
    impl Dnf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dnf {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dnf {
        #[inline(always)]
        fn from(val: u8) -> Dnf {
            Dnf::from_bits(val)
        }
    }
    impl From<Dnf> for u8 {
        #[inline(always)]
        fn from(val: Dnf) -> u8 {
            Dnf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Duty {
        #[doc = "Duty cycle t_low/t_high = 2/1"]
        DUTY2_1 = 0x0,
        #[doc = "Duty cycle t_low/t_high = 16/9"]
        DUTY16_9 = 0x01,
    }
    impl Duty {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Duty {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Duty {
        #[inline(always)]
        fn from(val: u8) -> Duty {
            Duty::from_bits(val)
        }
    }
    impl From<Duty> for u8 {
        #[inline(always)]
        fn from(val: Duty) -> u8 {
            Duty::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Endual {
        #[doc = "Single addressing mode"]
        SINGLE = 0x0,
        #[doc = "Dual addressing mode"]
        DUAL = 0x01,
    }
    impl Endual {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Endual {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Endual {
        #[inline(always)]
        fn from(val: u8) -> Endual {
            Endual::from_bits(val)
        }
    }
    impl From<Endual> for u8 {
        #[inline(always)]
        fn from(val: Endual) -> u8 {
            Endual::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FS {
        #[doc = "Standard mode I2C"]
        STANDARD = 0x0,
        #[doc = "Fast mode I2C"]
        FAST = 0x01,
    }
    impl FS {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FS {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FS {
        #[inline(always)]
        fn from(val: u8) -> FS {
            FS::from_bits(val)
        }
    }
    impl From<FS> for u8 {
        #[inline(always)]
        fn from(val: FS) -> u8 {
            FS::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pos {
        #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
        CURRENT = 0x0,
        #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
        NEXT = 0x01,
    }
    impl Pos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pos {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pos {
        #[inline(always)]
        fn from(val: u8) -> Pos {
            Pos::from_bits(val)
        }
    }
    impl From<Pos> for u8 {
        #[inline(always)]
        fn from(val: Pos) -> u8 {
            Pos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smbtype {
        #[doc = "SMBus Device"]
        DEVICE = 0x0,
        #[doc = "SMBus Host"]
        HOST = 0x01,
    }
    impl Smbtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smbtype {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smbtype {
        #[inline(always)]
        fn from(val: u8) -> Smbtype {
            Smbtype::from_bits(val)
        }
    }
    impl From<Smbtype> for u8 {
        #[inline(always)]
        fn from(val: Smbtype) -> u8 {
            Smbtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smbus {
        #[doc = "I2C Mode"]
        I2C = 0x0,
        #[doc = "SMBus"]
        SMBUS = 0x01,
    }
    impl Smbus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smbus {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smbus {
        #[inline(always)]
        fn from(val: u8) -> Smbus {
            Smbus::from_bits(val)
        }
    }
    impl From<Smbus> for u8 {
        #[inline(always)]
        fn from(val: Smbus) -> u8 {
            Smbus::to_bits(val)
        }
    }
}
