#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "4kb addressable space."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "COMMAND register."]
    #[inline(always)]
    pub const fn command(self) -> crate::common::Reg<regs::Command, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CONFIG register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "IRQSTAT register."]
    #[inline(always)]
    pub const fn irqstat(self) -> crate::common::Reg<regs::Irqstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "IRQMASK register."]
    #[inline(always)]
    pub const fn irqmask(self) -> crate::common::Reg<regs::Irqmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "IRQRAW register."]
    #[inline(always)]
    pub const fn irqraw(self) -> crate::common::Reg<regs::Irqraw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SIZE register."]
    #[inline(always)]
    pub const fn size(self) -> crate::common::Reg<regs::Size, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ADDRESS register."]
    #[inline(always)]
    pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "LFSRVAL register."]
    #[inline(always)]
    pub const fn lfsrval(self) -> crate::common::Reg<regs::Lfsrval, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PAGEPROT0 register."]
    #[inline(always)]
    pub const fn pageprot0(self) -> crate::common::Reg<regs::Pageprot0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PAGEPROT1 register."]
    #[inline(always)]
    pub const fn pageprot1(self) -> crate::common::Reg<regs::Pageprot1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DATA0 register."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<regs::Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DATA1 register."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<regs::Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DATA2 register."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<regs::Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DATA3 register."]
    #[inline(always)]
    pub const fn data3(self) -> crate::common::Reg<regs::Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADDRESS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Address(pub u32);
    impl Address {
        #[doc = "Flash column address offset to be used with some COMMAND."]
        #[inline(always)]
        pub const fn yaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Flash column address offset to be used with some COMMAND."]
        #[inline(always)]
        pub fn set_yaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Flash row address offset to be used with some COMMAND."]
        #[inline(always)]
        pub const fn xaddr(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x03ff;
            val as u16
        }
        #[doc = "Flash row address offset to be used with some COMMAND."]
        #[inline(always)]
        pub fn set_xaddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
        }
    }
    impl Default for Address {
        #[inline(always)]
        fn default() -> Address {
            Address(0)
        }
    }
    impl core::fmt::Debug for Address {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Address")
                .field("yaddr", &self.yaddr())
                .field("xaddr", &self.xaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Address {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Address {{ yaddr: {=u8:?}, xaddr: {=u16:?} }}",
                self.yaddr(),
                self.xaddr()
            )
        }
    }
    #[doc = "COMMAND register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Command(pub u32);
    impl Command {
        #[doc = "Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE."]
        #[inline(always)]
        pub const fn command(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE."]
        #[inline(always)]
        pub fn set_command(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Command {
        #[inline(always)]
        fn default() -> Command {
            Command(0)
        }
    }
    impl core::fmt::Debug for Command {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Command").field("command", &self.command()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Command {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Command {{ command: {=u8:?} }}", self.command())
        }
    }
    #[doc = "CONFIG register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "Bit to redirect boot area on SRAM0."]
        #[inline(always)]
        pub const fn remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bit to redirect boot area on SRAM0."]
        #[inline(always)]
        pub fn set_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden."]
        #[inline(always)]
        pub const fn dis_group_write(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden."]
        #[inline(always)]
        pub fn set_dis_group_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Number of wait states to be inserted on Flash read (AHB accesses)."]
        #[inline(always)]
        pub const fn wait_states(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Number of wait states to be inserted on Flash read (AHB accesses)."]
        #[inline(always)]
        pub fn set_wait_states(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("remap", &self.remap())
                .field("dis_group_write", &self.dis_group_write())
                .field("wait_states", &self.wait_states())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Config {{ remap: {=bool:?}, dis_group_write: {=bool:?}, wait_states: {=u8:?} }}",
                self.remap(),
                self.dis_group_write(),
                self.wait_states()
            )
        }
    }
    #[doc = "DATA0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data0(pub u32);
    impl Data0 {
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD."]
        #[inline(always)]
        pub const fn data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data0 {
        #[inline(always)]
        fn default() -> Data0 {
            Data0(0)
        }
    }
    impl core::fmt::Debug for Data0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data0").field("data0", &self.data0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data0 {{ data0: {=u32:?} }}", self.data0())
        }
    }
    #[doc = "DATA1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data1(pub u32);
    impl Data1 {
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub const fn data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data1 {
        #[inline(always)]
        fn default() -> Data1 {
            Data1(0)
        }
    }
    impl core::fmt::Debug for Data1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data1").field("data1", &self.data1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data1 {{ data1: {=u32:?} }}", self.data1())
        }
    }
    #[doc = "DATA2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data2(pub u32);
    impl Data2 {
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub const fn data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data2 {
        #[inline(always)]
        fn default() -> Data2 {
            Data2(0)
        }
    }
    impl core::fmt::Debug for Data2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data2").field("data2", &self.data2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data2 {{ data2: {=u32:?} }}", self.data2())
        }
    }
    #[doc = "DATA3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data3(pub u32);
    impl Data3 {
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub const fn data3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Value to be used as DATA for any COMMAND of type WRITE."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data3 {
        #[inline(always)]
        fn default() -> Data3 {
            Data3(0)
        }
    }
    impl core::fmt::Debug for Data3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data3").field("data3", &self.data3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data3 {{ data3: {=u32:?} }}", self.data3())
        }
    }
    #[doc = "IRQMASK register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqmask(pub u32);
    impl Irqmask {
        #[doc = "Command done mask."]
        #[inline(always)]
        pub const fn cmddonem(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command done mask."]
        #[inline(always)]
        pub fn set_cmddonem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command started mask."]
        #[inline(always)]
        pub const fn cmdstartm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command started mask."]
        #[inline(always)]
        pub fn set_cmdstartm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command error mask."]
        #[inline(always)]
        pub const fn cmderrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command error mask."]
        #[inline(always)]
        pub fn set_cmderrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Illegal command mask."]
        #[inline(always)]
        pub const fn illcmdm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal command mask."]
        #[inline(always)]
        pub fn set_illcmdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Mass read OK mask."]
        #[inline(always)]
        pub const fn readokm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Mass read OK mask."]
        #[inline(always)]
        pub fn set_readokm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "(1: mask, 0: inactive) FNREADY_MIS mask."]
        #[inline(always)]
        pub const fn fnreadym(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "(1: mask, 0: inactive) FNREADY_MIS mask."]
        #[inline(always)]
        pub fn set_fnreadym(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Irqmask {
        #[inline(always)]
        fn default() -> Irqmask {
            Irqmask(0)
        }
    }
    impl core::fmt::Debug for Irqmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irqmask")
                .field("cmddonem", &self.cmddonem())
                .field("cmdstartm", &self.cmdstartm())
                .field("cmderrm", &self.cmderrm())
                .field("illcmdm", &self.illcmdm())
                .field("readokm", &self.readokm())
                .field("fnreadym", &self.fnreadym())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irqmask {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Irqmask {{ cmddonem: {=bool:?}, cmdstartm: {=bool:?}, cmderrm: {=bool:?}, illcmdm: {=bool:?}, readokm: {=bool:?}, fnreadym: {=bool:?} }}" , self . cmddonem () , self . cmdstartm () , self . cmderrm () , self . illcmdm () , self . readokm () , self . fnreadym ())
        }
    }
    #[doc = "IRQRAW register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqraw(pub u32);
    impl Irqraw {
        #[doc = "Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1."]
        #[inline(always)]
        pub const fn cmddone_ris(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_cmddone_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started."]
        #[inline(always)]
        pub const fn cmdstart_ris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started."]
        #[inline(always)]
        pub fn set_cmdstart_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command error raw/unmasked interrupt status."]
        #[inline(always)]
        pub const fn cmderr_ris(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command error raw/unmasked interrupt status."]
        #[inline(always)]
        pub fn set_cmderr_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Illegal command raw/unmasked interrupt status."]
        #[inline(always)]
        pub const fn illcmd_ris(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal command raw/unmasked interrupt status."]
        #[inline(always)]
        pub fn set_illcmd_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Mass read OK raw/unmasked interrupt status."]
        #[inline(always)]
        pub const fn readok_ris(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Mass read OK raw/unmasked interrupt status."]
        #[inline(always)]
        pub fn set_readok_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "(1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)."]
        #[inline(always)]
        pub const fn cmdsleeperr_ris(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "(1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)."]
        #[inline(always)]
        pub fn set_cmdsleeperr_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Irqraw {
        #[inline(always)]
        fn default() -> Irqraw {
            Irqraw(0)
        }
    }
    impl core::fmt::Debug for Irqraw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irqraw")
                .field("cmddone_ris", &self.cmddone_ris())
                .field("cmdstart_ris", &self.cmdstart_ris())
                .field("cmderr_ris", &self.cmderr_ris())
                .field("illcmd_ris", &self.illcmd_ris())
                .field("readok_ris", &self.readok_ris())
                .field("cmdsleeperr_ris", &self.cmdsleeperr_ris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irqraw {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Irqraw {{ cmddone_ris: {=bool:?}, cmdstart_ris: {=bool:?}, cmderr_ris: {=bool:?}, illcmd_ris: {=bool:?}, readok_ris: {=bool:?}, cmdsleeperr_ris: {=bool:?} }}" , self . cmddone_ris () , self . cmdstart_ris () , self . cmderr_ris () , self . illcmd_ris () , self . readok_ris () , self . cmdsleeperr_ris ())
        }
    }
    #[doc = "IRQSTAT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqstat(pub u32);
    impl Irqstat {
        #[doc = "Command done masked interrupt status."]
        #[inline(always)]
        pub const fn cmddone_mis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command done masked interrupt status."]
        #[inline(always)]
        pub fn set_cmddone_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command started masked interrupt status."]
        #[inline(always)]
        pub const fn cmdstart_mis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command started masked interrupt status."]
        #[inline(always)]
        pub fn set_cmdstart_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command error masked interrupt status."]
        #[inline(always)]
        pub const fn cmderr_mis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command error masked interrupt status."]
        #[inline(always)]
        pub fn set_cmderr_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Illegal command masked interrupt status."]
        #[inline(always)]
        pub const fn illcmd_mis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal command masked interrupt status."]
        #[inline(always)]
        pub fn set_illcmd_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Mass read OK masked interrupt status."]
        #[inline(always)]
        pub const fn readok_mis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Mass read OK masked interrupt status."]
        #[inline(always)]
        pub fn set_readok_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "(1: clear, 0: inactive) FNREADY_MIS flag."]
        #[inline(always)]
        pub const fn fnready_mis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "(1: clear, 0: inactive) FNREADY_MIS flag."]
        #[inline(always)]
        pub fn set_fnready_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Irqstat {
        #[inline(always)]
        fn default() -> Irqstat {
            Irqstat(0)
        }
    }
    impl core::fmt::Debug for Irqstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irqstat")
                .field("cmddone_mis", &self.cmddone_mis())
                .field("cmdstart_mis", &self.cmdstart_mis())
                .field("cmderr_mis", &self.cmderr_mis())
                .field("illcmd_mis", &self.illcmd_mis())
                .field("readok_mis", &self.readok_mis())
                .field("fnready_mis", &self.fnready_mis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irqstat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Irqstat {{ cmddone_mis: {=bool:?}, cmdstart_mis: {=bool:?}, cmderr_mis: {=bool:?}, illcmd_mis: {=bool:?}, readok_mis: {=bool:?}, fnready_mis: {=bool:?} }}" , self . cmddone_mis () , self . cmdstart_mis () , self . cmderr_mis () , self . illcmd_mis () , self . readok_mis () , self . fnready_mis ())
        }
    }
    #[doc = "LFSRVAL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lfsrval(pub u32);
    impl Lfsrval {
        #[doc = "Flash read data CRC signature."]
        #[inline(always)]
        pub const fn lfsrval(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash read data CRC signature."]
        #[inline(always)]
        pub fn set_lfsrval(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lfsrval {
        #[inline(always)]
        fn default() -> Lfsrval {
            Lfsrval(0)
        }
    }
    impl core::fmt::Debug for Lfsrval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lfsrval").field("lfsrval", &self.lfsrval()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lfsrval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lfsrval {{ lfsrval: {=u32:?} }}", self.lfsrval())
        }
    }
    #[doc = "PAGEPROT0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pageprot0(pub u32);
    impl Pageprot0 {
        #[doc = "First segment definition."]
        #[inline(always)]
        pub const fn seg0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "First segment definition."]
        #[inline(always)]
        pub fn set_seg0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Second segment definition. See SEG0 description for details on SEG1\\[31:16\\]
content."]
        #[inline(always)]
        pub const fn seg1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Second segment definition. See SEG0 description for details on SEG1\\[31:16\\]
content."]
        #[inline(always)]
        pub fn set_seg1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pageprot0 {
        #[inline(always)]
        fn default() -> Pageprot0 {
            Pageprot0(0)
        }
    }
    impl core::fmt::Debug for Pageprot0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pageprot0")
                .field("seg0", &self.seg0())
                .field("seg1", &self.seg1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pageprot0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pageprot0 {{ seg0: {=u16:?}, seg1: {=u16:?} }}",
                self.seg0(),
                self.seg1()
            )
        }
    }
    #[doc = "PAGEPROT1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pageprot1(pub u32);
    impl Pageprot1 {
        #[doc = "Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\\[15:0\\]
content."]
        #[inline(always)]
        pub const fn seg2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\\[15:0\\]
content."]
        #[inline(always)]
        pub fn set_seg2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\\[15:0\\]
content."]
        #[inline(always)]
        pub const fn seg3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\\[15:0\\]
content."]
        #[inline(always)]
        pub fn set_seg3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pageprot1 {
        #[inline(always)]
        fn default() -> Pageprot1 {
            Pageprot1(0)
        }
    }
    impl core::fmt::Debug for Pageprot1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pageprot1")
                .field("seg2", &self.seg2())
                .field("seg3", &self.seg3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pageprot1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pageprot1 {{ seg2: {=u16:?}, seg3: {=u16:?} }}",
                self.seg2(),
                self.seg3()
            )
        }
    }
    #[doc = "SIZE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Size(pub u32);
    impl Size {
        #[doc = "Maximum valid address for flash memory: - 00 : 0x0BFFF (192kb) - 01 : 0x0FFFF (256kb) - 10 : 0x17FFF (384kb) - 11 : 0x1FFFF (512kb)."]
        #[inline(always)]
        pub const fn flash_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum valid address for flash memory: - 00 : 0x0BFFF (192kb) - 01 : 0x0FFFF (256kb) - 10 : 0x17FFF (384kb) - 11 : 0x1FFFF (512kb)."]
        #[inline(always)]
        pub fn set_flash_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "RAM memory size selection: - 00 : 32kb - 01 : 32kb - 10 : 48kb - 11 : 64kb."]
        #[inline(always)]
        pub const fn ram_size(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "RAM memory size selection: - 00 : 32kb - 01 : 32kb - 10 : 48kb - 11 : 64kb."]
        #[inline(always)]
        pub fn set_ram_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Flash memory protection (0: no key present, 1: key present)."]
        #[inline(always)]
        pub const fn flash_secure(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory protection (0: no key present, 1: key present)."]
        #[inline(always)]
        pub fn set_flash_secure(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Flash+SWD protection: 0: No SWD protection (refer to FLASH_SECURE) 1: Flash and SWD protected."]
        #[inline(always)]
        pub const fn swd_disable(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flash+SWD protection: 0: No SWD protection (refer to FLASH_SECURE) 1: Flash and SWD protected."]
        #[inline(always)]
        pub fn set_swd_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins."]
        #[inline(always)]
        pub const fn package_size(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins."]
        #[inline(always)]
        pub fn set_package_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
    }
    impl Default for Size {
        #[inline(always)]
        fn default() -> Size {
            Size(0)
        }
    }
    impl core::fmt::Debug for Size {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Size")
                .field("flash_size", &self.flash_size())
                .field("ram_size", &self.ram_size())
                .field("flash_secure", &self.flash_secure())
                .field("swd_disable", &self.swd_disable())
                .field("package_size", &self.package_size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Size {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Size {{ flash_size: {=u16:?}, ram_size: {=u8:?}, flash_secure: {=bool:?}, swd_disable: {=bool:?}, package_size: {=u8:?} }}" , self . flash_size () , self . ram_size () , self . flash_secure () , self . swd_disable () , self . package_size ())
        }
    }
}
