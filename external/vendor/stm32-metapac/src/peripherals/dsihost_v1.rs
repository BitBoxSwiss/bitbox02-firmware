#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DSI Host."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsihost {
    ptr: *mut u8,
}
unsafe impl Send for Dsihost {}
unsafe impl Sync for Dsihost {}
impl Dsihost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DSI Host Version Register."]
    #[inline(always)]
    pub const fn vr(self) -> crate::common::Reg<regs::Vr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DSI Host Control Register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DSI HOST Clock Control Register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DSI Host LTDC VCID Register."]
    #[inline(always)]
    pub const fn lvcidr(self) -> crate::common::Reg<regs::Lvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DSI Host LTDC Color Coding Register."]
    #[inline(always)]
    pub const fn lcolcr(self) -> crate::common::Reg<regs::Lcolcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DSI Host LTDC Polarity Configuration Register."]
    #[inline(always)]
    pub const fn lpcr(self) -> crate::common::Reg<regs::Lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DSI Host Low-Power mode Configuration Register."]
    #[inline(always)]
    pub const fn lpmcr(self) -> crate::common::Reg<regs::Lpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DSI Host Protocol Configuration Register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DSI Host Generic VCID Register."]
    #[inline(always)]
    pub const fn gvcidr(self) -> crate::common::Reg<regs::Gvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "DSI Host mode Configuration Register."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "DSI Host Video mode Configuration Register."]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DSI Host Video Packet Configuration Register."]
    #[inline(always)]
    pub const fn vpcr(self) -> crate::common::Reg<regs::Vpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "DSI Host Video Chunks Configuration Register."]
    #[inline(always)]
    pub const fn vccr(self) -> crate::common::Reg<regs::Vccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DSI Host Video Null Packet Configuration Register."]
    #[inline(always)]
    pub const fn vnpcr(self) -> crate::common::Reg<regs::Vnpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DSI Host Video HSA Configuration Register."]
    #[inline(always)]
    pub const fn vhsacr(self) -> crate::common::Reg<regs::Vhsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DSI Host Video HBP Configuration Register."]
    #[inline(always)]
    pub const fn vhbpcr(self) -> crate::common::Reg<regs::Vhbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "DSI Host Video Line Configuration Register."]
    #[inline(always)]
    pub const fn vlcr(self) -> crate::common::Reg<regs::Vlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "DSI Host Video VSA Configuration Register."]
    #[inline(always)]
    pub const fn vvsacr(self) -> crate::common::Reg<regs::Vvsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "DSI Host Video VBP Configuration Register."]
    #[inline(always)]
    pub const fn vvbpcr(self) -> crate::common::Reg<regs::Vvbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "DSI Host Video VFP Configuration Register."]
    #[inline(always)]
    pub const fn vvfpcr(self) -> crate::common::Reg<regs::Vvfpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "DSI Host Video VA Configuration Register."]
    #[inline(always)]
    pub const fn vvacr(self) -> crate::common::Reg<regs::Vvacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "DSI Host LTDC Command Configuration Register."]
    #[inline(always)]
    pub const fn lccr(self) -> crate::common::Reg<regs::Lccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "DSI Host Command mode Configuration Register."]
    #[inline(always)]
    pub const fn cmcr(self) -> crate::common::Reg<regs::Cmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DSI Host Generic Header Configuration Register."]
    #[inline(always)]
    pub const fn ghcr(self) -> crate::common::Reg<regs::Ghcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "DSI Host Generic Payload Data Register."]
    #[inline(always)]
    pub const fn gpdr(self) -> crate::common::Reg<regs::Gpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "DSI Host Generic Packet Status Register."]
    #[inline(always)]
    pub const fn gpsr(self) -> crate::common::Reg<regs::Gpsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 0."]
    #[inline(always)]
    pub const fn tccr0(self) -> crate::common::Reg<regs::Tccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 1."]
    #[inline(always)]
    pub const fn tccr1(self) -> crate::common::Reg<regs::Tccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 2."]
    #[inline(always)]
    pub const fn tccr2(self) -> crate::common::Reg<regs::Tccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 3."]
    #[inline(always)]
    pub const fn tccr3(self) -> crate::common::Reg<regs::Tccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 4."]
    #[inline(always)]
    pub const fn tccr4(self) -> crate::common::Reg<regs::Tccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 5."]
    #[inline(always)]
    pub const fn tccr5(self) -> crate::common::Reg<regs::Tccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "DSI Host Clock Lane Configuration Register."]
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Clcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "DSI Host Clock Lane Timer Configuration Register."]
    #[inline(always)]
    pub const fn cltcr(self) -> crate::common::Reg<regs::Cltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "DSI Host Data Lane Timer Configuration Register."]
    #[inline(always)]
    pub const fn dltcr(self) -> crate::common::Reg<regs::Dltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "DSI Host PHY Control Register."]
    #[inline(always)]
    pub const fn pctlr(self) -> crate::common::Reg<regs::Pctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "DSI Host PHY Configuration Register."]
    #[inline(always)]
    pub const fn pconfr(self) -> crate::common::Reg<regs::Pconfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "DSI Host PHY ULPS Control Register."]
    #[inline(always)]
    pub const fn pucr(self) -> crate::common::Reg<regs::Pucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "DSI Host PHY TX Triggers Configuration Register."]
    #[inline(always)]
    pub const fn pttcr(self) -> crate::common::Reg<regs::Pttcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "DSI Host PHY Status Register."]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "DSI Host Interrupt & Status Register 0."]
    #[inline(always)]
    pub const fn isr0(self) -> crate::common::Reg<regs::Isr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "DSI Host Interrupt & Status Register 1."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "DSI Host Interrupt Enable Register 0."]
    #[inline(always)]
    pub const fn ier0(self) -> crate::common::Reg<regs::Ier0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "DSI Host Interrupt Enable Register 1."]
    #[inline(always)]
    pub const fn ier1(self) -> crate::common::Reg<regs::Ier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "DSI Host Force Interrupt Register 0."]
    #[inline(always)]
    pub const fn fir0(self) -> crate::common::Reg<regs::Fir0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "DSI Host Force Interrupt Register 1."]
    #[inline(always)]
    pub const fn fir1(self) -> crate::common::Reg<regs::Fir1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "DSI Host Video Shadow Control Register."]
    #[inline(always)]
    pub const fn vscr(self) -> crate::common::Reg<regs::Vscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DSI Host LTDC Current VCID Register."]
    #[inline(always)]
    pub const fn lcvcidr(self) -> crate::common::Reg<regs::Lcvcidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "DSI Host LTDC Current Color Coding Register."]
    #[inline(always)]
    pub const fn lcccr(self) -> crate::common::Reg<regs::Lcccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "DSI Host Low-Power mode Current Configuration Register."]
    #[inline(always)]
    pub const fn lpmccr(self) -> crate::common::Reg<regs::Lpmccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "DSI Host Video mode Current Configuration Register."]
    #[inline(always)]
    pub const fn vmccr(self) -> crate::common::Reg<regs::Vmccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "DSI Host Video Packet Current Configuration Register."]
    #[inline(always)]
    pub const fn vpccr(self) -> crate::common::Reg<regs::Vpccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "DSI Host Video Chunks Current Configuration Register."]
    #[inline(always)]
    pub const fn vcccr(self) -> crate::common::Reg<regs::Vcccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "DSI Host Video Null Packet Current Configuration Register."]
    #[inline(always)]
    pub const fn vnpccr(self) -> crate::common::Reg<regs::Vnpccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "DSI Host Video HSA Current Configuration Register."]
    #[inline(always)]
    pub const fn vhsaccr(self) -> crate::common::Reg<regs::Vhsaccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "DSI Host Video HBP Current Configuration Register."]
    #[inline(always)]
    pub const fn vhbpccr(self) -> crate::common::Reg<regs::Vhbpccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "DSI Host Video Line Current Configuration Register."]
    #[inline(always)]
    pub const fn vlccr(self) -> crate::common::Reg<regs::Vlccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "DSI Host Video VSA Current Configuration Register."]
    #[inline(always)]
    pub const fn vvsaccr(self) -> crate::common::Reg<regs::Vvsaccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "DSI Host Video VBP Current Configuration Register."]
    #[inline(always)]
    pub const fn vvbpccr(self) -> crate::common::Reg<regs::Vvbpccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "DSI Host Video VFP Current Configuration Register."]
    #[inline(always)]
    pub const fn vvfpccr(self) -> crate::common::Reg<regs::Vvfpccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "DSI Host Video VA Current Configuration Register."]
    #[inline(always)]
    pub const fn vvaccr(self) -> crate::common::Reg<regs::Vvaccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "DSI Wrapper Configuration Register."]
    #[inline(always)]
    pub const fn wcfgr(self) -> crate::common::Reg<regs::Wcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "DSI Wrapper Control Register."]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "DSI Wrapper Interrupt Enable Register."]
    #[inline(always)]
    pub const fn wier(self) -> crate::common::Reg<regs::Wier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "DSI Wrapper Interrupt & Status Register."]
    #[inline(always)]
    pub const fn wisr(self) -> crate::common::Reg<regs::Wisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "DSI Wrapper Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn wifcr(self) -> crate::common::Reg<regs::Wifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 0."]
    #[inline(always)]
    pub const fn wpcr0(self) -> crate::common::Reg<regs::Wpcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 1."]
    #[inline(always)]
    pub const fn wpcr1(self) -> crate::common::Reg<regs::Wpcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 2."]
    #[inline(always)]
    pub const fn wpcr2(self) -> crate::common::Reg<regs::Wpcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 3."]
    #[inline(always)]
    pub const fn wpcr3(self) -> crate::common::Reg<regs::Wpcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 4."]
    #[inline(always)]
    pub const fn wpcr4(self) -> crate::common::Reg<regs::Wpcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[doc = "DSI Wrapper Regulator and PLL Control Register."]
    #[inline(always)]
    pub const fn wrpcr(self) -> crate::common::Reg<regs::Wrpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
}
pub mod regs {
    #[doc = "DSI HOST Clock Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "TX Escape Clock Division."]
        #[inline(always)]
        pub const fn txeckdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TX Escape Clock Division."]
        #[inline(always)]
        pub fn set_txeckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timeout Clock Division."]
        #[inline(always)]
        pub const fn tockdiv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timeout Clock Division."]
        #[inline(always)]
        pub fn set_tockdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                .field("txeckdiv", &self.txeckdiv())
                .field("tockdiv", &self.tockdiv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ txeckdiv: {=u8:?}, tockdiv: {=u8:?} }}",
                self.txeckdiv(),
                self.tockdiv()
            )
        }
    }
    #[doc = "DSI Host Clock Lane Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clcr(pub u32);
    impl Clcr {
        #[doc = "D-PHY Clock Control."]
        #[inline(always)]
        pub const fn dpcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY Clock Control."]
        #[inline(always)]
        pub fn set_dpcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic Clock lane Control."]
        #[inline(always)]
        pub const fn acr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Clock lane Control."]
        #[inline(always)]
        pub fn set_acr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Clcr {
        #[inline(always)]
        fn default() -> Clcr {
            Clcr(0)
        }
    }
    impl core::fmt::Debug for Clcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clcr")
                .field("dpcc", &self.dpcc())
                .field("acr", &self.acr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clcr {{ dpcc: {=bool:?}, acr: {=bool:?} }}", self.dpcc(), self.acr())
        }
    }
    #[doc = "DSI Host Clock Lane Timer Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cltcr(pub u32);
    impl Cltcr {
        #[doc = "Low-Power to High-Speed Time."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Low-Power to High-Speed Time."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "High-Speed to Low-Power Time."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "High-Speed to Low-Power Time."]
        #[inline(always)]
        pub fn set_hs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Cltcr {
        #[inline(always)]
        fn default() -> Cltcr {
            Cltcr(0)
        }
    }
    impl core::fmt::Debug for Cltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cltcr")
                .field("lp2hs_time", &self.lp2hs_time())
                .field("hs2lp_time", &self.hs2lp_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cltcr {{ lp2hs_time: {=u16:?}, hs2lp_time: {=u16:?} }}",
                self.lp2hs_time(),
                self.hs2lp_time()
            )
        }
    }
    #[doc = "DSI Host Command mode Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmcr(pub u32);
    impl Cmcr {
        #[doc = "Tearing Effect Acknowledge Request Enable."]
        #[inline(always)]
        pub const fn teare(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing Effect Acknowledge Request Enable."]
        #[inline(always)]
        pub fn set_teare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge Request Enable."]
        #[inline(always)]
        pub const fn are(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Request Enable."]
        #[inline(always)]
        pub fn set_are(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Generic Short Write Zero parameters Transmission."]
        #[inline(always)]
        pub const fn gsw0tx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Write Zero parameters Transmission."]
        #[inline(always)]
        pub fn set_gsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic Short Write One parameters Transmission."]
        #[inline(always)]
        pub const fn gsw1tx(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Write One parameters Transmission."]
        #[inline(always)]
        pub fn set_gsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic Short Write Two parameters Transmission."]
        #[inline(always)]
        pub const fn gsw2tx(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Write Two parameters Transmission."]
        #[inline(always)]
        pub fn set_gsw2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic Short Read Zero parameters Transmission."]
        #[inline(always)]
        pub const fn gsr0tx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Read Zero parameters Transmission."]
        #[inline(always)]
        pub fn set_gsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic Short Read One parameters Transmission."]
        #[inline(always)]
        pub const fn gsr1tx(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Read One parameters Transmission."]
        #[inline(always)]
        pub fn set_gsr1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Generic Short Read Two parameters Transmission."]
        #[inline(always)]
        pub const fn gsr2tx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Short Read Two parameters Transmission."]
        #[inline(always)]
        pub fn set_gsr2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Generic Long Write Transmission."]
        #[inline(always)]
        pub const fn glwtx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Long Write Transmission."]
        #[inline(always)]
        pub fn set_glwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DCS Short Write Zero parameter Transmission."]
        #[inline(always)]
        pub const fn dsw0tx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCS Short Write Zero parameter Transmission."]
        #[inline(always)]
        pub fn set_dsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DCS Short Read One parameter Transmission."]
        #[inline(always)]
        pub const fn dsw1tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DCS Short Read One parameter Transmission."]
        #[inline(always)]
        pub fn set_dsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DCS Short Read Zero parameter Transmission."]
        #[inline(always)]
        pub const fn dsr0tx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DCS Short Read Zero parameter Transmission."]
        #[inline(always)]
        pub fn set_dsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DCS Long Write Transmission."]
        #[inline(always)]
        pub const fn dlwtx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DCS Long Write Transmission."]
        #[inline(always)]
        pub fn set_dlwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Maximum Read Packet Size."]
        #[inline(always)]
        pub const fn mrdps(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Maximum Read Packet Size."]
        #[inline(always)]
        pub fn set_mrdps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Cmcr {
        #[inline(always)]
        fn default() -> Cmcr {
            Cmcr(0)
        }
    }
    impl core::fmt::Debug for Cmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmcr")
                .field("teare", &self.teare())
                .field("are", &self.are())
                .field("gsw0tx", &self.gsw0tx())
                .field("gsw1tx", &self.gsw1tx())
                .field("gsw2tx", &self.gsw2tx())
                .field("gsr0tx", &self.gsr0tx())
                .field("gsr1tx", &self.gsr1tx())
                .field("gsr2tx", &self.gsr2tx())
                .field("glwtx", &self.glwtx())
                .field("dsw0tx", &self.dsw0tx())
                .field("dsw1tx", &self.dsw1tx())
                .field("dsr0tx", &self.dsr0tx())
                .field("dlwtx", &self.dlwtx())
                .field("mrdps", &self.mrdps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmcr {{ teare: {=bool:?}, are: {=bool:?}, gsw0tx: {=bool:?}, gsw1tx: {=bool:?}, gsw2tx: {=bool:?}, gsr0tx: {=bool:?}, gsr1tx: {=bool:?}, gsr2tx: {=bool:?}, glwtx: {=bool:?}, dsw0tx: {=bool:?}, dsw1tx: {=bool:?}, dsr0tx: {=bool:?}, dlwtx: {=bool:?}, mrdps: {=bool:?} }}" , self . teare () , self . are () , self . gsw0tx () , self . gsw1tx () , self . gsw2tx () , self . gsr0tx () , self . gsr1tx () , self . gsr2tx () , self . glwtx () , self . dsw0tx () , self . dsw1tx () , self . dsr0tx () , self . dlwtx () , self . mrdps ())
        }
    }
    #[doc = "DSI Host Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "DSI Host Data Lane Timer Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dltcr(pub u32);
    impl Dltcr {
        #[doc = "Maximum Read Time."]
        #[inline(always)]
        pub const fn mrd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Maximum Read Time."]
        #[inline(always)]
        pub fn set_mrd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Low-Power To High-Speed Time."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Low-Power To High-Speed Time."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "High-Speed To Low-Power Time."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "High-Speed To Low-Power Time."]
        #[inline(always)]
        pub fn set_hs2lp_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dltcr {
        #[inline(always)]
        fn default() -> Dltcr {
            Dltcr(0)
        }
    }
    impl core::fmt::Debug for Dltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dltcr")
                .field("mrd_time", &self.mrd_time())
                .field("lp2hs_time", &self.lp2hs_time())
                .field("hs2lp_time", &self.hs2lp_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dltcr {{ mrd_time: {=u16:?}, lp2hs_time: {=u8:?}, hs2lp_time: {=u8:?} }}",
                self.mrd_time(),
                self.lp2hs_time(),
                self.hs2lp_time()
            )
        }
    }
    #[doc = "DSI Host Force Interrupt Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir0(pub u32);
    impl Fir0 {
        #[doc = "Force Acknowledge Error 0."]
        #[inline(always)]
        pub const fn fae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 0."]
        #[inline(always)]
        pub fn set_fae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force Acknowledge Error 1."]
        #[inline(always)]
        pub const fn fae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 1."]
        #[inline(always)]
        pub fn set_fae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force Acknowledge Error 2."]
        #[inline(always)]
        pub const fn fae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 2."]
        #[inline(always)]
        pub fn set_fae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force Acknowledge Error 3."]
        #[inline(always)]
        pub const fn fae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 3."]
        #[inline(always)]
        pub fn set_fae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force Acknowledge Error 4."]
        #[inline(always)]
        pub const fn fae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 4."]
        #[inline(always)]
        pub fn set_fae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force Acknowledge Error 5."]
        #[inline(always)]
        pub const fn fae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 5."]
        #[inline(always)]
        pub fn set_fae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force Acknowledge Error 6."]
        #[inline(always)]
        pub const fn fae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 6."]
        #[inline(always)]
        pub fn set_fae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force Acknowledge Error 7."]
        #[inline(always)]
        pub const fn fae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 7."]
        #[inline(always)]
        pub fn set_fae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force Acknowledge Error 8."]
        #[inline(always)]
        pub const fn fae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 8."]
        #[inline(always)]
        pub fn set_fae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force Acknowledge Error 9."]
        #[inline(always)]
        pub const fn fae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 9."]
        #[inline(always)]
        pub fn set_fae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force Acknowledge Error 10."]
        #[inline(always)]
        pub const fn fae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 10."]
        #[inline(always)]
        pub fn set_fae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force Acknowledge Error 11."]
        #[inline(always)]
        pub const fn fae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 11."]
        #[inline(always)]
        pub fn set_fae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force Acknowledge Error 12."]
        #[inline(always)]
        pub const fn fae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 12."]
        #[inline(always)]
        pub fn set_fae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force Acknowledge Error 13."]
        #[inline(always)]
        pub const fn fae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 13."]
        #[inline(always)]
        pub fn set_fae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Force Acknowledge Error 14."]
        #[inline(always)]
        pub const fn fae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 14."]
        #[inline(always)]
        pub fn set_fae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Force Acknowledge Error 15."]
        #[inline(always)]
        pub const fn fae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Force Acknowledge Error 15."]
        #[inline(always)]
        pub fn set_fae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Force PHY Error 0."]
        #[inline(always)]
        pub const fn fpe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY Error 0."]
        #[inline(always)]
        pub fn set_fpe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Force PHY Error 1."]
        #[inline(always)]
        pub const fn fpe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY Error 1."]
        #[inline(always)]
        pub fn set_fpe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Force PHY Error 2."]
        #[inline(always)]
        pub const fn fpe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY Error 2."]
        #[inline(always)]
        pub fn set_fpe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Force PHY Error 3."]
        #[inline(always)]
        pub const fn fpe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY Error 3."]
        #[inline(always)]
        pub fn set_fpe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Force PHY Error 4."]
        #[inline(always)]
        pub const fn fpe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY Error 4."]
        #[inline(always)]
        pub fn set_fpe4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Fir0 {
        #[inline(always)]
        fn default() -> Fir0 {
            Fir0(0)
        }
    }
    impl core::fmt::Debug for Fir0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fir0")
                .field("fae0", &self.fae0())
                .field("fae1", &self.fae1())
                .field("fae2", &self.fae2())
                .field("fae3", &self.fae3())
                .field("fae4", &self.fae4())
                .field("fae5", &self.fae5())
                .field("fae6", &self.fae6())
                .field("fae7", &self.fae7())
                .field("fae8", &self.fae8())
                .field("fae9", &self.fae9())
                .field("fae10", &self.fae10())
                .field("fae11", &self.fae11())
                .field("fae12", &self.fae12())
                .field("fae13", &self.fae13())
                .field("fae14", &self.fae14())
                .field("fae15", &self.fae15())
                .field("fpe0", &self.fpe0())
                .field("fpe1", &self.fpe1())
                .field("fpe2", &self.fpe2())
                .field("fpe3", &self.fpe3())
                .field("fpe4", &self.fpe4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fir0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fir0 {{ fae0: {=bool:?}, fae1: {=bool:?}, fae2: {=bool:?}, fae3: {=bool:?}, fae4: {=bool:?}, fae5: {=bool:?}, fae6: {=bool:?}, fae7: {=bool:?}, fae8: {=bool:?}, fae9: {=bool:?}, fae10: {=bool:?}, fae11: {=bool:?}, fae12: {=bool:?}, fae13: {=bool:?}, fae14: {=bool:?}, fae15: {=bool:?}, fpe0: {=bool:?}, fpe1: {=bool:?}, fpe2: {=bool:?}, fpe3: {=bool:?}, fpe4: {=bool:?} }}" , self . fae0 () , self . fae1 () , self . fae2 () , self . fae3 () , self . fae4 () , self . fae5 () , self . fae6 () , self . fae7 () , self . fae8 () , self . fae9 () , self . fae10 () , self . fae11 () , self . fae12 () , self . fae13 () , self . fae14 () , self . fae15 () , self . fpe0 () , self . fpe1 () , self . fpe2 () , self . fpe3 () , self . fpe4 ())
        }
    }
    #[doc = "DSI Host Force Interrupt Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir1(pub u32);
    impl Fir1 {
        #[doc = "Force Timeout High-Speed Transmission."]
        #[inline(always)]
        pub const fn ftohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force Timeout High-Speed Transmission."]
        #[inline(always)]
        pub fn set_ftohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force Timeout Low-Power Reception."]
        #[inline(always)]
        pub const fn ftolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force Timeout Low-Power Reception."]
        #[inline(always)]
        pub fn set_ftolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force ECC Single-bit Error."]
        #[inline(always)]
        pub const fn feccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC Single-bit Error."]
        #[inline(always)]
        pub fn set_feccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force ECC Multi-bit Error."]
        #[inline(always)]
        pub const fn feccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC Multi-bit Error."]
        #[inline(always)]
        pub fn set_feccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force CRC Error."]
        #[inline(always)]
        pub const fn fcrce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force CRC Error."]
        #[inline(always)]
        pub fn set_fcrce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force Packet Size Error."]
        #[inline(always)]
        pub const fn fpse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force Packet Size Error."]
        #[inline(always)]
        pub fn set_fpse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force EoTp Error."]
        #[inline(always)]
        pub const fn feotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force EoTp Error."]
        #[inline(always)]
        pub fn set_feotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force LTDC Payload Write Error."]
        #[inline(always)]
        pub const fn flpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force LTDC Payload Write Error."]
        #[inline(always)]
        pub fn set_flpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force Generic Command Write Error."]
        #[inline(always)]
        pub const fn fgcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force Generic Command Write Error."]
        #[inline(always)]
        pub fn set_fgcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force Generic Payload Write Error."]
        #[inline(always)]
        pub const fn fgpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force Generic Payload Write Error."]
        #[inline(always)]
        pub fn set_fgpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force Generic Payload Transmit Error."]
        #[inline(always)]
        pub const fn fgptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force Generic Payload Transmit Error."]
        #[inline(always)]
        pub fn set_fgptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force Generic Payload Read Error."]
        #[inline(always)]
        pub const fn fgprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force Generic Payload Read Error."]
        #[inline(always)]
        pub fn set_fgprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force Generic Payload Receive Error."]
        #[inline(always)]
        pub const fn fgprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force Generic Payload Receive Error."]
        #[inline(always)]
        pub fn set_fgprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Fir1 {
        #[inline(always)]
        fn default() -> Fir1 {
            Fir1(0)
        }
    }
    impl core::fmt::Debug for Fir1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fir1")
                .field("ftohstx", &self.ftohstx())
                .field("ftolprx", &self.ftolprx())
                .field("feccse", &self.feccse())
                .field("feccme", &self.feccme())
                .field("fcrce", &self.fcrce())
                .field("fpse", &self.fpse())
                .field("feotpe", &self.feotpe())
                .field("flpwre", &self.flpwre())
                .field("fgcwre", &self.fgcwre())
                .field("fgpwre", &self.fgpwre())
                .field("fgptxe", &self.fgptxe())
                .field("fgprde", &self.fgprde())
                .field("fgprxe", &self.fgprxe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fir1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fir1 {{ ftohstx: {=bool:?}, ftolprx: {=bool:?}, feccse: {=bool:?}, feccme: {=bool:?}, fcrce: {=bool:?}, fpse: {=bool:?}, feotpe: {=bool:?}, flpwre: {=bool:?}, fgcwre: {=bool:?}, fgpwre: {=bool:?}, fgptxe: {=bool:?}, fgprde: {=bool:?}, fgprxe: {=bool:?} }}" , self . ftohstx () , self . ftolprx () , self . feccse () , self . feccme () , self . fcrce () , self . fpse () , self . feotpe () , self . flpwre () , self . fgcwre () , self . fgpwre () , self . fgptxe () , self . fgprde () , self . fgprxe ())
        }
    }
    #[doc = "DSI Host Generic Header Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ghcr(pub u32);
    impl Ghcr {
        #[doc = "Type."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Type."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "WordCount LSB."]
        #[inline(always)]
        pub const fn wclsb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount LSB."]
        #[inline(always)]
        pub fn set_wclsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "WordCount MSB."]
        #[inline(always)]
        pub const fn wcmsb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount MSB."]
        #[inline(always)]
        pub fn set_wcmsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ghcr {
        #[inline(always)]
        fn default() -> Ghcr {
            Ghcr(0)
        }
    }
    impl core::fmt::Debug for Ghcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ghcr")
                .field("dt", &self.dt())
                .field("vcid", &self.vcid())
                .field("wclsb", &self.wclsb())
                .field("wcmsb", &self.wcmsb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ghcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ghcr {{ dt: {=u8:?}, vcid: {=u8:?}, wclsb: {=u8:?}, wcmsb: {=u8:?} }}",
                self.dt(),
                self.vcid(),
                self.wclsb(),
                self.wcmsb()
            )
        }
    }
    #[doc = "DSI Host Generic Payload Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpdr(pub u32);
    impl Gpdr {
        #[doc = "Payload Byte 1."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Payload Byte 1."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Payload Byte 2."]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Payload Byte 2."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Payload Byte 3."]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Payload Byte 3."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Payload Byte 4."]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Payload Byte 4."]
        #[inline(always)]
        pub fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Gpdr {
        #[inline(always)]
        fn default() -> Gpdr {
            Gpdr(0)
        }
    }
    impl core::fmt::Debug for Gpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpdr")
                .field("data1", &self.data1())
                .field("data2", &self.data2())
                .field("data3", &self.data3())
                .field("data4", &self.data4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gpdr {{ data1: {=u8:?}, data2: {=u8:?}, data3: {=u8:?}, data4: {=u8:?} }}",
                self.data1(),
                self.data2(),
                self.data3(),
                self.data4()
            )
        }
    }
    #[doc = "DSI Host Generic Packet Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpsr(pub u32);
    impl Gpsr {
        #[doc = "Command FIFO Empty."]
        #[inline(always)]
        pub const fn cmdfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO Empty."]
        #[inline(always)]
        pub fn set_cmdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command FIFO Full."]
        #[inline(always)]
        pub const fn cmdff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO Full."]
        #[inline(always)]
        pub fn set_cmdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Payload Write FIFO Empty."]
        #[inline(always)]
        pub const fn pwrfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Payload Write FIFO Empty."]
        #[inline(always)]
        pub fn set_pwrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Payload Write FIFO Full."]
        #[inline(always)]
        pub const fn pwrff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Payload Write FIFO Full."]
        #[inline(always)]
        pub fn set_pwrff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Payload Read FIFO Empty."]
        #[inline(always)]
        pub const fn prdfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Payload Read FIFO Empty."]
        #[inline(always)]
        pub fn set_prdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Payload Read FIFO Full."]
        #[inline(always)]
        pub const fn prdff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Payload Read FIFO Full."]
        #[inline(always)]
        pub fn set_prdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Read Command Busy."]
        #[inline(always)]
        pub const fn rcb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read Command Busy."]
        #[inline(always)]
        pub fn set_rcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Gpsr {
        #[inline(always)]
        fn default() -> Gpsr {
            Gpsr(0)
        }
    }
    impl core::fmt::Debug for Gpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpsr")
                .field("cmdfe", &self.cmdfe())
                .field("cmdff", &self.cmdff())
                .field("pwrfe", &self.pwrfe())
                .field("pwrff", &self.pwrff())
                .field("prdfe", &self.prdfe())
                .field("prdff", &self.prdff())
                .field("rcb", &self.rcb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gpsr {{ cmdfe: {=bool:?}, cmdff: {=bool:?}, pwrfe: {=bool:?}, pwrff: {=bool:?}, prdfe: {=bool:?}, prdff: {=bool:?}, rcb: {=bool:?} }}" , self . cmdfe () , self . cmdff () , self . pwrfe () , self . pwrff () , self . prdfe () , self . prdff () , self . rcb ())
        }
    }
    #[doc = "DSI Host Generic VCID Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gvcidr(pub u32);
    impl Gvcidr {
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Gvcidr {
        #[inline(always)]
        fn default() -> Gvcidr {
            Gvcidr(0)
        }
    }
    impl core::fmt::Debug for Gvcidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gvcidr").field("vcid", &self.vcid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gvcidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gvcidr {{ vcid: {=u8:?} }}", self.vcid())
        }
    }
    #[doc = "DSI Host Interrupt Enable Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier0(pub u32);
    impl Ier0 {
        #[doc = "Acknowledge Error 0 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae0ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 0 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge Error 1 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 1 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge Error 2 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 2 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge Error 3 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 3 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge Error 4 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 4 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge Error 5 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae5ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 5 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge Error 6 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae6ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 6 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge Error 7 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae7ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 7 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge Error 8 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae8ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 8 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge Error 9 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae9ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 9 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge Error 10 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae10ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 10 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae10ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge Error 11 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae11ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 11 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge Error 12 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae12ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 12 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae12ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge Error 13 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae13ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 13 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae13ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge Error 14 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae14ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 14 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae14ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge Error 15 Interrupt Enable."]
        #[inline(always)]
        pub const fn ae15ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 15 Interrupt Enable."]
        #[inline(always)]
        pub fn set_ae15ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY Error 0 Interrupt Enable."]
        #[inline(always)]
        pub const fn pe0ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 0 Interrupt Enable."]
        #[inline(always)]
        pub fn set_pe0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY Error 1 Interrupt Enable."]
        #[inline(always)]
        pub const fn pe1ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 1 Interrupt Enable."]
        #[inline(always)]
        pub fn set_pe1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY Error 2 Interrupt Enable."]
        #[inline(always)]
        pub const fn pe2ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 2 Interrupt Enable."]
        #[inline(always)]
        pub fn set_pe2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY Error 3 Interrupt Enable."]
        #[inline(always)]
        pub const fn pe3ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 3 Interrupt Enable."]
        #[inline(always)]
        pub fn set_pe3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY Error 4 Interrupt Enable."]
        #[inline(always)]
        pub const fn pe4ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 4 Interrupt Enable."]
        #[inline(always)]
        pub fn set_pe4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ier0 {
        #[inline(always)]
        fn default() -> Ier0 {
            Ier0(0)
        }
    }
    impl core::fmt::Debug for Ier0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier0")
                .field("ae0ie", &self.ae0ie())
                .field("ae1ie", &self.ae1ie())
                .field("ae2ie", &self.ae2ie())
                .field("ae3ie", &self.ae3ie())
                .field("ae4ie", &self.ae4ie())
                .field("ae5ie", &self.ae5ie())
                .field("ae6ie", &self.ae6ie())
                .field("ae7ie", &self.ae7ie())
                .field("ae8ie", &self.ae8ie())
                .field("ae9ie", &self.ae9ie())
                .field("ae10ie", &self.ae10ie())
                .field("ae11ie", &self.ae11ie())
                .field("ae12ie", &self.ae12ie())
                .field("ae13ie", &self.ae13ie())
                .field("ae14ie", &self.ae14ie())
                .field("ae15ie", &self.ae15ie())
                .field("pe0ie", &self.pe0ie())
                .field("pe1ie", &self.pe1ie())
                .field("pe2ie", &self.pe2ie())
                .field("pe3ie", &self.pe3ie())
                .field("pe4ie", &self.pe4ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier0 {{ ae0ie: {=bool:?}, ae1ie: {=bool:?}, ae2ie: {=bool:?}, ae3ie: {=bool:?}, ae4ie: {=bool:?}, ae5ie: {=bool:?}, ae6ie: {=bool:?}, ae7ie: {=bool:?}, ae8ie: {=bool:?}, ae9ie: {=bool:?}, ae10ie: {=bool:?}, ae11ie: {=bool:?}, ae12ie: {=bool:?}, ae13ie: {=bool:?}, ae14ie: {=bool:?}, ae15ie: {=bool:?}, pe0ie: {=bool:?}, pe1ie: {=bool:?}, pe2ie: {=bool:?}, pe3ie: {=bool:?}, pe4ie: {=bool:?} }}" , self . ae0ie () , self . ae1ie () , self . ae2ie () , self . ae3ie () , self . ae4ie () , self . ae5ie () , self . ae6ie () , self . ae7ie () , self . ae8ie () , self . ae9ie () , self . ae10ie () , self . ae11ie () , self . ae12ie () , self . ae13ie () , self . ae14ie () , self . ae15ie () , self . pe0ie () , self . pe1ie () , self . pe2ie () , self . pe3ie () , self . pe4ie ())
        }
    }
    #[doc = "DSI Host Interrupt Enable Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier1(pub u32);
    impl Ier1 {
        #[doc = "Timeout High-Speed Transmission Interrupt Enable."]
        #[inline(always)]
        pub const fn tohstxie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout High-Speed Transmission Interrupt Enable."]
        #[inline(always)]
        pub fn set_tohstxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout Low-Power Reception Interrupt Enable."]
        #[inline(always)]
        pub const fn tolprxie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Low-Power Reception Interrupt Enable."]
        #[inline(always)]
        pub fn set_tolprxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC Single-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn eccseie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Single-bit Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_eccseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Multi-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn eccmeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Multi-bit Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_eccmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC Error Interrupt Enable."]
        #[inline(always)]
        pub const fn crceie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_crceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet Size Error Interrupt Enable."]
        #[inline(always)]
        pub const fn pseie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet Size Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_pseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp Error Interrupt Enable."]
        #[inline(always)]
        pub const fn eotpeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_eotpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC Payload Write Error Interrupt Enable."]
        #[inline(always)]
        pub const fn lpwreie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC Payload Write Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_lpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic Command Write Error Interrupt Enable."]
        #[inline(always)]
        pub const fn gcwreie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Command Write Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_gcwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic Payload Write Error Interrupt Enable."]
        #[inline(always)]
        pub const fn gpwreie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Write Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_gpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic Payload Transmit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn gptxeie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Transmit Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_gptxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic Payload Read Error Interrupt Enable."]
        #[inline(always)]
        pub const fn gprdeie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Read Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_gprdeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic Payload Receive Error Interrupt Enable."]
        #[inline(always)]
        pub const fn gprxeie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Receive Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_gprxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ier1 {
        #[inline(always)]
        fn default() -> Ier1 {
            Ier1(0)
        }
    }
    impl core::fmt::Debug for Ier1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier1")
                .field("tohstxie", &self.tohstxie())
                .field("tolprxie", &self.tolprxie())
                .field("eccseie", &self.eccseie())
                .field("eccmeie", &self.eccmeie())
                .field("crceie", &self.crceie())
                .field("pseie", &self.pseie())
                .field("eotpeie", &self.eotpeie())
                .field("lpwreie", &self.lpwreie())
                .field("gcwreie", &self.gcwreie())
                .field("gpwreie", &self.gpwreie())
                .field("gptxeie", &self.gptxeie())
                .field("gprdeie", &self.gprdeie())
                .field("gprxeie", &self.gprxeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier1 {{ tohstxie: {=bool:?}, tolprxie: {=bool:?}, eccseie: {=bool:?}, eccmeie: {=bool:?}, crceie: {=bool:?}, pseie: {=bool:?}, eotpeie: {=bool:?}, lpwreie: {=bool:?}, gcwreie: {=bool:?}, gpwreie: {=bool:?}, gptxeie: {=bool:?}, gprdeie: {=bool:?}, gprxeie: {=bool:?} }}" , self . tohstxie () , self . tolprxie () , self . eccseie () , self . eccmeie () , self . crceie () , self . pseie () , self . eotpeie () , self . lpwreie () , self . gcwreie () , self . gpwreie () , self . gptxeie () , self . gprdeie () , self . gprxeie ())
        }
    }
    #[doc = "DSI Host Interrupt & Status Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr0(pub u32);
    impl Isr0 {
        #[doc = "Acknowledge Error 0."]
        #[inline(always)]
        pub const fn ae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 0."]
        #[inline(always)]
        pub fn set_ae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge Error 1."]
        #[inline(always)]
        pub const fn ae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 1."]
        #[inline(always)]
        pub fn set_ae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge Error 2."]
        #[inline(always)]
        pub const fn ae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 2."]
        #[inline(always)]
        pub fn set_ae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge Error 3."]
        #[inline(always)]
        pub const fn ae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 3."]
        #[inline(always)]
        pub fn set_ae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge Error 4."]
        #[inline(always)]
        pub const fn ae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 4."]
        #[inline(always)]
        pub fn set_ae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge Error 5."]
        #[inline(always)]
        pub const fn ae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 5."]
        #[inline(always)]
        pub fn set_ae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge Error 6."]
        #[inline(always)]
        pub const fn ae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 6."]
        #[inline(always)]
        pub fn set_ae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge Error 7."]
        #[inline(always)]
        pub const fn ae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 7."]
        #[inline(always)]
        pub fn set_ae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge Error 8."]
        #[inline(always)]
        pub const fn ae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 8."]
        #[inline(always)]
        pub fn set_ae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge Error 9."]
        #[inline(always)]
        pub const fn ae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 9."]
        #[inline(always)]
        pub fn set_ae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge Error 10."]
        #[inline(always)]
        pub const fn ae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 10."]
        #[inline(always)]
        pub fn set_ae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge Error 11."]
        #[inline(always)]
        pub const fn ae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 11."]
        #[inline(always)]
        pub fn set_ae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge Error 12."]
        #[inline(always)]
        pub const fn ae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 12."]
        #[inline(always)]
        pub fn set_ae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge Error 13."]
        #[inline(always)]
        pub const fn ae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 13."]
        #[inline(always)]
        pub fn set_ae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge Error 14."]
        #[inline(always)]
        pub const fn ae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 14."]
        #[inline(always)]
        pub fn set_ae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge Error 15."]
        #[inline(always)]
        pub const fn ae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge Error 15."]
        #[inline(always)]
        pub fn set_ae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY Error 0."]
        #[inline(always)]
        pub const fn pe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 0."]
        #[inline(always)]
        pub fn set_pe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY Error 1."]
        #[inline(always)]
        pub const fn pe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 1."]
        #[inline(always)]
        pub fn set_pe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY Error 2."]
        #[inline(always)]
        pub const fn pe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 2."]
        #[inline(always)]
        pub fn set_pe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY Error 3."]
        #[inline(always)]
        pub const fn pe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 3."]
        #[inline(always)]
        pub fn set_pe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY Error 4."]
        #[inline(always)]
        pub const fn pe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Error 4."]
        #[inline(always)]
        pub fn set_pe4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Isr0 {
        #[inline(always)]
        fn default() -> Isr0 {
            Isr0(0)
        }
    }
    impl core::fmt::Debug for Isr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr0")
                .field("ae0", &self.ae0())
                .field("ae1", &self.ae1())
                .field("ae2", &self.ae2())
                .field("ae3", &self.ae3())
                .field("ae4", &self.ae4())
                .field("ae5", &self.ae5())
                .field("ae6", &self.ae6())
                .field("ae7", &self.ae7())
                .field("ae8", &self.ae8())
                .field("ae9", &self.ae9())
                .field("ae10", &self.ae10())
                .field("ae11", &self.ae11())
                .field("ae12", &self.ae12())
                .field("ae13", &self.ae13())
                .field("ae14", &self.ae14())
                .field("ae15", &self.ae15())
                .field("pe0", &self.pe0())
                .field("pe1", &self.pe1())
                .field("pe2", &self.pe2())
                .field("pe3", &self.pe3())
                .field("pe4", &self.pe4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr0 {{ ae0: {=bool:?}, ae1: {=bool:?}, ae2: {=bool:?}, ae3: {=bool:?}, ae4: {=bool:?}, ae5: {=bool:?}, ae6: {=bool:?}, ae7: {=bool:?}, ae8: {=bool:?}, ae9: {=bool:?}, ae10: {=bool:?}, ae11: {=bool:?}, ae12: {=bool:?}, ae13: {=bool:?}, ae14: {=bool:?}, ae15: {=bool:?}, pe0: {=bool:?}, pe1: {=bool:?}, pe2: {=bool:?}, pe3: {=bool:?}, pe4: {=bool:?} }}" , self . ae0 () , self . ae1 () , self . ae2 () , self . ae3 () , self . ae4 () , self . ae5 () , self . ae6 () , self . ae7 () , self . ae8 () , self . ae9 () , self . ae10 () , self . ae11 () , self . ae12 () , self . ae13 () , self . ae14 () , self . ae15 () , self . pe0 () , self . pe1 () , self . pe2 () , self . pe3 () , self . pe4 ())
        }
    }
    #[doc = "DSI Host Interrupt & Status Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Timeout High-Speed Transmission."]
        #[inline(always)]
        pub const fn tohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout High-Speed Transmission."]
        #[inline(always)]
        pub fn set_tohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout Low-Power Reception."]
        #[inline(always)]
        pub const fn tolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Low-Power Reception."]
        #[inline(always)]
        pub fn set_tolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC Single-bit Error."]
        #[inline(always)]
        pub const fn eccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Single-bit Error."]
        #[inline(always)]
        pub fn set_eccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Multi-bit Error."]
        #[inline(always)]
        pub const fn eccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Multi-bit Error."]
        #[inline(always)]
        pub fn set_eccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC Error."]
        #[inline(always)]
        pub const fn crce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Error."]
        #[inline(always)]
        pub fn set_crce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet Size Error."]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet Size Error."]
        #[inline(always)]
        pub fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp Error."]
        #[inline(always)]
        pub const fn eotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp Error."]
        #[inline(always)]
        pub fn set_eotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC Payload Write Error."]
        #[inline(always)]
        pub const fn lpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC Payload Write Error."]
        #[inline(always)]
        pub fn set_lpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic Command Write Error."]
        #[inline(always)]
        pub const fn gcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Command Write Error."]
        #[inline(always)]
        pub fn set_gcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic Payload Write Error."]
        #[inline(always)]
        pub const fn gpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Write Error."]
        #[inline(always)]
        pub fn set_gpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic Payload Transmit Error."]
        #[inline(always)]
        pub const fn gptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Transmit Error."]
        #[inline(always)]
        pub fn set_gptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic Payload Read Error."]
        #[inline(always)]
        pub const fn gprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Read Error."]
        #[inline(always)]
        pub fn set_gprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic Payload Receive Error."]
        #[inline(always)]
        pub const fn gprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic Payload Receive Error."]
        #[inline(always)]
        pub fn set_gprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Isr1 {
        #[inline(always)]
        fn default() -> Isr1 {
            Isr1(0)
        }
    }
    impl core::fmt::Debug for Isr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr1")
                .field("tohstx", &self.tohstx())
                .field("tolprx", &self.tolprx())
                .field("eccse", &self.eccse())
                .field("eccme", &self.eccme())
                .field("crce", &self.crce())
                .field("pse", &self.pse())
                .field("eotpe", &self.eotpe())
                .field("lpwre", &self.lpwre())
                .field("gcwre", &self.gcwre())
                .field("gpwre", &self.gpwre())
                .field("gptxe", &self.gptxe())
                .field("gprde", &self.gprde())
                .field("gprxe", &self.gprxe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr1 {{ tohstx: {=bool:?}, tolprx: {=bool:?}, eccse: {=bool:?}, eccme: {=bool:?}, crce: {=bool:?}, pse: {=bool:?}, eotpe: {=bool:?}, lpwre: {=bool:?}, gcwre: {=bool:?}, gpwre: {=bool:?}, gptxe: {=bool:?}, gprde: {=bool:?}, gprxe: {=bool:?} }}" , self . tohstx () , self . tolprx () , self . eccse () , self . eccme () , self . crce () , self . pse () , self . eotpe () , self . lpwre () , self . gcwre () , self . gpwre () , self . gptxe () , self . gprde () , self . gprxe ())
        }
    }
    #[doc = "DSI Host LTDC Current Color Coding Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcccr(pub u32);
    impl Lcccr {
        #[doc = "Color Coding."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color Coding."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely Packed Enable."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely Packed Enable."]
        #[inline(always)]
        pub fn set_lpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Lcccr {
        #[inline(always)]
        fn default() -> Lcccr {
            Lcccr(0)
        }
    }
    impl core::fmt::Debug for Lcccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcccr")
                .field("colc", &self.colc())
                .field("lpe", &self.lpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lcccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lcccr {{ colc: {=u8:?}, lpe: {=bool:?} }}", self.colc(), self.lpe())
        }
    }
    #[doc = "DSI Host LTDC Command Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lccr(pub u32);
    impl Lccr {
        #[doc = "Command Size."]
        #[inline(always)]
        pub const fn cmdsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Command Size."]
        #[inline(always)]
        pub fn set_cmdsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lccr {
        #[inline(always)]
        fn default() -> Lccr {
            Lccr(0)
        }
    }
    impl core::fmt::Debug for Lccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lccr").field("cmdsize", &self.cmdsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lccr {{ cmdsize: {=u16:?} }}", self.cmdsize())
        }
    }
    #[doc = "DSI Host LTDC Color Coding Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcolcr(pub u32);
    impl Lcolcr {
        #[doc = "Color Coding."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color Coding."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely Packet Enable."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely Packet Enable."]
        #[inline(always)]
        pub fn set_lpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Lcolcr {
        #[inline(always)]
        fn default() -> Lcolcr {
            Lcolcr(0)
        }
    }
    impl core::fmt::Debug for Lcolcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcolcr")
                .field("colc", &self.colc())
                .field("lpe", &self.lpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lcolcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lcolcr {{ colc: {=u8:?}, lpe: {=bool:?} }}", self.colc(), self.lpe())
        }
    }
    #[doc = "DSI Host LTDC Current VCID Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcvcidr(pub u32);
    impl Lcvcidr {
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lcvcidr {
        #[inline(always)]
        fn default() -> Lcvcidr {
            Lcvcidr(0)
        }
    }
    impl core::fmt::Debug for Lcvcidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcvcidr").field("vcid", &self.vcid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lcvcidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lcvcidr {{ vcid: {=u8:?} }}", self.vcid())
        }
    }
    #[doc = "DSI Host LTDC Polarity Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpcr(pub u32);
    impl Lpcr {
        #[doc = "Data Enable Polarity."]
        #[inline(always)]
        pub const fn dep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Enable Polarity."]
        #[inline(always)]
        pub fn set_dep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VSYNC Polarity."]
        #[inline(always)]
        pub const fn vsp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC Polarity."]
        #[inline(always)]
        pub fn set_vsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSYNC Polarity."]
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSYNC Polarity."]
        #[inline(always)]
        pub fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Lpcr {
        #[inline(always)]
        fn default() -> Lpcr {
            Lpcr(0)
        }
    }
    impl core::fmt::Debug for Lpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpcr")
                .field("dep", &self.dep())
                .field("vsp", &self.vsp())
                .field("hsp", &self.hsp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpcr {{ dep: {=bool:?}, vsp: {=bool:?}, hsp: {=bool:?} }}",
                self.dep(),
                self.vsp(),
                self.hsp()
            )
        }
    }
    #[doc = "DSI Host Low-Power mode Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmccr(pub u32);
    impl Lpmccr {
        #[doc = "VACT Largest Packet Size."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT Largest Packet Size."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest Packet Size."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest Packet Size."]
        #[inline(always)]
        pub fn set_lpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lpmccr {
        #[inline(always)]
        fn default() -> Lpmccr {
            Lpmccr(0)
        }
    }
    impl core::fmt::Debug for Lpmccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpmccr")
                .field("vlpsize", &self.vlpsize())
                .field("lpsize", &self.lpsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpmccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpmccr {{ vlpsize: {=u8:?}, lpsize: {=u8:?} }}",
                self.vlpsize(),
                self.lpsize()
            )
        }
    }
    #[doc = "DSI Host Low-Power mode Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmcr(pub u32);
    impl Lpmcr {
        #[doc = "VACT Largest Packet Size."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT Largest Packet Size."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest Packet Size."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest Packet Size."]
        #[inline(always)]
        pub fn set_lpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lpmcr {
        #[inline(always)]
        fn default() -> Lpmcr {
            Lpmcr(0)
        }
    }
    impl core::fmt::Debug for Lpmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpmcr")
                .field("vlpsize", &self.vlpsize())
                .field("lpsize", &self.lpsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpmcr {{ vlpsize: {=u8:?}, lpsize: {=u8:?} }}",
                self.vlpsize(),
                self.lpsize()
            )
        }
    }
    #[doc = "DSI Host LTDC VCID Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvcidr(pub u32);
    impl Lvcidr {
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual Channel ID."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lvcidr {
        #[inline(always)]
        fn default() -> Lvcidr {
            Lvcidr(0)
        }
    }
    impl core::fmt::Debug for Lvcidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lvcidr").field("vcid", &self.vcid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lvcidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lvcidr {{ vcid: {=u8:?} }}", self.vcid())
        }
    }
    #[doc = "DSI Host mode Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Command mode."]
        #[inline(always)]
        pub const fn cmdm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command mode."]
        #[inline(always)]
        pub fn set_cmdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    impl core::fmt::Debug for Mcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr").field("cmdm", &self.cmdm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mcr {{ cmdm: {=bool:?} }}", self.cmdm())
        }
    }
    #[doc = "DSI Host PHY Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pconfr(pub u32);
    impl Pconfr {
        #[doc = "Number of Lanes."]
        #[inline(always)]
        pub const fn nl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Number of Lanes."]
        #[inline(always)]
        pub fn set_nl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Stop Wait Time."]
        #[inline(always)]
        pub const fn sw_time(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stop Wait Time."]
        #[inline(always)]
        pub fn set_sw_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Pconfr {
        #[inline(always)]
        fn default() -> Pconfr {
            Pconfr(0)
        }
    }
    impl core::fmt::Debug for Pconfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pconfr")
                .field("nl", &self.nl())
                .field("sw_time", &self.sw_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pconfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pconfr {{ nl: {=u8:?}, sw_time: {=u8:?} }}",
                self.nl(),
                self.sw_time()
            )
        }
    }
    #[doc = "DSI Host Protocol Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "EoTp Transmission Enable."]
        #[inline(always)]
        pub const fn ettxe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp Transmission Enable."]
        #[inline(always)]
        pub fn set_ettxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EoTp Reception Enable."]
        #[inline(always)]
        pub const fn etrxe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp Reception Enable."]
        #[inline(always)]
        pub fn set_etrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bus Turn Around Enable."]
        #[inline(always)]
        pub const fn btae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Turn Around Enable."]
        #[inline(always)]
        pub fn set_btae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Reception Enable."]
        #[inline(always)]
        pub const fn eccrxe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Reception Enable."]
        #[inline(always)]
        pub fn set_eccrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC Reception Enable."]
        #[inline(always)]
        pub const fn crcrxe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Reception Enable."]
        #[inline(always)]
        pub fn set_crcrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("ettxe", &self.ettxe())
                .field("etrxe", &self.etrxe())
                .field("btae", &self.btae())
                .field("eccrxe", &self.eccrxe())
                .field("crcrxe", &self.crcrxe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcr {{ ettxe: {=bool:?}, etrxe: {=bool:?}, btae: {=bool:?}, eccrxe: {=bool:?}, crcrxe: {=bool:?} }}",
                self.ettxe(),
                self.etrxe(),
                self.btae(),
                self.eccrxe(),
                self.crcrxe()
            )
        }
    }
    #[doc = "DSI Host PHY Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pctlr(pub u32);
    impl Pctlr {
        #[doc = "Digital Enable."]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Enable."]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock Enable."]
        #[inline(always)]
        pub const fn cke(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Enable."]
        #[inline(always)]
        pub fn set_cke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Pctlr {
        #[inline(always)]
        fn default() -> Pctlr {
            Pctlr(0)
        }
    }
    impl core::fmt::Debug for Pctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pctlr")
                .field("den", &self.den())
                .field("cke", &self.cke())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pctlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pctlr {{ den: {=bool:?}, cke: {=bool:?} }}", self.den(), self.cke())
        }
    }
    #[doc = "DSI Host PHY Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "PHY Direction."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Direction."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PHY Stop State Clock lane."]
        #[inline(always)]
        pub const fn pssc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Stop State Clock lane."]
        #[inline(always)]
        pub fn set_pssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS Active Not Clock lane."]
        #[inline(always)]
        pub const fn uanc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Active Not Clock lane."]
        #[inline(always)]
        pub fn set_uanc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PHY Stop State lane 0."]
        #[inline(always)]
        pub const fn pss0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Stop State lane 0."]
        #[inline(always)]
        pub fn set_pss0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ULPS Active Not lane 1."]
        #[inline(always)]
        pub const fn uan0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Active Not lane 1."]
        #[inline(always)]
        pub fn set_uan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX ULPS Escape lane 0."]
        #[inline(always)]
        pub const fn rue0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX ULPS Escape lane 0."]
        #[inline(always)]
        pub fn set_rue0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PHY Stop State lane 1."]
        #[inline(always)]
        pub const fn pss1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Stop State lane 1."]
        #[inline(always)]
        pub fn set_pss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULPS Active Not lane 1."]
        #[inline(always)]
        pub const fn uan1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Active Not lane 1."]
        #[inline(always)]
        pub fn set_uan1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Psr {
        #[inline(always)]
        fn default() -> Psr {
            Psr(0)
        }
    }
    impl core::fmt::Debug for Psr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psr")
                .field("pd", &self.pd())
                .field("pssc", &self.pssc())
                .field("uanc", &self.uanc())
                .field("pss0", &self.pss0())
                .field("uan0", &self.uan0())
                .field("rue0", &self.rue0())
                .field("pss1", &self.pss1())
                .field("uan1", &self.uan1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Psr {{ pd: {=bool:?}, pssc: {=bool:?}, uanc: {=bool:?}, pss0: {=bool:?}, uan0: {=bool:?}, rue0: {=bool:?}, pss1: {=bool:?}, uan1: {=bool:?} }}" , self . pd () , self . pssc () , self . uanc () , self . pss0 () , self . uan0 () , self . rue0 () , self . pss1 () , self . uan1 ())
        }
    }
    #[doc = "DSI Host PHY TX Triggers Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pttcr(pub u32);
    impl Pttcr {
        #[doc = "Transmission Trigger."]
        #[inline(always)]
        pub const fn tx_trig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmission Trigger."]
        #[inline(always)]
        pub fn set_tx_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Pttcr {
        #[inline(always)]
        fn default() -> Pttcr {
            Pttcr(0)
        }
    }
    impl core::fmt::Debug for Pttcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pttcr").field("tx_trig", &self.tx_trig()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pttcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pttcr {{ tx_trig: {=u8:?} }}", self.tx_trig())
        }
    }
    #[doc = "DSI Host PHY ULPS Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucr(pub u32);
    impl Pucr {
        #[doc = "ULPS Request on Clock Lane."]
        #[inline(always)]
        pub const fn urcl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Request on Clock Lane."]
        #[inline(always)]
        pub fn set_urcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ULPS Exit on Clock Lane."]
        #[inline(always)]
        pub const fn uecl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Exit on Clock Lane."]
        #[inline(always)]
        pub fn set_uecl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ULPS Request on Data Lane."]
        #[inline(always)]
        pub const fn urdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Request on Data Lane."]
        #[inline(always)]
        pub fn set_urdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS Exit on Data Lane."]
        #[inline(always)]
        pub const fn uedl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS Exit on Data Lane."]
        #[inline(always)]
        pub fn set_uedl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pucr {
        #[inline(always)]
        fn default() -> Pucr {
            Pucr(0)
        }
    }
    impl core::fmt::Debug for Pucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pucr")
                .field("urcl", &self.urcl())
                .field("uecl", &self.uecl())
                .field("urdl", &self.urdl())
                .field("uedl", &self.uedl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pucr {{ urcl: {=bool:?}, uecl: {=bool:?}, urdl: {=bool:?}, uedl: {=bool:?} }}",
                self.urcl(),
                self.uecl(),
                self.urdl(),
                self.uedl()
            )
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr0(pub u32);
    impl Tccr0 {
        #[doc = "Low-power Reception Timeout Counter."]
        #[inline(always)]
        pub const fn lprx_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power Reception Timeout Counter."]
        #[inline(always)]
        pub fn set_lprx_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "High-Speed Transmission Timeout Counter."]
        #[inline(always)]
        pub const fn hstx_tocnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "High-Speed Transmission Timeout Counter."]
        #[inline(always)]
        pub fn set_hstx_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tccr0 {
        #[inline(always)]
        fn default() -> Tccr0 {
            Tccr0(0)
        }
    }
    impl core::fmt::Debug for Tccr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr0")
                .field("lprx_tocnt", &self.lprx_tocnt())
                .field("hstx_tocnt", &self.hstx_tocnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tccr0 {{ lprx_tocnt: {=u16:?}, hstx_tocnt: {=u16:?} }}",
                self.lprx_tocnt(),
                self.hstx_tocnt()
            )
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr1(pub u32);
    impl Tccr1 {
        #[doc = "High-Speed Read Timeout Counter."]
        #[inline(always)]
        pub const fn hsrd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-Speed Read Timeout Counter."]
        #[inline(always)]
        pub fn set_hsrd_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr1 {
        #[inline(always)]
        fn default() -> Tccr1 {
            Tccr1(0)
        }
    }
    impl core::fmt::Debug for Tccr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr1").field("hsrd_tocnt", &self.hsrd_tocnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tccr1 {{ hsrd_tocnt: {=u16:?} }}", self.hsrd_tocnt())
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr2(pub u32);
    impl Tccr2 {
        #[doc = "Low-Power Read Timeout Counter."]
        #[inline(always)]
        pub const fn lprd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-Power Read Timeout Counter."]
        #[inline(always)]
        pub fn set_lprd_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr2 {
        #[inline(always)]
        fn default() -> Tccr2 {
            Tccr2(0)
        }
    }
    impl core::fmt::Debug for Tccr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr2").field("lprd_tocnt", &self.lprd_tocnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tccr2 {{ lprd_tocnt: {=u16:?} }}", self.lprd_tocnt())
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr3(pub u32);
    impl Tccr3 {
        #[doc = "High-Speed Write Timeout Counter."]
        #[inline(always)]
        pub const fn hswr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-Speed Write Timeout Counter."]
        #[inline(always)]
        pub fn set_hswr_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Presp mode."]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Presp mode."]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Tccr3 {
        #[inline(always)]
        fn default() -> Tccr3 {
            Tccr3(0)
        }
    }
    impl core::fmt::Debug for Tccr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr3")
                .field("hswr_tocnt", &self.hswr_tocnt())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tccr3 {{ hswr_tocnt: {=u16:?}, pm: {=bool:?} }}",
                self.hswr_tocnt(),
                self.pm()
            )
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr4(pub u32);
    impl Tccr4 {
        #[doc = "Low-Power Write Timeout Counter."]
        #[inline(always)]
        pub const fn lswr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-Power Write Timeout Counter."]
        #[inline(always)]
        pub fn set_lswr_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr4 {
        #[inline(always)]
        fn default() -> Tccr4 {
            Tccr4(0)
        }
    }
    impl core::fmt::Debug for Tccr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr4").field("lswr_tocnt", &self.lswr_tocnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tccr4 {{ lswr_tocnt: {=u16:?} }}", self.lswr_tocnt())
        }
    }
    #[doc = "DSI Host Timeout Counter Configuration Register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr5(pub u32);
    impl Tccr5 {
        #[doc = "Bus-Turn-Around Timeout Counter."]
        #[inline(always)]
        pub const fn bta_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bus-Turn-Around Timeout Counter."]
        #[inline(always)]
        pub fn set_bta_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr5 {
        #[inline(always)]
        fn default() -> Tccr5 {
            Tccr5(0)
        }
    }
    impl core::fmt::Debug for Tccr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tccr5").field("bta_tocnt", &self.bta_tocnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tccr5 {{ bta_tocnt: {=u16:?} }}", self.bta_tocnt())
        }
    }
    #[doc = "DSI Host Video Chunks Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vcccr(pub u32);
    impl Vcccr {
        #[doc = "Number of Chunks."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Chunks."]
        #[inline(always)]
        pub fn set_numc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vcccr {
        #[inline(always)]
        fn default() -> Vcccr {
            Vcccr(0)
        }
    }
    impl core::fmt::Debug for Vcccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vcccr").field("numc", &self.numc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vcccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vcccr {{ numc: {=u16:?} }}", self.numc())
        }
    }
    #[doc = "DSI Host Video Chunks Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vccr(pub u32);
    impl Vccr {
        #[doc = "Number of Chunks."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Chunks."]
        #[inline(always)]
        pub fn set_numc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vccr {
        #[inline(always)]
        fn default() -> Vccr {
            Vccr(0)
        }
    }
    impl core::fmt::Debug for Vccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vccr").field("numc", &self.numc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vccr {{ numc: {=u16:?} }}", self.numc())
        }
    }
    #[doc = "DSI Host Video HBP Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpccr(pub u32);
    impl Vhbpccr {
        #[doc = "Horizontal Back-Porch duration."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Back-Porch duration."]
        #[inline(always)]
        pub fn set_hbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhbpccr {
        #[inline(always)]
        fn default() -> Vhbpccr {
            Vhbpccr(0)
        }
    }
    impl core::fmt::Debug for Vhbpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vhbpccr").field("hbp", &self.hbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vhbpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vhbpccr {{ hbp: {=u16:?} }}", self.hbp())
        }
    }
    #[doc = "DSI Host Video HBP Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpcr(pub u32);
    impl Vhbpcr {
        #[doc = "Horizontal Back-Porch duration."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Back-Porch duration."]
        #[inline(always)]
        pub fn set_hbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhbpcr {
        #[inline(always)]
        fn default() -> Vhbpcr {
            Vhbpcr(0)
        }
    }
    impl core::fmt::Debug for Vhbpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vhbpcr").field("hbp", &self.hbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vhbpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vhbpcr {{ hbp: {=u16:?} }}", self.hbp())
        }
    }
    #[doc = "DSI Host Video HSA Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsaccr(pub u32);
    impl Vhsaccr {
        #[doc = "Horizontal Synchronism Active duration."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Synchronism Active duration."]
        #[inline(always)]
        pub fn set_hsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhsaccr {
        #[inline(always)]
        fn default() -> Vhsaccr {
            Vhsaccr(0)
        }
    }
    impl core::fmt::Debug for Vhsaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vhsaccr").field("hsa", &self.hsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vhsaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vhsaccr {{ hsa: {=u16:?} }}", self.hsa())
        }
    }
    #[doc = "DSI Host Video HSA Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsacr(pub u32);
    impl Vhsacr {
        #[doc = "Horizontal Synchronism Active duration."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Synchronism Active duration."]
        #[inline(always)]
        pub fn set_hsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhsacr {
        #[inline(always)]
        fn default() -> Vhsacr {
            Vhsacr(0)
        }
    }
    impl core::fmt::Debug for Vhsacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vhsacr").field("hsa", &self.hsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vhsacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vhsacr {{ hsa: {=u16:?} }}", self.hsa())
        }
    }
    #[doc = "DSI Host Video Line Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlccr(pub u32);
    impl Vlccr {
        #[doc = "Horizontal Line duration."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal Line duration."]
        #[inline(always)]
        pub fn set_hline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Vlccr {
        #[inline(always)]
        fn default() -> Vlccr {
            Vlccr(0)
        }
    }
    impl core::fmt::Debug for Vlccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vlccr").field("hline", &self.hline()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vlccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vlccr {{ hline: {=u16:?} }}", self.hline())
        }
    }
    #[doc = "DSI Host Video Line Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlcr(pub u32);
    impl Vlcr {
        #[doc = "Horizontal Line duration."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal Line duration."]
        #[inline(always)]
        pub fn set_hline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Vlcr {
        #[inline(always)]
        fn default() -> Vlcr {
            Vlcr(0)
        }
    }
    impl core::fmt::Debug for Vlcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vlcr").field("hline", &self.hline()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vlcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vlcr {{ hline: {=u16:?} }}", self.hline())
        }
    }
    #[doc = "DSI Host Video mode Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmccr(pub u32);
    impl Vmccr {
        #[doc = "Video mode Type."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode Type."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-Power Vertical Sync time Enable."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Vertical Sync time Enable."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Low-power Vertical Back-Porch Enable."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Vertical Back-Porch Enable."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Low-power Vertical Front-Porch Enable."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Vertical Front-Porch Enable."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Low-Power Vertical Active Enable."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Vertical Active Enable."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Low-power Horizontal Back-Porch Enable."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Horizontal Back-Porch Enable."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Low-Power Horizontal Front-Porch Enable."]
        #[inline(always)]
        pub const fn lphfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Horizontal Front-Porch Enable."]
        #[inline(always)]
        pub fn set_lphfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame BTA Acknowledge Enable."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Frame BTA Acknowledge Enable."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-Power Command Enable."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Command Enable."]
        #[inline(always)]
        pub fn set_lpce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vmccr {
        #[inline(always)]
        fn default() -> Vmccr {
            Vmccr(0)
        }
    }
    impl core::fmt::Debug for Vmccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmccr")
                .field("vmt", &self.vmt())
                .field("lpvsae", &self.lpvsae())
                .field("lpvbpe", &self.lpvbpe())
                .field("lpvfpe", &self.lpvfpe())
                .field("lpvae", &self.lpvae())
                .field("lphbpe", &self.lphbpe())
                .field("lphfe", &self.lphfe())
                .field("fbtaae", &self.fbtaae())
                .field("lpce", &self.lpce())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vmccr {{ vmt: {=u8:?}, lpvsae: {=bool:?}, lpvbpe: {=bool:?}, lpvfpe: {=bool:?}, lpvae: {=bool:?}, lphbpe: {=bool:?}, lphfe: {=bool:?}, fbtaae: {=bool:?}, lpce: {=bool:?} }}" , self . vmt () , self . lpvsae () , self . lpvbpe () , self . lpvfpe () , self . lpvae () , self . lphbpe () , self . lphfe () , self . fbtaae () , self . lpce ())
        }
    }
    #[doc = "DSI Host Video mode Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[doc = "Video mode Type."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode Type."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-Power Vertical Sync Active Enable."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Vertical Sync Active Enable."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power Vertical Back-Porch Enable."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Vertical Back-Porch Enable."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low-power Vertical Front-porch Enable."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power Vertical Front-porch Enable."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Low-Power Vertical Active Enable."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Vertical Active Enable."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Low-Power Horizontal Back-Porch Enable."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Horizontal Back-Porch Enable."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Low-Power Horizontal Front-Porch Enable."]
        #[inline(always)]
        pub const fn lphfpe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Horizontal Front-Porch Enable."]
        #[inline(always)]
        pub fn set_lphfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Frame Bus-Turn-Around Acknowledge Enable."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Bus-Turn-Around Acknowledge Enable."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Low-Power Command Enable."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Command Enable."]
        #[inline(always)]
        pub fn set_lpce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Pattern Generator Enable."]
        #[inline(always)]
        pub const fn pge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern Generator Enable."]
        #[inline(always)]
        pub fn set_pge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pattern Generator mode."]
        #[inline(always)]
        pub const fn pgm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern Generator mode."]
        #[inline(always)]
        pub fn set_pgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Pattern Generator Orientation."]
        #[inline(always)]
        pub const fn pgo(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern Generator Orientation."]
        #[inline(always)]
        pub fn set_pgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    impl core::fmt::Debug for Vmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmcr")
                .field("vmt", &self.vmt())
                .field("lpvsae", &self.lpvsae())
                .field("lpvbpe", &self.lpvbpe())
                .field("lpvfpe", &self.lpvfpe())
                .field("lpvae", &self.lpvae())
                .field("lphbpe", &self.lphbpe())
                .field("lphfpe", &self.lphfpe())
                .field("fbtaae", &self.fbtaae())
                .field("lpce", &self.lpce())
                .field("pge", &self.pge())
                .field("pgm", &self.pgm())
                .field("pgo", &self.pgo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vmcr {{ vmt: {=u8:?}, lpvsae: {=bool:?}, lpvbpe: {=bool:?}, lpvfpe: {=bool:?}, lpvae: {=bool:?}, lphbpe: {=bool:?}, lphfpe: {=bool:?}, fbtaae: {=bool:?}, lpce: {=bool:?}, pge: {=bool:?}, pgm: {=bool:?}, pgo: {=bool:?} }}" , self . vmt () , self . lpvsae () , self . lpvbpe () , self . lpvfpe () , self . lpvae () , self . lphbpe () , self . lphfpe () , self . fbtaae () , self . lpce () , self . pge () , self . pgm () , self . pgo ())
        }
    }
    #[doc = "DSI Host Video Null Packet Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpccr(pub u32);
    impl Vnpccr {
        #[doc = "Null Packet Size."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null Packet Size."]
        #[inline(always)]
        pub fn set_npsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vnpccr {
        #[inline(always)]
        fn default() -> Vnpccr {
            Vnpccr(0)
        }
    }
    impl core::fmt::Debug for Vnpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vnpccr").field("npsize", &self.npsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vnpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vnpccr {{ npsize: {=u16:?} }}", self.npsize())
        }
    }
    #[doc = "DSI Host Video Null Packet Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpcr(pub u32);
    impl Vnpcr {
        #[doc = "Null Packet Size."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null Packet Size."]
        #[inline(always)]
        pub fn set_npsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vnpcr {
        #[inline(always)]
        fn default() -> Vnpcr {
            Vnpcr(0)
        }
    }
    impl core::fmt::Debug for Vnpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vnpcr").field("npsize", &self.npsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vnpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vnpcr {{ npsize: {=u16:?} }}", self.npsize())
        }
    }
    #[doc = "DSI Host Video Packet Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpccr(pub u32);
    impl Vpccr {
        #[doc = "Video Packet Size."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video Packet Size."]
        #[inline(always)]
        pub fn set_vpsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vpccr {
        #[inline(always)]
        fn default() -> Vpccr {
            Vpccr(0)
        }
    }
    impl core::fmt::Debug for Vpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vpccr").field("vpsize", &self.vpsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vpccr {{ vpsize: {=u16:?} }}", self.vpsize())
        }
    }
    #[doc = "DSI Host Video Packet Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpcr(pub u32);
    impl Vpcr {
        #[doc = "Video Packet Size."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video Packet Size."]
        #[inline(always)]
        pub fn set_vpsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vpcr {
        #[inline(always)]
        fn default() -> Vpcr {
            Vpcr(0)
        }
    }
    impl core::fmt::Debug for Vpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vpcr").field("vpsize", &self.vpsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vpcr {{ vpsize: {=u16:?} }}", self.vpsize())
        }
    }
    #[doc = "DSI Host Version Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vr(pub u32);
    impl Vr {
        #[doc = "Version of the DSI Host."]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Version of the DSI Host."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Vr {
        #[inline(always)]
        fn default() -> Vr {
            Vr(0)
        }
    }
    impl core::fmt::Debug for Vr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vr").field("version", &self.version()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vr {{ version: {=u32:?} }}", self.version())
        }
    }
    #[doc = "DSI Host Video Shadow Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vscr(pub u32);
    impl Vscr {
        #[doc = "Enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update Register."]
        #[inline(always)]
        pub const fn ur(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update Register."]
        #[inline(always)]
        pub fn set_ur(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vscr {
        #[inline(always)]
        fn default() -> Vscr {
            Vscr(0)
        }
    }
    impl core::fmt::Debug for Vscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vscr")
                .field("en", &self.en())
                .field("ur", &self.ur())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vscr {{ en: {=bool:?}, ur: {=bool:?} }}", self.en(), self.ur())
        }
    }
    #[doc = "DSI Host Video VA Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvaccr(pub u32);
    impl Vvaccr {
        #[doc = "Vertical Active duration."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical Active duration."]
        #[inline(always)]
        pub fn set_va(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vvaccr {
        #[inline(always)]
        fn default() -> Vvaccr {
            Vvaccr(0)
        }
    }
    impl core::fmt::Debug for Vvaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvaccr").field("va", &self.va()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvaccr {{ va: {=u16:?} }}", self.va())
        }
    }
    #[doc = "DSI Host Video VA Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvacr(pub u32);
    impl Vvacr {
        #[doc = "Vertical Active duration."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical Active duration."]
        #[inline(always)]
        pub fn set_va(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vvacr {
        #[inline(always)]
        fn default() -> Vvacr {
            Vvacr(0)
        }
    }
    impl core::fmt::Debug for Vvacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvacr").field("va", &self.va()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvacr {{ va: {=u16:?} }}", self.va())
        }
    }
    #[doc = "DSI Host Video VBP Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpccr(pub u32);
    impl Vvbpccr {
        #[doc = "Vertical Back-Porch duration."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Back-Porch duration."]
        #[inline(always)]
        pub fn set_vbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvbpccr {
        #[inline(always)]
        fn default() -> Vvbpccr {
            Vvbpccr(0)
        }
    }
    impl core::fmt::Debug for Vvbpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvbpccr").field("vbp", &self.vbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvbpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvbpccr {{ vbp: {=u16:?} }}", self.vbp())
        }
    }
    #[doc = "DSI Host Video VBP Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpcr(pub u32);
    impl Vvbpcr {
        #[doc = "Vertical Back-Porch duration."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Back-Porch duration."]
        #[inline(always)]
        pub fn set_vbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvbpcr {
        #[inline(always)]
        fn default() -> Vvbpcr {
            Vvbpcr(0)
        }
    }
    impl core::fmt::Debug for Vvbpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvbpcr").field("vbp", &self.vbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvbpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvbpcr {{ vbp: {=u16:?} }}", self.vbp())
        }
    }
    #[doc = "DSI Host Video VFP Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpccr(pub u32);
    impl Vvfpccr {
        #[doc = "Vertical Front-Porch duration."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Front-Porch duration."]
        #[inline(always)]
        pub fn set_vfp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvfpccr {
        #[inline(always)]
        fn default() -> Vvfpccr {
            Vvfpccr(0)
        }
    }
    impl core::fmt::Debug for Vvfpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvfpccr").field("vfp", &self.vfp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvfpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvfpccr {{ vfp: {=u16:?} }}", self.vfp())
        }
    }
    #[doc = "DSI Host Video VFP Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpcr(pub u32);
    impl Vvfpcr {
        #[doc = "Vertical Front-Porch duration."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Front-Porch duration."]
        #[inline(always)]
        pub fn set_vfp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvfpcr {
        #[inline(always)]
        fn default() -> Vvfpcr {
            Vvfpcr(0)
        }
    }
    impl core::fmt::Debug for Vvfpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvfpcr").field("vfp", &self.vfp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvfpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvfpcr {{ vfp: {=u16:?} }}", self.vfp())
        }
    }
    #[doc = "DSI Host Video VSA Current Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsaccr(pub u32);
    impl Vvsaccr {
        #[doc = "Vertical Synchronism Active duration."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Synchronism Active duration."]
        #[inline(always)]
        pub fn set_vsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvsaccr {
        #[inline(always)]
        fn default() -> Vvsaccr {
            Vvsaccr(0)
        }
    }
    impl core::fmt::Debug for Vvsaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvsaccr").field("vsa", &self.vsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvsaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvsaccr {{ vsa: {=u16:?} }}", self.vsa())
        }
    }
    #[doc = "DSI Host Video VSA Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsacr(pub u32);
    impl Vvsacr {
        #[doc = "Vertical Synchronism Active duration."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical Synchronism Active duration."]
        #[inline(always)]
        pub fn set_vsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvsacr {
        #[inline(always)]
        fn default() -> Vvsacr {
            Vvsacr(0)
        }
    }
    impl core::fmt::Debug for Vvsacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vvsacr").field("vsa", &self.vsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vvsacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vvsacr {{ vsa: {=u16:?} }}", self.vsa())
        }
    }
    #[doc = "DSI Wrapper Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcfgr(pub u32);
    impl Wcfgr {
        #[doc = "DSI Mode."]
        #[inline(always)]
        pub const fn dsim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Mode."]
        #[inline(always)]
        pub fn set_dsim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Multiplexing."]
        #[inline(always)]
        pub const fn colmux(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Color Multiplexing."]
        #[inline(always)]
        pub fn set_colmux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "TE Source."]
        #[inline(always)]
        pub const fn tesrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TE Source."]
        #[inline(always)]
        pub fn set_tesrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TE Polarity."]
        #[inline(always)]
        pub const fn tepol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TE Polarity."]
        #[inline(always)]
        pub fn set_tepol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic Refresh."]
        #[inline(always)]
        pub const fn ar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Refresh."]
        #[inline(always)]
        pub fn set_ar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VSync Polarity."]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VSync Polarity."]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wcfgr {
        #[inline(always)]
        fn default() -> Wcfgr {
            Wcfgr(0)
        }
    }
    impl core::fmt::Debug for Wcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wcfgr")
                .field("dsim", &self.dsim())
                .field("colmux", &self.colmux())
                .field("tesrc", &self.tesrc())
                .field("tepol", &self.tepol())
                .field("ar", &self.ar())
                .field("vspol", &self.vspol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wcfgr {{ dsim: {=bool:?}, colmux: {=u8:?}, tesrc: {=bool:?}, tepol: {=bool:?}, ar: {=bool:?}, vspol: {=bool:?} }}" , self . dsim () , self . colmux () , self . tesrc () , self . tepol () , self . ar () , self . vspol ())
        }
    }
    #[doc = "DSI Wrapper Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcr(pub u32);
    impl Wcr {
        #[doc = "Color Mode."]
        #[inline(always)]
        pub const fn colm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Color Mode."]
        #[inline(always)]
        pub fn set_colm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Shutdown."]
        #[inline(always)]
        pub const fn shtdn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Shutdown."]
        #[inline(always)]
        pub fn set_shtdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LTDC Enable."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC Enable."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DSI Enable."]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Enable."]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Wcr {
        #[inline(always)]
        fn default() -> Wcr {
            Wcr(0)
        }
    }
    impl core::fmt::Debug for Wcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wcr")
                .field("colm", &self.colm())
                .field("shtdn", &self.shtdn())
                .field("ltdcen", &self.ltdcen())
                .field("dsien", &self.dsien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wcr {{ colm: {=bool:?}, shtdn: {=bool:?}, ltdcen: {=bool:?}, dsien: {=bool:?} }}",
                self.colm(),
                self.shtdn(),
                self.ltdcen(),
                self.dsien()
            )
        }
    }
    #[doc = "DSI Wrapper Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wier(pub u32);
    impl Wier {
        #[doc = "Tearing Effect Interrupt Enable."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing Effect Interrupt Enable."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of Refresh Interrupt Enable."]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of Refresh Interrupt Enable."]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PLL Lock Interrupt Enable."]
        #[inline(always)]
        pub const fn plllie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Lock Interrupt Enable."]
        #[inline(always)]
        pub fn set_plllie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL Unlock Interrupt Enable."]
        #[inline(always)]
        pub const fn plluie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Unlock Interrupt Enable."]
        #[inline(always)]
        pub fn set_plluie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Regulator Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Wier {
        #[inline(always)]
        fn default() -> Wier {
            Wier(0)
        }
    }
    impl core::fmt::Debug for Wier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wier")
                .field("teie", &self.teie())
                .field("erie", &self.erie())
                .field("plllie", &self.plllie())
                .field("plluie", &self.plluie())
                .field("rrie", &self.rrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wier {{ teie: {=bool:?}, erie: {=bool:?}, plllie: {=bool:?}, plluie: {=bool:?}, rrie: {=bool:?} }}",
                self.teie(),
                self.erie(),
                self.plllie(),
                self.plluie(),
                self.rrie()
            )
        }
    }
    #[doc = "DSI Wrapper Interrupt Flag Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wifcr(pub u32);
    impl Wifcr {
        #[doc = "Clear Tearing Effect Interrupt Flag."]
        #[inline(always)]
        pub const fn cteif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Tearing Effect Interrupt Flag."]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear End of Refresh Interrupt Flag."]
        #[inline(always)]
        pub const fn cerif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear End of Refresh Interrupt Flag."]
        #[inline(always)]
        pub fn set_cerif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear PLL Lock Interrupt Flag."]
        #[inline(always)]
        pub const fn cplllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL Lock Interrupt Flag."]
        #[inline(always)]
        pub fn set_cplllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear PLL Unlock Interrupt Flag."]
        #[inline(always)]
        pub const fn cplluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL Unlock Interrupt Flag."]
        #[inline(always)]
        pub fn set_cplluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear Regulator Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn crrif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Regulator Ready Interrupt Flag."]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Wifcr {
        #[inline(always)]
        fn default() -> Wifcr {
            Wifcr(0)
        }
    }
    impl core::fmt::Debug for Wifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wifcr")
                .field("cteif", &self.cteif())
                .field("cerif", &self.cerif())
                .field("cplllif", &self.cplllif())
                .field("cplluif", &self.cplluif())
                .field("crrif", &self.crrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wifcr {{ cteif: {=bool:?}, cerif: {=bool:?}, cplllif: {=bool:?}, cplluif: {=bool:?}, crrif: {=bool:?} }}" , self . cteif () , self . cerif () , self . cplllif () , self . cplluif () , self . crrif ())
        }
    }
    #[doc = "DSI Wrapper Interrupt & Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wisr(pub u32);
    impl Wisr {
        #[doc = "Tearing Effect Interrupt Flag."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing Effect Interrupt Flag."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of Refresh Interrupt Flag."]
        #[inline(always)]
        pub const fn erif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of Refresh Interrupt Flag."]
        #[inline(always)]
        pub fn set_erif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Busy Flag."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Busy Flag."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLL Lock Status."]
        #[inline(always)]
        pub const fn pllls(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Lock Status."]
        #[inline(always)]
        pub fn set_pllls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLL Lock Interrupt Flag."]
        #[inline(always)]
        pub const fn plllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Lock Interrupt Flag."]
        #[inline(always)]
        pub fn set_plllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL Unlock Interrupt Flag."]
        #[inline(always)]
        pub const fn plluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Unlock Interrupt Flag."]
        #[inline(always)]
        pub fn set_plluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Regulator Ready Status."]
        #[inline(always)]
        pub const fn rrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator Ready Status."]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Regulator Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator Ready Interrupt Flag."]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Wisr {
        #[inline(always)]
        fn default() -> Wisr {
            Wisr(0)
        }
    }
    impl core::fmt::Debug for Wisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wisr")
                .field("teif", &self.teif())
                .field("erif", &self.erif())
                .field("busy", &self.busy())
                .field("pllls", &self.pllls())
                .field("plllif", &self.plllif())
                .field("plluif", &self.plluif())
                .field("rrs", &self.rrs())
                .field("rrif", &self.rrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wisr {{ teif: {=bool:?}, erif: {=bool:?}, busy: {=bool:?}, pllls: {=bool:?}, plllif: {=bool:?}, plluif: {=bool:?}, rrs: {=bool:?}, rrif: {=bool:?} }}" , self . teif () , self . erif () , self . busy () , self . pllls () , self . plllif () , self . plluif () , self . rrs () , self . rrif ())
        }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr0(pub u32);
    impl Wpcr0 {
        #[doc = "Unit Interval multiplied by 4."]
        #[inline(always)]
        pub const fn uix4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Unit Interval multiplied by 4."]
        #[inline(always)]
        pub fn set_uix4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Swap Clock Lane pins."]
        #[inline(always)]
        pub const fn swcl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Swap Clock Lane pins."]
        #[inline(always)]
        pub fn set_swcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Swap Data Lane 0 pins."]
        #[inline(always)]
        pub const fn swdl0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Swap Data Lane 0 pins."]
        #[inline(always)]
        pub fn set_swdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Swap Data Lane 1 pins."]
        #[inline(always)]
        pub const fn swdl1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Swap Data Lane 1 pins."]
        #[inline(always)]
        pub fn set_swdl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Invert Hight-Speed data signal on Clock Lane."]
        #[inline(always)]
        pub const fn hsicl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Invert Hight-Speed data signal on Clock Lane."]
        #[inline(always)]
        pub fn set_hsicl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Invert the Hight-Speed data signal on Data Lane 0."]
        #[inline(always)]
        pub const fn hsidl0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the Hight-Speed data signal on Data Lane 0."]
        #[inline(always)]
        pub fn set_hsidl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Invert the High-Speed data signal on Data Lane 1."]
        #[inline(always)]
        pub const fn hsidl1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the High-Speed data signal on Data Lane 1."]
        #[inline(always)]
        pub fn set_hsidl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force in TX Stop Mode the Clock Lane."]
        #[inline(always)]
        pub const fn ftxsmcl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop Mode the Clock Lane."]
        #[inline(always)]
        pub fn set_ftxsmcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force in TX Stop Mode the Data Lanes."]
        #[inline(always)]
        pub const fn ftxsmdl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop Mode the Data Lanes."]
        #[inline(always)]
        pub fn set_ftxsmdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Contention Detection OFF on Data Lanes."]
        #[inline(always)]
        pub const fn cdoffdl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Contention Detection OFF on Data Lanes."]
        #[inline(always)]
        pub fn set_cdoffdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Turn Disable Data Lanes."]
        #[inline(always)]
        pub const fn tddl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Turn Disable Data Lanes."]
        #[inline(always)]
        pub fn set_tddl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pull-Down Enable."]
        #[inline(always)]
        pub const fn pden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Pull-Down Enable."]
        #[inline(always)]
        pub fn set_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "custom time for tCLK-PREPARE Enable."]
        #[inline(always)]
        pub const fn tclkprepen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tCLK-PREPARE Enable."]
        #[inline(always)]
        pub fn set_tclkprepen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "custom time for tCLK-ZERO Enable."]
        #[inline(always)]
        pub const fn tclkzeroen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tCLK-ZERO Enable."]
        #[inline(always)]
        pub fn set_tclkzeroen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "custom time for tHS-PREPARE Enable."]
        #[inline(always)]
        pub const fn thsprepen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tHS-PREPARE Enable."]
        #[inline(always)]
        pub fn set_thsprepen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "custom time for tHS-TRAIL Enable."]
        #[inline(always)]
        pub const fn thstrailen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tHS-TRAIL Enable."]
        #[inline(always)]
        pub fn set_thstrailen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "custom time for tHS-ZERO Enable."]
        #[inline(always)]
        pub const fn thszeroen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tHS-ZERO Enable."]
        #[inline(always)]
        pub fn set_thszeroen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "custom time for tLPX for Data lanes Enable."]
        #[inline(always)]
        pub const fn tlpxden(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tLPX for Data lanes Enable."]
        #[inline(always)]
        pub fn set_tlpxden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "custom time for tHS-EXIT Enable."]
        #[inline(always)]
        pub const fn thsexiten(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tHS-EXIT Enable."]
        #[inline(always)]
        pub fn set_thsexiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "custom time for tLPX for Clock lane Enable."]
        #[inline(always)]
        pub const fn tlpxcen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tLPX for Clock lane Enable."]
        #[inline(always)]
        pub fn set_tlpxcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "custom time for tCLK-POST Enable."]
        #[inline(always)]
        pub const fn tclkposten(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "custom time for tCLK-POST Enable."]
        #[inline(always)]
        pub fn set_tclkposten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Wpcr0 {
        #[inline(always)]
        fn default() -> Wpcr0 {
            Wpcr0(0)
        }
    }
    impl core::fmt::Debug for Wpcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpcr0")
                .field("uix4", &self.uix4())
                .field("swcl", &self.swcl())
                .field("swdl0", &self.swdl0())
                .field("swdl1", &self.swdl1())
                .field("hsicl", &self.hsicl())
                .field("hsidl0", &self.hsidl0())
                .field("hsidl1", &self.hsidl1())
                .field("ftxsmcl", &self.ftxsmcl())
                .field("ftxsmdl", &self.ftxsmdl())
                .field("cdoffdl", &self.cdoffdl())
                .field("tddl", &self.tddl())
                .field("pden", &self.pden())
                .field("tclkprepen", &self.tclkprepen())
                .field("tclkzeroen", &self.tclkzeroen())
                .field("thsprepen", &self.thsprepen())
                .field("thstrailen", &self.thstrailen())
                .field("thszeroen", &self.thszeroen())
                .field("tlpxden", &self.tlpxden())
                .field("thsexiten", &self.thsexiten())
                .field("tlpxcen", &self.tlpxcen())
                .field("tclkposten", &self.tclkposten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wpcr0 {{ uix4: {=u8:?}, swcl: {=bool:?}, swdl0: {=bool:?}, swdl1: {=bool:?}, hsicl: {=bool:?}, hsidl0: {=bool:?}, hsidl1: {=bool:?}, ftxsmcl: {=bool:?}, ftxsmdl: {=bool:?}, cdoffdl: {=bool:?}, tddl: {=bool:?}, pden: {=bool:?}, tclkprepen: {=bool:?}, tclkzeroen: {=bool:?}, thsprepen: {=bool:?}, thstrailen: {=bool:?}, thszeroen: {=bool:?}, tlpxden: {=bool:?}, thsexiten: {=bool:?}, tlpxcen: {=bool:?}, tclkposten: {=bool:?} }}" , self . uix4 () , self . swcl () , self . swdl0 () , self . swdl1 () , self . hsicl () , self . hsidl0 () , self . hsidl1 () , self . ftxsmcl () , self . ftxsmdl () , self . cdoffdl () , self . tddl () , self . pden () , self . tclkprepen () , self . tclkzeroen () , self . thsprepen () , self . thstrailen () , self . thszeroen () , self . tlpxden () , self . thsexiten () , self . tlpxcen () , self . tclkposten ())
        }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr1(pub u32);
    impl Wpcr1 {
        #[doc = "High-Speed Transmission Delay on Clock Lane."]
        #[inline(always)]
        pub const fn hstxdcl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "High-Speed Transmission Delay on Clock Lane."]
        #[inline(always)]
        pub fn set_hstxdcl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "High-Speed Transmission Delay on Data Lanes."]
        #[inline(always)]
        pub const fn hstxdll(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "High-Speed Transmission Delay on Data Lanes."]
        #[inline(always)]
        pub fn set_hstxdll(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Low-Power transmission Slew Rate Compensation on Clock Lane."]
        #[inline(always)]
        pub const fn lpsrcl(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Low-Power transmission Slew Rate Compensation on Clock Lane."]
        #[inline(always)]
        pub fn set_lpsrcl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Low-Power transmission Slew Rate Compensation on Data Lanes."]
        #[inline(always)]
        pub const fn lpsrdl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Low-Power transmission Slew Rate Compensation on Data Lanes."]
        #[inline(always)]
        pub fn set_lpsrdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "SDD Control."]
        #[inline(always)]
        pub const fn sdcc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SDD Control."]
        #[inline(always)]
        pub fn set_sdcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "High-Speed Transmission Slew Rate Control on Clock Lane."]
        #[inline(always)]
        pub const fn hstxsrccl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "High-Speed Transmission Slew Rate Control on Clock Lane."]
        #[inline(always)]
        pub fn set_hstxsrccl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "High-Speed Transmission Slew Rate Control on Data Lanes."]
        #[inline(always)]
        pub const fn hstxsrcdl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "High-Speed Transmission Slew Rate Control on Data Lanes."]
        #[inline(always)]
        pub fn set_hstxsrcdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Forces LP Receiver in Low-Power Mode."]
        #[inline(always)]
        pub const fn flprxlpm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Forces LP Receiver in Low-Power Mode."]
        #[inline(always)]
        pub fn set_flprxlpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Low-Power RX low-pass Filtering Tuning."]
        #[inline(always)]
        pub const fn lprxft(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "Low-Power RX low-pass Filtering Tuning."]
        #[inline(always)]
        pub fn set_lprxft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for Wpcr1 {
        #[inline(always)]
        fn default() -> Wpcr1 {
            Wpcr1(0)
        }
    }
    impl core::fmt::Debug for Wpcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpcr1")
                .field("hstxdcl", &self.hstxdcl())
                .field("hstxdll", &self.hstxdll())
                .field("lpsrcl", &self.lpsrcl())
                .field("lpsrdl", &self.lpsrdl())
                .field("sdcc", &self.sdcc())
                .field("hstxsrccl", &self.hstxsrccl())
                .field("hstxsrcdl", &self.hstxsrcdl())
                .field("flprxlpm", &self.flprxlpm())
                .field("lprxft", &self.lprxft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wpcr1 {{ hstxdcl: {=u8:?}, hstxdll: {=u8:?}, lpsrcl: {=u8:?}, lpsrdl: {=u8:?}, sdcc: {=bool:?}, hstxsrccl: {=u8:?}, hstxsrcdl: {=u8:?}, flprxlpm: {=bool:?}, lprxft: {=u8:?} }}" , self . hstxdcl () , self . hstxdll () , self . lpsrcl () , self . lpsrdl () , self . sdcc () , self . hstxsrccl () , self . hstxsrcdl () , self . flprxlpm () , self . lprxft ())
        }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr2(pub u32);
    impl Wpcr2 {
        #[doc = "tCLK-PREPARE."]
        #[inline(always)]
        pub const fn tclkprep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "tCLK-PREPARE."]
        #[inline(always)]
        pub fn set_tclkprep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "tCLK-ZERO."]
        #[inline(always)]
        pub const fn tclkzeo(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "tCLK-ZERO."]
        #[inline(always)]
        pub fn set_tclkzeo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "tHS-PREPARE."]
        #[inline(always)]
        pub const fn thsprep(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "tHS-PREPARE."]
        #[inline(always)]
        pub fn set_thsprep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "tHSTRAIL."]
        #[inline(always)]
        pub const fn thstrail(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "tHSTRAIL."]
        #[inline(always)]
        pub fn set_thstrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Wpcr2 {
        #[inline(always)]
        fn default() -> Wpcr2 {
            Wpcr2(0)
        }
    }
    impl core::fmt::Debug for Wpcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpcr2")
                .field("tclkprep", &self.tclkprep())
                .field("tclkzeo", &self.tclkzeo())
                .field("thsprep", &self.thsprep())
                .field("thstrail", &self.thstrail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wpcr2 {{ tclkprep: {=u8:?}, tclkzeo: {=u8:?}, thsprep: {=u8:?}, thstrail: {=u8:?} }}",
                self.tclkprep(),
                self.tclkzeo(),
                self.thsprep(),
                self.thstrail()
            )
        }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr3(pub u32);
    impl Wpcr3 {
        #[doc = "tHS-ZERO."]
        #[inline(always)]
        pub const fn thszero(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "tHS-ZERO."]
        #[inline(always)]
        pub fn set_thszero(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "tLPX for Data lanes."]
        #[inline(always)]
        pub const fn tlpxd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "tLPX for Data lanes."]
        #[inline(always)]
        pub fn set_tlpxd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "tHSEXIT."]
        #[inline(always)]
        pub const fn thsexit(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "tHSEXIT."]
        #[inline(always)]
        pub fn set_thsexit(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "tLPXC for Clock lane."]
        #[inline(always)]
        pub const fn tlpxc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "tLPXC for Clock lane."]
        #[inline(always)]
        pub fn set_tlpxc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Wpcr3 {
        #[inline(always)]
        fn default() -> Wpcr3 {
            Wpcr3(0)
        }
    }
    impl core::fmt::Debug for Wpcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpcr3")
                .field("thszero", &self.thszero())
                .field("tlpxd", &self.tlpxd())
                .field("thsexit", &self.thsexit())
                .field("tlpxc", &self.tlpxc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wpcr3 {{ thszero: {=u8:?}, tlpxd: {=u8:?}, thsexit: {=u8:?}, tlpxc: {=u8:?} }}",
                self.thszero(),
                self.tlpxd(),
                self.thsexit(),
                self.tlpxc()
            )
        }
    }
    #[doc = "DSI Wrapper PHY Configuration Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr4(pub u32);
    impl Wpcr4 {
        #[doc = "tCLK-POST."]
        #[inline(always)]
        pub const fn tclkpost(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "tCLK-POST."]
        #[inline(always)]
        pub fn set_tclkpost(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wpcr4 {
        #[inline(always)]
        fn default() -> Wpcr4 {
            Wpcr4(0)
        }
    }
    impl core::fmt::Debug for Wpcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpcr4").field("tclkpost", &self.tclkpost()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wpcr4 {{ tclkpost: {=u8:?} }}", self.tclkpost())
        }
    }
    #[doc = "DSI Wrapper Regulator and PLL Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpcr(pub u32);
    impl Wrpcr {
        #[doc = "PLL Enable."]
        #[inline(always)]
        pub const fn pllen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Enable."]
        #[inline(always)]
        pub fn set_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLL Loop Division Factor."]
        #[inline(always)]
        pub const fn ndiv(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[doc = "PLL Loop Division Factor."]
        #[inline(always)]
        pub fn set_ndiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
        #[doc = "PLL Input Division Factor."]
        #[inline(always)]
        pub const fn idf(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "PLL Input Division Factor."]
        #[inline(always)]
        pub fn set_idf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "PLL Output Division Factor."]
        #[inline(always)]
        pub const fn odf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "PLL Output Division Factor."]
        #[inline(always)]
        pub fn set_odf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Regulator Enable."]
        #[inline(always)]
        pub const fn regen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator Enable."]
        #[inline(always)]
        pub fn set_regen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Wrpcr {
        #[inline(always)]
        fn default() -> Wrpcr {
            Wrpcr(0)
        }
    }
    impl core::fmt::Debug for Wrpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpcr")
                .field("pllen", &self.pllen())
                .field("ndiv", &self.ndiv())
                .field("idf", &self.idf())
                .field("odf", &self.odf())
                .field("regen", &self.regen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrpcr {{ pllen: {=bool:?}, ndiv: {=u8:?}, idf: {=u8:?}, odf: {=u8:?}, regen: {=bool:?} }}",
                self.pllen(),
                self.ndiv(),
                self.idf(),
                self.odf(),
                self.regen()
            )
        }
    }
}
