#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SDMMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc {
    ptr: *mut u8,
}
unsafe impl Send for Sdmmc {}
unsafe impl Sync for Sdmmc {}
impl Sdmmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDMMC power control register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::Clkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    #[inline(always)]
    pub const fn argr(self) -> crate::common::Reg<regs::Argr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    #[inline(always)]
    pub const fn cmdr(self) -> crate::common::Reg<regs::Cmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SDMMC command response register"]
    #[inline(always)]
    pub const fn respcmdr(self) -> crate::common::Reg<regs::Respcmdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn respr(self, n: usize) -> crate::common::Reg<regs::RespxR, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    #[inline(always)]
    pub const fn dtimer(self) -> crate::common::Reg<regs::Dtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    #[inline(always)]
    pub const fn dlenr(self) -> crate::common::Reg<regs::Dlenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    #[inline(always)]
    pub const fn dctrl(self) -> crate::common::Reg<regs::Dctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    #[inline(always)]
    pub const fn dcntr(self) -> crate::common::Reg<regs::Dcntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    #[inline(always)]
    pub const fn maskr(self) -> crate::common::Reg<regs::Maskr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    #[inline(always)]
    pub const fn acktimer(self) -> crate::common::Reg<regs::Acktimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    #[inline(always)]
    pub const fn idmactrlr(self) -> crate::common::Reg<regs::Idmactrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    #[inline(always)]
    pub const fn idmabsizer(self) -> crate::common::Reg<regs::Idmabsizer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    #[inline(always)]
    pub const fn idmabase0r(self) -> crate::common::Reg<regs::Idmabase0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    #[inline(always)]
    pub const fn idmabase1r(self) -> crate::common::Reg<regs::Idmabase1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn fifor(self) -> crate::common::Reg<regs::Fifor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "SDMMC IP version register"]
    #[inline(always)]
    pub const fn ver(self) -> crate::common::Reg<regs::Ver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "SDMMC IP identification register"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
}
pub mod regs {
    #[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acktimer(pub u32);
    impl Acktimer {
        #[doc = "Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
        #[inline(always)]
        pub const fn acktime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
        #[inline(always)]
        pub fn set_acktime(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Acktimer {
        #[inline(always)]
        fn default() -> Acktimer {
            Acktimer(0)
        }
    }
    impl core::fmt::Debug for Acktimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acktimer").field("acktime", &self.acktime()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acktimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Acktimer {{ acktime: {=u32:?} }}", self.acktime())
        }
    }
    #[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Argr(pub u32);
    impl Argr {
        #[doc = "Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
        #[inline(always)]
        pub const fn cmdarg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
        #[inline(always)]
        pub fn set_cmdarg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Argr {
        #[inline(always)]
        fn default() -> Argr {
            Argr(0)
        }
    }
    impl core::fmt::Debug for Argr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Argr").field("cmdarg", &self.cmdarg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Argr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Argr {{ cmdarg: {=u32:?} }}", self.cmdarg())
        }
    }
    #[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkcr(pub u32);
    impl Clkcr {
        #[doc = "Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
        #[inline(always)]
        pub const fn pwrsav(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
        #[inline(always)]
        pub fn set_pwrsav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub const fn widbus(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub fn set_widbus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &gt;1 (CLKDIV &gt; 0) &amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) & DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &gt;1 (CLKDIV &gt; 0) &amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) & DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
        #[inline(always)]
        pub fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
        #[inline(always)]
        pub const fn hwfc_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
        #[inline(always)]
        pub fn set_hwfc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &gt;1. (CLKDIV &gt; 0)"]
        #[inline(always)]
        pub const fn ddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &gt;1. (CLKDIV &gt; 0)"]
        #[inline(always)]
        pub fn set_ddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub const fn busspeed(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub fn set_busspeed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub const fn selclkrx(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
        #[inline(always)]
        pub fn set_selclkrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Clkcr {
        #[inline(always)]
        fn default() -> Clkcr {
            Clkcr(0)
        }
    }
    impl core::fmt::Debug for Clkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkcr")
                .field("clkdiv", &self.clkdiv())
                .field("pwrsav", &self.pwrsav())
                .field("widbus", &self.widbus())
                .field("negedge", &self.negedge())
                .field("hwfc_en", &self.hwfc_en())
                .field("ddr", &self.ddr())
                .field("busspeed", &self.busspeed())
                .field("selclkrx", &self.selclkrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Clkcr {{ clkdiv: {=u16:?}, pwrsav: {=bool:?}, widbus: {=u8:?}, negedge: {=bool:?}, hwfc_en: {=bool:?}, ddr: {=bool:?}, busspeed: {=bool:?}, selclkrx: {=u8:?} }}" , self . clkdiv () , self . pwrsav () , self . widbus () , self . negedge () , self . hwfc_en () , self . ddr () , self . busspeed () , self . selclkrx ())
        }
    }
    #[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdr(pub u32);
    impl Cmdr {
        #[doc = "Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
        #[inline(always)]
        pub const fn cmdindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
        #[inline(always)]
        pub fn set_cmdindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
        #[inline(always)]
        pub const fn cmdtrans(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
        #[inline(always)]
        pub fn set_cmdtrans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
        #[inline(always)]
        pub const fn cmdstop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
        #[inline(always)]
        pub fn set_cmdstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
        #[inline(always)]
        pub const fn waitresp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
        #[inline(always)]
        pub fn set_waitresp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
        #[inline(always)]
        pub const fn waitint(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
        #[inline(always)]
        pub fn set_waitint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
        #[inline(always)]
        pub const fn waitpend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
        #[inline(always)]
        pub fn set_waitpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
        #[inline(always)]
        pub const fn cpsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
        #[inline(always)]
        pub fn set_cpsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
        #[inline(always)]
        pub const fn dthold(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
        #[inline(always)]
        pub fn set_dthold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
        #[inline(always)]
        pub const fn bootmode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
        #[inline(always)]
        pub fn set_bootmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable boot mode procedure."]
        #[inline(always)]
        pub const fn booten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable boot mode procedure."]
        #[inline(always)]
        pub fn set_booten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
        #[inline(always)]
        pub const fn cmdsuspend(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
        #[inline(always)]
        pub fn set_cmdsuspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cmdr {
        #[inline(always)]
        fn default() -> Cmdr {
            Cmdr(0)
        }
    }
    impl core::fmt::Debug for Cmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdr")
                .field("cmdindex", &self.cmdindex())
                .field("cmdtrans", &self.cmdtrans())
                .field("cmdstop", &self.cmdstop())
                .field("waitresp", &self.waitresp())
                .field("waitint", &self.waitint())
                .field("waitpend", &self.waitpend())
                .field("cpsmen", &self.cpsmen())
                .field("dthold", &self.dthold())
                .field("bootmode", &self.bootmode())
                .field("booten", &self.booten())
                .field("cmdsuspend", &self.cmdsuspend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmdr {{ cmdindex: {=u8:?}, cmdtrans: {=bool:?}, cmdstop: {=bool:?}, waitresp: {=u8:?}, waitint: {=bool:?}, waitpend: {=bool:?}, cpsmen: {=bool:?}, dthold: {=bool:?}, bootmode: {=bool:?}, booten: {=bool:?}, cmdsuspend: {=bool:?} }}" , self . cmdindex () , self . cmdtrans () , self . cmdstop () , self . waitresp () , self . waitint () , self . waitpend () , self . cpsmen () , self . dthold () , self . bootmode () , self . booten () , self . cmdsuspend ())
        }
    }
    #[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcntr(pub u32);
    impl Dcntr {
        #[doc = "Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
        #[inline(always)]
        pub const fn datacount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
        #[inline(always)]
        pub fn set_datacount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dcntr {
        #[inline(always)]
        fn default() -> Dcntr {
            Dcntr(0)
        }
    }
    impl core::fmt::Debug for Dcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcntr").field("datacount", &self.datacount()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcntr {{ datacount: {=u32:?} }}", self.datacount())
        }
    }
    #[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctrl(pub u32);
    impl Dctrl {
        #[doc = "Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
        #[inline(always)]
        pub fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn dtdir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_dtdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
        #[inline(always)]
        pub const fn dblocksize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
        #[inline(always)]
        pub fn set_dblocksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Read wait start. If this bit is set, read wait operation starts."]
        #[inline(always)]
        pub const fn rwstart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait start. If this bit is set, read wait operation starts."]
        #[inline(always)]
        pub fn set_rwstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
        #[inline(always)]
        pub const fn rwstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
        #[inline(always)]
        pub fn set_rwstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn rwmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_rwmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
        #[inline(always)]
        pub fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn bootacken(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_bootacken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
        #[inline(always)]
        pub const fn fiforst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
        #[inline(always)]
        pub fn set_fiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Dctrl {
        #[inline(always)]
        fn default() -> Dctrl {
            Dctrl(0)
        }
    }
    impl core::fmt::Debug for Dctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dctrl")
                .field("dten", &self.dten())
                .field("dtdir", &self.dtdir())
                .field("dtmode", &self.dtmode())
                .field("dblocksize", &self.dblocksize())
                .field("rwstart", &self.rwstart())
                .field("rwstop", &self.rwstop())
                .field("rwmod", &self.rwmod())
                .field("sdioen", &self.sdioen())
                .field("bootacken", &self.bootacken())
                .field("fiforst", &self.fiforst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dctrl {{ dten: {=bool:?}, dtdir: {=bool:?}, dtmode: {=u8:?}, dblocksize: {=u8:?}, rwstart: {=bool:?}, rwstop: {=bool:?}, rwmod: {=bool:?}, sdioen: {=bool:?}, bootacken: {=bool:?}, fiforst: {=bool:?} }}" , self . dten () , self . dtdir () , self . dtmode () , self . dblocksize () , self . rwstart () , self . rwstop () , self . rwmod () , self . sdioen () , self . bootacken () , self . fiforst ())
        }
    }
    #[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlenr(pub u32);
    impl Dlenr {
        #[doc = "Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
        #[inline(always)]
        pub const fn datalength(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
        #[inline(always)]
        pub fn set_datalength(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dlenr {
        #[inline(always)]
        fn default() -> Dlenr {
            Dlenr(0)
        }
    }
    impl core::fmt::Debug for Dlenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlenr").field("datalength", &self.datalength()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlenr {{ datalength: {=u32:?} }}", self.datalength())
        }
    }
    #[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtimer(pub u32);
    impl Dtimer {
        #[doc = "Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
        #[inline(always)]
        pub const fn datatime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
        #[inline(always)]
        pub fn set_datatime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dtimer {
        #[inline(always)]
        fn default() -> Dtimer {
            Dtimer(0)
        }
    }
    impl core::fmt::Debug for Dtimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtimer").field("datatime", &self.datatime()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dtimer {{ datatime: {=u32:?} }}", self.datatime())
        }
    }
    #[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifor(pub u32);
    impl Fifor {
        #[doc = "Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
        #[inline(always)]
        pub const fn fifodata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
        #[inline(always)]
        pub fn set_fifodata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifor {
        #[inline(always)]
        fn default() -> Fifor {
            Fifor(0)
        }
    }
    impl core::fmt::Debug for Fifor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifor").field("fifodata", &self.fifodata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifor {{ fifodata: {=u32:?} }}", self.fifodata())
        }
    }
    #[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
        #[inline(always)]
        pub const fn ccrcfailc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
        #[inline(always)]
        pub fn set_ccrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
        #[inline(always)]
        pub const fn dcrcfailc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
        #[inline(always)]
        pub fn set_dcrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
        #[inline(always)]
        pub const fn ctimeoutc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
        #[inline(always)]
        pub fn set_ctimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
        #[inline(always)]
        pub const fn dtimeoutc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
        #[inline(always)]
        pub fn set_dtimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
        #[inline(always)]
        pub const fn txunderrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
        #[inline(always)]
        pub fn set_txunderrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
        #[inline(always)]
        pub const fn rxoverrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
        #[inline(always)]
        pub fn set_rxoverrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMDREND flag clear bit Set by software to clear the CMDREND flag."]
        #[inline(always)]
        pub const fn cmdrendc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMDREND flag clear bit Set by software to clear the CMDREND flag."]
        #[inline(always)]
        pub fn set_cmdrendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
        #[inline(always)]
        pub const fn cmdsentc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
        #[inline(always)]
        pub fn set_cmdsentc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DATAEND flag clear bit Set by software to clear the DATAEND flag."]
        #[inline(always)]
        pub const fn dataendc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DATAEND flag clear bit Set by software to clear the DATAEND flag."]
        #[inline(always)]
        pub fn set_dataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DHOLD flag clear bit Set by software to clear the DHOLD flag."]
        #[inline(always)]
        pub const fn dholdc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DHOLD flag clear bit Set by software to clear the DHOLD flag."]
        #[inline(always)]
        pub fn set_dholdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
        #[inline(always)]
        pub const fn dbckendc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
        #[inline(always)]
        pub fn set_dbckendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DABORT flag clear bit Set by software to clear the DABORT flag."]
        #[inline(always)]
        pub const fn dabortc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DABORT flag clear bit Set by software to clear the DABORT flag."]
        #[inline(always)]
        pub fn set_dabortc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
        #[inline(always)]
        pub const fn busyd0endc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
        #[inline(always)]
        pub fn set_busyd0endc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
        #[inline(always)]
        pub const fn sdioitc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
        #[inline(always)]
        pub fn set_sdioitc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
        #[inline(always)]
        pub const fn ackfailc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
        #[inline(always)]
        pub fn set_ackfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
        #[inline(always)]
        pub const fn acktimeoutc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
        #[inline(always)]
        pub fn set_acktimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VSWEND flag clear bit Set by software to clear the VSWEND flag."]
        #[inline(always)]
        pub const fn vswendc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VSWEND flag clear bit Set by software to clear the VSWEND flag."]
        #[inline(always)]
        pub fn set_vswendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
        #[inline(always)]
        pub const fn ckstopc(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
        #[inline(always)]
        pub fn set_ckstopc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
        #[inline(always)]
        pub const fn idmatec(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
        #[inline(always)]
        pub fn set_idmatec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
        #[inline(always)]
        pub const fn idmabtcc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
        #[inline(always)]
        pub fn set_idmabtcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("ccrcfailc", &self.ccrcfailc())
                .field("dcrcfailc", &self.dcrcfailc())
                .field("ctimeoutc", &self.ctimeoutc())
                .field("dtimeoutc", &self.dtimeoutc())
                .field("txunderrc", &self.txunderrc())
                .field("rxoverrc", &self.rxoverrc())
                .field("cmdrendc", &self.cmdrendc())
                .field("cmdsentc", &self.cmdsentc())
                .field("dataendc", &self.dataendc())
                .field("dholdc", &self.dholdc())
                .field("dbckendc", &self.dbckendc())
                .field("dabortc", &self.dabortc())
                .field("busyd0endc", &self.busyd0endc())
                .field("sdioitc", &self.sdioitc())
                .field("ackfailc", &self.ackfailc())
                .field("acktimeoutc", &self.acktimeoutc())
                .field("vswendc", &self.vswendc())
                .field("ckstopc", &self.ckstopc())
                .field("idmatec", &self.idmatec())
                .field("idmabtcc", &self.idmabtcc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ ccrcfailc: {=bool:?}, dcrcfailc: {=bool:?}, ctimeoutc: {=bool:?}, dtimeoutc: {=bool:?}, txunderrc: {=bool:?}, rxoverrc: {=bool:?}, cmdrendc: {=bool:?}, cmdsentc: {=bool:?}, dataendc: {=bool:?}, dholdc: {=bool:?}, dbckendc: {=bool:?}, dabortc: {=bool:?}, busyd0endc: {=bool:?}, sdioitc: {=bool:?}, ackfailc: {=bool:?}, acktimeoutc: {=bool:?}, vswendc: {=bool:?}, ckstopc: {=bool:?}, idmatec: {=bool:?}, idmabtcc: {=bool:?} }}" , self . ccrcfailc () , self . dcrcfailc () , self . ctimeoutc () , self . dtimeoutc () , self . txunderrc () , self . rxoverrc () , self . cmdrendc () , self . cmdsentc () , self . dataendc () , self . dholdc () , self . dbckendc () , self . dabortc () , self . busyd0endc () , self . sdioitc () , self . ackfailc () , self . acktimeoutc () , self . vswendc () , self . ckstopc () , self . idmatec () , self . idmabtcc ())
        }
    }
    #[doc = "SDMMC IP identification register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Id(pub u32);
    impl Id {
        #[doc = "SDMMC IP identification."]
        #[inline(always)]
        pub const fn ip_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SDMMC IP identification."]
        #[inline(always)]
        pub fn set_ip_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Id {
        #[inline(always)]
        fn default() -> Id {
            Id(0)
        }
    }
    impl core::fmt::Debug for Id {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Id").field("ip_id", &self.ip_id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Id {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Id {{ ip_id: {=u32:?} }}", self.ip_id())
        }
    }
    #[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabase0r(pub u32);
    impl Idmabase0r {
        #[doc = "Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
        #[inline(always)]
        pub const fn idmabase0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
        #[inline(always)]
        pub fn set_idmabase0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Idmabase0r {
        #[inline(always)]
        fn default() -> Idmabase0r {
            Idmabase0r(0)
        }
    }
    impl core::fmt::Debug for Idmabase0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabase0r")
                .field("idmabase0", &self.idmabase0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabase0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabase0r {{ idmabase0: {=u32:?} }}", self.idmabase0())
        }
    }
    #[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabase1r(pub u32);
    impl Idmabase1r {
        #[doc = "Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
        #[inline(always)]
        pub const fn idmabase1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
        #[inline(always)]
        pub fn set_idmabase1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Idmabase1r {
        #[inline(always)]
        fn default() -> Idmabase1r {
            Idmabase1r(0)
        }
    }
    impl core::fmt::Debug for Idmabase1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabase1r")
                .field("idmabase1", &self.idmabase1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabase1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabase1r {{ idmabase1: {=u32:?} }}", self.idmabase1())
        }
    }
    #[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabsizer(pub u32);
    impl Idmabsizer {
        #[doc = "Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn idmabndt(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[doc = "Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_idmabndt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
    }
    impl Default for Idmabsizer {
        #[inline(always)]
        fn default() -> Idmabsizer {
            Idmabsizer(0)
        }
    }
    impl core::fmt::Debug for Idmabsizer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabsizer")
                .field("idmabndt", &self.idmabndt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabsizer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabsizer {{ idmabndt: {=u8:?} }}", self.idmabndt())
        }
    }
    #[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmactrlr(pub u32);
    impl Idmactrlr {
        #[doc = "IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn idmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_idmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub const fn idmabmode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
        #[inline(always)]
        pub fn set_idmabmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
        #[inline(always)]
        pub const fn idmabact(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
        #[inline(always)]
        pub fn set_idmabact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Idmactrlr {
        #[inline(always)]
        fn default() -> Idmactrlr {
            Idmactrlr(0)
        }
    }
    impl core::fmt::Debug for Idmactrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmactrlr")
                .field("idmaen", &self.idmaen())
                .field("idmabmode", &self.idmabmode())
                .field("idmabact", &self.idmabact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmactrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idmactrlr {{ idmaen: {=bool:?}, idmabmode: {=bool:?}, idmabact: {=bool:?} }}",
                self.idmaen(),
                self.idmabmode(),
                self.idmabact()
            )
        }
    }
    #[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskr(pub u32);
    impl Maskr {
        #[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
        #[inline(always)]
        pub const fn ccrcfailie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
        #[inline(always)]
        pub fn set_ccrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
        #[inline(always)]
        pub const fn dcrcfailie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
        #[inline(always)]
        pub fn set_dcrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
        #[inline(always)]
        pub const fn ctimeoutie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
        #[inline(always)]
        pub fn set_ctimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
        #[inline(always)]
        pub const fn dtimeoutie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
        #[inline(always)]
        pub fn set_dtimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
        #[inline(always)]
        pub const fn txunderrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
        #[inline(always)]
        pub fn set_txunderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
        #[inline(always)]
        pub const fn rxoverrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
        #[inline(always)]
        pub fn set_rxoverrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
        #[inline(always)]
        pub const fn cmdrendie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
        #[inline(always)]
        pub fn set_cmdrendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
        #[inline(always)]
        pub const fn cmdsentie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
        #[inline(always)]
        pub fn set_cmdsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
        #[inline(always)]
        pub const fn dataendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
        #[inline(always)]
        pub fn set_dataendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
        #[inline(always)]
        pub const fn dholdie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
        #[inline(always)]
        pub fn set_dholdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
        #[inline(always)]
        pub const fn dbckendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
        #[inline(always)]
        pub fn set_dbckendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
        #[inline(always)]
        pub const fn dabortie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
        #[inline(always)]
        pub fn set_dabortie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
        #[inline(always)]
        pub const fn txfifoheie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
        #[inline(always)]
        pub fn set_txfifoheie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
        #[inline(always)]
        pub const fn rxfifohfie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
        #[inline(always)]
        pub fn set_rxfifohfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
        #[inline(always)]
        pub const fn rxfifofie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
        #[inline(always)]
        pub fn set_rxfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
        #[inline(always)]
        pub const fn txfifoeie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
        #[inline(always)]
        pub fn set_txfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
        #[inline(always)]
        pub const fn busyd0endie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
        #[inline(always)]
        pub fn set_busyd0endie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
        #[inline(always)]
        pub const fn sdioitie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
        #[inline(always)]
        pub fn set_sdioitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
        #[inline(always)]
        pub const fn ackfailie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
        #[inline(always)]
        pub fn set_ackfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
        #[inline(always)]
        pub const fn acktimeoutie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
        #[inline(always)]
        pub fn set_acktimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
        #[inline(always)]
        pub const fn vswendie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
        #[inline(always)]
        pub fn set_vswendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
        #[inline(always)]
        pub const fn ckstopie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
        #[inline(always)]
        pub fn set_ckstopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
        #[inline(always)]
        pub const fn idmabtcie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
        #[inline(always)]
        pub fn set_idmabtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Maskr {
        #[inline(always)]
        fn default() -> Maskr {
            Maskr(0)
        }
    }
    impl core::fmt::Debug for Maskr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maskr")
                .field("ccrcfailie", &self.ccrcfailie())
                .field("dcrcfailie", &self.dcrcfailie())
                .field("ctimeoutie", &self.ctimeoutie())
                .field("dtimeoutie", &self.dtimeoutie())
                .field("txunderrie", &self.txunderrie())
                .field("rxoverrie", &self.rxoverrie())
                .field("cmdrendie", &self.cmdrendie())
                .field("cmdsentie", &self.cmdsentie())
                .field("dataendie", &self.dataendie())
                .field("dholdie", &self.dholdie())
                .field("dbckendie", &self.dbckendie())
                .field("dabortie", &self.dabortie())
                .field("txfifoheie", &self.txfifoheie())
                .field("rxfifohfie", &self.rxfifohfie())
                .field("rxfifofie", &self.rxfifofie())
                .field("txfifoeie", &self.txfifoeie())
                .field("busyd0endie", &self.busyd0endie())
                .field("sdioitie", &self.sdioitie())
                .field("ackfailie", &self.ackfailie())
                .field("acktimeoutie", &self.acktimeoutie())
                .field("vswendie", &self.vswendie())
                .field("ckstopie", &self.ckstopie())
                .field("idmabtcie", &self.idmabtcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maskr {{ ccrcfailie: {=bool:?}, dcrcfailie: {=bool:?}, ctimeoutie: {=bool:?}, dtimeoutie: {=bool:?}, txunderrie: {=bool:?}, rxoverrie: {=bool:?}, cmdrendie: {=bool:?}, cmdsentie: {=bool:?}, dataendie: {=bool:?}, dholdie: {=bool:?}, dbckendie: {=bool:?}, dabortie: {=bool:?}, txfifoheie: {=bool:?}, rxfifohfie: {=bool:?}, rxfifofie: {=bool:?}, txfifoeie: {=bool:?}, busyd0endie: {=bool:?}, sdioitie: {=bool:?}, ackfailie: {=bool:?}, acktimeoutie: {=bool:?}, vswendie: {=bool:?}, ckstopie: {=bool:?}, idmabtcie: {=bool:?} }}" , self . ccrcfailie () , self . dcrcfailie () , self . ctimeoutie () , self . dtimeoutie () , self . txunderrie () , self . rxoverrie () , self . cmdrendie () , self . cmdsentie () , self . dataendie () , self . dholdie () , self . dbckendie () , self . dabortie () , self . txfifoheie () , self . rxfifohfie () , self . rxfifofie () , self . txfifoeie () , self . busyd0endie () , self . sdioitie () , self . ackfailie () , self . acktimeoutie () , self . vswendie () , self . ckstopie () , self . idmabtcie ())
        }
    }
    #[doc = "SDMMC power control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u32);
    impl Power {
        #[doc = "SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
        #[inline(always)]
        pub const fn pwrctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
        #[inline(always)]
        pub fn set_pwrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
        #[inline(always)]
        pub const fn vswitch(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
        #[inline(always)]
        pub fn set_vswitch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
        #[inline(always)]
        pub const fn vswitchen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
        #[inline(always)]
        pub fn set_vswitchen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
        #[inline(always)]
        pub const fn dirpol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
        #[inline(always)]
        pub fn set_dirpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    impl core::fmt::Debug for Power {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Power")
                .field("pwrctrl", &self.pwrctrl())
                .field("vswitch", &self.vswitch())
                .field("vswitchen", &self.vswitchen())
                .field("dirpol", &self.dirpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Power {{ pwrctrl: {=u8:?}, vswitch: {=bool:?}, vswitchen: {=bool:?}, dirpol: {=bool:?} }}",
                self.pwrctrl(),
                self.vswitch(),
                self.vswitchen(),
                self.dirpol()
            )
        }
    }
    #[doc = "SDMMC command response register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Respcmdr(pub u32);
    impl Respcmdr {
        #[doc = "Response command index"]
        #[inline(always)]
        pub const fn respcmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Response command index"]
        #[inline(always)]
        pub fn set_respcmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Respcmdr {
        #[inline(always)]
        fn default() -> Respcmdr {
            Respcmdr(0)
        }
    }
    impl core::fmt::Debug for Respcmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Respcmdr").field("respcmd", &self.respcmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Respcmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Respcmdr {{ respcmd: {=u8:?} }}", self.respcmd())
        }
    }
    #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RespxR(pub u32);
    impl RespxR {
        #[doc = "see Table 432"]
        #[inline(always)]
        pub const fn cardstatus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "see Table 432"]
        #[inline(always)]
        pub fn set_cardstatus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RespxR {
        #[inline(always)]
        fn default() -> RespxR {
            RespxR(0)
        }
    }
    impl core::fmt::Debug for RespxR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RespxR")
                .field("cardstatus", &self.cardstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RespxR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RespxR {{ cardstatus: {=u32:?} }}", self.cardstatus())
        }
    }
    #[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star(pub u32);
    impl Star {
        #[doc = "Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn ccrcfail(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_ccrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dcrcfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dcrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods."]
        #[inline(always)]
        pub const fn ctimeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods."]
        #[inline(always)]
        pub fn set_ctimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dtimeout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dtimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn txunderr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_txunderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn rxoverr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_rxoverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn cmdrend(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_cmdrend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn cmdsent(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_cmdsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dataend(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dhold(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dhold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dbckend(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dbckend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn dabort(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_dabort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
        #[inline(always)]
        pub const fn dpsmact(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
        #[inline(always)]
        pub fn set_dpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
        #[inline(always)]
        pub const fn cpsmact(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
        #[inline(always)]
        pub fn set_cpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full."]
        #[inline(always)]
        pub const fn txfifohe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full."]
        #[inline(always)]
        pub fn set_txfifohe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty."]
        #[inline(always)]
        pub const fn rxfifohf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty."]
        #[inline(always)]
        pub fn set_rxfifohf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty."]
        #[inline(always)]
        pub const fn txfifof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty."]
        #[inline(always)]
        pub fn set_txfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive FIFO full This bit is cleared when one FIFO location becomes empty."]
        #[inline(always)]
        pub const fn rxfifof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO full This bit is cleared when one FIFO location becomes empty."]
        #[inline(always)]
        pub fn set_rxfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Transmit FIFO empty This bit is cleared when one FIFO location becomes full."]
        #[inline(always)]
        pub const fn txfifoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO empty This bit is cleared when one FIFO location becomes full."]
        #[inline(always)]
        pub fn set_txfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full."]
        #[inline(always)]
        pub const fn rxfifoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full."]
        #[inline(always)]
        pub fn set_rxfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt."]
        #[inline(always)]
        pub const fn busyd0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt."]
        #[inline(always)]
        pub fn set_busyd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn busyd0end(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_busyd0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn sdioit(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_sdioit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn ackfail(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_ackfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn acktimeout(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_acktimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn vswend(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_vswend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn ckstop(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_ckstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn idmate(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_idmate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub const fn idmabtc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
        #[inline(always)]
        pub fn set_idmabtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Star {
        #[inline(always)]
        fn default() -> Star {
            Star(0)
        }
    }
    impl core::fmt::Debug for Star {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Star")
                .field("ccrcfail", &self.ccrcfail())
                .field("dcrcfail", &self.dcrcfail())
                .field("ctimeout", &self.ctimeout())
                .field("dtimeout", &self.dtimeout())
                .field("txunderr", &self.txunderr())
                .field("rxoverr", &self.rxoverr())
                .field("cmdrend", &self.cmdrend())
                .field("cmdsent", &self.cmdsent())
                .field("dataend", &self.dataend())
                .field("dhold", &self.dhold())
                .field("dbckend", &self.dbckend())
                .field("dabort", &self.dabort())
                .field("dpsmact", &self.dpsmact())
                .field("cpsmact", &self.cpsmact())
                .field("txfifohe", &self.txfifohe())
                .field("rxfifohf", &self.rxfifohf())
                .field("txfifof", &self.txfifof())
                .field("rxfifof", &self.rxfifof())
                .field("txfifoe", &self.txfifoe())
                .field("rxfifoe", &self.rxfifoe())
                .field("busyd0", &self.busyd0())
                .field("busyd0end", &self.busyd0end())
                .field("sdioit", &self.sdioit())
                .field("ackfail", &self.ackfail())
                .field("acktimeout", &self.acktimeout())
                .field("vswend", &self.vswend())
                .field("ckstop", &self.ckstop())
                .field("idmate", &self.idmate())
                .field("idmabtc", &self.idmabtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Star {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Star {{ ccrcfail: {=bool:?}, dcrcfail: {=bool:?}, ctimeout: {=bool:?}, dtimeout: {=bool:?}, txunderr: {=bool:?}, rxoverr: {=bool:?}, cmdrend: {=bool:?}, cmdsent: {=bool:?}, dataend: {=bool:?}, dhold: {=bool:?}, dbckend: {=bool:?}, dabort: {=bool:?}, dpsmact: {=bool:?}, cpsmact: {=bool:?}, txfifohe: {=bool:?}, rxfifohf: {=bool:?}, txfifof: {=bool:?}, rxfifof: {=bool:?}, txfifoe: {=bool:?}, rxfifoe: {=bool:?}, busyd0: {=bool:?}, busyd0end: {=bool:?}, sdioit: {=bool:?}, ackfail: {=bool:?}, acktimeout: {=bool:?}, vswend: {=bool:?}, ckstop: {=bool:?}, idmate: {=bool:?}, idmabtc: {=bool:?} }}" , self . ccrcfail () , self . dcrcfail () , self . ctimeout () , self . dtimeout () , self . txunderr () , self . rxoverr () , self . cmdrend () , self . cmdsent () , self . dataend () , self . dhold () , self . dbckend () , self . dabort () , self . dpsmact () , self . cpsmact () , self . txfifohe () , self . rxfifohf () , self . txfifof () , self . rxfifof () , self . txfifoe () , self . rxfifoe () , self . busyd0 () , self . busyd0end () , self . sdioit () , self . ackfail () , self . acktimeout () , self . vswend () , self . ckstop () , self . idmate () , self . idmabtc ())
        }
    }
    #[doc = "SDMMC IP version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ver(pub u32);
    impl Ver {
        #[doc = "IP minor revision number."]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "IP minor revision number."]
        #[inline(always)]
        pub fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "IP major revision number."]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "IP major revision number."]
        #[inline(always)]
        pub fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Ver {
        #[inline(always)]
        fn default() -> Ver {
            Ver(0)
        }
    }
    impl core::fmt::Debug for Ver {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ver")
                .field("minrev", &self.minrev())
                .field("majrev", &self.majrev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ver {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ver {{ minrev: {=u8:?}, majrev: {=u8:?} }}",
                self.minrev(),
                self.majrev()
            )
        }
    }
}
